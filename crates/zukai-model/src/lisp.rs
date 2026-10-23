//! tatara-lisp bridge — manual `TataraDomain` impls for every primitive that
//! contains custom-typed fields. Complements `#[derive(TataraDomain)]` on
//! [`crate::Canvas`] which handles scalar-only types.
//!
//! Authoring surface — a complete diagram lives as one `.tlisp` form:
//!
//! ```lisp
//! (defdiagram
//!   :canvas  (defcanvas :width 960 :height 560)
//!   :hero    (defhero   :title "Multi-Region" :subtitle "primary + replica")
//!   :regions ((defregion :id "primary"
//!                        :label "Primary"
//!                        :role :master
//!                        :at (defrect :x 40 :y 200 :w 420 :h 250)))
//!   :nodes   ((defnode :id "svc-a"
//!                      :label "Service A"
//!                      :role :master
//!                      :region "primary"
//!                      :at (defrect :x 60 :y 270 :w 120 :h 34)))
//!   :edges   ()
//!   :bands   ())
//! ```
//!
//! Once the tatara `#[derive(TataraDomain)]` macro grows recursion into
//! custom struct types (open compounding-ledger item), every impl in this
//! file collapses to a one-line `#[derive]`.

// Phase F: only `KeywordSexp` (for `.to_keyword()` calls inside to_sexp
// methods) and `Sexp` (the return type) are needed in this module now.
// Everything else (`TataraDomain`, `parse_kwargs`, `extract_*`, etc.) is
// consumed inside derive-generated code over in lib.rs.
use tatara_lisp::{KeywordSexp, Sexp};

use crate::{Badge, Band, Canvas, Diagram, Edge, Hero, Node, Point, Rect, Region};

// ── Phase F: every primitive now uses `#[derive(TataraDomain)]` directly
// ── on the type (see lib.rs). The manual impls + helper parsers
// ── (parse_keyword / role_from_keyword / stroke_from_keyword /
// ── marker_from_keyword / list_of) that lived here are gone.
// ──
// ── Role/Stroke/Marker use `KeywordSexp` (derived). Id uses From<String>.
// ── Nested struct fields use `#[tatara(domain)]`. Recursion is mechanical.

// ── round-trip: typed value → Sexp (the inverse of compile_from_sexp) ──
//
// Every primitive that has a `compile_from_sexp` impl also gets a `to_sexp`
// method, so kata patterns (Phase B) can build typed values in Rust and
// return them to tatara-lisp as `Value::Sexp` for further composition.

use tatara_lisp::Atom;

fn s_kw(key: &str) -> Sexp {
    Sexp::keyword(key)
}

fn push_opt_str(list: &mut Vec<Sexp>, key: &str, val: Option<&str>) {
    if let Some(v) = val {
        list.push(s_kw(key));
        list.push(Sexp::string(v));
    }
}

impl Canvas {
    #[must_use]
    pub fn to_sexp(&self) -> Sexp {
        Sexp::List(vec![
            Sexp::symbol("defcanvas"),
            s_kw("width"), Sexp::int(self.width as i64),
            s_kw("height"), Sexp::int(self.height as i64),
        ])
    }
}

impl Rect {
    #[must_use]
    pub fn to_sexp(&self) -> Sexp {
        Sexp::List(vec![
            Sexp::symbol("defrect"),
            s_kw("x"), Sexp::int(self.x as i64),
            s_kw("y"), Sexp::int(self.y as i64),
            s_kw("w"), Sexp::int(self.w as i64),
            s_kw("h"), Sexp::int(self.h as i64),
        ])
    }
}

impl Point {
    #[must_use]
    pub fn to_sexp(&self) -> Sexp {
        Sexp::List(vec![
            Sexp::symbol("defpoint"),
            s_kw("x"), Sexp::int(self.x as i64),
            s_kw("y"), Sexp::int(self.y as i64),
        ])
    }
}

// Phase F: Role / Stroke / Marker get `to_keyword(self) -> &'static str` from
// the `KeywordSexp` derive — see lib.rs. To embed the keyword in a Sexp tree,
// wrap with `s_kw(role.to_keyword())`.

impl Hero {
    #[must_use]
    pub fn to_sexp(&self) -> Sexp {
        let mut v = vec![
            Sexp::symbol("defhero"),
            s_kw("title"), Sexp::string(&self.title),
        ];
        push_opt_str(&mut v, "subtitle", self.subtitle.as_deref());
        Sexp::List(v)
    }
}

