//! Behavioral tests for the tabs component.

use crate::iced_compat::widget::container;
use crate::iced_compat::{Element, Length, Padding};
use crate::{Tabs, TabsActivationMode, TabsContent, TabsList, TabsListLoop, TabsListVariant};
use crate::{TabsOrientation, TabsSize, TabsTrigger, TabsWrap, Theme};

use super::render::{
    TabsTriggerMeta, first_enabled_index, next_enabled_index, resolve_active_index,
    resolve_active_value,
};

#[derive(Debug, Clone)]
enum Message {
    Changed,
}

#[test]
fn defaults_match_the_web_component() {
    let theme = Theme::light();
    let tabs = Tabs::<Message>::new(&theme);

    assert_eq!(tabs.active_value(), "");
    assert!(tabs.is_empty());
    assert!(format!("{tabs:?}").contains("orientation: Horizontal"));
    assert_eq!(TabsListLoop::default(), TabsListLoop::Disabled);

    let list = TabsList::<Message>::new(&theme);
    assert_eq!(list.variant, TabsListVariant::Default);
    assert_eq!(list.size, TabsSize::Default);
}

#[test]
fn selected_value_falls_back_to_the_first_enabled_trigger() {
    let theme = Theme::light();
    let triggers: Vec<TabsTrigger<'_, Message>> = vec![
        TabsTrigger::text("disabled", "Disabled", &theme).disabled(true),
        TabsTrigger::text("account", "Account", &theme),
        TabsTrigger::text("password", "Password", &theme),
    ];

    assert_eq!(resolve_active_value(&triggers, "missing"), "account");
    assert_eq!(resolve_active_value(&triggers, "password"), "password");
    assert_eq!(resolve_active_value(&triggers, "disabled"), "account");

    let items = triggers
        .iter()
        .map(|trigger| TabsTriggerMeta {
            value: trigger.value.clone(),
            disabled: trigger.disabled,
        })
        .collect::<Vec<_>>();
    assert_eq!(resolve_active_index(&items, "missing"), Some(1));
    assert_eq!(first_enabled_index(&items), Some(1));
}

#[test]
fn arrow_navigation_skips_disabled_items_and_honors_looping() {
    let items = [
        TabsTriggerMeta {
            value: "account".to_owned(),
            disabled: false,
        },
        TabsTriggerMeta {
            value: "disabled".to_owned(),
            disabled: true,
        },
        TabsTriggerMeta {
            value: "password".to_owned(),
            disabled: false,
        },
    ];

    assert_eq!(next_enabled_index(&items, 0, 1, false), Some(2));
    assert_eq!(next_enabled_index(&items, 2, 1, false), None);
    assert_eq!(next_enabled_index(&items, 2, 1, true), Some(0));
    assert_eq!(next_enabled_index(&items, 0, -1, true), Some(2));
}

#[test]
fn builder_exposes_controlled_and_compositional_api() {
    let theme = Theme::light();
    let tabs = Tabs::with_children(
        &theme,
        [
            TabsTrigger::text("account", "Account", &theme),
            TabsTrigger::text("password", "Password", &theme),
        ],
        [
            TabsContent::text("account", "Account settings", &theme),
            TabsContent::text("password", "Password settings", &theme),
        ],
    )
    .value("account")
    .orientation(TabsOrientation::Vertical)
    .activation_mode(TabsActivationMode::Manual)
    .list_loop(TabsListLoop::Disabled)
    .on_value_change(|_| Message::Changed);

    let _: Element<'_, Message> = tabs.into();
}

#[test]
fn arbitrary_content_and_overrides_are_supported() {
    let theme = Theme::light();
    let trigger = TabsTrigger::new("custom", container("Custom"), &theme)
        .width(Length::Fixed(140.0))
        .height(Length::Fixed(42.0))
        .padding(Padding::from([2.0, 8.0]))
        .style_override(|mut style, _| {
            style.border.width = 1.0;
            style
        });
    let content = TabsContent::new("custom", container("Panel"), &theme)
        .padding(Padding::from([4.0, 8.0]))
        .style_override(|mut style| {
            style.border.width = 1.0;
            style
        });

    let _: Element<'_, Message> = Tabs::new(&theme)
        .list(TabsList::new(&theme).push(trigger).wrap(TabsWrap::Wrap))
        .push(content)
        .value("custom")
        .into();
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebugMessage;

    let theme = Theme::light();
    let tabs = Tabs::<NoDebugMessage>::new(&theme)
        .list(TabsList::new(&theme).push(TabsTrigger::text("home", "Home", &theme)));

    let debug = format!("{tabs:?}");
    assert!(debug.contains("Tabs"));
    assert!(debug.contains("home"));
}
