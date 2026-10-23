//! zukai-model — the typed diagram domain. Pure data; every visual attribute
//! resolves through a [`Role`], never a literal color. See `docs/DESIGN.md`.
//!
//! **tatara-lisp authoring surface (Phase A bridge, landed).** Every primitive
//! that contains custom-typed fields (`Hero`, `Badge`, `Region`, `Node`,
//! `Edge`, `Band`, `Diagram`) has a manual `impl TataraDomain` in the
//! private `lisp` module, so a complete diagram can be authored as one
//! `.tlisp` form and compiled through `tatara_lisp::reader::read` +
//! `Diagram::compile_from_sexp`. [`Canvas`] already carried the
//! `#[derive(TataraDomain)]` proof for scalar-only types; the manual
//! impls extend the bridge until tatara's derive macro grows recursion
//! into custom nested struct types.

#![forbid(unsafe_code)]

mod lisp;

/// Stable identifier for a diagram element. Phase F: derives `From<String>`
/// (manual impl) so `#[derive(TataraDomain)]` fields can use `#[tatara(via_string)]`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(pub String);

impl Id {
    #[must_use]
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Semantic role. The render morphism maps a role → a theme color; the model
/// never carries a hex. This seam is what keeps brand identity private: the
/// public engine knows roles, the private `bordado` overlay knows hexes.
// NOT `#[non_exhaustive]`: every renderer/theme should match exhaustively, so
// adding a new variant forces a compile error at every render site. That's the
// whole point of typed roles — exhaustiveness is the proof we don't drop one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, tatara_lisp::DeriveKeywordSexp)]
pub enum Role {
    Neutral,
    Master,
    Read,
    Accent,
    Success,
    Warning,
    Danger,
    Info,
}

/// Canvas extent in user units. Carries `#[derive(TataraDomain)]` —
/// authorable as `(defcanvas :width 960 :height 560)` and re-enters the
/// canonical SExpr IR via `compile_from_sexp` (`theory/THEORY.md` §II.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, tatara_lisp::DeriveTataraDomain)]
#[tatara(keyword = "defcanvas")]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
}

/// Axis-aligned rectangle in canvas units. Phase F: derives `TataraDomain` —
/// authorable as `(defrect :x N :y N :w N :h N)` directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, tatara_lisp::DeriveTataraDomain)]
#[tatara(keyword = "defrect")]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

/// A point in canvas units. Phase F: derives `TataraDomain` —
/// authorable as `(defpoint :x N :y N)` directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, tatara_lisp::DeriveTataraDomain)]
#[tatara(keyword = "defpoint")]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Title band at the top of a diagram. Phase F: derives `TataraDomain` —
/// authorable as `(defhero :title "…" :subtitle "…")` directly.
#[derive(Debug, Clone, PartialEq, Eq, tatara_lisp::DeriveTataraDomain)]
#[tatara(keyword = "defhero")]
pub struct Hero {
    pub title: String,
    pub subtitle: Option<String>,
}

/// A short status pill (e.g. "MASTER · R/W"). Phase F: full derive.
#[derive(Debug, Clone, PartialEq, Eq, tatara_lisp::DeriveTataraDomain)]
#[tatara(keyword = "defbadge")]
pub struct Badge {
    pub text: String,
    #[tatara(keyword_enum)] pub role: Role,
}

/// A bounded region card (e.g. a cloud region). Phase F: full derive.
#[derive(Debug, Clone, PartialEq, Eq, tatara_lisp::DeriveTataraDomain)]
#[tatara(keyword = "defregion")]
pub struct Region {
    #[tatara(via_string)] pub id: Id,
    pub label: String,
    pub sublabel: Option<String>,
    #[tatara(keyword_enum)] pub role: Role,
    #[tatara(domain)] pub badge: Option<Badge>,
    #[tatara(domain)] pub at: Rect,
}

/// A service chip / box. Phase F: full derive.
#[derive(Debug, Clone, PartialEq, Eq, tatara_lisp::DeriveTataraDomain)]
#[tatara(keyword = "defnode")]
pub struct Node {
    #[tatara(via_string)] pub id: Id,
    pub label: String,
    #[tatara(keyword_enum)] pub role: Role,
    #[tatara(via_string)] pub region: Option<Id>,
    #[tatara(domain)] pub at: Rect,
}

