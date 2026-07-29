//! Minimum-width layout wrapper backing the `min-w-5` rule of `cn-kbd`.
//!
//! iced containers have `max_width` but no `min_width`, and `Stack` clamps
//! layers to its base layer, so neither can express "at least N px wide,
//! grow with content". This small pass-through widget can.

use crate::iced_compat::advanced::layout::{self, Layout};
use crate::iced_compat::advanced::widget::{Operation, Tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, overlay, renderer};
use crate::iced_compat::{Element, Event, Length, Point, Rectangle, Size, Vector, mouse};

/// Wraps `content` so it is laid out at least `min_width` px wide.
///
/// Narrower content is centered horizontally inside the widened bounds —
/// matching the `inline-flex justify-center` + `min-w-*` web behavior.
pub(super) fn min_width<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    min_width: f32,
) -> Element<'a, Message> {
    let content = content.into();

    if min_width <= 0.0 {
        return content;
    }

    Element::new(MinWidth { content, min_width })
}

struct MinWidth<'a, Message> {
    content: Element<'a, Message>,
    min_width: f32,
}

impl<Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for MinWidth<'_, Message>
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: self.content.as_widget().size().height,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &crate::iced_compat::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let content = self
            .content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits);
        let content_size = content.size();

        let width = content_size
            .width
            .max(self.min_width.min(limits.max().width));
        let offset_x = ((width - content_size.width) / 2.0).max(0.0);

        layout::Node::with_children(
            Size::new(width, content_size.height),
            vec![content.move_to(Point::new(offset_x, 0.0))],
        )
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
            layout
                .children()
                .next()
                .expect("kbd min-width child layout"),
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
            layout
                .children()
                .next()
                .expect("kbd min-width child layout"),
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
            layout
                .children()
                .next()
                .expect("kbd min-width child layout"),
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
            layout
                .children()
                .next()
                .expect("kbd min-width child layout"),
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
            layout
                .children()
                .next()
                .expect("kbd min-width child layout"),
            renderer,
            viewport,
            translation,
        )
    }
}
