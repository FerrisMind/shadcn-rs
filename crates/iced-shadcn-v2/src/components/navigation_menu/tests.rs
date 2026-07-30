//! Smoke tests for the navigation-menu public API.

use super::*;
use crate::iced_compat::widget::text;
use crate::theme::Theme;
use shadcn_common::{StyleId, navigation_menu_recipe};

#[test]
fn builder_defaults_match_bits_ui() {
    let theme = Theme::light();
    let menu = NavigationMenu::<()>::new(&theme)
        .viewport(true)
        .indicator(false)
        .timing(NavigationMenuTiming::default());
    let _ = menu;
    assert_eq!(NavigationMenuTiming::default().delay_duration_ms, 200);
    assert_eq!(NavigationMenuTiming::default().skip_delay_duration_ms, 300);
}

#[test]
fn trigger_builder_attaches_content() {
    let theme = Theme::light();
    let item = NavigationMenuItem::<()>::trigger("home", "Home")
        .content(navigation_menu_content(text("Hi"), &theme));
    match item {
        NavigationMenuItem::Trigger { value, .. } => assert_eq!(value, "home"),
        NavigationMenuItem::Link { .. } => panic!("expected trigger"),
    }
}

#[test]
fn recipe_varies_by_style_pack() {
    let vega = navigation_menu_recipe(StyleId::Vega);
    let lyra = navigation_menu_recipe(StyleId::Lyra);
    assert_ne!(vega.trigger_radius, lyra.trigger_radius);
}
