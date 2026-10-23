use blue_lang_host::host;
use zukai_model::Role;

use crate::record::{record, Form, RoleKey};
use crate::{
    api_fanout::{api_fanout, ApiFanoutArgs, BackendArgs},
    architecture_layers::{architecture_layers, ArchitectureLayersArgs, LayerArgs},
    closing::{closing, ClosingArgs},
    customer_gateway::{customer_gateway, GatewayArgs},
    data_tier::{data_tier, DataTierArgs, TierArgs},
    decision_branch::{decision_branch, BranchArgs, DecisionBranchArgs},
    event_timeline::{event_timeline, EventArgs, EventTimelineArgs},
    ha_failover::{ha_failover, HaFailoverArgs, HaNodeArgs},
    hero::{hero, HeroArgs},
    interaction_lane::{interaction_lane, InteractionLaneArgs, InteractionMessageArgs},
    multi_region::{multi_region, MultiRegionArgs, RegionPatternArgs},
    pillars::{pillars, PillarArgs, PillarsArgs},
    read_replica_pair::{read_replica_pair, ReadReplicaArgs, ReadReplicaPairArgs},
    routing_detail::{routing_detail, RoutingDetailArgs, RoutingStageArgs},
    service_tier::{service_tier, ServiceArgs, ServiceTierArgs},
    tree_hierarchy::{tree_hierarchy, TreeHierarchyArgs, TreeNodeArgs},
    zero_knowledge::{zero_knowledge, ZeroKnowledgeArgs},
};

record!(Chip = "chip" { id: String, label: String, role: RoleKey });
record!(Staged = "stage" { id: String, label: String, role: RoleKey, arrow_label: Option<String> });
record!(Branch = "branch" { id: String, label: String, role: RoleKey, branch_label: String });
record!(Message = "message" { label: String, from_left_to_right: bool, role: RoleKey });
record!(HaNode = "ha_node" { id: String, label: String, sublabel: Option<String> });
record!(Replica = "replica" { id: String, label: String });
record!(Gateway = "gateway" { label: String });
record!(Region = "region" {
    id: String,
    label: String,
    sublabel: Option<String>,
    role: RoleKey,
    badge_text: Option<String>,
    services: Vec<Chip>,
});

record!(MultiRegionSpec = "multi_region" {
    title: String,
    subtitle: Option<String>,
    primary: Option<Region>,
    replicas: Option<Vec<Region>>,
    n_replicas: Option<u32>,
    gateway: Option<Gateway>,
    gateway_label: Option<String>,
});
record!(HeroSpec = "hero" { title: String, subtitle: Option<String> });
record!(ClosingSpec = "closing" { label: String, sublabel: Option<String>, role: RoleKey });
record!(PillarsSpec = "pillars" {
    origin_x: i32,
    origin_y: i32,
    width: u32,
    pillar_height: u32,
    gap: u32,
    pillars: Vec<Chip>,
});
record!(ServiceTierSpec = "service_tier" {
    region_id: String,
    region_origin_x: i32,
    region_origin_y: i32,
    region_inner_top: i32,
    services: Vec<Chip>,
});
record!(ArchitectureLayersSpec = "architecture_layers" {
    origin_x: i32,
    origin_y: i32,
    width: u32,
    layer_height: u32,
    gap: u32,
    layers: Vec<Chip>,
});
record!(ZeroKnowledgeSpec = "zero_knowledge" {
    client_id: String,
    client_label: String,
    service_id: String,
    service_label: String,
    client_x: i32,
    client_y: i32,
    service_x: i32,
    service_y: i32,
    payload_label: Option<String>,
});
record!(HaFailoverSpec = "ha_failover" {
    primary: HaNode,
    standby: HaNode,
    primary_x: i32,
    primary_y: i32,
    standby_x: i32,
    standby_y: i32,
    failover_label: Option<String>,
    recovery_label: Option<String>,
});
record!(ReadReplicaPairSpec = "read_replica_pair" {
    master_id: String,
    master_label: String,
    master_x: i32,
    master_y: i32,
    replicas: Vec<Replica>,
    replicas_y: i32,
    canvas_width: u32,
});
record!(ApiFanoutSpec = "api_fanout" {
    api_id: String,
    api_label: String,
    api_x: i32,
    api_y: i32,
    backends: Vec<Chip>,
    backends_y: i32,
    canvas_width: u32,
});
record!(DecisionBranchSpec = "decision_branch" {
    decision_id: String,
    decision_label: String,
    decision_x: i32,
    decision_y: i32,
    branches: Vec<Branch>,
    branches_y: i32,
    canvas_width: u32,
});
record!(TreeHierarchySpec = "tree_hierarchy" {
    root: Chip,
    root_x: i32,
    root_y: i32,
    branches: Vec<Chip>,
    branches_y: i32,
    canvas_width: u32,
});
record!(RoutingDetailSpec = "routing_detail" {
    origin_x: i32,
    origin_y: i32,
    stage_width: u32,
    stage_height: u32,
    gap: u32,
    stages: Vec<Staged>,
});
record!(DataTierSpec = "data_tier" {
    origin_x: i32,
    origin_y: i32,
    width: u32,
    tier_height: u32,
    gap: u32,
    tiers: Vec<Staged>,
});
record!(EventTimelineSpec = "event_timeline" {
    origin_x: i32,
    origin_y: i32,
    width: u32,
    events: Vec<Chip>,
});
record!(InteractionLaneSpec = "interaction_lane" {
    left_id: String,
    left_label: String,
    right_id: String,
    right_label: String,
    origin_x: i32,
    origin_y: i32,
    lane_width: u32,
    lifeline_length: u32,
    messages: Vec<Message>,
});

