//! `kata::decision_branch` — single decision node + N branches with
//! per-branch arrow labels (yes/no/maybe, error/success, etc.). The
//! flowchart-style decision pattern. Distinct from [`api_fanout`](crate::api_fanout):
//! decision arrows carry semantically meaningful LABELS (the decision values).

use zukai_model::{Edge, Id, Marker, Node, Point, Rect, Role, Stroke};

const DECISION_W: u32 = 200;
const DECISION_H: u32 = 70;
const BRANCH_W: u32 = 160;
const BRANCH_H: u32 = 50;
const BRANCH_GAP: u32 = 24;

/// One outgoing branch.
#[derive(Debug, Clone, Copy)]
pub struct BranchArgs<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub role: Role,
    /// Label on the arrow leading to this branch (e.g., "yes", "no", "error").
    pub branch_label: &'a str,
}

/// Args for [`decision_branch`].
pub struct DecisionBranchArgs<'a> {
    pub decision_id: &'a str,
    pub decision_label: &'a str,
    /// Top-left of the decision node.
    pub decision_at: (i32, i32),
    pub branches: &'a [BranchArgs<'a>],
    pub branches_y: i32,
    pub canvas_width: u32,
}

/// Fragment returned by [`decision_branch`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionBranchFragment {
    /// Decision node, themed `Role::Accent`.
    pub decision: Node,
    pub branches: Vec<Node>,
    /// One labeled arrow per branch: decision → branch.
    pub edges: Vec<Edge>,
}

/// Build a decision node + N branches + labeled arrows.
#[must_use]
pub fn decision_branch(args: DecisionBranchArgs<'_>) -> DecisionBranchFragment {
    let decision = Node {
        id: Id::new(args.decision_id),
        label: args.decision_label.into(),
        role: Role::Accent,
        region: None,
        at: Rect {
            x: args.decision_at.0,
            y: args.decision_at.1,
            w: DECISION_W,
            h: DECISION_H,
        },
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

    let decision_anchor = Point {
        x: args.decision_at.0 + (DECISION_W as i32) / 2,
        y: args.decision_at.1 + DECISION_H as i32,
    };
    let edges: Vec<Edge> = branches
        .iter()
        .enumerate()
        .map(|(i, b)| Edge {
            from: decision_anchor,
            to: Point { x: b.at.x + (b.at.w as i32) / 2, y: b.at.y },
            stroke: Stroke::Solid,
            marker: Marker::Arrow,
            role: Role::Neutral,
            label: Some(args.branches[i].branch_label.to_string()),
        })
        .collect();

    DecisionBranchFragment { decision, branches, edges }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn three_branches() -> [BranchArgs<'static>; 3] {
        [
            BranchArgs { id: "ok",   label: "Continue", role: Role::Success, branch_label: "ok" },
            BranchArgs { id: "warn", label: "Retry",    role: Role::Accent,  branch_label: "warn" },
            BranchArgs { id: "err",  label: "Abort",    role: Role::Danger,  branch_label: "error" },
        ]
    }

    #[test]
    fn has_decision_and_n_branches_and_n_labeled_edges() {
        let bs = three_branches();
        let f = decision_branch(DecisionBranchArgs {
            decision_id: "d",
            decision_label: "Status?",
            decision_at: (380, 120),
            branches: &bs,
            branches_y: 280,
            canvas_width: 960,
        });
        assert_eq!(f.decision.role, Role::Accent);
        assert_eq!(f.branches.len(), 3);
        assert_eq!(f.edges.len(), 3);
    }

    #[test]
    fn each_edge_carries_its_branch_label() {
        let bs = three_branches();
        let f = decision_branch(DecisionBranchArgs {
            decision_id: "d",
            decision_label: "Status?",
            decision_at: (380, 120),
            branches: &bs,
            branches_y: 280,
            canvas_width: 960,
        });
        assert_eq!(f.edges[0].label.as_deref(), Some("ok"));
        assert_eq!(f.edges[1].label.as_deref(), Some("warn"));
        assert_eq!(f.edges[2].label.as_deref(), Some("error"));
    }

    #[test]
    fn is_deterministic() {
        let bs = three_branches();
        let args = || DecisionBranchArgs {
            decision_id: "d",
            decision_label: "Status?",
            decision_at: (380, 120),
            branches: &bs,
            branches_y: 280,
            canvas_width: 960,
        };
        assert_eq!(decision_branch(args()), decision_branch(args()));
    }

    #[test]
    fn empty_branches_returns_no_edges() {
        let f = decision_branch(DecisionBranchArgs {
            decision_id: "d",
            decision_label: "Q",
            decision_at: (0, 0),
            branches: &[],
            branches_y: 200,
            canvas_width: 1000,
        });
        assert!(f.branches.is_empty());
        assert!(f.edges.is_empty());
    }
}
