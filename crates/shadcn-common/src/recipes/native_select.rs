//! Native-select recipes from `.cn-native-select` / `.cn-native-select-icon`
//! across style packs.
//!
//! The web component styles only the trigger `<select>` field; the dropdown
//! itself is OS-rendered and never receives design-system tokens. Canvas
//! backends (iced, egui) mirror that split: the recipe below covers the
//! field only, while the dropdown uses the backend's stock menu styling.
//! The menu constants describe the shared geometry of that stock dropdown
//! (row rhythm, group indent, scroll cap) so both backends behave alike.

use crate::style::StyleId;

use super::ComponentRadius;

/// `has-[select:disabled]:opacity-50` on the wrapper.
pub const NATIVE_SELECT_DISABLED_OPACITY: f32 = 0.5;

/// Vertical padding of one dropdown row.
pub const NATIVE_SELECT_MENU_ITEM_PAD_Y_PX: f32 = 6.0;

/// Horizontal padding of one dropdown row.
pub const NATIVE_SELECT_MENU_ITEM_PAD_X_PX: f32 = 8.0;

/// Extra indentation of options nested inside an opt-group, mirroring the
/// native `<optgroup>` indent.
pub const NATIVE_SELECT_MENU_GROUP_INDENT_PX: f32 = 12.0;

/// Maximum dropdown height before it scrolls.
pub const NATIVE_SELECT_MENU_MAX_HEIGHT_PX: f32 = 288.0;

/// Status-independent `.cn-native-select` numbers of one style pack.
///
/// Control heights are not part of the recipe: the default size maps to the
/// pack's `md` control-height slot and `sm` to the `sm` slot, exactly like
/// the input recipe.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NativeSelectRecipe {
    /// `pl-*` in px (`pl-2.5` → 10).
    pub pad_left_px: f32,
    /// `pr-*` in px — reserves room for the chevron (`pr-8` → 32).
    pub pad_right_px: f32,
    /// Value text size (`text-sm` → 14, `text-xs` → 12).
    pub text_size_px: f32,
    /// Value text size of the `sm` slot (`data-[size=sm]:text-[0.625rem]` on
    /// Mira; equal to [`Self::text_size_px`] elsewhere).
    pub text_size_sm_px: f32,
    /// Corner treatment of the default size.
    pub radius: ComponentRadius,
    /// Corner treatment of the `sm` size (`data-[size=sm]:rounded-*`).
    pub radius_sm: ComponentRadius,
    /// `bg-input/N` alpha in light mode (0 = `bg-transparent`).
    pub fill_alpha_light: f32,
    /// `dark:bg-input/N` alpha.
    pub fill_alpha_dark: f32,
    /// `dark:hover:bg-input/N` alpha (equal to [`Self::fill_alpha_dark`]
    /// when the pack has no hover wash).
    pub hover_fill_alpha_dark: f32,
    /// Whether the resting border is painted (`border-input` vs
    /// `border-transparent`).
    pub bordered: bool,
    /// Sera's `border-b-input`: only the bottom hairline is painted.
    pub underline_only: bool,
    /// Chevron icon edge (`size-4` → 16).
    pub icon_size_px: f32,
    /// Chevron icon edge of the `sm` slot (`size-3` on Mira).
    pub icon_size_sm_px: f32,
    /// Distance from the right edge to the icon (`right-2.5` → 10).
    pub icon_right_px: f32,
}

