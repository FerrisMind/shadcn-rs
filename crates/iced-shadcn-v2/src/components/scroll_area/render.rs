//! Viewport composition and widget construction for [`super::ScrollArea`].

use crate::iced_compat::Element;
use crate::iced_compat::widget::{container, scrollable};

use super::{ScrollArea, geometry, style};

/// Builds the underlying iced scrollable for a configured scroll area.
pub(super) fn build_scrollable<'a, Message: 'a>(
    area: ScrollArea<'a, Message>,
) -> scrollable::Scrollable<'a, Message> {
    let ScrollArea {
        content,
        theme,
        orientation,
        vertical,
        horizontal,
        width,
        height,
        padding,
        radius,
        thumb_radius,
        bordered,
        background,
        track_color,
        thumb_color,
        auto_scroll,
        id,
        on_scroll,
        style_override,
    } = area;

    let tokens = style::Tokens {
        theme,
        frame_radius: geometry::frame_radius_px(theme, radius),
        bordered,
        background,
        thumb_radius: geometry::thumb_radius_px(theme, thumb_radius),
        track_color,
        thumb_color,
    };

    let mut widget = scrollable::Scrollable::with_direction(
        build_viewport(content, padding),
        geometry::direction(orientation, vertical, horizontal),
    )
    .auto_scroll(auto_scroll);

    if let Some(width) = width {
        widget = widget.width(width);
    }

    if let Some(height) = height {
        widget = widget.height(height);
    }

    if let Some(id) = id {
        widget = widget.id(id);
    }

    if let Some(on_scroll) = on_scroll {
        widget = widget.on_scroll(on_scroll);
    }

    widget.style(move |_iced_theme, status| {
        let mut style = style::resolve_scroll_area_style(tokens, status);

        if let Some(override_fn) = style_override.as_ref() {
            style = override_fn(style, status);
        }

        style
    })
}

/// Insets the content the way the padded root of the reference component does.
///
/// The inset lives inside the scrolled content rather than on the frame, so the
/// rails keep hugging the frame edge instead of floating in the padding box.
fn build_viewport<'a, Message: 'a>(
    content: Element<'a, Message>,
    padding: Option<crate::iced_compat::Padding>,
) -> Element<'a, Message> {
    match padding {
        Some(padding) => container(content).padding(padding).into(),
        None => content,
    }
}
