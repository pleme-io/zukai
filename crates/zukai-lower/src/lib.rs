//! zukai-lower — `DiagramSpec → iac_forge::sexpr::SExpr`.
//!
//! Realizes "four Lisps → one IR" (`theory/THEORY.md` §VIII.1, §III.1) for
//! the diagram domain: the typed `DiagramSpec` collapses to the canonical
//! SExpr that every other typescape citizen shares. From here, the
//! `iac-forge` morphism / pipeline / transform machinery applies — that's
//! what makes "arbitrary build steps before delivery" structural rather
//! than ad-hoc.
//!
//! v0 lowering covers canvas + hero. Nested collections
//! (regions/nodes/edges/bands) extend the same pattern (small follow-up
//! compound — one `lower_*` helper each).

#![forbid(unsafe_code)]

use iac_forge::sexpr::{self, SExpr};
use zukai_model::{Canvas, Diagram, Hero};

/// Lower a `Canvas` to canonical SExpr: `(canvas :width N :height N)`.
#[must_use]
pub fn lower_canvas(c: &Canvas) -> SExpr {
    sexpr::struct_expr(
        "canvas",
        vec![
            ("width", SExpr::Integer(i64::from(c.width))),
            ("height", SExpr::Integer(i64::from(c.height))),
        ],
    )
}

/// Lower a `Hero` to canonical SExpr: `(hero :title "…" [:subtitle "…"])`.
#[must_use]
pub fn lower_hero(h: &Hero) -> SExpr {
    let mut fields = vec![("title", SExpr::String(h.title.clone()))];
    if let Some(sub) = &h.subtitle {
        fields.push(("subtitle", SExpr::String(sub.clone())));
    }
    sexpr::struct_expr("hero", fields)
}

/// Lower the entire `Diagram` to a single canonical SExpr — the same IR
/// every typescape citizen reduces to (`theory/THEORY.md` §VIII.1).
#[must_use]
pub fn lower(d: &Diagram) -> SExpr {
    sexpr::struct_expr(
        "diagram",
        vec![
            ("canvas", lower_canvas(&d.canvas)),
            ("hero", lower_hero(&d.hero)),
            // TODO compound: regions / nodes / edges / bands as nested List
            // SExprs — small follow-up, one helper each.
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use zukai_model::{Canvas, Diagram, Hero};

    #[test]
    fn canvas_lowers_to_canonical_sexpr_roundtrips() {
        let canvas = Canvas { width: 960, height: 560 };
        let sexpr = lower_canvas(&canvas);
        let emitted = sexpr.emit();
        // Parse back through iac-forge's canonical reader.
        let parsed = SExpr::parse(&emitted).expect("canonical round-trip");
        assert_eq!(parsed, sexpr, "canonical round-trip must be byte-stable");
    }

    #[test]
    fn lower_canvas_content_hash_is_stable() {
        let c = Canvas { width: 960, height: 560 };
        let h1 = lower_canvas(&c).content_hash().to_hex();
        let h2 = lower_canvas(&c).content_hash().to_hex();
        assert_eq!(h1, h2, "BLAKE3 content hash must be deterministic");
    }

    #[test]
    fn lower_diagram_smoke() {
        let d = Diagram::new(
            Canvas { width: 960, height: 560 },
            Hero {
                title: "Demo".into(),
                subtitle: Some("a subtitle".into()),
            },
        );
        let sexpr = lower(&d);
        let emitted = sexpr.emit();
        assert!(emitted.starts_with("(diagram"), "must be a (diagram …) form");
        assert!(emitted.contains("(canvas"), "must contain (canvas …)");
        assert!(emitted.contains("(hero"), "must contain (hero …)");
    }

    #[test]
    fn lower_is_deterministic() {
        // The whole point of canonicalizing through SExpr is byte-stability.
        let d = Diagram::new(
            Canvas { width: 960, height: 560 },
            Hero { title: "Demo".into(), subtitle: None },
        );
        assert_eq!(lower(&d).emit(), lower(&d).emit());
    }
}
