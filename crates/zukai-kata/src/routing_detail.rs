//! `kata::routing_detail` — N-stage horizontal request-routing chain.
//! Each stage is a node; consecutive stages connect with arrows.
//! Canonical use: "DNS → LB → ingress → service" follow-the-packet diagrams.

use zukai_model::{Edge, Id, Marker, Node, Point, Rect, Role, Stroke};

/// One routing stage.
#[derive(Debug, Clone, Copy)]
pub struct RoutingStageArgs<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub role: Role,
    /// Optional label on the arrow LEAVING this stage (transition to the next).
    /// Ignored on the last stage.
    pub arrow_label: Option<&'a str>,
}

/// Args for [`routing_detail`].
pub struct RoutingDetailArgs<'a> {
    /// Top-left of the first stage.
    pub origin: (i32, i32),
    pub stage_width: u32,
    pub stage_height: u32,
    pub gap: u32,
    pub stages: &'a [RoutingStageArgs<'a>],
}

/// Fragment returned by [`routing_detail`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingDetailFragment {
    pub stages: Vec<Node>,
    /// N-1 arrows between consecutive stages.
    pub edges: Vec<Edge>,
}

/// Build N stages laid out horizontally + N-1 connecting arrows.
#[must_use]
pub fn routing_detail(args: RoutingDetailArgs<'_>) -> RoutingDetailFragment {
    let (origin_x, origin_y) = args.origin;
    let stages: Vec<Node> = args
        .stages
        .iter()
        .enumerate()
        .map(|(i, s)| Node {
            id: Id::new(s.id),
            label: s.label.into(),
            role: s.role,
            region: None,
            at: Rect {
                x: origin_x + (i as i32) * (args.stage_width as i32 + args.gap as i32),
                y: origin_y,
                w: args.stage_width,
                h: args.stage_height,
            },
        })
        .collect();

    let edges: Vec<Edge> = stages
        .windows(2)
        .enumerate()
        .map(|(i, pair)| {
            let from = &pair[0];
            let to = &pair[1];
            Edge {
                from: Point {
                    x: from.at.x + from.at.w as i32,
                    y: from.at.y + (from.at.h as i32) / 2,
                },
                to: Point {
                    x: to.at.x,
                    y: to.at.y + (to.at.h as i32) / 2,
                },
                stroke: Stroke::Solid,
                marker: Marker::Arrow,
                role: Role::Neutral,
                label: args.stages[i].arrow_label.map(String::from),
            }
        })
        .collect();

    RoutingDetailFragment { stages, edges }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn four_stages() -> [RoutingStageArgs<'static>; 4] {
        [
            RoutingStageArgs { id: "dns",     label: "DNS",     role: Role::Neutral, arrow_label: Some("CNAME") },
            RoutingStageArgs { id: "lb",      label: "LB",      role: Role::Accent,  arrow_label: Some("HTTP") },
            RoutingStageArgs { id: "ingress", label: "Ingress", role: Role::Master,  arrow_label: Some("route") },
            RoutingStageArgs { id: "svc",     label: "Service", role: Role::Master,  arrow_label: None },
        ]
    }

    #[test]
    fn has_n_stages_and_n_minus_one_edges() {
        let ss = four_stages();
        let f = routing_detail(RoutingDetailArgs {
            origin: (40, 240),
            stage_width: 160,
            stage_height: 60,
            gap: 40,
            stages: &ss,
        });
        assert_eq!(f.stages.len(), 4);
        assert_eq!(f.edges.len(), 3);
    }

    #[test]
    fn stages_lay_out_left_to_right_with_gaps() {
        let ss = four_stages();
        let f = routing_detail(RoutingDetailArgs {
            origin: (40, 240),
            stage_width: 160,
            stage_height: 60,
            gap: 40,
            stages: &ss,
        });
        for (i, n) in f.stages.iter().enumerate() {
            assert_eq!(n.at.x, 40 + (i as i32) * (160 + 40));
            assert_eq!(n.at.y, 240);
            assert_eq!(n.at.w, 160);
            assert_eq!(n.at.h, 60);
        }
    }

    #[test]
    fn edges_carry_per_stage_arrow_labels() {
        let ss = four_stages();
        let f = routing_detail(RoutingDetailArgs {
            origin: (40, 240),
            stage_width: 160,
            stage_height: 60,
            gap: 40,
            stages: &ss,
        });
        assert_eq!(f.edges[0].label.as_deref(), Some("CNAME"));
        assert_eq!(f.edges[1].label.as_deref(), Some("HTTP"));
        assert_eq!(f.edges[2].label.as_deref(), Some("route"));
    }

    #[test]
    fn is_deterministic() {
        let ss = four_stages();
        let args = || RoutingDetailArgs {
            origin: (40, 240),
            stage_width: 160,
            stage_height: 60,
            gap: 40,
            stages: &ss,
        };
        assert_eq!(routing_detail(args()), routing_detail(args()));
    }

    #[test]
    fn empty_stages_returns_no_edges() {
        let f = routing_detail(RoutingDetailArgs {
            origin: (0, 0),
            stage_width: 100,
            stage_height: 50,
            gap: 10,
            stages: &[],
        });
        assert!(f.stages.is_empty());
        assert!(f.edges.is_empty());
    }
}
