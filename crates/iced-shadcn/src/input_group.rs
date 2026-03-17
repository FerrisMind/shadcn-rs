use iced::advanced::Renderer as _;
use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::text::Wrapping;
use iced::advanced::widget::Operation;
use iced::advanced::widget::Tree;
use iced::advanced::widget::operation::focusable;
use iced::advanced::widget::operation::{Outcome, black_box};
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::alignment::Vertical;
use iced::border::Border;
use iced::widget::{column, container, row, text, text_editor, text_input};
use iced::{Alignment, Background, Color, Element, Event, Length, Rectangle, Shadow, Vector};

use crate::button::{
    ButtonProps, ButtonRadius, ButtonSize, ButtonVariant, button_content, icon_button,
};
use crate::input::InputSize;
use crate::textarea::{TextareaProps, TextareaResize, TextareaSize, textarea_apply_action};
use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_color, ensure_contrast, is_dark};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputGroupAddonAlign {
    #[default]
    InlineStart,
    InlineEnd,
    BlockStart,
    BlockEnd,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InputGroupZone {
    BlockStart,
    InlineStart,
    Control,
    InlineEnd,
    BlockEnd,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct InputGroupProps {
    pub radius: Option<ButtonRadius>,
    pub invalid: bool,
    pub disabled: bool,
}

impl InputGroupProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InputGroupAddonProps {
    pub align: InputGroupAddonAlign,
}

impl Default for InputGroupAddonProps {
    fn default() -> Self {
        Self {
            align: InputGroupAddonAlign::InlineStart,
        }
    }
}

impl InputGroupAddonProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn align(mut self, align: InputGroupAddonAlign) -> Self {
        self.align = align;
        self
    }
}

pub struct InputGroupAddon<'a, Message> {
    pub content: Element<'a, Message>,
    pub props: InputGroupAddonProps,
}

pub enum InputGroupItem<'a, Message> {
    Control(Element<'a, Message>),
    Addon(InputGroupAddon<'a, Message>),
}

pub fn input_group_addon<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    props: InputGroupAddonProps,
) -> InputGroupItem<'a, Message> {
    InputGroupItem::Addon(InputGroupAddon {
        content: content.into(),
        props,
    })
}

pub fn input_group_control<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> InputGroupItem<'a, Message> {
    InputGroupItem::Control(content.into())
}

pub fn input_group<'a, Message: Clone + 'a>(
    items: Vec<InputGroupItem<'a, Message>>,
    props: InputGroupProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let layout = input_group_layout(items, props.disabled, theme);
    let content = render_input_group_layout(layout);
    Element::new(InputGroupWidget::new(content, props, theme))
}

struct InputGroupLayout<'a, Message> {
    block_start: Vec<Element<'a, Message>>,
    inline_start: Vec<Element<'a, Message>>,
    controls: Vec<Element<'a, Message>>,
    inline_end: Vec<Element<'a, Message>>,
    block_end: Vec<Element<'a, Message>>,
}

fn input_group_layout<'a, Message: Clone + 'a>(
    items: Vec<InputGroupItem<'a, Message>>,
    disabled: bool,
    theme: &Theme,
) -> InputGroupLayout<'a, Message> {
    let mut layout = InputGroupLayout {
        block_start: Vec::new(),
        inline_start: Vec::new(),
        controls: Vec::new(),
        inline_end: Vec::new(),
        block_end: Vec::new(),
    };

    for item in items {
        match item {
            InputGroupItem::Control(content) => layout.controls.push(content),
            InputGroupItem::Addon(addon) => {
                let align = addon.props.align;
                let element = render_addon(addon, disabled, theme);
                match align {
                    InputGroupAddonAlign::InlineStart => layout.inline_start.push(element),
                    InputGroupAddonAlign::InlineEnd => layout.inline_end.push(element),
                    InputGroupAddonAlign::BlockStart => layout.block_start.push(element),
                    InputGroupAddonAlign::BlockEnd => layout.block_end.push(element),
                }
            }
        }
    }

    layout
}

