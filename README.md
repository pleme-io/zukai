# zukai (図解)

> The typed diagram domain. Author once via the **kata library**, render to SVG / PNG / Mermaid, publish to Confluence. Theme via `ishou`. Both new diagrams (**Case A**) and re-themed existing Mermaid (**Case B**).

**Status:** v0.1 shipped (2026-05-28) · 120 tests green across 5 crates · three live Confluence proof pages. See [`docs/MILESTONE-v0.1.md`](./docs/MILESTONE-v0.1.md).

## Quick start

```bash
cd zukai

# 1. Verify the workspace is green.
cargo test                                          # → 120/120 ok

# 2. Render the standing acceptance demo (single pattern).
cargo run --example multi_region -p zukai-kata
# → /tmp/zukai-demo-multi-region.{svg,png,mmd}

# 3. Multi-pattern composition demo (3 patterns on one canvas set).
cargo run --example composition_demo -p zukai-kata
# → /tmp/zukai-comp-{multi-region-n3,pillars,timeline}.png

# 4. Case B — lift an existing Mermaid block, render through kata.
cargo run --example case_b_demo -p zukai-kata
# → /tmp/zukai-case-b.png
```

## Authoring a diagram (Case A)

Pick a kata pattern by intent (full glossary: [`docs/VOCABULARY.md`](./docs/VOCABULARY.md)):

```rust
use zukai_kata::{multi_region, MultiRegionArgs, RegionPatternArgs, GatewayArgs, ServiceArgs};
use zukai_model::Role;
use zukai_render::{GenericTheme, Png, Renderer};

let d = multi_region(MultiRegionArgs {
    title: "Multi-Region",
    subtitle: Some("primary + 2 replicas"),
    primary: RegionPatternArgs { /* … */ },
    replicas: vec![ /* N regions, star replication */ ],
    gateway: Some(GatewayArgs { label: "Client gateway" }),
});

let png: Vec<u8> = Png.render(&d, &GenericTheme);
// → upload via Atlassian MCP + embed via zukai_render::confluence::embed
```

## Case B — re-theme an existing Mermaid block

```rust
use zukai_kata::from_mermaid;
use zukai_render::{GenericTheme, Png, Renderer};

let src = "\
flowchart LR
A[Frontend] -- HTTP --> B[Backend]
B -- query --> C[Database]
";
let d = from_mermaid(src).unwrap();
let png = Png.render(&d, &GenericTheme);
// → same publish pipeline as Case A
```

v0 surface: `flowchart TD|BT|LR|RL`, node decls `A[Label]` / `A(Label)`, edges one-per-line (`A --> B`, `A -- label --> B`), implicit nodes (id-as-label), comments + unrecognized lines silently skipped.

## Publishing to Confluence

```
DiagramSpec (kata)
  → Png.render(&d, &theme) → bytes
  → Atlassian MCP confluence_upload_attachment(page_id, "<id>.png", bytes)
  → zukai_render::confluence::embed("<id>.png", 840)   // → <ac:image …>
  → Atlassian MCP confluence_update_page(body)
  → (optional) curupira Chrome MCP for visual verification
```

**Why PNG via `ac:image ac:width` (not SVG, not Mermaid)?** PNG renders reliably across every Confluence configuration (consistent font fallback, consistent sizing, PDF-export friendly). SVG `ac:width` is honored inconsistently. Mermaid via the macro is editable in Confluence — defeats "spec is the source."

## The 17 kata patterns at a glance

| Category | Patterns |
|---|---|
| Top/bottom bands | `hero`, `closing` |
| Chip arrangements | `pillars`, `service_tier`, `architecture_layers` |
| Region patterns | `multi_region` (N regions, v1), `customer_gateway` |
| Central + spokes | `api_fanout`, `tree_hierarchy`, `read_replica_pair`, `decision_branch` |
| Two-actor pairs | `ha_failover`, `zero_knowledge` |
| N-stage chains | `routing_detail` (horizontal), `data_tier` (vertical) |
| Timelines / sequences | `event_timeline`, `interaction_lane` (UML-style) |
| **Case B helper** | `from_mermaid` |

Each pattern's args / returns / when-to-use is documented in [`docs/VOCABULARY.md`](./docs/VOCABULARY.md).

## Workspace

```
crates/zukai-model    typed primitives (DiagramSpec, Canvas with TataraDomain probe)
crates/zukai-render   Theme seam + Renderer trait + Svg/Png/Mermaid + IshouTheme
                       + RenderAttestation + confluence::embed
crates/zukai-cli      clap CLI (`zukai render --target {svg|png|mermaid} --out PATH`)
crates/zukai-lower    DiagramSpec → iac_forge::sexpr::SExpr
crates/zukai-kata     17 named patterns + multi_region v1 + from_mermaid Case B helper
                       (THE authoring entry point)
docs/                 DESIGN.md, VOCABULARY.md, MILESTONE-v0.1.md
examples/             in crates/zukai-kata/examples/
```

## See also

- [`CLAUDE.md`](./CLAUDE.md) — AI agent instructions / doctrine.
- [`docs/DESIGN.md`](./docs/DESIGN.md) — canonical design + reuse ledger + seven-question gate.
- [`docs/VOCABULARY.md`](./docs/VOCABULARY.md) — pattern glossary (kata 型).
- [`docs/MILESTONE-v0.1.md`](./docs/MILESTONE-v0.1.md) — v0.1 milestone capture.
- [`CHANGELOG.md`](./CHANGELOG.md) — release notes.
- `pleme-io/blackmatter-pleme/skills/diagram-authoring/SKILL.md` — the skill that maps user intent to kata patterns + drives the publish pipeline.
- `pleme-io/ishou/` — the token system zukai themes off of.
- `pleme-io/theory/` — `THEORY.md` §III.1 (artifact ≠ source), §III.2 (AST domains + morphisms), §VI (generation over composition).
