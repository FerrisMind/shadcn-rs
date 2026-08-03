//! Unit tests for [`super::PhoneInput`].

use super::*;
use crate::theme::Theme;
use shadcn_common::{CountryCode, PhoneInputOptions};

#[test]
fn builder_defaults() {
    let theme = Theme::light();
    let input = PhoneInput::<()>::new(&theme);
    assert!(input.value.is_empty());
    assert_eq!(input.country, None);
    assert_eq!(input.options, PhoneInputOptions::DEFAULT);
    assert!(!input.disabled);
    assert!(!input.readonly);
    assert!(input.on_change.is_none());
}

#[test]
fn builder_setters() {
    let theme = Theme::light();
    let us = CountryCode::parse("US").expect("US");
    let input = PhoneInput::<String>::new(&theme)
        .value("+14185438090")
        .country(Some(us))
        .default_country(Some(us))
        .placeholder("Enter a phone number")
        .disabled(true)
        .readonly(true)
        .required(true)
        .invalid(true)
        .open(true)
        .query("united")
        .on_change(|change| change.value)
        .on_open_change(|open| open.to_string())
        .on_query_change(|query| query);

    assert_eq!(input.value, "+14185438090");
    assert_eq!(input.country, Some(us));
    assert_eq!(input.resolved_country(), Some(us));
    assert_eq!(input.placeholder, Some("Enter a phone number"));
    assert!(input.disabled);
    assert!(input.readonly);
    assert!(input.required);
    assert_eq!(input.invalid, Some(true));
    assert_eq!(input.open, Some(true));
    assert_eq!(input.query, "united");
    assert!(input.on_change.is_some());
    assert!(input.on_open_change.is_some());
    assert!(input.on_query_change.is_some());
}

#[test]
fn change_from_detailed_copies_fields() {
    let us = CountryCode::parse("US").expect("US");
    let detailed = shadcn_common::parse_phone_input("+1 418 543 8090", Some(us));
    let change = PhoneInputChange::from_detailed(detailed.clone(), Some(false));
    assert_eq!(change.country, Some(us));
    assert!(change.valid);
    assert_eq!(change.open, Some(false));
    assert_eq!(change.detailed, detailed);
}
