//! Layout, focus tracking, and rendering for [`super::InputGroup`].

use crate::iced_compat::advanced::layout::{self, Layout};
use crate::iced_compat::advanced::widget::operation::{Outcome, black_box, focusable};
use crate::iced_compat::advanced::widget::{Operation, Tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, overlay, renderer};
use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::{column, container, row, text as iced_text, text_editor};
use crate::iced_compat::{
    Background, Border, Element, Event, Length, Point, Rectangle, Size, Vector, mouse,
};
use iced_core::Renderer as _;

use crate::fonts::iced_font;
use crate::theme::Theme;

use super::style;
use super::{
    InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupText, InputGroupTextContent,
    ItemKind,
};

/// Builds the root group and keeps its border in sync with descendant focus.
pub(super) fn build_group<'a, Message>(group: InputGroup<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let InputGroup {
        theme,
        items,
        radius,
        invalid,
        disabled,
        width,
        height,
        aria_label: _,
        style_override,
    } = group;

    let mut invalid = invalid;
    let mut disabled = disabled;
    for item in &items {
        match &item.kind {
            ItemKind::Control {
                invalid: control_invalid,
                disabled: control_disabled,
                ..
            } => {
                invalid |= *control_invalid;
                disabled |= *control_disabled;
            }
            ItemKind::Input(input) => {
                invalid |= input.is_invalid();
                disabled |= input.is_disabled();
            }
            ItemKind::Textarea(textarea) => {
                invalid |= textarea.props.invalid;
                disabled |= textarea.props.disabled;
            }
            ItemKind::Addon(_) => {}
        }
    }
    let focus_id = items.iter().find_map(|item| match &item.kind {
        ItemKind::Control { focus_id, .. } => focus_id.clone(),
        ItemKind::Input(input) => input.focus_id(),
        ItemKind::Textarea(textarea) => textarea.id.clone(),
        ItemKind::Addon(addon) => addon.focus_id.clone(),
    });
    let has_inline_start = items.iter().any(|item| {
        matches!(
            &item.kind,
            ItemKind::Addon(addon) if addon.align == InputGroupAddonAlign::InlineStart
        )
    });
    let has_inline_end = items.iter().any(|item| {
        matches!(
            &item.kind,
            ItemKind::Addon(addon) if addon.align == InputGroupAddonAlign::InlineEnd
        )
    });

    let mut layout = GroupLayout::default();

    for item in items {
        match item.kind {
            ItemKind::Control { element, .. } => layout.controls.push(element),
            ItemKind::Input(input) => {
                layout
                    .controls
                    .push(group_input(input, has_inline_start, has_inline_end))
            }
            ItemKind::Textarea(textarea) => layout.controls.push(build_textarea(textarea)),
            ItemKind::Addon(addon) => {
                let align = addon.align;
                let addon = build_addon(addon, disabled);

                match align {
                    InputGroupAddonAlign::InlineStart => layout.inline_start.push(addon),
                    InputGroupAddonAlign::InlineEnd => layout.inline_end.push(addon),
                    InputGroupAddonAlign::BlockStart => layout.block_start.push(addon),
                    InputGroupAddonAlign::BlockEnd => layout.block_end.push(addon),
                }
            }
        }
    }

    let content = build_slots(layout);

    Element::new(InputGroupWidget {
        content,
        theme,
        radius,
        invalid,
        disabled,
        width,
        height,
        focus_id,
        style_override,
    })
}

struct GroupLayout<'a, Message> {
    block_start: Vec<Element<'a, Message>>,
    inline_start: Vec<Element<'a, Message>>,
    controls: Vec<Element<'a, Message>>,
    inline_end: Vec<Element<'a, Message>>,
    block_end: Vec<Element<'a, Message>>,
}

impl<'a, Message> Default for GroupLayout<'a, Message> {
    fn default() -> Self {
        Self {
            block_start: Vec::new(),
            inline_start: Vec::new(),
            controls: Vec::new(),
            inline_end: Vec::new(),
            block_end: Vec::new(),
        }
    }
}