/// Resolves `.cn-native-select` tokens for `style`.
pub const fn native_select_recipe(style: StyleId) -> NativeSelectRecipe {
    match style {
        StyleId::Vega => VEGA,
        // `h-8 rounded-lg pl-2.5 pr-8 text-sm`; sm:
        // `rounded-[min(--radius-md,10px)]` → twill md slot.
        StyleId::Nova => NativeSelectRecipe {
            radius: ComponentRadius::Lg,
            radius_sm: ComponentRadius::Md,
            ..VEGA
        },
        // `bg-input/30 h-9 rounded-4xl pl-3 pr-8`, icon at `right-3.5`.
        StyleId::Maia => NativeSelectRecipe {
            pad_left_px: 12.0,
            radius: ComponentRadius::S4xl,
            radius_sm: ComponentRadius::S4xl,
            fill_alpha_light: 0.3,
            fill_alpha_dark: 0.3,
            hover_fill_alpha_dark: 0.3,
            icon_right_px: 14.0,
            ..VEGA
        },
        // `h-8 rounded-none pl-2.5 pr-8 text-xs`.
        StyleId::Lyra => NativeSelectRecipe {
            text_size_px: 12.0,
            text_size_sm_px: 12.0,
            radius: ComponentRadius::None,
            radius_sm: ComponentRadius::None,
            ..VEGA
        },
        // `bg-input/20 h-7 rounded-md pl-2 pr-6 text-xs`, sm: 10px text,
        // icon `size-3.5` (`size-3` on sm) at `right-1.5`.
        StyleId::Mira => NativeSelectRecipe {
            pad_left_px: 8.0,
            pad_right_px: 24.0,
            text_size_px: 12.0,
            text_size_sm_px: 10.0,
            fill_alpha_light: 0.2,
            icon_size_px: 14.0,
            icon_size_sm_px: 12.0,
            icon_right_px: 6.0,
            ..VEGA
        },
        // `bg-input/50 h-9 rounded-3xl border-transparent pl-3 pr-8` — pill.
        // Soft panel: `bg-input/50 h-9 rounded-3xl border-transparent`.
        StyleId::Luma => NativeSelectRecipe {
            pad_left_px: 12.0,
            radius: ComponentRadius::S3xl,
            radius_sm: ComponentRadius::S3xl,
            fill_alpha_light: 0.5,
            fill_alpha_dark: 0.5,
            hover_fill_alpha_dark: 0.5,
            bordered: false,
            ..VEGA
        },
        // Underline-only `border-b-input pl-0 pr-8 h-10`, icon at `right-0`
        // sized `size-3.5`.
        StyleId::Sera => NativeSelectRecipe {
            pad_left_px: 0.0,
            radius: ComponentRadius::None,
            radius_sm: ComponentRadius::None,
            fill_alpha_dark: 0.0,
            hover_fill_alpha_dark: 0.0,
            underline_only: true,
            icon_size_px: 14.0,
            icon_size_sm_px: 14.0,
            icon_right_px: 0.0,
            ..VEGA
        },
        // `bg-input/50 h-8 rounded-2xl border-transparent pl-2.5 pr-8`.
        StyleId::Rhea => NativeSelectRecipe {
            radius: ComponentRadius::S2xl,
            radius_sm: ComponentRadius::S2xl,
            fill_alpha_light: 0.5,
            fill_alpha_dark: 0.5,
            hover_fill_alpha_dark: 0.5,
            bordered: false,
            ..VEGA
        },
    }
}

/// Vega `.cn-native-select` used as the fallback for unknown future packs:
/// `h-9 rounded-md border-input bg-transparent dark:bg-input/30
/// dark:hover:bg-input/50 py-1 pr-8 pl-2.5 text-sm`, icon `size-4 right-2.5`.
const VEGA: NativeSelectRecipe = NativeSelectRecipe {
    pad_left_px: 10.0,
    pad_right_px: 32.0,
    text_size_px: 14.0,
    text_size_sm_px: 14.0,
    radius: ComponentRadius::Md,
    radius_sm: ComponentRadius::Md,
    fill_alpha_light: 0.0,
    fill_alpha_dark: 0.3,
    hover_fill_alpha_dark: 0.5,
    bordered: true,
    underline_only: false,
    icon_size_px: 16.0,
    icon_size_sm_px: 16.0,
    icon_right_px: 10.0,
};