fn render_input_group_layout<'a, Message: Clone + 'a>(
    layout: InputGroupLayout<'a, Message>,
) -> Element<'a, Message> {
    let mut outer_children: Vec<Element<'a, Message>> = Vec::new();

    if !layout.block_start.is_empty() {
        outer_children.push(
            column(layout.block_start)
                .spacing(0)
                .width(Length::Fill)
                .into(),
        );
    }

    let mut middle_children: Vec<Element<'a, Message>> = Vec::new();
    middle_children.extend(layout.inline_start);
    middle_children.extend(layout.controls);
    middle_children.extend(layout.inline_end);

    if !middle_children.is_empty() {
        outer_children.push(
            row(middle_children)
                .spacing(0)
                .width(Length::Fill)
                .align_y(Alignment::Center)
                .into(),
        );
    }

    if !layout.block_end.is_empty() {
        outer_children.push(
            column(layout.block_end)
                .spacing(0)
                .width(Length::Fill)
                .into(),
        );
    }

    if outer_children.len() == 1 {
        outer_children.remove(0)
    } else {
        column(outer_children).spacing(0).width(Length::Fill).into()
    }
}

#[derive(Debug, Default)]
struct InputGroupState {
    is_focused: bool,
}

struct InputGroupWidget<'a, Message> {
    content: Element<'a, Message>,
    props: InputGroupProps,
    theme: Theme,
}

impl<'a, Message> InputGroupWidget<'a, Message> {
    fn new(content: Element<'a, Message>, props: InputGroupProps, theme: &Theme) -> Self {
        Self {
            content,
            props,
            theme: theme.clone(),
        }
    }
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer> for InputGroupWidget<'_, Message>
where
    Message: Clone,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<InputGroupState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(InputGroupState::default())
    }

    fn size(&self) -> iced::Size<Length> {
        iced::Size::new(Length::Fill, Length::Shrink)
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.content.as_widget()]);
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let content = self
            .content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits);

        layout::Node::with_children(content.size(), vec![content])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        if let Some(child_layout) = layout.children().next() {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                child_layout,
                renderer,
                operation,
            );
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: iced::mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let Some(child_layout) = layout.children().next() else {
            return;
        };

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

        let mut count = focusable::count();
        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            child_layout,
            renderer,
            &mut black_box(&mut count),
        );

        let is_focused = matches!(
            count.finish(),
            Outcome::Some(result) if result.focused.is_some()
        );

        let state = tree.state.downcast_mut::<InputGroupState>();
        if state.is_focused != is_focused {
            state.is_focused = is_focused;
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: iced::mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> iced::mouse::Interaction {
        let Some(child_layout) = layout.children().next() else {
            return iced::mouse::Interaction::default();
        };

        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            child_layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        _theme: &iced::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: iced::mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let state = tree.state.downcast_ref::<InputGroupState>();
        let style = input_group_style(&self.theme, self.props, state.is_focused);

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: style.border,
                shadow: style.shadow,
                ..renderer::Quad::default()
            },
            style
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
        );

        if let Some(child_layout) = layout.children().next() {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                _theme,
                _style,
                child_layout,
                cursor,
                viewport,
            );
        }
    }
}

impl<'a, Message> From<InputGroupWidget<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(widget: InputGroupWidget<'a, Message>) -> Element<'a, Message> {
        Element::new(widget)
    }
}

fn render_addon<'a, Message: Clone + 'a>(
    addon: InputGroupAddon<'a, Message>,
    disabled: bool,
    theme: &Theme,
) -> Element<'a, Message> {
    let padding = match addon.props.align {
        InputGroupAddonAlign::InlineStart | InputGroupAddonAlign::InlineEnd => [6.0, 12.0],
        InputGroupAddonAlign::BlockStart | InputGroupAddonAlign::BlockEnd => [12.0, 12.0],
    };

    let muted = theme.palette.muted_foreground;
    let disabled_color = apply_opacity(muted, 0.5);
    let mut wrapper = container(addon.content)
        .padding(padding)
        .align_y(Vertical::Center)
        .style(move |_t| iced::widget::container::Style {
            text_color: Some(if disabled { disabled_color } else { muted }),
            ..Default::default()
        });

    if matches!(
        addon.props.align,
        InputGroupAddonAlign::BlockStart | InputGroupAddonAlign::BlockEnd
    ) {
        wrapper = wrapper.width(Length::Fill);
    }

    wrapper.into()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputGroupButtonSize {
    #[default]
    Xs,
    Sm,
    IconXs,
    IconSm,
}

impl InputGroupButtonSize {
    fn button_size(self) -> ButtonSize {
        match self {
            InputGroupButtonSize::Xs | InputGroupButtonSize::IconXs => ButtonSize::Size0,
            InputGroupButtonSize::Sm | InputGroupButtonSize::IconSm => ButtonSize::Size1,
        }
    }

