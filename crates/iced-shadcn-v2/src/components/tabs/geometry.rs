//! Style-pack geometry and numeric input normalization for tabs.

use crate::components::button::ButtonSize;
use crate::iced_compat::{Length, Padding};
use crate::theme::Theme;
use shadcn_common::{ControlSize, StyleId};

use super::{TabsListVariant, TabsOrientation, TabsSize};

/// Resolved dimensions shared by the list layout and trigger styling.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct TabsMetrics {
    pub(super) list_padding: f32,
    pub(super) vertical_list_padding: f32,
    pub(super) gap: f32,
    pub(super) line_gap: f32,
    pub(super) trigger_height: f32,
    pub(super) trigger_pad_x: f32,
    pub(super) trigger_pad_y: f32,
    pub(super) vertical_trigger_pad_x: f32,
    pub(super) vertical_trigger_pad_y: f32,
    pub(super) list_radius: f32,
    pub(super) trigger_radius: f32,
    pub(super) content_text_size: f32,
    pub(super) content_line_height: f32,
    pub(super) active_shadow: bool,
}

impl TabsMetrics {
    pub(super) fn list_padding_for(self, orientation: TabsOrientation) -> f32 {
        if orientation.is_vertical() {
            self.vertical_list_padding
        } else {
            self.list_padding
        }
    }

    pub(super) fn trigger_padding_for(self, orientation: TabsOrientation) -> (f32, f32) {
        if orientation.is_vertical() {
            (self.vertical_trigger_pad_x, self.vertical_trigger_pad_y)
        } else {
            (self.trigger_pad_x, self.trigger_pad_y)
        }
    }
}

/// Converts the public tabs size into the existing button size ladder.
pub(super) const fn button_size(size: TabsSize) -> ButtonSize {
    match size {
        TabsSize::Sm => ButtonSize::Sm,
        TabsSize::Default => ButtonSize::Default,
        TabsSize::Lg => ButtonSize::Lg,
    }
}

/// Resolves tabs dimensions from the same style-pack tokens as buttons.
pub(super) fn resolve_metrics(
    theme: &Theme,
    size: TabsSize,
    orientation: TabsOrientation,
    variant: TabsListVariant,
) -> TabsMetrics {
    let control_size = match size {
        TabsSize::Sm => ControlSize::Sm,
        TabsSize::Default => ControlSize::Md,
        TabsSize::Lg => ControlSize::Lg,
    };
    let list_height = theme.style.toggle_size(control_size).height_px;
    let style = theme.style.id;

    let list_padding = match style {
        StyleId::Sera | StyleId::Luma => 4.0,
        _ => 3.0,
    };
    let vertical_list_padding = match style {
        StyleId::Luma | StyleId::Rhea => 4.0,
        _ => list_padding,
    };

    let (
        trigger_pad_x,
        trigger_pad_y,
        vertical_trigger_pad_x,
        vertical_trigger_pad_y,
        list_radius,
        trigger_radius,
        content_text_size,
        content_line_height,
        trigger_line_height,
    ) = match style {
        StyleId::Vega => (
            scale_size(8.0, size),
            scale_size(4.0, size),
            scale_size(8.0, size),
            scale_size(4.0, size),
            theme.style.twill_radius_md.px_value(),
            theme.style.twill_radius_sm.px_value(),
            14.0,
            20.0,
            20.0,
        ),
        StyleId::Nova => (
            scale_size(6.0, size),
            scale_size(2.0, size),
            scale_size(6.0, size),
            scale_size(2.0, size),
            theme.style.twill_radius_md.px_value(),
            theme.style.twill_radius_sm.px_value(),
            14.0,
            20.0,
            20.0,
        ),
        StyleId::Maia => (
            scale_size(8.0, size),
            scale_size(4.0, size),
            scale_size(10.0, size),
            scale_size(6.0, size),
            theme.style.twill_radius_lg.px_value(),
            theme.style.twill_radius_md.px_value(),
            14.0,
            20.0,
            20.0,
        ),
        StyleId::Lyra => (
            scale_size(6.0, size),
            scale_size(2.0, size),
            scale_size(6.0, size),
            scale_size(5.0, size),
            0.0,
            0.0,
            12.0,
            19.5,
            16.0,
        ),
        StyleId::Mira => (
            scale_size(6.0, size),
            scale_size(2.0, size),
            scale_size(6.0, size),
            scale_size(5.0, size),
            theme.style.twill_radius_md.px_value(),
            theme.style.twill_radius_sm.px_value(),
            12.0,
            19.5,
            16.0,
        ),
        StyleId::Luma => (
            scale_size(12.0, size),
            scale_size(4.0, size),
            scale_size(12.0, size),
            scale_size(6.0, size),
            9999.0,
            9999.0,
            14.0,
            20.0,
            20.0,
        ),
        StyleId::Sera => (
            scale_size(16.0, size),
            scale_size(6.0, size),
            scale_size(16.0, size),
            scale_size(8.0, size),
            0.0,
            0.0,
            14.0,
            20.0,
            16.0,
        ),
        StyleId::Rhea => (
            scale_size(6.0, size),
            scale_size(2.0, size),
            scale_size(12.0, size),
            scale_size(2.0, size),
            theme.style.twill_radius_lg.px_value(),
            theme.style.twill_radius_lg.px_value(),
            14.0,
            20.0,
            20.0,
        ),
    };

    let horizontal_trigger_height = (list_height - 2.0 * list_padding - 1.0).max(1.0);
    let vertical_trigger_height =
        (trigger_line_height + 2.0 * vertical_trigger_pad_y + 2.0).max(1.0);
    let trigger_height = if orientation.is_vertical() {
        vertical_trigger_height
    } else {
        horizontal_trigger_height
    };

    TabsMetrics {
        list_padding,
        vertical_list_padding,
        gap: if variant == TabsListVariant::Line {
            4.0
        } else {
            0.0
        },
        line_gap: 4.0,
        trigger_height,
        trigger_pad_x,
        trigger_pad_y,
        vertical_trigger_pad_x,
        vertical_trigger_pad_y,
        list_radius,
        trigger_radius,
        content_text_size,
        content_line_height,
        active_shadow: matches!(style, StyleId::Vega | StyleId::Nova)
            && variant == TabsListVariant::Default,
    }
}

fn scale_size(value: f32, size: TabsSize) -> f32 {
    match size {
        TabsSize::Sm => (value - 1.0).max(0.0),
        TabsSize::Default => value,
        TabsSize::Lg => value + 2.0,
    }
}

pub(super) fn normalize_px(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

pub(super) fn normalize_padding(padding: Padding) -> Padding {
    Padding {
        top: normalize_px(padding.top),
        right: normalize_px(padding.right),
        bottom: normalize_px(padding.bottom),
        left: normalize_px(padding.left),
    }
}

pub(super) fn resolve_length(length: Length, natural: f32, min: f32, max: f32) -> f32 {
    let max = max.max(min);
    match length {
        Length::Fixed(value) => normalize_px(value).clamp(min, max),
        Length::Fill | Length::FillPortion(_) => max,
        Length::Shrink => natural.clamp(min, max),
    }
}
