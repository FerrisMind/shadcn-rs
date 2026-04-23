use iced::alignment::{Horizontal, Vertical};
use iced::border::Border;
use iced::widget::button as button_widget;
use iced::widget::text::{Fragment, IntoFragment};
use iced::widget::{button as iced_button, container, stack, text as iced_text};
use iced::{Background, Color, Element, Length, Shadow, Vector};

use twill::backends::iced::{to_border_radius, to_color, to_color_value};
use twill::prelude::{
    BackgroundColor, BorderColor, BorderRadius, BorderStyle, BorderWidth, Color as TwillColor,
    ColorValueToken, Padding, PaddingValue, Scale, SemanticColor, Shadow as TwillShadow, Spacing,
    Style, TextColor, ThemeVariant,
};
use twill::traits::{ComputeValue, Merge};

use crate::spinner::{Spinner, SpinnerSize, spinner};
use crate::theme::Theme;
use crate::tokens::AccentColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
    Soft,
    Surface,
    Classic,
    Solid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Size0,
    Size1,
    #[default]
    Size2,
    Size3,
    Size4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonRadius {
    None,
    Small,
    #[default]
    Medium,
    Large,
    Full,
}

/// Experimental builder-first button API backed by `twill`.
///
/// The component semantics stay in `iced-shadcn`, while `twill` is used as the
/// internal utility-style composition layer.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn::{AccentColor, Theme};
/// use iced_shadcn::new_api::{Button, ButtonSize, ButtonVariant};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Save,
/// }
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     Button::text("Save", theme)
///         .variant(ButtonVariant::Default)
///         .size(ButtonSize::Size3)
///         .color(AccentColor::Blue)
///         .on_press(Message::Save)
///         .into()
/// }
/// ```
pub struct Button<'a, Message> {
    content: ButtonContent<'a, Message>,
    theme: Theme,
    variant: ButtonVariant,
    size: ButtonSize,
    radius: Option<ButtonRadius>,
    color: AccentColor,
    width: Length,
    height: Option<Length>,
    padding: Option<Padding>,
    full_width: bool,
    loading: bool,
    disabled: bool,
    on_press: Option<Message>,
    style_override: Option<
        Box<dyn Fn(button_widget::Style, button_widget::Status) -> button_widget::Style + 'a>,
    >,
}

enum ButtonContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
    Icon(Element<'a, Message>),
}

