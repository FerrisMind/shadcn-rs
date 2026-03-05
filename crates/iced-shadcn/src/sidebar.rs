use std::rc::Rc;

use iced::alignment::{Horizontal, Vertical};
use iced::border::Border;
use iced::widget::{button as iced_button, column, container, row, text};
use iced::{Background, Color, Element, Length};

use crate::button::{ButtonProps, ButtonSize, ButtonVariant, button};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

pub struct SidebarProviderProps {
    pub open: bool,
    pub default_open: bool,
    pub expanded_width: f32,
    pub collapsed_width: f32,
    pub animate: bool,
}

impl SidebarProviderProps {
    pub fn new(open: bool) -> Self {
        let style = crate::theme::ThemeStyles::default().sidebar;
        Self {
            open,
            default_open: true,
            expanded_width: style.expanded_width,
            collapsed_width: style.collapsed_width,
            animate: true,
        }
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn expanded_width(mut self, width: f32) -> Self {
        self.expanded_width = width;
        self
    }

    pub fn collapsed_width(mut self, width: f32) -> Self {
        self.collapsed_width = width;
        self
    }

    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }
}

pub struct SidebarContext<'a, Message> {
    pub open: bool,
    pub expanded_width: f32,
    pub collapsed_width: f32,
    pub animate: bool,
    on_open_change: Option<Rc<dyn Fn(bool) -> Message + 'a>>,
}

impl<'a, Message: Clone> SidebarContext<'a, Message> {
    pub fn is_collapsed(&self) -> bool {
        !self.open
    }

    pub fn set_open_message(&self, open: bool) -> Option<Message> {
        self.on_open_change.as_ref().map(|f| f(open))
    }

    pub fn toggle_message(&self) -> Option<Message> {
        self.on_open_change.as_ref().map(|f| f(!self.open))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SidebarProps {
    pub side: SidebarSide,
    pub padding: f32,
    pub border: bool,
}

impl SidebarProps {
    pub fn new() -> Self {
        Self {
            side: SidebarSide::Left,
            padding: 0.0,
            border: true,
        }
    }

    pub fn side(mut self, side: SidebarSide) -> Self {
        self.side = side;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }
}

impl Default for SidebarProps {
    fn default() -> Self {
        Self::new()
    }
}

pub fn sidebar_provider<'a, Message: Clone + 'a, F>(
    props: SidebarProviderProps,
    on_open_change: Option<F>,
    add_contents: impl FnOnce(&SidebarContext<'a, Message>) -> Element<'a, Message>,
) -> Element<'a, Message>
where
    F: Fn(bool) -> Message + 'a,
{
    let on_open_change = on_open_change.map(|f| Rc::new(f) as Rc<dyn Fn(bool) -> Message + 'a>);
    let ctx = SidebarContext {
        open: props.open,
        expanded_width: props.expanded_width,
        collapsed_width: props.collapsed_width,
        animate: props.animate,
        on_open_change,
    };

    add_contents(&ctx)
}

pub fn sidebar<'a, Message: Clone + 'a>(
    ctx: &SidebarContext<'_, Message>,
    props: SidebarProps,
    theme: &Theme,
    add_contents: impl FnOnce(&SidebarContext<'_, Message>) -> Element<'a, Message>,
) -> Element<'a, Message> {
    let width = if ctx.open {
        ctx.expanded_width
    } else {
        ctx.collapsed_width
    };

    let palette = theme.palette;
    let border = Border {
        radius: theme.radius.md.into(),
        width: if props.border { 1.0 } else { 0.0 },
        color: palette.sidebar_border,
    };
    let theme = theme.clone();

    container(add_contents(ctx))
        .width(Length::Fixed(width))
        .height(Length::Fill)
        .padding(props.padding)
        .style(move |_t| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.sidebar)),
            text_color: Some(theme.palette.sidebar_foreground),
            border,
            ..Default::default()
        })
        .into()
}

pub fn sidebar_trigger<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    ctx: &SidebarContext<'_, Message>,
    theme: &Theme,
) -> Element<'a, Message> {
    button(
        label.into(),
        ctx.toggle_message(),
        ButtonProps::new()
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::Size1),
        theme,
    )
    .into()
}

pub fn sidebar_header<'a, Message: Clone + 'a>(
    ctx: &SidebarContext<'_, Message>,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    sidebar_section(
        ctx,
        crate::theme::ThemeStyles::default()
            .sidebar
            .header_footer_padding,
        content,
    )
}

pub fn sidebar_content<'a, Message: Clone + 'a>(
    ctx: &SidebarContext<'_, Message>,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    sidebar_section(
        ctx,
        crate::theme::ThemeStyles::default().sidebar.content_padding,
        content,
    )
}

pub fn sidebar_footer<'a, Message: Clone + 'a>(
    ctx: &SidebarContext<'_, Message>,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    sidebar_section(
        ctx,
        crate::theme::ThemeStyles::default()
            .sidebar
            .header_footer_padding,
        content,
    )
}

fn sidebar_section<'a, Message: Clone + 'a>(
    _ctx: &SidebarContext<'_, Message>,
    padding: f32,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(content).padding(padding).into()
}

#[derive(Clone, Copy, Debug)]
pub struct SidebarGroupProps {
    pub spacing: f32,
}