    fn is_icon(self) -> bool {
        matches!(
            self,
            InputGroupButtonSize::IconXs | InputGroupButtonSize::IconSm
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InputGroupButtonProps {
    pub variant: ButtonVariant,
    pub size: InputGroupButtonSize,
    pub radius: Option<ButtonRadius>,
    pub disabled: bool,
}

impl Default for InputGroupButtonProps {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Ghost,
            size: InputGroupButtonSize::Xs,
            radius: None,
            disabled: false,
        }
    }
}

impl InputGroupButtonProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: InputGroupButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub fn input_group_button<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    on_press: Option<Message>,
    props: InputGroupButtonProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let mut button_props = ButtonProps::new()
        .variant(props.variant)
        .size(props.size.button_size())
        .disabled(props.disabled);

    if let Some(radius) = props.radius {
        button_props = button_props.radius(radius);
    }

    if props.size.is_icon() {
        icon_button(content, on_press, button_props, theme).into()
    } else {
        button_content(content, on_press, button_props, theme).into()
    }
}

pub fn input_group_text<'a, Message: Clone + 'a>(
    value: impl Into<String>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    text(value.into())
        .size(12.0)
        .style(move |_t| iced::widget::text::Style {
            color: Some(theme.palette.muted_foreground),
        })
        .into()
}

#[derive(Clone, Copy, Debug)]
pub struct InputGroupInputProps {
    pub size: InputSize,
    pub disabled: bool,
    pub read_only: bool,
}

impl Default for InputGroupInputProps {
    fn default() -> Self {
        Self {
            size: InputSize::Size2,
            disabled: false,
            read_only: false,
        }
    }
}

impl InputGroupInputProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }
}

pub fn input_group_input<'a, Message: Clone + 'a, F>(
    value: &'a str,
    placeholder: &'a str,
    on_input: Option<F>,
    props: InputGroupInputProps,
    theme: &Theme,
) -> InputGroupItem<'a, Message>
where
    F: Fn(String) -> Message + 'a,
{
    let theme = theme.clone();
    let mut widget = text_input::TextInput::new(placeholder, value)
        .padding(input_padding(props.size))
        .size(input_text_size(props.size))
        .width(Length::Fill)
        .style(move |_t, status| input_group_input_style(&theme, props, status));

    if let Some(on_input) = on_input {
        if props.disabled || props.read_only {
            widget = widget.on_input_maybe(None::<fn(String) -> Message>);
        } else {
            widget = widget.on_input(on_input);
        }
    } else {
        widget = widget.on_input_maybe(None::<fn(String) -> Message>);
    }

    InputGroupItem::Control(widget.into())
}

#[derive(Clone, Copy, Debug)]
pub struct InputGroupTextareaProps {
    pub size: TextareaSize,
    pub disabled: bool,
    pub padding: Option<[f32; 2]>,
    pub text_color: Option<iced::Color>,
    pub placeholder_color: Option<iced::Color>,
    pub read_only: bool,
    pub max_len: Option<usize>,
    pub rows: Option<usize>,
    pub resize: TextareaResize,
    pub wrapping: Wrapping,
}

impl Default for InputGroupTextareaProps {
    fn default() -> Self {
        Self {
            size: TextareaSize::Size2,
            disabled: false,
            padding: None,
            text_color: None,
            placeholder_color: None,
            read_only: false,
            max_len: None,
            rows: None,
            resize: TextareaResize::None,
            wrapping: Wrapping::WordOrGlyph,
        }
    }
}

impl InputGroupTextareaProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: TextareaSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn padding(mut self, padding: [f32; 2]) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn text_color(mut self, color: iced::Color) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn placeholder_color(mut self, color: iced::Color) -> Self {
        self.placeholder_color = Some(color);
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn max_len(mut self, max_len: usize) -> Self {
        self.max_len = Some(max_len);
        self
    }

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = Some(rows);
        self
    }

    pub fn resize(mut self, resize: TextareaResize) -> Self {
        self.resize = resize;
        self
    }

    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }
}

