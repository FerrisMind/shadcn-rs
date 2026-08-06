//! Snapshot emitted when the phone value, country, or validity changes.

use shadcn_common::{CountryCode, DetailedPhoneValue};

/// Aggregated phone-input update mirroring the web bindable props.
///
/// Prefer wiring [`super::PhoneInput::on_change`] and deriving UI state from
/// this snapshot so a single country pick or keystroke cannot desync
/// `value` / `country` / `valid` / `detailedValue`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhoneInputChange {
    /// Next controlled value (formatted when valid, otherwise the raw edit).
    pub value: String,
    /// Next selected country.
    pub country: Option<CountryCode>,
    /// Whether [`Self::value`] is a valid phone number.
    pub valid: bool,
    /// Full parse snapshot (`detailedValue`).
    pub detailed: DetailedPhoneValue,
    /// When `Some`, the country popover should adopt this open state
    /// (typically `false` after a country is picked).
    pub open: Option<bool>,
}

impl PhoneInputChange {
    /// Builds a change from a detailed parse result.
    #[must_use]
    pub fn from_detailed(detailed: DetailedPhoneValue, open: Option<bool>) -> Self {
        Self {
            value: detailed.number.clone().unwrap_or_default(),
            country: detailed.country_code,
            valid: detailed.valid,
            detailed,
            open,
        }
    }
}
