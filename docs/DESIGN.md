# zukai — Design Note

> **★★★ CSE / Knowable Construction.** This repo operates under **Constructive Substrate Engineering** — canonical spec at `pleme-io/theory/CONSTRUCTIVE-SUBSTRATE-ENGINEERING.md`. The Compounding Directive (solve once, load-bearing fixes only, idiom-first, models stay current, direction beats velocity) is in `pleme-io/CLAUDE.md` ★★★. Read both before non-trivial changes.

**zukai (図解, "diagram")** is a new citizen of the typescape. It owns one concern: **the typed declaration of a diagram, and its projection into any viewing surface.** Every visual artifact zukai produces — SVG, PNG, Mermaid, Graphviz, draw.io, Excalidraw, a Confluence `ac:image` — is a *projection of a typed `DiagramSpec` through a morphism*. The spec is the source we mine; the artifact is how it looks from one angle. Every solution flows from the model; rewriting an artifact without going through the spec is drift (`theory/THEORY.md` §III.1).

This note is the design artifact the six-line-contract rule asks for (`THEORY.md` §II.1) — it records the seven-question gate and the reuse ledger before any code.

## 1. Why this shape is forced, not chosen

zukai introduces **no new theory**. Every structural decision is dictated by an existing pillar:

| Decision | Directive |
|---|---|
| Rust, not TypeScript | Pillar 1 — Rust; server-side TS forbidden (TS edge-only, `lilitu/`) |
| `DiagramSpec` via `#[derive(TataraDomain)]` `(defdiagram …)` | §II.1 six-line contract |
| Diagrams are never hand-authored | §III.1 — the artifact is never the source; hand-editing is drift |
| SVG/Mermaid/Graphviz/draw.io/Excalidraw = AST domains; renderers = `Synthesizer` backends | §III.2 — a new backend inherits all proofs |
| "Arbitrary build steps before delivery" = morphism pipeline over the SExpr IR | §VIII.1 — four Lisps → one IR (`iac_forge::sexpr::SExpr`) |
| Theming reuses `ishou` (token→targets); no new theme tool | Pillar 6 + idiom-first |
| Brand-specific overlay = a private `bordado` over the generic base | `VOCABULARY.md` `bordado` |
| Japanese name | §II.3 — discipline layer → Japanese |
| Scaffolded by `repo-forge`, built by `substrate`, `nix run .#app` | Pillar 9 |
| Each render attested (artifact⊕control⊕intent BLAKE3) + `.typescape.yaml` | §V.3, Pillar 10 |

## 2. The pipeline (the one structure)

```
(defdiagram …)            DiagramSpec                  SExpr IR              render morphisms
 YAML · tatara-lisp · ───→ #[derive(TataraDomain)] ───→ iac_forge::sexpr ───→ {svg, png, mermaid,
 Nix · Rust builder        BLAKE3 identity, registry    (canonical, BLAKE3)    graphviz, drawio,
                                                                                excalidraw, confluence}
        ThemeCase = ishou TokenSet  ⊕  private `bordado` overlay  ────────────┘
```

- **Authoring (the Four Lisps).** Because `DiagramSpec` derives `TataraDomain`, a diagram is authorable as a tatara-lisp form, a YAML doc, a Nix expression, or a Rust value — all reduce to the same `SExpr` (§II.2, §VIII.1). No bespoke parser.
- **The model.** Pure typed primitives reverse-engineered from a hand-tuned multi-region acceptance corpus: `Diagram`, `Region`, `Node`, `Edge`, `Band`, `Badge`, `Legend`, `Role`, `Anchor`, `Stroke`, `Marker`, `Grid`. Colors are never literals — every visual attribute resolves through a `ThemeCase` role.
- **The IR + build steps.** The spec lowers to `iac_forge::sexpr::SExpr`; the build steps are morphisms/transforms over that IR (`iac-forge` `morphism`/`pipeline`/`transform`) — layout, theme-application, optimization, target-wrapping. Stage-to-stage `sexpr` diff gives semantic change tracking.
- **The render morphisms.** Each target is an AST domain; each renderer is a `Synthesizer`/`Backend`. A renderer is a pure total `(LoweredDiagram, ThemeCase) -> Vec<u8>`; SVG/Mermaid are *leaf* outputs, so `format!`/a small writer is idiomatic (the no-`format!()` rule binds *intermediate* code, not leaf artifacts — `ishou-render/src/svg.rs` precedent).
- **Theming.** `ishou` already renders a `TokenSet` to many formats; zukai consumes the `TokenSet` for diagram styling. The generic base is public (borealis/ishou); brand-specific theming arrives only via a private `bordado` overlay (vendored from a private brand source repo), applied at render time. zukai itself never contains a brand hex.

## 3. The seven questions (§IV.6 gate)
1. **Declaration** — `DiagramSpec` (TataraDomain) + `ThemeCase` (ishou TokenSet ± bordado).
2. **Invariants** — render is pure, total, byte-deterministic; output validates against the target grammar; no hardcoded colors; identical `(spec, theme)` ⇒ identical bytes.
3. **Compliance baseline** — the public/private separation is the binding lattice constraint: a private overlay's brand hexes or any tenant-naming content may never be rendered from, or committed to, a public crate. A render carries a visibility class; a public-target render refuses private content.
4. **Render** — `Synthesizer` morphisms, one per target AST domain.
5. **Deploy** — the Confluence delivery morphism (model → storage `ac:image`) + the publish step (Atlassian REST/MCP: upload attachment, embed at width).
6. **Drift** — regenerate; a BLAKE3 mismatch is drift. A hand-edited Confluence SVG is drift.
7. **Remediate** — re-render from the spec. The artifact is never patched.

