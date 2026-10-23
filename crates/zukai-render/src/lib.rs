//! zukai-render — render morphisms. A [`Renderer`] is the diagram-domain
//! Synthesizer (`theory/THEORY.md` §III.2): a pure total
//! `(Diagram, &dyn Theme) -> Vec<u8>`. Bytes are the unified leaf abstraction
//! — UTF-8 text for [`Svg`]/Mermaid, binary for [`Png`].
//!
//! SVG is a *leaf* output, so string building is idiomatic
//! (`ishou-render/src/svg.rs` precedent); the no-`format!()` rule binds
//! *intermediate* code, not leaf artifacts.
//!
//! COMPOUNDING NOTE: [`Theme`] is the seam where `ishou`'s `TokenSet` plugs in
//! (see [`IshouTheme`]), and a private `bordado` overlay specializes it in a
//! private crate. [`GenericTheme`] below is the public, borealis-flavored
//! default — it carries NO private-overlay brand hex (enforced by the
//! `*_keeps_overlay_brand_private` tests).

#![forbid(unsafe_code)]

use std::fmt::Write as _;
use zukai_model::{Band, Diagram, Edge, Node, Rect, Region, Role, Stroke};

mod mermaid;
pub use mermaid::Mermaid;

mod ishou_theme;
pub use ishou_theme::IshouTheme;

mod attestation;
pub use attestation::RenderAttestation;

pub mod confluence;

/// The theming seam. `ishou` implements this over its `TokenSet`
/// ([`IshouTheme`]); a private `bordado` overlay specializes it privately.
pub trait Theme {
    fn bg(&self) -> &str;
    fn panel(&self) -> &str;
    fn fg(&self) -> &str;
    fn muted(&self) -> &str;
    fn border(&self) -> &str;
    fn font(&self) -> &str;
    /// Color for a semantic [`Role`].
    fn role(&self, role: Role) -> &str;
}

/// Generic, public, borealis-flavored theme. Evokes the aurora palette
/// WITHOUT any private-overlay brand hex — that separation *is* the
/// genericization.
#[derive(Debug, Clone, Copy, Default)]
pub struct GenericTheme;

impl Theme for GenericTheme {
    fn bg(&self) -> &str { "#0A1726" }
    fn panel(&self) -> &str { "#0B1D32" }
    fn fg(&self) -> &str { "#EAF1F8" }
    fn muted(&self) -> &str { "#9DB4D0" }
    fn border(&self) -> &str { "#2A3F5C" }
    fn font(&self) -> &str { "-apple-system, Segoe UI, Roboto, Helvetica, Arial, sans-serif" }
    fn role(&self, role: Role) -> &str {
        match role {
            Role::Neutral => "#9DB4D0",
            Role::Master => "#22D7EC",
            Role::Read => "#2E90FF",
            Role::Accent => "#17C9A8",
            Role::Success => "#22E0A6",
            Role::Warning => "#E8A23C",
            Role::Danger => "#FF5C7A",
            Role::Info => "#2E90FF",
        }
    }
}

/// Render targets (AST domains). The add-a-target recipe is in
/// `docs/DESIGN.md` §9 — drop a new variant + a new [`Renderer`] impl.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Target {
    Svg,
    Png,
    Mermaid,
}

/// A render morphism: typed diagram + theme → a leaf artifact (bytes).
pub trait Renderer {
    fn target(&self) -> Target;
    fn render(&self, d: &Diagram, t: &dyn Theme) -> Vec<u8>;
}

/// SVG backend (UTF-8 text).
#[derive(Debug, Clone, Copy, Default)]
pub struct Svg;

impl Renderer for Svg {
    fn target(&self) -> Target {
        Target::Svg
    }

    fn render(&self, d: &Diagram, t: &dyn Theme) -> Vec<u8> {
        let mut s = String::new();
        let _ = write!(
            s,
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" font-family="{font}">"#,
            w = d.canvas.width,
            h = d.canvas.height,
            font = t.font()
        );
        let _ = write!(
            s,
            r#"<rect width="{}" height="{}" fill="{}"/>"#,
            d.canvas.width,
            d.canvas.height,
            t.bg()
        );
        let _ = write!(
            s,
            r#"<text x="40" y="46" font-size="24" font-weight="700" fill="{}">{}</text>"#,
            t.fg(),
            esc(&d.hero.title)
        );
        if let Some(sub) = &d.hero.subtitle {
            let _ = write!(
                s,
                r#"<text x="40" y="70" font-size="14" fill="{}">{}</text>"#,
                t.muted(),
                esc(sub)
            );
        }
        for b in &d.bands {
            band(&mut s, b, t);
        }
        for r in &d.regions {
            region(&mut s, r, t);
        }
        for n in &d.nodes {
            node(&mut s, n, t);
        }
        for e in &d.edges {
            edge(&mut s, e, t);
        }
        s.push_str("</svg>");
        s.into_bytes()
    }
}

