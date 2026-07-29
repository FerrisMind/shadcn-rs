//! Tests for checkbox component.

use super::Checkbox;
use super::types;
use crate::iced_compat::Element;

#[derive(Debug, Clone)]
enum Message {
    Changed,
    Pressed,
}

#[test]
fn test_state_cycle() {
    let state = types::CheckboxState::Unchecked;
    assert_eq!(state.cycle(), types::CheckboxState::Checked);
    let state2 = types::CheckboxState::Checked;
    assert_eq!(state2.cycle(), types::CheckboxState::Indeterminate);
    let state3 = types::CheckboxState::Indeterminate;
    assert_eq!(state3.cycle(), types::CheckboxState::Unchecked);
}

#[test]
fn test_default() {
    let default = types::CheckboxConfig::default();
    assert_eq!(default.state, types::CheckboxState::Unchecked);
}

#[test]
fn test_config_is_cloneable() {
    let config = types::CheckboxConfig {
        state: types::CheckboxState::Indeterminate,
        variant: types::CheckboxVariant::Soft,
        size: types::CheckboxSize::Sm,
        label: Some("Notifications".to_owned()),
        disabled: true,
    };

    assert_eq!(config.clone(), config);
}

#[test]
fn builder_supports_controlled_callbacks_and_press_messages() {
    let theme = crate::theme::Theme::light();

    let _: Element<'_, Message> = Checkbox::new(&theme)
        .label("Terms")
        .on_toggle(|_| Message::Changed)
        .into();
    let _: Element<'_, Message> = Checkbox::new(&theme).on_press(Message::Pressed).into();
}
