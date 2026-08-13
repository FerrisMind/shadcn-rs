//! Unit tests for the command builder and shared filter wiring.

use shadcn_common::{
    ComponentRadius, StyleId, command_matches, command_recipe, default_command_filter, fuzzy_score,
};

use super::style::{self, resolve_style};
use super::{
    Command, CommandDialog, CommandGlyph, CommandGroup, CommandItem, CommandLoading, CommandRadius,
};
use crate::components::button::Button;
use crate::iced_compat::widget::text;
use crate::theme::Theme;

#[test]
fn builder_defaults_match_shadcn() {
    let theme = Theme::light();
    let command = Command::<&str, ()>::new(&theme)
        .placeholder("Search...")
        .empty("No results found.")
        .group(
            CommandGroup::new("Suggestions")
                .item(
                    CommandItem::new("calendar", "Calendar")
                        .icon(CommandGlyph::Calendar)
                        .keywords(["date"]),
                )
                .item(CommandItem::new("emoji", "Search Emoji").disabled(true)),
        )
        .separator()
        .loading(CommandLoading::new("Loading...").progress(0.5))
        .radius(CommandRadius::Xl)
        .max_height(240.0)
        .should_filter(true);

    assert_eq!(command.placeholder, "Search...");
    assert!(command.should_filter);
    assert_eq!(command.rows.len(), 3);
    assert_eq!(command.max_height, 240.0);
}

#[test]
fn input_adornment_slots_are_optional() {
    let theme = Theme::light();
    let bare = Command::<&str, ()>::new(&theme);
    assert!(bare.input_leading.is_none());
    assert!(bare.input_trailing.is_none());

    let with_slots = Command::<&str, ()>::new(&theme)
        .input_leading(text("L"))
        .input_trailing(text("T"));
    assert!(with_slots.input_leading.is_some());
    assert!(with_slots.input_trailing.is_some());
}

#[test]
fn filter_helpers_score_keywords() {
    assert!(fuzzy_score("set", "settings") > 0.0);
    assert!(command_matches(
        "pay",
        "Billing",
        &["payments"],
        true,
        default_command_filter
    ));
    assert!(!command_matches(
        "zzz",
        "Billing",
        &["payments"],
        true,
        default_command_filter
    ));
}

#[test]
fn command_recipe_differs_across_style_packs() {
    // command.json is pack-specific (unlike form.json). Rhea uses
    // rounded-3xl; Vega uses rounded-xl; Lyra is sharp.
    assert_eq!(command_recipe(StyleId::Vega).radius, ComponentRadius::Xl);
    assert_eq!(command_recipe(StyleId::Rhea).radius, ComponentRadius::S3xl);
    assert_eq!(command_recipe(StyleId::Lyra).radius, ComponentRadius::None);
    assert_ne!(
        command_recipe(StyleId::Vega).input_fill_alpha,
        command_recipe(StyleId::Rhea).input_fill_alpha
    );
}

#[test]
fn command_and_composed_parts_follow_theme_style_pack() {
    // Command owns its recipe; Dialog / Button resolve through the same
    // Theme.style_id() — composite rule when a host has no pack deltas.
    use shadcn_common::dialog_recipe;

    let vega = Theme::light().with_style(StyleId::Vega);
    let rhea = Theme::light().with_style(StyleId::Rhea);

    assert_eq!(style::recipe(&vega).radius, ComponentRadius::Xl);
    assert_eq!(style::recipe(&rhea).radius, ComponentRadius::S3xl);

    let vega_style = resolve_style(&vega, None, false, true, true);
    let rhea_style = resolve_style(&rhea, None, false, true, true);
    assert_ne!(vega_style.radius, rhea_style.radius);
    assert_ne!(vega_style.input_radius, rhea_style.input_radius);

    assert_ne!(vega.style.button_type(), rhea.style.button_type());
    assert_ne!(
        dialog_recipe(vega.style_id()).radius,
        dialog_recipe(rhea.style_id()).radius
    );

    let trigger = Button::text("Open", &rhea);
    let command = Command::<&str, ()>::new(&rhea)
        .item(CommandItem::new("a", "Alpha"))
        .empty("No results found.");
    let _dialog = CommandDialog::new(trigger, command, &rhea);
    assert_eq!(rhea.style_id(), StyleId::Rhea);
}
