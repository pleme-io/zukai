//! zukai-kata (型) — named diagram patterns. **The re-use space** where the
//! structural primitives of `zukai-model` compose into idioms.
//!
//! Pick a pattern by intent (`multi_region`, `customer_gateway`, …); the
//! pattern handles layout + composition; the resulting `Diagram` renders
//! through the same `Theme` seam as any other. This is the layer the
//! `diagram-authoring` skill reaches for given a user's authoring intent.
//!
//! Naming follows `theory/THEORY.md` §II.3 — discipline layer is Japanese:
//! **kata (型, "established form")**. The pattern glossary + when-to-use lives
//! in `zukai/docs/VOCABULARY.md`.

#![forbid(unsafe_code)]

pub mod host;
pub mod record;

pub mod api_fanout;
pub mod architecture_layers;
pub mod closing;
pub mod customer_gateway;
pub mod data_tier;
pub mod decision_branch;
pub mod event_timeline;
pub mod from_mermaid;
pub mod ha_failover;
pub mod hero;
pub mod interaction_lane;
pub mod multi_region;
pub mod pillars;
pub mod read_replica_pair;
pub mod routing_detail;
pub mod service_tier;
pub mod tree_hierarchy;
pub mod zero_knowledge;

pub use api_fanout::{api_fanout, ApiFanoutArgs, ApiFanoutFragment, BackendArgs};
pub use architecture_layers::{architecture_layers, ArchitectureLayersArgs, LayerArgs};
pub use closing::{closing, ClosingArgs};
pub use customer_gateway::{customer_gateway, GatewayArgs, GatewayFragment};
pub use data_tier::{data_tier, DataTierArgs, DataTierFragment, TierArgs};
pub use decision_branch::{
    decision_branch, BranchArgs, DecisionBranchArgs, DecisionBranchFragment,
};
pub use event_timeline::{event_timeline, EventArgs, EventTimelineArgs, EventTimelineFragment};
pub use from_mermaid::{from_mermaid, FromMermaidError};
pub use ha_failover::{ha_failover, HaFailoverArgs, HaFailoverFragment, HaNodeArgs};
pub use hero::{hero, HeroArgs};
pub use interaction_lane::{
    interaction_lane, InteractionLaneArgs, InteractionLaneFragment, InteractionMessageArgs,
};
pub use multi_region::{multi_region, MultiRegionArgs, RegionPatternArgs};
pub use pillars::{pillars, PillarArgs, PillarsArgs};
pub use read_replica_pair::{
    read_replica_pair, ReadReplicaArgs, ReadReplicaPairArgs, ReadReplicaPairFragment,
};
pub use routing_detail::{
    routing_detail, RoutingDetailArgs, RoutingDetailFragment, RoutingStageArgs,
};
pub use service_tier::{service_tier, ServiceArgs, ServiceTierArgs};
pub use tree_hierarchy::{tree_hierarchy, TreeHierarchyArgs, TreeHierarchyFragment, TreeNodeArgs};
pub use zero_knowledge::{zero_knowledge, ZeroKnowledgeArgs, ZeroKnowledgeFragment};
