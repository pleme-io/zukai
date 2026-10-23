use blue_lang_host::{Entry, Error, Host, NoLoader, Program, Value};
use zukai_confluence::host::Confluence;

const PAGES: &str = include_str!("pages.b");

fn call(name: &str) -> String {
    let mut host = Confluence;
    let mut program = Program::load_with(Entry::anonymous(PAGES), &NoLoader, &mut host)
        .expect("pages.b loads");
    program.call(name, (), &mut host).expect("the definition runs")
}

#[test]
fn the_surface_is_every_confluence_primitive_and_compose() {
    let names: Vec<&str> = Confluence::SURFACE.iter().map(|f| f.name).collect();
    assert_eq!(
        names,
        [
            "cf_panel_info",
            "cf_panel_warning",
            "cf_panel_note",
            "cf_panel_tip",
            "cf_panel_success",
            "cf_expand",
            "cf_code",
            "cf_status",
            "cf_toc",
            "cf_tabs",
            "cf_tasklist",
            "cf_two_column",
            "cf_three_column",
            "cf_paragraph",
            "cf_hr",
            "cf_link",
            "cf_page_link",
            "cf_mention",
            "cf_compose",
            "cf_table",
        ]
    );
}

#[test]
fn panel_info() {
    let s = call("info_panel");
    assert!(s.contains(r#"ac:name="info""#));
    assert!(s.contains("Read this first."));
}

#[test]
fn all_five_panels_dispatch_distinctly() {
    let mut host = Confluence;
    let mut program =
        Program::load_with(Entry::anonymous(PAGES), &NoLoader, &mut host).expect("pages.b loads");
    let panels: Vec<String> = program.call("every_panel", (), &mut host).expect("runs");
    for (s, name) in panels
        .iter()
        .zip(["info", "warning", "note", "tip", "success"])
    {
        assert!(s.contains(&format!(r#"ac:name="{name}""#)), "{name}: {s}");
    }
}

#[test]
fn code_uses_cdata() {
    let s = call("rust_code");
    assert!(s.contains("<![CDATA[fn x() {}]]>"));
    assert!(s.contains(r#"<ac:parameter ac:name="language">rust</ac:parameter>"#));
}

#[test]
fn expand_with_inline_macro_composition() {
    let s = call("expand_with_tip");
    assert!(s.contains(r#"ac:name="expand""#));
    assert!(s.contains(r#"ac:name="tip""#));
    assert!(s.contains("use the API token"));
}

#[test]
fn status_carries_color_and_title() {
    let s = call("green_status");
    assert!(s.contains(r#"<ac:parameter ac:name="colour">Green</ac:parameter>"#));
    assert!(s.contains(r#"<ac:parameter ac:name="title">OK</ac:parameter>"#));
}

#[test]
fn toc_handles_integer_args() {
    let s = call("toc_two_to_three");
    assert!(s.contains(r#"<ac:parameter ac:name="minLevel">2</ac:parameter>"#));
    assert!(s.contains(r#"<ac:parameter ac:name="maxLevel">3</ac:parameter>"#));
}

#[test]
fn tabs_with_pair_list() {
    let s = call("two_tabs");
    assert!(s.contains(r#"ac:name="tabs""#));
    assert_eq!(s.matches(r#"ac:name="tab""#).count(), 2);
    assert!(s.contains("Overview"));
    assert!(s.contains("Details"));
}

#[test]
fn tasklist_complete_and_incomplete() {
    let s = call("done_and_todo");
    assert!(s.contains("<ac:task-status>complete</ac:task-status>"));
    assert!(s.contains("<ac:task-status>incomplete</ac:task-status>"));
}

#[test]
fn two_column_layout() {
    let s = call("two_columns");
    assert!(s.contains(r#"ac:type="two_equal""#));
    assert_eq!(s.matches("<ac:layout-cell>").count(), 2);
}

#[test]
fn paragraph_hr_link() {
    assert!(call("hello").contains("<p>hello</p>"));
    assert_eq!(call("divider"), "<hr/>");
    let s = call("docs_link");
    assert!(s.contains(r#"href="https://example.com""#));
    assert!(s.contains(">docs<"));
}

#[test]
fn page_link_with_and_without_text() {
    let with = call("titled_page_link");
    assert!(with.contains(r#"ri:content-id="4112318475""#));
    assert!(with.contains("<![CDATA[See demo]]>"));
    let without = call("bare_page_link");
    assert!(without.contains(r#"ri:content-id="12345""#));
    assert!(!without.contains("plain-text-link-body"));
}

#[test]
fn table_renders_header_plus_body() {
    let s = call("service_table");
    assert!(s.contains("<th>Service</th>"));
    assert!(s.contains("<th>IP</th>"));
    assert!(s.contains("<td>vault</td>"));
    assert!(s.contains("<td>5.6.7.8</td>"));
    assert_eq!(s.matches("<tr>").count(), 3);
}

#[test]
fn compose_concatenates_in_order() {
    let s = call("ordered_body");
    let info_pos = s.find(r#"ac:name="info""#).expect("info");
    let hr_pos = s.find("<hr/>").expect("hr");
    let tip_pos = s.find(r#"ac:name="tip""#).expect("tip");
    assert!(info_pos < hr_pos && hr_pos < tip_pos, "compose preserves order");
}

#[test]
fn compose_with_typed_string_args_only() {
    let mut host = Confluence;
    let mut program =
        Program::load_with(Entry::anonymous(PAGES), &NoLoader, &mut host).expect("pages.b loads");
    let parts = vec![Value::from("a"), Value::from(42i64)];
    let err = program
        .call::<String>("body_of", (parts,), &mut host)
        .unwrap_err();
    assert!(matches!(err, Error::Call(_)), "{err}");
    let msg = err.to_string();
    assert!(
        msg.contains("cf_compose") && msg.contains("argument 1 (parts)"),
        "{msg}"
    );
}

#[test]
fn a_malformed_pane_names_its_shape() {
    let mut host = Confluence;
    let src = "def bad()\n  cf_tabs([[\"only a title\"]])\nend\n";
    let mut program =
        Program::load_with(Entry::anonymous(src), &NoLoader, &mut host).expect("loads");
    let msg = program
        .call::<String>("bad", (), &mut host)
        .unwrap_err()
        .to_string();
    assert!(msg.contains("[title, body]") && msg.contains("a list of 1"), "{msg}");
}

#[test]
fn deep_nested_composition() {
    let s = call("nested_layout");
    assert!(s.contains(r#"ac:type="two_equal""#));
    assert!(s.contains(r#"ac:name="expand""#));
    assert!(s.contains(r#"ac:name="tip""#));
    assert!(s.contains(r#"ac:name="tabs""#));
    assert!(s.contains(r#"ac:name="code""#));
    assert!(s.contains("<![CDATA[Png.render(&d, &theme)]]>"));
    assert!(s.contains("<ac:task-status>complete</ac:task-status>"));
    assert!(s.contains("<ac:task-status>incomplete</ac:task-status>"));
}
