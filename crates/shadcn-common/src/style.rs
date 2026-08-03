//! Style packs (Vega, Nova, …) — numeric tokens only, not CSS recipes.
//!
//! Defaults aligned with shadcn-svelte create customizer / style CSS:
//! - Lyra & Sera lock radius to `none`
//! - Rhea disallows `large`
//! - Other styles use base-color `--radius: 0.625rem` when picker is `default`
//! - Preset fonts: Vega/Geist, Nova/Inter, Lyra/JetBrains Mono, Sera/Instrument Serif

use twill_core::tokens::{BorderRadius, Spacing};

use crate::radius::{RadiusId, RadiusScale};
use crate::typography::{FontId, FontPack};

/// Named shadcn style system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StyleId {
    #[default]
    Vega,
    Nova,
    Maia,
    Lyra,
    Mira,
    Luma,
    Sera,
    Rhea,
}

impl StyleId {
    pub const ALL: [Self; 8] = [
        Self::Vega,
        Self::Nova,
        Self::Maia,
        Self::Lyra,
        Self::Mira,
        Self::Luma,
        Self::Sera,
        Self::Rhea,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Vega => "vega",
            Self::Nova => "nova",
            Self::Maia => "maia",
            Self::Lyra => "lyra",
            Self::Mira => "mira",
            Self::Luma => "luma",
            Self::Sera => "sera",
            Self::Rhea => "rhea",
        }
    }

    pub fn from_str_name(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|style| style.as_str().eq_ignore_ascii_case(name))
    }

    /// shadcn: Lyra and Sera force radius to `none`.
    pub const fn locks_radius(self) -> bool {
        matches!(self, Self::Lyra | Self::Sera)
    }

    /// shadcn: Rhea cannot use `large`.
    pub const fn disallows_large_radius(self) -> bool {
        matches!(self, Self::Rhea)
    }

    /// Intrinsic `--radius` rem when the picker is `default`.
    ///
    /// Matches `buildRegistryTheme`: non-default radii overwrite base CSS vars;
    /// `default` keeps the base-color `--radius` (`0.625rem`). Locked styles
    /// always resolve to `0`.
    pub const fn default_radius_rem(self) -> f32 {
        if self.locks_radius() { 0.0 } else { 0.625 }
    }

    /// Picker value when no explicit radius override is set.
    ///
    /// Always [`RadiusId::Default`] — for Lyra/Sera that still resolves to
    /// `0rem` via [`Self::default_radius_rem`].
    pub const fn default_radius_id(self) -> RadiusId {
        RadiusId::Default
    }

    /// Normalize picker value for this style.
    ///
    /// Locked styles (Lyra/Sera) keep the picker on `default` (rem → none).
    /// Rhea cannot use `large`.
    pub const fn resolve_radius(self, radius: RadiusId) -> RadiusId {
        if self.locks_radius() {
            return RadiusId::Default;
        }
        if matches!(radius, RadiusId::Large) && self.disallows_large_radius() {
            return RadiusId::Default;
        }
        radius
    }

    pub const fn pack(self) -> StylePack {
        match self {
            Self::Vega => StylePack::VEGA,
            Self::Nova => StylePack::NOVA,
            Self::Maia => StylePack::MAIA,
            Self::Lyra => StylePack::LYRA,
            Self::Mira => StylePack::MIRA,
            Self::Luma => StylePack::LUMA,
            Self::Sera => StylePack::SERA,
            Self::Rhea => StylePack::RHEA,
        }
    }
}

/// Backend-agnostic numeric style tokens derived from shadcn style CSS.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StylePack {
    pub id: StyleId,
    pub radius: RadiusScale,
    pub spacing_unit_px: f32,
    pub control_height_sm_px: f32,
    pub control_height_md_px: f32,
    pub control_height_lg_px: f32,
    pub card_padding_px: f32,
    pub font_pack: FontPack,
    pub twill_radius_sm: BorderRadius,
    pub twill_radius_md: BorderRadius,
    pub twill_radius_lg: BorderRadius,
    pub gap_sm: Spacing,
    pub gap_md: Spacing,
}

