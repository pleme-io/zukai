//! Non-panel structural macros: `expand`, `code`, `toc`, `tabs`, `tasklist`, `status`.

use crate::xml;

/// Collapsible expand macro. `body` is inserted verbatim — caller composes
/// other macros / HTML in.
#[must_use]
pub fn expand(title: &str, body: &str) -> String {
    format!(
        r#"<ac:structured-macro ac:name="expand"><ac:parameter ac:name="title">{title}</ac:parameter><ac:rich-text-body>{body}</ac:rich-text-body></ac:structured-macro>"#,
        title = xml::attr(title),
    )
}

/// Code block with syntax highlighting. `body` is wrapped in CDATA — caller
/// must NOT pre-escape (escape would appear literally in the rendered code).
#[must_use]
pub fn code(language: &str, body: &str) -> String {
    format!(
        r#"<ac:structured-macro ac:name="code"><ac:parameter ac:name="language">{lang}</ac:parameter><ac:plain-text-body><![CDATA[{body}]]></ac:plain-text-body></ac:structured-macro>"#,
        lang = xml::attr(language),
    )
}

/// Auto table-of-contents macro spanning heading levels `[min_level, max_level]`.
#[must_use]
pub fn toc(min_level: u32, max_level: u32) -> String {
    format!(
        r#"<ac:structured-macro ac:name="toc"><ac:parameter ac:name="minLevel">{min_level}</ac:parameter><ac:parameter ac:name="maxLevel">{max_level}</ac:parameter></ac:structured-macro>"#,
    )
}

/// Tabbed content. Each `(title, body)` becomes a tab; bodies are inserted
/// verbatim (caller composes other macros / HTML).
#[must_use]
pub fn tabs(panes: &[(&str, &str)]) -> String {
    let body = panes
        .iter()
        .map(|(title, body)| {
            format!(
                r#"<ac:structured-macro ac:name="tab"><ac:parameter ac:name="title">{title}</ac:parameter><ac:rich-text-body>{body}</ac:rich-text-body></ac:structured-macro>"#,
                title = xml::attr(title),
            )
        })
        .collect::<String>();
    format!(
        r#"<ac:structured-macro ac:name="tabs"><ac:rich-text-body>{body}</ac:rich-text-body></ac:structured-macro>"#,
    )
}

/// Task list. Each `(checked, text)` is one task (text is XML-text-escaped).
#[must_use]
pub fn tasklist(tasks: &[(bool, &str)]) -> String {
    let items = tasks
        .iter()
        .map(|(checked, text)| {
            let s = if *checked { "complete" } else { "incomplete" };
            format!(
                r#"<ac:task><ac:task-status>{s}</ac:task-status><ac:task-body>{t}</ac:task-body></ac:task>"#,
                t = xml::text(text),
            )
        })
        .collect::<String>();
    format!("<ac:task-list>{items}</ac:task-list>")
}

/// Inline status chip. `color` is one of: `Grey`, `Red`, `Yellow`, `Green`, `Blue`, `Purple`.
#[must_use]
pub fn status(color: &str, text: &str) -> String {
    format!(
        r#"<ac:structured-macro ac:name="status"><ac:parameter ac:name="colour">{color}</ac:parameter><ac:parameter ac:name="title">{text}</ac:parameter></ac:structured-macro>"#,
        color = xml::attr(color),
        text = xml::attr(text),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_carries_title_and_body_verbatim() {
        let s = expand("Details", "<p>hi</p>");
        assert!(s.contains(r#"<ac:parameter ac:name="title">Details</ac:parameter>"#));
        assert!(s.contains("<p>hi</p>"));
    }

    #[test]
    fn expand_escapes_title_in_attribute() {
        let s = expand(r#"q"u"#, "x");
        assert!(s.contains("q&quot;u"));
    }

    #[test]
    fn code_uses_cdata_for_body_no_escape() {
        let s = code("rust", "fn x() {}");
        assert!(s.contains("<![CDATA[fn x() {}]]>"));
        assert!(s.contains(r#"ac:name="code""#));
    }

    #[test]
    fn code_escapes_language_in_attribute() {
        let s = code(r#"a"b"#, "x");
        assert!(s.contains("a&quot;b"));
    }

    #[test]
    fn toc_has_min_and_max_levels() {
        let s = toc(1, 3);
        assert!(s.contains(r#"<ac:parameter ac:name="minLevel">1</ac:parameter>"#));
        assert!(s.contains(r#"<ac:parameter ac:name="maxLevel">3</ac:parameter>"#));
    }

    #[test]
    fn tabs_emits_outer_macro_and_one_inner_per_pane() {
        let s = tabs(&[("Overview", "<p>over</p>"), ("Details", "<p>det</p>")]);
        assert!(s.contains(r#"ac:name="tabs""#));
        assert_eq!(s.matches(r#"ac:name="tab""#).count(), 2);
        assert!(s.contains("Overview"));
        assert!(s.contains("Details"));
    }

    #[test]
    fn tasklist_emits_complete_and_incomplete_states() {
        let s = tasklist(&[(true, "done"), (false, "todo")]);
        assert!(s.contains("<ac:task-status>complete</ac:task-status>"));
        assert!(s.contains("<ac:task-status>incomplete</ac:task-status>"));
        assert!(s.contains("done"));
        assert!(s.contains("todo"));
    }

    #[test]
    fn status_carries_color_and_title_attributes() {
        let s = status("Green", "OK");
        assert!(s.contains(r#"ac:name="status""#));
        assert!(s.contains(r#"<ac:parameter ac:name="colour">Green</ac:parameter>"#));
        assert!(s.contains(r#"<ac:parameter ac:name="title">OK</ac:parameter>"#));
    }

    #[test]
    fn structure_primitives_are_deterministic() {
        assert_eq!(expand("t", "b"), expand("t", "b"));
        assert_eq!(code("r", "x"), code("r", "x"));
        assert_eq!(toc(1, 3), toc(1, 3));
        assert_eq!(status("Green", "OK"), status("Green", "OK"));
        assert_eq!(
            tasklist(&[(true, "a"), (false, "b")]),
            tasklist(&[(true, "a"), (false, "b")])
        );
        assert_eq!(
            tabs(&[("a", "x"), ("b", "y")]),
            tabs(&[("a", "x"), ("b", "y")])
        );
    }
}
