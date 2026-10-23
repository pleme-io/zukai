//! `kata::api_fanout` — central API node with arrows fanning out to N backend
//! services. The "API gateway → backends" pattern.

use zukai_model::{Edge, Id, Marker, Node, Point, Rect, Role, Stroke};

const NODE_W: u32 = 140;
const NODE_H: u32 = 40;
const BACKEND_GAP: u32 = 20;

/// One backend chip.
#[derive(Debug, Clone, Copy)]
pub struct BackendArgs<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub role: Role,
}

/// Args for [`api_fanout`].
pub struct ApiFanoutArgs<'a> {
    pub api_id: &'a str,
    pub api_label: &'a str,
    /// Top-left of the central API node.
    pub api_at: (i32, i32),
    pub backends: &'a [BackendArgs<'a>],
    /// Y position of the backends row.
    pub backends_y: i32,
    /// Total horizontal extent the backends are centered within.
    pub canvas_width: u32,
}

/// Fragment returned by [`api_fanout`] — caller composes into a `Diagram`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiFanoutFragment {
    pub api_node: Node,
    pub backends: Vec<Node>,
    pub edges: Vec<Edge>,
}

/// Build the central API node + a row of backends + fanout arrows.
#[must_use]
pub fn api_fanout(args: ApiFanoutArgs<'_>) -> ApiFanoutFragment {
    let api_node = Node {
        id: Id::new(args.api_id),
        label: args.api_label.into(),
        role: Role::Accent,
        region: None,
        at: Rect { x: args.api_at.0, y: args.api_at.1, w: NODE_W, h: NODE_H },
    };

    let n = args.backends.len() as u32;
    let backends: Vec<Node> = if n == 0 {
        Vec::new()
    } else {
        let total_gap = BACKEND_GAP.saturating_mul(n.saturating_sub(1));
        let backend_w = args.canvas_width.saturating_sub(total_gap) / n.max(1);
        let total_used = n * backend_w + total_gap;
        let origin_x = ((args.canvas_width as i32 - total_used as i32) / 2).max(0);
        args.backends
            .iter()
            .enumerate()
            .map(|(i, b)| {
                let x = origin_x + (i as i32) * (backend_w as i32 + BACKEND_GAP as i32);
                Node {
                    id: Id::new(b.id),
                    label: b.label.into(),
                    role: b.role,
                    region: None,
                    at: Rect { x, y: args.backends_y, w: backend_w, h: NODE_H },
                }
            })
            .collect()
    };

    // Fanout arrows from API node bottom-center to each backend top-center.
    let api_anchor = Point {
        x: args.api_at.0 + (NODE_W as i32) / 2,
        y: args.api_at.1 + NODE_H as i32,
    };
    let edges: Vec<Edge> = backends
        .iter()
        .map(|b| Edge {
            from: api_anchor,
            to: Point {
                x: b.at.x + (b.at.w as i32) / 2,
                y: b.at.y,
            },
            stroke: Stroke::Solid,
            marker: Marker::Arrow,
            role: Role::Neutral,
            label: None,
        })
        .collect();

    ApiFanoutFragment { api_node, backends, edges }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn three_backends() -> [BackendArgs<'static>; 3] {
        [
            BackendArgs { id: "b1", label: "Backend 1", role: Role::Master },
            BackendArgs { id: "b2", label: "Backend 2", role: Role::Master },
            BackendArgs { id: "b3", label: "Backend 3", role: Role::Master },
        ]
    }

    #[test]
    fn has_central_api_and_n_backends_and_n_edges() {
        let bs = three_backends();
        let f = api_fanout(ApiFanoutArgs {
            api_id: "api",
            api_label: "API",
            api_at: (410, 200),
            backends: &bs,
            backends_y: 350,
            canvas_width: 960,
        });
        assert_eq!(f.api_node.label, "API");
        assert_eq!(f.api_node.role, Role::Accent);
        assert_eq!(f.backends.len(), 3);
        assert_eq!(f.edges.len(), 3);
    }

    #[test]
    fn edges_anchor_at_api_bottom_and_backend_top() {
        let bs = three_backends();
        let f = api_fanout(ApiFanoutArgs {
            api_id: "api",
            api_label: "API",
            api_at: (410, 200),
            backends: &bs,
            backends_y: 350,
            canvas_width: 960,
        });
        let api_anchor_x = 410 + (NODE_W as i32) / 2;
        let api_anchor_y = 200 + NODE_H as i32;
        for (i, e) in f.edges.iter().enumerate() {
            assert_eq!(e.from.x, api_anchor_x, "edge {i} from.x");
            assert_eq!(e.from.y, api_anchor_y, "edge {i} from.y");
            assert_eq!(e.to.y, 350, "edge {i} to.y");
            assert_eq!(
                e.to.x,
                f.backends[i].at.x + (f.backends[i].at.w as i32) / 2,
                "edge {i} to.x"
            );
        }
    }

    #[test]
    fn is_deterministic() {
        let bs = three_backends();
        let args = || ApiFanoutArgs {
            api_id: "api",
            api_label: "API",
            api_at: (410, 200),
            backends: &bs,
            backends_y: 350,
            canvas_width: 960,
        };
        assert_eq!(api_fanout(args()), api_fanout(args()));
    }

    #[test]
    fn empty_backends_returns_no_edges() {
        let f = api_fanout(ApiFanoutArgs {
            api_id: "api",
            api_label: "API",
            api_at: (0, 0),
            backends: &[],
            backends_y: 200,
            canvas_width: 1000,
        });
        assert!(f.backends.is_empty());
        assert!(f.edges.is_empty());
    }
}