impl SidebarGroupProps {
    pub fn new() -> Self {
        Self { spacing: 8.0 }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl Default for SidebarGroupProps {
    fn default() -> Self {
        Self::new()
    }
}

pub fn sidebar_group<'a, Message: Clone + 'a>(
    _ctx: &SidebarContext<'_, Message>,
    props: SidebarGroupProps,
    content: impl Into<Vec<Element<'a, Message>>>,
) -> Element<'a, Message> {
    column(content.into()).spacing(props.spacing).into()
}

#[derive(Clone, Debug)]
pub struct SidebarGroupLabelProps {
    pub text: String,
    pub show_when_collapsed: bool,
}

impl SidebarGroupLabelProps {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            show_when_collapsed: false,
        }
    }

    pub fn show_when_collapsed(mut self, show: bool) -> Self {
        self.show_when_collapsed = show;
        self
    }
}

pub fn sidebar_group_label<'a, Message: Clone + 'a>(
    props: SidebarGroupLabelProps,
    ctx: &SidebarContext<'_, Message>,
    theme: &Theme,
) -> Element<'a, Message> {
    if ctx.is_collapsed() && !props.show_when_collapsed {
        return container(text("")).into();
    }

    let color = apply_opacity(theme.palette.sidebar_foreground, 0.6);

    text(props.text)
        .size(11u32)
        .style(move |_t| iced::widget::text::Style { color: Some(color) })
        .into()
}

pub fn sidebar_group_content<'a, Message: Clone + 'a>(
    content: impl Into<Vec<Element<'a, Message>>>,
) -> Element<'a, Message> {
    column(content.into())
        .spacing(crate::theme::ThemeStyles::default().sidebar.menu_spacing)
        .into()
}

pub fn sidebar_menu<'a, Message: Clone + 'a>(
    content: impl Into<Vec<Element<'a, Message>>>,
) -> Element<'a, Message> {
    column(content.into())
        .spacing(crate::theme::ThemeStyles::default().sidebar.menu_spacing)
        .into()
}

pub fn sidebar_menu_item<'a, Message: Clone + 'a>(
    content: impl Into<Vec<Element<'a, Message>>>,
) -> Element<'a, Message> {
    row(content.into()).spacing(0).into()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarMenuButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl SidebarMenuButtonSize {
    fn height(self) -> f32 {
        match self {
            SidebarMenuButtonSize::Sm => 28.0,
            SidebarMenuButtonSize::Md => 32.0,
            SidebarMenuButtonSize::Lg => 40.0,
        }
    }

    fn padding(self) -> [f32; 2] {
        match self {
            SidebarMenuButtonSize::Sm => [6.0, 10.0],
            SidebarMenuButtonSize::Md => [8.0, 12.0],
            SidebarMenuButtonSize::Lg => [10.0, 12.0],
        }
    }

    fn text_size(self) -> u32 {
        match self {
            SidebarMenuButtonSize::Sm => 12,
            SidebarMenuButtonSize::Md => 13,
            SidebarMenuButtonSize::Lg => 14,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SidebarMenuButtonProps {
    pub label: String,
    pub size: SidebarMenuButtonSize,
    pub active: bool,
    pub disabled: bool,
    pub show_label_when_collapsed: bool,
}

impl SidebarMenuButtonProps {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            size: SidebarMenuButtonSize::Md,
            active: false,
            disabled: false,
            show_label_when_collapsed: true,
        }
    }

    pub fn size(mut self, size: SidebarMenuButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn show_label_when_collapsed(mut self, show: bool) -> Self {
        self.show_label_when_collapsed = show;
        self
    }
}

pub fn sidebar_menu_button<'a, Message: Clone + 'a>(
    props: SidebarMenuButtonProps,
    on_press: Option<Message>,
    ctx: &SidebarContext<'_, Message>,
    theme: &Theme,
) -> Element<'a, Message> {
    let collapsed = ctx.is_collapsed();
    let mut label = props.label.clone();
    if collapsed && !props.show_label_when_collapsed {
        label.truncate(1);
    }

    let mut content = container(text(label).size(props.size.text_size()))
        .height(Length::Fixed(props.size.height()))
        .width(Length::Fill)
        .align_y(Vertical::Center);

    if collapsed && !props.show_label_when_collapsed {
        content = content.align_x(Horizontal::Center).padding(0);
    } else {
        content = content
            .align_x(Horizontal::Left)
            .padding(props.size.padding());
    }

    let mut button = iced_button(content);

    if let Some(msg) = on_press
        && !props.disabled
    {
        button = button.on_press(msg);
    }

    let theme = theme.clone();
    let style_props = props.clone();
    button = button.style(move |_t, status| {
        sidebar_menu_button_style(&theme, &style_props, status, collapsed)
    });
    button.into()
}

fn sidebar_menu_button_style(
    theme: &Theme,
    props: &SidebarMenuButtonProps,
    status: iced_button::Status,
    _collapsed: bool,
) -> iced_button::Style {
    let palette = theme.palette;
    let hovered = matches!(status, iced_button::Status::Hovered);
    let pressed = matches!(status, iced_button::Status::Pressed);

    let mut background = Color::TRANSPARENT;
    if props.active || hovered || pressed {
        background = palette.sidebar_accent;
    }

    let mut text_color = palette.sidebar_foreground;
    if props.active || hovered || pressed {
        text_color = palette.sidebar_accent_foreground;
    }

    if props.disabled {
        text_color = palette.sidebar_foreground;
        background = Color::TRANSPARENT;
    }

    iced_button::Style {
        background: Some(Background::Color(background)),
        text_color,
        border: Border {
            radius: theme.radius.sm.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Default::default(),
        snap: true,
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}
