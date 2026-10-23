//! `kata::interaction_lane` — two-actor sequence diagram. Two actor chips
//! at the top + two vertical "lifeline" lines extending downward + N
//! horizontal labeled message arrows between the lifelines (direction
//! per-message). The UML-sequence-diagram shape (simplified).

use zukai_model::{Edge, Id, Marker, Node, Point, Rect, Role, Stroke};

const ACTOR_W: u32 = 160;
const ACTOR_H: u32 = 48;
const MSG_GAP: i32 = 60;
const FIRST_MSG_OFFSET: i32 = 40;

/// One horizontal message between the two lifelines.
#[derive(Debug, Clone, Copy)]
pub struct InteractionMessageArgs<'a> {
    pub label: &'a str,
    /// If true, arrow points left→right; otherwise right→left.
    pub from_left_to_right: bool,
    pub role: Role,
}

/// Args for [`interaction_lane`].
pub struct InteractionLaneArgs<'a> {
    pub left_id: &'a str,
    pub left_label: &'a str,
    pub right_id: &'a str,
    pub right_label: &'a str,
    /// Top-left of the LEFT actor chip.
    pub origin: (i32, i32),
    /// Horizontal distance from left actor's center to right actor's center.
    pub lane_width: u32,
    /// Total length of each lifeline below its actor.
    pub lifeline_length: u32,
    pub messages: &'a [InteractionMessageArgs<'a>],
}

/// Fragment returned by [`interaction_lane`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InteractionLaneFragment {
    pub left_actor: Node,
    pub right_actor: Node,
    /// Vertical line extending downward from below the left actor.
    pub left_lifeline: Edge,
    pub right_lifeline: Edge,
    /// Horizontal labeled arrows between the two lifelines, in declaration order.
    pub messages: Vec<Edge>,
}