impl StylePack {
    /// Classic shadcn look — base `--radius: 0.625rem`, button h-9.
    pub const VEGA: Self = Self {
        id: StyleId::Vega,
        radius: RadiusScale::from_rem(0.625),
        spacing_unit_px: 4.0,
        control_height_sm_px: 32.0,
        control_height_md_px: 36.0,
        control_height_lg_px: 40.0,
        card_padding_px: 24.0,
        font_pack: FontPack::GEIST,
        twill_radius_sm: BorderRadius::Md,
        twill_radius_md: BorderRadius::Lg,
        twill_radius_lg: BorderRadius::Xl,
        gap_sm: Spacing::S2,
        gap_md: Spacing::S4,
    };

    /// Compact controls; radius still base `0.625rem` when picker is `default`.
    pub const NOVA: Self = Self {
        id: StyleId::Nova,
        radius: RadiusScale::from_rem(0.625),
        spacing_unit_px: 4.0,
        control_height_sm_px: 28.0,
        control_height_md_px: 32.0,
        control_height_lg_px: 36.0,
        card_padding_px: 16.0,
        font_pack: FontPack::INTER,
        twill_radius_sm: BorderRadius::Md,
        twill_radius_md: BorderRadius::Lg,
        twill_radius_lg: BorderRadius::Xl,
        gap_sm: Spacing::S1,
        gap_md: Spacing::S3,
    };

    /// Soft / generous spacing; radius base `0.625rem` when `default`.
    pub const MAIA: Self = Self {
        id: StyleId::Maia,
        radius: RadiusScale::from_rem(0.625),
        spacing_unit_px: 4.0,
        control_height_sm_px: 32.0,
        control_height_md_px: 36.0,
        control_height_lg_px: 40.0,
        card_padding_px: 24.0,
        // Preset uses Figtree (not bundled); Inter is the closest available sans.
        font_pack: FontPack::INTER,
        twill_radius_sm: BorderRadius::Lg,
        twill_radius_md: BorderRadius::Xl,
        twill_radius_lg: BorderRadius::S2xl,
        gap_sm: Spacing::S2,
        gap_md: Spacing::S4,
    };

    /// Boxy / sharp — radius locked to `none`, mono preset font.
    pub const LYRA: Self = Self {
        id: StyleId::Lyra,
        radius: RadiusScale::none(),
        spacing_unit_px: 4.0,
        control_height_sm_px: 28.0,
        control_height_md_px: 32.0,
        control_height_lg_px: 36.0,
        card_padding_px: 16.0,
        font_pack: FontPack {
            sans: FontId::JetBrainsMono,
            heading: FontId::JetBrainsMono,
            mono: FontId::JetBrainsMono,
        },
        twill_radius_sm: BorderRadius::None,
        twill_radius_md: BorderRadius::None,
        twill_radius_lg: BorderRadius::None,
        gap_sm: Spacing::S1,
        gap_md: Spacing::S3,
    };

    /// Dense / compact controls.
    pub const MIRA: Self = Self {
        id: StyleId::Mira,
        radius: RadiusScale::from_rem(0.625),
        spacing_unit_px: 4.0,
        control_height_sm_px: 24.0,
        control_height_md_px: 28.0,
        control_height_lg_px: 32.0,
        card_padding_px: 16.0,
        font_pack: FontPack::GEIST,
        twill_radius_sm: BorderRadius::Sm,
        twill_radius_md: BorderRadius::Md,
        twill_radius_lg: BorderRadius::Md,
        gap_sm: Spacing::S1,
        gap_md: Spacing::S2,
    };

    /// Rounded geometry / breathable.
    pub const LUMA: Self = Self {
        id: StyleId::Luma,
        radius: RadiusScale::from_rem(0.625),
        spacing_unit_px: 4.0,
        control_height_sm_px: 32.0,
        control_height_md_px: 36.0,
        control_height_lg_px: 40.0,
        card_padding_px: 24.0,
        font_pack: FontPack::GEIST,
        twill_radius_sm: BorderRadius::Lg,
        twill_radius_md: BorderRadius::Xl,
        twill_radius_lg: BorderRadius::S2xl,
        gap_sm: Spacing::S2,
        gap_md: Spacing::S4,
    };

    /// Editorial — radius locked to `none`, Instrument Serif headings.
    pub const SERA: Self = Self {
        id: StyleId::Sera,
        radius: RadiusScale::none(),
        spacing_unit_px: 4.0,
        // Matches `.cn-button-size-*`: h-7 / h-9 / h-10 / h-11.
        control_height_sm_px: 36.0,
        control_height_md_px: 40.0,
        control_height_lg_px: 44.0,
        card_padding_px: 24.0,
        font_pack: FontPack::INSTRUMENT_SERIF,
        twill_radius_sm: BorderRadius::None,
        twill_radius_md: BorderRadius::None,
        twill_radius_lg: BorderRadius::None,
        gap_sm: Spacing::S2,
        gap_md: Spacing::S4,
    };

