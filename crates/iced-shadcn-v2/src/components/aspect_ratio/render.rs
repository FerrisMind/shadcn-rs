//! Custom layout widget and rendering for [`super::AspectRatio`].

use crate::iced_compat::advanced::layout::{self, Layout};
use crate::iced_compat::advanced::widget::{Operation, Tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, overlay, renderer};
use crate::iced_compat::widget::{container, container as container_widget};
use crate::iced_compat::{
    Background, Border, Element, Event, Length, Rectangle, Size, Vector, mouse,
};

use super::types::{AspectRatio, MIN_ASPECT_RATIO};

/// Wraps an [`AspectRatio`] configuration into a layout-preserving widget.
pub fn aspect_ratio<'a, Message: 'a>(config: AspectRatio<'a, Message>) -> Element<'a, Message> {
    let ratio = config.resolved_ratio();
    let inner = Element::new(AspectRatioWidget {
        content: config.content,
        ratio,
    });

    if config.background.is_none()
        && config.style_override.is_none()
        && config.radius <= 0.0
        && !config.clip
    {
        return inner;
    }

    let background = config.background;
    let radius = config.radius;
    let style_override = config.style_override;

    container_widget(inner)
        .width(Length::Fill)
        .clip(config.clip)
        .style(move |_iced_theme| {
            let mut resolved = container::Style {
                background: background.map(Background::Color),
                border: Border {
                    radius: radius.into(),
                    ..Border::default()
                },
                ..container::Style::default()
            };

            if let Some(override_fn) = style_override.as_ref() {
                resolved = override_fn(resolved);
            }

            resolved
        })
        .into()
}

struct AspectRatioWidget<'a, Message> {
    content: Element<'a, Message>,
    ratio: f32,
}

impl<Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for AspectRatioWidget<'_, Message>
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &crate::iced_compat::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let bounds = resolve_bounds(self.ratio, limits);
        let fixed = layout::Limits::new(bounds, bounds);
        let child = self
            .content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, &fixed);

        layout::Node::with_children(bounds, vec![child])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &crate::iced_compat::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            layout.children().next().expect("aspect-ratio child layout"),
            renderer,
            operation,
        );
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &crate::iced_compat::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.children().next().expect("aspect-ratio child layout"),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &crate::iced_compat::Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout.children().next().expect("aspect-ratio child layout"),
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut crate::iced_compat::Renderer,
        theme: &crate::iced_compat::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout.children().next().expect("aspect-ratio child layout"),
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &crate::iced_compat::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<
        overlay::Element<'b, Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>,
    > {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().expect("aspect-ratio child layout"),
            renderer,
            viewport,
            translation,
        )
    }
}

/// Computes the largest axis-aligned box that fits `limits` while honoring `ratio`.
pub(super) fn resolve_bounds(ratio: f32, limits: &layout::Limits) -> Size {
    let ratio = if ratio.is_finite() && ratio > MIN_ASPECT_RATIO {
        ratio
    } else {
        MIN_ASPECT_RATIO
    };

    let max = limits.max();
    let min = limits.min();

    let mut width = max.width;
    let mut height = width / ratio;

    if height > max.height {
        height = max.height;
        width = height * ratio;
    }

    Size::new(
        width.clamp(min.width, max.width.max(min.width)),
        height.clamp(min.height, max.height.max(min.height)),
    )
}
