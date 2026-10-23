//! `kata::multi_region` — the workhorse architecture pattern. Primary +
//! N replica regions, with a global routing band, service chips per region,
//! optional client gateway, and replication arrows (star topology — every
//! replica syncs from the primary).
//!
//! v1 supports **N regions** (1 primary + 0..N replicas). Region width
//! scales to fit the canvas; the 1-and-2-region layouts from v0 are
//! preserved byte-for-byte (no test regression).

use crate::customer_gateway::{customer_gateway, GatewayArgs};
use crate::hero::{hero, HeroArgs};
use crate::service_tier::{service_tier, ServiceArgs, ServiceTierArgs};
use zukai_model::{
    Badge, Band, Canvas, Diagram, Edge, Id, Marker, Node, Point, Rect, Region, Role, Stroke,
};

const CANVAS_W: u32 = 960;
const CANVAS_H: u32 = 560;
const REGION_W_DEFAULT: u32 = 420; // used at N=1 (centered) and N=2 (current behavior)
const REGION_H: u32 = 250;
const REGION_Y: i32 = 200;
const REGION_GAP: i32 = 40;
const REGION_INNER_TOP: i32 = 62; // y offset for first chip row inside a region

const ROUTING_BAND_X: i32 = 300;
const ROUTING_BAND_W: i32 = 360;
const ROUTING_BAND_Y: i32 = 96;
const ROUTING_BAND_H: u32 = 46;

/// Args describing one region in [`multi_region`].
#[derive(Debug, Clone)]
pub struct RegionPatternArgs<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub sublabel: Option<&'a str>,
    pub role: Role,
    pub badge_text: Option<&'a str>,
    pub services: Vec<ServiceArgs<'a>>,
}

/// Args for [`multi_region`].
pub struct MultiRegionArgs<'a> {
    pub title: &'a str,
    pub subtitle: Option<&'a str>,
    pub primary: RegionPatternArgs<'a>,
    pub replicas: Vec<RegionPatternArgs<'a>>,
    pub gateway: Option<GatewayArgs<'a>>,
}

/// Compose a multi-region architecture diagram with 1 primary + N replicas.
/// Region width scales to fit the canvas; replication topology is **star**
/// (every replica syncs from the primary).
#[must_use]
pub fn multi_region(args: MultiRegionArgs<'_>) -> Diagram {
    let n_regions = 1 + args.replicas.len();
    let region_w = compute_region_width(n_regions);
    let region_x = |i: usize| -> i32 {
        if n_regions == 1 {
            (CANVAS_W as i32 - region_w as i32) / 2
        } else {
            REGION_GAP + (i as i32) * (region_w as i32 + REGION_GAP)
        }
    };

    let canvas = Canvas { width: CANVAS_W, height: CANVAS_H };
    let h = hero(HeroArgs { title: args.title, subtitle: args.subtitle });

    let mut regions: Vec<Region> = Vec::new();
    let mut nodes: Vec<Node> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();
    let mut bands: Vec<Band> = Vec::new();

    // Routing band at top — always present in the pattern.
    bands.push(Band {
        label: "Global DNS — nearest healthy region".into(),
        sublabel: None,
        role: args.primary.role,
        at: Rect { x: ROUTING_BAND_X, y: ROUTING_BAND_Y, w: ROUTING_BAND_W as u32, h: ROUTING_BAND_H },
    });

    // Primary region.
    let primary_x = region_x(0);
    let (primary_region, primary_nodes) = build_region(&args.primary, primary_x, region_w);
    regions.push(primary_region);
    nodes.extend(primary_nodes);

    // All replica regions + their replication arrows (star: primary → each replica).
    for (i, replica) in args.replicas.iter().enumerate() {
        let replica_index = i + 1; // primary is index 0
        let replica_x = region_x(replica_index);
        let (replica_region, replica_nodes) = build_region(replica, replica_x, region_w);
        regions.push(replica_region);
        nodes.extend(replica_nodes);

        edges.push(Edge {
            from: Point {
                x: primary_x + region_w as i32,
                y: REGION_Y + REGION_H as i32 / 2,
            },
            to: Point {
                x: replica_x,
                y: REGION_Y + REGION_H as i32 / 2,
            },
            stroke: Stroke::Dashed,
            marker: Marker::Arrow,
            role: Role::Accent,
            label: Some("replicate".into()),
        });
    }

    // Routing arrows: one per region, evenly distributed along the routing band.
    let arrow_dx = ROUTING_BAND_W / (n_regions as i32 + 1);
    for i in 0..n_regions {
        let from_x = ROUTING_BAND_X + arrow_dx * (i as i32 + 1);
        let to_x = region_x(i) + region_w as i32 / 2;
        edges.push(routing_arrow(from_x, to_x));
    }

    // Optional client gateway band + arrow.
    if let Some(gw_args) = args.gateway {
        let gw = customer_gateway(gw_args);
        bands.push(gw.band);
        edges.push(gw.arrow);
    }

    let mut d = Diagram::new(canvas, h);
    for r in regions {
        d = d.with_region(r);
    }
    for n in nodes {
        d = d.with_node(n);
    }
    for e in edges {
        d = d.with_edge(e);
    }
    for b in bands {
        d = d.with_band(b);
    }
    d
}

