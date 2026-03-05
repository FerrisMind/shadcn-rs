//! Kbd (Keyboard) component for displaying keyboard shortcuts and commands.
//!
//! Inspired by Radix UI Themes Kbd and shadcn/ui Kbd components.
//!
//! # Example
//! ```rust,ignore
//! use iced_shadcn::{KbdProps, KbdSize, KbdGroupProps, kbd, kbd_group};
//!
//! // Single kbd
//! let kbd_element = kbd("Ctrl", KbdProps::new(), &theme);
//!
//! // Group of kbds
//! let group = kbd_group(
//!     vec![
//!         kbd("Ctrl", KbdProps::new(), &theme),
//!         kbd("K", KbdProps::new(), &theme),
//!     ],
//!     KbdGroupProps::new(),
//! );
//! ```

use iced::border::Border;
use iced::widget::{row, text as text_widget};
use iced::{Background, Color, Element, Length};

use crate::theme::Theme;

/// Size variants for the Kbd component.
/// Maps to Radix UI Themes size scale (1-9).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum KbdSize {
    /// Extra small size (size 1)
    Size1,
    /// Small size (size 2)
    #[default]
    Size2,
    /// Default size (size 3)
    Size3,
    /// Medium size (size 4)
    Size4,
    /// Large size (size 5)
    Five,
    /// Extra large size (size 6)
    Six,
    /// 2x Extra large size (size 7)
    Seven,
    /// 3x Extra large size (size 8)
    Eight,
    /// 4x Extra large size (size 9)
    Nine,
}

impl KbdSize {
    /// Returns the font size in pixels for this size variant.
    fn font_size(self) -> u32 {
        match self {
            KbdSize::Size1 => 10,
            KbdSize::Size2 => 11,
            KbdSize::Size3 => 12,
            KbdSize::Size4 => 13,
            KbdSize::Five => 14,
            KbdSize::Six => 16,
            KbdSize::Seven => 18,
            KbdSize::Eight => 20,
            KbdSize::Nine => 24,
        }
    }

    /// Returns the padding [vertical, horizontal] for this size variant.
    fn padding(self) -> [f32; 2] {
        match self {
            KbdSize::Size1 => [1.0, 4.0],
            KbdSize::Size2 => [2.0, 6.0],
            KbdSize::Size3 => [3.0, 8.0],
            KbdSize::Size4 => [4.0, 10.0],
            KbdSize::Five => [5.0, 12.0],
            KbdSize::Six => [6.0, 14.0],
            KbdSize::Seven => [7.0, 16.0],
            KbdSize::Eight => [8.0, 18.0],
            KbdSize::Nine => [9.0, 20.0],
        }
    }

    /// Returns the minimum width for this size variant.
    fn min_width(self) -> f32 {
        match self {
            KbdSize::Size1 => 16.0,
            KbdSize::Size2 => 20.0,
            KbdSize::Size3 => 24.0,
            KbdSize::Size4 => 28.0,
            KbdSize::Five => 32.0,
            KbdSize::Six => 36.0,
            KbdSize::Seven => 40.0,
            KbdSize::Eight => 44.0,
            KbdSize::Nine => 48.0,
        }
    }

    /// Returns the border radius for this size variant.
    fn radius(self, theme: &Theme) -> f32 {
        match self {
            KbdSize::Size1 => theme.radius.sm * 0.5,
            KbdSize::Size2 => theme.radius.sm * 0.6,
            KbdSize::Size3 => theme.radius.sm * 0.7,
            KbdSize::Size4 => theme.radius.sm,
            KbdSize::Five => theme.radius.md * 0.8,
            KbdSize::Six => theme.radius.md,
            KbdSize::Seven => theme.radius.md * 1.2,
            KbdSize::Eight => theme.radius.lg * 0.9,
            KbdSize::Nine => theme.radius.lg,
        }
    }
}

/// Props for the Kbd component.
#[derive(Clone, Copy, Debug)]
pub struct KbdProps {
    /// Size variant of the kbd
    pub size: KbdSize,
    /// Whether the kbd is interactive (can be clicked/hovered)
    pub interactive: bool,
    /// Custom background color (optional)
    pub background: Option<Color>,
    /// Custom text color (optional)
    pub color: Option<Color>,
    /// Custom border color (optional)
    pub border_color: Option<Color>,
    /// Whether to show a shadow (3D keyboard effect)
    pub shadow: bool,
    /// Custom font (defaults to MONOSPACE; use `Font::with_name("lucide")` for icons)
    pub font: Option<iced::Font>,
}

