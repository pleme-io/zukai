//! `kata::customer_gateway` — outbound-only client gateway band at the bottom
//! of a diagram + a dashed arrow upward to the global routing band. The
//! canonical "outbound HTTPS only, no inbound firewall changes" pattern.

use zukai_model::{Band, Edge, Marker, Point, Rect, Role, Stroke};

/// Args for [`customer_gateway`].
#[derive(Debug, Clone, Copy)]
pub struct GatewayArgs<'a> {
    pub label: &'a str,
}

/// Fragment returned by [`customer_gateway`] — caller composes the parts into
/// the surrounding [`Diagram`](zukai_model::Diagram).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayFragment {
    pub band: Band,
    pub arrow: Edge,
}

/// Build the canonical client-gateway band + outbound arrow.
#[must_use]
pub fn customer_gateway(args: GatewayArgs<'_>) -> GatewayFragment {
    GatewayFragment {
        band: Band {
            label: args.label.into(),
            sublabel: None,
            role: Role::Neutral,
            at: Rect { x: 300, y: 486, w: 360, h: 48 },
        },
        arrow: Edge {
            from: Point { x: 480, y: 486 },
            to: Point { x: 480, y: 146 },
            stroke: Stroke::Dashed,
            marker: Marker::Arrow,
            role: Role::Accent,
            label: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_band_at_bottom_with_label() {
        let g = customer_gateway(GatewayArgs { label: "Client gateway" });
        assert_eq!(g.band.label, "Client gateway");
        assert_eq!(g.band.role, Role::Neutral);
        assert_eq!(g.band.at.y, 486);
    }

    #[test]
    fn arrow_is_dashed_and_points_up() {
        let g = customer_gateway(GatewayArgs { label: "x" });
        assert_eq!(g.arrow.stroke, Stroke::Dashed);
        assert_eq!(g.arrow.marker, Marker::Arrow);
        // From the gateway band to the routing band (lower y is higher on screen).
        assert!(g.arrow.from.y > g.arrow.to.y);
    }

    #[test]
    fn is_deterministic() {
        let a = customer_gateway(GatewayArgs { label: "g" });
        let b = customer_gateway(GatewayArgs { label: "g" });
        assert_eq!(a, b);
    }
}