impl Region {
    fn standing_in(id: &str, label: &str, is_master: bool) -> Self {
        let (role, badge, suffix, sub) = if is_master {
            (Role::Master, "MASTER · R/W", "", "read + write")
        } else {
            (Role::Read, "READ", " (ro)", "read-only")
        };
        Self {
            id: id.into(),
            label: label.into(),
            sublabel: Some(sub.into()),
            role: RoleKey(role),
            badge_text: Some(badge.into()),
            services: (1..=4)
                .map(|i| Chip {
                    id: format!("svc-{i}-{id}"),
                    label: format!("Service {i}{suffix}"),
                    role: RoleKey(role),
                })
                .collect(),
        }
    }

    fn services(&self) -> Vec<ServiceArgs<'_>> {
        self.services
            .iter()
            .map(|s| ServiceArgs {
                id: &s.id,
                label: &s.label,
                role: s.role.0,
            })
            .collect()
    }

    fn pattern<'a>(&'a self, services: Vec<ServiceArgs<'a>>) -> RegionPatternArgs<'a> {
        RegionPatternArgs {
            id: &self.id,
            label: &self.label,
            sublabel: self.sublabel.as_deref(),
            role: self.role.0,
            badge_text: self.badge_text.as_deref(),
            services,
        }
    }
}

impl HaNode {
    fn args(&self) -> HaNodeArgs<'_> {
        HaNodeArgs {
            id: &self.id,
            label: &self.label,
            sublabel: self.sublabel.as_deref(),
        }
    }
}

impl Chip {
    fn tree_node(&self) -> TreeNodeArgs<'_> {
        TreeNodeArgs {
            id: &self.id,
            label: &self.label,
            role: self.role.0,
        }
    }
}

