//! `kata::ha_failover` — high-availability pattern with primary + standby
//! and two state-transition arrows (failover and recovery).

use zukai_model::{Edge, Id, Marker, Node, Point, Rect, Role, Stroke};

const NODE_W: u32 = 200;
const NODE_H: u32 = 80;
const TOP_INSET: i32 = 20;
const BOTTOM_INSET: i32 = 20;

/// One HA node descriptor.
#[derive(Debug, Clone, Copy)]
pub struct HaNodeArgs<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub sublabel: Option<&'a str>,
}

/// Args for [`ha_failover`].
pub struct HaFailoverArgs<'a> {
    pub primary: HaNodeArgs<'a>,
    pub standby: HaNodeArgs<'a>,
    /// Top-left of the primary node.
    pub primary_at: (i32, i32),
    /// Top-left of the standby node.
    pub standby_at: (i32, i32),
    /// Optional label on the failover (primary→standby) arrow.
    pub failover_label: Option<&'a str>,
    /// Optional label on the recovery (standby→primary) arrow.
    pub recovery_label: Option<&'a str>,
}

/// Fragment returned by [`ha_failover`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HaFailoverFragment {
    pub primary: Node,
    pub standby: Node,
    pub failover_edge: Edge,
    pub recovery_edge: Edge,
}

/// Build the primary + standby + two opposing transition arrows.
///
/// - Failover arrow: solid, `Role::Danger`, primary's right → standby's left
///   along the top of each node.
/// - Recovery arrow: dashed, `Role::Success`, standby's left → primary's right
///   along the bottom of each node.
#[must_use]
pub fn ha_failover(args: HaFailoverArgs<'_>) -> HaFailoverFragment {
    let primary = Node {
        id: Id::new(args.primary.id),
        label: args.primary.label.into(),
        role: Role::Master,
        region: None,
        at: Rect {
            x: args.primary_at.0,
            y: args.primary_at.1,
            w: NODE_W,
            h: NODE_H,
        },
    };
    let standby = Node {
        id: Id::new(args.standby.id),
        label: args.standby.label.into(),
        role: Role::Read,
        region: None,
        at: Rect {
            x: args.standby_at.0,
            y: args.standby_at.1,
            w: NODE_W,
            h: NODE_H,
        },
    };
    let p_right = primary.at.x + primary.at.w as i32;
    let s_left = standby.at.x;
    let failover_edge = Edge {
        from: Point { x: p_right, y: primary.at.y + TOP_INSET },
        to: Point { x: s_left, y: standby.at.y + TOP_INSET },
        stroke: Stroke::Solid,
        marker: Marker::Arrow,
        role: Role::Danger,
        label: args.failover_label.map(String::from),
    };
    let recovery_edge = Edge {
        from: Point { x: s_left, y: standby.at.y + NODE_H as i32 - BOTTOM_INSET },
        to: Point { x: p_right, y: primary.at.y + NODE_H as i32 - BOTTOM_INSET },
        stroke: Stroke::Dashed,
        marker: Marker::Arrow,
        role: Role::Success,
        label: args.recovery_label.map(String::from),
    };
    HaFailoverFragment { primary, standby, failover_edge, recovery_edge }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> HaFailoverArgs<'static> {
        HaFailoverArgs {
            primary: HaNodeArgs { id: "p", label: "Primary", sublabel: None },
            standby: HaNodeArgs { id: "s", label: "Standby", sublabel: None },
            primary_at: (100, 200),
            standby_at: (660, 200),
            failover_label: Some("failover"),
            recovery_label: Some("recover"),
        }
    }

    #[test]
    fn has_two_nodes_with_correct_roles() {
        let f = ha_failover(sample());
        assert_eq!(f.primary.label, "Primary");
        assert_eq!(f.primary.role, Role::Master);
        assert_eq!(f.standby.label, "Standby");
        assert_eq!(f.standby.role, Role::Read);
    }

    #[test]
    fn arrows_oppose_with_correct_styles_and_roles() {
        let f = ha_failover(sample());
        // failover: primary→standby (left→right), solid, Danger.
        assert!(f.failover_edge.from.x < f.failover_edge.to.x);
        assert_eq!(f.failover_edge.stroke, Stroke::Solid);
        assert_eq!(f.failover_edge.role, Role::Danger);
        assert_eq!(f.failover_edge.label.as_deref(), Some("failover"));
        // recovery: standby→primary (right→left), dashed, Success.
        assert!(f.recovery_edge.from.x > f.recovery_edge.to.x);
        assert_eq!(f.recovery_edge.stroke, Stroke::Dashed);
        assert_eq!(f.recovery_edge.role, Role::Success);
        assert_eq!(f.recovery_edge.label.as_deref(), Some("recover"));
    }

    #[test]
    fn is_deterministic() {
        let a = ha_failover(sample());
        let b = ha_failover(sample());
        assert_eq!(a, b);
    }

    #[test]
    fn arrow_labels_are_optional() {
        let mut args = sample();
        args.failover_label = None;
        args.recovery_label = None;
        let f = ha_failover(args);
        assert!(f.failover_edge.label.is_none());
        assert!(f.recovery_edge.label.is_none());
    }
}
