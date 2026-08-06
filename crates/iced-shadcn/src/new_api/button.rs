use std::fmt;

use iced::alignment::{Horizontal, Vertical};
use iced::border::Border;
use iced::widget::button as button_widget;
use iced::widget::text::{Fragment, IntoFragment, LineHeight, Rich, Span};
use iced::widget::{button as iced_button, container, hover, stack, text as iced_text};
use iced::{Background, Color, Element, Font, Length, Shadow, Vector};

use shadcn_common::AccentColor;
use twill::backends::iced::{to_border_radius, to_color, to_color_value};
use twill::prelude::{
    BackgroundColor, BorderColor, BorderRadius, BorderStyle, BorderWidth, Color as TwillColor,
    ColorValueToken, Padding, PaddingValue, SemanticColor, Shadow as TwillShadow, Spacing, Style,
    TextColor,
};
use twill::traits::Merge;

use crate::spinner::{Spinner, SpinnerSize, spinner};

use super::fonts::iced_font;
use super::theme::Theme;

/// Visual treatment of a [`Button`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ButtonVariant {
    /// Filled button using the theme primary color.
    #[default]
    Default,
    /// Soft destructive button using the theme destructive color.
    Destructive,
    /// Transparent button with a visible border.
    Outline,
    /// Filled button using the theme secondary surface.
    Secondary,
    /// Transparent button without a border.
    Ghost,
    /// Text-only button with a hover underline.
    Link,
    /// Filled button using the accent's soft surface.
    Soft,
    /// Elevated button using the background surface and a shadow.
    Surface,
}

/// Preset control size for a [`Button`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum ButtonSize {
    /// Extra-small control size.
    Size0,
    /// Small control size.
    Size1,
    /// Medium control size.
    #[default]
    Size2,
    /// Large control size.
    Size3,
    /// Extra-large control size.
    Size4,
}

/// Border radius preset for a [`Button`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum ButtonRadius {
    /// No corner radius.
    None,
    /// Small corner radius.
    Small,
    /// Medium corner radius.
    #[default]
    Medium,
    /// Large corner radius.
    Large,
    /// Fully rounded corners.
    Full,
}

/// Error returned when a button padding value cannot be represented by iced.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonBuildError {
    /// A custom-property padding variable has no value that iced can resolve.
    UnsupportedPaddingVariable {
        /// Name of the unsupported custom property.
        name: &'static str,
    },
    /// The CSS-like `auto` padding value has no iced equivalent.
    UnsupportedPaddingAuto,
}

impl fmt::Display for ButtonBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPaddingVariable { name } => write!(
                formatter,
                "padding variable `{name}` is not supported by iced-shadcn::new_api::Button"
            ),
            Self::UnsupportedPaddingAuto => formatter
                .write_str("padding value `auto` is not supported by iced-shadcn::new_api::Button"),
        }
    }
}

impl std::error::Error for ButtonBuildError {}

/// Experimental builder-first button API backed by `twill`.
///
/// The component semantics stay in `iced-shadcn`, while `twill` is used as the
/// internal utility-style composition layer.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn::new_api::{
///     AccentColor, Button, ButtonBuildError, ButtonSize, ButtonVariant, Theme,
/// };
/// use twill::prelude::{Padding, Spacing};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Save,
/// }
///
/// fn save_button(theme: &Theme) -> Result<Element<'_, Message>, ButtonBuildError> {
///     Ok(Button::text("Save", theme)
///         .variant(ButtonVariant::Default)
///         .size(ButtonSize::Size3)
///         .color(AccentColor::Blue)
///         .padding(Padding::all(Spacing::S4))?
///         .on_press(Message::Save)
///         .into())
/// }
/// ```
pub struct Button<'a, Message> {
    content: ButtonContent<'a, Message>,
    theme: &'a Theme,
    variant: ButtonVariant,
    size: ButtonSize,
    radius: Option<ButtonRadius>,
    /// `None` = theme primary; `Some` = accent overlay from `shadcn-common`.
    color: Option<AccentColor>,
    width: Length,
    height: Option<Length>,
    padding: Option<iced::Padding>,
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

