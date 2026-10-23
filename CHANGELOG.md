# Changelog

Notable changes to **zukai**. Format follows [Keep a Changelog](https://keepachangelog.com/); the project follows [SemVer](https://semver.org/).

## [Unreleased]

Enrichment in progress — greatly expanding the vocabulary for diagram AND documentation expression on Confluence, plus making **tatara-lisp the consistent templating language that drives the entire solution**.

### Changed

**blue replaces the hand-built tatara-lisp interpreters.** `zukai_confluence::lisp` and `zukai_kata::lisp` are gone; `zukai_confluence::host::Confluence` and `zukai_kata::host::Kata` are `blue-lang-host` `#[host]` blocks, and page bodies and layout patterns are written in blue. Names: `panel/info` → `cf_panel_info` (each primitive takes the `cf_` prefix, `-` becomes `_`), `(multi-region :title …)` → `kata_multi_region({title: …})` (each pattern takes the `kata_` prefix and one map). The 14 `*-spec` helpers are gone: a spec is a map. Pattern results are maps with a `kind` key instead of `Value::Sexp`; `zukai_kata::record::Typed<T>` reads one back as a typed value. `zukai-confluence` no longer depends on `tatara-lisp-eval`.

### Added

**Phase A — tatara-lisp bridge for the diagram authoring surface.** Every `zukai-model` primitive now compiles from a `.tlisp` form via `TataraDomain`:

- `zukai-model::lisp` — manual `impl TataraDomain` for `Hero` (`defhero`), `Badge` (`defbadge`), `Region` (`defregion`), `Node` (`defnode`), `Edge` (`defedge`), `Band` (`defband`), `Diagram` (`defdiagram`). Plus helper parsers for the non-Domain types: `Rect` (`defrect`), `Point` (`defpoint`), and keyword-form enums for `Role` (`:master :read :accent :neutral :danger :success`), `Stroke` (`:solid :dashed`), `Marker` (`:none :arrow`). Complements the existing `#[derive(TataraDomain)]` on `Canvas`. **11 round-trip tests** covering each primitive + a full nested `(defdiagram …)` compound.
- `zukai/specs/diagrams/` — checked-in `.tlisp` acceptance corpus. First entry: `example_2region.tlisp` — 3.4 KB hand-authored multi-region spec with 2 regions, 6 service nodes, 3 edges, 1 band.
- `crates/zukai-kata/examples/tlisp_render.rs` — Phase A end-to-end demo. Reads the `.tlisp` source from disk, compiles via `Diagram::compile_from_sexp`, renders SVG/PNG/Mermaid via `zukai-render`.

**`.tlisp` is now the dialect** every future authoring artifact flows through. Future phases (B–F documented in `docs/VOCABULARY.md`) translate kata patterns into tatara-lisp macros, `zukai-confluence` primitives into special forms, and page templates into composition macros — all on top of this bridge.

**Phase C — tatara-lisp bridge for the documentation half.** Every `zukai-confluence` primitive is now a native function on a `tatara_lisp_eval::Interpreter`:

- `zukai_confluence::lisp::register(interp)` — generic over host type, plugs in 17 native fns: 5 panels (`panel/info` / `panel/warning` / `panel/note` / `panel/tip` / `panel/success`), 6 structure (`expand`, `code`, `toc`, `tabs`, `tasklist`, `status`), 2 layout (`two-column`, `three-column`), 4 inline (`paragraph`, `hr`, `link`, `page-link`, `mention`), 1 table (`table`).
- Composition is free: every primitive returns `Value::Str` so `(two-column (expand "…" (panel/tip "…")) (tabs (list (list "Code" (code "rust" "…")) (list "Steps" (tasklist (list (list #t "…") (list #f "…")))))))` — a 5-level-deep nested form — evaluates to one storage-XML string.
- **13 round-trip tests** covering every primitive + a deep-nested composition + Lisp-syntax gotchas (`#t`/`#f` bool literals, explicit `(list …)` for list values).
- Both `tatara-lisp` and `tatara-lisp-eval` are now real deps of `zukai-confluence` (path: `../../../tatara-lisp/{tatara-lisp,tatara-lisp-eval}` — the standalone tree). `zukai-model` and `zukai-kata` migrated to the same standalone tree so the dialect has one source of truth.

**Phase B (v0 + v1 + v2) — All 17 kata patterns wrapped as tatara-lisp native functions.** `zukai_kata::lisp::register(interp)` plugs the whole kata library onto the `Interpreter`. The workhorse `(multi-region …)` accepts full parameterization (via `(region-spec …)` / `(service-spec …)` / `(gateway-spec …)`) or a short default form (`:n-replicas` + `:gateway-label`). The other 16 patterns (`hero`, `closing`, `pillars`, `service-tier`, `architecture-layers`, `customer-gateway`, `zero-knowledge`, `ha-failover`, `read-replica-pair`, `api-fanout`, `decision-branch`, `tree-hierarchy`, `routing-detail`, `data-tier`, `event-timeline`, `interaction-lane`) each have a native fn that returns either `Value::Sexp` (single primitive) or `Value::List` of `Value::Sexp` (fragments — caller compiles each item via TataraDomain to assemble a Diagram).

14 tagged-list spec helpers: `service-spec`, `region-spec`, `gateway-spec`, `pillar-spec`, `layer-spec`, `tier-spec`, `stage-spec`, `event-spec`, `branch-spec`, `tree-node-spec`, `message-spec`, `backend-spec`, `ha-node-spec`, `read-replica-spec`. 19 round-trip tests covering every pattern.

**`zukai-model::lisp` — `to_sexp()` round-trip.** Every primitive (Diagram, Hero, Badge, Region, Node, Edge, Band, Canvas, Rect, Point, plus Role/Stroke/Marker as keywords) gained a `to_sexp` method so kata patterns can produce Sexp values that compile back to typed primitives. 3 new round-trip tests including the full Diagram. Discovered `Role::Warning` and `Role::Info` already exist in the model and threaded them through the role parser + serializer.

**Phase D — Page composition.** The page-level layer.

- `compose` native fn in `zukai-confluence::lisp` — variadic string concatenation of Confluence primitive outputs. Type-safe: non-string args are an error, not silent coercion.
- `Page` TataraDomain in `zukai-confluence` — `(defpage :title "…" :body "…")` compiles to a typed `zukai_confluence::Page` value (title + body string). The unit of publish.
- **Cross-crate end-to-end integration test** in `zukai-kata::lisp::tests::end_to_end_page_with_kata_diagram_embed` — one Interpreter session: kata native fn builds the diagram → confluence primitives compose the body → defpage wraps as Page. Phases B + C + D in concert.

### Tests

**Phase F — derive-macro recursion via `#[tatara(domain)]`.** Extended the `tatara-lisp-derive` proc macro:

- New `#[tatara(domain)]` field attribute. On any field annotated, the macro emits `<T as TataraDomain>::compile_from_sexp(sexp)` directly — no serde JSON intermediate. Handles `T`, `Option<T>`, and `Vec<T>` wrapper cases via type-shape inspection.
- **First wave of derive-collapse:** `Hero` / `Rect` / `Point` switched from manual `impl TataraDomain` blocks (in `zukai-model::lisp`) to plain `#[derive(TataraDomain)] #[tatara(keyword = "…")]` on the type itself (in `zukai-model::lib`). Manual `rect_from_form` / `point_from_form` helpers (and their `extract_i32` / `extract_u32` private extractor helpers) removed. Callers in `Region`/`Node`/`Edge`/`Band` manual impls updated to call `Rect::compile_from_sexp` / `Point::compile_from_sexp` directly.
- **3 new tests** in `zukai_model::lisp::tests` prove the macro extension end-to-end with local test structs: `tatara_domain_attribute_plain_nested` (T), `tatara_domain_attribute_option_some_and_none` (Option<T>, both present and absent), `tatara_domain_attribute_vec_recursion` (Vec<T>).
- **Open Phase F follow-up:** add `#[tatara(role_keyword)]` (for keyword-form enums like `Role`/`Stroke`/`Marker`) and `#[tatara(via_string)]` (for newtype wrappers like `Id`). Once landed, every remaining manual primitive impl (`Badge`/`Region`/`Node`/`Edge`/`Band`/`Diagram`) collapses to a one-line derive.

### Tests

- **206 tests green** across 6 crates (up from 153 at the zukai-confluence landing; +11 Phase A, +13 Phase C, +19 Phase B v0/v1/v2, +3 to_sexp round-trip, +2 Page TataraDomain, +2 compose, +3 Phase F derive-recursion).



**`zukai-confluence` (new crate, 6th workspace member)** — Confluence document primitives. The **documentation half** of zukai, sibling to `zukai-render` (the diagram half). Pure Rust functions emitting Confluence storage XML, no theming, no diagram rendering. **33 tests** covering structural correctness + XML escaping + determinism. Modules:
- `panel` — `info` / `warning` / `note` / `tip` / `success` (5 variants).
- `structure` — `expand` (collapsible), `code` (CDATA-wrapped), `toc`, `tabs`, `tasklist`, `status` (inline chip).
- `layout` — `two_column`, `three_column`.
- `inline` — `paragraph`, `hr`, `link`, `page_link`, `mention`.
- `table` — header + body row builder.
- `xml` — single-source XML attribute / text escapers.

**Standing demo:** `cargo run --example page_demo -p zukai-confluence` — composes a complete Confluence page body using every primitive, writes to `/tmp/zukai-conf-demo-body.xml`.

**Live proof page (4th total):** `4112318475` — full doc-primitive demo (panels + tabs + code + expand + tasklist + status chips + two-column layout + table + embedded diagram). PASS: 3 img / 5 rows / 0 errors.

### Tests

- **153 tests green** across 6 crates (up from 120 at v0.1).

## [v0.1.0] — 2026-05-28

Milestone: complete typed-diagram pipeline for Confluence. Both **Case A** (new diagrams) and **Case B** (re-theme existing Mermaid) demonstrated end-to-end on live pages. See [`docs/MILESTONE-v0.1.md`](./docs/MILESTONE-v0.1.md).

### Added

**`zukai-kata` — 17 named patterns (the kata library, the canonical authoring entry point):**

`hero`, `service_tier`, `customer_gateway`, `pillars`, `closing`, `api_fanout`, `ha_failover`, `zero_knowledge`, `routing_detail`, `data_tier`, `tree_hierarchy`, `read_replica_pair`, `event_timeline`, `architecture_layers`, `decision_branch`, `interaction_lane`.

Plus the workhorse `multi_region` (extended to v1, see below).

**Case B helper:** `zukai_kata::from_mermaid(src) -> Result<Diagram, FromMermaidError>` — parses simple `flowchart TD/BT/LR/RL` blocks (node decls `A[Label]`/`A(Label)`, edges one-per-line `A --> B` or `A -- label --> B`, implicit nodes id-as-label, comments + unrecognized lines silently skipped) into a renderable `Diagram` that flows through the same `Theme` seam as kata-authored diagrams.

**Confluence delivery helper:** `zukai_render::confluence::embed(filename, width) -> String` — emits the Confluence storage-format `<ac:image ac:width="…"><ri:attachment …/></ac:image>` snippet; XML-attribute-escapes the filename.

**`zukai-render::RenderAttestation`** — three-pillar BLAKE3 attestation (artifact⊕control⊕intent) for the §IV.3 VERIFY phase.

**`zukai-render::IshouTheme`** — adapts `ishou::TokenSet` to the `Theme` seam (Nord palette wired as the public generic default).

**`zukai-lower`** (new crate) — `DiagramSpec → iac_forge::sexpr::SExpr` lowering, round-trip + BLAKE3 content-hash stable.

**Standing acceptance demos:**
- `cargo run --example multi_region -p zukai-kata` — single-pattern.
- `cargo run --example composition_demo -p zukai-kata` — 3-pattern composition.
- `cargo run --example case_b_demo -p zukai-kata` — Case B lift Mermaid → render.

### Changed

- **`multi_region` v0 → v1.** Relaxed the v0 `replicas.len() ≤ 1` constraint. Now supports **N** replica regions with **star replication** topology + region widths scaled to fit the canvas. **Byte-identical to v0 for 1- and 2-region inputs** (no regression). The workhorse pattern is now general.
- **Canonical authoring path.** `zukai-kata` (named-pattern library) replaces hand-composing primitives. The primitive-composition example in `zukai-render` was removed in favor of the kata-composition example in `zukai-kata`.

### Tests

- **120 tests green** across 5 crates. (Up from a green spine of ~6 tests at v0.0.)

### Live demonstrations

Three live Confluence pages in the personal-space scratch area (URLs in `docs/MILESTONE-v0.1.md`):

| Page id | Proves |
|---|---|
| `4112547842` | single-pattern pipeline (multi_region N=2 → PNG → upload → embed → 1 img, 0 errors) |
| `4112351245` | multi-pattern composition (3 distinct patterns on one page → 3 imgs, 0 errors) |
| `4112678914` | Case B re-theming (existing Mermaid → `from_mermaid` → PNG embed alongside source) |

### Documentation

- `docs/MILESTONE-v0.1.md` — milestone capture.
- `docs/VOCABULARY.md` — pattern glossary (kata 型) with row-per-pattern (args, returns, when-to-use).
- `CLAUDE.md` — refreshed doctrine for v0.1 (kata-first authoring rule).
- `README.md` — quick-start + status.
- `pleme-io/blackmatter-pleme/skills/diagram-authoring/SKILL.md` — updated trigger table for all 17 patterns + the publish workflow with `confluence::embed`.
- `pleme-io/theory/VOCABULARY.md` — zukai row refreshed.

## [v0.0] — pre-history

- Initial spine: `zukai-model` (primitives) + `zukai-render` (Svg renderer + `Theme` trait).
- Public/private boundary proofs (`*_keeps_overlay_brand_private` tests across each renderer).
- Design note + reuse ledger + seven-question gate (`docs/DESIGN.md`).
