//! Acceptance example — author a multi-region architecture via the kata
//! library, render it to SVG + PNG + Mermaid via zukai-render. The
//! idiomatic authoring path: declarative args in, themed bytes out.
//!
//! Run: `cargo run --example multi_region -p zukai-kata`

use std::fs;
use zukai_kata::{
    multi_region, GatewayArgs, MultiRegionArgs, RegionPatternArgs, ServiceArgs,
};
use zukai_model::Role;
use zukai_render::{GenericTheme, Mermaid, Png, Renderer, Svg};

fn main() {
    let d = multi_region(MultiRegionArgs {
        title: "Multi-Region — one set of hostnames",
        subtitle: Some(
            "single tenant · latency- and health-routed to the nearest healthy region",
        ),
        primary: RegionPatternArgs {
            id: "primary",
            label: "Primary region",
            sublabel: Some("read + write"),
            role: Role::Master,
            badge_text: Some("MASTER · R/W"),
            services: vec![
                ServiceArgs { id: "svc-a", label: "Service A", role: Role::Master },
                ServiceArgs { id: "svc-b", label: "Service B", role: Role::Master },
                ServiceArgs { id: "svc-c", label: "Service C", role: Role::Master },
                ServiceArgs { id: "svc-d", label: "Service D", role: Role::Master },
                ServiceArgs { id: "svc-e", label: "Service E", role: Role::Master },
                ServiceArgs { id: "svc-f", label: "Service F", role: Role::Master },
            ],
        },
        replicas: vec![RegionPatternArgs {
            id: "replica",
            label: "Replica region",
            sublabel: Some("read replicas · low-latency"),
            role: Role::Read,
            badge_text: Some("READ · read-only"),
            services: vec![
                ServiceArgs { id: "svc-a-r", label: "Service A (ro)", role: Role::Read },
                ServiceArgs { id: "svc-b-r", label: "Service B (ro)", role: Role::Read },
                ServiceArgs { id: "svc-c-r", label: "Service C (ro)", role: Role::Read },
                ServiceArgs { id: "svc-d-r", label: "Service D (ro)", role: Role::Read },
                ServiceArgs { id: "svc-e-r", label: "Service E (ro)", role: Role::Read },
                ServiceArgs { id: "cache-r", label: "Cache", role: Role::Neutral },
            ],
        }],
        gateway: Some(GatewayArgs { label: "Client network — gateway (outbound only)" }),
    });

    let theme = GenericTheme;
    let svg = Svg.render(&d, &theme);
    let png = Png.render(&d, &theme);
    let mmd = Mermaid.render(&d, &theme);

    let svg_path = "/tmp/zukai-demo-multi-region.svg";
    let png_path = "/tmp/zukai-demo-multi-region.png";
    let mmd_path = "/tmp/zukai-demo-multi-region.mmd";
    fs::write(svg_path, &svg).expect("write svg");
    fs::write(png_path, &png).expect("write png");
    fs::write(mmd_path, &mmd).expect("write mmd");

    println!("kata-authored multi_region rendered:");
    println!("  svg  {:>6} bytes → {svg_path}", svg.len());
    println!("  png  {:>6} bytes → {png_path}", png.len());
    println!("  mmd  {:>6} bytes → {mmd_path}", mmd.len());
    println!("open: open {svg_path} && open {png_path}");
}