/// PNG backend (binary). Renders [`Svg`] then rasterizes via `resvg`.
///
/// Compounding: Png does not duplicate any drawing logic — it composes the
/// existing SVG morphism with a single rasterization step. Adding a new
/// theme/primitive automatically benefits both targets.
#[derive(Debug, Clone, Copy, Default)]
pub struct Png;

impl Renderer for Png {
    fn target(&self) -> Target {
        Target::Png
    }

    fn render(&self, d: &Diagram, t: &dyn Theme) -> Vec<u8> {
        let svg_bytes = Svg.render(d, t);
        let mut opt = resvg::usvg::Options::default();
        let db = std::sync::Arc::make_mut(&mut opt.fontdb);
        db.load_system_fonts();
        let tree = resvg::usvg::Tree::from_data(&svg_bytes, &opt)
            .expect("zukai-render: usvg parse of own SVG must succeed");
        let size = tree.size().to_int_size();
        let mut pixmap = resvg::tiny_skia::Pixmap::new(size.width(), size.height())
            .expect("zukai-render: tiny_skia pixmap alloc");
        resvg::render(
            &tree,
            resvg::tiny_skia::Transform::default(),
            &mut pixmap.as_mut(),
        );
        pixmap.encode_png().expect("zukai-render: png encode")
    }
}

fn region(s: &mut String, r: &Region, t: &dyn Theme) {
    let Rect { x, y, w, h } = r.at;
    let _ = write!(
        s,
        r#"<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="14" fill="{}" stroke="{}"/>"#,
        t.panel(),
        t.border()
    );
    let _ = write!(
        s,
        r#"<rect x="{x}" y="{y}" width="{w}" height="44" rx="14" fill="{}"/>"#,
        t.role(r.role)
    );
    let _ = write!(
        s,
        r#"<text x="{}" y="{}" font-size="15" font-weight="700" fill="{}">{}</text>"#,
        x + 20,
        y + 28,
        t.bg(),
        esc(&r.label)
    );
    if let Some(sub) = &r.sublabel {
        let _ = write!(
            s,
            r#"<text x="{}" y="{}" font-size="12" fill="{}">{}</text>"#,
            x + 20,
            y + (h as i32) - 16,
            t.muted(),
            esc(sub)
        );
    }
}

fn node(s: &mut String, n: &Node, t: &dyn Theme) {
    let Rect { x, y, w, h } = n.at;
    let _ = write!(
        s,
        r#"<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="8" fill="{}" stroke="{}"/>"#,
        t.bg(),
        t.role(n.role)
    );
    let _ = write!(
        s,
        r#"<text x="{}" y="{}" font-size="12" fill="{}" text-anchor="middle">{}</text>"#,
        x + (w as i32) / 2,
        y + (h as i32) / 2 + 4,
        t.fg(),
        esc(&n.label)
    );
}

fn band(s: &mut String, b: &Band, t: &dyn Theme) {
    let Rect { x, y, w, h } = b.at;
    let _ = write!(
        s,
        r#"<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="10" fill="{}"/>"#,
        t.role(b.role)
    );
    let _ = write!(
        s,
        r#"<text x="{}" y="{}" font-size="13" font-weight="700" fill="{}" text-anchor="middle">{}</text>"#,
        x + (w as i32) / 2,
        y + (h as i32) / 2 + 4,
        t.bg(),
        esc(&b.label)
    );
}

fn edge(s: &mut String, e: &Edge, t: &dyn Theme) {
    let dash = match e.stroke {
        Stroke::Dashed => r#" stroke-dasharray="5 4""#,
        Stroke::Solid => "",
    };
    let _ = write!(
        s,
        r#"<path d="M{} {} L{} {}" stroke="{}" stroke-width="1.8" fill="none"{}/>"#,
        e.from.x,
        e.from.y,
        e.to.x,
        e.to.y,
        t.role(e.role),
        dash
    );
    if let Some(l) = &e.label {
        let _ = write!(
            s,
            r#"<text x="{}" y="{}" font-size="11" fill="{}" text-anchor="middle">{}</text>"#,
            (e.from.x + e.to.x) / 2,
            (e.from.y + e.to.y) / 2 - 6,
            t.muted(),
            esc(l)
        );
    }
}

