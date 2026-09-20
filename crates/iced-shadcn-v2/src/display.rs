//! [`fmt::Display`] implementations for the public configuration enums.
//!
//! Every value renders as the kebab-case token used by the shadcn web
//! components (`"default"`, `"icon-sm"`, `"space-between"`, …). Custom
//! numeric values render as `{value}px`.

#![allow(unused_imports)] // imports vary with cargo features

use std::fmt;

#[cfg(feature = "core")]
use crate::components::alert::{AlertRadius, AlertVariant};
#[cfg(feature = "data")]
use crate::components::avatar::{AvatarRadius, AvatarSize};
#[cfg(feature = "core")]
use crate::components::badge::{BadgeRadius, BadgeVariant};
#[cfg(feature = "core")]
use crate::components::button::{ButtonRadius, ButtonSize, ButtonVariant};
#[cfg(feature = "core")]
use crate::components::card::{
    CardBorder, CardFooterAlignment, CardFooterDirection, CardRadius, CardSize,
};
#[cfg(feature = "forms")]
use crate::components::checkbox::{CheckboxSize, CheckboxState, CheckboxVariant};
#[cfg(feature = "core")]
use crate::components::collapsible::{
    CollapsibleAlignment, CollapsibleEasing, CollapsibleIndicator, CollapsibleIndicatorPlacement,
    CollapsibleOrientation, CollapsibleState,
};
#[cfg(feature = "forms")]
use crate::components::field::{FieldLegendVariant, FieldOrientation};
#[cfg(feature = "forms")]
use crate::components::input::{InputRadius, InputSize};
#[cfg(feature = "forms")]
use crate::components::input_group::{
    InputGroupAddonAlign, InputGroupButtonSize, InputGroupRadius, InputGroupTextareaResize,
};
#[cfg(feature = "forms")]
use crate::components::input_otp::{InputOtpPattern, InputOtpRadius};
#[cfg(feature = "core")]
use crate::components::kbd::{KbdRadius, KbdSurface};
#[cfg(feature = "core")]
use crate::components::meter::{MeterOrientation, MeterRadius, MeterSize};
#[cfg(feature = "core")]
use crate::components::progress::{
    ProgressOrientation, ProgressRadius, ProgressSize, ProgressVariant,
};
#[cfg(feature = "forms")]
use crate::components::radio_group::{RadioGroupOrientation, RadioGroupRadius, RadioGroupSize};
#[cfg(feature = "core")]
use crate::components::scroll_area::{ScrollAreaAnchor, ScrollAreaOrientation, ScrollAreaRadius};
#[cfg(feature = "core")]
use crate::components::separator::SeparatorOrientation;
#[cfg(feature = "core")]
use crate::components::skeleton::{SkeletonAnimation, SkeletonFill, SkeletonRadius, SkeletonShape};
#[cfg(feature = "forms")]
use crate::components::slider::{SliderOrientation, SliderRadius};
#[cfg(feature = "core")]
use crate::components::spinner::{SpinnerSize, SpinnerVariant};
#[cfg(feature = "forms")]
use crate::components::star_rating::{StarRatingOrientation, StarRatingSize};
#[cfg(feature = "forms")]
use crate::components::switch::{SwitchRadius, SwitchSize};
#[cfg(feature = "navigation")]
use crate::components::tabs::{
    TabsActivationMode, TabsHover, TabsJustify, TabsListLoop, TabsListVariant, TabsOrientation,
    TabsSize, TabsWrap,
};
#[cfg(feature = "forms")]
use crate::components::toggle::{ToggleRadius, ToggleSize, ToggleVariant};
#[cfg(feature = "forms")]
use crate::components::toggle_group::{ToggleGroupOrientation, ToggleGroupType};
#[cfg(feature = "overlays")]
use crate::components::tooltip::{TooltipAlign, TooltipSide};
#[cfg(feature = "core")]
use crate::components::typography::TypographyVariant;

