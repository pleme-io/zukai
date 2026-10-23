# zukai — AI agent instructions

> **★★★ CSE / Knowable Construction.** This repo operates under **Constructive Substrate Engineering** — canonical spec at `pleme-io/theory/CONSTRUCTIVE-SUBSTRATE-ENGINEERING.md`. The Compounding Directive (solve once, load-bearing fixes only, idiom-first, models stay current, direction beats velocity) is in `pleme-io/CLAUDE.md` ★★★. Read both before non-trivial changes.

**zukai (図解, "diagram")** — typed diagram domain. **v0.1 shipped 2026-05-28:** 17 named kata patterns + `multi_region` v1 (N regions, star replication) + `from_mermaid` (Case B helper) + Confluence delivery helper + three live proof pages. **120 tests green** across 5 crates.

- **Canonical design + reuse ledger + seven-question gate:** [`docs/DESIGN.md`](./docs/DESIGN.md). Read it first.
- **Pattern glossary (kata 型):** [`docs/VOCABULARY.md`](./docs/VOCABULARY.md).
- **Milestone capture:** [`docs/MILESTONE-v0.1.md`](./docs/MILESTONE-v0.1.md).
- **Release notes:** [`CHANGELOG.md`](./CHANGELOG.md).
- **Quick start:** [`README.md`](./README.md).

## Doctrine (load-bearing rules)

- **Spec is the source.** Mine the model, flow every solution from it. An artifact is never the source — rewriting it without going through the spec is drift (§III.1).
- **★ Always reach for a kata pattern first.** `zukai-kata` is the canonical authoring entry point — pick by intent from `docs/VOCABULARY.md`. Hand-composing primitives bypasses the duplication budget; add a new pattern instead. The `diagram-authoring` skill maps user intent ("draw a multi-region architecture") to the right kata pattern.
- **Roles, never hexes.** The model carries `Role`; the renderer asks the `Theme` for a color. Hardcoded hexes in `zukai-*` crates are a smell. Hardcoded *private overlay* brand hexes anywhere in this public repo are a build failure (`*_keeps_overlay_brand_private` tests across each renderer are the proofs, not comments).
- **`format!()` for leaf output only.** SVG / Mermaid / DOT strings are leaf artifacts; `format!()` is idiomatic for them. Any *intermediate* code (e.g. emitting Go/Nix) must go through a typed AST (`iac-forge::goast` rule).
- **Determinism is the gate.** `cargo test` must show byte-equal output across two renders of the same `(spec, theme)`. Drift = red build.
- **PNG via `ac:image ac:width` is the chosen Confluence target.** Not SVG (`ac:width` honored inconsistently), not Mermaid (editable in Confluence — defeats "spec is the source"). Embed via `zukai_render::confluence::embed`.
- **Add-a-target = the §III.2 recipe.** Register the new AST domain in `arch-synthesizer` → new `Renderer` impl in `zukai-render` → new `Target` variant → `mkRenderApp` Nix app → determinism test → `theory/VOCABULARY.md` row.
- **Add-a-pattern = the kata recipe.** New `zukai-kata/src/<name>.rs` (args struct + fn + tests for structural invariants + determinism) → re-export from `lib.rs` → row in `docs/VOCABULARY.md` → trigger phrases in the `diagram-authoring` skill.
- **Public / private boundary.** zukai is public + generic (consumes `ishou` `TokenSet`). Brand-specific theming arrives only via a private `bordado` overlay (lives in a separate private brand source repo); it never lives here. Tenant-specific *content* (diagram specs naming real services/IPs) also stays in private consumers.

## Workspace

```
crates/zukai-model       typed primitives (DiagramSpec, Canvas with TataraDomain probe)
crates/zukai-render      Theme seam + Renderer trait + Svg/Png/Mermaid
                          + IshouTheme + RenderAttestation + confluence::embed (diagram half)
crates/zukai-cli         clap CLI (`zukai render --target {svg|png|mermaid} --out PATH`)
crates/zukai-lower       DiagramSpec → iac_forge::sexpr::SExpr (canvas + hero)
crates/zukai-kata        ★ 17 named patterns + multi_region v1 + from_mermaid Case B
                          — THE diagram-authoring entry point. Examples live here.
crates/zukai-confluence  ★ Confluence document primitives (doc half) — panels (info/warning/
                          note/tip/success), expand, code, toc, tabs, tasklist, status,
                          layout (two/three-column), inline (link/page_link/mention), table.
                          No theming, no Rendering — pure storage-XML emitters.
docs/                    DESIGN.md, VOCABULARY.md, MILESTONE-v0.1.md
```

