//! Simple HTML table builder for Confluence pages. Confluence renders
//! standard HTML tables natively; [`table`] produces
//! `<table><tbody><tr>…<th>…</th></tr><tr>…<td>…</td></tr></tbody></table>`
//! with header-row support.

use crate::xml;

/// Build a table from a header row + body rows. Each cell is XML-text-escaped.
///
/// Each body row must have the same number of cells as the header (caller's
/// responsibility — not enforced, but mis-shapen tables render badly).
#[must_use]
pub fn table(header: &[&str], rows: &[Vec<&str>]) -> String {
    let header_cells = header
        .iter()
        .map(|h| format!("<th>{}</th>", xml::text(h)))
        .collect::<String>();
    let body_rows = rows
        .iter()
        .map(|row| {
            let cells = row
                .iter()
                .map(|c| format!("<td>{}</td>", xml::text(c)))
                .collect::<String>();
            format!("<tr>{cells}</tr>")
        })
        .collect::<String>();
    format!(
        r#"<table><tbody><tr>{header_cells}</tr>{body_rows}</tbody></table>"#,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_has_header_row_plus_body_rows() {
        let t = table(&["A", "B"], &[vec!["1", "2"], vec!["3", "4"]]);
        assert!(t.contains("<th>A</th>"));
        assert!(t.contains("<th>B</th>"));
        // 1 header row + 2 body rows = 3 <tr> total
        assert_eq!(t.matches("<tr>").count(), 3);
        assert!(t.contains("<td>1</td>"));
        assert!(t.contains("<td>4</td>"));
    }

    #[test]
    fn cells_are_xml_text_escaped() {
        let t = table(&["<script>"], &[vec!["a&b"]]);
        assert!(t.contains("<th>&lt;script&gt;</th>"));
        assert!(t.contains("<td>a&amp;b</td>"));
    }

    #[test]
    fn empty_table_still_well_formed() {
        let t = table(&[], &[]);
        // Empty header row + no body rows.
        assert!(t.contains("<tr></tr>"));
        assert!(t.starts_with("<table>"));
        assert!(t.ends_with("</table>"));
    }

    #[test]
    fn is_deterministic() {
        let a = table(&["A"], &[vec!["1"]]);
        let b = table(&["A"], &[vec!["1"]]);
        assert_eq!(a, b);
    }
}
