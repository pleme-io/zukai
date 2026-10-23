# zukai v0.1 — milestone

**Date:** 2026-05-28
**Status:** shipped · 120/120 tests green · three live Confluence proof pages.

## What ships at v0.1

zukai is now a complete typed diagram domain for **"repeatedly produce great themed documentation + diagrams in Confluence."** Both authoring cases are demonstrated end-to-end:

- **Case A — new diagrams.** Author via the kata library (17 named patterns); render via Svg / Png / Mermaid; publish via Atlassian MCP + the `confluence::embed` helper.
- **Case B — re-theme existing Mermaid.** Lift an existing Confluence Mermaid block via `from_mermaid`; render through the same `Theme` seam as kata-authored diagrams.

The "well-defined, optimized set of state transitions" is the seven-phase loop: **DECLARE → SIMULATE → PROVE → RENDER → DEPLOY → VERIFY → RECONVERGE** (see `CLAUDE.md`).

## Workspace state

| Crate | Role | Tests |
|---|---|---|
| `zukai-model` | typed primitives + `Canvas` TataraDomain probe | 2 |
| `zukai-render` | `Theme` seam + `Renderer` trait + Svg/Png/Mermaid + `IshouTheme` + `RenderAttestation` + `confluence::embed` | 24 |
| `zukai-cli` | clap CLI (`zukai render --target {svg\|png\|mermaid} --out PATH`) | (smoke) |
| `zukai-lower` | `DiagramSpec → iac_forge::sexpr::SExpr` (canvas + hero) | 4 |
| `zukai-kata` | 17 named patterns + `multi_region` v1 + `from_mermaid` (Case B helper) | 90 |

**Total:** 120/120 green.

## The kata library — 17 named patterns

Pick by intent. Full glossary with args/returns/when-to-use lives in `docs/VOCABULARY.md`.

| # | Pattern | Returns | Shape / use |
|---|---|---|---|
| 1 | `hero` | `Hero` | title banner |
| 2 | `service_tier` | `Vec<Node>` | role-colored chip grid inside a region |
| 3 | `customer_gateway` | `GatewayFragment` | client gateway band + outbound arrow |
| 4 | `multi_region` **v1** | `Diagram` | primary + **N** replicas (star replication, scaled widths) |
| 5 | `pillars` | `Vec<Node>` | N-column foundation / principles |
| 6 | `closing` | `Band` | bottom summary band (mirror of hero) |
| 7 | `api_fanout` | `ApiFanoutFragment` | central API + row of backends + fanout arrows |
| 8 | `ha_failover` | `HaFailoverFragment` | primary + standby + opposing failover/recovery arrows |
| 9 | `zero_knowledge` | `ZeroKnowledgeFragment` | client (keys) + service (ciphertext) + sealed payload arrow |
| 10 | `routing_detail` | `RoutingDetailFragment` | N-stage horizontal request-routing chain |
| 11 | `data_tier` | `DataTierFragment` | N-stage vertical storage hierarchy (hot/warm/cold) |
| 12 | `tree_hierarchy` | `TreeHierarchyFragment` | root + N branches (containment, configurable root role) |
| 13 | `read_replica_pair` | `ReadReplicaPairFragment` | master + N read replicas with dashed Read sync arrows |
| 14 | `event_timeline` | `EventTimelineFragment` | horizontal baseline + N event chips above |
| 15 | `architecture_layers` | `Vec<Node>` | N labeled layers stacked vertically, no arrows (responsibility stack) |
| 16 | `decision_branch` | `DecisionBranchFragment` | decision + N branches with labeled arrows (flowchart) |
| 17 | `interaction_lane` | `InteractionLaneFragment` | 2 actors + 2 lifelines + N horizontal labeled messages (UML sequence) |

Plus **Case B helper:** `from_mermaid(src) -> Result<Diagram, FromMermaidError>` — parse simple `flowchart TD/BT/LR/RL` blocks into a renderable `Diagram`.

## The Confluence delivery pipeline

```
DiagramSpec (private consumer crate, built via kata)
   │
   ▼ Png.render(&d, &theme) → bytes  (zukai-render, resvg-rasterized)
   │
   ├── Atlassian MCP confluence_upload_attachment(page_id, "<id>.png", bytes)
   │
   └── zukai_render::confluence::embed("<id>.png", 840)
         → <ac:image ac:width="840"><ri:attachment ri:filename="<id>.png"/></ac:image>
         │
         ▼ Atlassian MCP confluence_update_page(body containing snippet)
         │
         ▼ curupira Chrome MCP — navigate, screenshot, eyeball  (optional VERIFY)
         │
         ▼ RenderAttestation (artifact⊕control⊕intent BLAKE3) → .typescape.yaml provenance
```

