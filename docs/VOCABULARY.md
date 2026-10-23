# zukai — pattern vocabulary (kata 型)

The named-pattern glossary for `zukai-kata` — the **re-use space** where
the structural primitives of `zukai-model` compose into idioms. Pick from
this glossary by intent; the `diagram-authoring` skill maps user intent
("draw the multi-region architecture for X") to the right pattern here.

Vocabulary follows `pleme-io/theory/CLAUDE.md`'s rule: terms are defined
before use. Each pattern earns its place by being reusable across many
diagrams (the duplication budget is zero).

## Patterns shipped (v0)

### `hero(HeroArgs) -> Hero`
Title + optional subtitle. The consistent header strip every diagram opens with.

- **Args:** `title: &str`, `subtitle: Option<&str>`.
- **Returns:** `zukai_model::Hero`.
- **When:** every top-level diagram. `multi_region` (and any composite pattern) calls this internally.
- **Example:**
  ```rust
  use zukai_kata::{hero, HeroArgs};
  let h = hero(HeroArgs { title: "Multi-Region", subtitle: Some("primary + replica") });
  ```

### `service_tier(ServiceTierArgs) -> Vec<Node>`
A grid (3 cols, 120×34 chips, 10px gaps) of role-colored service chips inside a region. Each chip carries a `Role` whose color resolves through the active `Theme`.

