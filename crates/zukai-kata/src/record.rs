use std::fmt;
use std::sync::Arc;

use blue_lang_host::{EvalError, FromValue, IntoValue, Span, Value};
use tatara_lisp::{Atom, KeywordSexp, Sexp, TataraDomain};
use tatara_lisp_eval::MapKey;
use zukai_model::Role;

const KIND: &str = "kind";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecError {
    NotARecord { record: &'static str, found: &'static str },
    UnknownKey { record: &'static str, key: String },
    Missing { record: &'static str, key: &'static str },
    Field { record: &'static str, key: &'static str, message: String },
    NotARole { found: &'static str },
    UnknownRole(String),
    NoKind,
    Unrepresentable(&'static str),
    Compile { form: &'static str, message: String },
}

impl fmt::Display for SpecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotARecord { record, found } => {
                write!(f, "{record} is a map of fields, got {found}")
            }
            Self::UnknownKey { record, key } => write!(f, "{record} has no field {key}"),
            Self::Missing { record, key } => write!(f, "{record} needs the field {key}"),
            Self::Field {
                record,
                key,
                message,
            } => write!(f, "{record}.{key}: {message}"),
            Self::NotARole { found } => write!(
                f,
                "a role is one of :master :read :accent :neutral :danger :success :warning :info, got {found}"
            ),
            Self::UnknownRole(name) => write!(f, "unknown role :{name}"),
            Self::NoKind => write!(f, "a form map needs a keyword kind"),
            Self::Unrepresentable(found) => write!(f, "a form cannot hold a {found}"),
            Self::Compile { form, message } => write!(f, "{form}: {message}"),
        }
    }
}

impl SpecError {
    #[must_use]
    pub fn at(&self, at: Span) -> EvalError {
        EvalError::native_fn("zukai", self.to_string(), at)
    }
}

fn keyword_of(key: &MapKey) -> Option<&str> {
    match key {
        MapKey::Keyword(k) => Some(k),
        _ => None,
    }
}

pub fn fields<'a>(
    record: &'static str,
    keys: &[&str],
    v: &'a Value,
    at: Span,
) -> Result<&'a Value, EvalError> {
    let Value::Map(m) = v else {
        return Err(SpecError::NotARecord {
            record,
            found: v.type_name(),
        }
        .at(at));
    };
    for key in m.keys() {
        match keyword_of(key) {
            Some(k) if keys.contains(&k) => {}
            _ => {
                return Err(SpecError::UnknownKey {
                    record,
                    key: key.to_string(),
                }
                .at(at))
            }
        }
    }
    Ok(v)
}

pub fn field<T: FromValue>(
    record: &'static str,
    v: &Value,
    key: &'static str,
    at: Span,
) -> Result<T, EvalError> {
    let found = match v {
        Value::Map(m) => m.get(&MapKey::Keyword(Arc::from(key))),
        _ => None,
    };
    match found {
        Some(x) => T::from_value(x, at).map_err(|e| {
            SpecError::Field {
                record,
                key,
                message: e.short_message(),
            }
            .at(at)
        }),
        None => T::from_value(&Value::Nil, at).map_err(|_| SpecError::Missing { record, key }.at(at)),
    }
}

macro_rules! record {
    ($name:ident = $label:literal { $($field:ident : $ty:ty),* $(,)? }) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            $(pub $field: $ty),*
        }

        impl blue_lang_host::FromValue for $name {
            fn from_value(
                v: &blue_lang_host::Value,
                at: blue_lang_host::Span,
            ) -> Result<Self, blue_lang_host::EvalError> {
                let m = $crate::record::fields($label, &[$(stringify!($field)),*], v, at)?;
                Ok(Self {
                    $($field: $crate::record::field($label, m, stringify!($field), at)?),*
                })
            }
        }
    };
}

pub(crate) use record;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoleKey(pub Role);

