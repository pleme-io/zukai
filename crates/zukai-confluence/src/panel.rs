//! Confluence panel macros — `info` / `warning` / `note` / `tip` / `success`.
//! Each emits the standard structured-macro form:
//!
//! ```text
//! <ac:structured-macro ac:name="<variant>">
//!   <ac:rich-text-body><p>…text-escaped body…</p></ac:rich-text-body>
//! </ac:structured-macro>
//! ```

use crate::xml;

fn panel(variant: &str, body: &str) -> String {
    format!(
        r#"<ac:structured-macro ac:name="{variant}"><ac:rich-text-body><p>{body}</p></ac:rich-text-body></ac:structured-macro>"#,
        body = xml::text(body),
    )
}

/// Info panel (Atlassian default: blue).
#[must_use]
pub fn info(body: &str) -> String { panel("info", body) }

/// Warning panel (Atlassian default: amber/red).
#[must_use]
pub fn warning(body: &str) -> String { panel("warning", body) }

/// Note panel (Atlassian default: purple).
#[must_use]
pub fn note(body: &str) -> String { panel("note", body) }

/// Tip panel (Atlassian default: green).
#[must_use]
pub fn tip(body: &str) -> String { panel("tip", body) }

/// Success / check panel (Atlassian default: green, formal).
#[must_use]
pub fn success(body: &str) -> String { panel("success", body) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_use_distinct_macro_names() {
        assert!(info("x").contains(r#"ac:name="info""#));
        assert!(warning("x").contains(r#"ac:name="warning""#));
        assert!(note("x").contains(r#"ac:name="note""#));
        assert!(tip("x").contains(r#"ac:name="tip""#));
        assert!(success("x").contains(r#"ac:name="success""#));
    }

    #[test]
    fn body_is_xml_text_escaped() {
        let s = info("<script>alert('x')</script>");
        assert!(!s.contains("<script>"));
        assert!(s.contains("&lt;script&gt;"));
    }

    #[test]
    fn all_variants_wrap_body_in_paragraph() {
        for s in [info("x"), warning("x"), note("x"), tip("x"), success("x")] {
            assert!(s.contains("<p>x</p>"));
            assert!(s.contains("<ac:rich-text-body>"));
        }
    }

    #[test]
    fn is_deterministic() {
        assert_eq!(info("hello"), info("hello"));
        assert_eq!(warning("x"), warning("x"));
    }
}