pub fn input_group_textarea<'a, Message: Clone + 'a, F>(
    content: &'a text_editor::Content,
    placeholder: &'a str,
    on_action: Option<F>,
    props: InputGroupTextareaProps,
    theme: &Theme,
) -> InputGroupItem<'a, Message>
where
    F: Fn(text_editor::Action) -> Message + 'a,
{
    let theme = theme.clone();
    let padding = props
        .padding
        .unwrap_or_else(|| textarea_padding(props.size));
    let text_size = textarea_text_size(props.size);
    let min_height = textarea_min_height(props);
    let mut widget = text_editor::TextEditor::new(content)
        .placeholder(placeholder)
        .padding(padding)
        .size(text_size)
        .min_height(min_height)
        .wrapping(props.wrapping)
        .style(move |_t, status| input_group_textarea_style(&theme, props, status));

    if props.resize == TextareaResize::None {
        widget = widget.height(Length::Fixed(min_height));
    }

    if !props.disabled
        && let Some(on_action) = on_action
    {
        widget = widget.on_action(on_action);
    }

    InputGroupItem::Control(widget.into())
}

pub fn input_group_textarea_apply_action(
    content: &mut text_editor::Content,
    action: text_editor::Action,
    props: InputGroupTextareaProps,
) -> bool {
    let mut textarea_props = TextareaProps::new()
        .size(props.size)
        .resize(props.resize)
        .disabled(props.disabled)
        .read_only(props.read_only);

    if let Some(max_len) = props.max_len {
        textarea_props = textarea_props.max_len(max_len);
    }

    textarea_apply_action(content, action, textarea_props)
}

fn input_padding(size: InputSize) -> [f32; 2] {
    match size {
        InputSize::Size1 => [6.0, 10.0],
        InputSize::Size2 => [8.0, 12.0],
        InputSize::Size3 => [10.0, 14.0],
    }
}

fn input_text_size(size: InputSize) -> u32 {
    match size {
        InputSize::Size1 | InputSize::Size2 => 14,
        InputSize::Size3 => 16,
    }
}

fn textarea_padding(size: TextareaSize) -> [f32; 2] {
    match size {
        TextareaSize::Size1 => [6.0, 10.0],
        TextareaSize::Size2 => [8.0, 12.0],
        TextareaSize::Size3 => [10.0, 14.0],
    }
}

fn textarea_text_size(size: TextareaSize) -> u32 {
    match size {
        TextareaSize::Size1 | TextareaSize::Size2 => 14,
        TextareaSize::Size3 => 16,
    }
}

fn textarea_min_height(props: InputGroupTextareaProps) -> f32 {
    if let Some(rows) = props.rows {
        let rows = rows.max(1) as f32;
        let text_size = textarea_text_size(props.size) as f32;
        let line_height = text_size * 1.4;
        let padding = props
            .padding
            .unwrap_or_else(|| textarea_padding(props.size));
        return line_height * rows + padding[0] * 2.0;
    }

    match props.size {
        TextareaSize::Size1 => 64.0,
        TextareaSize::Size2 => 96.0,
        TextareaSize::Size3 => 128.0,
    }
}

fn input_group_radius(theme: &Theme, props: InputGroupProps) -> f32 {
    match props.radius {
        Some(ButtonRadius::None) => 0.0,
        Some(ButtonRadius::Small) => theme.radius.sm,
        Some(ButtonRadius::Medium) => theme.radius.md,
        Some(ButtonRadius::Large) => theme.radius.lg,
        Some(ButtonRadius::Full) => 9999.0,
        None => theme.radius.sm,
    }
}

fn input_group_style(
    theme: &Theme,
    props: InputGroupProps,
    is_focused: bool,
) -> iced::widget::container::Style {
    let palette = theme.palette;
    let radius = input_group_radius(theme, props);

    let background = if props.disabled {
        palette.muted
    } else if is_dark(&palette) {
        palette.input
    } else {
        palette.background
    };

    let border_color = if props.invalid {
        palette.destructive
    } else if is_focused {
        palette.ring
    } else {
        palette.border
    };

    let text_color = if props.disabled {
        palette.muted_foreground
    } else {
        palette.foreground
    };

    iced::widget::container::Style {
        background: Some(Background::Color(background)),
        text_color: Some(text_color),
        border: Border {
            radius: radius.into(),
            width: if is_focused { 1.5 } else { 1.0 },
            color: border_color,
        },
        shadow: shadow_xs(props.disabled, is_focused),
        ..Default::default()
    }
}

fn input_group_input_style(
    theme: &Theme,
    props: InputGroupInputProps,
    _status: text_input::Status,
) -> text_input::Style {
    let palette = theme.palette;
    let accent = accent_color(&palette, AccentColor::Gray);

    let mut value = palette.foreground;
    let mut placeholder = palette.muted_foreground;

    if props.disabled {
        value = palette.muted_foreground;
        placeholder = palette.muted_foreground;
    }

    text_input::Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border::default(),
        icon: palette.muted_foreground,
        placeholder,
        value,
        selection: accent,
    }
}