impl<'a, Message> Button<'a, Message> {
    /// Creates a new button from arbitrary content.
    ///
    /// `theme` is required because `iced-shadcn` styling is derived from crate
    /// theme tokens instead of `iced::Theme`.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &Theme) -> Self {
        Self::from_content(ButtonContent::Element(content.into()), theme)
    }

    /// Creates a text button.
    pub fn text(label: impl IntoFragment<'a>, theme: &Theme) -> Self {
        Self::from_content(ButtonContent::Label(label.into_fragment()), theme)
    }

    /// Creates an icon button.
    pub fn icon(content: impl Into<Element<'a, Message>>, theme: &Theme) -> Self {
        Self::from_content(ButtonContent::Icon(content.into()), theme)
    }

    fn from_content(content: ButtonContent<'a, Message>, theme: &Theme) -> Self {
        Self {
            content,
            theme: theme.clone(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Size2,
            radius: None,
            color: AccentColor::Gray,
            width: Length::Shrink,
            height: None,
            padding: None,
            full_width: false,
            loading: false,
            disabled: false,
            on_press: None,
            style_override: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = color;
        self
    }

    pub fn tone(self, color: AccentColor) -> Self {
        self.color(color)
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Applies a narrow iced-style escape hatch after internal style
    /// resolution from `twill`.
    pub fn style_override(
        mut self,
        style_override: impl Fn(button_widget::Style, button_widget::Status) -> button_widget::Style
        + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the underlying `iced` button widget.
    pub fn into_button(self) -> button_widget::Button<'a, Message>
    where
        Message: Clone + 'a,
    {
        let Button {
            content,
            theme,
            variant,
            size,
            radius,
            color,
            width,
            height,
            padding,
            full_width,
            loading,
            disabled,
            on_press,
            style_override,
        } = self;

        let icon = matches!(content, ButtonContent::Icon(_));
        let control_height = height.unwrap_or(Length::Fixed(size.control_height()));
        let resolved_width = if full_width {
            Length::Fill
        } else if icon {
            Length::Fixed(size.control_height())
        } else {
            width
        };

        let content = build_content(content, size, loading, color, &theme);
        let content = build_wrapper(content, control_height, full_width, icon);
        let disabled_state = disabled || loading || on_press.is_none();

        let mut widget = iced_button(content)
            .padding(resolve_padding(size, padding.as_ref(), icon))
            .width(resolved_width)
            .height(control_height);

        if let Some(message) = on_press
            && !disabled_state
        {
            widget = widget.on_press(message);
        }

        widget.style(move |_iced_theme, status| {
            let mut style =
                resolve_button_style(&theme, variant, size, radius, color, disabled_state, status);

            if let Some(override_fn) = style_override.as_ref() {
                style = override_fn(style, status);
            }

            style
        })
    }
}

impl<'a, Message> From<Button<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(button: Button<'a, Message>) -> Self {
        button.into_button().into()
    }
}

fn build_content<'a, Message>(
    content: ButtonContent<'a, Message>,
    size: ButtonSize,
    loading: bool,
    color: AccentColor,
    theme: &Theme,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let content = match content {
        ButtonContent::Label(label) => iced_text(label)
            .size(u32::from(size.label_text_size()))
            .into(),
        ButtonContent::Element(content) => content,
        ButtonContent::Icon(content) => container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into(),
    };

    if loading {
        loading_overlay(content, size, color, theme)
    } else {
        content
    }
}

fn build_wrapper<'a, Message: 'a>(
    content: Element<'a, Message>,
    height: Length,
    full_width: bool,
    icon: bool,
) -> Element<'a, Message> {
    let mut wrapper = container(content).height(height).align_y(Vertical::Center);

    if full_width || icon {
        wrapper = wrapper.width(Length::Fill).align_x(Horizontal::Center);
    }

    wrapper.into()
}

fn loading_overlay<'a, Message>(
    content: Element<'a, Message>,
    size: ButtonSize,
    color: AccentColor,
    theme: &Theme,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let spinner_size = match size {
        ButtonSize::Size0 | ButtonSize::Size1 => SpinnerSize::Size1,
        ButtonSize::Size2 => SpinnerSize::Size2,
        ButtonSize::Size3 | ButtonSize::Size4 => SpinnerSize::Size3,
    };

    let spinner_color = accent_scale_color(theme, color, accent_soft_foreground_scale(theme));
    let spinner = spinner(Spinner::new(theme).size(spinner_size).color(spinner_color));
    let spinner_layer = container(spinner)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill);

    stack![container(content), spinner_layer]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn resolve_padding(size: ButtonSize, padding: Option<&Padding>, icon: bool) -> iced::Padding {
    if icon {
        return iced::Padding::ZERO;
    }

    let padding = padding.copied().unwrap_or_else(|| size.default_padding());

    iced::Padding {
        top: padding.top_side().map(padding_value_px).unwrap_or(0.0),
        right: padding.right_side().map(padding_value_px).unwrap_or(0.0),
        bottom: padding.bottom_side().map(padding_value_px).unwrap_or(0.0),
        left: padding.left_side().map(padding_value_px).unwrap_or(0.0),
    }
}

