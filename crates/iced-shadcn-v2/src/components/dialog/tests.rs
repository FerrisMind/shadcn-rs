//! Behavioral tests for the dialog component.

use shadcn_common::{
    ComponentRadius, DIALOG_ANIMATION_MS, DIALOG_CLOSE_ICON_PX, DIALOG_CLOSE_SIZE_PX,
    DIALOG_MARGIN_PX, DIALOG_ZOOM_FROM, StyleId, ThemeMode, dialog_recipe,
};

use super::style;
use super::types::DialogState;
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
    let dialog: Dialog<'_, Message> = Dialog::new(container("Open"), container("Body"), &theme)
        .footer(DialogFooter::new(&theme).push(container("Save")))
        .max_width(425.0)
        .content_padding(0.0)
        .vertical_anchor_top(1.0 / 3.0)
        .duration(Duration::from_millis(200))
        .animated(false)
        .disabled(true)
        .open(true)
        .default_open(true)
        .on_open_change(Message::OpenChanged)
        .show_close_button(false)
        .close_on_click_outside(false)
        .close_on_escape(false)
        .modal(false);

    assert!(dialog.footer.is_some());
    assert_eq!(dialog.max_width, Some(425.0));
    assert_eq!(dialog.content_padding, Some(0.0));
    assert_eq!(dialog.vertical_anchor_top, Some(1.0 / 3.0));
    assert_eq!(dialog.duration, Duration::from_millis(200));
    assert!(!dialog.animated);
    assert!(dialog.disabled);
    assert_eq!(dialog.open, Some(true));
    assert!(dialog.default_open);
    assert!(dialog.on_open_change.is_some());
    assert!(!dialog.show_close_button);
    assert!(!dialog.close_on_click_outside);
    assert!(!dialog.close_on_escape);
    assert!(!dialog.modal);
    assert!(std::ptr::eq(dialog.theme, &theme));
}

#[test]
fn defaults_match_shadcn_svelte() {
    let theme = Theme::light();
    let dialog: Dialog<'_, Message> = Dialog::new(container("Open"), container("Body"), &theme);

    // Modal, dismissable, animated `duration-100`, `showCloseButton = true`.
    assert_eq!(dialog.max_width, None);
    assert_eq!(dialog.duration, Duration::from_millis(DIALOG_ANIMATION_MS));
    assert!(dialog.animated);
    assert!(!dialog.disabled);
    assert_eq!(dialog.open, None);
    assert!(!dialog.default_open);
    assert!(dialog.show_close_button);
    assert!(dialog.close_on_click_outside);
    assert!(dialog.close_on_escape);
    assert!(dialog.modal);
    assert!(dialog.footer.is_none());
}

#[test]
fn dialogs_and_slots_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = Dialog::new(
        container("Open"),
        DialogHeader::new(&theme)
            .title(DialogTitle::text("Edit profile", &theme))
            .description(DialogDescription::text(
                "Make changes to your profile here.",
                &theme,
            )),
        &theme,
    )
    .footer(DialogFooter::new(&theme).push(container("Save")))
    .open(true)
    .on_open_change(Message::OpenChanged)
    .into();

    let _: Element<'_, Message> = DialogHeader::new(&theme).push(container("slot")).into();
    let _: Element<'_, Message> = DialogFooter::new(&theme).push(container("slot")).into();
    let _: Element<'_, Message> = DialogTitle::new(container("custom"), &theme).into();
    let _: Element<'_, Message> = DialogDescription::new(container("custom"), &theme).into();

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
    // `ring-foreground/10`: the foreground color at a tenth of its alpha.
    assert_eq!(
        resolved.border_color,
        theme.palette.foreground.scale_alpha(0.10)
    );
    // `bg-black/10` backdrop for Vega.
    assert_eq!(resolved.overlay, Color::BLACK.scale_alpha(0.10));
    // Vega casts no shadow on the dialog surface.
    assert_eq!(resolved.shadow.blur_radius, 0.0);
    // Ghost close button: transparent at rest, `bg-accent` on hover.
    assert_eq!(resolved.close_background, Color::TRANSPARENT);
    assert_eq!(resolved.close_hover_background, theme.palette.accent);
}