**The two halves.** Diagrams flow through `zukai-render` (Theme seam + Renderer trait). Documentation structures flow through `zukai-confluence` (pure storage-XML emitters). A real Confluence page is composed by concatenating both — see `crates/zukai-confluence/examples/page_demo.rs` for the canonical pattern.

## Authoring loop (the seven-phase state machine)

```
1. DECLARE    pick a kata pattern by intent; build the args struct
2. SIMULATE   cargo run --example <name> -p zukai-kata
              → /tmp/zukai-*.{svg,png,mmd}; open the PNG for a quicklook
3. PROVE      cargo test — determinism + *_keeps_overlay_brand_private green
4. RENDER     let png = Png.render(&d, &theme);
5. DEPLOY     Atlassian MCP confluence_upload_attachment(page_id, "<id>.png", png)
              snippet = zukai_render::confluence::embed("<id>.png", 840)
              Atlassian MCP confluence_update_page(body containing snippet)
6. VERIFY     curupira Chrome MCP — navigate, screenshot, eyeball
              (perfect the *viewing experience* in Confluence)
7. RECONVERGE diff vs intent → edit the spec → repeat
```

Skill: `pleme-io/blackmatter-pleme/skills/diagram-authoring/SKILL.md` reinforces this loop and contains the kata-pattern selector + trigger phrases.

## Compounding ledger

**Closed in v0.1:**

- `Theme ← ishou::TokenSet` — ✅ `IshouTheme` (Nord palette wired).
- `DiagramSpec` carries `TataraDomain` — ✅ at primitive level (`Canvas` via derive).
- **★ Phase A tatara-lisp bridge (post-v0.1, unreleased)** — manual `impl TataraDomain` for `Hero / Badge / Region / Node / Edge / Band / Diagram` plus helper parsers for `Rect / Point / Role / Stroke / Marker`. Full `(defdiagram :canvas … :hero … :regions … :nodes … :edges … :bands …)` form compiles to a typed `Diagram` via `tatara_lisp::reader::read` + `Diagram::compile_from_sexp`. Acceptance spec: `specs/diagrams/example_2region.tlisp`. Standing demo: `cargo run --example tlisp_render -p zukai-kata`. **`.tlisp` is now the dialect that drives the diagram authoring surface** — pure text, agent-friendly, machine-checkable. Open follow-up: tatara derive recursion into custom struct types collapses each manual impl to a one-liner.
- **★ blue authoring surface (2026-10-22)** — page bodies and layout patterns are authored as blue (`.b`) through `blue-lang-host` 0.0.84. `zukai_confluence::host::Confluence` (`cf_` prefix, 20 fns: the storage-XML primitives plus `cf_compose`) and `zukai_kata::host::Kata` (`kata_` prefix, all 17 patterns, each taking one map of snake_case args) are `#[host]` blocks; results are plain blue data (`{kind: :defnode, id: …}` maps), read back typed with `zukai_kata::record::Typed<T>`. Replaced the hand-built `tatara_lisp_eval::Interpreter` + `register_fn` bridges (Phases B, C, D). The surface is gated by `Host::SURFACE` tests; worked programs are `crates/*/tests/*.b`. Detail: `docs/VOCABULARY.md`, "blue authoring surface".
- **★ Phase F derive-macro recursion (post-v0.1, unreleased)** — extended `tatara-lisp-derive` with a `#[tatara(domain)]` field attribute that emits `<T as TataraDomain>::compile_from_sexp(sexp)` directly for nested TataraDomain types. Supports T, `Option<T>`, `Vec<T>` wrapper cases. Collapsed `Hero`/`Rect`/`Point` from manual `impl TataraDomain` blocks to plain `#[derive(TataraDomain)]` (with `#[tatara(keyword = "…")]`); removed the obsolete `rect_from_form`/`point_from_form` helpers; updated callers. 3 round-trip tests prove the macro extension end-to-end. **Open follow-up:** add `#[tatara(role_keyword)]` (for `Role`/`Stroke`/`Marker`) and `#[tatara(via_string)]` (for `Id` newtype) — once landed, every remaining manual primitive impl collapses to a one-line derive.
- `zukai-lower` produces `iac_forge::sexpr::SExpr` — ✅ round-trip + BLAKE3 stable.
- Three-pillar BLAKE3 attestation (`tameshi`) — ✅ `RenderAttestation`.
- 17 named kata patterns covering hero/closing, chips, regions, central-and-spokes, two-actor pairs, N-stage chains, timelines, UML-style sequence.
- `multi_region` v0 (≤1 replica) → v1 (N regions, star replication, scaled widths). v0 1- and 2-region inputs preserved byte-identically.
- `from_mermaid` Case B helper (11 tests, 3 error variants, inline-decl extraction).
- Confluence delivery via Atlassian MCP + `confluence::embed` snippet — ✅ proven 3× on live pages.