impl<Message> fmt::Debug for Button<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.content {
            ButtonContent::Label(_) => "label",
            ButtonContent::Element(_) => "element",
            ButtonContent::Icon(_) => "icon",
        };

        formatter
            .debug_struct("Button")
            .field("content", &content)
            .field("theme", &self.theme)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("color", &self.color)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("full_width", &self.full_width)
            .field("loading", &self.loading)
            .field("disabled", &self.disabled)
            .field("on_press", &self.on_press.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Button<'a, Message> {
    /// Creates a new button from arbitrary content.
    ///
    /// `theme` is required because `iced-shadcn` styling is derived from crate
    /// theme tokens instead of `iced::Theme`.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(ButtonContent::Element(content.into()), theme)
    }

    /// Creates a text button.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(ButtonContent::Label(label.into_fragment()), theme)
    }

    /// Creates an icon button.
    pub fn icon(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(ButtonContent::Icon(content.into()), theme)
    }

    fn from_content(content: ButtonContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            variant: ButtonVariant::Default,
            size: ButtonSize::Size2,
            radius: None,
            color: None,
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

    /// Sets the visual treatment of the button.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the preset control size.
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the button corner radius.
    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Applies an accent color overlay to the button's theme tokens.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Alias for [`Button::color`] retained for semantic color APIs.
    pub fn tone(self, color: AccentColor) -> Self {
        self.color(color)
    }

    /// Use the theme primary (no per-button accent overlay).
    pub fn theme_primary(mut self) -> Self {
        self.color = None;
        self
    }

    /// Sets a custom button width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets a custom button height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets all supported sides of the button padding.
    ///
    /// `PaddingValue::Var(_)` cannot be resolved by iced and is rejected with
    /// [`ButtonBuildError::UnsupportedPaddingVariable`]. The same applies to
    /// [`Spacing::Auto`], which has no fixed-size iced representation.
    ///
    /// # Errors
    ///
    /// Returns [`ButtonBuildError`] when any padding side contains a custom
    /// variable or `auto` value. The button is returned unchanged on error.
    pub fn padding(mut self, padding: Padding) -> Result<Self, ButtonBuildError> {
        self.padding = Some(resolve_padding(padding)?);
        Ok(self)
    }

    /// Makes the button fill the available width.
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    /// Shows a spinner and disables the button while loading.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Disables the button while retaining its configured content.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the message emitted when the button is pressed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets or clears the message emitted when the button is pressed.
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
        let control_height_px = size.control_height(theme);
        let control_height = height.unwrap_or(Length::Fixed(control_height_px));
        let resolved_width =
            resolve_button_width(width, control_height, full_width, icon, control_height_px);

        let content = build_content(content, variant, size, loading, color, theme);
        // Fill the button's content box and center it inside the configured
        // control bounds.
        let content = build_wrapper(content, full_width, icon);
        let disabled_state = disabled || loading || on_press.is_none();
        let resolved_padding = padding.unwrap_or_else(|| {
            if icon {
                iced::Padding::ZERO
            } else {
                default_padding(size)
            }
        });

        let mut widget = iced_button(content)
            .padding(resolved_padding)
            .width(resolved_width)
            .height(control_height);

        if let Some(message) = on_press
            && !disabled_state
        {
            widget = widget.on_press(message);
        }

        widget.style(move |_iced_theme, status| {
            let mut style =
                resolve_button_style(theme, variant, size, radius, color, disabled_state, status);

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
    variant: ButtonVariant,
    size: ButtonSize,
    loading: bool,
    color: Option<AccentColor>,
    theme: &Theme,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let content = match content {
        ButtonContent::Label(label) => {
            let size_px = size.label_text_size();
            let font = iced_font(theme.font_pack().sans);
            let line_height = LineHeight::Absolute(f32::from(size_px).into());

            if variant == ButtonVariant::Link {
                // shadcn: `underline-offset-4 hover:underline`
                link_label(label, size_px, font)
            } else {
                iced_text(label)
                    .size(u32::from(size_px))
                    .font(font)
                    .line_height(line_height)
                    .into()
            }
        }
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

fn link_label<'a, Message: 'a>(
    label: Fragment<'a>,
    size_px: u16,
    font: Font,
) -> Element<'a, Message> {
    let size = f32::from(size_px);
    // Leave room under the glyphs — iced `hover` layers clip to layout bounds,
    // so a tight Absolute line-height would crop `Span::underline`.
    let line_height = LineHeight::Absolute((size + 3.0).into());

    let base = Rich::<(), Message>::with_spans(vec![Span::new(label.clone())])
        .size(size)
        .font(font)
        .line_height(line_height);
    let underlined = Rich::<(), Message>::with_spans(vec![Span::new(label).underline(true)])
        .size(size)
        .font(font)
        .line_height(line_height);

    // Fill the button content box so hover tracks the whole control, not just
    // the tight text metrics (padding / vertical centering still apply outside).
    container(hover(base, underlined))
        .width(Length::Shrink)
        .height(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn build_wrapper<'a, Message: 'a>(
    content: Element<'a, Message>,
    full_width: bool,
    icon: bool,
) -> Element<'a, Message> {
    let mut wrapper = container(content)
        .width(Length::Shrink)
        .height(Length::Fill)
        .align_y(Vertical::Center);

    if full_width || icon {
        wrapper = wrapper.width(Length::Fill).align_x(Horizontal::Center);
    }

    wrapper.into()
}

fn loading_overlay<'a, Message>(
    content: Element<'a, Message>,
    size: ButtonSize,
    color: Option<AccentColor>,
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

    let spinner_color = accent_text(theme, color);
    let spinner = spinner(Spinner::from_color(spinner_color).size(spinner_size));
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

fn resolve_padding(padding: Padding) -> Result<iced::Padding, ButtonBuildError> {
    let (top, right, bottom, left) = padding.sides();

    Ok(iced::Padding {
        top: top.map(padding_value_px).transpose()?.unwrap_or(0.0),
        right: right.map(padding_value_px).transpose()?.unwrap_or(0.0),
        bottom: bottom.map(padding_value_px).transpose()?.unwrap_or(0.0),
        left: left.map(padding_value_px).transpose()?.unwrap_or(0.0),
    })
}

fn padding_value_px(value: PaddingValue) -> Result<f32, ButtonBuildError> {
    match value {
        PaddingValue::Scale(scale) => Ok(match scale {
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
            Spacing::Auto => return Err(ButtonBuildError::UnsupportedPaddingAuto),
        }),
        PaddingValue::Px(px) => Ok(px.max(0.0)),
        PaddingValue::Rem(rem) => Ok((rem * 16.0).max(0.0)),
        PaddingValue::Var(name) => Err(ButtonBuildError::UnsupportedPaddingVariable {
            name: name.as_str(),
        }),
    }
}

fn resolve_button_style(
    theme: &Theme,
    variant: ButtonVariant,
    size: ButtonSize,
    radius: Option<ButtonRadius>,
    color: Option<AccentColor>,
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
    color: Option<AccentColor>,
) -> Style {
    let accent = accent_fill(theme, color);
    let accent_fg = accent_on_fill(theme, color);
    let accent_txt = accent_text(theme, color);
    let soft_bg = accent_soft_fill(theme, color);
    let soft_fg = accent_txt;

    let (base_bg, base_fg, border_color, border_width, shadow) = match variant {
        ButtonVariant::Default => (Some(accent), accent_fg, accent, BorderWidth::S0, None),
        ButtonVariant::Secondary => (
            Some(semantic_color(theme, SemanticColor::Secondary)),
            semantic_color(theme, SemanticColor::SecondaryForeground),
            semantic_color(theme, SemanticColor::Secondary),
            BorderWidth::S0,
            None,
        ),
        ButtonVariant::Destructive => {
            // shadcn: `bg-destructive/10 text-destructive` (dark: `/20`)
            let destructive = semantic_color(theme, SemanticColor::Destructive);
            (
                Some(destructive_soft_fill(
                    theme,
                    destructive_soft_alpha(theme, SoftState::Base),
                )),
                destructive,
                Color::TRANSPARENT,
                BorderWidth::S0,
                None,
            )
        }
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
    color: Option<AccentColor>,
    current_text: Color,
) -> Style {
    match variant {
        ButtonVariant::Default => Style::new().background_token(background_token(shift_toward(
            accent_fill(theme, color),
            theme.is_dark(),
            0.12,
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
        ButtonVariant::Destructive => Style::new()
            .background_token(background_token(destructive_soft_fill(
                theme,
                destructive_soft_alpha(theme, SoftState::Hover),
            )))
            .text_color_token(text_color_token(semantic_color(
                theme,
                SemanticColor::Destructive,
            ))),
        ButtonVariant::Soft | ButtonVariant::Surface => {
            Style::new().background_token(background_token(shift_toward(
                accent_soft_fill(theme, color),
                theme.is_dark(),
                0.1,
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

fn pressed_state(theme: &Theme, variant: ButtonVariant, color: Option<AccentColor>) -> Style {
    match variant {
        ButtonVariant::Default => Style::new().background_token(background_token(shift_toward(
            accent_fill(theme, color),
            theme.is_dark(),
            0.22,
        ))),
        ButtonVariant::Secondary => Style::new().background_token(background_token(
            semantic_color(theme, SemanticColor::Muted),
        )),
        ButtonVariant::Destructive => Style::new()
            .background_token(background_token(destructive_soft_fill(
                theme,
                destructive_soft_alpha(theme, SoftState::Pressed),
            )))
            .text_color_token(text_color_token(semantic_color(
                theme,
                SemanticColor::Destructive,
            ))),
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

fn accent_fill(theme: &Theme, color: Option<AccentColor>) -> Color {
    match color {
        None => theme.palette.primary,
        Some(accent) => theme.color_with_accent(accent, SemanticColor::Primary),
    }
}

fn accent_on_fill(theme: &Theme, color: Option<AccentColor>) -> Color {
    match color {
        None => theme.palette.primary_foreground,
        Some(accent) => theme.color_with_accent(accent, SemanticColor::PrimaryForeground),
    }
}

fn accent_text(theme: &Theme, color: Option<AccentColor>) -> Color {
    match color {
        None => theme.palette.primary,
        Some(accent) => theme.color_with_accent(accent, SemanticColor::Primary),
    }
}

fn accent_soft_fill(theme: &Theme, color: Option<AccentColor>) -> Color {
    match color {
        None => theme.palette.secondary,
        Some(accent) => theme.color_with_accent(accent, SemanticColor::Secondary),
    }
}

#[derive(Clone, Copy)]
enum SoftState {
    Base,
    Hover,
    Pressed,
}

/// shadcn destructive button: `bg-destructive/10` (dark `/20`), hover `/20` (dark `/30`).
fn destructive_soft_alpha(theme: &Theme, state: SoftState) -> f32 {
    match (theme.is_dark(), state) {
        (false, SoftState::Base) => 0.10,
        (true, SoftState::Base) => 0.20,
        (false, SoftState::Hover) => 0.20,
        (true, SoftState::Hover) => 0.30,
        (false, SoftState::Pressed) => 0.25,
        (true, SoftState::Pressed) => 0.35,
    }
}

fn destructive_soft_fill(theme: &Theme, alpha: f32) -> Color {
    mix_color(
        semantic_color(theme, SemanticColor::Background),
        semantic_color(theme, SemanticColor::Destructive),
        alpha,
    )
}

fn mix_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

/// Shift toward black in light mode / toward white in dark mode.
fn shift_toward(color: Color, dark: bool, amount: f32) -> Color {
    if dark {
        mix_color(color, Color::WHITE, amount)
    } else {
        mix_color(color, Color::BLACK, amount)
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
    match radius {
        ButtonRadius::None => BorderRadius::None,
        ButtonRadius::Small => theme.style.twill_radius_sm,
        ButtonRadius::Medium => theme.style.twill_radius_md,
        ButtonRadius::Large => theme.style.twill_radius_lg,
        ButtonRadius::Full => BorderRadius::Full,
    }
}

impl ButtonSize {
    fn control_height(self, theme: &Theme) -> f32 {
        match self {
            ButtonSize::Size0 => theme.style.control_height_sm_px - 8.0,
            ButtonSize::Size1 => theme.style.control_height_sm_px,
            ButtonSize::Size2 => theme.style.control_height_md_px,
            ButtonSize::Size3 => theme.style.control_height_lg_px,
            ButtonSize::Size4 => theme.style.control_height_lg_px + 8.0,
        }
        .max(0.0)
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

fn default_padding(size: ButtonSize) -> iced::Padding {
    let (vertical, horizontal) = match size {
        ButtonSize::Size0 => (4.0, 8.0),
        ButtonSize::Size1 => (6.0, 12.0),
        ButtonSize::Size2 => (8.0, 16.0),
        ButtonSize::Size3 => (10.0, 24.0),
        ButtonSize::Size4 => (12.0, 28.0),
    };

    iced::Padding {
        top: vertical,
        right: horizontal,
        bottom: vertical,
        left: horizontal,
    }
}

fn resolve_button_width(
    width: Length,
    height: Length,
    full_width: bool,
    icon: bool,
    default_height: f32,
) -> Length {
    if full_width {
        Length::Fill
    } else if icon {
        match height {
            Length::Fixed(height) => Length::Fixed(height),
            _ => Length::Fixed(default_height),
        }
    } else {
        width
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use twill::prelude::PaddingVar;

    #[derive(Clone, Debug)]
    enum Message {
        Pressed,
    }

    #[test]
    fn builder_updates_semantic_fields() {
        let theme = Theme::light();
        let button: Button<'_, Message> = Button::text("Save", &theme)
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
        assert_eq!(button.color, Some(AccentColor::Blue));
        assert!(button.loading);
        assert!(button.disabled);
        assert!(std::ptr::eq(button.theme, &theme));
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
            Some(AccentColor::Blue),
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
            Some(AccentColor::Blue),
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
            Some(AccentColor::Blue),
            false,
            button_widget::Status::Active,
        );
        assert_eq!(outline_style.border.width, 1.0);

        let link_style = resolve_button_style(
            &theme,
            ButtonVariant::Link,
            ButtonSize::Size2,
            None,
            Some(AccentColor::Blue),
            false,
            button_widget::Status::Active,
        );
        assert!(link_style.background.is_none());
    }

    #[test]
    fn padding_maps_all_four_sides() {
        let padding = Padding::individual_value(
            PaddingValue::Px(1.0),
            PaddingValue::Px(2.0),
            PaddingValue::Px(3.0),
            PaddingValue::Px(4.0),
        );

        let resolved = resolve_padding(padding).expect("pixel padding is supported");

        assert_eq!(resolved.top, 1.0);
        assert_eq!(resolved.right, 2.0);
        assert_eq!(resolved.bottom, 3.0);
        assert_eq!(resolved.left, 4.0);
    }

    #[test]
    fn padding_builder_stores_resolved_padding() {
        let theme = Theme::light();
        let button: Button<'_, Message> = Button::text("Save", &theme)
            .padding(Padding::individual(
                Spacing::S1,
                Spacing::S2,
                Spacing::S3,
                Spacing::S4,
            ))
            .expect("scale padding is supported");

        assert_eq!(
            button.padding,
            Some(iced::Padding {
                top: 4.0,
                right: 8.0,
                bottom: 12.0,
                left: 16.0,
            })
        );
    }

    #[test]
    fn padding_variable_returns_a_descriptive_error() {
        let theme = Theme::light();
        let error = Button::<Message>::text("Save", &theme)
            .padding(Padding::individual_value(
                PaddingValue::Var(PaddingVar::new("--button-padding")),
                PaddingValue::Px(2.0),
                PaddingValue::Px(3.0),
                PaddingValue::Px(4.0),
            ))
            .expect_err("padding variables are unsupported");

        assert_eq!(
            error,
            ButtonBuildError::UnsupportedPaddingVariable {
                name: "--button-padding"
            }
        );
        assert!(error.to_string().contains("--button-padding"));
    }

    #[test]
    fn padding_auto_returns_a_descriptive_error() {
        let theme = Theme::light();
        let error = Button::<Message>::text("Save", &theme)
            .padding(Padding::all(Spacing::Auto))
            .expect_err("auto padding is unsupported");

        assert_eq!(error, ButtonBuildError::UnsupportedPaddingAuto);
        assert!(error.to_string().contains("auto"));
    }

    #[test]
    fn icon_button_uses_custom_fixed_height_for_both_dimensions() {
        let resolved = resolve_button_width(Length::Shrink, Length::Fixed(72.0), false, true, 36.0);

        assert_eq!(resolved, Length::Fixed(72.0));
    }

    #[test]
    fn button_sizes_never_resolve_to_negative_heights() {
        let mut theme = Theme::light();
        theme.style.control_height_sm_px = 4.0;
        theme.style.control_height_md_px = -1.0;
        theme.style.control_height_lg_px = -2.0;

        for size in [
            ButtonSize::Size0,
            ButtonSize::Size1,
            ButtonSize::Size2,
            ButtonSize::Size3,
            ButtonSize::Size4,
        ] {
            assert!(size.control_height(&theme) >= 0.0);
        }
    }

    #[test]
    fn debug_does_not_require_message_debug() {
        struct NoDebugMessage;

        let theme = Theme::light();
        let button = Button::<NoDebugMessage>::text("Save", &theme);
        let debug = format!("{button:?}");

        assert!(debug.contains("Button"));
        assert!(debug.contains("label"));
    }

    #[test]
    fn configuration_enums_support_hashing_and_expected_order() {
        fn hash<T: Hash>(value: &T) -> u64 {
            let mut hasher = DefaultHasher::new();
            value.hash(&mut hasher);
            hasher.finish()
        }

        let _ = hash(&ButtonVariant::Default);
        let _ = hash(&ButtonSize::Size2);
        let _ = hash(&ButtonRadius::Medium);
        assert!(ButtonSize::Size0 < ButtonSize::Size4);
        assert!(ButtonRadius::None < ButtonRadius::Full);
    }

    #[test]
    fn tone_is_an_alias_for_color() {
        let theme = Theme::light();
        let button: Button<'_, Message> = Button::text("Save", &theme).tone(AccentColor::Blue);

        assert_eq!(button.color, Some(AccentColor::Blue));
    }

    #[test]
    fn states_dimensions_and_style_override_are_configurable() {
        let theme = Theme::light();
        let button = Button::text("Save", &theme)
            .loading(true)
            .disabled(true)
            .full_width()
            .width(Length::Fixed(240.0))
            .height(Length::Fixed(48.0))
            .style_override(|mut style, _| {
                style.text_color = Color::from_rgb(1.0, 0.0, 1.0);
                style
            })
            .on_press(Message::Pressed);

        assert!(button.loading);
        assert!(button.disabled);
        assert!(button.full_width);
        assert_eq!(button.width, Length::Fixed(240.0));
        assert_eq!(button.height, Some(Length::Fixed(48.0)));
        assert!(button.style_override.is_some());

        let _ = button.into_button();
    }

    #[test]
    fn all_variants_resolve_in_light_and_dark_themes() {
        for theme in [Theme::light(), Theme::dark()] {
            for variant in [
                ButtonVariant::Default,
                ButtonVariant::Destructive,
                ButtonVariant::Outline,
                ButtonVariant::Secondary,
                ButtonVariant::Ghost,
                ButtonVariant::Link,
                ButtonVariant::Soft,
                ButtonVariant::Surface,
            ] {
                for status in [
                    button_widget::Status::Active,
                    button_widget::Status::Hovered,
                    button_widget::Status::Pressed,
                    button_widget::Status::Disabled,
                ] {
                    let style = resolve_button_style(
                        &theme,
                        variant,
                        ButtonSize::Size2,
                        None,
                        Some(AccentColor::Blue),
                        status == button_widget::Status::Disabled,
                        status,
                    );
                    assert!(style.text_color.a.is_finite());
                }
            }
        }
    }
}