fn build_slots<'a, Message: 'a>(layout: GroupLayout<'a, Message>) -> Element<'a, Message> {
    let GroupLayout {
        block_start,
        inline_start,
        controls,
        inline_end,
        block_end,
    } = layout;

    let mut outer = Vec::with_capacity(3);

    if !block_start.is_empty() {
        outer.push(column(block_start).spacing(0.0).width(Length::Fill).into());
    }

    let mut middle = Vec::new();
    middle.extend(inline_start);
    middle.extend(controls);
    middle.extend(inline_end);

    if !middle.is_empty() {
        outer.push(
            row(middle)
                .spacing(0.0)
                .width(Length::Fill)
                .align_y(Vertical::Center)
                .into(),
        );
    }

    if !block_end.is_empty() {
        outer.push(column(block_end).spacing(0.0).width(Length::Fill).into());
    }

    match outer.len() {
        0 => row(Vec::new()).width(Length::Fill).into(),
        1 => outer.pop().expect("input-group has one layout slot"),
        _ => column(outer).spacing(0.0).width(Length::Fill).into(),
    }
}

/// Renders one addon as a padded, muted slot.
pub(super) fn build_addon<'a, Message>(
    addon: InputGroupAddon<'a, Message>,
    group_disabled: bool,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let InputGroupAddon {
        theme,
        align,
        children,
        width,
        padding,
        spacing,
        disabled,
        focus_id: _focus_id,
        style_override,
    } = addon;

    let content: Element<'a, Message> = row(children)
        .spacing(spacing)
        .align_y(Vertical::Center)
        .into();
    let mut wrapper = container(content)
        .padding(padding.unwrap_or_else(|| style::addon_padding(theme, align)))
        .align_y(Vertical::Center)
        .style(move |_iced_theme| {
            let mut resolved = addon_style(theme, group_disabled || disabled);

            if let Some(override_fn) = style_override.as_ref() {
                resolved = override_fn(resolved);
            }

            resolved
        });

    let width = if align.is_block() && width == Length::Shrink {
        Length::Fill
    } else {
        width
    };

    wrapper = wrapper.width(width);
    if align.is_block() {
        wrapper = wrapper.align_x(Horizontal::Left);
    }

    wrapper.into()
}

fn addon_style(theme: &Theme, disabled: bool) -> crate::iced_compat::widget::container::Style {
    let mut style = crate::iced_compat::widget::container::Style {
        text_color: Some(
            theme.semantic_color(twill_core::prelude::theme::SemanticColor::MutedForeground),
        ),
        ..Default::default()
    };

    if disabled && let Some(color) = style.text_color {
        style.text_color = Some(ColorExt::with_alpha(color, 0.5));
    }

    style
}

/// Builds `InputGroupText`, preserving arbitrary element content.
pub(super) fn build_text<'a, Message>(text: InputGroupText<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    let InputGroupText {
        content,
        theme,
        text_size,
        width,
        style_override,
    } = text;

    let text_size = text_size.unwrap_or_else(|| style::addon_text_size(theme));
    let body: Element<'a, Message> = match content {
        InputGroupTextContent::Label(label) => iced_text(label.into_owned())
            .size(text_size)
            .font(iced_font(theme.font_pack().sans))
            .into(),
        InputGroupTextContent::Element(element) => element,
    };

    container(body)
        .width(width)
        .height(Length::Shrink)
        .align_y(Vertical::Center)
        .style(move |_iced_theme| {
            let mut resolved = crate::iced_compat::widget::container::Style {
                text_color: Some(
                    theme
                        .semantic_color(twill_core::prelude::theme::SemanticColor::MutedForeground),
                ),
                ..Default::default()
            };

            if let Some(override_fn) = style_override.as_ref() {
                resolved = override_fn(resolved);
            }

            resolved
        })
        .into()
}

/// Builds the multi-line control used by `InputGroupTextarea`.
pub(super) fn build_textarea<'a, Message>(
    textarea: super::InputGroupTextarea<'a, Message>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let super::InputGroupTextarea {
        theme,
        content,
        placeholder,
        props,
        id,
        on_action,
        style_override,
    } = textarea;

    let padding = style::textarea_padding(theme, props);
    let text_size = style::textarea_text_size(theme, props);
    let min_height = style::textarea_min_height(theme, props);
    let mut widget = text_editor::TextEditor::new(content)
        .placeholder(placeholder)
        .padding(padding)
        .size(text_size)
        .line_height(crate::iced_compat::widget::text::LineHeight::Absolute(
            (text_size * 1.4).into(),
        ))
        .min_height(min_height)
        .wrapping(props.wrapping)
        .style(move |_iced_theme, status| {
            let mut resolved = style::resolve_textarea_style(theme, props, status);

            if let Some(override_fn) = style_override.as_ref() {
                resolved = override_fn(resolved, status);
            }

            resolved
        });

    if let Some(max_height) = style::textarea_max_height(theme, props) {
        widget = widget.max_height(max_height);
    }

    if let Some(id) = id {
        widget = widget.id(id);
    }

    if matches!(props.resize, super::InputGroupTextareaResize::None) {
        widget = widget.height(Length::Fixed(min_height));
    }

    if !props.disabled
        && !props.read_only
        && let Some(on_action) = on_action
    {
        widget = widget.on_action(on_action);
    }

    widget.into()
}

