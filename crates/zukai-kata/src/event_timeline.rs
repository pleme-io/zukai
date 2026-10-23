//! `kata::event_timeline` — horizontal timeline (baseline arrow) with N
//! labeled event chips above it. Events are evenly distributed along the
//! width; each chip carries its own `Role`. Use for change-window plans,
//! incident timelines, milestone runbooks.

use zukai_model::{Edge, Id, Marker, Node, Point, Rect, Role, Stroke};

const EVENT_W: u32 = 140;
const EVENT_H: u32 = 40;
/// Gap between the bottom of an event chip and the baseline.
const EVENT_GAP_FROM_BASELINE: i32 = 30;

/// One event on the timeline.
#[derive(Debug, Clone, Copy)]
pub struct EventArgs<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub role: Role,
}

/// Args for [`event_timeline`].
pub struct EventTimelineArgs<'a> {
    /// Left endpoint of the baseline.
    pub origin: (i32, i32),
    /// Baseline length in pixels.
    pub width: u32,
    pub events: &'a [EventArgs<'a>],
}

/// Fragment returned by [`event_timeline`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventTimelineFragment {
    /// Horizontal arrow representing the flow of time, left→right.
    pub baseline: Edge,
    /// Event chips above the baseline, evenly distributed.
    pub event_chips: Vec<Node>,
}

/// Build the baseline + the N event chips.
#[must_use]
pub fn event_timeline(args: EventTimelineArgs<'_>) -> EventTimelineFragment {
    let baseline_y = args.origin.1;
    let baseline = Edge {
        from: Point { x: args.origin.0, y: baseline_y },
        to: Point { x: args.origin.0 + args.width as i32, y: baseline_y },
        stroke: Stroke::Solid,
        marker: Marker::Arrow,
        role: Role::Neutral,
        label: None,
    };

    let n = args.events.len() as u32;
    if n == 0 {
        return EventTimelineFragment { baseline, event_chips: Vec::new() };
    }
    // n+1 divisions leave a margin at both ends.
    let dx = args.width / (n + 1);
    let event_chips: Vec<Node> = args
        .events
        .iter()
        .enumerate()
        .map(|(i, ev)| {
            let center_x = args.origin.0 + ((i as i32) + 1) * (dx as i32);
            Node {
                id: Id::new(ev.id),
                label: ev.label.into(),
                role: ev.role,
                region: None,
                at: Rect {
                    x: center_x - (EVENT_W as i32) / 2,
                    y: baseline_y - EVENT_GAP_FROM_BASELINE - EVENT_H as i32,
                    w: EVENT_W,
                    h: EVENT_H,
                },
            }
        })
        .collect();
    EventTimelineFragment { baseline, event_chips }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn four_events() -> [EventArgs<'static>; 4] {
        [
            EventArgs { id: "t-30", label: "T-30 · prep",     role: Role::Neutral },
            EventArgs { id: "t-0",  label: "T+0 · cutover",   role: Role::Accent },
            EventArgs { id: "t+30", label: "T+30 · verify",   role: Role::Success },
            EventArgs { id: "t+60", label: "T+60 · all-clear", role: Role::Success },
        ]
    }

    #[test]
    fn has_baseline_and_n_event_chips() {
        let es = four_events();
        let f = event_timeline(EventTimelineArgs {
            origin: (60, 320),
            width: 840,
            events: &es,
        });
        assert_eq!(f.event_chips.len(), 4);
        assert_eq!(f.baseline.from.y, 320);
        assert_eq!(f.baseline.to.x, 60 + 840);
    }

    #[test]
    fn chips_sit_above_the_baseline() {
        let es = four_events();
        let baseline_y = 320;
        let f = event_timeline(EventTimelineArgs {
            origin: (60, baseline_y),
            width: 840,
            events: &es,
        });
        for c in &f.event_chips {
            assert!(c.at.y + c.at.h as i32 <= baseline_y,
                "chip bottom must be at or above baseline");
        }
    }

    #[test]
    fn chips_carry_per_event_role() {
        let es = four_events();
        let f = event_timeline(EventTimelineArgs {
            origin: (60, 320),
            width: 840,
            events: &es,
        });
        assert_eq!(f.event_chips[0].role, Role::Neutral);
        assert_eq!(f.event_chips[1].role, Role::Accent);
        assert_eq!(f.event_chips[2].role, Role::Success);
        assert_eq!(f.event_chips[3].role, Role::Success);
    }

    #[test]
    fn chips_distribute_left_to_right_with_increasing_x() {
        let es = four_events();
        let f = event_timeline(EventTimelineArgs {
            origin: (60, 320),
            width: 840,
            events: &es,
        });
        for w in f.event_chips.windows(2) {
            assert!(w[0].at.x < w[1].at.x, "chips must lay out left-to-right");
        }
    }

    #[test]
    fn is_deterministic() {
        let es = four_events();
        let args = || EventTimelineArgs {
            origin: (60, 320),
            width: 840,
            events: &es,
        };
        assert_eq!(event_timeline(args()), event_timeline(args()));
    }

    #[test]
    fn empty_events_returns_only_baseline() {
        let f = event_timeline(EventTimelineArgs {
            origin: (0, 100),
            width: 1000,
            events: &[],
        });
        assert!(f.event_chips.is_empty());
        assert_eq!(f.baseline.from.x, 0);
        assert_eq!(f.baseline.to.x, 1000);
    }
}
