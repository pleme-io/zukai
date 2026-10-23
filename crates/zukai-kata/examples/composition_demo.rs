//! Multi-pattern composition demo. Renders three distinct kata patterns
//! to PNG — `multi_region` (N=3, exercising the v1 extension), `pillars`,
//! and `event_timeline` — proving the library composes for a real
//! multi-diagram Confluence page.
//!
//! Run: `cargo run --example composition_demo -p zukai-kata`

use std::fs;
use zukai_kata::{
    closing, event_timeline, hero, multi_region, pillars, ClosingArgs, EventArgs,
    EventTimelineArgs, GatewayArgs, HeroArgs, MultiRegionArgs, PillarArgs, PillarsArgs,
    RegionPatternArgs, ServiceArgs,
};
use zukai_model::{Canvas, Diagram, Role};
use zukai_render::{GenericTheme, Png, Renderer};

fn main() {
    let theme = GenericTheme;

    // ── Diagram 1: multi_region at N=3 (the v1 extension) ──────────────
    let mr3 = multi_region(MultiRegionArgs {
        title: "Multi-Region — primary + two replicas",
        subtitle: Some("v1 extension · star replication · scaled widths"),
        primary: RegionPatternArgs {
            id: "primary",
            label: "Primary",
            sublabel: Some("read + write"),
            role: Role::Master,
            badge_text: Some("MASTER · R/W"),
            services: vec![
                ServiceArgs { id: "svc-a", label: "Service A", role: Role::Master },
                ServiceArgs { id: "svc-b", label: "Service B", role: Role::Master },
                ServiceArgs { id: "svc-c", label: "Service C", role: Role::Master },
            ],
        },
        replicas: vec![
            RegionPatternArgs {
                id: "r1",
                label: "Replica 1",
                sublabel: Some("read-only"),
                role: Role::Read,
                badge_text: Some("READ"),
                services: vec![
                    ServiceArgs { id: "svc-a-r1", label: "Service A (ro)", role: Role::Read },
                    ServiceArgs { id: "svc-b-r1", label: "Service B (ro)", role: Role::Read },
                ],
            },
            RegionPatternArgs {
                id: "r2",
                label: "Replica 2",
                sublabel: Some("read-only"),
                role: Role::Read,
                badge_text: Some("READ"),
                services: vec![
                    ServiceArgs { id: "svc-a-r2", label: "Service A (ro)", role: Role::Read },
                    ServiceArgs { id: "svc-b-r2", label: "Service B (ro)", role: Role::Read },
                ],
            },
        ],
        gateway: Some(GatewayArgs { label: "Client network — gateway (outbound only)" }),
    });
    let mr3_png = Png.render(&mr3, &theme);
    fs::write("/tmp/zukai-comp-multi-region-n3.png", &mr3_png).unwrap();

    // ── Diagram 2: pillars (4-principle foundation) ────────────────────
    let pillars_d = build_pillars_diagram();
    let pillars_png = Png.render(&pillars_d, &theme);
    fs::write("/tmp/zukai-comp-pillars.png", &pillars_png).unwrap();

    // ── Diagram 3: event_timeline (change-window plan) ─────────────────
    let timeline_d = build_timeline_diagram();
    let timeline_png = Png.render(&timeline_d, &theme);
    fs::write("/tmp/zukai-comp-timeline.png", &timeline_png).unwrap();

    println!("composition demo rendered:");
    println!("  multi_region N=3  {:>6} bytes → /tmp/zukai-comp-multi-region-n3.png", mr3_png.len());
    println!("  pillars            {:>6} bytes → /tmp/zukai-comp-pillars.png", pillars_png.len());
    println!("  event_timeline     {:>6} bytes → /tmp/zukai-comp-timeline.png", timeline_png.len());
}

fn build_pillars_diagram() -> Diagram {
    let canvas = Canvas { width: 960, height: 260 };
    let h = hero(HeroArgs {
        title: "Four Operating Principles",
        subtitle: Some("the foundations every diagram rests on"),
    });
    let pillar_nodes = pillars(PillarsArgs {
        origin: (40, 100),
        width: 880,
        pillar_height: 120,
        gap: 20,
        pillars: &[
            PillarArgs { id: "p1", label: "Principle 1", role: Role::Master },
            PillarArgs { id: "p2", label: "Principle 2", role: Role::Read },
            PillarArgs { id: "p3", label: "Principle 3", role: Role::Accent },
            PillarArgs { id: "p4", label: "Principle 4", role: Role::Neutral },
        ],
    });
    let mut d = Diagram::new(canvas, h);
    for n in pillar_nodes {
        d = d.with_node(n);
    }
    d
}

fn build_timeline_diagram() -> Diagram {
    let canvas = Canvas { width: 960, height: 280 };
    let h = hero(HeroArgs {
        title: "Change Window — example sequence",
        subtitle: Some("ordered milestones with roles"),
    });
    let tl = event_timeline(EventTimelineArgs {
        origin: (60, 180),
        width: 840,
        events: &[
            EventArgs { id: "t-30", label: "T-30 · prep",       role: Role::Neutral },
            EventArgs { id: "t-0",  label: "T+0 · cutover",     role: Role::Accent },
            EventArgs { id: "t+30", label: "T+30 · verify",     role: Role::Success },
            EventArgs { id: "t+60", label: "T+60 · all-clear",  role: Role::Success },
        ],
    });
    let mut d = Diagram::new(canvas, h);
    d = d.with_edge(tl.baseline);
    for n in tl.event_chips {
        d = d.with_node(n);
    }
    // Close with a summary band.
    d = d.with_band(closing(ClosingArgs {
        label: "All milestones complete · post-mortem to follow",
        sublabel: None,
        role: Role::Success,
    }));
    d
}