impl FromValue for RoleKey {
    fn from_value(v: &Value, at: Span) -> Result<Self, EvalError> {
        match v {
            Value::Keyword(k) => Role::from_keyword(k)
                .map(RoleKey)
                .map_err(|_| SpecError::UnknownRole(k.to_string()).at(at)),
            other => Err(SpecError::NotARole {
                found: other.type_name(),
            }
            .at(at)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Form(pub Sexp);

impl IntoValue for Form {
    fn into_value(self) -> Value {
        value_of(&self.0)
    }
}

impl FromValue for Form {
    fn from_value(v: &Value, at: Span) -> Result<Self, EvalError> {
        sexp_of(v).map(Form).map_err(|e| e.at(at))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Typed<T>(pub T);

impl<T: TataraDomain> FromValue for Typed<T> {
    fn from_value(v: &Value, at: Span) -> Result<Self, EvalError> {
        let sexp = sexp_of(v).map_err(|e| e.at(at))?;
        T::compile_from_sexp(&sexp).map(Typed).map_err(|e| {
            SpecError::Compile {
                form: T::KEYWORD,
                message: e.to_string(),
            }
            .at(at)
        })
    }
}

fn atom_value(a: &Atom) -> Value {
    match a {
        Atom::Symbol(s) => Value::Symbol(Arc::from(s.as_str())),
        Atom::Keyword(k) => Value::Keyword(Arc::from(k.as_str())),
        Atom::Str(s) => Value::Str(Arc::from(s.as_str())),
        Atom::Int(n) => Value::Int(*n),
        Atom::Float(x) => Value::Float(*x),
        Atom::Bool(b) => Value::Bool(*b),
    }
}

fn record_value(head: &str, rest: &[Sexp]) -> Option<Value> {
    if !rest.len().is_multiple_of(2) {
        return None;
    }
    let mut entries = vec![(
        MapKey::Keyword(Arc::from(KIND)),
        Value::Keyword(Arc::from(head)),
    )];
    for pair in rest.chunks(2) {
        let Sexp::Atom(Atom::Keyword(k)) = &pair[0] else {
            return None;
        };
        entries.push((MapKey::Keyword(Arc::from(k.as_str())), value_of(&pair[1])));
    }
    Some(Value::map(entries))
}

#[must_use]
pub fn value_of(s: &Sexp) -> Value {
    match s {
        Sexp::Nil => Value::Nil,
        Sexp::Atom(a) => atom_value(a),
        Sexp::List(xs) => match xs.split_first() {
            Some((Sexp::Atom(Atom::Symbol(head)), rest)) => {
                record_value(head, rest).unwrap_or_else(|| Value::list(xs.iter().map(value_of)))
            }
            _ => Value::list(xs.iter().map(value_of)),
        },
        Sexp::Quote(x) | Sexp::Quasiquote(x) | Sexp::Unquote(x) | Sexp::UnquoteSplice(x) => {
            value_of(x)
        }
    }
}

pub fn sexp_of(v: &Value) -> Result<Sexp, SpecError> {
    Ok(match v {
        Value::Nil => Sexp::Nil,
        Value::Bool(b) => Sexp::Atom(Atom::Bool(*b)),
        Value::Int(n) => Sexp::Atom(Atom::Int(*n)),
        Value::Float(x) => Sexp::Atom(Atom::Float(*x)),
        Value::Str(s) => Sexp::string(s.as_ref()),
        Value::Symbol(s) => Sexp::symbol(s.as_ref()),
        Value::Keyword(k) => Sexp::keyword(k.as_ref()),
        Value::List(xs) => Sexp::List(xs.iter().map(sexp_of).collect::<Result<_, _>>()?),
        Value::Map(m) => {
            let kind = match m.get(&MapKey::Keyword(Arc::from(KIND))) {
                Some(Value::Keyword(k)) => k.clone(),
                _ => return Err(SpecError::NoKind),
            };
            let mut entries: Vec<(&str, &Value)> = Vec::with_capacity(m.len());
            for (key, value) in m.iter() {
                match keyword_of(key) {
                    Some(KIND) => {}
                    Some(k) => entries.push((k, value)),
                    None => return Err(SpecError::Unrepresentable("non-keyword key")),
                }
            }
            entries.sort_by(|a, b| a.0.cmp(b.0));
            let mut out = Vec::with_capacity(1 + entries.len() * 2);
            out.push(Sexp::symbol(kind.as_ref()));
            for (k, value) in entries {
                out.push(Sexp::keyword(k));
                out.push(sexp_of(value)?);
            }
            Sexp::List(out)
        }
        other => return Err(SpecError::Unrepresentable(other.type_name())),
    })
}
