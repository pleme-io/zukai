//! Inline HTML / Confluence references — `paragraph`, `hr`, `link`, `page_link`,
//! `mention`. Together with [`crate::panel`] these are the smallest building
//! blocks for a Confluence page body.

use crate::xml;

/// HTML `<p>…</p>` paragraph with XML-text-escaped body.
#[must_use]
pub fn paragraph(body: &str) -> String {
    format!("<p>{}</p>", xml::text(body))
}

/// Horizontal rule.
#[must_use]
pub fn hr() -> String {
    "<hr/>".to_string()
}

/// External link (`<a href="…">…</a>`). URL and text are both escaped.
#[must_use]
pub fn link(text: &str, url: &str) -> String {
    format!(
        r#"<a href="{url}">{text}</a>"#,
        url = xml::attr(url),
        text = xml::text(text),
    )
}

/// Internal Confluence page link by content id. If `link_text` is `Some`,
/// renders as a long-form `<ac:link>` with a `plain-text-link-body`;
/// otherwise the short-form is used and Confluence displays the page title.
#[must_use]
pub fn page_link(content_id: &str, link_text: Option<&str>) -> String {
    match link_text {
        Some(t) => format!(
            r#"<ac:link><ri:page ri:content-id="{id}"/><ac:plain-text-link-body><![CDATA[{t}]]></ac:plain-text-link-body></ac:link>"#,
            id = xml::attr(content_id),
        ),
        None => format!(
            r#"<ac:link><ri:page ri:content-id="{id}"/></ac:link>"#,
            id = xml::attr(content_id),
        ),
    }
}

/// User mention by Atlassian account id.
#[must_use]
pub fn mention(account_id: &str) -> String {
    format!(
        r#"<ac:link><ri:user ri:account-id="{id}"/></ac:link>"#,
        id = xml::attr(account_id),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paragraph_wraps_in_p_with_text_escape() {
        let s = paragraph("hello & world <x>");
        assert_eq!(s, "<p>hello &amp; world &lt;x&gt;</p>");
    }

    #[test]
    fn hr_emits_self_closed_hr() {
        assert_eq!(hr(), "<hr/>");
    }

    #[test]
    fn link_escapes_both_url_and_text() {
        let s = link("docs &c", "https://example.com/?q=a&b=c");
        assert!(s.contains(r#"href="https://example.com/?q=a&amp;b=c""#));
        assert!(s.contains(">docs &amp;c<"));
    }

    #[test]
    fn page_link_with_text_uses_long_form_with_cdata() {
        let s = page_link("4112678914", Some("See Case B demo"));
        assert!(s.contains(r#"ri:content-id="4112678914""#));
        assert!(s.contains("<![CDATA[See Case B demo]]>"));
    }

    #[test]
    fn page_link_without_text_uses_short_form() {
        let s = page_link("12345", None);
        assert!(s.contains(r#"ri:content-id="12345""#));
        assert!(!s.contains("plain-text-link-body"));
    }

    #[test]
    fn mention_carries_account_id() {
        let s = mention("712020:abc-123");
        assert!(s.contains(r#"ri:account-id="712020:abc-123""#));
    }

    #[test]
    fn inline_primitives_are_deterministic() {
        assert_eq!(paragraph("hi"), paragraph("hi"));
        assert_eq!(link("a", "b"), link("a", "b"));
        assert_eq!(page_link("12345", Some("t")), page_link("12345", Some("t")));
        assert_eq!(mention("abc"), mention("abc"));
    }
}
