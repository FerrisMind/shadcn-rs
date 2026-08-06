//! Behavioral tests for the sheet component.

use shadcn_common::{
    SHEET_ANIMATION_MS, SHEET_CLOSE_ICON_PX, SHEET_CLOSE_SIZE_PX, SHEET_MAX_WIDTH_PX,
    SHEET_SLIDE_PX, SheetSide, StyleId, ThemeMode, sheet_recipe,
};

use super::style;
use super::types::SheetState;
use super::*;
use crate::iced_compat::widget::container;
use crate::iced_compat::{Color, Element, time::Duration};
use crate::theme::Theme;

#[derive(Clone, Debug)]
enum Message {
    Pressed,
    OpenChanged(bool),
}

impl Message {
    fn is_open(&self) -> bool {
        matches!(self, Self::OpenChanged(true))
    }
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let sheet: Sheet<'_, Message> = Sheet::new(container("Open"), container("Body"), &theme)
        .footer(SheetFooter::new(&theme).push(container("Save")))
        .side(SheetSide::Left)
        .max_width(320.0)
        .max_height(400.0)
        .duration(Duration::from_millis(100))
        .animated(false)
        .disabled(true)
        .open(true)
        .default_open(true)
        .on_open_change(Message::OpenChanged)
        .show_close_button(false)
        .close_on_click_outside(false)
        .close_on_escape(false)
        .modal(false);

    assert!(sheet.footer.is_some());
    assert_eq!(sheet.side, SheetSide::Left);
    assert_eq!(sheet.max_width, Some(320.0));
    assert_eq!(sheet.max_height, Some(400.0));
    assert_eq!(sheet.duration, Duration::from_millis(100));
    assert!(!sheet.animated);
    assert!(sheet.disabled);
    assert_eq!(sheet.open, Some(true));
    assert!(sheet.default_open);
    assert!(sheet.on_open_change.is_some());
    assert!(!sheet.show_close_button);
    assert!(!sheet.close_on_click_outside);
    assert!(!sheet.close_on_escape);
    assert!(!sheet.modal);
    assert!(std::ptr::eq(sheet.theme, &theme));
}

#[test]
fn defaults_match_shadcn_svelte() {
    let theme = Theme::light();
    let sheet: Sheet<'_, Message> = Sheet::new(container("Open"), container("Body"), &theme);

    assert_eq!(sheet.side, SheetSide::Right);
    assert_eq!(sheet.max_width, None);
    assert_eq!(sheet.max_height, None);
    assert_eq!(sheet.duration, Duration::from_millis(SHEET_ANIMATION_MS));
    assert!(sheet.animated);
    assert!(!sheet.disabled);
    assert_eq!(sheet.open, None);
    assert!(!sheet.default_open);
    assert!(sheet.show_close_button);
    assert!(sheet.close_on_click_outside);
    assert!(sheet.close_on_escape);
    assert!(sheet.modal);
    assert!(sheet.footer.is_none());
}

#[test]
fn sheets_and_slots_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = Sheet::new(
        container("Open"),
        SheetHeader::new(&theme)
            .title(SheetTitle::text("Edit profile", &theme))
            .description(SheetDescription::text(
                "Make changes to your profile here.",
                &theme,
            )),
        &theme,
    )
    .footer(SheetFooter::new(&theme).push(container("Save")))
    .side(SheetSide::Bottom)
    .max_height(400.0)
    .open(true)
    .on_open_change(Message::OpenChanged)
    .into();

    let _: Element<'_, Message> = SheetBody::new(&theme).push(container("form")).into();

    let _: Element<'_, Message> = SheetHeader::new(&theme).push(container("slot")).into();
    let _: Element<'_, Message> = SheetFooter::new(&theme).push(container("slot")).into();
    let _: Element<'_, Message> = SheetTitle::new(container("custom"), &theme).into();
    let _: Element<'_, Message> = SheetDescription::new(container("custom"), &theme).into();

    let _ = Message::Pressed;
    assert!(Message::OpenChanged(true).is_open());
    assert!(!Message::OpenChanged(false).is_open());
}

#[test]
fn style_uses_popover_pair_over_black_backdrop() {
    let theme = Theme::light();
    let resolved = style::resolve_style(&theme);

    assert_eq!(resolved.background, theme.palette.popover);
    assert_eq!(resolved.text_color, theme.palette.popover_foreground);
    assert_eq!(resolved.border_width, 1.0);
    assert_eq!(resolved.border_color, theme.palette.border);
    assert_eq!(resolved.overlay, Color::BLACK.scale_alpha(0.10));
    assert!(resolved.shadow.blur_radius > 0.0);
    assert_eq!(resolved.close_background, Color::TRANSPARENT);
    assert_eq!(resolved.close_hover_background, theme.palette.accent);
}

#[test]
fn dark_mode_and_secondary_close_track_the_pack() {
    let dark = Theme::dark().with_style(StyleId::Rhea);
    assert_eq!(dark.mode(), ThemeMode::Dark);

    let resolved = style::resolve_style(&dark);
    assert_eq!(resolved.close_background, dark.palette.secondary);
    assert!(resolved.shadow.blur_radius > 0.0);
    assert_eq!(resolved.overlay, Color::BLACK.scale_alpha(0.30));
}

#[test]
fn style_override_patches_resolved_style() {
    let theme = Theme::light();
    let sheet: Sheet<'_, Message> = Sheet::new(container("Open"), container("Body"), &theme)
        .style_override(|mut style| {
            style.overlay = Color::from_rgba(0.0, 0.0, 0.0, 0.5);
            style
        });

    assert!(sheet.style_override.is_some());
}

#[test]
fn state_visibility_follows_transition() {
    let mut state = SheetState::new(false);
    assert!(!state.is_visible());

    state.requested_open = true;
    state.open = true;
    state.transition.reset(1.0);
    assert!(state.is_visible());
    assert_eq!(state.progress(), 1.0);
}

#[test]
fn recipe_constants_match_web() {
    assert_eq!(SHEET_ANIMATION_MS, 200);
    assert_eq!(SHEET_SLIDE_PX, 40.0);
    assert_eq!(SHEET_MAX_WIDTH_PX, 384.0);
    assert_eq!(SHEET_CLOSE_SIZE_PX, 32.0);
    assert_eq!(SHEET_CLOSE_ICON_PX, 16.0);

    let vega = sheet_recipe(StyleId::Vega);
    assert_eq!(vega.max_width_px, SHEET_MAX_WIDTH_PX);
}
