use iced::Background;
use iced::Element;
use iced::Event;
use iced::Font;
use iced::Length;
use iced::Rectangle;
use iced::Shadow;
use iced::Size;
use iced::alignment;
use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::text;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::border::Border;
use iced::mouse;
use iced::touch;
use iced::widget::checkbox as checkbox_widget;
use iced::window;
use lucide_icons::Icon as LucideIcon;

use crate::theme::Theme;
use crate::tokens::{
    AccentColor, accent_color, accent_foreground, accent_high, accent_low, accent_soft, accent_text,
    is_dark,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxState {
    Unchecked,
    Checked,
    Indeterminate,
}

impl CheckboxState {
    pub fn is_checked(self) -> bool {
        matches!(self, CheckboxState::Checked)
    }

    pub fn is_active(self) -> bool {
        matches!(self, CheckboxState::Checked | CheckboxState::Indeterminate)
    }

    pub fn is_indeterminate(self) -> bool {
        matches!(self, CheckboxState::Indeterminate)
    }

    pub fn next(self) -> Self {
        match self {
            CheckboxState::Unchecked => CheckboxState::Checked,
            CheckboxState::Checked => CheckboxState::Unchecked,
            CheckboxState::Indeterminate => CheckboxState::Checked,
        }
    }
}

impl From<bool> for CheckboxState {
    fn from(value: bool) -> Self {
        if value {
            CheckboxState::Checked
        } else {
            CheckboxState::Unchecked
        }
    }
}

impl From<CheckboxState> for bool {
    fn from(value: CheckboxState) -> Self {
        value.is_checked()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxSize {
    One,
    Two,
    Three,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxVariant {
    Classic,
    Surface,
    Soft,
}

#[derive(Clone, Copy, Debug)]
pub struct CheckboxProps {
    pub size: CheckboxSize,
    pub variant: CheckboxVariant,
    pub color: AccentColor,
    pub high_contrast: bool,
    pub disabled: bool,
}

impl Default for CheckboxProps {
    fn default() -> Self {
        Self {
            size: CheckboxSize::Two,
            variant: CheckboxVariant::Surface,
            color: AccentColor::Gray,
            high_contrast: false,
            disabled: false,
        }
    }
}

impl CheckboxProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: CheckboxSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: CheckboxVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = color;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl CheckboxSize {
    fn dimension(self) -> f32 {
        match self {
            CheckboxSize::One => 14.0,
            CheckboxSize::Two => 16.0,
            CheckboxSize::Three => 20.0,
        }
    }

    fn icon_size(self) -> f32 {
        self.dimension() * 0.875
    }
}

fn checkbox_radius(props: CheckboxProps) -> f32 {
    props.size.dimension() * 0.25
}

pub fn checkbox<'a, Message: Clone + 'a, F>(
    state: CheckboxState,
    on_toggle: Option<F>,
    props: CheckboxProps,
    theme: &Theme,
) -> CheckboxWidget<'a, Message>
where
    F: Fn(CheckboxState) -> Message + 'a,
{
    CheckboxWidget::new(state, on_toggle, props, theme)
}

pub struct CheckboxWidget<'a, Message> {
    state: CheckboxState,
    on_toggle: Option<Box<dyn Fn(CheckboxState) -> Message + 'a>>,
    props: CheckboxProps,
    theme: Theme,
    last_status: Option<checkbox_widget::Status>,
}

impl<'a, Message> CheckboxWidget<'a, Message> {
    fn new<F>(state: CheckboxState, on_toggle: Option<F>, props: CheckboxProps, theme: &Theme) -> Self
    where
        F: Fn(CheckboxState) -> Message + 'a,
    {
        Self {
            state,
            on_toggle: on_toggle.map(|f| Box::new(f) as _),
            props,
            theme: theme.clone(),
            last_status: None,
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for CheckboxWidget<'_, Message>
where
    Renderer: renderer::Renderer + text::Renderer<Font = Font>,
{
    fn size(&self) -> Size<Length> {
        let size = self.props.size.dimension();
        Size::new(Length::Fixed(size), Length::Fixed(size))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = self.props.size.dimension();
        layout::atomic(limits, Length::Fixed(size), Length::Fixed(size))
    }

    fn update(
        &mut self,
        _tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let mouse_over = cursor.is_over(layout.bounds());

                if mouse_over && let Some(on_toggle) = &self.on_toggle {
                    shell.publish((on_toggle)(self.state.next()));
                    shell.capture_event();
                }
            }
            _ => {}
        }

        let current_status = {
            let is_mouse_over = cursor.is_over(layout.bounds());
            let is_disabled = self.on_toggle.is_none();
            let is_checked = self.state.is_active();

            if is_disabled {
                checkbox_widget::Status::Disabled { is_checked }
            } else if is_mouse_over {
                checkbox_widget::Status::Hovered { is_checked }
            } else {
                checkbox_widget::Status::Active { is_checked }
            }
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.last_status = Some(current_status);
        } else if self
            .last_status
            .is_some_and(|status| status != current_status)
        {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) && self.on_toggle.is_some() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _defaults: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let status = self.last_status.unwrap_or(checkbox_widget::Status::Disabled {
            is_checked: self.state.is_active(),
        });
        let style = checkbox_style(&self.theme, self.props, status, self.state);

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: style.border,
                shadow: Shadow::default(),
                snap: false,
            },
            style.background,
        );

    if !self.state.is_active() {
        return;
    }

    let icon = if self.state.is_indeterminate() {
        LucideIcon::Minus
    } else {
        LucideIcon::Check
    };
    let icon_size = self.props.size.icon_size();
    let center = bounds.center();
    // Nudge lucide glyphs to optical center.
    let icon_offset = icon_size * 0.03;

    renderer.fill_text(
        text::Text {
            content: char::from(icon).to_string(),
            font: Font::with_name("lucide"),
            size: icon_size.into(),
            line_height: text::LineHeight::Absolute(icon_size.into()),
            bounds: bounds.size(),
            align_x: text::Alignment::Center,
            align_y: alignment::Vertical::Center,
            shaping: text::Shaping::Basic,
            wrapping: text::Wrapping::default(),
            },
            iced::Point::new(center.x, center.y + icon_offset),
            style.icon_color,
            *viewport,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<CheckboxWidget<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: renderer::Renderer + text::Renderer<Font = Font> + 'a,
{
    fn from(widget: CheckboxWidget<'a, Message>) -> Element<'a, Message, Theme, Renderer> {
        Element::new(widget)
    }
}

fn checkbox_style(
    theme: &Theme,
    props: CheckboxProps,
    status: checkbox_widget::Status,
    state: CheckboxState,
) -> checkbox_widget::Style {
    let palette = theme.palette;
    let radius = checkbox_radius(props);
    let accent = accent_color(&palette, props.color);
    let accent_fg = accent_foreground(&palette, props.color);
    let soft_bg = accent_soft(&palette, props.color);
    let text_color = accent_text(&palette, props.color);
    let base_bg = if is_dark(&palette) {
        Background::Color(apply_opacity(palette.input, 0.3))
    } else {
        Background::Color(iced::Color::TRANSPARENT)
    };

    let is_checked = state.is_checked();
    let is_indeterminate = state.is_indeterminate();
    let is_active = state.is_active();

    let mut background = match props.variant {
        CheckboxVariant::Soft => Background::Color(soft_bg),
        CheckboxVariant::Classic | CheckboxVariant::Surface => base_bg,
    };
    let mut border_color = match props.variant {
        CheckboxVariant::Soft => iced::Color::TRANSPARENT,
        CheckboxVariant::Classic | CheckboxVariant::Surface => palette.input,
    };
    let mut icon_color = palette.foreground;
    let mut label_color = palette.foreground;

    let indeterminate_as_checked = props.variant != CheckboxVariant::Surface;
    if is_checked || (is_indeterminate && indeterminate_as_checked) {
        match props.variant {
            CheckboxVariant::Soft => {
                background = Background::Color(soft_bg);
                icon_color = text_color;
            }
            CheckboxVariant::Classic | CheckboxVariant::Surface => {
                background = Background::Color(accent);
                border_color = accent;
                icon_color = accent_fg;
            }
        }
    } else if is_indeterminate {
        icon_color = palette.foreground;
    }

    if props.high_contrast && is_active {
        match props.variant {
            CheckboxVariant::Soft => {
                icon_color = accent_high(&palette, props.color);
            }
            CheckboxVariant::Classic | CheckboxVariant::Surface => {
                let high_bg = accent_high(&palette, props.color);
                background = Background::Color(high_bg);
                border_color = high_bg;
                icon_color = accent_low(&palette, props.color);
            }
        }
    }

    if matches!(status, checkbox_widget::Status::Disabled { .. }) {
        background = apply_background_opacity(background, 0.5);
        border_color = apply_opacity(border_color, 0.5);
        icon_color = apply_opacity(icon_color, 0.5);
        label_color = apply_opacity(label_color, 0.7);
    }

    checkbox_widget::Style {
        background,
        icon_color,
        border: Border {
            radius: radius.into(),
            width: 1.0,
            color: border_color,
        },
        text_color: Some(label_color),
    }
}

fn apply_opacity(color: iced::Color, opacity: f32) -> iced::Color {
    iced::Color {
        a: color.a * opacity,
        ..color
    }
}

fn apply_background_opacity(background: Background, opacity: f32) -> Background {
    match background {
        Background::Color(color) => Background::Color(apply_opacity(color, opacity)),
        _ => background,
    }
}