**Why PNG (not SVG, not Mermaid):** PNG via `ac:image ac:width` renders reliably across Confluence configurations (consistent font fallback, consistent sizing, PDF export friendly). SVG `ac:width` is honored inconsistently. Mermaid via the macro is editable in Confluence — defeats "spec is the source."

## Live proof points

Three live Confluence pages in the personal-space scratch area:

| # | Page id | Proves |
|---|---|---|
| 1 | `4112547842` | single-pattern end-to-end pipeline (`multi_region` N=2 → 1 img, 0 errors) |
| 2 | `4112351245` | multi-pattern composition (3 distinct patterns on one page → 3 imgs, 0 errors) |
| 3 | `4112678914` | Case B re-theming (existing Mermaid → `from_mermaid` → 3 nodes / 4 edges → PNG embed alongside source code-block → 1 img, 0 errors) |

(Live demo pages — page IDs above — live in a private engineer scratch space; URLs are not published here. The page IDs alone are sufficient to look them up from inside that workspace.)

## Standing acceptance demos

- `cargo run --example multi_region -p zukai-kata` — single-pattern (kata-authored multi_region N=2) → `/tmp/zukai-demo-multi-region.{svg,png,mmd}`.
- `cargo run --example composition_demo -p zukai-kata` — 3-pattern composition (multi_region N=3 + pillars + event_timeline) → `/tmp/zukai-comp-*.png`.
- `cargo run --example case_b_demo -p zukai-kata` — Case B lift Mermaid → render → `/tmp/zukai-case-b.png`.

## Skill

`pleme-io/blackmatter-pleme/skills/diagram-authoring/SKILL.md` — the skill that maps user intent to kata patterns + drives the publish pipeline. Updated for v0.1 with all 17 patterns + trigger phrases + the `confluence::embed` step in the DEPLOY phase.

## Closed loops since v0.0

- `Theme ← ishou::TokenSet` — ✅ `IshouTheme` (Nord palette wired).
- `DiagramSpec` carries `TataraDomain` — ✅ at primitive level (`Canvas` via reader round-trip).
- `zukai-lower` produces `iac_forge::sexpr::SExpr` — ✅ round-trip + BLAKE3 content-hash stable.
- Three-pillar BLAKE3 attestation (`tameshi`) — ✅ `RenderAttestation` primitive.
- 17 named kata patterns shipped (multi-region acceptance corpus + roadmap mining).
- Workhorse `multi_region` extended v0 (≤1 replica) → v1 (N regions, star replication, scaled widths). v0 1- and 2-region inputs preserved byte-identically (no regression).
- Case B helper `from_mermaid` shipped — 11 tests + 3 error variants + inline-decl extraction.
- Confluence delivery — proven 3× on live pages (single, composition, Case B).
- Kata-first authoring path established; primitive-composition example removed from `zukai-render` in favor of kata example in `zukai-kata`.

## Roadmap (next compounds)

- `cluster_topology` (single bounded region with services + I/O markers).
- `legend` — needs a new `Legend` primitive in `zukai-model` + renderer support (multi-crate scope).
- `from_mermaid` v1: chained edges (`A --> B --> C` on one line), alternate arrows (`==>`, `-.->`), subgraphs, classDefs.
- Shared `radial_distribution` base + thin wrappers (collapse `api_fanout` / `tree_hierarchy` / `read_replica_pair` / `decision_branch` — deferred; named patterns serve intent matching today).
- private `bordado` brand overlay (separate private repo; `Theme` seam is the contract).
- `arch-synthesizer` typescape registration — workspace-absorption (15 sibling-path-dep chain).
- `repo-forge new` formal scaffold for zukai — operational.
- Tatara extension for nested `Diagram` (extend `extractor_for` to recurse into custom structs).

## Public / private boundary

zukai is public. The brand overlay lives in a separate private repo (consumes the `Theme` seam). Per-renderer `*_keeps_overlay_brand_private` tests are structural proofs, not comments. All v0.1 artifacts grep-clean for tenant/brand names.

## How to validate

```bash
cd zukai
cargo test                                      # 120/120 ok
cargo run --example multi_region -p zukai-kata  # → /tmp/zukai-demo-multi-region.png
cargo run --example composition_demo -p zukai-kata
cargo run --example case_b_demo -p zukai-kata
```
