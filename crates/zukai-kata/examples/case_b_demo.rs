//! Case B demo — lift an existing Mermaid block through `from_mermaid`
//! into a renderable `Diagram`, then render it to PNG via the same `Theme`
//! seam every kata-authored diagram flows through.
//!
//! Run: `cargo run --example case_b_demo -p zukai-kata`

use std::fs;
use zukai_kata::from_mermaid;
use zukai_render::{GenericTheme, Png, Renderer};

/// Representative Mermaid block — what a typical Confluence page would
/// contain today. v0 of `from_mermaid` supports edges one-per-line; chained
/// `A --> B --> C` on a single line is a v1 follow-up.
const MERMAID_SRC: &str = "\
flowchart LR
A[Frontend] -- HTTP --> B[Backend]
B -- query --> C[Database]
C -- result --> B
B -- JSON --> A
";

fn main() {
    let d = from_mermaid(MERMAID_SRC).expect("from_mermaid: parse should succeed");
    let png = Png.render(&d, &GenericTheme);

    let png_path = "/tmp/zukai-case-b.png";
    fs::write(png_path, &png).expect("write PNG");

    println!("Case B demo rendered:");
    println!("  source Mermaid (verbatim):");
    for line in MERMAID_SRC.lines() {
        println!("    {line}");
    }
    println!("  parsed: {} nodes, {} edges", d.nodes.len(), d.edges.len());
    println!("  output PNG: {} bytes → {png_path}", png.len());
}
