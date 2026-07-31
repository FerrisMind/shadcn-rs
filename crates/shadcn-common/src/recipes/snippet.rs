//! Snippet recipes from the shadcn-svelte-extra `Snippet` component.
//!
//! The web component is a styled `<pre>` frame with an absolutely-positioned
//! copy button; the geometry below is shared by the iced and egui backends.

use crate::style::StyleId;

use super::{ComponentRadius, FontWeight, TypeRecipe};

/// Geometry + typography recipe for a code snippet frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnippetRecipe {
    /// `text-sm font-light` code typography (14 px / weight 300 / 20 px line).
    pub typography: TypeRecipe,
    /// `py-2.5` vertical padding of the frame.
    pub pad_y_px: f32,
    /// `pl-3` left padding of the frame.
    pub pad_left_px: f32,
    /// `pr-12` right padding, reserving room for the copy button.
    pub pad_right_px: f32,
    /// `size-7` copy button footprint.
    pub copy_button_px: f32,
    /// `right-2` inset of the copy button from the frame edge.
    pub copy_button_offset_px: f32,
    /// Copy icon footprint inside the `size-7` button (measured 16 px on the
    /// reference site; the lucide default of 24 px would overflow the button).
    pub copy_icon_px: f32,
    /// Frame corner radius — inherits the per-style button radius, since the
    /// reference snippet (a `pre` frame + copy button) has no style-specific
    /// tokens of its own: Vega/Mira `md`, Nova `lg`, Maia/Luma `4xl`, Rhea
    /// `2xl`, Lyra/Sera `none`.
    pub default_radius: ComponentRadius,
}

/// Resolves the snippet frame tokens for `style`.
///
/// The web component is style-agnostic (`rounded-md` + fixed paddings); the
/// frame therefore adopts the radius of the button recipe it embeds, while
/// keeping the reference's fixed geometry for paddings and the `size-7` copy
/// button.
pub const fn snippet_recipe(style: StyleId) -> SnippetRecipe {
    SnippetRecipe {
        typography: TypeRecipe {
            size_px: 14.0, // text-sm
            weight: FontWeight::Light,
            uppercase: false,
            tracking_em: 0.0,
            line_height_px: 20.0, // 1.25rem
        },
        pad_y_px: 10.0,             // py-2.5
        pad_left_px: 12.0,          // pl-3
        pad_right_px: 48.0,         // pr-12
        copy_button_px: 28.0,       // size-7
        copy_button_offset_px: 8.0, // right-2
        copy_icon_px: 16.0,         // measured 16x16 on shadcn-svelte-extras.com
        default_radius: crate::recipes::button_type(style).default_radius,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipes::button_type;
    use crate::style::StyleId;

    #[test]
    fn snippet_radius_inherits_button_recipe() {
        for style in StyleId::ALL {
            assert_eq!(
                snippet_recipe(style).default_radius,
                button_type(style).default_radius,
                "{style:?} snippet radius must match button"
            );
        }
    }
}