fn input_group_textarea_style(
    theme: &Theme,
    props: InputGroupTextareaProps,
    _status: text_editor::Status,
) -> text_editor::Style {
    let palette = theme.palette;
    let accent = accent_color(&palette, AccentColor::Gray);

    let mut value = if props.disabled {
        palette.muted_foreground
    } else {
        palette.foreground
    };
    let mut placeholder = palette.muted_foreground;
    let mut selection = accent;
    let value_overridden = props.text_color.is_some();
    let placeholder_overridden = props.placeholder_color.is_some();

    if !props.disabled {
        if let Some(color) = props.text_color {
            value = color;
        }
        if let Some(color) = props.placeholder_color {
            placeholder = color;
        }

        let background = Background::Color(Color::TRANSPARENT);
        let fallback_bg = palette.background;
        if !value_overridden {
            value = ensure_contrast(background, fallback_bg, value);
        }
        if !placeholder_overridden {
            placeholder = ensure_contrast(background, fallback_bg, placeholder);
        }
    }

    if props.disabled {
        selection = palette.muted;
    }

    if props.read_only && !props.disabled {
        value = palette.muted_foreground;
        placeholder = palette.muted_foreground;
        selection = palette.muted;
    }

    text_editor::Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border::default(),
        placeholder,
        value,
        selection,
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}

fn shadow_xs(disabled: bool, is_focused: bool) -> Shadow {
    let opacity = if disabled {
        0.03
    } else if is_focused {
        0.08
    } else {
        0.05
    };
    Shadow {
        color: apply_opacity(Color::BLACK, opacity),
        offset: Vector::new(0.0, 1.0),
        blur_radius: 2.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::widget::text;

    fn control<'a>() -> InputGroupItem<'a, ()> {
        InputGroupItem::Control(text("control").into())
    }

    fn addon<'a>(align: InputGroupAddonAlign, label: &'a str) -> InputGroupItem<'a, ()> {
        input_group_addon(text(label), InputGroupAddonProps::new().align(align))
    }

    fn slot_order<'a>(layout: InputGroupLayout<'a, ()>) -> Vec<InputGroupZone> {
        let mut order = Vec::new();

        if !layout.block_start.is_empty() {
            order.push(InputGroupZone::BlockStart);
        }
        if !layout.inline_start.is_empty() {
            order.push(InputGroupZone::InlineStart);
        }
        if !layout.controls.is_empty() {
            order.push(InputGroupZone::Control);
        }
        if !layout.inline_end.is_empty() {
            order.push(InputGroupZone::InlineEnd);
        }
        if !layout.block_end.is_empty() {
            order.push(InputGroupZone::BlockEnd);
        }

        order
    }

    #[test]
    fn layout_groups_items_by_alignment_in_render_order() {
        let theme = Theme::default();
        let layout = input_group_layout(
            vec![
                addon(InputGroupAddonAlign::InlineEnd, "end"),
                control(),
                addon(InputGroupAddonAlign::InlineStart, "start"),
                addon(InputGroupAddonAlign::BlockEnd, "bottom"),
                addon(InputGroupAddonAlign::BlockStart, "top"),
            ],
            false,
            &theme,
        );

        assert_eq!(
            slot_order(layout),
            vec![
                InputGroupZone::BlockStart,
                InputGroupZone::InlineStart,
                InputGroupZone::Control,
                InputGroupZone::InlineEnd,
                InputGroupZone::BlockEnd,
            ]
        );
    }

    #[test]
    fn layout_keeps_multiple_controls_in_input_order() {
        let theme = Theme::default();
        let layout = input_group_layout(vec![control(), control(), control()], false, &theme);

        assert_eq!(layout.controls.len(), 3);
        assert!(layout.block_start.is_empty());
        assert!(layout.inline_start.is_empty());
        assert!(layout.inline_end.is_empty());
        assert!(layout.block_end.is_empty());
    }

    #[test]
    fn layout_detects_block_addons() {
        let theme = Theme::default();
        let layout = input_group_layout(
            vec![addon(InputGroupAddonAlign::BlockStart, "top")],
            false,
            &theme,
        );

        assert_eq!(slot_order(layout), vec![InputGroupZone::BlockStart]);
    }
}