    /// Like Luma but compact; `large` radius disallowed.
    pub const RHEA: Self = Self {
        id: StyleId::Rhea,
        radius: RadiusScale::from_rem(0.625),
        spacing_unit_px: 4.0,
        control_height_sm_px: 28.0,
        control_height_md_px: 32.0,
        control_height_lg_px: 36.0,
        card_padding_px: 20.0,
        font_pack: FontPack::GEIST,
        twill_radius_sm: BorderRadius::Lg,
        twill_radius_md: BorderRadius::Xl,
        twill_radius_lg: BorderRadius::S2xl,
        gap_sm: Spacing::S2,
        gap_md: Spacing::S3,
    };

    pub const fn with_radius(mut self, radius: RadiusId) -> Self {
        let resolved = self.id.resolve_radius(radius);
        let rem = resolved.resolved_rem(self.id.default_radius_rem());
        self.radius = RadiusScale::from_rem(rem);
        // An explicit picker value recomputes the corner tokens. `Default`
        // keeps the pack's intrinsic corners (style-*.css: Vega/Nova md=8px,
        // Mira md=6px, Maia/Luma/Rhea md=12px, Lyra/Sera none) — without this,
        // every unlocked style collapses to the same radius.
        if resolved.rem().is_some() {
            let (sm, md, lg) = RadiusId::twill_radii_for_rem(rem);
            self.twill_radius_sm = sm;
            self.twill_radius_md = md;
            self.twill_radius_lg = lg;
        }
        self
    }

    pub const fn with_font(mut self, font: FontId) -> Self {
        self.font_pack = self.font_pack.with_sans(font);
        self
    }

    pub const fn with_font_heading(mut self, heading: FontId) -> Self {
        self.font_pack = self.font_pack.with_heading_font(heading);
        self
    }

    pub const fn with_font_pack(mut self, font_pack: FontPack) -> Self {
        self.font_pack = font_pack;
        self
    }

    pub fn radius_id(self) -> RadiusId {
        self.id.default_radius_id()
    }

    /// `.cn-label` recipe for this pack.
    pub const fn label(self, context: crate::recipes::LabelContext) -> crate::recipes::LabelRecipe {
        crate::recipes::label_recipe(self.id, context)
    }

    /// `.cn-button-size-*` geometry for this pack.
    pub const fn button_size(
        self,
        size: crate::recipes::ControlSize,
    ) -> crate::recipes::ButtonSizeRecipe {
        crate::recipes::button_size(self.id, size)
    }

    /// Base `.cn-button` type + default radius for this pack.
    pub const fn button_type(self) -> crate::recipes::ButtonTypeRecipe {
        crate::recipes::button_type(self.id)
    }

    /// `.cn-badge` recipe for this pack.
    pub const fn badge(self) -> crate::recipes::BadgeRecipe {
        crate::recipes::badge_recipe(self.id)
    }

    /// `.cn-carousel-previous` / `.cn-carousel-next` control tokens for this pack.
    pub const fn carousel(self) -> crate::recipes::CarouselRecipe {
        crate::recipes::carousel_recipe(self.id)
    }

    /// `.cn-kbd` recipe for this pack.
    pub const fn kbd(self) -> crate::recipes::KbdRecipe {
        crate::recipes::kbd_recipe(self.id)
    }

    /// `.cn-skeleton` default radius for this pack.
    pub const fn skeleton_default_radius(self) -> crate::recipes::ComponentRadius {
        crate::recipes::skeleton_default_radius(self.id)
    }

    /// Snippet frame geometry + typography for this pack.
    pub const fn snippet(self) -> crate::recipes::SnippetRecipe {
        crate::recipes::snippet_recipe(self.id)
    }

    /// Code-block frame geometry for this pack.
    pub const fn code(self) -> crate::recipes::CodeRecipe {
        crate::recipes::code_recipe(self.id)
    }

    /// `.cn-checkbox` track radius for this pack.
    pub const fn checkbox(self) -> crate::recipes::CheckboxRecipe {
        crate::recipes::checkbox_recipe(self.id)
    }

    /// `.cn-progress` geometry and default radius for this pack.
    pub const fn progress(self) -> crate::recipes::ProgressRecipe {
        crate::recipes::progress_recipe(self.id)
    }

