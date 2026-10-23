//! `kata::from_mermaid` — **Case B helper**. Parse a simple Mermaid
//! `flowchart` block into a [`Diagram`] ready to render through the same
//! `Theme` seam as kata-authored diagrams. The "re-theme an existing
//! Mermaid block" path.
//!
//! ## v0 surface
//!
//! Supported:
//! - Header: `flowchart TD|BT|LR|RL` or `graph TD|BT|LR|RL`.
//! - Node decls: `A[Label]` or `A(Label)`.
//! - Edges (one per line): `A --> B` or `A -- label --> B`.
//! - Implicit nodes (referenced in edges, never declared) — id is reused
//!   as the visible label.
//! - Comments (`%% …`) and unrecognized lines are silently skipped, which
//!   lets Mermaid with subgraphs / classDefs / styles parse partially.
//!
//! Not yet supported (v1 follow-ups): chained edges on one line
//! (`A --> B --> C`), subgraphs, classDefs, link styling, alternate edge
//! glyphs (`==>`, `-.->`, etc.).

use std::collections::BTreeMap;
use zukai_model::{Canvas, Diagram, Edge, Hero, Id, Marker, Node, Point, Rect, Role, Stroke};

const NODE_W: u32 = 160;
const NODE_H: u32 = 50;
const NODE_GAP: u32 = 40;
const CANVAS_PAD: u32 = 40;

/// Parse / validation error from [`from_mermaid`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FromMermaidError {
    Empty,
    NoFlowchartHeader,
    InvalidDirection(String),
}

impl std::fmt::Display for FromMermaidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "input is empty"),
            Self::NoFlowchartHeader => {
                write!(f, "no `flowchart` or `graph` header found")
            }
            Self::InvalidDirection(d) => write!(f, "invalid flowchart direction: {d}"),
        }
    }
}

impl std::error::Error for FromMermaidError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    TD,
    BT,
    LR,
    RL,
}

/// Parse a simple Mermaid `flowchart` source into a renderable [`Diagram`].
pub fn from_mermaid(src: &str) -> Result<Diagram, FromMermaidError> {
    if src.trim().is_empty() {
        return Err(FromMermaidError::Empty);
    }
    let (direction, node_labels, edges) = parse(src)?;
    Ok(lay_out(direction, &node_labels, &edges))
}

type Parsed = (
    Direction,
    BTreeMap<String, String>,
    Vec<(String, Option<String>, String)>,
);

fn parse(src: &str) -> Result<Parsed, FromMermaidError> {
    let mut direction: Option<Direction> = None;
    let mut node_labels: BTreeMap<String, String> = BTreeMap::new();
    let mut edges: Vec<(String, Option<String>, String)> = Vec::new();

    for raw in src.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with("%%") {
            continue;
        }
        if line.starts_with("flowchart ") || line.starts_with("graph ") {
            let dir = line.split_whitespace().nth(1).unwrap_or("TD").trim();
            direction = Some(match dir {
                "TD" | "TB" => Direction::TD,
                "BT" => Direction::BT,
                "LR" => Direction::LR,
                "RL" => Direction::RL,
                other => return Err(FromMermaidError::InvalidDirection(other.into())),
            });
            continue;
        }
        if line.contains(" --> ") {
            // Extract any inline node-label decls (`A[Label]`, `B(Label)`) that
            // appear in the same line as the edge BEFORE registering id-as-label
            // fallbacks via `or_insert_with`.
            for (id, label) in extract_inline_decls(line) {
                node_labels.insert(id, label);
            }
            if let Some((from, label, to)) = parse_edge(line) {
                node_labels.entry(from.clone()).or_insert_with(|| from.clone());
                node_labels.entry(to.clone()).or_insert_with(|| to.clone());
                edges.push((from, label, to));
            }
            continue;
        }
        if let Some((id, label)) = parse_node_decl(line) {
            node_labels.insert(id, label);
            continue;
        }
        // Unrecognized line — silently skip so partial parse succeeds.
    }
    let direction = direction.ok_or(FromMermaidError::NoFlowchartHeader)?;
    Ok((direction, node_labels, edges))
}

