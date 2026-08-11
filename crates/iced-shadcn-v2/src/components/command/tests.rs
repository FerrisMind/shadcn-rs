//! Unit tests for the command builder and shared filter wiring.

use shadcn_common::{command_matches, default_command_filter, fuzzy_score};

use super::{Command, CommandGlyph, CommandGroup, CommandItem, CommandLoading, CommandRadius};
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