/// Implements [`fmt::Display`] for unit-variant enums with fixed token text.
macro_rules! impl_display {
    ($($ty:ty { $($variant:ident => $text:literal),+ $(,)? })+) => {$(
        impl fmt::Display for $ty {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(match self {
                    $(Self::$variant => $text,)+
                })
            }
        }
    )+};
}

/// Implements [`fmt::Display`] for enums that additionally carry a
/// `Custom(f32)` pixel variant.
macro_rules! impl_display_custom_px {
    ($($ty:ty { $($variant:ident => $text:literal),+ $(,)? })+) => {$(
        impl fmt::Display for $ty {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$variant => formatter.write_str($text),)+
                    Self::Custom(value) => write!(formatter, "{value}px"),
                }
            }
        }
    )+};
}

#[cfg(feature = "core")]
impl_display! {
    AlertVariant { Default => "default", Destructive => "destructive" }
    BadgeVariant {
        Default => "default", Destructive => "destructive", Outline => "outline",
        Secondary => "secondary", Ghost => "ghost", Link => "link",
    }
    BadgeRadius {
        None => "none", Small => "small", Medium => "medium",
        Large => "large", Full => "full",
    }
    ButtonVariant {
        Default => "default", Destructive => "destructive", Outline => "outline",
        Secondary => "secondary", Ghost => "ghost", Link => "link",
        Soft => "soft", Surface => "surface",
    }
    ButtonSize {
        Xs => "xs", Sm => "sm", Default => "default", Lg => "lg",
        IconXs => "icon-xs", IconSm => "icon-sm", Icon => "icon", IconLg => "icon-lg",
    }
    ButtonRadius {
        None => "none", Small => "small", Medium => "medium",
        Large => "large", Full => "full",
    }
    CardSize { Default => "default", Sm => "sm" }
    CardBorder { Theme => "theme", None => "none", Present => "present" }
    CardFooterDirection { Row => "row", Column => "column" }
    CardFooterAlignment {
        Start => "start", Center => "center", End => "end",
        SpaceBetween => "space-between",
    }
    CollapsibleState { Closed => "closed", Open => "open" }
    CollapsibleOrientation { Vertical => "vertical", Horizontal => "horizontal" }
    CollapsibleAlignment { Start => "start", Center => "center", End => "end" }
    CollapsibleIndicator { Chevron => "chevron", ChevronDown => "chevron-down" }
    CollapsibleIndicatorPlacement { Leading => "leading", Trailing => "trailing" }
    CollapsibleEasing {
        Linear => "linear", EaseOut => "ease-out", EaseInOut => "ease-in-out",
    }
    KbdSurface {
        Default => "default", Tooltip => "tooltip", InputGroup => "input-group",
    }
    KbdRadius {
        None => "none", Small => "small", Medium => "medium",
        Large => "large", Full => "full",
    }
    ProgressVariant {
        Default => "default", Classic => "classic",
        Surface => "surface", Soft => "soft",
    }
    ProgressOrientation { Horizontal => "horizontal", Vertical => "vertical" }
    MeterOrientation { Horizontal => "horizontal", Vertical => "vertical" }
    ScrollAreaOrientation {
        Vertical => "vertical", Horizontal => "horizontal", Both => "both",
    }
    ScrollAreaAnchor { Start => "start", End => "end" }
    SeparatorOrientation { Horizontal => "horizontal", Vertical => "vertical" }
    SkeletonAnimation { Pulse => "pulse", Static => "static" }
    SpinnerVariant {
        LegacyLucide => "legacy-lucide", AiLoaderIcon => "ai-loader-icon",
        Circular => "circular", Classic => "classic", Pulse => "pulse",
        PulseDot => "pulse-dot", Dots => "dots", Typing => "typing",
        Wave => "wave", Bars => "bars", Terminal => "terminal",
        TextBlink => "text-blink", TextShimmer => "text-shimmer",
        LoadingDots => "loading-dots",
    }
    TypographyVariant {
        H1 => "h1", H2 => "h2", H3 => "h3", H4 => "h4", P => "p",
        Blockquote => "blockquote", InlineCode => "inline-code",
        Lead => "lead", Large => "large", Small => "small", Muted => "muted",
    }
}

