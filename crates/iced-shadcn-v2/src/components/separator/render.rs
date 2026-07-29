//! Widget construction for [`super::Separator`].

use crate::iced_compat::widget::container::{self, Container};
use crate::iced_compat::widget::{Space, container as container_widget};
use crate::iced_compat::{Border, Element};

use super::types::Separator;

/// Wraps a [`Separator`] into a styled iced container widget.
///
/// Returned as a concrete [`Container`] so callers can keep tweaking the
/// widget (e.g. padding) before turning it into an [`Element`](iced_core::Element).
pub fn separator<'a, Message: 'a>(separator: Separator) -> Container<'a, Message> {
    let (width, height) = separator.resolved_axes();
    let color = separator.color;
    let radius = separator.radius;

    container_widget(Space::new())
        .width(width)
        .height(height)
        .style(move |_iced_theme| container::Style {
            background: Some(color.into()),
            border: Border {
                radius: radius.into(),
                ..Border::default()
            },
            ..container::Style::default()
        })
}

impl<'a, Message: 'a> From<Separator> for Element<'a, Message> {
    fn from(config: Separator) -> Self {
        separator(config).into()
    }
}