impl Badge {
    #[must_use]
    pub fn to_sexp(&self) -> Sexp {
        Sexp::List(vec![
            Sexp::symbol("defbadge"),
            s_kw("text"), Sexp::string(&self.text),
            s_kw("role"), s_kw(self.role.to_keyword()),
        ])
    }
}

impl Region {
    #[must_use]
    pub fn to_sexp(&self) -> Sexp {
        let mut v = vec![
            Sexp::symbol("defregion"),
            s_kw("id"), Sexp::string(self.id.as_str()),
            s_kw("label"), Sexp::string(&self.label),
        ];
        push_opt_str(&mut v, "sublabel", self.sublabel.as_deref());
        v.push(s_kw("role"));
        v.push(s_kw(self.role.to_keyword()));
        if let Some(b) = &self.badge {
            v.push(s_kw("badge"));
            v.push(b.to_sexp());
        }
        v.push(s_kw("at"));
        v.push(self.at.to_sexp());
        Sexp::List(v)
    }
}

impl Node {
    #[must_use]
    pub fn to_sexp(&self) -> Sexp {
        let mut v = vec![
            Sexp::symbol("defnode"),
            s_kw("id"), Sexp::string(self.id.as_str()),
            s_kw("label"), Sexp::string(&self.label),
            s_kw("role"), s_kw(self.role.to_keyword()),
        ];
        if let Some(rid) = &self.region {
            v.push(s_kw("region"));
            v.push(Sexp::string(rid.as_str()));
        }
        v.push(s_kw("at"));
        v.push(self.at.to_sexp());
        Sexp::List(v)
    }
}

impl Edge {
    #[must_use]
    pub fn to_sexp(&self) -> Sexp {
        let mut v = vec![
            Sexp::symbol("defedge"),
            s_kw("from"), self.from.to_sexp(),
            s_kw("to"), self.to.to_sexp(),
            s_kw("stroke"), s_kw(self.stroke.to_keyword()),
            s_kw("marker"), s_kw(self.marker.to_keyword()),
            s_kw("role"), s_kw(self.role.to_keyword()),
        ];
        push_opt_str(&mut v, "label", self.label.as_deref());
        Sexp::List(v)
    }
}

impl Band {
    #[must_use]
    pub fn to_sexp(&self) -> Sexp {
        let mut v = vec![
            Sexp::symbol("defband"),
            s_kw("label"), Sexp::string(&self.label),
        ];
        push_opt_str(&mut v, "sublabel", self.sublabel.as_deref());
        v.push(s_kw("role"));
        v.push(s_kw(self.role.to_keyword()));
        v.push(s_kw("at"));
        v.push(self.at.to_sexp());
        Sexp::List(v)
    }
}

impl Diagram {
    #[must_use]
    pub fn to_sexp(&self) -> Sexp {
        Sexp::List(vec![
            Sexp::symbol("defdiagram"),
            s_kw("canvas"), self.canvas.to_sexp(),
            s_kw("hero"), self.hero.to_sexp(),
            s_kw("regions"), Sexp::List(self.regions.iter().map(Region::to_sexp).collect()),
            s_kw("nodes"), Sexp::List(self.nodes.iter().map(Node::to_sexp).collect()),
            s_kw("edges"), Sexp::List(self.edges.iter().map(Edge::to_sexp).collect()),
            s_kw("bands"), Sexp::List(self.bands.iter().map(Band::to_sexp).collect()),
        ])
    }
}

// Atom not currently used directly in this module — re-suppress unused warning.
#[allow(dead_code)]
fn _silence_atom_import() -> Atom { Atom::Symbol("nil".into()) }

// ── tests — the bridge is real if these compile from text ───────

#[cfg(test)]
mod tests {
    use super::*;
    use tatara_lisp::TataraDomain;
    use crate::{Id, Marker, Role, Stroke};
    use tatara_lisp::reader::read;

    fn one_form(src: &str) -> Sexp {
        let forms = read(src).expect("reader parses src");
        forms.into_iter().next().expect("at least one form")
    }

