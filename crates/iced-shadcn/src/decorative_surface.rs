use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::{Operation, Tree};
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::border::Border;
use iced::widget::{container, stack};
use iced::{Background, Color, Element, Event, Length, Padding, Rectangle, Shadow, Size, mouse};
use twill::prelude::SemanticColor;

use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecorativeSurfaceProps {
    pub width: Length,
    pub height: Length,
    pub padding: Padding,
    pub radius: Option<f32>,
    pub clip: bool,
    pub background: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: f32,
    pub shadow: Option<Shadow>,
    pub theme_background: bool,
    pub theme_border: bool,
    pub theme_shadow: bool,
}

impl Default for DecorativeSurfaceProps {
    fn default() -> Self {
        Self {
            width: Length::Shrink,
            height: Length::Shrink,
            padding: Padding::ZERO,
            radius: None,
            clip: true,
            background: None,
            border_color: None,
            border_width: 0.0,
            shadow: None,
            theme_background: false,
            theme_border: false,
            theme_shadow: false,
        }
    }
}

impl DecorativeSurfaceProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius.max(0.0));
        self
    }

    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    pub fn background(mut self, background: Color) -> Self {
        self.background = Some(background);
        self
    }

    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border_color = Some(border_color);
        self
    }

    pub fn border_width(mut self, border_width: f32) -> Self {
        self.border_width = border_width.max(0.0);
        self
    }

    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.shadow = Some(shadow);
        self
    }

    pub fn theme_background(mut self, enabled: bool) -> Self {
        self.theme_background = enabled;
        self
    }

    pub fn theme_border(mut self, border_width: f32) -> Self {
        self.theme_border = true;
        self.border_width = border_width.max(0.0);
        self
    }

    pub fn theme_shadow(mut self, enabled: bool) -> Self {
        self.theme_shadow = enabled;
        self
    }

    pub fn themed(mut self) -> Self {
        self.theme_background = true;
        self.theme_border = true;
        self.theme_shadow = true;
        self.border_width = self.border_width.max(1.0);
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LayerRole {
    Underlay,
    Base,
    Overlay,
}

#[derive(Clone, Debug, PartialEq)]
struct ResolvedDecorativeSurface {
    radius: f32,
    clip: bool,
    layer_roles: Vec<LayerRole>,
    background: Option<Color>,
    border_color: Color,
    border_width: f32,
    shadow: Shadow,
}

fn resolved_radius(props: DecorativeSurfaceProps, theme: &Theme) -> f32 {
    props.radius.unwrap_or(theme.radius.md).max(0.0)
}

fn resolved_background(props: DecorativeSurfaceProps, theme: &Theme) -> Option<Color> {
    props.background.or_else(|| {
        props
            .theme_background
            .then(|| theme.semantic_color(SemanticColor::Card))
    })
}

fn resolved_border_color(props: DecorativeSurfaceProps, theme: &Theme) -> Color {
    props
        .border_color
        .or_else(|| {
            props
                .theme_border
                .then(|| theme.semantic_color(SemanticColor::Border))
        })
        .unwrap_or(Color::TRANSPARENT)
}

fn resolved_border_width(props: DecorativeSurfaceProps) -> f32 {
    if props.border_color.is_some() {
        props.border_width
    } else if props.theme_border {
        props.border_width.max(1.0)
    } else {
        props.border_width
    }
}

fn default_surface_shadow(theme: &Theme) -> Shadow {
    let alpha = if theme.variant().is_dark() {
        0.34
    } else {
        0.14
    };

    Shadow {
        color: Color::from_rgba(0.0, 0.0, 0.0, alpha),
        offset: iced::Vector::new(0.0, 12.0),
        blur_radius: 28.0,
    }
}

fn resolved_shadow(props: DecorativeSurfaceProps, theme: &Theme) -> Shadow {
    props.shadow.unwrap_or_else(|| {
        if props.theme_shadow {
            default_surface_shadow(theme)
        } else {
            Default::default()
        }
    })
}

fn resolve_surface(
    underlays: usize,
    overlays: usize,
    props: DecorativeSurfaceProps,
    theme: &Theme,
) -> ResolvedDecorativeSurface {
    let mut layer_roles = Vec::with_capacity(underlays + overlays + 1);
    layer_roles.extend(std::iter::repeat_n(LayerRole::Underlay, underlays));
    layer_roles.push(LayerRole::Base);
    layer_roles.extend(std::iter::repeat_n(LayerRole::Overlay, overlays));

    ResolvedDecorativeSurface {
        radius: resolved_radius(props, theme),
        clip: props.clip,
        layer_roles,
        background: resolved_background(props, theme),
        border_color: resolved_border_color(props, theme),
        border_width: resolved_border_width(props),
        shadow: resolved_shadow(props, theme),
    }
}

fn surface_layer<'a, Message: Clone + 'a>(
    layer: Element<'a, Message>,
    clip: bool,
) -> Element<'a, Message> {
    container(DecorativeLayer::new(layer))
        .width(Length::Fill)
        .height(Length::Fill)
        .clip(clip)
        .into()
}

fn base_style(resolved: &ResolvedDecorativeSurface) -> container::Style {
    container::Style {
        background: resolved.background.map(Background::Color),
        border: Border {
            color: resolved.border_color,
            width: resolved.border_width,
            radius: resolved.radius.into(),
        },
        shadow: resolved.shadow,
        snap: true,
        ..container::Style::default()
    }
}

