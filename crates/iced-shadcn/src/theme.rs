use crate::tokens::{Palette, Radius, Spacing};
use iced::Color;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorToken {
    Background,
    Foreground,
    Card,
    CardForeground,
    Popover,
    PopoverForeground,
    Border,
    Input,
    Ring,
    Primary,
    PrimaryForeground,
    Secondary,
    SecondaryForeground,
    Accent,
    AccentForeground,
    Muted,
    MutedForeground,
    Destructive,
    DestructiveForeground,
    Chart1,
    Chart2,
    Chart3,
    Chart4,
    Chart5,
    Sidebar,
    SidebarForeground,
    SidebarPrimary,
    SidebarPrimaryForeground,
    SidebarAccent,
    SidebarAccentForeground,
    SidebarBorder,
    SidebarRing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadiusToken {
    Sm,
    Md,
    Lg,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpacingToken {
    Xs,
    Sm,
    Md,
    Lg,
}

pub trait ThemeTokensSource {
    fn color(&self, token: ColorToken) -> Color;
    fn radius(&self, token: RadiusToken) -> f32;
    fn spacing(&self, token: SpacingToken) -> f32;

    fn styles(&self) -> ThemeStyles {
        ThemeStyles::default()
    }

    fn registry(&self) -> ThemeTokenRegistry {
        ThemeTokenRegistry::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct ThemeTokenRegistry {
    pub colors: BTreeMap<String, Color>,
    pub numbers: BTreeMap<String, f32>,
    pub durations_ms: BTreeMap<String, u64>,
    pub strings: BTreeMap<String, String>,
}

impl ThemeTokenRegistry {
    pub fn set_color(&mut self, key: impl Into<String>, value: Color) {
        self.colors.insert(key.into(), value);
    }

    pub fn set_number(&mut self, key: impl Into<String>, value: f32) {
        self.numbers.insert(key.into(), value);
    }

    pub fn set_duration_ms(&mut self, key: impl Into<String>, value: u64) {
        self.durations_ms.insert(key.into(), value);
    }

    pub fn set_string(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.strings.insert(key.into(), value.into());
    }

    pub fn color(&self, key: &str) -> Option<Color> {
        self.colors.get(key).copied()
    }

    pub fn number(&self, key: &str) -> Option<f32> {
        self.numbers.get(key).copied()
    }

    pub fn duration_ms(&self, key: &str) -> Option<u64> {
        self.durations_ms.get(key).copied()
    }

    pub fn string(&self, key: &str) -> Option<&str> {
        self.strings.get(key).map(String::as_str)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ShadowStyle {
    pub opacity: f32,
    pub offset_y: f32,
    pub blur_radius: f32,
}

impl Default for ShadowStyle {
    fn default() -> Self {
        Self {
            opacity: 0.12,
            offset_y: 4.0,
            blur_radius: 12.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CommandStyleTokens {
    pub min_width: f32,
    pub input_padding_y: f32,
    pub input_padding_x: f32,
    pub list_item_gap: f32,
    pub group_item_gap: f32,
    pub list_max_height: f32,
}

impl Default for CommandStyleTokens {
    fn default() -> Self {
        Self {
            min_width: 280.0,
            input_padding_y: 8.0,
            input_padding_x: 12.0,
            list_item_gap: 4.0,
            group_item_gap: 2.0,
            list_max_height: 300.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TabsStyleTokens {
    pub list_padding_size1: f32,
    pub list_padding_size2: f32,
    pub gap: f32,
    pub line_gap: f32,
    pub indicator_height: f32,
    pub active_pill_shadow: ShadowStyle,
}

impl Default for TabsStyleTokens {
    fn default() -> Self {
        Self {
            list_padding_size1: 2.0,
            list_padding_size2: 3.0,
            gap: 6.0,
            line_gap: 6.0,
            indicator_height: 2.0,
            active_pill_shadow: ShadowStyle {
                opacity: 0.18,
                offset_y: 1.0,
                blur_radius: 6.0,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SwitchStyleTokens {
    pub base_height: f32,
    pub base_width: f32,
    pub base_thumb: f32,
    pub track_shadow: ShadowStyle,
    pub animation_ms: u64,
}

impl Default for SwitchStyleTokens {
    fn default() -> Self {
        Self {
            base_height: 18.4,
            base_width: 32.0,
            base_thumb: 14.0,
            track_shadow: ShadowStyle {
                opacity: 0.05,
                offset_y: 1.0,
                blur_radius: 2.0,
            },
            animation_ms: 150,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ToastStyleTokens {
    pub width: f32,
    pub height: f32,
    pub margin: f32,
    pub gap: f32,
    pub close_inset: f32,
    pub close_size: f32,
    pub close_glyph_nudge_x: f32,
    pub close_glyph_nudge_y: f32,
    pub animation_ms: u64,
    pub shadow: ShadowStyle,
}

impl Default for ToastStyleTokens {
    fn default() -> Self {
        Self {
            width: 360.0,
            height: 64.0,
            margin: 16.0,
            gap: 8.0,
            close_inset: 10.0,
            close_size: 14.0,
            close_glyph_nudge_x: 1.0,
            close_glyph_nudge_y: 1.0,
            animation_ms: 180,
            shadow: ShadowStyle {
                opacity: 0.15,
                offset_y: 12.0,
                blur_radius: 28.0,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MenuStyleTokens {
    pub border_width: f32,
    pub shadow: ShadowStyle,
}

impl Default for MenuStyleTokens {
    fn default() -> Self {
        Self {
            border_width: 1.0,
            shadow: ShadowStyle::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InputStyleTokens {
    pub size1_padding_y: f32,
    pub size1_padding_x: f32,
    pub size2_padding_y: f32,
    pub size2_padding_x: f32,
    pub size3_padding_y: f32,
    pub size3_padding_x: f32,
    pub border_width: f32,
    pub focused_border_width: f32,
}

impl Default for InputStyleTokens {
    fn default() -> Self {
        Self {
            size1_padding_y: 6.0,
            size1_padding_x: 10.0,
            size2_padding_y: 8.0,
            size2_padding_x: 12.0,
            size3_padding_y: 10.0,
            size3_padding_x: 14.0,
            border_width: 1.0,
            focused_border_width: 1.5,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SidebarStyleTokens {
    pub expanded_width: f32,
    pub collapsed_width: f32,
    pub header_footer_padding: f32,
    pub content_padding: f32,
    pub group_spacing: f32,
    pub menu_spacing: f32,
}

impl Default for SidebarStyleTokens {
    fn default() -> Self {
        Self {
            expanded_width: 240.0,
            collapsed_width: 64.0,
            header_footer_padding: 12.0,
            content_padding: 8.0,
            group_spacing: 8.0,
            menu_spacing: 4.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FieldStyleTokens {
    pub spacing: f32,
    pub label_size: u32,
    pub description_size: u32,
    pub error_size: u32,
}

impl Default for FieldStyleTokens {
    fn default() -> Self {
        Self {
            spacing: 4.0,
            label_size: 14,
            description_size: 12,
            error_size: 12,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NavigationMenuStyleTokens {
    pub border_width: f32,
    pub shadow: ShadowStyle,
}

impl Default for NavigationMenuStyleTokens {
    fn default() -> Self {
        Self {
            border_width: 1.0,
            shadow: ShadowStyle {
                opacity: 0.18,
                offset_y: 8.0,
                blur_radius: 22.0,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ScrollAreaStyleTokens {
    pub size1_scrollbar_width: f32,
    pub size2_scrollbar_width: f32,
    pub size3_scrollbar_width: f32,
    pub default_scrollbar_margin: f32,
}

impl Default for ScrollAreaStyleTokens {
    fn default() -> Self {
        Self {
            size1_scrollbar_width: 4.0,
            size2_scrollbar_width: 8.0,
            size3_scrollbar_width: 12.0,
            default_scrollbar_margin: 4.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EmptyStyleTokens {
    pub root_gap: f32,
    pub root_padding: f32,
    pub root_max_width: f32,
    pub root_min_height: f32,
    pub header_gap: f32,
    pub header_max_width: f32,
    pub media_size: f32,
    pub media_icon_size: f32,
    pub title_size: f32,
    pub description_size: f32,
    pub description_max_width: f32,
    pub content_gap: f32,
    pub content_max_width: f32,
}

impl Default for EmptyStyleTokens {
    fn default() -> Self {
        Self {
            root_gap: 24.0,
            root_padding: 24.0,
            root_max_width: 384.0,
            root_min_height: 0.0,
            header_gap: 8.0,
            header_max_width: 384.0,
            media_size: 40.0,
            media_icon_size: 24.0,
            title_size: 18.0,
            description_size: 14.0,
            description_max_width: 320.0,
            content_gap: 16.0,
            content_max_width: 384.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ThemeStyles {
    pub command: CommandStyleTokens,
    pub tabs: TabsStyleTokens,
    pub switch: SwitchStyleTokens,
    pub toast: ToastStyleTokens,
    pub menu: MenuStyleTokens,
    pub input: InputStyleTokens,
    pub sidebar: SidebarStyleTokens,
    pub field: FieldStyleTokens,
    pub navigation_menu: NavigationMenuStyleTokens,
    pub scroll_area: ScrollAreaStyleTokens,
    pub empty: EmptyStyleTokens,
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub palette: Palette,
    pub radius: Radius,
    pub spacing: Spacing,
    pub styles: ThemeStyles,
    pub registry: ThemeTokenRegistry,
}

impl Theme {
    pub fn from_parts(
        palette: Palette,
        radius: Radius,
        spacing: Spacing,
        styles: ThemeStyles,
    ) -> Self {
        Self::from_parts_with_registry(
            palette,
            radius,
            spacing,
            styles,
            ThemeTokenRegistry::default(),
        )
    }

    pub fn from_parts_with_registry(
        palette: Palette,
        radius: Radius,
        spacing: Spacing,
        styles: ThemeStyles,
        registry: ThemeTokenRegistry,
    ) -> Self {
        Self {
            palette,
            radius,
            spacing,
            styles,
            registry,
        }
    }

    pub fn light() -> Self {
        Self::from_parts(
            Palette::light(),
            Radius::default(),
            Spacing::default(),
            ThemeStyles::default(),
        )
    }

    pub fn dark() -> Self {
        Self::from_parts(
            Palette::dark(),
            Radius::default(),
            Spacing::default(),
            ThemeStyles::default(),
        )
    }

    pub fn with_palette(palette: Palette) -> Self {
        Self::from_parts(
            palette,
            Radius::default(),
            Spacing::default(),
            ThemeStyles::default(),
        )
    }

    pub fn with_radius(mut self, radius: Radius) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_spacing(mut self, spacing: Spacing) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn with_styles(mut self, styles: ThemeStyles) -> Self {
        self.styles = styles;
        self
    }

    pub fn with_registry(mut self, registry: ThemeTokenRegistry) -> Self {
        self.registry = registry;
        self
    }

    pub fn from_tokens(source: &impl ThemeTokensSource) -> Self {
        let palette = Palette {
            background: source.color(ColorToken::Background),
            foreground: source.color(ColorToken::Foreground),
            card: source.color(ColorToken::Card),
            card_foreground: source.color(ColorToken::CardForeground),
            popover: source.color(ColorToken::Popover),
            popover_foreground: source.color(ColorToken::PopoverForeground),
            border: source.color(ColorToken::Border),
            input: source.color(ColorToken::Input),
            ring: source.color(ColorToken::Ring),
            primary: source.color(ColorToken::Primary),
            primary_foreground: source.color(ColorToken::PrimaryForeground),
            secondary: source.color(ColorToken::Secondary),
            secondary_foreground: source.color(ColorToken::SecondaryForeground),
            accent: source.color(ColorToken::Accent),
            accent_foreground: source.color(ColorToken::AccentForeground),
            muted: source.color(ColorToken::Muted),
            muted_foreground: source.color(ColorToken::MutedForeground),
            destructive: source.color(ColorToken::Destructive),
            destructive_foreground: source.color(ColorToken::DestructiveForeground),
            chart_1: source.color(ColorToken::Chart1),
            chart_2: source.color(ColorToken::Chart2),
            chart_3: source.color(ColorToken::Chart3),
            chart_4: source.color(ColorToken::Chart4),
            chart_5: source.color(ColorToken::Chart5),
            sidebar: source.color(ColorToken::Sidebar),
            sidebar_foreground: source.color(ColorToken::SidebarForeground),
            sidebar_primary: source.color(ColorToken::SidebarPrimary),
            sidebar_primary_foreground: source.color(ColorToken::SidebarPrimaryForeground),
            sidebar_accent: source.color(ColorToken::SidebarAccent),
            sidebar_accent_foreground: source.color(ColorToken::SidebarAccentForeground),
            sidebar_border: source.color(ColorToken::SidebarBorder),
            sidebar_ring: source.color(ColorToken::SidebarRing),
        };

        let radius = Radius {
            sm: source.radius(RadiusToken::Sm),
            md: source.radius(RadiusToken::Md),
            lg: source.radius(RadiusToken::Lg),
        };

        let spacing = Spacing {
            xs: source.spacing(SpacingToken::Xs),
            sm: source.spacing(SpacingToken::Sm),
            md: source.spacing(SpacingToken::Md),
            lg: source.spacing(SpacingToken::Lg),
        };

        Self::from_parts_with_registry(palette, radius, spacing, source.styles(), source.registry())
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}