fn padding_value_px(value: PaddingValue) -> f32 {
    match value {
        PaddingValue::Scale(scale) => match scale {
            Spacing::S0 => 0.0,
            Spacing::Px => 1.0,
            Spacing::S0_5 => 2.0,
            Spacing::S1 => 4.0,
            Spacing::S1_5 => 6.0,
            Spacing::S2 => 8.0,
            Spacing::S2_5 => 10.0,
            Spacing::S3 => 12.0,
            Spacing::S3_5 => 14.0,
            Spacing::S4 => 16.0,
            Spacing::S5 => 20.0,
            Spacing::S6 => 24.0,
            Spacing::S7 => 28.0,
            Spacing::S8 => 32.0,
            Spacing::S9 => 36.0,
            Spacing::S10 => 40.0,
            Spacing::S11 => 44.0,
            Spacing::S12 => 48.0,
            Spacing::S14 => 56.0,
            Spacing::S16 => 64.0,
            Spacing::S20 => 80.0,
            Spacing::S24 => 96.0,
            Spacing::S28 => 112.0,
            Spacing::S32 => 128.0,
            Spacing::S36 => 144.0,
            Spacing::S40 => 160.0,
            Spacing::S44 => 176.0,
            Spacing::S48 => 192.0,
            Spacing::S52 => 208.0,
            Spacing::S56 => 224.0,
            Spacing::S60 => 240.0,
            Spacing::S64 => 256.0,
            Spacing::S72 => 288.0,
            Spacing::S80 => 320.0,
            Spacing::S96 => 384.0,
            Spacing::Auto => 0.0,
        },
        PaddingValue::Px(px) => px.max(0.0),
        PaddingValue::Rem(rem) => (rem * 16.0).max(0.0),
        PaddingValue::Var(_) => 0.0,
    }
}

fn resolve_button_style(
    theme: &Theme,
    variant: ButtonVariant,
    size: ButtonSize,
    radius: Option<ButtonRadius>,
    color: AccentColor,
    disabled: bool,
    status: button_widget::Status,
) -> button_widget::Style {
    let base = button_style(theme, variant, size, radius, color);

    let resolved = match status {
        button_widget::Status::Hovered => {
            base.merge(base.hover_style().cloned().unwrap_or_default())
        }
        button_widget::Status::Pressed => {
            base.merge(base.active_style().cloned().unwrap_or_default())
        }
        button_widget::Status::Disabled => {
            if disabled {
                base.merge(base.disabled_style().cloned().unwrap_or_default())
            } else {
                base
            }
        }
        button_widget::Status::Active => base,
    };

    style_from_twill(&resolved)
}

fn button_style(
    theme: &Theme,
    variant: ButtonVariant,
    size: ButtonSize,
    radius: Option<ButtonRadius>,
    color: AccentColor,
) -> Style {
    let accent = accent_scale_color(theme, color, accent_solid_scale(theme));
    let accent_fg = accent_contrast_color(theme, color, accent_solid_scale(theme));
    let accent_txt = accent_scale_color(theme, color, accent_soft_foreground_scale(theme));
    let soft_bg = accent_scale_color(theme, color, accent_soft_background_scale(theme));
    let soft_fg = accent_txt;

    let (base_bg, base_fg, border_color, border_width, shadow) = match variant {
        ButtonVariant::Default | ButtonVariant::Classic | ButtonVariant::Solid => {
            (Some(accent), accent_fg, accent, BorderWidth::S0, None)
        }
        ButtonVariant::Secondary => (
            Some(semantic_color(theme, SemanticColor::Secondary)),
            semantic_color(theme, SemanticColor::SecondaryForeground),
            semantic_color(theme, SemanticColor::Secondary),
            BorderWidth::S0,
            None,
        ),
        ButtonVariant::Destructive => (
            Some(semantic_color(theme, SemanticColor::Destructive)),
            semantic_foreground(theme, SemanticColor::Destructive),
            semantic_color(theme, SemanticColor::Destructive),
            BorderWidth::S0,
            None,
        ),
        ButtonVariant::Outline => (
            None,
            semantic_color(theme, SemanticColor::Foreground),
            semantic_color(theme, SemanticColor::Input),
            BorderWidth::S1,
            None,
        ),
        ButtonVariant::Ghost => (
            None,
            semantic_color(theme, SemanticColor::Foreground),
            Color::TRANSPARENT,
            BorderWidth::S0,
            None,
        ),
        ButtonVariant::Link => (None, accent, Color::TRANSPARENT, BorderWidth::S0, None),
        ButtonVariant::Soft => (Some(soft_bg), soft_fg, soft_bg, BorderWidth::S0, None),
        ButtonVariant::Surface => (
            Some(semantic_color(theme, SemanticColor::Background)),
            accent_txt,
            semantic_color(theme, SemanticColor::Border),
            BorderWidth::S1,
            Some(TwillShadow::Sm),
        ),
    };

    let mut style = Style::new()
        .padding(size.default_padding())
        .rounded(twill_radius(theme, radius.unwrap_or_default()))
        .text_color_token(text_color_token(base_fg))
        .border(border_width, BorderStyle::Solid, TwillColor::black())
        .border_color_token(border_color_token(border_color))
        .hover(|_| hovered_state(theme, variant, color, base_fg))
        .active(|_| pressed_state(theme, variant, color))
        .disabled(|_| disabled_state(theme));

    if let Some(bg) = base_bg {
        style = style.background_token(background_token(bg));
    } else {
        style = style.bg_transparent();
    }

    if let Some(shadow) = shadow {
        style = style.shadow(shadow);
    }

    style
}

