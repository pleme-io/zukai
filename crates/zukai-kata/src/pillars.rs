//! `kata::pillars` — N-column foundation pattern. Evenly distributes labeled
//! pillars across a width, each themed by its `Role`. The "principles"
//! diagram shape (cluster of foundational ideas/values/services).

use zukai_model::{Id, Node, Rect, Role};

/// One pillar.
#[derive(Debug, Clone, Copy)]
pub struct PillarArgs<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub role: Role,
}

/// Args for [`pillars`].
pub struct PillarsArgs<'a> {
    /// Top-left of the pillar row in canvas units.
    pub origin: (i32, i32),
    /// Total available width to distribute pillars across.
    pub width: u32,
    /// Pillar height.
    pub pillar_height: u32,
    /// Horizontal gap between pillars.
    pub gap: u32,
    pub pillars: &'a [PillarArgs<'a>],
}

/// Distribute N pillars evenly across the available width.
#[must_use]
pub fn pillars(args: PillarsArgs<'_>) -> Vec<Node> {
    let n = args.pillars.len() as u32;
    if n == 0 {
        return Vec::new();
    }
    let total_gap = args.gap.saturating_mul(n.saturating_sub(1));
    let pillar_w = args.width.saturating_sub(total_gap) / n;
    let (origin_x, origin_y) = args.origin;
    args.pillars
        .iter()
        .enumerate()
        .map(|(i, p)| Node {
            id: Id::new(p.id),
            label: p.label.into(),
            role: p.role,
            region: None,
            at: Rect {
                x: origin_x + (i as i32) * (pillar_w as i32 + args.gap as i32),
                y: origin_y,
                w: pillar_w,
                h: args.pillar_height,
            },
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn four() -> [PillarArgs<'static>; 4] {
        [
            PillarArgs { id: "p1", label: "One", role: Role::Master },
            PillarArgs { id: "p2", label: "Two", role: Role::Read },
            PillarArgs { id: "p3", label: "Three", role: Role::Accent },
            PillarArgs { id: "p4", label: "Four", role: Role::Neutral },
        ]
    }

    #[test]
    fn distributes_n_pillars_evenly() {
        let ps = four();
        let nodes = pillars(PillarsArgs {
            origin: (40, 200),
            width: 880,    // 880 - 3*10 = 850; 850/4 = 212
            pillar_height: 120,
            gap: 10,
            pillars: &ps,
        });
        assert_eq!(nodes.len(), 4);
        assert_eq!(nodes[0].at.w, 212);
        assert_eq!(nodes[0].at.x, 40);
        assert_eq!(nodes[1].at.x, 40 + 212 + 10);
        assert_eq!(nodes[2].at.x, 40 + 2 * (212 + 10));
        assert_eq!(nodes[3].at.x, 40 + 3 * (212 + 10));
        for n in &nodes {
            assert_eq!(n.at.y, 200);
            assert_eq!(n.at.h, 120);
            assert!(n.region.is_none(), "pillars are top-level, no parent region");
        }
    }

    #[test]
    fn carries_per_pillar_role() {
        let ps = four();
        let nodes = pillars(PillarsArgs {
            origin: (0, 0),
            width: 1000,
            pillar_height: 100,
            gap: 0,
            pillars: &ps,
        });
        assert_eq!(nodes[0].role, Role::Master);
        assert_eq!(nodes[1].role, Role::Read);
        assert_eq!(nodes[2].role, Role::Accent);
        assert_eq!(nodes[3].role, Role::Neutral);
    }

    #[test]
    fn empty_pillars_returns_empty_vec() {
        assert!(pillars(PillarsArgs {
            origin: (0, 0),
            width: 100,
            pillar_height: 50,
            gap: 0,
            pillars: &[],
        })
        .is_empty());
    }

    #[test]
    fn is_deterministic() {
        let ps = four();
        let args = || PillarsArgs {
            origin: (40, 200),
            width: 880,
            pillar_height: 120,
            gap: 10,
            pillars: &ps,
        };
        assert_eq!(pillars(args()), pillars(args()));
    }
}