fn forms<I: IntoIterator<Item = tatara_lisp::Sexp>>(items: I) -> Vec<Form> {
    items.into_iter().map(Form).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Kata;

#[host(prefix = "kata_")]
impl Kata {
    fn multi_region(spec: MultiRegionSpec) -> Form {
        let primary = spec
            .primary
            .unwrap_or_else(|| Region::standing_in("primary", "Primary region", true));
        let replicas = spec.replicas.unwrap_or_else(|| {
            (0..spec.n_replicas.unwrap_or(1))
                .map(|r| Region::standing_in(&format!("replica-{r}"), &format!("Replica region {r}"), false))
                .collect()
        });
        let gateway = spec
            .gateway
            .map(|g| g.label)
            .or(spec.gateway_label);
        let d = multi_region(MultiRegionArgs {
            title: &spec.title,
            subtitle: spec.subtitle.as_deref(),
            primary: primary.pattern(primary.services()),
            replicas: replicas.iter().map(|r| r.pattern(r.services())).collect(),
            gateway: gateway.as_deref().map(|label| GatewayArgs { label }),
        });
        Form(d.to_sexp())
    }

    fn hero(spec: HeroSpec) -> Form {
        Form(
            hero(HeroArgs {
                title: &spec.title,
                subtitle: spec.subtitle.as_deref(),
            })
            .to_sexp(),
        )
    }

    fn closing(spec: ClosingSpec) -> Form {
        Form(
            closing(ClosingArgs {
                label: &spec.label,
                sublabel: spec.sublabel.as_deref(),
                role: spec.role.0,
            })
            .to_sexp(),
        )
    }

    fn pillars(spec: PillarsSpec) -> Vec<Form> {
        let chips: Vec<PillarArgs> = spec
            .pillars
            .iter()
            .map(|p| PillarArgs {
                id: &p.id,
                label: &p.label,
                role: p.role.0,
            })
            .collect();
        let nodes = pillars(PillarsArgs {
            origin: (spec.origin_x, spec.origin_y),
            width: spec.width,
            pillar_height: spec.pillar_height,
            gap: spec.gap,
            pillars: &chips,
        });
        forms(nodes.iter().map(|n| n.to_sexp()))
    }

    fn service_tier(spec: ServiceTierSpec) -> Vec<Form> {
        let chips: Vec<ServiceArgs> = spec
            .services
            .iter()
            .map(|s| ServiceArgs {
                id: &s.id,
                label: &s.label,
                role: s.role.0,
            })
            .collect();
        let nodes = service_tier(ServiceTierArgs {
            region_id: &spec.region_id,
            region_origin: (spec.region_origin_x, spec.region_origin_y),
            region_inner_top: spec.region_inner_top,
            services: &chips,
        });
        forms(nodes.iter().map(|n| n.to_sexp()))
    }

    fn architecture_layers(spec: ArchitectureLayersSpec) -> Vec<Form> {
        let chips: Vec<LayerArgs> = spec
            .layers
            .iter()
            .map(|l| LayerArgs {
                id: &l.id,
                label: &l.label,
                role: l.role.0,
            })
            .collect();
        let nodes = architecture_layers(ArchitectureLayersArgs {
            origin: (spec.origin_x, spec.origin_y),
            width: spec.width,
            layer_height: spec.layer_height,
            gap: spec.gap,
            layers: &chips,
        });
        forms(nodes.iter().map(|n| n.to_sexp()))
    }

    fn customer_gateway(spec: Gateway) -> Vec<Form> {
        let f = customer_gateway(GatewayArgs { label: &spec.label });
        forms([f.band.to_sexp(), f.arrow.to_sexp()])
    }

    fn zero_knowledge(spec: ZeroKnowledgeSpec) -> Vec<Form> {
        let f = zero_knowledge(ZeroKnowledgeArgs {
            client_id: &spec.client_id,
            client_label: &spec.client_label,
            service_id: &spec.service_id,
            service_label: &spec.service_label,
            client_at: (spec.client_x, spec.client_y),
            service_at: (spec.service_x, spec.service_y),
            payload_label: spec.payload_label.as_deref(),
        });
        forms([
            f.client.to_sexp(),
            f.service.to_sexp(),
            f.sealed_payload_edge.to_sexp(),
        ])
    }

    fn ha_failover(spec: HaFailoverSpec) -> Vec<Form> {
        let f = ha_failover(HaFailoverArgs {
            primary: spec.primary.args(),
            standby: spec.standby.args(),
            primary_at: (spec.primary_x, spec.primary_y),
            standby_at: (spec.standby_x, spec.standby_y),
            failover_label: spec.failover_label.as_deref(),
            recovery_label: spec.recovery_label.as_deref(),
        });
        forms([
            f.primary.to_sexp(),
            f.standby.to_sexp(),
            f.failover_edge.to_sexp(),
            f.recovery_edge.to_sexp(),
        ])
    }

    fn read_replica_pair(spec: ReadReplicaPairSpec) -> Vec<Form> {
        let replicas: Vec<ReadReplicaArgs> = spec
            .replicas
            .iter()
            .map(|r| ReadReplicaArgs {
                id: &r.id,
                label: &r.label,
            })
            .collect();
        let f = read_replica_pair(ReadReplicaPairArgs {
            master_id: &spec.master_id,
            master_label: &spec.master_label,
            master_at: (spec.master_x, spec.master_y),
            replicas: &replicas,
            replicas_y: spec.replicas_y,
            canvas_width: spec.canvas_width,
        });
        forms(
            std::iter::once(f.master.to_sexp())
                .chain(f.replicas.iter().map(|n| n.to_sexp()))
                .chain(f.sync_edges.iter().map(|e| e.to_sexp())),
        )
    }

    fn api_fanout(spec: ApiFanoutSpec) -> Vec<Form> {
        let backends: Vec<BackendArgs> = spec
            .backends
            .iter()
            .map(|b| BackendArgs {
                id: &b.id,
                label: &b.label,
                role: b.role.0,
            })
            .collect();
        let f = api_fanout(ApiFanoutArgs {
            api_id: &spec.api_id,
            api_label: &spec.api_label,
            api_at: (spec.api_x, spec.api_y),
            backends: &backends,
            backends_y: spec.backends_y,
            canvas_width: spec.canvas_width,
        });
        forms(
            std::iter::once(f.api_node.to_sexp())
                .chain(f.backends.iter().map(|n| n.to_sexp()))
                .chain(f.edges.iter().map(|e| e.to_sexp())),
        )
    }

    fn decision_branch(spec: DecisionBranchSpec) -> Vec<Form> {
        let branches: Vec<BranchArgs> = spec
            .branches
            .iter()
            .map(|b| BranchArgs {
                id: &b.id,
                label: &b.label,
                role: b.role.0,
                branch_label: &b.branch_label,
            })
            .collect();
        let f = decision_branch(DecisionBranchArgs {
            decision_id: &spec.decision_id,
            decision_label: &spec.decision_label,
            decision_at: (spec.decision_x, spec.decision_y),
            branches: &branches,
            branches_y: spec.branches_y,
            canvas_width: spec.canvas_width,
        });
        forms(
            std::iter::once(f.decision.to_sexp())
                .chain(f.branches.iter().map(|n| n.to_sexp()))
                .chain(f.edges.iter().map(|e| e.to_sexp())),
        )
    }

    fn tree_hierarchy(spec: TreeHierarchySpec) -> Vec<Form> {
        let branches: Vec<TreeNodeArgs> = spec.branches.iter().map(Chip::tree_node).collect();
        let f = tree_hierarchy(TreeHierarchyArgs {
            root: spec.root.tree_node(),
            root_at: (spec.root_x, spec.root_y),
            branches: &branches,
            branches_y: spec.branches_y,
            canvas_width: spec.canvas_width,
        });
        forms(
            std::iter::once(f.root.to_sexp())
                .chain(f.branches.iter().map(|n| n.to_sexp()))
                .chain(f.edges.iter().map(|e| e.to_sexp())),
        )
    }

    fn routing_detail(spec: RoutingDetailSpec) -> Vec<Form> {
        let stages: Vec<RoutingStageArgs> = spec
            .stages
            .iter()
            .map(|s| RoutingStageArgs {
                id: &s.id,
                label: &s.label,
                role: s.role.0,
                arrow_label: s.arrow_label.as_deref(),
            })
            .collect();
        let f = routing_detail(RoutingDetailArgs {
            origin: (spec.origin_x, spec.origin_y),
            stage_width: spec.stage_width,
            stage_height: spec.stage_height,
            gap: spec.gap,
            stages: &stages,
        });
        forms(
            f.stages
                .iter()
                .map(|n| n.to_sexp())
                .chain(f.edges.iter().map(|e| e.to_sexp())),
        )
    }

    fn data_tier(spec: DataTierSpec) -> Vec<Form> {
        let tiers: Vec<TierArgs> = spec
            .tiers
            .iter()
            .map(|t| TierArgs {
                id: &t.id,
                label: &t.label,
                role: t.role.0,
                arrow_label: t.arrow_label.as_deref(),
            })
            .collect();
        let f = data_tier(DataTierArgs {
            origin: (spec.origin_x, spec.origin_y),
            width: spec.width,
            tier_height: spec.tier_height,
            gap: spec.gap,
            tiers: &tiers,
        });
        forms(
            f.tiers
                .iter()
                .map(|n| n.to_sexp())
                .chain(f.edges.iter().map(|e| e.to_sexp())),
        )
    }

    fn event_timeline(spec: EventTimelineSpec) -> Vec<Form> {
        let events: Vec<EventArgs> = spec
            .events
            .iter()
            .map(|e| EventArgs {
                id: &e.id,
                label: &e.label,
                role: e.role.0,
            })
            .collect();
        let f = event_timeline(EventTimelineArgs {
            origin: (spec.origin_x, spec.origin_y),
            width: spec.width,
            events: &events,
        });
        forms(
            std::iter::once(f.baseline.to_sexp())
                .chain(f.event_chips.iter().map(|n| n.to_sexp())),
        )
    }

    fn interaction_lane(spec: InteractionLaneSpec) -> Vec<Form> {
        let messages: Vec<InteractionMessageArgs> = spec
            .messages
            .iter()
            .map(|m| InteractionMessageArgs {
                label: &m.label,
                from_left_to_right: m.from_left_to_right,
                role: m.role.0,
            })
            .collect();
        let f = interaction_lane(InteractionLaneArgs {
            left_id: &spec.left_id,
            left_label: &spec.left_label,
            right_id: &spec.right_id,
            right_label: &spec.right_label,
            origin: (spec.origin_x, spec.origin_y),
            lane_width: spec.lane_width,
            lifeline_length: spec.lifeline_length,
            messages: &messages,
        });
        forms(
            [
                f.left_actor.to_sexp(),
                f.right_actor.to_sexp(),
                f.left_lifeline.to_sexp(),
                f.right_lifeline.to_sexp(),
            ]
            .into_iter()
            .chain(f.messages.iter().map(|e| e.to_sexp())),
        )
    }
}