#[test]
fn dark_mode_and_secondary_close_track_the_pack() {
    let dark = Theme::dark().with_style(StyleId::Rhea);
    assert_eq!(dark.mode(), ThemeMode::Dark);

    let resolved = style::resolve_style(&dark);
    // Rhea: `ring-foreground/5 dark:ring-foreground/10`, `bg-secondary`
    // close button, `shadow-xl`.
    assert_eq!(
        resolved.border_color,
        dark.palette.foreground.scale_alpha(0.10)
    );
    assert_eq!(resolved.close_background, dark.palette.secondary);
    assert!(resolved.shadow.blur_radius > 0.0);
    // `rounded-[min(var(--radius-4xl),24px)]`.
    assert_eq!(resolved.radius, 24.0);
}

#[test]
fn style_override_patches_resolved_style() {
    let theme = Theme::light();
    let dialog: Dialog<'_, Message> = Dialog::new(container("Open"), container("Body"), &theme)
        .style_override(|style| DialogStyle {
            radius: 0.0,
            ..style
        });

    let resolved =
        (dialog.style_override.as_ref().expect("override set"))(style::resolve_style(&theme));
    assert_eq!(resolved.radius, 0.0);
}

#[test]
fn luma_and_maia_use_theme_radius_4xl_not_literal_32() {
    let luma = Theme::light().with_style(StyleId::Luma);
    let maia = Theme::light().with_style(StyleId::Maia);
    let expected = luma.style.radius.xxxxl_px;

    assert_eq!(style::resolve_style(&luma).radius, expected);
    assert_eq!(style::resolve_style(&maia).radius, expected);
    // Not the Tailwind default literal for `rounded-4xl`.
    assert_ne!(expected, 32.0);
}

#[test]
fn recipe_tracks_style_pack_tokens() {
    // Vega: `bg-black/10`, `gap-6 rounded-xl p-6 sm:max-w-md`, close at
    // `top-4 right-4`.
    let vega = dialog_recipe(StyleId::Vega);
    assert_eq!(vega.overlay_alpha, 0.10);
    assert_eq!(vega.max_width_px, 448.0);
    assert_eq!(vega.pad_px, 24.0);
    assert_eq!(vega.gap_px, 24.0);
    assert_eq!(vega.radius, ComponentRadius::Xl);
    assert_eq!(vega.close_offset_px, 16.0);
    assert!(!vega.footer_bar);

    // Nova: compact `sm:max-w-sm p-4 gap-4` with the muted footer bar.
    let nova = dialog_recipe(StyleId::Nova);
    assert_eq!(nova.max_width_px, 384.0);
    assert_eq!(nova.pad_px, 16.0);
    assert!(nova.footer_bar);

    // Maia: `bg-black/80` backdrop with `rounded-4xl` (`--radius-4xl`).
    let maia = dialog_recipe(StyleId::Maia);
    assert_eq!(maia.overlay_alpha, 0.80);
    assert_eq!(maia.radius, ComponentRadius::S4xl);
    assert_eq!(maia.radius_px, None);

    // Sera: square with an uppercase wide-tracked title.
    let sera = dialog_recipe(StyleId::Sera);
    assert_eq!(sera.radius, ComponentRadius::None);
    assert!(sera.title.uppercase);
    assert!(sera.close_secondary_bg);
}

#[test]
fn state_visibility_follows_open_and_transition() {
    let mut state = DialogState::new(false);
    assert!(!state.is_visible());
    assert!(!state.requested_open);

    state.open = true;
    assert!(state.is_visible());

    state.open = false;
    state.transition.reset(0.4);
    assert!(state.is_visible());

    state.transition.reset(0.0);
    assert!(!state.is_visible());

    // `defaultOpen` seeds the uncontrolled intent only.
    let state = DialogState::new(true);
    assert!(state.requested_open);
    assert!(!state.open);
}

#[test]
fn animation_and_geometry_constants_match_the_web() {
    // `duration-100`, `zoom-in-95`, `max-w-[calc(100%-2rem)]`, `icon-sm`
    // close button with a `size-4` glyph.
    assert_eq!(DIALOG_ANIMATION_MS, 100);
    assert_eq!(DIALOG_ZOOM_FROM, 0.95);
    assert_eq!(DIALOG_MARGIN_PX, 16.0);
    assert_eq!(DIALOG_CLOSE_SIZE_PX, 32.0);
    assert_eq!(DIALOG_CLOSE_ICON_PX, 16.0);
}
