//! `kata::tree_hierarchy` — root + N branches (v0: 2-level). Distinct from
//! [`api_fanout`](crate::api_fanout) semantically: `api_fanout` is REQUEST
//! routing (one→many for distribution); `tree_hierarchy` is CONTAINMENT
//! (one is composed of many). Root's `Role` is caller-configurable; arrows
//! are themed `Neutral` (no implied data direction).

use zukai_model::{Edge, Id, Marker, Node, Point, Rect, Role, Stroke};

const ROOT_W: u32 = 200;
const ROOT_H: u32 = 60;
const BRANCH_W: u32 = 140;
const BRANCH_H: u32 = 50;
const BRANCH_GAP: u32 = 20;

/// One tree node (used for root and branches).
#[derive(Debug, Clone, Copy)]
pub struct TreeNodeArgs<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub role: Role,
}

/// Args for [`tree_hierarchy`].
pub struct TreeHierarchyArgs<'a> {
    pub root: TreeNodeArgs<'a>,
    /// Top-left of the root node.
    pub root_at: (i32, i32),
    pub branches: &'a [TreeNodeArgs<'a>],
    /// Y position of the branches row.
    pub branches_y: i32,
    /// Total horizontal extent the branches are centered within.
    pub canvas_width: u32,
}

/// Fragment returned by [`tree_hierarchy`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeHierarchyFragment {
    pub root: Node,
    pub branches: Vec<Node>,
    /// One arrow per branch: root → branch.
    pub edges: Vec<Edge>,
}

/// Build root + branches + arrows from the root to each branch.
#[must_use]
pub fn tree_hierarchy(args: TreeHierarchyArgs<'_>) -> TreeHierarchyFragment {
    let root = Node {
        id: Id::new(args.root.id),
        label: args.root.label.into(),
        role: args.root.role,
        region: None,
        at: Rect { x: args.root_at.0, y: args.root_at.1, w: ROOT_W, h: ROOT_H },
    };

    let n = args.branches.len() as u32;
    let branches: Vec<Node> = if n == 0 {
        Vec::new()
    } else {
        let total_gap = BRANCH_GAP.saturating_mul(n.saturating_sub(1));
        let total_used = n * BRANCH_W + total_gap;
        let origin_x = ((args.canvas_width as i32 - total_used as i32) / 2).max(0);
        args.branches
            .iter()
            .enumerate()
            .map(|(i, b)| Node {
                id: Id::new(b.id),
                label: b.label.into(),
                role: b.role,
                region: None,
                at: Rect {
                    x: origin_x + (i as i32) * (BRANCH_W as i32 + BRANCH_GAP as i32),
                    y: args.branches_y,
                    w: BRANCH_W,
                    h: BRANCH_H,
                },
            })
            .collect()
    };

    let root_anchor = Point {
        x: args.root_at.0 + (ROOT_W as i32) / 2,
        y: args.root_at.1 + ROOT_H as i32,
    };
    let edges: Vec<Edge> = branches
        .iter()
        .map(|b| Edge {
            from: root_anchor,
            to: Point { x: b.at.x + (b.at.w as i32) / 2, y: b.at.y },
            stroke: Stroke::Solid,
            marker: Marker::Arrow,
            role: Role::Neutral,
            label: None,
        })
        .collect();

    TreeHierarchyFragment { root, branches, edges }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn three_branches() -> [TreeNodeArgs<'static>; 3] {
        [
            TreeNodeArgs { id: "b1", label: "Branch 1", role: Role::Master },
            TreeNodeArgs { id: "b2", label: "Branch 2", role: Role::Master },
            TreeNodeArgs { id: "b3", label: "Branch 3", role: Role::Master },
        ]
    }

    #[test]
    fn has_root_and_n_branches_and_n_edges() {
        let bs = three_branches();
        let f = tree_hierarchy(TreeHierarchyArgs {
            root: TreeNodeArgs { id: "root", label: "Root", role: Role::Accent },
            root_at: (380, 120),
            branches: &bs,
            branches_y: 240,
            canvas_width: 960,
        });
        assert_eq!(f.root.label, "Root");
        assert_eq!(f.root.role, Role::Accent);
        assert_eq!(f.branches.len(), 3);
        assert_eq!(f.edges.len(), 3);
    }

    #[test]
    fn root_role_is_caller_configurable() {
        let bs = three_branches();
        let f1 = tree_hierarchy(TreeHierarchyArgs {
            root: TreeNodeArgs { id: "r", label: "R", role: Role::Master },
            root_at: (0, 0), branches: &bs, branches_y: 100, canvas_width: 960,
        });
        let f2 = tree_hierarchy(TreeHierarchyArgs {
            root: TreeNodeArgs { id: "r", label: "R", role: Role::Neutral },
            root_at: (0, 0), branches: &bs, branches_y: 100, canvas_width: 960,
        });
        assert_eq!(f1.root.role, Role::Master);
        assert_eq!(f2.root.role, Role::Neutral);
    }

    #[test]
    fn edges_anchor_at_root_bottom_center_and_branch_top_center() {
        let bs = three_branches();
        let f = tree_hierarchy(TreeHierarchyArgs {
            root: TreeNodeArgs { id: "root", label: "Root", role: Role::Accent },
            root_at: (380, 120),
            branches: &bs,
            branches_y: 240,
            canvas_width: 960,
        });
        let root_anchor_x = 380 + (ROOT_W as i32) / 2;
        let root_anchor_y = 120 + ROOT_H as i32;
        for (i, e) in f.edges.iter().enumerate() {
            assert_eq!(e.from.x, root_anchor_x);
            assert_eq!(e.from.y, root_anchor_y);
            assert_eq!(e.to.y, 240);
            assert_eq!(e.to.x, f.branches[i].at.x + (f.branches[i].at.w as i32) / 2);
        }
    }

    #[test]
    fn is_deterministic() {
        let bs = three_branches();
        let args = || TreeHierarchyArgs {
            root: TreeNodeArgs { id: "root", label: "Root", role: Role::Accent },
            root_at: (380, 120),
            branches: &bs,
            branches_y: 240,
            canvas_width: 960,
        };
        assert_eq!(tree_hierarchy(args()), tree_hierarchy(args()));
    }

    #[test]
    fn empty_branches_returns_no_edges() {
        let f = tree_hierarchy(TreeHierarchyArgs {
            root: TreeNodeArgs { id: "root", label: "Root", role: Role::Accent },
            root_at: (0, 0),
            branches: &[],
            branches_y: 200,
            canvas_width: 1000,
        });
        assert!(f.branches.is_empty());
        assert!(f.edges.is_empty());
    }
}
