//! Behavioral tests for the alert-dialog component.

use shadcn_common::{
    ComponentRadius, DIALOG_ANIMATION_MS, DIALOG_MARGIN_PX, DIALOG_ZOOM_FROM, FontWeight, StyleId,
    ThemeMode, alert_dialog_recipe,
};

use super::style;
use super::types::AlertDialogState;
use super::*;
use crate::iced_compat::widget::container;
use crate::iced_compat::{Element, time::Duration};
use crate::theme::Theme;

#[derive(Clone, Debug, PartialEq)]
enum Message {
    Pressed,
    Confirmed,
    Cancelled,
    OpenChanged(bool),
    OpenSettled(bool),
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let dialog: AlertDialog<'_, Message> =
        AlertDialog::new(container("Open"), container("Body"), &theme)
            .footer(
                AlertDialogFooter::new(&theme)
                    .cancel(AlertDialogCancel::text("Cancel", &theme))
                    .action(AlertDialogAction::text("Continue", &theme)),
            )
            .size(AlertDialogSize::Sm)
            .max_width(425.0)
            .duration(Duration::from_millis(200))
            .animated(false)
            .disabled(true)
            .open(true)
            .default_open(true)
            .on_open_change(Message::OpenChanged)
            .on_open_change_complete(Message::OpenSettled)
            .close_on_click_outside(true)
            .close_on_escape(false);

    assert!(dialog.footer.is_some());
    assert_eq!(dialog.size, AlertDialogSize::Sm);
    assert_eq!(dialog.max_width, Some(425.0));
    assert_eq!(dialog.duration, Duration::from_millis(200));
    assert!(!dialog.animated);
    assert!(dialog.disabled);
    assert_eq!(dialog.open, Some(true));
    assert!(dialog.default_open);
    assert!(dialog.on_open_change.is_some());
    assert!(dialog.on_open_change_complete.is_some());
    assert!(dialog.close_on_click_outside);
    assert!(!dialog.close_on_escape);
    assert!(std::ptr::eq(dialog.theme, &theme));
}

#[test]
fn defaults_match_shadcn_svelte() {
    let theme = Theme::light();
    let dialog: AlertDialog<'_, Message> =
        AlertDialog::new(container("Open"), container("Body"), &theme);

    // Modal, `duration-100`, Esc closes, and — unlike the regular dialog —
    // backdrop clicks are ignored (`interactOutsideBehavior: "ignore"`).
    assert_eq!(dialog.size, AlertDialogSize::Default);
    assert_eq!(dialog.max_width, None);
    assert_eq!(dialog.duration, Duration::from_millis(DIALOG_ANIMATION_MS));
    assert!(dialog.animated);
    assert!(!dialog.disabled);
    assert_eq!(dialog.open, None);
    assert!(!dialog.default_open);
    assert!(!dialog.close_on_click_outside);
    assert!(dialog.close_on_escape);
    assert!(dialog.footer.is_none());
}

#[test]
fn dialogs_and_slots_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = AlertDialog::new(
        container("Show Dialog"),
        AlertDialogHeader::new(&theme)
            .media(AlertDialogMedia::new(container("!"), &theme))
            .title(AlertDialogTitle::text("Are you absolutely sure?", &theme))
            .description(AlertDialogDescription::text(
                "This action cannot be undone.",
                &theme,
            )),
        &theme,
    )
    .footer(
        AlertDialogFooter::new(&theme)
            .cancel(AlertDialogCancel::text("Cancel", &theme).on_press(Message::Cancelled))
            .action(AlertDialogAction::text("Continue", &theme).on_press(Message::Confirmed)),
    )
    .open(true)
    .on_open_change(Message::OpenChanged)
    .into();

    let _: Element<'_, Message> = AlertDialogHeader::new(&theme)
        .size(AlertDialogSize::Sm)
        .push(container("slot"))
        .into();
    let _: Element<'_, Message> = AlertDialogTitle::new(container("custom"), &theme).into();
    let _: Element<'_, Message> = AlertDialogDescription::new(container("custom"), &theme).into();
    let _: Element<'_, Message> = AlertDialogMedia::new(container("icon"), &theme).into();

    let _ = Message::Pressed;
    assert!(AlertDialogMedia::<Message>::icon_px(&theme) > 0.0);
}

#[test]
fn footer_items_wire_dismissal_and_open_change_fallback() {
    let theme = Theme::light();
    let on_open_change = |open: bool| Message::OpenChanged(open);

    // A cancel button without its own message falls back to publishing
    // `onOpenChange(false)` itself, so the overlay must not repeat it.
    let cancel: AlertDialogCancel<'_, Message> = AlertDialogCancel::text("Cancel", &theme);
    let child = AlertDialogFooterItem::Cancel(cancel).into_child(Some(&on_open_change), false);
    assert!(child.dismisses);
    assert!(child.publishes_open_change);

    // An action with its own message keeps `onOpenChange(false)` with the
    // overlay.
    let action = AlertDialogAction::text("Continue", &theme).on_press(Message::Confirmed);
    let child = AlertDialogFooterItem::Action(action).into_child(Some(&on_open_change), true);
    assert!(child.dismisses);
    assert!(!child.publishes_open_change);

    // Custom footer content never dismisses.
    let custom = AlertDialogFooterItem::Custom(container("hint").into());
    let child = custom.into_child(Some(&on_open_change), false);
    assert!(!child.dismisses);
    assert!(!child.publishes_open_change);
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
    assert_eq!(
        resolved.overlay,
        crate::iced_compat::Color::BLACK.scale_alpha(0.10)
    );
    // Vega casts no shadow on the alert-dialog surface.
    assert_eq!(resolved.shadow.blur_radius, 0.0);
    // `.cn-alert-dialog-media`: `bg-muted`.
    assert_eq!(resolved.media_background, theme.palette.muted);
}