**Open seams:**

- `Theme` ← private `bordado` overlay impl (separate private repo; this is the boundary seam).
- `arch-synthesizer` typescape registration.
- Tatara extension for nested `Diagram` (extend `extractor_for` to recurse into custom structs).
- Shared `radial_distribution` base + thin wrappers (collapse api_fanout / tree_hierarchy / read_replica_pair / decision_branch — deferred; named patterns serve intent matching today).
- `from_mermaid` v1 — chained edges, alternate arrows, subgraphs.
- `cluster_topology`, `legend` (latter needs new `Legend` primitive).

## Repo, dependencies, release

- **Repo:** `pleme-io/zukai`. Declared in `pangea-architectures` `org.yaml` and repo-forge `repos.lisp`.
- **Every dependency is a crates.io version.** tatara-lisp 0.3.61 (the derive path), blue-lang-host 0.0.84 and tatara-lisp-eval 0.4 (zukai-kata only, for `MapKey`), iac-forge
  0.1.6, ishou-tokens 0.1.18. Sibling crates in this workspace carry both `path` and
  `version`. Do not reintroduce `../../../<sibling>` path deps: they are what kept this
  directory out of git and made CI impossible.
- **Build:** `flake.nix` is substrate's `lib/rust-tool-release-flake.nix` (`packageName =
  "zukai-cli"`, binary `zukai`). `gen build` regenerates `Cargo.gen.lock` +
  `Cargo.build-spec.json` together with `Cargo.lock`; commit all three.
- **Release:** `.github/workflows/auto-release.yml` calls substrate's `rust-auto-release.yml`.
  Every push to `main` bumps, tags and releases.

## Building / running

- `cargo test --workspace` — must be green. `crates/zukai-cli/tests/cli.rs` covers the CLI
  render path (every target, byte-equal across two renders).
- `cargo run --example multi_region -p zukai-kata` — kata-authored multi_region N=2 → `/tmp/zukai-demo-multi-region.{svg,png,mmd}`.
- `cargo run --example composition_demo -p zukai-kata` — 3-pattern composition → `/tmp/zukai-comp-*.png`.
- `cargo run --example case_b_demo -p zukai-kata` — Case B lift Mermaid → render → `/tmp/zukai-case-b.png`.
- `cargo run -p zukai-cli -- render --target {svg|png|mermaid} --out PATH` — CLI.
- `nix build` — the release binary via substrate.

## Live Confluence proof points

| # | Page id | Proves |
|---|---|---|
| 1 | `4112547842` | single-pattern pipeline (`multi_region` N=2 → 1 img, 0 errors) |
| 2 | `4112351245` | multi-pattern composition (3 distinct patterns on one page → 3 imgs, 0 errors) |
| 3 | `4112678914` | Case B re-theming (existing Mermaid → `from_mermaid` → PNG embed alongside source) |

URLs in `docs/MILESTONE-v0.1.md`.