fn hovered_state(
    theme: &Theme,
    variant: ButtonVariant,
    color: AccentColor,
    current_text: Color,
) -> Style {
    match variant {
        ButtonVariant::Default | ButtonVariant::Classic | ButtonVariant::Solid => Style::new()
            .background_token(background_token(accent_scale_color(
                theme,
                color,
                accent_solid_hover_scale(theme),
            ))),
        ButtonVariant::Secondary => Style::new()
            .background_token(background_token(semantic_color(
                theme,
                SemanticColor::Accent,
            )))
            .text_color_token(text_color_token(semantic_color(
                theme,
                SemanticColor::AccentForeground,
            ))),
        ButtonVariant::Destructive => {
            Style::new().background_token(background_token(destructive_hover_color(theme)))
        }
        ButtonVariant::Soft | ButtonVariant::Surface => {
            Style::new().background_token(background_token(accent_scale_color(
                theme,
                color,
                accent_soft_hover_scale(theme),
            )))
        }
        ButtonVariant::Outline => Style::new()
            .background_token(background_token(semantic_color(
                theme,
                SemanticColor::Accent,
            )))
            .text_color_token(text_color_token(semantic_color(
                theme,
                SemanticColor::AccentForeground,
            ))),
        ButtonVariant::Ghost => Style::new()
            .background_token(background_token(semantic_color(
                theme,
                SemanticColor::Accent,
            )))
            .text_color_token(text_color_token(semantic_color(
                theme,
                SemanticColor::AccentForeground,
            ))),
        ButtonVariant::Link => {
            Style::new().text_color_token(text_color_token(current_text_for_state(
                current_text,
                semantic_color(theme, SemanticColor::Foreground),
            )))
        }
    }
}

fn pressed_state(theme: &Theme, variant: ButtonVariant, color: AccentColor) -> Style {
    match variant {
        ButtonVariant::Default | ButtonVariant::Classic | ButtonVariant::Solid => Style::new()
            .background_token(background_token(accent_scale_color(
                theme,
                color,
                accent_solid_active_scale(theme),
            ))),
        ButtonVariant::Secondary => Style::new().background_token(background_token(
            semantic_color(theme, SemanticColor::Muted),
        )),
        ButtonVariant::Destructive => {
            Style::new().background_token(background_token(destructive_active_color(theme)))
        }
        ButtonVariant::Soft
        | ButtonVariant::Surface
        | ButtonVariant::Ghost
        | ButtonVariant::Outline => Style::new().background_token(background_token(
            semantic_color(theme, SemanticColor::Muted),
        )),
        ButtonVariant::Link => Style::new(),
    }
}

