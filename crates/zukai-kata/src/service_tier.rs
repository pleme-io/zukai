//! `kata::service_tier` — a grid (3 cols by default) of role-colored service
//! chips inside a region. Each chip carries a `Role` that drives its color via
//! the active `Theme`.

use zukai_model::{Id, Node, Rect, Role};

const CHIP_W: u32 = 120;
const CHIP_H: u32 = 34;
const COLS: usize = 3;
const COL_GAP: i32 = 10;
const ROW_GAP: i32 = 10;
const REGION_INNER_PAD_X: i32 = 20;

/// One service chip definition.
#[derive(Debug, Clone, Copy)]
pub struct ServiceArgs<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub role: Role,
}

/// Args for [`service_tier`].
pub struct ServiceTierArgs<'a> {
    /// Id of the containing region (chips carry this as their `region`).
    pub region_id: &'a str,
    /// Top-left of the containing region in canvas units.
    pub region_origin: (i32, i32),
    /// Y offset (within the region) for the first chip row — accounts for the
    /// region's header band.
    pub region_inner_top: i32,
    pub services: &'a [ServiceArgs<'a>],
}

/// Lay out service chips in a fixed grid (3 cols, fixed chip size).
#[must_use]
pub fn service_tier(args: ServiceTierArgs<'_>) -> Vec<Node> {
    let (origin_x, origin_y) = args.region_origin;
    let inner_x = origin_x + REGION_INNER_PAD_X;
    let inner_y = origin_y + args.region_inner_top;

    args.services
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let col = (i % COLS) as i32;
            let row = (i / COLS) as i32;
            let x = inner_x + col * (CHIP_W as i32 + COL_GAP);
            let y = inner_y + row * (CHIP_H as i32 + ROW_GAP);
            Node {
                id: Id::new(s.id),
                label: s.label.into(),
                role: s.role,
                region: Some(Id::new(args.region_id)),
                at: Rect { x, y, w: CHIP_W, h: CHIP_H },
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lays_out_chips_in_three_column_grid() {
        let services = [
            ServiceArgs { id: "a", label: "A", role: Role::Master },
            ServiceArgs { id: "b", label: "B", role: Role::Master },
            ServiceArgs { id: "c", label: "C", role: Role::Master },
            ServiceArgs { id: "d", label: "D", role: Role::Master },
        ];
        let tier = service_tier(ServiceTierArgs {
            region_id: "r1",
            region_origin: (40, 200),
            region_inner_top: 62,
            services: &services,
        });
        assert_eq!(tier.len(), 4);
        // Row 0: cols 0/1/2, all at y = 200+62 = 262.
        assert_eq!(tier[0].at.x, 60); // 40 + 20
        assert_eq!(tier[0].at.y, 262);
        assert_eq!(tier[1].at.x, 60 + (CHIP_W as i32 + COL_GAP));
        assert_eq!(tier[2].at.x, 60 + 2 * (CHIP_W as i32 + COL_GAP));
        // Row 1: col 0, y advances by chip height + row gap.
        assert_eq!(tier[3].at.x, 60);
        assert_eq!(tier[3].at.y, 262 + CHIP_H as i32 + ROW_GAP);
    }

    #[test]
    fn each_chip_carries_its_region_and_role() {
        let services = [ServiceArgs { id: "x", label: "X", role: Role::Read }];
        let tier = service_tier(ServiceTierArgs {
            region_id: "ap",
            region_origin: (500, 200),
            region_inner_top: 62,
            services: &services,
        });
        assert_eq!(tier[0].region.as_ref().map(|i| i.as_str()), Some("ap"));
        assert_eq!(tier[0].role, Role::Read);
        assert_eq!(tier[0].label, "X");
    }

    #[test]
    fn is_deterministic() {
        let services = [
            ServiceArgs { id: "a", label: "A", role: Role::Master },
            ServiceArgs { id: "b", label: "B", role: Role::Master },
        ];
        let args = || ServiceTierArgs {
            region_id: "r",
            region_origin: (40, 200),
            region_inner_top: 62,
            services: &services,
        };
        assert_eq!(service_tier(args()), service_tier(args()));
    }
}
