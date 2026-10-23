//! Mermaid (`flowchart TD`) backend. Theme-driven `classDef` styling.
//!
//! Mermaid is *structural* (nodes + edges, no coordinates), so this target
//! emits regions as subgraphs and nodes as classed boxes. Mermaid edges need
//! node-to-node refs; zukai's current `Edge` is point-to-point (a layout
//! concern), so edges aren't emitted in v0 — that's a tracked compound (see
//! `DESIGN.md`: add `Anchor::Node(Id)` to the model). Like SVG, Mermaid is a
//! leaf text DSL, so `format!`-style string building is idiomatic here.

use crate::{Renderer, Target, Theme};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use zukai_model::{Diagram, Id, Node, Role};

#[derive(Debug, Clone, Copy, Default)]
pub struct Mermaid;

impl Renderer for Mermaid {
    fn target(&self) -> Target {
        Target::Mermaid
    }

    fn render(&self, d: &Diagram, t: &dyn Theme) -> Vec<u8> {
        let mut s = String::new();
        s.push_str("flowchart TD\n");
        let _ = writeln!(s, "  %% {}", strip_newlines(&d.hero.title));

        // Bucket nodes by their region for clean subgraph emission.
        let mut by_region: BTreeMap<&str, Vec<&Node>> = BTreeMap::new();
        let mut unassigned: Vec<&Node> = Vec::new();
        for n in &d.nodes {
            match &n.region {
                Some(r) => by_region.entry(r.as_str()).or_default().push(n),
                None => unassigned.push(n),
            }
        }

        for r in &d.regions {
            let _ = writeln!(s, "  subgraph {}[\"{}\"]", sanitize(&r.id), esc_label(&r.label));
            if let Some(ns) = by_region.remove(r.id.as_str()) {
                for n in ns {
                    let _ = writeln!(
                        s,
                        "    {}[\"{}\"]:::{}",
                        sanitize(&n.id),
                        esc_label(&n.label),
                        role_class(n.role)
                    );
                }
            }
            let _ = writeln!(s, "  end");
            let _ = writeln!(s, "  class {} {}", sanitize(&r.id), role_class(r.role));
        }
        for n in unassigned {
            let _ = writeln!(
                s,
                "  {}[\"{}\"]:::{}",
                sanitize(&n.id),
                esc_label(&n.label),
                role_class(n.role)
            );
        }

        // Theme-sourced classDefs — one per Role. Emitting all keeps output
        // deterministic regardless of which roles appear in the spec.
        for role in [
            Role::Neutral, Role::Master, Role::Read, Role::Accent,
            Role::Success, Role::Warning, Role::Danger, Role::Info,
        ] {
            let _ = writeln!(
                s,
                "  classDef {} fill:{},stroke:{},color:{}",
                role_class(role),
                t.panel(),
                t.role(role),
                t.fg()
            );
        }
        s.into_bytes()
    }
}

fn role_class(r: Role) -> &'static str {
    match r {
        Role::Neutral => "neutral",
        Role::Master => "master",
        Role::Read => "read",
        Role::Accent => "accent",
        Role::Success => "success",
        Role::Warning => "warning",
        Role::Danger => "danger",
        Role::Info => "info",
    }
}

/// Mermaid node IDs must be alphanumeric / underscore.
fn sanitize(id: &Id) -> String {
    id.as_str()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect()
}

fn esc_label(s: &str) -> String {
    s.replace('"', "''")
}

fn strip_newlines(s: &str) -> String {
    s.replace('\n', " ")
}