fn disabled_state(theme: &Theme) -> Style {
    Style::new()
        .background_token(background_token(semantic_color(
            theme,
            SemanticColor::Muted,
        )))
        .text_color_token(text_color_token(semantic_color(
            theme,
            SemanticColor::MutedForeground,
        )))
        .border(BorderWidth::S1, BorderStyle::Solid, TwillColor::black())
        .border_color_token(border_color_token(semantic_color(
            theme,
            SemanticColor::Border,
        )))
}

fn style_from_twill(style: &Style) -> button_widget::Style {
    button_widget::Style {
        background: resolve_background(style.background_color_value()),
        text_color: resolve_text_color(style.text_color_token_value()),
        border: Border {
            radius: style
                .border_radius_value()
                .map(to_border_radius)
                .unwrap_or_default()
                .into(),
            width: style
                .border_width_value()
                .map(|width| width.px_value() as f32)
                .unwrap_or(0.0),
            color: resolve_border_color(style.border_color_token_value()),
        },
        shadow: resolve_shadow(style.box_shadow_value()),
        snap: true,
    }
}

fn resolve_background(token: Option<BackgroundColor>) -> Option<Background> {
    match token {
        Some(BackgroundColor::Palette(color)) => Some(Background::Color(to_color(color))),
        Some(BackgroundColor::Arbitrary(value)) => {
            let color = to_color_value(value.into());

            if color.a <= f32::EPSILON {
                None
            } else {
                Some(Background::Color(color))
            }
        }
        Some(BackgroundColor::Transparent) => None,
        _ => None,
    }
}

fn resolve_text_color(token: Option<TextColor>) -> Color {
    match token {
        Some(TextColor::Palette(color)) => to_color(color),
        Some(TextColor::Arbitrary(value)) => to_color_value(value.into()),
        Some(TextColor::Transparent) => Color::TRANSPARENT,
        _ => Color::BLACK,
    }
}

fn resolve_border_color(token: Option<BorderColor>) -> Color {
    match token {
        Some(BorderColor::Palette(color)) => to_color(color),
        Some(BorderColor::Arbitrary(value)) => to_color_value(value.into()),
        Some(BorderColor::Transparent) => Color::TRANSPARENT,
        _ => Color::TRANSPARENT,
    }
}

fn resolve_shadow(token: Option<TwillShadow>) -> Shadow {
    match token {
        Some(TwillShadow::Xs2) => Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 0.0,
        },
        Some(TwillShadow::Xs) => Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        },
        Some(TwillShadow::Sm) => Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.10),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 3.0,
        },
        Some(TwillShadow::Md) => Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.10),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 6.0,
        },
        Some(TwillShadow::Lg) => Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.10),
            offset: Vector::new(0.0, 10.0),
            blur_radius: 15.0,
        },
        Some(TwillShadow::Xl) => Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.10),
            offset: Vector::new(0.0, 20.0),
            blur_radius: 25.0,
        },
        Some(TwillShadow::S2xl) => Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
            offset: Vector::new(0.0, 25.0),
            blur_radius: 50.0,
        },
        _ => Shadow::default(),
    }
}

fn background_token(color: Color) -> BackgroundColor {
    BackgroundColor::arbitrary(color_value_token(color))
}

fn text_color_token(color: Color) -> TextColor {
    TextColor::arbitrary(color_value_token(color))
}

fn border_color_token(color: Color) -> BorderColor {
    BorderColor::arbitrary(color_value_token(color))
}

fn color_value_token(color: Color) -> ColorValueToken {
    ColorValueToken::from_rgba8(
        color_channel(color.r),
        color_channel(color.g),
        color_channel(color.b),
        color_channel(color.a),
    )
}

