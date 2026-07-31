//! Style resolution for navigation-menu triggers, links, and viewport.

use crate::floating_surface::fill_floating_surface;
use crate::iced_compat::advanced::renderer::{self, Renderer as _};
use crate::iced_compat::widget::button;
use crate::iced_compat::{Background, Border, Color, Rectangle, Renderer, Shadow, Vector};
use crate::recipes::component_radius_px;
use crate::theme::Theme;
use shadcn_common::{
    NAVIGATION_MENU_DISABLED_OPACITY, NAVIGATION_MENU_OPEN_MUTED_ALPHA, NavigationMenuRecipe,
    navigation_menu_recipe,
};

use super::types::{NavigationMenuLinkProps, NavigationMenuLinkVariant, NavigationMenuMetrics};

/// Resolved visuals of the shared viewport / popup surface.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationMenuViewportStyle {
    /// Surface fill (`bg-popover`).
    pub background: Color,
    /// Content text color (`text-popover-foreground`).
    pub text_color: Color,
    /// Hairline ring color (`ring-foreground/N`).
    pub border_color: Color,
    /// Hairline ring width (`ring-1`).
    pub border_width: f32,
    /// Surface corner radius in px.
    pub radius: f32,
    /// Surface drop shadow.
    pub shadow: Shadow,
}

/// Resolved visuals for a trigger or link surface.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationMenuItemStyle {
    /// Fill behind the label.
    pub background: Color,
    /// Label / icon color.
    pub text_color: Color,
    /// Corner radius in px.
    pub radius: f32,
}

/// Backend-agnostic geometry recipe for the active style pack.
pub(super) fn recipe(theme: &Theme) -> NavigationMenuRecipe {
    navigation_menu_recipe(theme.style_id())
}

/// Resolves the viewport / popup surface style.
pub(super) fn resolve_viewport_style(theme: &Theme) -> NavigationMenuViewportStyle {
    let recipe = recipe(theme);
    let ring_alpha = if theme.is_dark() {
        recipe.ring_alpha_dark
    } else {
        recipe.ring_alpha
    };

    NavigationMenuViewportStyle {
        background: theme.palette.popover,
        text_color: theme.palette.popover_foreground,
        border_color: theme.palette.foreground.scale_alpha(ring_alpha),
        border_width: 1.0,
        radius: component_radius_px(theme, recipe.viewport_radius),
        shadow: Shadow {
            color: Color::BLACK.scale_alpha(recipe.shadow.alpha),
            offset: Vector::new(0.0, recipe.shadow.offset_y_px),
            blur_radius: recipe.shadow.blur_px,
        },
    }
}

/// Resolves without-viewport content panel style (same tokens, content radius).
pub(super) fn resolve_content_style(theme: &Theme) -> NavigationMenuViewportStyle {
    let mut style = resolve_viewport_style(theme);
    style.radius = component_radius_px(theme, recipe(theme).content_radius);
    style
}

/// Layout metrics from list props + recipe.
pub(super) fn metrics(list_padding: f32, gap: f32, theme: &Theme) -> NavigationMenuMetrics {
    let recipe = recipe(theme);
    NavigationMenuMetrics {
        list_padding,
        gap,
        line_gap: gap,
        indicator_size: 8.0,
        indicator_offset: 6.0,
        radius: component_radius_px(theme, recipe.trigger_radius),
    }
}

/// Resolves a top-level / in-content link surface.
pub(super) fn resolve_link_style(
    theme: &Theme,
    props: NavigationMenuLinkProps,
    status: button::Status,
    open: bool,
) -> NavigationMenuItemStyle {
    let recipe = recipe(theme);
    let radius = component_radius_px(
        theme,
        match props.variant {
            NavigationMenuLinkVariant::Trigger => recipe.trigger_radius,
            NavigationMenuLinkVariant::Default => recipe.link_radius,
        },
    );

    let muted = theme.palette.muted;
    let foreground = theme.palette.foreground;
    let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);

    let mut background = Color::TRANSPARENT;
    let mut text_color = foreground;

    if open || props.active {
        background = muted.scale_alpha(NAVIGATION_MENU_OPEN_MUTED_ALPHA);
    }

    if hovered && !props.disabled {
        background = muted;
    }

    if props.disabled {
        text_color = text_color.scale_alpha(NAVIGATION_MENU_DISABLED_OPACITY);
    }

    NavigationMenuItemStyle {
        background,
        text_color,
        radius,
    }
}

/// Paints the viewport / popup surface fill (CSS ring is painted after
/// content by the caller).
pub(super) fn paint_viewport_surface(
    renderer: &mut Renderer,
    bounds: Rectangle,
    style: NavigationMenuViewportStyle,
) {
    fill_floating_surface(
        renderer,
        bounds,
        style.background,
        style.radius,
        style.shadow,
    );
}

/// Paints the CSS `ring-1` hairline outside the viewport bounds.
pub(super) fn paint_viewport_ring(
    renderer: &mut Renderer,
    bounds: Rectangle,
    style: NavigationMenuViewportStyle,
) {
    crate::floating_surface::paint_outside_ring(
        renderer,
        bounds,
        style.border_color,
        style.border_width,
        style.radius,
    );
}

/// Paints a filled trigger / link quad when it has a non-transparent fill.
pub(super) fn paint_item_surface(
    renderer: &mut Renderer,
    bounds: Rectangle,
    style: NavigationMenuItemStyle,
) {
    if style.background.a <= f32::EPSILON {
        return;
    }

    renderer.fill_quad(
        renderer::Quad {
            bounds,
            border: Border {
                radius: style.radius.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            shadow: Shadow::default(),
            ..renderer::Quad::default()
        },
        Background::Color(style.background),
    );
}
