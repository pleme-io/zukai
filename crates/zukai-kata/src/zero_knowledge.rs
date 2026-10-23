//! `kata::zero_knowledge` — client-sealed flow pattern. Client holds keys
//! locally; service operates on ciphertext only. The arrow between them
//! carries the encrypted payload and is themed `Accent` to signal "sealed".

use zukai_model::{Edge, Id, Marker, Node, Point, Rect, Role, Stroke};

const NODE_W: u32 = 200;
const NODE_H: u32 = 80;

/// Args for [`zero_knowledge`].
pub struct ZeroKnowledgeArgs<'a> {
    pub client_id: &'a str,
    pub client_label: &'a str,
    pub service_id: &'a str,
    pub service_label: &'a str,
    pub client_at: (i32, i32),
    pub service_at: (i32, i32),
    /// Label for the sealed payload arrow (e.g., "ciphertext only").
    pub payload_label: Option<&'a str>,
}

/// Fragment returned by [`zero_knowledge`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZeroKnowledgeFragment {
    /// Client side — holds the keys; themed `Role::Accent`.
    pub client: Node,
    /// Service side — sees ciphertext only; themed `Role::Read`.
    pub service: Node,
    /// Solid Accent arrow client → service; the sealed payload.
    pub sealed_payload_edge: Edge,
}

/// Build a client + service + a single labeled "sealed payload" arrow.
#[must_use]
pub fn zero_knowledge(args: ZeroKnowledgeArgs<'_>) -> ZeroKnowledgeFragment {
    let client = Node {
        id: Id::new(args.client_id),
        label: args.client_label.into(),
        role: Role::Accent,
        region: None,
        at: Rect { x: args.client_at.0, y: args.client_at.1, w: NODE_W, h: NODE_H },
    };
    let service = Node {
        id: Id::new(args.service_id),
        label: args.service_label.into(),
        role: Role::Read,
        region: None,
        at: Rect { x: args.service_at.0, y: args.service_at.1, w: NODE_W, h: NODE_H },
    };
    let sealed_payload_edge = Edge {
        from: Point {
            x: client.at.x + client.at.w as i32,
            y: client.at.y + (NODE_H as i32) / 2,
        },
        to: Point {
            x: service.at.x,
            y: service.at.y + (NODE_H as i32) / 2,
        },
        stroke: Stroke::Solid,
        marker: Marker::Arrow,
        role: Role::Accent,
        label: args.payload_label.map(String::from),
    };
    ZeroKnowledgeFragment { client, service, sealed_payload_edge }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ZeroKnowledgeArgs<'static> {
        ZeroKnowledgeArgs {
            client_id: "client",
            client_label: "Client (holds keys)",
            service_id: "service",
            service_label: "Service (ciphertext only)",
            client_at: (60, 240),
            service_at: (720, 240),
            payload_label: Some("ciphertext only"),
        }
    }

    #[test]
    fn client_is_accent_service_is_read() {
        let f = zero_knowledge(sample());
        assert_eq!(f.client.role, Role::Accent);
        assert_eq!(f.service.role, Role::Read);
    }

    #[test]
    fn payload_edge_flows_client_to_service_and_carries_label() {
        let f = zero_knowledge(sample());
        assert!(f.sealed_payload_edge.from.x < f.sealed_payload_edge.to.x);
        assert_eq!(f.sealed_payload_edge.role, Role::Accent);
        assert_eq!(f.sealed_payload_edge.stroke, Stroke::Solid);
        assert_eq!(
            f.sealed_payload_edge.label.as_deref(),
            Some("ciphertext only")
        );
    }

    #[test]
    fn payload_label_is_optional() {
        let mut args = sample();
        args.payload_label = None;
        let f = zero_knowledge(args);
        assert!(f.sealed_payload_edge.label.is_none());
    }

    #[test]
    fn is_deterministic() {
        assert_eq!(zero_knowledge(sample()), zero_knowledge(sample()));
    }
}
