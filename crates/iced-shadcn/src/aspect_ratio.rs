use iced::advanced::layout;
use iced::advanced::widget::Tree;
use iced::advanced::{Layout, Widget};
use iced::{Element, Length, Rectangle, Size};

#[derive(Clone, Copy, Debug)]
pub struct AspectRatioProps {
    pub ratio: f32,
}

impl Default for AspectRatioProps {
    fn default() -> Self {
        Self { ratio: 1.0 }
    }
}

impl AspectRatioProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ratio(mut self, ratio: f32) -> Self {
        self.ratio = ratio.max(0.0001);
        self
    }
}

pub fn aspect_ratio<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    props: AspectRatioProps,
) -> AspectRatio<'a, Message> {
    AspectRatio {
        content: content.into(),
        props,
    }
}

pub struct AspectRatio<'a, Message> {
    content: Element<'a, Message>,
    props: AspectRatioProps,
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer> for AspectRatio<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.content.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let ratio = self.props.ratio.max(0.0001);
        let max = limits.max();
        let min = limits.min();

        let mut width = max.width;
        let mut height = width / ratio;

        if height > max.height {
            height = max.height;
            width = height * ratio;
        }

        width = width.clamp(min.width, max.width.max(min.width));
        height = height.clamp(min.height, max.height.max(min.height));

        let fixed = layout::Limits::new(Size::new(width, height), Size::new(width, height));
        let child = self
            .content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, &fixed);

        layout::Node::with_children(Size::new(width, height), vec![child])
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
        style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        cursor: iced::mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }
}

impl<'a, Message: 'a> From<AspectRatio<'a, Message>> for Element<'a, Message> {
    fn from(widget: AspectRatio<'a, Message>) -> Element<'a, Message> {
        Element::new(widget)
    }
}