    /// shadcn-svelte-extras `Meter` geometry (`h-2`, `rounded-full`, `/20` track).
    pub const fn meter(self) -> crate::recipes::MeterRecipe {
        crate::recipes::meter_recipe(self.id)
    }

    /// `.cn-slider*` track, range, and thumb tokens for this pack.
    pub const fn slider(self) -> crate::recipes::SliderRecipe {
        crate::recipes::slider_recipe(self.id)
    }

    /// `.cn-radio-group*` indicator, dot, ring, and gap tokens for this pack.
    pub const fn radio_group(self) -> crate::recipes::RadioGroupRecipe {
        crate::recipes::radio_group_recipe(self.id)
    }

    /// `.cn-native-select` field and icon tokens for this pack.
    pub const fn native_select(self) -> crate::recipes::NativeSelectRecipe {
        crate::recipes::native_select_recipe(self.id)
    }

    /// `.cn-select-*` trigger, content, and item tokens for this pack.
    pub const fn select(self) -> crate::recipes::SelectRecipe {
        crate::recipes::select_recipe(self.id)
    }

    /// `.cn-dropdown-menu-*` content and item tokens for this pack.
    pub const fn dropdown_menu(self) -> crate::recipes::DropdownMenuRecipe {
        crate::recipes::dropdown_menu_recipe(self.id)
    }

    /// `.cn-context-menu-*` content and item tokens for this pack.
    ///
    /// The shadcn-svelte context-menu shares its `.cn-menu-target` surface with
    /// the dropdown-menu, so this resolves to the same recipe; the method exists
    /// so call sites read `style.context_menu()` and a future split stays local.
    pub const fn context_menu(self) -> crate::recipes::ContextMenuRecipe {
        crate::recipes::context_menu_recipe(self.id)
    }

    /// `.cn-menubar-*` bar, trigger, and content tokens for this pack.
    pub const fn menubar(self) -> crate::recipes::MenubarRecipe {
        crate::recipes::menubar_recipe(self.id)
    }

    /// `.cn-switch` border, ring, and default radius for this pack.
    pub const fn switch(self) -> crate::recipes::SwitchRecipe {
        crate::recipes::switch_recipe(self.id)
    }

    /// `.cn-switch` / `.cn-switch-thumb` geometry for this pack.
    pub const fn switch_size(
        self,
        size: crate::recipes::ControlSize,
    ) -> crate::recipes::SwitchSizeRecipe {
        crate::recipes::switch_size(self.id, size)
    }

    /// Base `.cn-toggle` typography, radius, and shadow for this pack.
    pub const fn toggle(self) -> crate::recipes::ToggleRecipe {
        crate::recipes::toggle_recipe(self.id)
    }

    /// `.cn-toggle-size-*` geometry for this pack.
    pub const fn toggle_size(
        self,
        size: crate::recipes::ControlSize,
    ) -> crate::recipes::ToggleSizeRecipe {
        crate::recipes::toggle_size(self.id, size)
    }

    /// `.cn-form-*` layout and supporting-text tokens for this pack.
    pub const fn form(self) -> crate::recipes::FormRecipe {
        crate::recipes::form_recipe(self.id)
    }

    /// `.cn-textarea` geometry + surface tokens for this pack.
    pub const fn textarea(self) -> crate::recipes::TextareaRecipe {
        crate::recipes::textarea_recipe(self.id)
    }

    /// Star-rating geometry from shadcn-svelte-extras (`size-5` / `gap-1`).
    pub const fn star_rating(self) -> crate::recipes::StarRatingRecipe {
        crate::recipes::star_rating_recipe(self.id)
    }

    /// Password geometry from shadcn-svelte-extras (`size-9` / `h-[6px]`).
    pub const fn password(self) -> crate::recipes::PasswordRecipe {
        crate::recipes::password_recipe(self.id)
    }

    /// File-drop-zone trigger geometry from shadcn-svelte-extras (`h-48` / `p-6`).
    pub const fn file_drop_zone(self) -> crate::recipes::FileDropZoneRecipe {
        crate::recipes::file_drop_zone_recipe(self.id)
    }

    /// Phone-input geometry from shadcn-svelte-extras (`h-9` / `w-[300px]`).
    pub const fn phone_input(self) -> crate::recipes::PhoneInputRecipe {
        crate::recipes::phone_input_recipe(self.id)
    }
}
