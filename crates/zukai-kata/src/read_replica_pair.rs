//! `kata::read_replica_pair` — master + N read replicas with dashed sync
//! arrows themed `Read`. Distinct from [`ha_failover`](crate::ha_failover)
//! (which is 1:1 + bidirectional state transitions). Use for read-scale
//! architectures where data flows master → many readers.

use zukai_model::{Edge, Id, Marker, Node, Point, Rect, Role, Stroke};

const MASTER_W: u32 = 180;
const MASTER_H: u32 = 60;
const REPLICA_W: u32 = 140;
const REPLICA_H: u32 = 50;
const REPLICA_GAP: u32 = 20;

/// One read replica.
#[derive(Debug, Clone, Copy)]
pub struct ReadReplicaArgs<'a> {
    pub id: &'a str,
    pub label: &'a str,
}

/// Args for [`read_replica_pair`].
pub struct ReadReplicaPairArgs<'a> {
    pub master_id: &'a str,
    pub master_label: &'a str,
    /// Top-left of the master node.
    pub master_at: (i32, i32),
    pub replicas: &'a [ReadReplicaArgs<'a>],
    /// Y position of the replicas row.
    pub replicas_y: i32,
    /// Total horizontal extent the replicas are centered within.
    pub canvas_width: u32,
}

/// Fragment returned by [`read_replica_pair`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadReplicaPairFragment {
    /// Master node themed `Role::Master`.
    pub master: Node,
    /// Replica nodes themed `Role::Read`.
    pub replicas: Vec<Node>,
    /// One sync arrow per replica: master → replica, dashed, `Role::Read`.
    pub sync_edges: Vec<Edge>,
}

/// Build a master + N read replicas + dashed sync arrows master → replicas.
#[must_use]
pub fn read_replica_pair(args: ReadReplicaPairArgs<'_>) -> ReadReplicaPairFragment {
    let master = Node {
        id: Id::new(args.master_id),
        label: args.master_label.into(),
        role: Role::Master,
        region: None,
        at: Rect { x: args.master_at.0, y: args.master_at.1, w: MASTER_W, h: MASTER_H },
    };

    let n = args.replicas.len() as u32;
    let replicas: Vec<Node> = if n == 0 {
        Vec::new()
    } else {
        let total_gap = REPLICA_GAP.saturating_mul(n.saturating_sub(1));
        let total_used = n * REPLICA_W + total_gap;
        let origin_x = ((args.canvas_width as i32 - total_used as i32) / 2).max(0);
        args.replicas
            .iter()
            .enumerate()
            .map(|(i, r)| Node {
                id: Id::new(r.id),
                label: r.label.into(),
                role: Role::Read,
                region: None,
                at: Rect {
                    x: origin_x + (i as i32) * (REPLICA_W as i32 + REPLICA_GAP as i32),
                    y: args.replicas_y,
                    w: REPLICA_W,
                    h: REPLICA_H,
                },
            })
            .collect()
    };

    let master_anchor = Point {
        x: args.master_at.0 + (MASTER_W as i32) / 2,
        y: args.master_at.1 + MASTER_H as i32,
    };
    let sync_edges: Vec<Edge> = replicas
        .iter()
        .map(|r| Edge {
            from: master_anchor,
            to: Point { x: r.at.x + (r.at.w as i32) / 2, y: r.at.y },
            stroke: Stroke::Dashed,
            marker: Marker::Arrow,
            role: Role::Read,
            label: None,
        })
        .collect();

    ReadReplicaPairFragment { master, replicas, sync_edges }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn three_replicas() -> [ReadReplicaArgs<'static>; 3] {
        [
            ReadReplicaArgs { id: "r1", label: "Replica 1" },
            ReadReplicaArgs { id: "r2", label: "Replica 2" },
            ReadReplicaArgs { id: "r3", label: "Replica 3" },
        ]
    }

    fn sample<'a>(replicas: &'a [ReadReplicaArgs<'a>]) -> ReadReplicaPairArgs<'a> {
        ReadReplicaPairArgs {
            master_id: "m",
            master_label: "Master",
            master_at: (380, 120),
            replicas,
            replicas_y: 280,
            canvas_width: 960,
        }
    }

    #[test]
    fn has_master_and_n_replicas_and_n_sync_edges() {
        let rs = three_replicas();
        let f = read_replica_pair(sample(&rs));
        assert_eq!(f.master.role, Role::Master);
        assert_eq!(f.replicas.len(), 3);
        assert_eq!(f.sync_edges.len(), 3);
    }

    #[test]
    fn replicas_all_themed_read() {
        let rs = three_replicas();
        let f = read_replica_pair(sample(&rs));
        for r in &f.replicas {
            assert_eq!(r.role, Role::Read);
        }
    }

    #[test]
    fn sync_edges_are_dashed_and_themed_read() {
        let rs = three_replicas();
        let f = read_replica_pair(sample(&rs));
        for e in &f.sync_edges {
            assert_eq!(e.stroke, Stroke::Dashed);
            assert_eq!(e.role, Role::Read);
        }
    }

    #[test]
    fn is_deterministic() {
        let rs = three_replicas();
        let a = read_replica_pair(sample(&rs));
        let b = read_replica_pair(sample(&rs));
        assert_eq!(a, b);
    }

    #[test]
    fn empty_replicas_returns_no_sync_edges() {
        let f = read_replica_pair(ReadReplicaPairArgs {
            master_id: "m",
            master_label: "Master",
            master_at: (0, 0),
            replicas: &[],
            replicas_y: 200,
            canvas_width: 1000,
        });
        assert!(f.replicas.is_empty());
        assert!(f.sync_edges.is_empty());
    }
}