## 4. Public / private boundary
- **Public** (a generic OSS diagram engine; reveals no private association): the `zukai` crates, the model, the render morphisms, the IR/pipeline reuse, the borealis/ishou generic base, the build+release machinery, and the generic `diagram-authoring` skill.
- **Private** (brand + content): a private `bordado` overlay (brand-specific theme tokens) and diagram *content* (specs that name specific tenants/services/IPs). Both live in private consumers; the engine sees them only at render time.

## 5. The eight-phase loop = the authoring + Confluence-perfection loop (§IV.3)
```
DECLARE    (defdiagram …) / YAML spec
SIMULATE   zukai render --target png  (resvg) → qlmanage quicklook            ← fast inner loop
PROVE      determinism test + target-grammar validation
REMEDIATE  auto-fix (theme-role coercion, anchor resolution)
RENDER     Synthesizer morphism → artifact
DEPLOY     Confluence delivery morphism + REST publish (scratch page)
VERIFY     curupira (Chrome MCP) renders the live Confluence page, screenshots, measures  ← perfect the view
RECONVERGE diff vs intent → edit spec → repeat
```
Confluence is one delivery target among several; "perfecting the viewing experience in Confluence" is the VERIFY phase via the browser MCP, not a special case in the engine.

## 6. Attestation & provenance (§V.3, Pillar 10, §VI.1)
Every render emits a three-pillar BLAKE3 attestation (artifact ⊕ control ⊕ intent) into tameshi's chain, plus a `.typescape.yaml` recording the contributing dimensions. Regeneration is byte-identical; a diff across regenerations is drift to reconcile. The determinism test is the build gate.

## 7. Crate layout (Pillar 9; `ishou` is the workspace template)
```
zukai/
  crates/
    zukai-model     DiagramSpec + primitives; #[derive(TataraDomain)]; pure data
    zukai-lower     DiagramSpec → SExpr IR; deterministic layout morphisms (coordinates)
    zukai-render    Synthesizer backends: svg, png(resvg), mermaid, graphviz, drawio, excalidraw, confluence
    zukai-cli       clap: `zukai render --target … --theme … --out …`, `render-all`, `hash`, `targets`
  configs/          generic reference ThemeCase + diagram examples (no real-org naming)
  docs/DESIGN.md    this note
  flake.nix         substrate rust-workspace-release-flake.nix
  CLAUDE.md  AGENTS.md   + CSE header
```
Depends on: `tatara-lisp` (authoring), `iac-forge` (sexpr/morphism/pipeline), `ishou` (TokenSet), `arch-synthesizer` (typescape registration), `tameshi` (attestation), `resvg`/`usvg` (PNG — new external dep), `substrate` (build).

## 8. v0 scope
Model + lower/layout + **SVG + PNG** render + **Confluence** delivery, themed generic, with a determinism test. **Acceptance corpus = the hand-tuned multi-region architecture diagrams** (kept private; re-expressed as `DiagramSpec` for verification). Mermaid is the second target. Graphviz/draw.io/Excalidraw/Figma/Tailwind-theme follow as "add a target" once the spine is proven.

## 9. Add-a-target recipe (the repeatable situation)
1. Register the new AST domain in the typescape (`arch-synthesizer`). 2. `zukai-render/src/<target>.rs`: `pub fn render(&LoweredDiagram, &ThemeCase) -> String`. 3. `Target` enum variant + match arms. 4. `mkRenderApp` Nix app. 5. determinism test. 6. VOCABULARY row.

## 10. Solidification (the context lock-in)
- `theory/VOCABULARY.md` rows: `zukai`, the new diagram AST domains, `ThemeCase`.
- `zukai/CLAUDE.md` (+ CSE header) + this `docs/DESIGN.md`.
- `blackmatter-pleme/skills/diagram-authoring/SKILL.md` — trigger: *"author/iterate a diagram or visualization for Confluence or docs."* "Tools used": `zukai render …`. Anti-patterns: never hand-author SVG; never hardcode hexes; private overlay content stays private; `format!` only for leaf output. Registered in `skill-map.d`, `skill-lint`-clean, deployed via home-manager rebuild.
- memory lock.

## 11. Reuse ledger (near-zero new substrate)
| Concern | Reused |
|---|---|
| typed authoring (Lisp/YAML/Nix/Rust) | tatara-lisp |
| canonical IR + morphisms + pipeline + diff | iac-forge |
| typescape registration + Synthesizer trait | arch-synthesizer |
| theme tokens → formats | ishou |
| generic theme base | borealis / ishou |
| private brand overlay | `bordado` (vendored from a private brand source repo) |
| attestation | tameshi |
| build + cross-target + release | substrate, gen |
| scaffold | repo-forge |
| Confluence publish | Atlassian MCP / REST |
| viewing-experience verification | curupira (Chrome MCP) |
| skill distribution | blackmatter-pleme, skill-map, skill-lint |

**New code:** `zukai-{model,lower,render,cli}` + the `resvg` dependency. That is the entire net-new surface.

## 12. Non-goals
- Not a general drawing app; diagrams are *declared*, not dragged.
- Not a layout-AI; v0 layout is deterministic explicit/grid placement — beauty stays hand-composed from shared primitives + a shared palette (consistency from the kit, craft from the author).
- Not server-side TypeScript anything (Pillar 1).