#[cfg(feature = "forms")]
impl_display! {
    CheckboxState {
        Unchecked => "unchecked", Checked => "checked",
        Indeterminate => "indeterminate",
    }
    CheckboxVariant { Surface => "surface", Classic => "classic", Soft => "soft" }
    CheckboxSize { Xs => "xs", Sm => "sm", Md => "md", Lg => "lg" }
    FieldOrientation {
        Vertical => "vertical", Horizontal => "horizontal", Responsive => "responsive",
    }
    FieldLegendVariant { Legend => "legend", Label => "label" }
    InputSize { Sm => "sm", Default => "default", Lg => "lg" }
    InputRadius {
        None => "none", Small => "small", Medium => "medium",
        Large => "large", Full => "full",
    }
    InputGroupAddonAlign {
        InlineStart => "inline-start", InlineEnd => "inline-end",
        BlockStart => "block-start", BlockEnd => "block-end",
    }
    InputGroupButtonSize {
        Xs => "xs", Sm => "sm", IconXs => "icon-xs", IconSm => "icon-sm",
    }
    InputGroupRadius {
        None => "none", Small => "small", Medium => "medium",
        Large => "large", Full => "full",
    }
    InputGroupTextareaResize {
        None => "none", Vertical => "vertical", Horizontal => "horizontal", Both => "both",
    }
    InputOtpPattern {
        Any => "any", Digits => "digits", Chars => "chars",
        DigitsAndChars => "digits-and-chars",
    }
    InputOtpRadius {
        None => "none", Small => "small", Medium => "medium",
        Large => "large", Full => "full",
    }
    RadioGroupOrientation { Vertical => "vertical", Horizontal => "horizontal" }
    SliderOrientation { Horizontal => "horizontal", Vertical => "vertical" }
    StarRatingOrientation { Horizontal => "horizontal", Vertical => "vertical" }
    ToggleVariant { Default => "default", Outline => "outline" }
    ToggleSize { Sm => "sm", Default => "default", Lg => "lg" }
    ToggleRadius {
        None => "none", Small => "small", Medium => "medium",
        Large => "large", Full => "full",
    }
    ToggleGroupType { Single => "single", Multiple => "multiple" }
    ToggleGroupOrientation { Horizontal => "horizontal", Vertical => "vertical" }
}

#[cfg(feature = "overlays")]
impl_display! {
    TooltipSide { Top => "top", Right => "right", Bottom => "bottom", Left => "left" }
    TooltipAlign { Start => "start", Center => "center", End => "end" }
}

#[cfg(feature = "navigation")]
impl_display! {
    TabsActivationMode { Automatic => "automatic", Manual => "manual" }
    TabsHover { None => "none", Subtle => "subtle", Soft => "soft" }
    TabsJustify { Start => "start", Center => "center", End => "end" }
    TabsListLoop { Enabled => "enabled", Disabled => "disabled" }
    TabsListVariant { Default => "default", Line => "line" }
    TabsOrientation { Horizontal => "horizontal", Vertical => "vertical" }
    TabsSize { Sm => "sm", Default => "default", Lg => "lg" }
    TabsWrap { NoWrap => "no-wrap", Wrap => "wrap", WrapReverse => "wrap-reverse" }
}

