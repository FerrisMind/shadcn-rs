//! Unit tests for the password component builders.

use iced::Element;
use shadcn_common::{PasswordAction, PasswordScore, PasswordState, password_reduce};

use super::{Password, PasswordCopy, PasswordInput, PasswordStrength, PasswordToggleVisibility};
use crate::Theme;

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Message {
    Password(PasswordAction),
    Copy(crate::CopyButtonAction),
}

#[test]
fn root_defaults_to_recipe_gap() {
    let theme = Theme::light();
    let root = Password::<Message>::new(&theme);
    assert!(format!("{root:?}").contains("Password"));
}

#[test]
fn input_redacts_value_in_debug() {
    let theme = Theme::light();
    let input = PasswordInput::<Message>::new(&theme).value("super-secret");
    let debug = format!("{input:?}");
    assert!(debug.contains("<redacted>"));
    assert!(!debug.contains("super-secret"));
}

#[test]
fn composed_suite_builds_element() {
    let theme = Theme::light();
    let state = PasswordState::new()
        .with_toggle_mounted(true)
        .with_strength_mounted(true)
        .with_value("secret");

    let element: Element<'_, Message> = Password::new(&theme)
        .push(
            PasswordInput::new(&theme)
                .value(state.value())
                .hidden(state.hidden())
                .invalid(state.is_invalid())
                .placeholder("Password")
                .on_input(|value| Message::Password(PasswordAction::SetValue(value)))
                .toggle(
                    PasswordToggleVisibility::new(&theme)
                        .hidden(state.hidden())
                        .on_toggle(|hidden| Message::Password(PasswordAction::SetHidden(hidden))),
                ),
        )
        .push(PasswordStrength::new(&theme).score(state.score()))
        .into();

    let _ = element;
}

#[test]
fn both_actions_build() {
    let theme = Theme::light();
    let state = PasswordState::new()
        .with_toggle_mounted(true)
        .with_copy_mounted(true)
        .with_value("c5xZTsVUs8HoLpBAajKGfbtG8SSbQAC6");

    let element: Element<'_, Message> = PasswordInput::new(&theme)
        .value(state.value())
        .hidden(state.hidden())
        .toggle(
            PasswordToggleVisibility::new(&theme)
                .hidden(state.hidden())
                .on_toggle(|hidden| Message::Password(PasswordAction::SetHidden(hidden))),
        )
        .copy(PasswordCopy::new(state.value(), &theme).on_copy(Message::Copy))
        .into();

    let _ = element;
}

#[test]
fn strength_scores_cover_palette() {
    let theme = Theme::light();
    for score in PasswordScore::ALL {
        let element: Element<'_, Message> = PasswordStrength::new(&theme).score(score).into();
        let _ = element;
    }
}

#[test]
fn state_roundtrip_matches_common() {
    let state = password_reduce(
        PasswordState::new().with_strength_mounted(true),
        PasswordAction::SetValue("password1".to_owned()),
    );
    assert!(state.is_invalid());
    assert_eq!(state.end_padding_px(), 0.0);

    let state = password_reduce(state, PasswordAction::MountToggle(true));
    assert_eq!(state.end_padding_px(), 36.0);

    let state = password_reduce(state, PasswordAction::MountCopy(true));
    assert_eq!(state.end_padding_px(), 72.0);
}

#[test]
fn password_children_follow_theme_style_pack() {
    // Extras Password has no pack tables, but composed Input / Toggle / Button do.
    // Selecting Rhea on the shared Theme must surface Rhea recipes to parts.
    let vega = Theme::light().with_style(shadcn_common::StyleId::Vega);
    let rhea = Theme::light().with_style(shadcn_common::StyleId::Rhea);
    let mira = Theme::light().with_style(shadcn_common::StyleId::Mira);

    assert_eq!(vega.style.password(), rhea.style.password());
    assert_ne!(
        vega.style.control_height_md_px,
        mira.style.control_height_md_px
    );
    assert_ne!(vega.style.button_type(), rhea.style.button_type());
    assert_ne!(vega.style.toggle(), mira.style.toggle());

    let _input = PasswordInput::<Message>::new(&rhea).placeholder("Password");
    let _toggle = PasswordToggleVisibility::<Message>::new(&rhea);
    let _copy = PasswordCopy::<Message>::new("secret", &rhea);
    assert_eq!(rhea.style_id(), shadcn_common::StyleId::Rhea);
}
