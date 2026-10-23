//! Minimal XML escaping. Single source of truth for the rest of the crate.

/// Escape for use inside an XML attribute value (assumes double-quoted).
#[must_use]
pub fn attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Escape for use as XML text content.
#[must_use]
pub fn text(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attr_escapes_quotes_and_angle_brackets_and_amp() {
        let s = attr(r#"<a href="x">&"#);
        assert_eq!(s, "&lt;a href=&quot;x&quot;&gt;&amp;");
    }

    #[test]
    fn text_does_not_escape_quotes() {
        let s = text("hello \"world\"");
        assert_eq!(s, "hello \"world\"");
    }

    #[test]
    fn text_escapes_angle_brackets_and_amp() {
        assert_eq!(text("<script>&amp"), "&lt;script&gt;&amp;amp");
    }

    #[test]
    fn empty_in_empty_out() {
        assert_eq!(attr(""), "");
        assert_eq!(text(""), "");
    }

    #[test]
    fn idempotent_on_already_escaped() {
        // Already-escaped strings get escaped AGAIN — by design (the input is
        // a literal, not pre-escaped). Caller's responsibility to not pre-escape.
        let once = text("a&b");
        let twice = text(&once);
        assert_eq!(once, "a&amp;b");
        assert_eq!(twice, "a&amp;amp;b");
    }
}