impl Default for KbdProps {
    fn default() -> Self {
        Self {
            size: KbdSize::default(),
            interactive: false,
            background: None,
            color: None,
            border_color: None,
            shadow: true,
            font: None,
        }
    }
}

impl KbdProps {
    /// Creates a new KbdProps with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the size variant.
    pub fn size(mut self, size: KbdSize) -> Self {
        self.size = size;
        self
    }

    /// Sets whether the kbd is interactive.
    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    /// Sets a custom background color.
    pub fn background(mut self, background: Color) -> Self {
        self.background = Some(background);
        self
    }

    /// Sets a custom text color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets a custom border color.
    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border_color = Some(border_color);
        self
    }

    /// Sets whether to show a shadow.
    pub fn shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }

    /// Sets a custom font (e.g., `Font::with_name("lucide")` for icon glyphs).
    pub fn font(mut self, font: iced::Font) -> Self {
        self.font = Some(font);
        self
    }
}

/// Props for the KbdGroup component.
#[derive(Clone, Copy, Debug)]
pub struct KbdGroupProps {
    /// Gap between items in the group
    pub gap: f32,
    /// Whether to add a separator between items
    pub separator: Option<&'static str>,
    /// Optional custom separator color
    pub separator_color: Option<Color>,
}

impl Default for KbdGroupProps {
    fn default() -> Self {
        Self {
            gap: 4.0,
            separator: None,
            separator_color: None,
        }
    }
}

impl KbdGroupProps {
    /// Creates a new KbdGroupProps with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the gap between items.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Sets a separator string between items.
    pub fn separator(mut self, separator: &'static str) -> Self {
        self.separator = Some(separator);
        self
    }

    /// Sets a custom separator color.
    pub fn separator_color(mut self, separator_color: Color) -> Self {
        self.separator_color = Some(separator_color);
        self
    }
}

/// Creates a Kbd (keyboard) element displaying a keyboard key or shortcut.
///
/// # Arguments
/// * `label` - The text to display inside the kbd
/// * `props` - Configuration props for the kbd
/// * `theme` - The current theme
pub fn kbd<'a, Message: 'a>(
    label: impl Into<String>,
    props: KbdProps,
    theme: &'a Theme,
) -> Element<'a, Message> {
    Element::new(KbdWidget::new(label.into(), props, theme))
}

use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::text;
use iced::advanced::widget::Tree;
use iced::advanced::{Layout, Widget};
use iced::{Font, Rectangle, Size, alignment};

struct KbdWidget {
    label: String,
    props: KbdProps,
    theme: Theme,
}

impl KbdWidget {
    fn new(label: String, props: KbdProps, theme: &Theme) -> Self {
        Self {
            label,
            props,
            theme: theme.clone(),
        }
    }
}

impl<Message, AppTheme, Renderer> Widget<Message, AppTheme, Renderer> for KbdWidget
where
    Renderer: renderer::Renderer + text::Renderer<Font = Font>,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        let padding = self.props.size.padding();
        let font_size = self.props.size.font_size() as f32;

        // Approximate text width: monospace char_width ≈ font_size * 0.6
        let char_count = self.label.chars().count() as f32;
        let text_width = char_count * font_size * 0.6;
        let text_height = font_size * 1.3; // line_height factor

        let total_width = text_width + padding[1] * 2.0;
        let total_height = text_height + padding[0] * 2.0;
        let min_width = self.props.size.min_width();

        layout::Node::new(Size::new(total_width.max(min_width), total_height))
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &AppTheme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let palette = &self.theme.palette;

        let background = self
            .props
            .background
            .unwrap_or_else(|| mix(palette.muted, palette.background, 0.6));
        let text_color = self.props.color.unwrap_or(palette.muted_foreground);
        let border_color = self.props.border_color.unwrap_or(palette.border);
        let radius = self.props.size.radius(&self.theme);

        // Draw background
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: radius.into(),
                },
                shadow: if self.props.shadow {
                    iced::Shadow {
                        color: apply_opacity(palette.foreground, 0.05),
                        offset: iced::Vector::new(0.0, 1.0),
                        blur_radius: 2.0,
                    }
                } else {
                    iced::Shadow::default()
                },
                ..renderer::Quad::default()
            },
            Background::Color(background),
        );

        // Draw text
        let font_size: iced::Pixels = (self.props.size.font_size() as f32).into();
        let font = self.props.font.unwrap_or(Font::MONOSPACE);

        renderer.fill_text(
            text::Text {
                content: self.label.clone(),
                font,
                size: font_size,
                line_height: text::LineHeight::Absolute(font_size),
                bounds: bounds.size(),
                align_x: text::Alignment::Center,
                align_y: alignment::Vertical::Center,
                shaping: text::Shaping::Advanced,
                wrapping: text::Wrapping::None,
            },
            bounds.center(),
            text_color,
            bounds,
        );
    }

    // No operate() → text is not selectable
}