fn parse_edge(line: &str) -> Option<(String, Option<String>, String)> {
    let arrow_idx = line.find(" --> ")?;
    let before = line[..arrow_idx].trim();
    let after = line[arrow_idx + 5..].trim();
    let to = first_token(after)?;
    // Inline label: "A -- label --> B" — `before` is "A -- label".
    if let Some(dd) = before.find(" -- ") {
        let from = first_token(before[..dd].trim())?;
        let label = before[dd + 4..].trim().to_string();
        return Some((from, Some(label), to));
    }
    let from = first_token(before)?;
    Some((from, None, to))
}

/// Pull every `id[label]` or `id(label)` decl out of one edge line. Splits
/// around the edge tokens (`" --> "`, `" -- "`) and runs `parse_node_decl`
/// on each fragment.
fn extract_inline_decls(line: &str) -> Vec<(String, String)> {
    line.split(" --> ")
        .flat_map(|s| s.split(" -- "))
        .filter_map(|s| parse_node_decl(s.trim()))
        .collect()
}

fn parse_node_decl(line: &str) -> Option<(String, String)> {
    for (open_ch, close_ch) in &[('[', ']'), ('(', ')')] {
        if let Some(open) = line.find(*open_ch) {
            if let Some(close) = line.rfind(*close_ch) {
                if close > open {
                    let id = line[..open].trim().to_string();
                    let label = line[open + 1..close].to_string();
                    if !id.is_empty() {
                        return Some((id, label));
                    }
                }
            }
        }
    }
    None
}

fn first_token(s: &str) -> Option<String> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let end = s
        .find(|c: char| c == '[' || c == '(' || c.is_whitespace())
        .unwrap_or(s.len());
    if end == 0 {
        None
    } else {
        Some(s[..end].to_string())
    }
}

