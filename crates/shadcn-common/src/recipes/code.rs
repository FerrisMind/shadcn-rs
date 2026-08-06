//! Code-block recipes from the shadcn-svelte-extra `Code` component.
//!
//! The web component is a styled frame (`rounded-lg border`) embedding a
//! copy button and an optional expand button; the geometry below is shared
//! by the iced and egui backends.

use crate::style::StyleId;

use super::ComponentRadius;

/// Geometry recipe for a highlighted code block frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CodeRecipe {
    /// `py-4` vertical padding of the `<pre>` block.
    pub pad_y_px: f32,
    /// `px-4` horizontal padding when line numbers are hidden.
    pub pad_x_px: f32,
    /// `px-2` horizontal padding when line numbers are shown.
    pub pad_x_with_numbers_px: f32,
    /// `top-2` / `right-2` inset of the copy button.
    pub copy_offset_px: f32,
    /// `bottom-2` inset of the expand button.
    pub expand_offset_px: f32,
    /// `max-h-[300px]` collapsed overflow height.
    pub overflow_max_height_px: f32,
    /// Frame corner radius — inherits the per-style button radius, since the
    /// reference code block (a frame + copy/expand buttons) has no
    /// style-specific tokens of its own: Vega/Mira `md`, Nova `lg`,
    /// Maia/Luma `4xl`, Rhea `2xl`, Lyra/Sera `none`.
    pub default_radius: ComponentRadius,
}

/// Resolves the code-block frame tokens for `style`.
///
/// The web component is style-agnostic (`rounded-lg` + fixed paddings); the
/// frame therefore adopts the radius of the button recipes it embeds
/// (copy + expand), while keeping the reference's fixed geometry.
pub const fn code_recipe(style: StyleId) -> CodeRecipe {
    CodeRecipe {
        pad_y_px: 16.0,                // py-4
        pad_x_px: 16.0,                // px-4
        pad_x_with_numbers_px: 8.0,    // px-2
        copy_offset_px: 8.0,           // top-2 right-2
        expand_offset_px: 8.0,         // bottom-2
        overflow_max_height_px: 300.0, // max-h-[300px]
        default_radius: crate::recipes::button_type(style).default_radius,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipes::button_type;
    use crate::style::StyleId;

    #[test]
    fn code_radius_inherits_button_recipe() {
        for style in StyleId::ALL {
            assert_eq!(
                code_recipe(style).default_radius,
                button_type(style).default_radius,
                "{style:?} code radius must match button"
            );
        }
    }
}