#[test]
fn dark_mode_and_round_media_track_the_pack() {
    let dark = Theme::dark().with_style(StyleId::Rhea);
    assert_eq!(dark.mode(), ThemeMode::Dark);

    let resolved = style::resolve_style(&dark);
    // Rhea: `ring-foreground/5 dark:ring-foreground/10`, `shadow-xl`,
    // `rounded-[min(var(--radius-4xl),24px)]`, `rounded-full` media.
    assert_eq!(
        resolved.border_color,
        dark.palette.foreground.scale_alpha(0.10)
    );
    assert!(resolved.shadow.blur_radius > 0.0);
    assert_eq!(resolved.radius, 24.0);
    // `rounded-full` clamps to half the media box.
    let recipe = alert_dialog_recipe(StyleId::Rhea);
    assert_eq!(resolved.media_radius, recipe.media_size_px / 2.0);
}

#[test]
fn style_override_patches_resolved_style() {
    let theme = Theme::light();
    let dialog: AlertDialog<'_, Message> =
        AlertDialog::new(container("Open"), container("Body"), &theme).style_override(|style| {
            AlertDialogStyle {
                radius: 0.0,
                ..style
            }
        });

    let resolved =
        (dialog.style_override.as_ref().expect("override set"))(style::resolve_style(&theme));
    assert_eq!(resolved.radius, 0.0);
}

#[test]
fn recipe_tracks_style_pack_tokens() {
    // Vega: `bg-black/10`, `gap-6 rounded-xl p-6`, `sm:max-w-lg` and
    // `max-w-xs` at `size="sm"`, `text-lg font-medium` title.
    let vega = alert_dialog_recipe(StyleId::Vega);
    assert_eq!(vega.overlay_alpha, 0.10);
    assert_eq!(vega.max_width_px, 512.0);
    assert_eq!(vega.max_width_sm_px, 320.0);
    assert_eq!(vega.pad_px, 24.0);
    assert_eq!(vega.gap_px, 24.0);
    assert_eq!(vega.radius, ComponentRadius::Xl);
    assert_eq!(vega.title.size_px, 18.0);
    assert_eq!(vega.title.weight, FontWeight::Medium);
    assert_eq!(vega.media_size_px, 64.0);

    // Nova: compact `sm:max-w-sm p-4 gap-4` with a `text-base` title.
    let nova = alert_dialog_recipe(StyleId::Nova);
    assert_eq!(nova.max_width_px, 384.0);
    assert_eq!(nova.pad_px, 16.0);
    assert_eq!(nova.title.size_px, 16.0);

    // Maia: `bg-black/80` backdrop with `rounded-4xl` and round media.
    let maia = alert_dialog_recipe(StyleId::Maia);
    assert_eq!(maia.overlay_alpha, 0.80);
    assert_eq!(maia.radius, ComponentRadius::S4xl);
    assert_eq!(maia.radius_px, None);
    assert_eq!(maia.media_radius, ComponentRadius::Full);

    // Sera: square with an uppercase wide-tracked title.
    let sera = alert_dialog_recipe(StyleId::Sera);
    assert_eq!(sera.radius, ComponentRadius::None);
    assert!(sera.title.uppercase);
    assert_eq!(sera.description_margin_top_px, 2.0);
}

#[test]
fn state_visibility_follows_open_and_transition() {
    let mut state = AlertDialogState::new(false);
    assert!(!state.is_visible());
    assert!(!state.requested_open);
    assert_eq!(state.pressed_footer, None);

    state.open = true;
    assert!(state.is_visible());

    state.open = false;
    state.transition.reset(0.4);
    assert!(state.is_visible());

    state.transition.reset(0.0);
    assert!(!state.is_visible());

    // `defaultOpen` seeds the uncontrolled intent only.
    let state = AlertDialogState::new(true);
    assert!(state.requested_open);
    assert!(!state.open);
}

#[test]
fn animation_and_geometry_constants_match_the_web() {
    // The alert dialog shares `duration-100`, `zoom-in-95`, and
    // `max-w-[calc(100%-2rem)]` with the dialog.
    assert_eq!(DIALOG_ANIMATION_MS, 100);
    assert_eq!(DIALOG_ZOOM_FROM, 0.95);
    assert_eq!(DIALOG_MARGIN_PX, 16.0);
}
