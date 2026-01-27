use iced::border::Border;
use iced::widget::{column, container, row, text, text_editor, text_input};
use iced::advanced::text::Wrapping;
use iced::{Background, Color, Element, Length};

use crate::button::{ButtonProps, ButtonRadius, ButtonSize, ButtonVariant, button_content, icon_button};
use crate::input::TextFieldSize;
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

impl InputGroupAddonAlign {
    fn is_block(self) -> bool {
        matches!(self, InputGroupAddonAlign::BlockStart | InputGroupAddonAlign::BlockEnd)
    }
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
    let has_block = items.iter().any(|item| match item {
        InputGroupItem::Addon(addon) => addon.props.align.is_block(),
        _ => false,
    });

    let mut children: Vec<Element<'a, Message>> = Vec::with_capacity(items.len());
    for item in items {
        match item {
            InputGroupItem::Control(content) => children.push(content),
            InputGroupItem::Addon(addon) => {
                children.push(render_addon(addon, props.disabled, theme))
            }
        }
    }

    let content: Element<'a, Message> = if has_block {
        column(children).spacing(0).into()
    } else {
        row(children).spacing(0).into()
    };

    let theme = theme.clone();
    container(content)
        .width(Length::Fill)
        .style(move |_t| input_group_style(&theme, props))
        .into()
}

fn render_addon<'a, Message: Clone + 'a>(
    addon: InputGroupAddon<'a, Message>,
    disabled: bool,
    theme: &Theme,
) -> Element<'a, Message> {
    let padding = match addon.props.align {
        InputGroupAddonAlign::InlineStart | InputGroupAddonAlign::InlineEnd => [6.0, 12.0],
        InputGroupAddonAlign::BlockStart | InputGroupAddonAlign::BlockEnd => [8.0, 12.0],
    };

    let muted = theme.palette.muted_foreground;
    let disabled_color = apply_opacity(muted, 0.6);
    let mut wrapper = container(addon.content).padding(padding).style(move |_t| {
        iced::widget::container::Style {
            text_color: Some(if disabled { disabled_color } else { muted }),
            ..Default::default()
        }
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
            InputGroupButtonSize::Xs | InputGroupButtonSize::IconXs => ButtonSize::One,
            InputGroupButtonSize::Sm | InputGroupButtonSize::IconSm => ButtonSize::Two,
        }
    }

    fn is_icon(self) -> bool {
        matches!(self, InputGroupButtonSize::IconXs | InputGroupButtonSize::IconSm)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InputGroupButtonProps {
    pub variant: ButtonVariant,
    pub size: InputGroupButtonSize,
    pub disabled: bool,
}

impl Default for InputGroupButtonProps {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Ghost,
            size: InputGroupButtonSize::Xs,
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
    let button_props = ButtonProps::new()
        .variant(props.variant)
        .size(props.size.button_size())
        .disabled(props.disabled);

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
    pub size: TextFieldSize,
    pub disabled: bool,
    pub read_only: bool,
}

impl Default for InputGroupInputProps {
    fn default() -> Self {
        Self {
            size: TextFieldSize::Two,
            disabled: false,
            read_only: false,
        }
    }
}

impl InputGroupInputProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: TextFieldSize) -> Self {
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
        .style(move |_t, status| input_group_input_style(&theme, props, status));

    if let Some(on_input) = on_input {
        if props.disabled {
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
            size: TextareaSize::Two,
            disabled: false,
            text_color: None,
            placeholder_color: None,
            read_only: false,
            max_len: None,
            rows: None,
            resize: TextareaResize::None,
            wrapping: Wrapping::Word,
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
    let padding = textarea_padding(props.size);
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

fn input_padding(size: TextFieldSize) -> [f32; 2] {
    match size {
        TextFieldSize::One => [6.0, 10.0],
        TextFieldSize::Two => [8.0, 12.0],
        TextFieldSize::Three => [10.0, 14.0],
    }
}

fn input_text_size(size: TextFieldSize) -> u32 {
    match size {
        TextFieldSize::One | TextFieldSize::Two => 14,
        TextFieldSize::Three => 16,
    }
}

fn textarea_padding(size: TextareaSize) -> [f32; 2] {
    match size {
        TextareaSize::One => [6.0, 10.0],
        TextareaSize::Two => [8.0, 12.0],
        TextareaSize::Three => [10.0, 14.0],
    }
}

fn textarea_text_size(size: TextareaSize) -> u32 {
    match size {
        TextareaSize::One | TextareaSize::Two => 14,
        TextareaSize::Three => 16,
    }
}

fn textarea_min_height(props: InputGroupTextareaProps) -> f32 {
    if let Some(rows) = props.rows {
        let rows = rows.max(1) as f32;
        let text_size = textarea_text_size(props.size) as f32;
        let line_height = text_size * 1.4;
        let padding = textarea_padding(props.size);
        return line_height * rows + padding[0] * 2.0;
    }

    match props.size {
        TextareaSize::One => 64.0,
        TextareaSize::Two => 96.0,
        TextareaSize::Three => 128.0,
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

fn input_group_style(theme: &Theme, props: InputGroupProps) -> iced::widget::container::Style {
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
            width: 1.0,
            color: border_color,
        },
        ..Default::default()
    }
}

fn input_group_input_style(
    theme: &Theme,
    props: InputGroupInputProps,
    status: text_input::Status,
) -> text_input::Style {
    let palette = theme.palette;
    let accent = accent_color(&palette, AccentColor::Gray);

    let mut value = palette.foreground;
    let mut placeholder = palette.muted_foreground;

    if props.disabled {
        value = palette.muted_foreground;
        placeholder = palette.muted_foreground;
    }

    let mut border = Border {
        radius: 0.0.into(),
        width: 0.0,
        color: Color::TRANSPARENT,
    };

    if matches!(status, text_input::Status::Focused { .. }) {
        border.color = palette.ring;
    }

    text_input::Style {
        background: Background::Color(Color::TRANSPARENT),
        border,
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
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        placeholder,
        value,
        selection,
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color { a: color.a * opacity, ..color }
}