impl<'a, Message, AppTheme, Renderer> From<KbdWidget> for Element<'a, Message, AppTheme, Renderer>
where
    Renderer: renderer::Renderer + text::Renderer<Font = Font> + 'a,
    Message: 'a,
{
    fn from(widget: KbdWidget) -> Element<'a, Message, AppTheme, Renderer> {
        Element::new(widget)
    }
}

/// Creates a KbdGroup element for grouping multiple Kbd elements.
///
/// # Arguments
/// * `items` - Vector of Kbd elements to group
/// * `props` - Configuration props for the group
pub fn kbd_group<'a, Message: 'a>(
    items: Vec<Element<'a, Message>>,
    props: &KbdGroupProps,
) -> Element<'a, Message> {
    if items.is_empty() {
        return row([]).into();
    }

    let gap = props.gap;

    if let Some(separator) = props.separator {
        let separator_color = props
            .separator_color
            .unwrap_or(Color::from_rgb(0.5, 0.5, 0.5));
        // Build row with separators
        let mut elements: Vec<Element<'a, Message>> = Vec::with_capacity(items.len() * 2 - 1);
        for (i, item) in items.into_iter().enumerate() {
            if i > 0 {
                elements.push(
                    text_widget(separator)
                        .size(12)
                        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                            color: Some(separator_color),
                        })
                        .into(),
                );
            }
            elements.push(item);
        }
        row(elements).spacing(gap).into()
    } else {
        row(items).spacing(gap).into()
    }
}

/// Creates a KbdGroup with a standard "+" separator for keyboard shortcuts.
///
/// # Arguments
/// * `labels` - Vector of key labels (e.g., vec!["Ctrl", "K"])
/// * `kbd_props` - Props applied to each Kbd element
/// * `theme` - The current theme
pub fn kbd_shortcut<'a, Message: 'a>(
    labels: Vec<&str>,
    kbd_props: KbdProps,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let items: Vec<Element<'a, Message>> = labels
        .into_iter()
        .map(|label| kbd(label, kbd_props, theme))
        .collect();

    let group_props = KbdGroupProps::new().separator("+");
    kbd_group(items, &group_props)
}

use crate::tokens::mix;

/// Helper function to apply opacity to a color.
fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kbd_size_default() {
        let size = KbdSize::default();
        assert_eq!(size, KbdSize::Size2);
    }

    #[test]
    fn test_kbd_size_font_sizes() {
        assert_eq!(KbdSize::Size1.font_size(), 10);
        assert_eq!(KbdSize::Size2.font_size(), 11);
        assert_eq!(KbdSize::Size3.font_size(), 12);
        assert_eq!(KbdSize::Nine.font_size(), 24);
    }

    #[test]
    fn test_kbd_props_builder() {
        let props = KbdProps::new()
            .size(KbdSize::Size3)
            .interactive(true)
            .shadow(false);

        assert_eq!(props.size, KbdSize::Size3);
        assert!(props.interactive);
        assert!(!props.shadow);
    }

    #[test]
    fn test_kbd_group_props_builder() {
        let props = KbdGroupProps::new().gap(8.0).separator("then");

        assert_eq!(props.gap, 8.0);
        assert_eq!(props.separator, Some("then"));
    }

    #[test]
    fn test_mix_colors() {
        let red = Color::from_rgb(1.0, 0.0, 0.0);
        let blue = Color::from_rgb(0.0, 0.0, 1.0);
        let mixed = crate::tokens::mix(red, blue, 0.5);

        assert!((mixed.r - 0.5).abs() < 0.001);
        assert!((mixed.g - 0.0).abs() < 0.001);
        assert!((mixed.b - 0.5).abs() < 0.001);
    }
}