    #[test]
    fn hero_round_trip() {
        let form = one_form(r#"(defhero :title "Multi-Region" :subtitle "primary + replica")"#);
        let h = Hero::compile_from_sexp(&form).unwrap();
        assert_eq!(h.title, "Multi-Region");
        assert_eq!(h.subtitle.as_deref(), Some("primary + replica"));
    }

    #[test]
    fn hero_subtitle_optional() {
        let form = one_form(r#"(defhero :title "T")"#);
        let h = Hero::compile_from_sexp(&form).unwrap();
        assert_eq!(h.title, "T");
        assert!(h.subtitle.is_none());
    }

    #[test]
    fn badge_round_trip() {
        let form = one_form(r#"(defbadge :text "MASTER · R/W" :role :master)"#);
        let b = Badge::compile_from_sexp(&form).unwrap();
        assert_eq!(b.text, "MASTER · R/W");
        assert_eq!(b.role, Role::Master);
    }

    #[test]
    fn role_keyword_dispatch() {
        for (kw, role) in [
            (":master", Role::Master),
            (":read", Role::Read),
            (":accent", Role::Accent),
            (":neutral", Role::Neutral),
            (":danger", Role::Danger),
            (":success", Role::Success),
        ] {
            let form = one_form(&format!(r#"(defbadge :text "x" :role {kw})"#));
            let b = Badge::compile_from_sexp(&form).unwrap();
            assert_eq!(b.role, role, "for keyword {kw}");
        }
    }

    #[test]
    fn role_unknown_keyword_errors_cleanly() {
        let form = one_form(r#"(defbadge :text "x" :role :purple)"#);
        let err = Badge::compile_from_sexp(&form).unwrap_err();
        let msg = format!("{err:?}");
        assert!(msg.contains("purple"), "error mentions the unknown role: {msg}");
    }

    #[test]
    fn region_with_badge_and_rect() {
        let form = one_form(
            r#"(defregion :id "primary"
                          :label "Primary"
                          :sublabel "read + write"
                          :role :master
                          :badge (defbadge :text "MASTER · R/W" :role :master)
                          :at (defrect :x 40 :y 200 :w 420 :h 250))"#,
        );
        let r = Region::compile_from_sexp(&form).unwrap();
        assert_eq!(r.id, Id::new("primary"));
        assert_eq!(r.label, "Primary");
        assert_eq!(r.sublabel.as_deref(), Some("read + write"));
        assert_eq!(r.role, Role::Master);
        assert!(r.badge.is_some());
        assert_eq!(r.badge.as_ref().unwrap().text, "MASTER · R/W");
        assert_eq!(r.at, Rect { x: 40, y: 200, w: 420, h: 250 });
    }

    #[test]
    fn node_with_optional_region() {
        let form = one_form(
            r#"(defnode :id "svc-a"
                        :label "Service A"
                        :role :master
                        :region "primary"
                        :at (defrect :x 60 :y 270 :w 120 :h 34))"#,
        );
        let n = Node::compile_from_sexp(&form).unwrap();
        assert_eq!(n.id, Id::new("svc-a"));
        assert_eq!(n.label, "Service A");
        assert_eq!(n.role, Role::Master);
        assert_eq!(n.region, Some(Id::new("primary")));
        assert_eq!(n.at.w, 120);
    }

    #[test]
    fn edge_with_points_stroke_marker() {
        let form = one_form(
            r#"(defedge :from (defpoint :x 0 :y 0)
                        :to (defpoint :x 100 :y 50)
                        :stroke :dashed
                        :marker :arrow
                        :role :accent
                        :label "replicate")"#,
        );
        let e = Edge::compile_from_sexp(&form).unwrap();
        assert_eq!(e.from, Point { x: 0, y: 0 });
        assert_eq!(e.to, Point { x: 100, y: 50 });
        assert_eq!(e.stroke, Stroke::Dashed);
        assert_eq!(e.marker, Marker::Arrow);
        assert_eq!(e.role, Role::Accent);
        assert_eq!(e.label.as_deref(), Some("replicate"));
    }

    #[test]
    fn band_round_trip() {
        let form = one_form(
            r#"(defband :label "Global DNS"
                        :sublabel "nearest healthy region"
                        :role :master
                        :at (defrect :x 300 :y 96 :w 360 :h 46))"#,
        );
        let b = Band::compile_from_sexp(&form).unwrap();
        assert_eq!(b.label, "Global DNS");
        assert_eq!(b.role, Role::Master);
        assert_eq!(b.at.h, 46);
    }

    #[test]
    fn diagram_full_compound_compiles() {
        // The workhorse round-trip: a complete `(defdiagram ...)` form
        // produces a fully-typed `Diagram` value via TataraDomain recursion.
        let src = r#"
            (defdiagram
              :canvas  (defcanvas :width 960 :height 560)
              :hero    (defhero   :title "Multi-Region" :subtitle "primary + replica")
              :regions ((defregion :id "primary"
                                   :label "Primary"
                                   :role :master
                                   :at (defrect :x 40 :y 200 :w 420 :h 250))
                        (defregion :id "replica"
                                   :label "Replica"
                                   :role :read
                                   :at (defrect :x 500 :y 200 :w 420 :h 250)))
              :nodes   ((defnode :id "svc-a"
                                 :label "Service A"
                                 :role :master
                                 :region "primary"
                                 :at (defrect :x 60 :y 270 :w 120 :h 34)))
              :edges   ((defedge :from (defpoint :x 460 :y 325)
                                 :to (defpoint :x 500 :y 325)
                                 :stroke :dashed
                                 :marker :arrow
                                 :role :accent
                                 :label "replicate"))
              :bands   ((defband :label "Global DNS"
                                 :sublabel "nearest healthy region"
                                 :role :master
                                 :at (defrect :x 300 :y 96 :w 360 :h 46))))
        "#;
        let form = one_form(src);
        let d = Diagram::compile_from_sexp(&form).unwrap();
        assert_eq!(d.canvas, Canvas { width: 960, height: 560 });
        assert_eq!(d.hero.title, "Multi-Region");
        assert_eq!(d.regions.len(), 2);
        assert_eq!(d.regions[0].label, "Primary");
        assert_eq!(d.regions[1].label, "Replica");
        assert_eq!(d.nodes.len(), 1);
        assert_eq!(d.nodes[0].id, Id::new("svc-a"));
        assert_eq!(d.edges.len(), 1);
        assert_eq!(d.edges[0].label.as_deref(), Some("replicate"));
        assert_eq!(d.bands.len(), 1);
        assert_eq!(d.bands[0].label, "Global DNS");
    }

    #[test]
    fn diagram_with_empty_lists_uses_defaults() {
        // :regions/:nodes/:edges/:bands all missing → empty vecs.
        let src = r#"
            (defdiagram
              :canvas (defcanvas :width 100 :height 50)
              :hero   (defhero :title "Empty"))
        "#;
        let form = one_form(src);
        let d = Diagram::compile_from_sexp(&form).unwrap();
        assert!(d.regions.is_empty());
        assert!(d.nodes.is_empty());
        assert!(d.edges.is_empty());
        assert!(d.bands.is_empty());
    }

    // ── round-trip: typed → Sexp → typed (Phase B feeder) ──

    #[test]
    fn diagram_to_sexp_round_trip() {
        // Build a typed Diagram in Rust, serialize to Sexp via to_sexp,
        // then compile back via TataraDomain — must be byte-equal.
        let canvas = Canvas { width: 960, height: 560 };
        let hero = Hero {
            title: "Multi-Region".into(),
            subtitle: Some("primary + replica".into()),
        };
        let mut d = Diagram::new(canvas, hero);
        d = d.with_region(Region {
            id: Id::new("primary"),
            label: "Primary".into(),
            sublabel: Some("read + write".into()),
            role: Role::Master,
            badge: Some(Badge {
                text: "MASTER · R/W".into(),
                role: Role::Master,
            }),
            at: Rect { x: 40, y: 200, w: 420, h: 250 },
        });
        d = d.with_node(Node {
            id: Id::new("svc-a"),
            label: "Service A".into(),
            role: Role::Master,
            region: Some(Id::new("primary")),
            at: Rect { x: 60, y: 270, w: 120, h: 34 },
        });
        d = d.with_edge(Edge {
            from: Point { x: 460, y: 325 },
            to: Point { x: 500, y: 325 },
            stroke: Stroke::Dashed,
            marker: Marker::Arrow,
            role: Role::Accent,
            label: Some("replicate".into()),
        });
        d = d.with_band(Band {
            label: "Global DNS".into(),
            sublabel: None,
            role: Role::Master,
            at: Rect { x: 300, y: 96, w: 360, h: 46 },
        });
        let sexp = d.to_sexp();
        let round = Diagram::compile_from_sexp(&sexp).unwrap();
        assert_eq!(d, round, "Diagram → to_sexp → compile_from_sexp must round-trip");
    }

    #[test]
    fn role_stroke_marker_keyword_round_trip() {
        // Every Role/Stroke/Marker variant round-trips through its keyword form.
        for role in [Role::Master, Role::Read, Role::Accent, Role::Neutral, Role::Danger, Role::Success] {
            let badge = Badge { text: "x".into(), role };
            let s = badge.to_sexp();
            let round = Badge::compile_from_sexp(&s).unwrap();
            assert_eq!(badge.role, round.role);
        }
    }

    // ── Phase F: derive-macro recursion via #[tatara(domain)] ──

    #[test]
    fn tatara_domain_attribute_plain_nested() {
        // The new `#[tatara(domain)]` field attribute generates
        // `<T as TataraDomain>::compile_from_sexp(sexp)` — no serde JSON
        // intermediate. Proves the macro extension end-to-end.
        use tatara_lisp::DeriveTataraDomain;
        #[derive(Debug, Clone, PartialEq, Eq, DeriveTataraDomain)]
        #[tatara(keyword = "deftest1")]
        #[allow(dead_code)]
        struct Test1 {
            title: String,
            #[tatara(domain)]
            bounds: Rect,
        }
        let form = one_form(r#"(deftest1 :title "T" :bounds (defrect :x 10 :y 20 :w 100 :h 50))"#);
        let t = Test1::compile_from_sexp(&form).unwrap();
        assert_eq!(t.title, "T");
        assert_eq!(t.bounds, Rect { x: 10, y: 20, w: 100, h: 50 });
    }

    #[test]
    fn tatara_domain_attribute_option_some_and_none() {
        use tatara_lisp::DeriveTataraDomain;
        #[derive(Debug, Clone, PartialEq, Eq, DeriveTataraDomain)]
        #[tatara(keyword = "deftest2")]
        #[allow(dead_code)]
        struct Test2 {
            title: String,
            #[tatara(domain)]
            anchor: Option<Point>,
        }
        // With :anchor present
        let f1 = one_form(r#"(deftest2 :title "A" :anchor (defpoint :x 5 :y 7))"#);
        let t1 = Test2::compile_from_sexp(&f1).unwrap();
        assert_eq!(t1.anchor, Some(Point { x: 5, y: 7 }));
        // With :anchor absent
        let f2 = one_form(r#"(deftest2 :title "B")"#);
        let t2 = Test2::compile_from_sexp(&f2).unwrap();
        assert!(t2.anchor.is_none());
    }

    #[test]
    fn tatara_domain_attribute_vec_recursion() {
        use tatara_lisp::DeriveTataraDomain;
        #[derive(Debug, Clone, PartialEq, Eq, DeriveTataraDomain)]
        #[tatara(keyword = "deftest3")]
        #[allow(dead_code)]
        struct Test3 {
            title: String,
            #[tatara(domain)]
            rects: Vec<Rect>,
        }
        let src = r#"
            (deftest3
              :title "M"
              :rects ((defrect :x 1 :y 1 :w 10 :h 10)
                      (defrect :x 2 :y 2 :w 20 :h 20)
                      (defrect :x 3 :y 3 :w 30 :h 30)))
        "#;
        let form = one_form(src);
        let t = Test3::compile_from_sexp(&form).unwrap();
        assert_eq!(t.rects.len(), 3);
        assert_eq!(t.rects[0], Rect { x: 1, y: 1, w: 10, h: 10 });
        assert_eq!(t.rects[2], Rect { x: 3, y: 3, w: 30, h: 30 });
    }

    #[test]
    fn optional_fields_omitted_when_none() {
        // Hero without subtitle, Node without region, Edge without label, Band without sublabel:
        // each should serialize without the missing key, then deserialize with None.
        let h = Hero { title: "T".into(), subtitle: None };
        let s = h.to_sexp();
        // Round-trip
        assert_eq!(h, Hero::compile_from_sexp(&s).unwrap());
        // Form text shouldn't contain `:subtitle`
        let txt = format!("{s:?}");
        assert!(!txt.contains("subtitle"), "absent optional should not be serialized: {txt}");
    }
}
