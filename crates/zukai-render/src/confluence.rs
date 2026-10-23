//! Confluence delivery helpers. Diagrams render to PNG via [`Png`](crate::Png);
//! this module emits the Confluence storage-format embed snippet that points
//! the page body at the uploaded attachment.
//!
//! The full pipeline (orchestrated by the `diagram-authoring` skill):
//!
//! 1. `Png.render(&diagram, &theme)` → PNG bytes.
//! 2. Atlassian MCP `confluence_upload_attachment(page_id, filename, bytes)`.
//! 3. `confluence::embed(filename, width)` → the `<ac:image>` storage snippet.
//! 4. Atlassian MCP `confluence_update_page(body containing the snippet)`.
//! 5. `curupira` Chrome MCP opens the live page → screenshot → verify.

/// Build the Confluence storage-format snippet that embeds an uploaded
/// attachment as a sized image.
///
/// ```text
/// <ac:image ac:width="840"><ri:attachment ri:filename="diagram.png"/></ac:image>
/// ```
///
/// `filename` must match the attachment filename uploaded to the page;
/// `width` is the display width in pixels (Confluence honors this on `ac:image`).
#[must_use]
pub fn embed(filename: &str, width: u32) -> String {
    format!(
        r#"<ac:image ac:width="{width}"><ri:attachment ri:filename="{}"/></ac:image>"#,
        xml_attr_escape(filename)
    )
}

/// Minimal XML attribute escaping for filenames embedded in storage.
fn xml_attr_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embed_has_attachment_and_width() {
        let s = embed("diagram.png", 840);
        assert!(s.contains(r#"ri:filename="diagram.png""#));
        assert!(s.contains(r#"ac:width="840""#));
        assert!(s.starts_with("<ac:image"));
        assert!(s.ends_with("</ac:image>"));
    }

    #[test]
    fn embed_is_deterministic() {
        assert_eq!(embed("x.png", 840), embed("x.png", 840));
    }

    #[test]
    fn embed_escapes_dangerous_filename_chars() {
        let s = embed(r#"a"b&c<d>.png"#, 100);
        assert!(s.contains("a&quot;b&amp;c&lt;d&gt;.png"));
        // Quotes must not break out of the attribute.
        let opening = s.find(r#"ri:filename=""#).unwrap();
        let inner = &s[opening + r#"ri:filename=""#.len()..];
        let closing = inner.find('"').unwrap();
        // The filename region ends BEFORE any unescaped `"`.
        assert!(!inner[..closing].contains('"'));
    }
}
