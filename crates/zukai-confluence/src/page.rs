//! `Page` — a named Confluence page composed of a title + a body string.
//! The body is the storage-XML produced by the `cf_` blue host functions in
//! [`crate::host`] (or hand-built from the primitive functions). `Page`
//! carries a `TataraDomain` impl, so a page is one form:
//!
//! ```lisp
//! (defpage :title "Architecture hand-over"
//!          :body  "<p>Architecture follows.</p>")
//! ```
//!
//! A blue program builds the same form as a map,
//! `{kind: :defpage, title: …, body: …}`, which `zukai_kata::record::Typed`
//! reads back as a `Page`.

use tatara_lisp::{
    domain::{extract_string, parse_kwargs},
    Result as LResult, Sexp, TataraDomain,
};

/// A named Confluence page — title + storage-XML body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page {
    pub title: String,
    pub body: String,
}

impl Page {
    /// Construct a Page in Rust (mirror of the `.tlisp` `defpage` form).
    #[must_use]
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self { title: title.into(), body: body.into() }
    }
}

impl TataraDomain for Page {
    const KEYWORD: &'static str = "defpage";

    fn compile_from_args(args: &[Sexp]) -> LResult<Self> {
        let kw = parse_kwargs(args)?;
        Ok(Page {
            title: extract_string(&kw, "title")?.to_string(),
            body: extract_string(&kw, "body")?.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tatara_lisp::reader::read;

    #[test]
    fn defpage_compiles_from_lisp() {
        let src = r#"(defpage :title "Hand-Over" :body "<p>body</p>")"#;
        let forms = read(src).unwrap();
        let p = Page::compile_from_sexp(forms.first().unwrap()).unwrap();
        assert_eq!(p.title, "Hand-Over");
        assert_eq!(p.body, "<p>body</p>");
    }

    #[test]
    fn page_round_trip_via_constructor() {
        let p = Page::new("t", "<p>b</p>");
        assert_eq!(p, Page::new("t", "<p>b</p>"));
    }
}