fn color_channel(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn semantic_color(theme: &Theme, token: SemanticColor) -> Color {
    theme.semantic_color(token)
}

fn semantic_foreground(theme: &Theme, token: SemanticColor) -> Color {
    theme.semantic_foreground(token)
}

fn theme_variant(theme: &Theme) -> ThemeVariant {
    theme.variant()
}

fn accent_family(color: AccentColor, scale: Scale) -> TwillColor {
    match color {
        AccentColor::Gray => TwillColor::neutral(scale),
        AccentColor::Gold | AccentColor::Yellow | AccentColor::Amber => TwillColor::amber(scale),
        AccentColor::Bronze | AccentColor::Brown => TwillColor::orange(scale),
        AccentColor::Orange | AccentColor::Tomato => TwillColor::orange(scale),
        AccentColor::Red | AccentColor::Ruby | AccentColor::Crimson => TwillColor::red(scale),
        AccentColor::Pink | AccentColor::Plum => TwillColor::pink(scale),
        AccentColor::Purple | AccentColor::Violet | AccentColor::Iris => TwillColor::violet(scale),
        AccentColor::Indigo => TwillColor::indigo(scale),
        AccentColor::Blue => TwillColor::blue(scale),
        AccentColor::Cyan | AccentColor::Sky => TwillColor::sky(scale),
        AccentColor::Teal | AccentColor::Jade | AccentColor::Mint => TwillColor::teal(scale),
        AccentColor::Green | AccentColor::Grass => TwillColor::green(scale),
        AccentColor::Lime => TwillColor::lime(scale),
    }
}

fn accent_scale_color(theme: &Theme, color: AccentColor, scale: Scale) -> Color {
    let _ = theme;
    to_color(accent_family(color, scale))
}

fn accent_contrast_color(theme: &Theme, color: AccentColor, scale: Scale) -> Color {
    let preferred = accent_family(color, scale).compute().preferred_text_color();
    let variant = theme_variant(theme);

    match preferred {
        twill::prelude::SpecialColor::Black => Color::BLACK,
        twill::prelude::SpecialColor::White => Color::WHITE,
        twill::prelude::SpecialColor::Transparent | twill::prelude::SpecialColor::Current => {
            if variant.is_dark() {
                Color::WHITE
            } else {
                Color::BLACK
            }
        }
    }
}

fn accent_solid_scale(theme: &Theme) -> Scale {
    if theme_variant(theme).is_dark() {
        Scale::S500
    } else {
        Scale::S600
    }
}

fn accent_solid_hover_scale(theme: &Theme) -> Scale {
    if theme_variant(theme).is_dark() {
        Scale::S400
    } else {
        Scale::S700
    }
}

fn accent_solid_active_scale(theme: &Theme) -> Scale {
    if theme_variant(theme).is_dark() {
        Scale::S300
    } else {
        Scale::S800
    }
}

fn accent_soft_background_scale(theme: &Theme) -> Scale {
    if theme_variant(theme).is_dark() {
        Scale::S950
    } else {
        Scale::S100
    }
}

fn accent_soft_hover_scale(theme: &Theme) -> Scale {
    if theme_variant(theme).is_dark() {
        Scale::S900
    } else {
        Scale::S200
    }
}

fn accent_soft_foreground_scale(theme: &Theme) -> Scale {
    if theme_variant(theme).is_dark() {
        Scale::S200
    } else {
        Scale::S700
    }
}

fn destructive_hover_color(theme: &Theme) -> Color {
    if theme_variant(theme).is_dark() {
        to_color(TwillColor::red(Scale::S500))
    } else {
        to_color(TwillColor::red(Scale::S700))
    }
}

fn destructive_active_color(theme: &Theme) -> Color {
    if theme_variant(theme).is_dark() {
        to_color(TwillColor::red(Scale::S400))
    } else {
        to_color(TwillColor::red(Scale::S800))
    }
}

fn current_text_for_state(current: Color, fallback: Color) -> Color {
    let alpha = 0.85;
    Color {
        r: current.r * alpha + fallback.r * (1.0 - alpha),
        g: current.g * alpha + fallback.g * (1.0 - alpha),
        b: current.b * alpha + fallback.b * (1.0 - alpha),
        a: 1.0,
    }
}

fn twill_radius(theme: &Theme, radius: ButtonRadius) -> BorderRadius {
    let _ = theme;
    match radius {
        ButtonRadius::None => BorderRadius::None,
        ButtonRadius::Small => BorderRadius::Sm,
        ButtonRadius::Medium => BorderRadius::Md,
        ButtonRadius::Large => BorderRadius::Lg,
        ButtonRadius::Full => BorderRadius::Full,
    }
}

impl ButtonSize {
    fn control_height(self) -> f32 {
        match self {
            ButtonSize::Size0 => 24.0,
            ButtonSize::Size1 => 32.0,
            ButtonSize::Size2 => 36.0,
            ButtonSize::Size3 => 40.0,
            ButtonSize::Size4 => 48.0,
        }
    }

    fn label_text_size(self) -> u16 {
        match self {
            ButtonSize::Size0 => 12,
            ButtonSize::Size1 | ButtonSize::Size2 | ButtonSize::Size3 => 14,
            ButtonSize::Size4 => 16,
        }
    }

    fn default_padding(self) -> Padding {
        match self {
            ButtonSize::Size0 => Padding::symmetric(Spacing::S1, Spacing::S2),
            ButtonSize::Size1 => Padding::symmetric(Spacing::S1_5, Spacing::S3),
            ButtonSize::Size2 => Padding::symmetric(Spacing::S2, Spacing::S4),
            ButtonSize::Size3 => Padding::symmetric(Spacing::S2_5, Spacing::S6),
            ButtonSize::Size4 => Padding::symmetric(Spacing::S3, Spacing::S7),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    enum Message {
        Pressed,
    }

    #[test]
    fn builder_updates_semantic_fields() {
        let button: Button<'_, Message> = Button::text("Save", &Theme::light())
            .variant(ButtonVariant::Outline)
            .size(ButtonSize::Size3)
            .radius(ButtonRadius::Large)
            .color(AccentColor::Blue)
            .loading(true)
            .disabled(true);

        assert!(matches!(button.content, ButtonContent::Label(_)));
        assert_eq!(button.variant, ButtonVariant::Outline);
        assert_eq!(button.size, ButtonSize::Size3);
        assert_eq!(button.radius, Some(ButtonRadius::Large));
        assert_eq!(button.color, AccentColor::Blue);
        assert!(button.loading);
        assert!(button.disabled);
    }

    #[test]
    fn text_and_generic_buttons_convert_to_elements() {
        let theme = Theme::light();

        let _: Element<'_, Message> = Button::new(container("Custom"), &theme)
            .on_press(Message::Pressed)
            .into();

        let _: Element<'_, Message> = Button::text("Save", &theme)
            .on_press(Message::Pressed)
            .into();
    }

    #[test]
    fn disabled_style_uses_muted_surface() {
        let style = resolve_button_style(
            &Theme::light(),
            ButtonVariant::Default,
            ButtonSize::Size2,
            None,
            AccentColor::Blue,
            true,
            button_widget::Status::Disabled,
        );

        assert!(style.background.is_some());
        assert_eq!(style.border.width, 1.0);
    }

    #[test]
    fn variant_mapping_matches_expected_surface_rules() {
        let theme = Theme::light();

        let default_style = resolve_button_style(
            &theme,
            ButtonVariant::Default,
            ButtonSize::Size2,
            None,
            AccentColor::Blue,
            false,
            button_widget::Status::Active,
        );
        assert!(default_style.background.is_some());
        assert_eq!(default_style.border.width, 0.0);

        let outline_style = resolve_button_style(
            &theme,
            ButtonVariant::Outline,
            ButtonSize::Size2,
            None,
            AccentColor::Blue,
            false,
            button_widget::Status::Active,
        );
        assert_eq!(outline_style.border.width, 1.0);

        let link_style = resolve_button_style(
            &theme,
            ButtonVariant::Link,
            ButtonSize::Size2,
            None,
            AccentColor::Blue,
            false,
            button_widget::Status::Active,
        );
        assert!(link_style.background.is_none());
    }
}
