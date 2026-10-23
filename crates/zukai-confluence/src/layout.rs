//! Confluence page layout macros — multi-column page sections.
//!
//! Confluence's `<ac:layout>` lets a page body be split into columns; cells
//! receive arbitrary storage XML (panels / diagrams / text / tables).

/// Two-column equal-width layout.
#[must_use]
pub fn two_column(left: &str, right: &str) -> String {
    format!(
        r#"<ac:layout><ac:layout-section ac:type="two_equal"><ac:layout-cell>{left}</ac:layout-cell><ac:layout-cell>{right}</ac:layout-cell></ac:layout-section></ac:layout>"#,
    )
}

/// Three-column equal-width layout.
#[must_use]
pub fn three_column(left: &str, middle: &str, right: &str) -> String {
    format!(
        r#"<ac:layout><ac:layout-section ac:type="three_equal"><ac:layout-cell>{left}</ac:layout-cell><ac:layout-cell>{middle}</ac:layout-cell><ac:layout-cell>{right}</ac:layout-cell></ac:layout-section></ac:layout>"#,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_column_has_two_cells_and_two_equal_section_type() {
        let s = two_column("<p>L</p>", "<p>R</p>");
        assert!(s.contains(r#"ac:type="two_equal""#));
        assert_eq!(s.matches("<ac:layout-cell>").count(), 2);
        assert!(s.contains("<p>L</p>"));
        assert!(s.contains("<p>R</p>"));
    }

    #[test]
    fn three_column_has_three_cells_and_three_equal_section_type() {
        let s = three_column("L", "M", "R");
        assert!(s.contains(r#"ac:type="three_equal""#));
        assert_eq!(s.matches("<ac:layout-cell>").count(), 3);
    }

    #[test]
    fn cell_content_is_verbatim_no_escape() {
        // Cells may contain other macros (incl. diagram embeds) — those
        // are valid storage XML, not text. Don't escape.
        let s = two_column(
            r#"<ac:image ac:width="840"><ri:attachment ri:filename="x.png"/></ac:image>"#,
            "<p>desc</p>",
        );
        assert!(s.contains("<ac:image ac:width=\"840\">"));
        assert!(s.contains("<p>desc</p>"));
    }

    #[test]
    fn is_deterministic() {
        assert_eq!(two_column("a", "b"), two_column("a", "b"));
        assert_eq!(three_column("a", "b", "c"), three_column("a", "b", "c"));
    }
}
