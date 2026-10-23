//! Compose a full Confluence page body entirely from `zukai-confluence`
//! primitives + a single diagram embed. The body is written to
//! `/tmp/zukai-conf-demo-body.xml` for a publish script to pick up.
//!
//! Run: `cargo run --example page_demo -p zukai-confluence`

use std::fs;
use zukai_confluence::{
    code, expand, hr, info, layout, link, note, paragraph, status, success, table, tabs,
    tasklist, tip, toc, warning,
};

fn main() {
    let body = build_body();
    let path = "/tmp/zukai-conf-demo-body.xml";
    fs::write(path, &body).expect("write body");
    println!("zukai-confluence page body composed:");
    println!("  {} bytes → {path}", body.len());
}

fn build_body() -> String {
    let mut b = String::new();

    // Intro + auto TOC.
    b.push_str(&paragraph(
        "This page is composed entirely from zukai-confluence primitives — no \
         hand-authored storage XML. Each section below is rendered by a Rust \
         function call from the new crate.",
    ));
    b.push_str(&toc(2, 3));

    // 1 — panels.
    b.push_str("<h2>1 · Panel primitives</h2>");
    b.push_str(&info("Info panel — for context the reader needs."));
    b.push_str(&warning("Warning panel — for risks before they act."));
    b.push_str(&note("Note panel — for tangential context."));
    b.push_str(&tip("Tip panel — for shortcuts and best practices."));
    b.push_str(&success("Success panel — for state changes confirmed."));

    // 2 — inline primitives.
    b.push_str("<h2>2 · Inline primitives</h2>");
    b.push_str(&format!(
        "<p>External link: {}.</p>",
        link("milestone doc", "https://example.com/milestone")
    ));
    b.push_str(&format!(
        "<p>Inline status chips: {} {} {}.</p>",
        status("Grey", "DRAFT"),
        status("Yellow", "IN REVIEW"),
        status("Green", "DONE"),
    ));
    b.push_str(&hr());

    // 3 — code block.
    b.push_str("<h2>3 · Code block (CDATA, no escape)</h2>");
    b.push_str(&code(
        "rust",
        "fn render_themed(d: &Diagram, t: &dyn Theme) -> Vec<u8> {\n    Png.render(d, t)\n}",
    ));

    // 4 — expand containing a tasklist.
    b.push_str("<h2>4 · Expand (collapsible) — wrapping a tasklist</h2>");
    b.push_str(&expand(
        "Click to expand the v0.1 checklist",
        &tasklist(&[
            (true, "ship 17 kata patterns"),
            (true, "multi_region v0 → v1 (N regions)"),
            (true, "from_mermaid (Case B helper)"),
            (true, "confluence::embed snippet"),
            (true, "three live Confluence proof pages"),
            (true, "zukai-confluence crate (this page proves it)"),
            (false, "more renderer targets (Graphviz / D2 / Excalidraw)"),
            (false, "pleme-io-inspired kata patterns (controller_loop, …)"),
            (false, "page templates (runbook, ADR, postmortem)"),
        ]),
    ));

    // 5 — tabs.
    b.push_str("<h2>5 · Tabs</h2>");
    b.push_str(&tabs(&[
        ("Overview", "<p>What the system does, at a glance.</p>"),
        (
            "Architecture",
            "<p>How it's wired up — see the diagram below.</p>",
        ),
        ("Operations", "<p>Runbooks, alerts, on-call rotations.</p>"),
    ]));

    // 6 — table.
    b.push_str("<h2>6 · Table</h2>");
    b.push_str(&table(
        &["Pattern", "Returns", "Use"],
        &[
            vec!["hero", "Hero", "title banner"],
            vec!["multi_region", "Diagram", "primary + N replicas"],
            vec!["pillars", "Vec<Node>", "N-column foundation"],
            vec!["from_mermaid", "Result<Diagram>", "Case B re-theming"],
        ],
    ));

    // 7 — two-column layout pairing two panels side-by-side.
    b.push_str("<h2>7 · Two-column layout</h2>");
    b.push_str(&layout::two_column(
        &info("Left cell — structured info about the topic."),
        &tip("Right cell — a tip the reader needs near the info."),
    ));

    // 8 — diagram embed (the bridge to zukai-render::confluence::embed shape).
    b.push_str("<h2>8 · Diagram embed (kata-rendered PNG)</h2>");
    b.push_str(&paragraph(
        "The next image is the kata-authored multi_region diagram from the v0.1 milestone.",
    ));
    b.push_str(
        r#"<ac:image ac:width="840"><ri:attachment ri:filename="multi-region.png"/></ac:image>"#,
    );

    b.push_str(&hr());
    b.push_str(&paragraph(
        "Every primitive used on this page lives in zukai-confluence. \
         No hand-authored storage XML. Spec is the source.",
    ));
    b
}
