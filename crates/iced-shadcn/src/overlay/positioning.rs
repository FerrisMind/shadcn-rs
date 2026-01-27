use iced::{Point, Size};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OverlaySide {
    Above,
    Below,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OverlayPlacement {
    pub position: Point,
    pub side: OverlaySide,
}

pub fn place_overlay_centered(
    anchor_position: Point,
    target_size: Size,
    overlay_size: Size,
    window_size: Size,
    gap: f32,
) -> OverlayPlacement {
    let space_below = window_size.height - (anchor_position.y + target_size.height);
    let space_above = anchor_position.y;

    let aligned_x = anchor_position.x + (target_size.width - overlay_size.width) / 2.0;
    let clamped_x = aligned_x.clamp(0.0, (window_size.width - overlay_size.width).max(0.0));

    if space_below > space_above {
        OverlayPlacement {
            position: Point::new(clamped_x, anchor_position.y + target_size.height + gap),
            side: OverlaySide::Below,
        }
    } else {
        OverlayPlacement {
            position: Point::new(clamped_x, anchor_position.y - overlay_size.height - gap),
            side: OverlaySide::Above,
        }
    }
}