- **Args:** `region_id`, `region_origin: (i32, i32)` (region's top-left in canvas units), `region_inner_top: i32` (y offset past the region header band), `services: &[ServiceArgs]`.
- **Returns:** `Vec<zukai_model::Node>` — caller pushes these into the `Diagram`.
- **When:** populating a region with named services. `multi_region` calls this per region.

### `customer_gateway(GatewayArgs) -> GatewayFragment`
The canonical "outbound HTTPS only, no inbound firewall changes" pattern: a band at the bottom of the canvas + a dashed arrow upward to the global routing band.

- **Args:** `label: &str`.
- **Returns:** `GatewayFragment { band: Band, arrow: Edge }` — caller pushes both into the `Diagram`.
- **When:** any diagram showing a customer/client side reaching into the platform.

### `multi_region(MultiRegionArgs) -> Diagram`
The workhorse architecture pattern. Primary + N replica regions, laid out side-by-side with a global routing band + per-region routing arrows + optional client gateway + replication arrows (**star topology** — every replica syncs from the primary).

- **Args:** `title`, `subtitle`, `primary: RegionPatternArgs`, `replicas: Vec<RegionPatternArgs>`, `gateway: Option<GatewayArgs>`.
- **Returns:** a fully composed `Diagram` ready to render through any `Renderer`.
- **When:** any multi-region architecture diagram.
- **Notes (v1):** N regions supported. Region width scales to fit the canvas for N≥3; the 1- and 2-region layouts from v0 are preserved byte-identically.
- **Example:**
  ```rust
  use zukai_kata::{multi_region, MultiRegionArgs, RegionPatternArgs, GatewayArgs, ServiceArgs};
  use zukai_model::Role;
  let d = multi_region(MultiRegionArgs {
      title: "Multi-Region",
      subtitle: Some("primary + replica"),
      primary: RegionPatternArgs {
          id: "primary", label: "Primary", sublabel: Some("read + write"),
          role: Role::Master, badge_text: Some("MASTER · R/W"),
          services: vec![ServiceArgs { id: "a", label: "Service A", role: Role::Master }],
      },
      replicas: vec![RegionPatternArgs { /* … */ }],
      gateway: Some(GatewayArgs { label: "Client gateway" }),
  });
  ```

### `pillars(PillarsArgs) -> Vec<Node>` — N-column foundation
Evenly distribute labeled pillars across a width, each themed by its `Role`. The "principles / foundational ideas / N pillars" diagram shape.

- **Args:** `origin: (i32, i32)`, `width: u32`, `pillar_height: u32`, `gap: u32`, `pillars: &[PillarArgs]` (each with `id`, `label`, `role`).
- **Returns:** `Vec<zukai_model::Node>`.
- **When:** "principles of X", "the four pillars of Y", any cluster of foundational labels.

### `api_fanout(ApiFanoutArgs) -> ApiFanoutFragment` — central API → N backends
A central API node + a row of backend chips + fanout arrows from the API down to each backend. The "API gateway routes to N services" pattern.

- **Args:** `api_id`, `api_label`, `api_at: (i32,i32)`, `backends: &[BackendArgs]`, `backends_y: i32`, `canvas_width: u32`.
- **Returns:** `ApiFanoutFragment { api_node, backends: Vec<Node>, edges: Vec<Edge> }`.
- **When:** any "endpoint X fans out to N services" diagram.

### `ha_failover(HaFailoverArgs) -> HaFailoverFragment` — primary + standby + two opposing arrows
Master/standby pair with a solid `Danger` failover arrow (primary→standby) and a dashed `Success` recovery arrow (standby→primary) along the top/bottom edges respectively.

- **Args:** `primary`, `standby` (each `HaNodeArgs`), `primary_at`, `standby_at`, optional `failover_label` + `recovery_label`.
- **Returns:** `HaFailoverFragment { primary, standby, failover_edge, recovery_edge }`.
- **When:** any HA/DR diagram showing failover + recovery state transitions.

### `closing(ClosingArgs) -> Band` — bottom summary band
The mirror of `hero` at the end of a diagram — a wide single-line panel with optional sublabel.

- **Args:** `label: &str`, `sublabel: Option<&str>`, `role: Role`.
- **Returns:** `zukai_model::Band` — caller pushes into the `Diagram`.
- **When:** to close out a diagram with a summary statement / closing thought.

### `zero_knowledge(ZeroKnowledgeArgs) -> ZeroKnowledgeFragment` — client-sealed flow
Client (holds keys, `Role::Accent`) + Service (ciphertext only, `Role::Read`) + a single labeled solid `Accent` arrow client → service carrying the sealed payload.

- **Args:** `client_id/label`, `service_id/label`, `client_at/service_at`, optional `payload_label`.
- **Returns:** `ZeroKnowledgeFragment { client, service, sealed_payload_edge }`.
- **When:** any "client controls keys, service handles ciphertext" diagram (zero-knowledge architecture, end-to-end encryption, sealed payloads).

### `routing_detail(RoutingDetailArgs) -> RoutingDetailFragment` — N-stage horizontal routing chain
N stages laid out left-to-right + N-1 arrows connecting consecutive stages. Each stage carries its own role; each arrow can carry an optional transition label.

- **Args:** `origin`, `stage_width`, `stage_height`, `gap`, `stages: &[RoutingStageArgs]` (each `id`, `label`, `role`, optional `arrow_label`).
- **Returns:** `RoutingDetailFragment { stages: Vec<Node>, edges: Vec<Edge> }`.
- **When:** "follow-the-packet" / "DNS → LB → ingress → service" request-flow diagrams.

### `data_tier(DataTierArgs) -> DataTierFragment` — vertical storage hierarchy
Tiers stack top→bottom; downward arrows between adjacent tiers signal data aging / flow. Each tier carries its own role; each arrow can carry an optional transition label.

- **Args:** `origin`, `width`, `tier_height`, `gap`, `tiers: &[TierArgs]` (each `id`, `label`, `role`, optional `arrow_label`).
- **Returns:** `DataTierFragment { tiers: Vec<Node>, edges: Vec<Edge> }`.
- **When:** storage hierarchy diagrams (hot → warm → cold), cache tier diagrams, archive flow diagrams.

### `tree_hierarchy(TreeHierarchyArgs) -> TreeHierarchyFragment` — root + N branches (2-level)
v0 is 2-level (root + branches with `Neutral` arrows). Distinct from [`api_fanout`](#) semantically: `api_fanout` is REQUEST routing (one→many for distribution); `tree_hierarchy` is CONTAINMENT (one is composed of many). Root's role is caller-configurable.

- **Args:** `root: TreeNodeArgs`, `root_at: (i32,i32)`, `branches: &[TreeNodeArgs]`, `branches_y: i32`, `canvas_width: u32`.
- **Returns:** `TreeHierarchyFragment { root, branches: Vec<Node>, edges: Vec<Edge> }`.
- **When:** containment hierarchies (org charts, attribution trees, dependency trees, forge distribution maps).
- **v1 follow-up:** add a 3rd `leaves: &[TreeLeafArgs]` level (each branch with its own leaves stacked vertically).

### `read_replica_pair(ReadReplicaPairArgs) -> ReadReplicaPairFragment` — master + N read replicas
Master themed `Master` + N replicas themed `Read` + dashed `Read` sync arrows master → each replica. Distinct from [`ha_failover`](#) (which is 1:1 + bidirectional state transitions); this is 1:N replication.

- **Args:** `master_id/label`, `master_at`, `replicas: &[ReadReplicaArgs]`, `replicas_y`, `canvas_width`.
- **Returns:** `ReadReplicaPairFragment { master, replicas: Vec<Node>, sync_edges: Vec<Edge> }`.
- **When:** read-scale architectures (database read replicas, cache replicas, materialized-view fan-out).

### `event_timeline(EventTimelineArgs) -> EventTimelineFragment` — horizontal timeline with event markers
Baseline (horizontal arrow left→right showing time) + N event chips evenly distributed above the baseline. Each chip carries its own `Role` (e.g., `Accent` for the critical event, `Success` for verifications).

- **Args:** `origin: (i32, i32)` (left endpoint of baseline), `width: u32`, `events: &[EventArgs]` (each `id`, `label`, `role`).
- **Returns:** `EventTimelineFragment { baseline: Edge, event_chips: Vec<Node> }`.
- **When:** change-window plans, incident timelines, milestone runbooks, sequence-of-events diagrams.
- **v1 follow-up:** separate timestamp labels below the baseline + tick marks connecting each event to its baseline point.

### `architecture_layers(ArchitectureLayersArgs) -> Vec<Node>` — N labeled layers stacked vertically
Each layer is a wide chip; **no arrows** between layers. The visual distinction from [`data_tier`](#) (which shows downward data flow): layers are about **responsibility** and depends-on relationships, not data motion.

- **Args:** `origin`, `width`, `layer_height`, `gap`, `layers: &[LayerArgs]` (each `id`, `label`, `role`).
- **Returns:** `Vec<zukai_model::Node>`.
- **When:** presentation/business/data, controller/domain/infrastructure, any "responsibility stack" diagram.

### `decision_branch(DecisionBranchArgs) -> DecisionBranchFragment` — flowchart-style decision with N labeled branches
Decision node (themed `Accent`) + N branch nodes + N arrows whose **labels carry the decision values** (yes/no, ok/warn/error, etc.). Distinct from [`api_fanout`](#) because the arrow labels are first-class.

- **Args:** `decision_id/label`, `decision_at`, `branches: &[BranchArgs]` (each `id`, `label`, `role`, `branch_label`), `branches_y`, `canvas_width`.
- **Returns:** `DecisionBranchFragment { decision, branches: Vec<Node>, edges: Vec<Edge> }`.
- **When:** any flowchart decision point, error-handling fork, conditional branch.

### `interaction_lane(InteractionLaneArgs) -> InteractionLaneFragment` — two-actor sequence diagram (UML-style)
Two actor chips at the top + two vertical dashed lifelines + N horizontal labeled message arrows between the lifelines (each message has its own direction).

- **Args:** `left_id/label`, `right_id/label`, `origin: (i32,i32)`, `lane_width: u32`, `lifeline_length: u32`, `messages: &[InteractionMessageArgs]` (each `label`, `from_left_to_right: bool`, `role`).
- **Returns:** `InteractionLaneFragment { left_actor, right_actor, left_lifeline, right_lifeline, messages: Vec<Edge> }`.
- **When:** client/server protocol diagrams, request/response sequences, two-party negotiation flows, sequence-of-RPC diagrams.

## tatara-lisp authoring surface — `.tlisp` files

The **dialect that drives the entire solution.** Every `zukai-model` primitive (`Hero`, `Badge`, `Region`, `Node`, `Edge`, `Band`, `Diagram`, plus `Canvas` via `#[derive(TataraDomain)]`) carries a `TataraDomain` impl, so a complete diagram lives as one `.tlisp` form in `zukai/specs/diagrams/<name>.tlisp` and compiles via `tatara_lisp::reader::read` + `Diagram::compile_from_sexp`.

### Authoring surface (`(defdiagram …)`)

```lisp
(defdiagram
  :canvas  (defcanvas :width 960 :height 560)
  :hero    (defhero   :title "…" :subtitle "…")
  :regions ((defregion :id "primary" :label "Primary" :role :master
                       :badge (defbadge :text "MASTER · R/W" :role :master)
                       :at (defrect :x 40 :y 200 :w 420 :h 250))
            …)
  :nodes   ((defnode :id "svc-a" :label "Service A" :role :master :region "primary"
                     :at (defrect :x 60 :y 270 :w 120 :h 34))
            …)
  :edges   ((defedge :from (defpoint :x 460 :y 325) :to (defpoint :x 500 :y 325)
                     :stroke :dashed :marker :arrow :role :accent :label "replicate")
            …)
  :bands   ((defband :label "Global DNS — …" :role :master
                     :at (defrect :x 300 :y 96 :w 360 :h 46))
            …))
```

### Keyword vocabulary (Phase A, landed)

| Keyword | Type | Role |
|---|---|---|
| `defcanvas`  | `Canvas`  | `:width :height` |
| `defhero`    | `Hero`    | `:title :subtitle` |
| `defbadge`   | `Badge`   | `:text :role` |
| `defregion`  | `Region`  | `:id :label :sublabel :role :badge :at` |
| `defnode`    | `Node`    | `:id :label :role :region :at` |
| `defedge`    | `Edge`    | `:from :to :stroke :marker :role :label` |
| `defband`    | `Band`    | `:label :sublabel :role :at` |
| `defrect`    | `Rect` (helper) | `:x :y :w :h` |
| `defpoint`   | `Point` (helper) | `:x :y` |
| `defdiagram` | `Diagram` | `:canvas :hero :regions :nodes :edges :bands` |

`Role` / `Stroke` / `Marker` are keyword-form enums (`:master :read :accent :neutral :danger :success`, `:solid :dashed`, `:none :arrow`).

### Standing acceptance demo

`cargo run --example tlisp_render -p zukai-kata` reads `specs/diagrams/example_2region.tlisp` from disk, compiles to a typed `Diagram`, renders SVG/PNG/Mermaid into `/tmp/zukai-tlisp-demo.{svg,png,mmd}`.

### What's next

All Phases A–F shipped (see sections below). Open follow-ups: extend the derive macro with `#[tatara(role_keyword)]` (keyword-form enums like Role/Stroke/Marker) and `#[tatara(via_string)]` (newtype wrappers like Id). With those, every remaining manual `TataraDomain` impl on Region/Node/Edge/Band/Badge/Diagram collapses to a one-line derive — Phase F-extended is mechanical at that point.

## blue authoring surface — kata patterns and Confluence primitives

Page bodies and layout patterns are authored as **blue** (`.b`) and run through
`blue-lang-host`. Two host types carry the surface; each method is one blue host
function, and `Host::SURFACE` lists them (gated by `tests/host.rs` in each crate).

| Host | Prefix | Functions |
|---|---|---|
| `zukai_confluence::host::Confluence` | `cf_` | `panel_info` `panel_warning` `panel_note` `panel_tip` `panel_success` `expand` `code` `status` `toc` `tabs` `tasklist` `two_column` `three_column` `paragraph` `hr` `link` `page_link` `mention` `compose` `table` |
| `zukai_kata::host::Kata` | `kata_` | `multi_region` `hero` `closing` `pillars` `service_tier` `architecture_layers` `customer_gateway` `zero_knowledge` `ha_failover` `read_replica_pair` `api_fanout` `decision_branch` `tree_hierarchy` `routing_detail` `data_tier` `event_timeline` `interaction_lane` |

```blue
def body()
  cf_compose(
    [
      cf_panel_info("Read this first."),
      cf_hr,
      cf_paragraph("Architecture follows.")
    ]
  )
end

def tiers()
  kata_data_tier(
    {
      origin_x: 200,
      origin_y: 120,
      width: 560,
      tier_height: 80,
      gap: 30,
      tiers: [
        {id: "hot", label: "Hot", role: :accent, arrow_label: "age out"},
        {id: "cold", label: "Cold", role: :neutral}
      ]
    }
  )
end
```

- **Confluence primitives** take strings and return storage XML, so nesting is composition. `cf_tabs` takes `[title, body]` pairs, `cf_tasklist` takes `[done, text]` pairs, `cf_page_link(id, nil)` omits the link text, and `cf_compose` takes one list of strings.
- **Kata patterns** take one map whose keys are the kata args in snake_case (`origin_x`, `canvas_width`); nested specs are maps too (`{id:, label:, role:}`). A misspelled or missing key and an unknown role are refused naming the field. `kata_multi_region` keeps its short form (`n_replicas`, `gateway_label`).
- **Results are plain blue data.** A primitive is a map with `kind` (`:defnode`, `:defedge`, `:defband`, `:defhero`, `:defdiagram`) and its fields, so blue reads `node.id`. Rust reads one back as a typed value with `zukai_kata::record::Typed<T>` (map → form → `TataraDomain::compile_from_sexp`), and a `{kind: :defpage, title:, body:}` map reads as a `Page`.
- **One program, one host.** `blue-lang-host` composes no two `#[host]` types, so a program reaches either the `cf_` or the `kata_` surface; a page that needs both builds the diagram in one program and the body in the other.

The worked programs are `crates/zukai-confluence/tests/pages.b` and `crates/zukai-kata/tests/patterns.b`.

## Case B helper — re-theme existing Mermaid

### `from_mermaid(src) -> Result<Diagram, FromMermaidError>`
Parse a simple Mermaid `flowchart` block into a renderable `Diagram` so existing Confluence Mermaid can flow through the same `Theme` seam as kata-authored diagrams. The **Case B** path — "enhance an existing page by re-theming its diagram."

- **Args:** `src: &str` (Mermaid source).
- **Returns:** `Result<Diagram, FromMermaidError>` (`Empty` / `NoFlowchartHeader` / `InvalidDirection(_)`).
- **v0 surface:** header `flowchart TD|BT|LR|RL` (or `graph …`), node decls `A[Label]` / `A(Label)`, edges (one per line) `A --> B` and `A -- label --> B`, implicit nodes (id-as-label), comments (`%%`) and unrecognized lines silently skipped.
- **v0 limitations:** no chained edges on one line (`A --> B --> C` — write as two lines), no subgraphs, no `classDef`/styling, no alternate arrow glyphs (`==>`, `-.->`).
- **When:** lifting an existing Mermaid block out of a Confluence page (Case B workflow), then re-rendering through `Png` + uploading + embedding for the themed look.

## Confluence delivery helper

### `zukai_render::confluence::embed(filename, width) -> String`
Build the Confluence storage-format snippet that embeds a previously-uploaded PNG attachment as a sized image:
```
<ac:image ac:width="840"><ri:attachment ri:filename="diagram.png"/></ac:image>
```
**Use with:** `Atlassian MCP confluence_upload_attachment` to upload the PNG bytes from `Png.render(...)`, then drop this snippet into the page body via `confluence_update_page`. XML-attribute-escapes the filename.

## Phase F — derive-macro recursion via `#[tatara(domain)]`

`tatara-lisp-derive` gained a `#[tatara(domain)]` field attribute (post-v0.1, this push). On any field annotated with it, the macro generates a direct `<T as TataraDomain>::compile_from_sexp(sexp)` call — no serde JSON intermediate. Supports all three wrapper cases:

```rust
#[derive(TataraDomain)]
#[tatara(keyword = "defframed")]
struct Framed {
    title: String,                      // existing scalar handling
    #[tatara(domain)] bounds: Rect,     // T → compile_from_sexp(sexp)
    #[tatara(domain)] anchor: Option<Point>,  // Option<T> → if present, compile
    #[tatara(domain)] rects: Vec<Rect>,       // Vec<T> → iterate + compile each
}
```

**Already-collapsed primitives (post-Phase F, this push):**

| Type | Form | Status |
|---|---|---|
| `Canvas` | `(defcanvas :width :height)` | derive (since v0.0) |
| `Hero`   | `(defhero :title :subtitle)` | **derive (post-Phase F)** |
| `Rect`   | `(defrect :x :y :w :h)` | **derive (post-Phase F)** |
| `Point`  | `(defpoint :x :y)` | **derive (post-Phase F)** |
| `Badge` `Region` `Node` `Edge` `Band` `Diagram` | (their existing `def…` forms) | manual — pending macro extensions for keyword-form enums + newtype wrappers |

The manual `rect_from_form` / `point_from_form` helpers in `zukai-model::lisp` are gone — callers use `Rect::compile_from_sexp(sexp)` / `Point::compile_from_sexp(sexp)` directly via the derive-generated impl.

3 round-trip tests in `zukai_model::lisp::tests` prove the macro extension end-to-end:

- `tatara_domain_attribute_plain_nested` — `Test1 { title: String, #[tatara(domain)] bounds: Rect }` round-trips.
- `tatara_domain_attribute_option_some_and_none` — `Option<Point>` field — both present and absent cases.
- `tatara_domain_attribute_vec_recursion` — `Vec<Rect>` field — three-element list compiles back to three typed values.

## Confluence document primitives — `zukai-confluence`

The **documentation half** of zukai. Sibling crate to `zukai-render`; pure Rust functions that emit Confluence storage XML for non-diagram structural elements. Every primitive returns `String`, XML-escapes user input where unsafe, and is deterministic. Compose into a Confluence page body alongside `zukai_render::confluence::embed` for the diagram parts.

### Panel macros — `panel::*`
- `info(body)` · `warning(body)` · `note(body)` · `tip(body)` · `success(body)`
- All wrap `body` in `<p>…</p>` (text-escaped) inside an `<ac:rich-text-body>` of the corresponding macro.

### Structure macros — `structure::*`
- `expand(title, body)` — collapsible (`body` verbatim, may contain other macros).
- `code(language, body)` — CDATA-wrapped code block with syntax highlighting.
- `toc(min_level, max_level)` — auto table-of-contents.
- `tabs(&[(title, body)])` — tabbed content (one tab per pane; body verbatim).
- `tasklist(&[(checked, text)])` — task list with complete/incomplete state.
- `status(color, text)` — inline status chip; color ∈ `{Grey, Red, Yellow, Green, Blue, Purple}`.

### Layout macros — `layout::*`
- `two_column(left, right)` — two-column equal-width layout.
- `three_column(left, middle, right)` — three-column equal-width.
- Cell content is verbatim (panels, diagrams, tables all valid).

### Inline references — `inline::*`
- `paragraph(body)` — `<p>` with text-escaped body.
- `hr()` — horizontal rule.
- `link(text, url)` — external `<a>` link (text + URL both escaped).
- `page_link(content_id, link_text)` — internal Confluence page link.
- `mention(account_id)` — `@`-mention by Atlassian account id.

### Table builder — `table(header, rows)`
- Standard HTML `<table>` with `<th>` header row + `<td>` body cells (all text-escaped). Confluence renders natively.

## Future patterns (open follow-up compounds)

Roadmap mined from the multi-region acceptance corpus + cross-cluster (FinOps/Perf) docs:

- `legend(LegendArgs) -> Legend` — diagram legend. Requires adding a `Legend` primitive to `zukai-model` first.
- `data_flow_pipeline(...)` — N-stage pipeline with transitions (Perf Phase A→B→C shape).
- `cluster_topology(...)` — bounded cluster with services + ingress/egress.
- `kata::from_mermaid(src) -> Result<Diagram>` — Case B helper: parse simple Mermaid `flowchart` into matching kata patterns automatically.

## Adding a new pattern (the recipe)

1. Create `crates/zukai-kata/src/<name>.rs` with the args struct(s) + the pattern fn.
2. Re-export from `crates/zukai-kata/src/lib.rs`.
3. Add a row to this file (name · args · returns · when).
4. Write tests: structural invariants (counts, positions) + determinism (calling twice produces equal output).
5. (Optional) update `blackmatter-pleme/skills/diagram-authoring/SKILL.md` trigger list if the pattern unlocks a new authoring intent.

## Cross-references

- Structural primitives the patterns compose: `crates/zukai-model/src/lib.rs`.
- Render seam (every `Diagram` flows through this): `crates/zukai-render/src/lib.rs`.
- Authoring + Confluence loop skill that reaches for these patterns: `blackmatter-pleme/skills/diagram-authoring/SKILL.md`.
- Design note + reuse ledger: `docs/DESIGN.md`.
- `theory/THEORY.md` §II.3 (naming), §III.1 (artifact ≠ source), §VI (generation over composition).
