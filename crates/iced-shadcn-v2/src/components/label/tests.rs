//! Behavioral tests for the label component.

use crate::iced_compat::widget::{container, text};
use crate::iced_compat::{Color, Element, Length};
use shadcn_common::{FontWeight, StyleId};

use super::style;
use super::*;
use crate::theme::Theme;

#[derive(Clone, Debug)]
enum Message {
    Focus,
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let label: Label<'_, Message> = Label::text("Email", &theme)
        .context(LabelContext::AdjacentControl)
        .color(Color::WHITE)
        .width(Length::Fill)
        .disabled(true)
        .for_id("email")
        .on_press(Message::Focus)
        .icon_start(text("*"))
        .icon_end(text("?"));

    assert!(matches!(label.content, LabelContent::Text(_)));
    assert_eq!(label.context, LabelContext::AdjacentControl);
    assert_eq!(label.color, Some(Color::WHITE));
    assert_eq!(label.width, Length::Fill);
    assert!(label.disabled);
    assert_eq!(label.associated_id(), Some("email"));
    assert!(label.on_press.is_some());
    assert!(label.icon_start.is_some());
    assert!(label.icon_end.is_some());
    assert!(std::ptr::eq(label.theme, &theme));
}

#[test]
fn text_and_generic_labels_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = Label::new(container(text("Custom")), &theme).into();
    let _: Element<'_, Message> = Label::text("Email", &theme).on_press(Message::Focus).into();
    let _: Element<'_, Message> = Label::text("Disabled", &theme).disabled(true).into();
}

#[test]
fn vega_defaults_come_from_shadcn_common() {
    let theme = Theme::light().with_style(StyleId::Vega);
    let recipe = style::resolve_recipe(&theme, LabelContext::Field);

    assert_eq!(recipe.typography.size_px, 14.0);
    assert_eq!(recipe.typography.weight, FontWeight::Medium);
    assert!(!recipe.typography.uppercase);
}

#[test]
fn lyra_uses_compact_normal_weight() {
    let theme = Theme::light().with_style(StyleId::Lyra);
    let recipe = style::resolve_recipe(&theme, LabelContext::Field);

    assert_eq!(recipe.typography.size_px, 12.0);
    assert_eq!(recipe.typography.weight, FontWeight::Normal);
}

#[test]
fn sera_field_and_peer_come_from_common() {
    let theme = Theme::light().with_style(StyleId::Sera);
    let field = style::resolve_recipe(&theme, LabelContext::Field);
    assert!(field.typography.uppercase);
    assert_eq!(field.typography.weight, FontWeight::Semibold);

    let peer = style::resolve_recipe(&theme, LabelContext::AdjacentControl);
    assert!(!peer.typography.uppercase);
    assert!((peer.typography.line_height_px - 20.0).abs() < f32::EPSILON);
}

#[test]
fn disabled_halves_foreground_alpha() {
    let theme = Theme::light();
    let color = style::resolve_color(&theme, None, true);
    assert!((color.a - theme.palette.foreground.a * 0.5).abs() < 1e-4);
}

#[test]
fn color_override_beats_theme_foreground() {
    let theme = Theme::light();
    let color = style::resolve_color(&theme, Some(Color::WHITE), false);
    assert_eq!(color, Color::WHITE);
}

#[test]
fn debug_is_nonempty_without_message_debug() {
    struct NoDebugMessage;

    impl Clone for NoDebugMessage {
        fn clone(&self) -> Self {
            Self
        }
    }

    let theme = Theme::light();
    let label = Label::<NoDebugMessage>::text("Email", &theme);
    let debug = format!("{label:?}");
    assert!(debug.contains("Label"));
    assert!(debug.contains("content"));
}

#[test]
fn label_context_defaults_to_field() {
    assert_eq!(LabelContext::default(), LabelContext::Field);
}