/// Stroke style for an edge. Exhaustive on purpose (see [`Role`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, tatara_lisp::DeriveKeywordSexp)]
pub enum Stroke {
    Solid,
    Dashed,
}

/// Arrowhead marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, tatara_lisp::DeriveKeywordSexp)]
#[non_exhaustive]
pub enum Marker {
    None,
    Arrow,
}

/// Directed connector between two points. Phase F: full derive.
#[derive(Debug, Clone, PartialEq, Eq, tatara_lisp::DeriveTataraDomain)]
#[tatara(keyword = "defedge")]
pub struct Edge {
    #[tatara(domain)] pub from: Point,
    #[tatara(domain)] pub to: Point,
    #[tatara(keyword_enum)] pub stroke: Stroke,
    #[tatara(keyword_enum)] pub marker: Marker,
    #[tatara(keyword_enum)] pub role: Role,
    pub label: Option<String>,
}

/// A full-width lane (routing band, customer-gateway band, …). Phase F: full derive.
#[derive(Debug, Clone, PartialEq, Eq, tatara_lisp::DeriveTataraDomain)]
#[tatara(keyword = "defband")]
pub struct Band {
    pub label: String,
    pub sublabel: Option<String>,
    #[tatara(keyword_enum)] pub role: Role,
    #[tatara(domain)] pub at: Rect,
}

/// The complete typed declaration. Every artifact is a projection of this.
/// Phase F: full derive — every nested field uses `#[tatara(domain)]`.
#[derive(Debug, Clone, PartialEq, Eq, tatara_lisp::DeriveTataraDomain)]
#[tatara(keyword = "defdiagram")]
#[non_exhaustive]
pub struct Diagram {
    #[tatara(domain)] pub canvas: Canvas,
    #[tatara(domain)] pub hero: Hero,
    #[tatara(domain)] pub regions: Vec<Region>,
    #[tatara(domain)] pub nodes: Vec<Node>,
    #[tatara(domain)] pub edges: Vec<Edge>,
    #[tatara(domain)] pub bands: Vec<Band>,
}

impl Diagram {
    #[must_use]
    pub fn new(canvas: Canvas, hero: Hero) -> Self {
        Self {
            canvas,
            hero,
            regions: Vec::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
            bands: Vec::new(),
        }
    }
    #[must_use]
    pub fn with_region(mut self, r: Region) -> Self {
        self.regions.push(r);
        self
    }
    #[must_use]
    pub fn with_node(mut self, n: Node) -> Self {
        self.nodes.push(n);
        self
    }
    #[must_use]
    pub fn with_edge(mut self, e: Edge) -> Self {
        self.edges.push(e);
        self
    }
    #[must_use]
    pub fn with_band(mut self, b: Band) -> Self {
        self.bands.push(b);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_compiles_from_lisp() {
        // The six-line contract proof: a primitive-only TataraDomain wiring.
        // Parses `(defcanvas :width 960 :height 560)` via the canonical
        // tatara-lisp reader, then derives the Canvas value through the
        // auto-generated `compile_from_sexp` impl.
        use tatara_lisp::TataraDomain;
        let forms = tatara_lisp::reader::read("(defcanvas :width 960 :height 560)")
            .expect("reader parses well-formed (defcanvas …)");
        let sexp = forms.first().expect("at least one top-level form");
        let canvas = Canvas::compile_from_sexp(sexp).expect("compile_from_sexp succeeds");
        assert_eq!(canvas, Canvas { width: 960, height: 560 });
    }

    #[test]
    fn builder_accumulates() {
        let d = Diagram::new(
            Canvas { width: 960, height: 560 },
            Hero { title: "T".into(), subtitle: None },
        )
        .with_region(Region {
            id: Id::new("eu"),
            label: "Frankfurt".into(),
            sublabel: None,
            role: Role::Master,
            badge: None,
            at: Rect { x: 40, y: 200, w: 420, h: 250 },
        })
        .with_node(Node {
            id: Id::new("vault"),
            label: "Vault".into(),
            role: Role::Master,
            region: Some(Id::new("eu")),
            at: Rect { x: 60, y: 262, w: 120, h: 34 },
        });
        assert_eq!(d.regions.len(), 1);
        assert_eq!(d.nodes.len(), 1);
        assert_eq!(d.regions[0].id.as_str(), "eu");
    }
}