/// Makes a regular [`crate::Input`] occupy the group control slot without
/// painting its own border or fill.
pub(super) fn group_input<'a, Message>(
    input: crate::components::input::Input<'a, Message>,
    has_inline_start: bool,
    has_inline_end: bool,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    input
        .group_slot_padding(has_inline_start, has_inline_end)
        .width(Length::Fill)
        .style_override(|mut resolved, _status| {
            resolved.background = Background::Color(crate::iced_compat::Color::TRANSPARENT);
            resolved.border = Border::default();
            resolved
        })
        .into()
}

#[derive(Debug, Default)]
struct InputGroupState {
    focused: bool,
}

struct InputGroupWidget<'a, Message> {
    content: Element<'a, Message>,
    theme: &'a Theme,
    radius: Option<super::InputGroupRadius>,
    invalid: bool,
    disabled: bool,
    width: Length,
    height: Length,
    focus_id: Option<crate::iced_compat::widget::Id>,
    style_override: Option<
        Box<
            dyn Fn(
                    crate::iced_compat::widget::container::Style,
                ) -> crate::iced_compat::widget::container::Style
                + 'a,
        >,
    >,
}

impl<Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for InputGroupWidget<'_, Message>
{
    fn tag(&self) -> crate::iced_compat::advanced::widget::tree::Tag {
        crate::iced_compat::advanced::widget::tree::Tag::of::<InputGroupState>()
    }

    fn state(&self) -> crate::iced_compat::advanced::widget::tree::State {
        crate::iced_compat::advanced::widget::tree::State::new(InputGroupState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &crate::iced_compat::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);
        let child = self
            .content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, &limits);
        let size = limits.resolve(self.width, self.height, child.size());
        // Honor Fixed/Fill height: previously the node shrank to the child, so
        // parents with Fixed height top-aligned the row (adornment buttons sat high).
        let offset_y = ((size.height - child.size().height) / 2.0).max(0.0);
        let child = child.move_to(Point::new(0.0, offset_y));

        layout::Node::with_children(size, vec![child])
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
            layout.children().next().expect("input-group child layout"),
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
        let child_layout = layout.children().next().expect("input-group child layout");

        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            child_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if !shell.is_event_captured()
            && cursor.is_over(layout.bounds())
            && matches!(
                event,
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            )
            && let Some(focus_id) = &self.focus_id
        {
            let mut focus = focusable::focus::<()>(focus_id.clone());
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                child_layout,
                renderer,
                &mut focus,
            );
            shell.capture_event();
        }

        let focused = if let Some(focus_id) = &self.focus_id {
            let mut focused = focusable::is_focused(focus_id.clone());
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                child_layout,
                renderer,
                &mut black_box(&mut focused),
            );

            matches!(focused.finish(), Outcome::Some(true))
        } else {
            let mut count = focusable::count();
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                child_layout,
                renderer,
                &mut black_box(&mut count),
            );

            matches!(
                count.finish(),
                Outcome::Some(result) if result.focused.is_some()
            )
        };
        let state = tree.state.downcast_mut::<InputGroupState>();
        if state.focused != focused {
            state.focused = focused;
            shell.request_redraw();
        }
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
            layout.children().next().expect("input-group child layout"),
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut crate::iced_compat::Renderer,
        _theme: &crate::iced_compat::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let state = tree.state.downcast_ref::<InputGroupState>();
        let mut resolved = style::resolve_group_style(
            self.theme,
            self.radius,
            self.invalid,
            self.disabled,
            state.focused,
        );

        if let Some(override_fn) = self.style_override.as_ref() {
            resolved = override_fn(resolved);
        }

        let bottom_border = style::uses_bottom_border(self.theme);
        let mut border = resolved.border;
        if bottom_border {
            border.color = crate::iced_compat::Color::TRANSPARENT;
        }

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border,
                shadow: resolved.shadow,
                ..renderer::Quad::default()
            },
            resolved
                .background
                .unwrap_or(Background::Color(crate::iced_compat::Color::TRANSPARENT)),
        );

        if bottom_border && resolved.border.width > 0.0 {
            let thickness = resolved.border.width.min(bounds.height);
            let line_bounds = Rectangle {
                x: bounds.x,
                y: bounds.y + bounds.height - thickness,
                width: bounds.width,
                height: thickness,
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds: line_bounds,
                    ..renderer::Quad::default()
                },
                Background::Color(resolved.border.color),
            );
        }

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            _theme,
            _style,
            layout.children().next().expect("input-group child layout"),
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
            layout.children().next().expect("input-group child layout"),
            renderer,
            viewport,
            translation,
        )
    }
}

trait ColorExt {
    fn with_alpha(self, alpha: f32) -> Self;
}

impl ColorExt for crate::iced_compat::Color {
    fn with_alpha(self, alpha: f32) -> Self {
        Self {
            a: (self.a * alpha).clamp(0.0, 1.0),
            ..self
        }
    }
}