#[cfg(feature = "core")]
impl_display_custom_px! {
    AlertRadius {
        Theme => "theme", None => "none", Small => "small", Medium => "medium",
        Large => "large", Xl => "xl", Full => "full",
    }
    CardRadius {
        Theme => "theme", None => "none", Small => "small", Medium => "medium",
        Large => "large", Xl => "xl", Full => "full",
    }
    ProgressSize { Xs => "xs", Sm => "sm", Default => "default", Lg => "lg", Xl => "xl" }
    ProgressRadius {
        None => "none", Small => "small", Medium => "medium",
        Large => "large", Full => "full",
    }
    MeterSize { Xs => "xs", Sm => "sm", Default => "default", Lg => "lg", Xl => "xl" }
    MeterRadius {
        None => "none", Small => "small", Medium => "medium",
        Large => "large", Full => "full",
    }
    ScrollAreaRadius {
        Theme => "theme", None => "none", Small => "small", Medium => "medium",
        Large => "large", Xl => "xl", Full => "full",
    }
    SkeletonRadius {
        None => "none", Small => "small", Medium => "medium",
        Large => "large", Full => "full",
    }
    SpinnerSize { Xs => "xs", Sm => "sm", Default => "default", Lg => "lg", Xl => "xl" }
}

#[cfg(feature = "data")]
impl_display_custom_px! {
    AvatarSize { Sm => "sm", Default => "default", Lg => "lg" }
    AvatarRadius {
        Theme => "theme", None => "none", Small => "small", Medium => "medium",
        Large => "large", Xl => "xl", Full => "full",
    }
}

#[cfg(feature = "forms")]
impl_display_custom_px! {
    RadioGroupSize { Sm => "sm", Default => "default", Lg => "lg" }
    RadioGroupRadius {
        None => "none", Small => "small", Medium => "medium",
        Large => "large", Full => "full",
    }
    SliderRadius {
        None => "none", Small => "small", Medium => "medium",
        Large => "large", Full => "full",
    }
    StarRatingSize { Sm => "sm", Default => "default", Md => "md", Lg => "lg", Xl => "xl" }
    SwitchSize { Sm => "sm", Default => "default" }
    SwitchRadius {
        None => "none", Small => "small", Medium => "medium",
        Large => "large", Full => "full",
    }
}

#[cfg(feature = "core")]
impl fmt::Display for SkeletonShape {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rounded(radius) => write!(formatter, "rounded-{radius}"),
            Self::Circle => formatter.write_str("circle"),
        }
    }
}

#[cfg(feature = "core")]
impl fmt::Display for SkeletonFill {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Semantic(color) => write!(formatter, "semantic({color:?})"),
            Self::Custom(color) => write!(
                formatter,
                "rgba({:.3}, {:.3}, {:.3}, {:.3})",
                color.r, color.g, color.b, color.a
            ),
        }
    }
}

#[cfg(all(test, feature = "core", feature = "forms", feature = "overlays", feature = "data"))]
mod tests {
    use super::*;

    #[test]
    fn display_matches_shadcn_kebab_tokens() {
        assert_eq!(ButtonVariant::Destructive.to_string(), "destructive");
        assert_eq!(ButtonSize::IconSm.to_string(), "icon-sm");
        assert_eq!(
            CardFooterAlignment::SpaceBetween.to_string(),
            "space-between"
        );
        assert_eq!(KbdSurface::InputGroup.to_string(), "input-group");
        assert_eq!(
            InputOtpPattern::DigitsAndChars.to_string(),
            "digits-and-chars"
        );
        assert_eq!(SpinnerVariant::TextShimmer.to_string(), "text-shimmer");
        assert_eq!(ToggleGroupType::Multiple.to_string(), "multiple");
        assert_eq!(ToggleGroupOrientation::Vertical.to_string(), "vertical");
        assert_eq!(TooltipSide::Bottom.to_string(), "bottom");
        assert_eq!(TooltipAlign::Start.to_string(), "start");
        assert_eq!(TypographyVariant::InlineCode.to_string(), "inline-code");
    }

    #[test]
    fn display_renders_custom_values_as_pixels() {
        assert_eq!(CardRadius::Custom(12.0).to_string(), "12px");
        assert_eq!(AvatarSize::Custom(48.5).to_string(), "48.5px");
        assert_eq!(
            SkeletonShape::Rounded(SkeletonRadius::Custom(4.0)).to_string(),
            "rounded-4px"
        );
        assert_eq!(SkeletonShape::Circle.to_string(), "circle");
    }
}
