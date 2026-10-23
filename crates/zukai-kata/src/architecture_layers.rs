//! `kata::architecture_layers` — N labeled layers stacked vertically.
//! Each layer is a wide chip; **no arrows** between layers (the visual
//! distinction from [`data_tier`](crate::data_tier), which shows downward
//! data flow). Use for "responsibility stack" diagrams (presentation /
//! business / data, or controller / domain / infrastructure).

use zukai_model::{Id, Node, Rect, Role};

/// One architectural layer.
#[derive(Debug, Clone, Copy)]
pub struct LayerArgs<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub role: Role,
}

/// Args for [`architecture_layers`].
pub struct ArchitectureLayersArgs<'a> {
    /// Top-left of the topmost layer.
    pub origin: (i32, i32),
    pub width: u32,
    pub layer_height: u32,
    pub gap: u32,
    pub layers: &'a [LayerArgs<'a>],
}

/// Build N wide nodes stacked top-to-bottom with no connecting arrows.
#[must_use]
pub fn architecture_layers(args: ArchitectureLayersArgs<'_>) -> Vec<Node> {
    let (origin_x, origin_y) = args.origin;
    args.layers
        .iter()
        .enumerate()
        .map(|(i, l)| Node {
            id: Id::new(l.id),
            label: l.label.into(),
            role: l.role,
            region: None,
            at: Rect {
                x: origin_x,
                y: origin_y + (i as i32) * (args.layer_height as i32 + args.gap as i32),
                w: args.width,
                h: args.layer_height,
            },
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn three_layers() -> [LayerArgs<'static>; 3] {
        [
            LayerArgs { id: "presentation", label: "Presentation", role: Role::Accent },
            LayerArgs { id: "domain",       label: "Domain",       role: Role::Master },
            LayerArgs { id: "infra",        label: "Infrastructure", role: Role::Neutral },
        ]
    }

    #[test]
    fn stacks_layers_vertically_with_gaps() {
        let ls = three_layers();
        let nodes = architecture_layers(ArchitectureLayersArgs {
            origin: (60, 120),
            width: 840,
            layer_height: 80,
            gap: 24,
            layers: &ls,
        });
        assert_eq!(nodes.len(), 3);
        for (i, n) in nodes.iter().enumerate() {
            assert_eq!(n.at.x, 60);
            assert_eq!(n.at.y, 120 + (i as i32) * (80 + 24));
            assert_eq!(n.at.w, 840);
            assert_eq!(n.at.h, 80);
        }
    }

    #[test]
    fn carries_per_layer_role() {
        let ls = three_layers();
        let nodes = architecture_layers(ArchitectureLayersArgs {
            origin: (0, 0),
            width: 800,
            layer_height: 60,
            gap: 10,
            layers: &ls,
        });
        assert_eq!(nodes[0].role, Role::Accent);
        assert_eq!(nodes[1].role, Role::Master);
        assert_eq!(nodes[2].role, Role::Neutral);
    }

    #[test]
    fn empty_layers_returns_empty_vec() {
        let nodes = architecture_layers(ArchitectureLayersArgs {
            origin: (0, 0),
            width: 800,
            layer_height: 60,
            gap: 10,
            layers: &[],
        });
        assert!(nodes.is_empty());
    }

    #[test]
    fn is_deterministic() {
        let ls = three_layers();
        let args = || ArchitectureLayersArgs {
            origin: (60, 120),
            width: 840,
            layer_height: 80,
            gap: 24,
            layers: &ls,
        };
        assert_eq!(architecture_layers(args()), architecture_layers(args()));
    }
}