fn lay_out(
    direction: Direction,
    node_labels: &BTreeMap<String, String>,
    edges_in: &[(String, Option<String>, String)],
) -> Diagram {
    let n = node_labels.len();
    if n == 0 {
        let canvas = Canvas { width: 200, height: 100 };
        let h = Hero { title: "(empty)".into(), subtitle: None };
        return Diagram::new(canvas, h);
    }
    let horizontal = matches!(direction, Direction::LR | Direction::RL);
    let reverse = matches!(direction, Direction::RL | Direction::BT);

    let canvas_w = if horizontal {
        (n as u32) * NODE_W + ((n as u32).saturating_sub(1)) * NODE_GAP + 2 * CANVAS_PAD
    } else {
        NODE_W + 2 * CANVAS_PAD
    };
    let canvas_h = if horizontal {
        NODE_H + 2 * CANVAS_PAD
    } else {
        (n as u32) * NODE_H + ((n as u32).saturating_sub(1)) * NODE_GAP + 2 * CANVAS_PAD
    };
    let canvas = Canvas { width: canvas_w, height: canvas_h };
    let h = Hero { title: "from_mermaid".into(), subtitle: None };

    let pos: BTreeMap<String, (i32, i32)> = node_labels
        .keys()
        .enumerate()
        .map(|(i, id)| {
            let i = if reverse { n - 1 - i } else { i };
            let x = if horizontal {
                CANVAS_PAD as i32 + (i as i32) * (NODE_W + NODE_GAP) as i32
            } else {
                CANVAS_PAD as i32
            };
            let y = if horizontal {
                CANVAS_PAD as i32
            } else {
                CANVAS_PAD as i32 + (i as i32) * (NODE_H + NODE_GAP) as i32
            };
            (id.clone(), (x, y))
        })
        .collect();

    let mut d = Diagram::new(canvas, h);
    for (id, label) in node_labels {
        let (x, y) = pos[id];
        d = d.with_node(Node {
            id: Id::new(id),
            label: label.clone(),
            role: Role::Neutral,
            region: None,
            at: Rect { x, y, w: NODE_W, h: NODE_H },
        });
    }
    for (from, label, to) in edges_in {
        if let (Some(&(fx, fy)), Some(&(tx, ty))) = (pos.get(from), pos.get(to)) {
            let (from_pt, to_pt) = if horizontal {
                (
                    Point { x: fx + NODE_W as i32, y: fy + NODE_H as i32 / 2 },
                    Point { x: tx, y: ty + NODE_H as i32 / 2 },
                )
            } else {
                (
                    Point { x: fx + NODE_W as i32 / 2, y: fy + NODE_H as i32 },
                    Point { x: tx + NODE_W as i32 / 2, y: ty },
                )
            };
            d = d.with_edge(Edge {
                from: from_pt,
                to: to_pt,
                stroke: Stroke::Solid,
                marker: Marker::Arrow,
                role: Role::Neutral,
                label: label.clone(),
            });
        }
    }
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn err_on_empty_input() {
        assert_eq!(from_mermaid("").unwrap_err(), FromMermaidError::Empty);
        assert_eq!(from_mermaid("   \n  ").unwrap_err(), FromMermaidError::Empty);
    }

    #[test]
    fn err_on_missing_flowchart_header() {
        let err = from_mermaid("A --> B\n").unwrap_err();
        assert_eq!(err, FromMermaidError::NoFlowchartHeader);
    }

    #[test]
    fn err_on_invalid_direction() {
        let err = from_mermaid("flowchart XX\nA --> B").unwrap_err();
        match err {
            FromMermaidError::InvalidDirection(d) => assert_eq!(d, "XX"),
            other => panic!("expected InvalidDirection, got {other:?}"),
        }
    }

    #[test]
    fn parses_simple_td_chain() {
        let src = "flowchart TD\nA[Start] --> B[End]";
        let d = from_mermaid(src).unwrap();
        assert_eq!(d.nodes.len(), 2);
        assert_eq!(d.edges.len(), 1);
        let labels: Vec<_> = d.nodes.iter().map(|n| n.label.as_str()).collect();
        assert!(labels.contains(&"Start"));
        assert!(labels.contains(&"End"));
    }

    #[test]
    fn parses_inline_edge_label() {
        let src = "flowchart LR\nA[Start] -- ok --> B[End]";
        let d = from_mermaid(src).unwrap();
        assert_eq!(d.edges.len(), 1);
        assert_eq!(d.edges[0].label.as_deref(), Some("ok"));
    }

    #[test]
    fn implicit_nodes_use_id_as_label() {
        let src = "flowchart TD\nA --> B";
        let d = from_mermaid(src).unwrap();
        let labels: Vec<_> = d.nodes.iter().map(|n| n.label.as_str()).collect();
        assert!(labels.contains(&"A"));
        assert!(labels.contains(&"B"));
    }

    #[test]
    fn lr_layout_lines_up_horizontally() {
        let src = "flowchart LR\nA --> B\nB --> C";
        let d = from_mermaid(src).unwrap();
        assert_eq!(d.nodes.len(), 3);
        let y0 = d.nodes[0].at.y;
        for n in &d.nodes {
            assert_eq!(n.at.y, y0, "LR: all nodes share y");
        }
    }

    #[test]
    fn td_layout_stacks_vertically() {
        let src = "flowchart TD\nA --> B\nB --> C";
        let d = from_mermaid(src).unwrap();
        let x0 = d.nodes[0].at.x;
        for n in &d.nodes {
            assert_eq!(n.at.x, x0, "TD: all nodes share x");
        }
    }

    #[test]
    fn rl_reverses_position_index_relative_to_lr() {
        // For input declared A → B → C, LR places A at min-x, C at max-x;
        // RL must reverse: C at min-x, A at max-x.
        let lr_src = "flowchart LR\nA --> B\nB --> C";
        let rl_src = "flowchart RL\nA --> B\nB --> C";
        let lr = from_mermaid(lr_src).unwrap();
        let rl = from_mermaid(rl_src).unwrap();
        let by_id = |d: &Diagram, id: &str| -> i32 {
            d.nodes.iter().find(|n| n.id.as_str() == id).unwrap().at.x
        };
        assert!(by_id(&lr, "A") < by_id(&lr, "C"), "LR: A is leftmost");
        assert!(by_id(&rl, "A") > by_id(&rl, "C"), "RL: A is rightmost");
    }

    #[test]
    fn is_deterministic() {
        let src = "flowchart TD\nA[Alpha] --> B[Beta]\nB --> C[Gamma]";
        assert_eq!(from_mermaid(src), from_mermaid(src));
    }

    #[test]
    fn skips_unsupported_lines_silently() {
        let src = "flowchart TD\n%% this is a comment\nA --> B\nclassDef foo fill:#000\n";
        let d = from_mermaid(src).unwrap();
        assert_eq!(d.nodes.len(), 2);
        assert_eq!(d.edges.len(), 1);
    }
}
