use std::fmt;

use blue_lang_host::{host, EvalError, FromValue, Span, Value};

use crate::{inline, layout, panel, structure, table as table_mod};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Confluence;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pane {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub done: bool,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairError {
    NotAList { shape: &'static str, found: &'static str },
    Length { shape: &'static str, found: usize },
}

impl fmt::Display for PairError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAList { shape, found } => write!(f, "expected {shape}, got {found}"),
            Self::Length { shape, found } => {
                write!(f, "expected {shape}, got a list of {found}")
            }
        }
    }
}

fn pair(shape: &'static str, v: &Value, at: Span) -> Result<(Value, Value), EvalError> {
    let refuse = |e: PairError| EvalError::native_fn(shape, e.to_string(), at);
    let Value::List(xs) = v else {
        return Err(refuse(PairError::NotAList {
            shape,
            found: v.type_name(),
        }));
    };
    match (xs.len(), xs.get(0), xs.get(1)) {
        (2, Some(a), Some(b)) => Ok((a.clone(), b.clone())),
        (n, _, _) => Err(refuse(PairError::Length { shape, found: n })),
    }
}

impl FromValue for Pane {
    fn from_value(v: &Value, at: Span) -> Result<Self, EvalError> {
        let (title, body) = pair("[title, body]", v, at)?;
        Ok(Self {
            title: String::from_value(&title, at)?,
            body: String::from_value(&body, at)?,
        })
    }
}

impl FromValue for Task {
    fn from_value(v: &Value, at: Span) -> Result<Self, EvalError> {
        let (done, text) = pair("[done, text]", v, at)?;
        Ok(Self {
            done: bool::from_value(&done, at)?,
            text: String::from_value(&text, at)?,
        })
    }
}

#[host(prefix = "cf_")]
impl Confluence {
    fn panel_info(body: String) -> String {
        panel::info(&body)
    }

    fn panel_warning(body: String) -> String {
        panel::warning(&body)
    }

    fn panel_note(body: String) -> String {
        panel::note(&body)
    }

    fn panel_tip(body: String) -> String {
        panel::tip(&body)
    }

    fn panel_success(body: String) -> String {
        panel::success(&body)
    }

    fn expand(title: String, body: String) -> String {
        structure::expand(&title, &body)
    }

    fn code(language: String, body: String) -> String {
        structure::code(&language, &body)
    }

    fn status(color: String, text: String) -> String {
        structure::status(&color, &text)
    }

    fn toc(min_level: u32, max_level: u32) -> String {
        structure::toc(min_level, max_level)
    }

    fn tabs(panes: Vec<Pane>) -> String {
        let refs: Vec<(&str, &str)> = panes
            .iter()
            .map(|p| (p.title.as_str(), p.body.as_str()))
            .collect();
        structure::tabs(&refs)
    }

    fn tasklist(tasks: Vec<Task>) -> String {
        let refs: Vec<(bool, &str)> = tasks.iter().map(|t| (t.done, t.text.as_str())).collect();
        structure::tasklist(&refs)
    }

    fn two_column(left: String, right: String) -> String {
        layout::two_column(&left, &right)
    }

    fn three_column(left: String, middle: String, right: String) -> String {
        layout::three_column(&left, &middle, &right)
    }

    fn paragraph(body: String) -> String {
        inline::paragraph(&body)
    }

    fn hr() -> String {
        inline::hr()
    }

    fn link(text: String, url: String) -> String {
        inline::link(&text, &url)
    }

    fn page_link(content_id: String, link_text: Option<String>) -> String {
        inline::page_link(&content_id, link_text.as_deref())
    }

    fn mention(account_id: String) -> String {
        inline::mention(&account_id)
    }

    fn compose(parts: Vec<String>) -> String {
        parts.concat()
    }

    fn table(header: Vec<String>, rows: Vec<Vec<String>>) -> String {
        let header_refs: Vec<&str> = header.iter().map(String::as_str).collect();
        let row_refs: Vec<Vec<&str>> = rows
            .iter()
            .map(|r| r.iter().map(String::as_str).collect())
            .collect();
        table_mod::table(&header_refs, &row_refs)
    }
}