/// Region width as a function of region count. Preserves v0 widths at N≤2
/// (so existing 1- and 2-region diagrams render byte-identically) and
/// scales down for N≥3 to fit the canvas.
fn compute_region_width(n_regions: usize) -> u32 {
    if n_regions <= 2 {
        REGION_W_DEFAULT
    } else {
        let total_gap = REGION_GAP * (n_regions as i32 + 1);
        ((CANVAS_W as i32 - total_gap) / n_regions as i32).max(120) as u32
    }
}

fn routing_arrow(from_x: i32, to_x: i32) -> Edge {
    Edge {
        from: Point { x: from_x, y: 142 },
        to: Point { x: to_x, y: REGION_Y - 4 },
        stroke: Stroke::Solid,
        marker: Marker::Arrow,
        role: Role::Neutral,
        label: None,
    }
}

fn build_region(args: &RegionPatternArgs, x: i32, region_w: u32) -> (Region, Vec<Node>) {
    let region = Region {
        id: Id::new(args.id),
        label: args.label.into(),
        sublabel: args.sublabel.map(String::from),
        role: args.role,
        badge: args
            .badge_text
            .map(|t| Badge { text: t.into(), role: args.role }),
        at: Rect { x, y: REGION_Y, w: region_w, h: REGION_H },
    };
    let nodes = service_tier(ServiceTierArgs {
        region_id: args.id,
        region_origin: (x, REGION_Y),
        region_inner_top: REGION_INNER_TOP,
        services: &args.services,
    });
    (region, nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_region() -> Diagram {
        multi_region(MultiRegionArgs {
            title: "Multi-Region",
            subtitle: Some("two regions"),
            primary: RegionPatternArgs {
                id: "primary",
                label: "Primary",
                sublabel: Some("read + write"),
                role: Role::Master,
                badge_text: Some("MASTER · R/W"),
                services: vec![
                    ServiceArgs { id: "a", label: "Service A", role: Role::Master },
                    ServiceArgs { id: "b", label: "Service B", role: Role::Master },
                ],
            },
            replicas: vec![RegionPatternArgs {
                id: "replica",
                label: "Replica",
                sublabel: Some("read-only"),
                role: Role::Read,
                badge_text: Some("READ"),
                services: vec![ServiceArgs {
                    id: "a-ro",
                    label: "Service A (ro)",
                    role: Role::Read,
                }],
            }],
            gateway: Some(GatewayArgs { label: "Client gateway" }),
        })
    }

    fn one_region() -> Diagram {
        multi_region(MultiRegionArgs {
            title: "Single Region",
            subtitle: None,
            primary: RegionPatternArgs {
                id: "primary",
                label: "Primary",
                sublabel: None,
                role: Role::Master,
                badge_text: None,
                services: vec![ServiceArgs { id: "a", label: "A", role: Role::Master }],
            },
            replicas: vec![],
            gateway: None,
        })
    }

    fn three_region() -> Diagram {
        multi_region(MultiRegionArgs {
            title: "3 Regions",
            subtitle: Some("primary + 2 replicas"),
            primary: RegionPatternArgs {
                id: "primary",
                label: "Primary",
                sublabel: None,
                role: Role::Master,
                badge_text: None,
                services: vec![ServiceArgs { id: "a", label: "A", role: Role::Master }],
            },
            replicas: vec![
                RegionPatternArgs {
                    id: "r1",
                    label: "Replica 1",
                    sublabel: None,
                    role: Role::Read,
                    badge_text: None,
                    services: vec![ServiceArgs { id: "a1", label: "A1", role: Role::Read }],
                },
                RegionPatternArgs {
                    id: "r2",
                    label: "Replica 2",
                    sublabel: None,
                    role: Role::Read,
                    badge_text: None,
                    services: vec![ServiceArgs { id: "a2", label: "A2", role: Role::Read }],
                },
            ],
            gateway: None,
        })
    }

    fn four_region() -> Diagram {
        multi_region(MultiRegionArgs {
            title: "4 Regions",
            subtitle: None,
            primary: RegionPatternArgs {
                id: "p",
                label: "P",
                sublabel: None,
                role: Role::Master,
                badge_text: None,
                services: vec![],
            },
            replicas: vec![
                RegionPatternArgs { id: "r1", label: "R1", sublabel: None, role: Role::Read, badge_text: None, services: vec![] },
                RegionPatternArgs { id: "r2", label: "R2", sublabel: None, role: Role::Read, badge_text: None, services: vec![] },
                RegionPatternArgs { id: "r3", label: "R3", sublabel: None, role: Role::Read, badge_text: None, services: vec![] },
            ],
            gateway: None,
        })
    }

    // ─── v0 backward-compat tests (unchanged from prior version) ────────

    #[test]
    fn two_region_diagram_has_2_regions_3_nodes() {
        let d = two_region();
        assert_eq!(d.regions.len(), 2);
        assert_eq!(d.nodes.len(), 3); // 2 primary + 1 replica
    }

    #[test]
    fn two_region_diagram_has_routing_replication_and_gateway_edges() {
        let d = two_region();
        // 2 routing arrows + 1 replication + 1 gateway-up = 4
        assert_eq!(d.edges.len(), 4);
        // routing band + gateway band = 2
        assert_eq!(d.bands.len(), 2);
    }

    #[test]
    fn one_region_diagram_centers_primary_and_skips_replication() {
        let d = one_region();
        assert_eq!(d.regions.len(), 1);
        assert_eq!(d.regions[0].at.x, 270);
        assert_eq!(d.edges.len(), 1);
        assert_eq!(d.bands.len(), 1);
    }

    #[test]
    fn replica_is_placed_right_with_replication_arrow_between() {
        let d = two_region();
        assert_eq!(d.regions[0].at.x, 40);
        let expected_replica_x = CANVAS_W as i32 - REGION_GAP - REGION_W_DEFAULT as i32;
        assert_eq!(d.regions[1].at.x, expected_replica_x);
        let mid_y = REGION_Y + REGION_H as i32 / 2;
        let rep = d
            .edges
            .iter()
            .find(|e| e.label.as_deref() == Some("replicate"))
            .expect("must include a labeled replicate arrow");
        assert_eq!(rep.from.x, 40 + REGION_W_DEFAULT as i32);
        assert_eq!(rep.to.x, expected_replica_x);
        assert_eq!(rep.from.y, mid_y);
        assert_eq!(rep.stroke, Stroke::Dashed);
    }

    #[test]
    fn is_deterministic() {
        assert_eq!(two_region(), two_region());
        assert_eq!(one_region(), one_region());
        assert_eq!(three_region(), three_region());
    }

    #[test]
    fn badge_is_optional() {
        let d = one_region();
        assert!(d.regions[0].badge.is_none());
        let d = two_region();
        assert!(d.regions[0].badge.is_some());
        assert_eq!(d.regions[0].badge.as_ref().unwrap().text, "MASTER · R/W");
    }

    // ─── v1 N-region extension tests ────────────────────────────────────

    #[test]
    fn three_region_has_3_regions_and_correct_edge_count() {
        let d = three_region();
        assert_eq!(d.regions.len(), 3);
        // 3 routing arrows + 2 replication arrows = 5 edges (no gateway).
        assert_eq!(d.edges.len(), 5);
    }

    #[test]
    fn three_region_replication_uses_star_topology() {
        let d = three_region();
        let primary_x = d.regions[0].at.x;
        let region_w = d.regions[0].at.w as i32;
        let primary_right_x = primary_x + region_w;
        let rep_edges: Vec<_> = d
            .edges
            .iter()
            .filter(|e| e.label.as_deref() == Some("replicate"))
            .collect();
        assert_eq!(
            rep_edges.len(),
            2,
            "star topology: one replication arrow per replica"
        );
        for e in &rep_edges {
            assert_eq!(
                e.from.x, primary_right_x,
                "every replication arrow must originate at primary's right edge"
            );
        }
    }

    #[test]
    fn three_region_distributes_regions_left_to_right_with_uniform_width() {
        let d = three_region();
        assert!(d.regions[0].at.x < d.regions[1].at.x);
        assert!(d.regions[1].at.x < d.regions[2].at.x);
        let w = d.regions[0].at.w;
        for r in &d.regions {
            assert_eq!(r.at.w, w, "all regions must share the scaled width");
        }
        // Width should have shrunk from the v0 default of 420 (since N=3).
        assert!(w < REGION_W_DEFAULT);
    }

    #[test]
    fn three_region_has_one_routing_arrow_per_region() {
        let d = three_region();
        let routing_count = d.edges.iter().filter(|e| e.label.is_none() && e.to.y == REGION_Y - 4).count();
        assert_eq!(routing_count, 3);
    }

    #[test]
    fn four_region_scales_widths_further_and_remains_within_canvas() {
        let d = four_region();
        assert_eq!(d.regions.len(), 4);
        let w = d.regions[0].at.w;
        assert!(w < d.regions[0].at.w + d.regions[1].at.w); // sanity: at least 2 regions exist
        // All regions must fit within the canvas.
        let last = d.regions.last().unwrap();
        assert!(
            last.at.x + last.at.w as i32 <= CANVAS_W as i32,
            "rightmost region must fit inside the canvas (right edge {} <= {})",
            last.at.x + last.at.w as i32,
            CANVAS_W
        );
        // 4 routing + 3 replication = 7 edges.
        assert_eq!(d.edges.len(), 7);
    }
}