/// Minimal XML text escaping for leaf SVG content.
fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use zukai_model::{Canvas, Edge, Hero, Id, Marker, Node, Point, Rect, Region, Role, Stroke};

    /// Sentinel hexes the private `bordado` overlay would use. The boundary
    /// proof: a public render must never contain any of these.
    const OVERLAY_BRAND_SENTINELS: &[&str] =
        &["#01D9C1", "#3181F8", "#3D68FF", "#01d9c1", "#3181f8"];

    fn sample() -> Diagram {
        Diagram::new(
            Canvas { width: 960, height: 560 },
            Hero {
                title: "Multi-Region".into(),
                subtitle: Some("nearest healthy region".into()),
            },
        )
        .with_band(Band {
            label: "Gateway".into(),
            sublabel: None,
            role: Role::Neutral,
            at: Rect { x: 300, y: 486, w: 360, h: 48 },
        })
        .with_region(Region {
            id: Id::new("eu"),
            label: "Frankfurt".into(),
            sublabel: Some("master".into()),
            role: Role::Master,
            badge: None,
            at: Rect { x: 40, y: 200, w: 420, h: 250 },
        })
        .with_region(Region {
            id: Id::new("ap"),
            label: "Singapore".into(),
            sublabel: Some("read".into()),
            role: Role::Read,
            badge: None,
            at: Rect { x: 500, y: 200, w: 420, h: 250 },
        })
        .with_node(Node {
            id: Id::new("vault"),
            label: "Vault".into(),
            role: Role::Master,
            region: Some(Id::new("eu")),
            at: Rect { x: 60, y: 262, w: 120, h: 34 },
        })
        .with_edge(Edge {
            from: Point { x: 460, y: 300 },
            to: Point { x: 500, y: 300 },
            stroke: Stroke::Dashed,
            marker: Marker::Arrow,
            role: Role::Accent,
            label: Some("replicate".into()),
        })
    }

    fn as_str(b: &[u8]) -> &str {
        std::str::from_utf8(b).expect("svg is utf-8")
    }

    // ── Svg ─────────────────────────────────────────────────────────────

    #[test]
    fn svg_render_is_deterministic() {
        let (d, t) = (sample(), GenericTheme);
        assert_eq!(
            Svg.render(&d, &t),
            Svg.render(&d, &t),
            "identical (spec, theme) must produce byte-identical output"
        );
    }

    #[test]
    fn svg_envelope_is_wellformed() {
        let out = Svg.render(&sample(), &GenericTheme);
        let s = as_str(&out);
        assert!(s.starts_with("<svg"), "must open with <svg");
        assert!(s.ends_with("</svg>"), "must close with </svg>");
        assert_eq!(s.matches("<svg").count(), 1);
        assert_eq!(s.matches("</svg>").count(), 1);
    }

    #[test]
    fn svg_colors_come_from_theme_not_literals() {
        let out = Svg.render(&sample(), &GenericTheme);
        assert!(
            as_str(&out).contains(GenericTheme.role(Role::Master)),
            "a master region must carry the theme's master role color"
        );
    }

    #[test]
    fn svg_generic_theme_keeps_overlay_brand_private() {
        // The boundary, proven: zukai's generic SVG output must never carry
        // the private overlay's brand sentinel hexes.
        let out = Svg.render(&sample(), &GenericTheme);
        let s = as_str(&out);
        for hex in OVERLAY_BRAND_SENTINELS {
            assert!(
                !s.contains(hex),
                "generic SVG leaked private-overlay brand hex {hex}"
            );
        }
    }

    #[test]
    fn svg_leaf_text_is_escaped() {
        let mut d = sample();
        d.hero.title = "A & B <x>".into();
        let out = Svg.render(&d, &GenericTheme);
        assert!(as_str(&out).contains("A &amp; B &lt;x&gt;"));
    }

    // ── Png ─────────────────────────────────────────────────────────────

    /// PNG magic: 0x89 P N G CR LF SUB LF
    const PNG_MAGIC: &[u8] = &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];

    #[test]
    fn png_starts_with_magic() {
        let out = Png.render(&sample(), &GenericTheme);
        assert!(out.starts_with(PNG_MAGIC), "must be a real PNG (magic bytes)");
        assert!(out.len() > 200, "PNG too small to be a real render: {} bytes", out.len());
    }

    #[test]
    fn png_render_is_deterministic() {
        let (d, t) = (sample(), GenericTheme);
        assert_eq!(
            Png.render(&d, &t),
            Png.render(&d, &t),
            "identical (spec, theme) must produce byte-identical PNG output"
        );
    }

    #[test]
    fn png_target_is_distinct_from_svg() {
        assert_ne!(Svg.target(), Png.target());
        assert_eq!(Svg.target(), Target::Svg);
        assert_eq!(Png.target(), Target::Png);
    }

    // ── Mermaid ─────────────────────────────────────────────────────────

    #[test]
    fn mermaid_render_is_deterministic() {
        let (d, t) = (sample(), GenericTheme);
        assert_eq!(Mermaid.render(&d, &t), Mermaid.render(&d, &t));
    }

    #[test]
    fn mermaid_starts_with_flowchart() {
        let out = Mermaid.render(&sample(), &GenericTheme);
        assert!(as_str(&out).starts_with("flowchart TD\n"));
    }

    #[test]
    fn mermaid_has_classdefs_per_role() {
        let out = Mermaid.render(&sample(), &GenericTheme);
        let s = as_str(&out);
        for class in ["master", "read", "accent", "neutral"] {
            let needle = format!("classDef {class}");
            assert!(s.contains(&needle), "missing classDef {class}");
        }
    }

    #[test]
    fn mermaid_keeps_overlay_brand_private() {
        let out = Mermaid.render(&sample(), &GenericTheme);
        let s = as_str(&out);
        for hex in OVERLAY_BRAND_SENTINELS {
            assert!(!s.contains(hex), "mermaid leaked private-overlay brand hex {hex}");
        }
    }

    #[test]
    fn mermaid_target_distinct() {
        assert_ne!(Mermaid.target(), Svg.target());
        assert_ne!(Mermaid.target(), Png.target());
        assert_eq!(Mermaid.target(), Target::Mermaid);
    }

    // ── IshouTheme (the first pleme-io wiring) ─────────────────────────

    #[test]
    fn ishou_theme_pleme_renders_real_svg() {
        let out = Svg.render(&sample(), &IshouTheme::pleme());
        let s = as_str(&out);
        assert!(s.starts_with("<svg"), "ishou-themed SVG must be well-formed");
        assert!(out.len() > 500, "ishou-themed render too small: {} bytes", out.len());
    }

    #[test]
    fn ishou_theme_keeps_overlay_brand_private() {
        // Nord palette ≠ private overlay's brand — the boundary holds on the
        // real pleme-io wiring too.
        let out = Svg.render(&sample(), &IshouTheme::pleme());
        let s = as_str(&out);
        for hex in OVERLAY_BRAND_SENTINELS {
            assert!(!s.contains(hex), "IshouTheme leaked private-overlay brand hex {hex}");
        }
    }

    #[test]
    fn ishou_and_generic_produce_different_bytes() {
        // ishou Nord and borealis aurora are different palettes — the bytes
        // must differ, proving the theme seam actually drives the render.
        let g = Svg.render(&sample(), &GenericTheme);
        let i = Svg.render(&sample(), &IshouTheme::pleme());
        assert_ne!(g, i, "two distinct themes must produce distinct bytes");
    }

    #[test]
    fn ishou_theme_is_deterministic_too() {
        let (d, t) = (sample(), IshouTheme::pleme());
        assert_eq!(Svg.render(&d, &t), Svg.render(&d, &t));
    }

    // ── Three-pillar attestation (THEORY.md §V.3 + Pillar 10) ──────────

    #[test]
    fn render_attestation_is_byte_stable_across_renders() {
        // Same (spec, theme) ⇒ same artifact bytes ⇒ same attestation root.
        // The attestation IS the determinism gate elevated to a cryptographic
        // receipt.
        let bytes = Svg.render(&sample(), &GenericTheme);
        let a1 = RenderAttestation::new(&bytes, b"defdiagram-canonical-bytes", b"author=zukai-test");
        let a2 = RenderAttestation::new(&bytes, b"defdiagram-canonical-bytes", b"author=zukai-test");
        assert_eq!(a1.root, a2.root);
        assert_eq!(a1.root_hex().len(), 64);
    }
}