pub fn decorative_surface<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    underlays: Vec<Element<'a, Message>>,
    overlays: Vec<Element<'a, Message>>,
    props: DecorativeSurfaceProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let resolved = resolve_surface(underlays.len(), overlays.len(), props, theme);
    let width = props.width;
    let height = props.height;
    let padding = props.padding;
    let clip = resolved.clip;
    let base_style = base_style(&resolved);

    let base = container(content.into())
        .width(width)
        .height(height)
        .padding(padding)
        .clip(clip)
        .style(move |_iced_theme| base_style);

    let mut layered = stack(vec![base.into()]).clip(clip);

    for underlay in underlays {
        layered = layered.push_under(surface_layer(underlay, clip));
    }

    for overlay in overlays {
        layered = layered.push(surface_layer(overlay, clip));
    }

    layered.into()
}

struct DecorativeLayer<'a, Message> {
    content: Element<'a, Message>,
}

impl<'a, Message> DecorativeLayer<'a, Message> {
    fn new(content: Element<'a, Message>) -> Self {
        Self { content }
    }
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer> for DecorativeLayer<'_, Message>
where
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.content.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
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
            layout,
            cursor,
            viewport,
        );
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            mouse::Cursor::Unavailable,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        mouse::Interaction::None
    }
}

impl<'a, Message: Clone + 'a> From<DecorativeLayer<'a, Message>> for Element<'a, Message> {
    fn from(widget: DecorativeLayer<'a, Message>) -> Element<'a, Message> {
        Element::new(widget)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::new_api::{Button, ButtonVariant};

    #[test]
    fn decorative_surface_defaults_are_generic() {
        let props = DecorativeSurfaceProps::new();

        assert_eq!(props.width, Length::Shrink);
        assert_eq!(props.height, Length::Shrink);
        assert_eq!(props.padding, Padding::ZERO);
        assert_eq!(props.radius, None);
        assert!(props.clip);
        assert_eq!(props.background, None);
        assert_eq!(props.border_color, None);
        assert_eq!(props.border_width, 0.0);
        assert_eq!(props.shadow, None);
        assert!(!props.theme_background);
        assert!(!props.theme_border);
        assert!(!props.theme_shadow);
    }

    #[test]
    fn resolve_surface_preserves_underlay_base_overlay_order() {
        let theme = Theme::dark();
        let resolved = resolve_surface(2, 3, DecorativeSurfaceProps::new(), &theme);

        assert_eq!(
            resolved.layer_roles,
            vec![
                LayerRole::Underlay,
                LayerRole::Underlay,
                LayerRole::Base,
                LayerRole::Overlay,
                LayerRole::Overlay,
                LayerRole::Overlay,
            ]
        );
    }

    #[test]
    fn resolve_surface_without_decorations_keeps_base_layer_only() {
        let theme = Theme::light();
        let resolved = resolve_surface(0, 0, DecorativeSurfaceProps::new(), &theme);

        assert_eq!(resolved.layer_roles, vec![LayerRole::Base]);
    }

    #[test]
    fn explicit_radius_overrides_theme_radius_and_clip_is_preserved() {
        let theme = Theme::light();
        let props = DecorativeSurfaceProps::new().radius(24.0).clip(false);
        let resolved = resolve_surface(1, 1, props, &theme);

        assert_eq!(resolved.radius, 24.0);
        assert!(!resolved.clip);
    }

    #[test]
    fn theme_radius_is_used_when_radius_is_not_provided() {
        let theme = Theme::dark().with_radius(crate::tokens::Radius {
            sm: 4.0,
            md: 18.0,
            lg: 28.0,
        });
        let resolved = resolve_surface(1, 0, DecorativeSurfaceProps::new(), &theme);

        assert_eq!(resolved.radius, 18.0);
    }

    #[test]
    fn themed_surface_uses_theme_derived_defaults() {
        let theme = Theme::from_semantic_theme(
            twill::prelude::SemanticThemeVars::shadcn_neutral(),
            twill::prelude::ThemeVariant::Dark,
        )
        .with_radius(crate::tokens::Radius {
            sm: 6.0,
            md: 20.0,
            lg: 28.0,
        });
        let resolved = resolve_surface(1, 1, DecorativeSurfaceProps::new().themed(), &theme);

        assert_eq!(resolved.radius, 20.0);
        assert_eq!(
            resolved.background,
            Some(theme.semantic_color(SemanticColor::Card))
        );
        assert_eq!(
            resolved.border_color,
            theme.semantic_color(SemanticColor::Border)
        );
        assert_eq!(resolved.border_width, 1.0);
        assert_eq!(resolved.shadow, default_surface_shadow(&theme));
    }

    #[test]
    fn explicit_surface_values_override_theme_defaults() {
        let theme = Theme::dark();
        let shadow = Shadow {
            color: Color::from_rgba(1.0, 0.0, 0.0, 0.25),
            offset: iced::Vector::new(3.0, 4.0),
            blur_radius: 9.0,
        };
        let background = Color::from_rgb(0.2, 0.3, 0.4);
        let border = Color::from_rgb(0.7, 0.4, 0.2);
        let resolved = resolve_surface(
            0,
            2,
            DecorativeSurfaceProps::new()
                .themed()
                .background(background)
                .border_color(border)
                .border_width(3.0)
                .shadow(shadow),
            &theme,
        );

        assert_eq!(resolved.background, Some(background));
        assert_eq!(resolved.border_color, border);
        assert_eq!(resolved.border_width, 3.0);
        assert_eq!(resolved.shadow, shadow);
    }

    #[test]
    fn decorative_surface_wraps_component_content_with_theme_defaults() {
        let crate_theme = Theme::light();
        let api_theme = crate::new_api::Theme::light();
        let button = Button::text("Surface action", &api_theme)
            .variant(ButtonVariant::Default)
            .into_button();
        let surface: Element<'_, ()> = decorative_surface(
            button,
            vec![container("underlay").into()],
            vec![container("overlay").into()],
            DecorativeSurfaceProps::new().themed().padding(16),
            &crate_theme,
        );

        let _ = surface;
    }
}