/// Build two actors + two vertical lifelines + N horizontal labeled messages.
#[must_use]
pub fn interaction_lane(args: InteractionLaneArgs<'_>) -> InteractionLaneFragment {
    let (left_x, top_y) = args.origin;
    let right_x = left_x + args.lane_width as i32 - ACTOR_W as i32;

    let left_actor = Node {
        id: Id::new(args.left_id),
        label: args.left_label.into(),
        role: Role::Accent,
        region: None,
        at: Rect { x: left_x, y: top_y, w: ACTOR_W, h: ACTOR_H },
    };
    let right_actor = Node {
        id: Id::new(args.right_id),
        label: args.right_label.into(),
        role: Role::Master,
        region: None,
        at: Rect { x: right_x, y: top_y, w: ACTOR_W, h: ACTOR_H },
    };

    let left_lifeline_x = left_x + (ACTOR_W as i32) / 2;
    let right_lifeline_x = right_x + (ACTOR_W as i32) / 2;
    let lifeline_top = top_y + ACTOR_H as i32;
    let lifeline_bottom = lifeline_top + args.lifeline_length as i32;

    let left_lifeline = Edge {
        from: Point { x: left_lifeline_x, y: lifeline_top },
        to: Point { x: left_lifeline_x, y: lifeline_bottom },
        stroke: Stroke::Dashed,
        marker: Marker::Arrow,
        role: Role::Neutral,
        label: None,
    };
    let right_lifeline = Edge {
        from: Point { x: right_lifeline_x, y: lifeline_top },
        to: Point { x: right_lifeline_x, y: lifeline_bottom },
        stroke: Stroke::Dashed,
        marker: Marker::Arrow,
        role: Role::Neutral,
        label: None,
    };

    let messages: Vec<Edge> = args
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let y = lifeline_top + FIRST_MSG_OFFSET + (i as i32) * MSG_GAP;
            let (from_x, to_x) = if m.from_left_to_right {
                (left_lifeline_x, right_lifeline_x)
            } else {
                (right_lifeline_x, left_lifeline_x)
            };
            Edge {
                from: Point { x: from_x, y },
                to: Point { x: to_x, y },
                stroke: Stroke::Solid,
                marker: Marker::Arrow,
                role: m.role,
                label: Some(m.label.to_string()),
            }
        })
        .collect();

    InteractionLaneFragment {
        left_actor,
        right_actor,
        left_lifeline,
        right_lifeline,
        messages,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn three_messages() -> [InteractionMessageArgs<'static>; 3] {
        [
            InteractionMessageArgs { label: "request",  from_left_to_right: true,  role: Role::Accent },
            InteractionMessageArgs { label: "process",  from_left_to_right: false, role: Role::Neutral },
            InteractionMessageArgs { label: "response", from_left_to_right: false, role: Role::Success },
        ]
    }

    #[test]
    fn has_two_actors_two_lifelines_and_n_messages() {
        let ms = three_messages();
        let f = interaction_lane(InteractionLaneArgs {
            left_id: "client",
            left_label: "Client",
            right_id: "server",
            right_label: "Server",
            origin: (60, 100),
            lane_width: 700,
            lifeline_length: 280,
            messages: &ms,
        });
        assert_eq!(f.left_actor.label, "Client");
        assert_eq!(f.right_actor.label, "Server");
        assert_eq!(f.messages.len(), 3);
    }

    #[test]
    fn lifelines_are_vertical_below_actors() {
        let ms = three_messages();
        let f = interaction_lane(InteractionLaneArgs {
            left_id: "c", left_label: "Client",
            right_id: "s", right_label: "Server",
            origin: (60, 100),
            lane_width: 700,
            lifeline_length: 280,
            messages: &ms,
        });
        assert_eq!(f.left_lifeline.from.x, f.left_lifeline.to.x, "left lifeline must be vertical");
        assert_eq!(f.right_lifeline.from.x, f.right_lifeline.to.x, "right lifeline must be vertical");
        assert!(f.left_lifeline.to.y > f.left_lifeline.from.y, "lifeline extends downward");
    }

    #[test]
    fn messages_are_horizontal_and_respect_direction() {
        let ms = three_messages();
        let f = interaction_lane(InteractionLaneArgs {
            left_id: "c", left_label: "Client",
            right_id: "s", right_label: "Server",
            origin: (60, 100),
            lane_width: 700,
            lifeline_length: 280,
            messages: &ms,
        });
        for m in &f.messages {
            assert_eq!(m.from.y, m.to.y, "message must be horizontal");
        }
        // first message: left→right
        assert!(f.messages[0].from.x < f.messages[0].to.x);
        // second + third: right→left
        assert!(f.messages[1].from.x > f.messages[1].to.x);
        assert!(f.messages[2].from.x > f.messages[2].to.x);
    }

    #[test]
    fn messages_carry_their_labels_and_roles() {
        let ms = three_messages();
        let f = interaction_lane(InteractionLaneArgs {
            left_id: "c", left_label: "Client",
            right_id: "s", right_label: "Server",
            origin: (60, 100),
            lane_width: 700,
            lifeline_length: 280,
            messages: &ms,
        });
        assert_eq!(f.messages[0].label.as_deref(), Some("request"));
        assert_eq!(f.messages[0].role, Role::Accent);
        assert_eq!(f.messages[2].label.as_deref(), Some("response"));
        assert_eq!(f.messages[2].role, Role::Success);
    }

    #[test]
    fn messages_stack_top_to_bottom_in_declaration_order() {
        let ms = three_messages();
        let f = interaction_lane(InteractionLaneArgs {
            left_id: "c", left_label: "Client",
            right_id: "s", right_label: "Server",
            origin: (60, 100),
            lane_width: 700,
            lifeline_length: 280,
            messages: &ms,
        });
        for w in f.messages.windows(2) {
            assert!(w[0].from.y < w[1].from.y, "messages stack top→bottom");
        }
    }

    #[test]
    fn is_deterministic() {
        let ms = three_messages();
        let args = || InteractionLaneArgs {
            left_id: "c", left_label: "Client",
            right_id: "s", right_label: "Server",
            origin: (60, 100),
            lane_width: 700,
            lifeline_length: 280,
            messages: &ms,
        };
        assert_eq!(interaction_lane(args()), interaction_lane(args()));
    }
}
