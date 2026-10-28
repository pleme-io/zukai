use blue_lang_host::{Entry, Error, Host, NoLoader, Program};
use tatara_lisp::TataraDomain;
use zukai_confluence::{host::Confluence, Page};
use zukai_kata::host::Kata;
use zukai_kata::record::{Form, Typed};
use zukai_model::{Band, Diagram, Edge, Hero, Node, Role};

const PATTERNS: &str = include_str!("patterns.b");

fn program() -> Program<Kata> {
    Program::load_with(Entry::anonymous(PATTERNS), &NoLoader, &mut Kata).expect("patterns.b loads")
}

fn one<T: blue_lang_host::FromValue>(name: &str) -> T {
    program().call(name, (), &mut Kata).expect("the definition runs")
}

fn fragment(name: &str) -> Vec<Form> {
    one(name)
}

fn refusal(name: &str) -> String {
    program()
        .call::<Form>(name, (), &mut Kata)
        .unwrap_err()
        .to_string()
}

#[test]
fn the_surface_is_every_kata_pattern() {
    let names: Vec<&str> = Kata::SURFACE.iter().map(|f| f.name).collect();
    assert_eq!(
        names,
        [
            "kata_multi_region",
            "kata_hero",
            "kata_closing",
            "kata_pillars",
            "kata_service_tier",
            "kata_architecture_layers",
            "kata_customer_gateway",
            "kata_zero_knowledge",
            "kata_ha_failover",
            "kata_read_replica_pair",
            "kata_api_fanout",
            "kata_decision_branch",
            "kata_tree_hierarchy",
            "kata_routing_detail",
            "kata_data_tier",
            "kata_event_timeline",
            "kata_interaction_lane",
        ]
    );
    assert!(Kata::SURFACE.iter().all(|f| f.params == ["spec"]));
}

#[test]
fn multi_region_default_two_region_round_trip() {
    let Typed(d) = one::<Typed<Diagram>>("default_two_region");
    assert_eq!(d.regions.len(), 2);
    assert_eq!(d.nodes.len(), 8);
}

#[test]
fn multi_region_parameterized() {
    let Typed(d) = one::<Typed<Diagram>>("parameterized_region");
    assert_eq!(d.regions.len(), 2);
    assert_eq!(d.nodes.len(), 2, "1 primary svc-a + 1 replica svc-a-r");
}

#[test]
fn hero_pattern() {
    let Typed(h) = one::<Typed<Hero>>("welcome_hero");
    assert_eq!(h.title, "W");
    assert_eq!(h.subtitle.as_deref(), Some("to zukai"));
}

#[test]
fn closing_pattern() {
    let Typed(b) = one::<Typed<Band>>("all_clear");
    assert_eq!(b.label, "All clear");
    assert_eq!(b.role, Role::Success);
}

#[test]
fn pillars_pattern() {
    let nodes = one::<Vec<Typed<Node>>>("four_pillars");
    assert_eq!(nodes.len(), 4);
}

#[test]
fn service_tier_returns_n_nodes() {
    let nodes = one::<Vec<Typed<Node>>>("three_services");
    assert_eq!(nodes.len(), 3);
}

#[test]
fn architecture_layers_returns_n_nodes() {
    assert_eq!(fragment("three_layers").len(), 3);
}

#[test]
fn customer_gateway_returns_band_plus_edge() {
    let items = fragment("outbound_gateway");
    assert_eq!(items.len(), 2);
    Band::compile_from_sexp(&items[0].0).expect("a band first");
    Edge::compile_from_sexp(&items[1].0).expect("an edge second");
}

#[test]
fn zero_knowledge_returns_2_nodes_plus_edge() {
    assert_eq!(fragment("sealed_exchange").len(), 3);
}

#[test]
fn ha_failover_returns_2_nodes_plus_2_edges() {
    assert_eq!(fragment("failover_pair").len(), 4);
}

#[test]
fn read_replica_pair_pattern() {
    assert_eq!(fragment("three_replicas").len(), 7);
}

#[test]
fn api_fanout_pattern() {
    assert_eq!(fragment("three_backends").len(), 7);
}

#[test]
fn decision_branch_pattern() {
    assert_eq!(fragment("status_decision").len(), 5);
}

#[test]
fn tree_hierarchy_pattern() {
    assert_eq!(fragment("three_branch_tree").len(), 7);
}

#[test]
fn routing_detail_pattern() {
    assert_eq!(fragment("dns_to_service").len(), 5);
}

#[test]
fn data_tier_pattern() {
    assert_eq!(fragment("hot_warm_cold").len(), 5);
}

#[test]
fn event_timeline_pattern() {
    assert_eq!(fragment("cutover_timeline").len(), 4);
}

#[test]
fn interaction_lane_pattern() {
    assert_eq!(fragment("request_response").len(), 7);
}

#[test]
fn a_pattern_result_is_plain_data_blue_can_read() {
    let mut p = program();
    let nodes: Vec<Form> = p.call("three_services", (), &mut Kata).expect("runs");
    let ids: Vec<String> = p.call("ids_of", (nodes,), &mut Kata).expect("runs");
    assert_eq!(ids, ["a", "b", "c"]);
}

#[test]
fn a_misspelled_field_is_refused_by_name() {
    let msg = refusal("misspelled_hero");
    assert!(
        msg.contains("kata_hero") && msg.contains("hero has no field :subtitel"),
        "{msg}"
    );
}

#[test]
fn an_unknown_role_is_refused_by_name() {
    let msg = refusal("unknown_role");
    assert!(msg.contains("unknown role :purple"), "{msg}");
}

#[test]
fn a_missing_field_is_refused_by_name() {
    let src = "def bare()\n  kata_hero({subtitle: \"no title\"})\nend\n";
    let mut p = Program::load_with(Entry::anonymous(src), &NoLoader, &mut Kata).expect("loads");
    let err = p.call::<Form>("bare", (), &mut Kata).unwrap_err();
    assert!(matches!(err, Error::Call(_)), "{err}");
    assert!(err.to_string().contains("hero needs the field title"), "{err}");
}

const HAND_OVER: &str = "\
def body()
  cf_compose([cf_panel_info(\"x\"), cf_hr, cf_panel_tip(\"y\")])
end

def page(title, body)
  {kind: :defpage, title:, body:}
end
";

#[test]
fn end_to_end_page_with_kata_diagram_embed() {
    let Typed(diagram) = one::<Typed<Diagram>>("parameterized_region");
    assert_eq!(diagram.regions.len(), 2);

    let mut pages = Program::load_with(Entry::anonymous(HAND_OVER), &NoLoader, &mut Confluence)
        .expect("the page program loads");
    let body: String = pages.call("body", (), &mut Confluence).expect("body");
    assert!(body.contains(r#"ac:name="info""#));

    let Typed(page) = pages
        .call::<Typed<Page>>("page", ("Hand-Over", body), &mut Confluence)
        .expect("a defpage map reads as a Page");
    assert_eq!(page.title, "Hand-Over");
    assert!(page.body.contains(r#"ac:name="tip""#));
}
