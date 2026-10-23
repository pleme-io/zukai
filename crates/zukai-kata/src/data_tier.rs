//! `kata::data_tier` — vertical storage hierarchy. Tiers stack top→bottom
//! (typically hot → warm → cold); downward arrows between adjacent tiers
//! signal data aging / flow.

use zukai_model::{Edge, Id, Marker, Node, Point, Rect, Role, Stroke};

/// One storage tier.
#[derive(Debug, Clone, Copy)]
pub struct TierArgs<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub role: Role,
    /// Optional label on the arrow leaving this tier downward to the next.
    /// Ignored on the last tier.
    pub arrow_label: Option<&'a str>,
}

/// Args for [`data_tier`].
pub struct DataTierArgs<'a> {
    /// Top-left of the topmost tier.
    pub origin: (i32, i32),
    pub width: u32,
    pub tier_height: u32,
    pub gap: u32,
    pub tiers: &'a [TierArgs<'a>],
}

/// Fragment returned by [`data_tier`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataTierFragment {
    pub tiers: Vec<Node>,
    /// N-1 downward arrows between consecutive tiers.
    pub edges: Vec<Edge>,
}

/// Build a top-down storage hierarchy with arrows between adjacent tiers.
#[must_use]
pub fn data_tier(args: DataTierArgs<'_>) -> DataTierFragment {
    let (origin_x, origin_y) = args.origin;
    let tiers: Vec<Node> = args
        .tiers
        .iter()
        .enumerate()
        .map(|(i, t)| Node {
            id: Id::new(t.id),
            label: t.label.into(),
            role: t.role,
            region: None,
            at: Rect {
                x: origin_x,
                y: origin_y + (i as i32) * (args.tier_height as i32 + args.gap as i32),
                w: args.width,
                h: args.tier_height,
            },
        })
        .collect();

    let edges: Vec<Edge> = tiers
        .windows(2)
        .enumerate()
        .map(|(i, pair)| {
            let from = &pair[0];
            let to = &pair[1];
            Edge {
                from: Point {
                    x: from.at.x + (from.at.w as i32) / 2,
                    y: from.at.y + from.at.h as i32,
                },
                to: Point {
                    x: to.at.x + (to.at.w as i32) / 2,
                    y: to.at.y,
                },
                stroke: Stroke::Solid,
                marker: Marker::Arrow,
                role: Role::Neutral,
                label: args.tiers[i].arrow_label.map(String::from),
            }
        })
        .collect();

    DataTierFragment { tiers, edges }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn three_tiers() -> [TierArgs<'static>; 3] {
        [
            TierArgs { id: "hot",  label: "Hot",  role: Role::Accent,  arrow_label: Some("age out") },
            TierArgs { id: "warm", label: "Warm", role: Role::Read,    arrow_label: Some("archive") },
            TierArgs { id: "cold", label: "Cold", role: Role::Neutral, arrow_label: None },
        ]
    }

    #[test]
    fn has_n_tiers_and_n_minus_one_edges() {
        let ts = three_tiers();
        let f = data_tier(DataTierArgs {
            origin: (200, 120),
            width: 560,
            tier_height: 80,
            gap: 30,
            tiers: &ts,
        });
        assert_eq!(f.tiers.len(), 3);
        assert_eq!(f.edges.len(), 2);
    }

    #[test]
    fn tiers_stack_vertically_with_gaps() {
        let ts = three_tiers();
        let f = data_tier(DataTierArgs {
            origin: (200, 120),
            width: 560,
            tier_height: 80,
            gap: 30,
            tiers: &ts,
        });
        for (i, n) in f.tiers.iter().enumerate() {
            assert_eq!(n.at.x, 200);
            assert_eq!(n.at.y, 120 + (i as i32) * (80 + 30));
            assert_eq!(n.at.w, 560);
            assert_eq!(n.at.h, 80);
        }
    }

    #[test]
    fn edges_flow_downward_with_per_tier_labels() {
        let ts = three_tiers();
        let f = data_tier(DataTierArgs {
            origin: (200, 120),
            width: 560,
            tier_height: 80,
            gap: 30,
            tiers: &ts,
        });
        for e in &f.edges {
            assert!(e.from.y < e.to.y, "arrow must flow downward");
        }
        assert_eq!(f.edges[0].label.as_deref(), Some("age out"));
        assert_eq!(f.edges[1].label.as_deref(), Some("archive"));
    }

    #[test]
    fn is_deterministic() {
        let ts = three_tiers();
        let args = || DataTierArgs {
            origin: (200, 120),
            width: 560,
            tier_height: 80,
            gap: 30,
            tiers: &ts,
        };
        assert_eq!(data_tier(args()), data_tier(args()));
    }

    #[test]
    fn empty_tiers_returns_no_edges() {
        let f = data_tier(DataTierArgs {
            origin: (0, 0),
            width: 100,
            tier_height: 50,
            gap: 10,
            tiers: &[],
        });
        assert!(f.tiers.is_empty());
        assert!(f.edges.is_empty());
    }
}
