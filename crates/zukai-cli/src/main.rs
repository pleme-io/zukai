//! zukai — CLI for the typed diagram domain. v0: built-in demo renderer.
//!
//! NEXT COMPOUND: once the six-line contract is wired (TataraDomain on
//! `DiagramSpec`), this gains `render --spec FILE.{yaml,scm,nix}` that loads
//! any of the Four Lisps via the canonical SExpr IR (`theory/THEORY.md` §II.2).

use clap::{Parser, Subcommand, ValueEnum};
use std::{fs, path::PathBuf, process::ExitCode};
use zukai_model::{Band, Canvas, Diagram, Edge, Hero, Id, Marker, Node, Point, Rect, Region, Role, Stroke};
use zukai_render::{GenericTheme, Mermaid, Png, Renderer, Svg, Target};

#[derive(Debug, Parser)]
#[command(name = "zukai", version, about = "typed diagram domain — render morphisms")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Render a built-in demo diagram to a leaf artifact.
    Render {
        /// Output target.
        #[arg(short, long, value_enum)]
        target: TargetArg,
        /// Output file path.
        #[arg(short, long)]
        out: PathBuf,
    },
    /// List available render targets.
    Targets,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum TargetArg {
    Svg,
    Png,
    Mermaid,
}

fn main() -> ExitCode {
    let Cli { cmd } = Cli::parse();
    match cmd {
        Cmd::Render { target, out } => {
            let d = demo();
            let theme = GenericTheme;
            let bytes = match target {
                TargetArg::Svg => Svg.render(&d, &theme),
                TargetArg::Png => Png.render(&d, &theme),
                TargetArg::Mermaid => Mermaid.render(&d, &theme),
            };
            if let Err(e) = fs::write(&out, &bytes) {
                eprintln!("zukai: write {}: {e}", out.display());
                return ExitCode::FAILURE;
            }
            eprintln!("zukai: wrote {} bytes → {}", bytes.len(), out.display());
            ExitCode::SUCCESS
        }
        Cmd::Targets => {
            for t in [Target::Svg, Target::Png, Target::Mermaid] {
                println!("{t:?}");
            }
            ExitCode::SUCCESS
        }
    }
}

fn demo() -> Diagram {
    Diagram::new(
        Canvas { width: 960, height: 560 },
        Hero {
            title: "zukai demo".into(),
            subtitle: Some("multi-region · theme = generic (borealis-flavored)".into()),
        },
    )
    .with_band(Band {
        label: "Global DNS — nearest healthy region".into(),
        sublabel: None,
        role: Role::Master,
        at: Rect { x: 300, y: 96, w: 360, h: 46 },
    })
    .with_region(Region {
        id: Id::new("eu"),
        label: "europe-west3 · Frankfurt".into(),
        sublabel: Some("master".into()),
        role: Role::Master,
        badge: None,
        at: Rect { x: 40, y: 200, w: 420, h: 250 },
    })
    .with_region(Region {
        id: Id::new("ap"),
        label: "asia-southeast1 · Singapore".into(),
        sublabel: Some("read".into()),
        role: Role::Read,
        badge: None,
        at: Rect { x: 500, y: 200, w: 420, h: 250 },
    })
    .with_node(Node { id: Id::new("vault"), label: "Vault".into(), role: Role::Master, region: Some(Id::new("eu")), at: Rect { x: 60, y: 262, w: 120, h: 34 } })
    .with_node(Node { id: Id::new("api"), label: "API gateway".into(), role: Role::Accent, region: Some(Id::new("eu")), at: Rect { x: 60, y: 350, w: 250, h: 34 } })
    .with_node(Node { id: Id::new("vro"), label: "Vault-ro".into(), role: Role::Read, region: Some(Id::new("ap")), at: Rect { x: 520, y: 262, w: 120, h: 34 } })
    .with_edge(Edge {
        from: Point { x: 460, y: 300 },
        to: Point { x: 500, y: 300 },
        stroke: Stroke::Dashed,
        marker: Marker::Arrow,
        role: Role::Accent,
        label: Some("replicate".into()),
    })
}
