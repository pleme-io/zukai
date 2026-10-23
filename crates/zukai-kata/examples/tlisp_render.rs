//! tlisp_render — Phase A bridge end-to-end demo. Read a `.tlisp` source
//! file from disk, compile to a typed `Diagram` via `TataraDomain`, render
//! through the same `Renderer` seam every kata-authored diagram uses.
//!
//! This is the acceptance proof for tatara-lisp as the diagram authoring
//! surface — same render path as the kata examples, source is plain text
//! that an agent can write/read/edit naturally.
//!
//! Run: `cargo run --example tlisp_render -p zukai-kata`

use std::fs;
use tatara_lisp::{domain::TataraDomain, reader::read};
use zukai_model::Diagram;
use zukai_render::{GenericTheme, Mermaid, Png, Renderer, Svg};

/// `.tlisp` source — checked into `zukai/specs/diagrams/`. Compile-time inlined.
const SPEC: &str = include_str!("../../../specs/diagrams/example_2region.tlisp");

fn main() {
    let forms = read(SPEC).expect("reader parses well-formed .tlisp source");
    let form = forms.first().expect("at least one top-level form in the source");
    let d = Diagram::compile_from_sexp(form)
        .expect("Diagram::compile_from_sexp succeeds — .tlisp → typed Diagram");

    let theme = GenericTheme;
    let svg = Svg.render(&d, &theme);
    let png = Png.render(&d, &theme);
    let mmd = Mermaid.render(&d, &theme);

    let svg_path = "/tmp/zukai-tlisp-demo.svg";
    let png_path = "/tmp/zukai-tlisp-demo.png";
    let mmd_path = "/tmp/zukai-tlisp-demo.mmd";
    fs::write(svg_path, &svg).expect("write svg");
    fs::write(png_path, &png).expect("write png");
    fs::write(mmd_path, &mmd).expect("write mmd");

    println!(".tlisp-authored diagram rendered:");
    println!(
        "  source:   {} bytes (specs/diagrams/example_2region.tlisp)",
        SPEC.len()
    );
    println!(
        "  parsed:   canvas={}×{}  hero={:?}",
        d.canvas.width, d.canvas.height, d.hero.title
    );
    println!(
        "  primitives: {} regions, {} nodes, {} edges, {} bands",
        d.regions.len(),
        d.nodes.len(),
        d.edges.len(),
        d.bands.len()
    );
    println!("  svg:  {:>6} bytes → {svg_path}", svg.len());
    println!("  png:  {:>6} bytes → {png_path}", png.len());
    println!("  mmd:  {:>6} bytes → {mmd_path}", mmd.len());
}
