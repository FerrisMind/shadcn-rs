//! Backend-agnostic phone-input behaviour for shadcn-svelte-extras `PhoneInput`.
//!
//! Ports the pure country-list / E.164 parse / format / validate layer so iced
//! and egui share one source of truth. Parsing is backed by [`phonelib`]
//! (dependency-free country tables + E.164 normalisation).

use std::cmp::Ordering;
use std::fmt;

use phonelib::{
    PhoneFormat, PhoneNumber, countries as phonelib_countries, country_by_code,
    format_phone_number, is_valid_phone_number,
};

/// ISO 3166-1 alpha-2 country code used by the web `CountryCode` type.
///
/// Stored as two ASCII uppercase letters. Invalid inputs are rejected by
/// [`CountryCode::parse`] / [`TryFrom`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CountryCode([u8; 2]);

impl CountryCode {
    /// Creates a code from two uppercase ASCII letters without validation.
    ///
    /// Prefer [`Self::parse`] at API boundaries.
    #[must_use]
    pub const fn from_bytes_unchecked(bytes: [u8; 2]) -> Self {
        Self(bytes)
    }

    /// Parses a two-letter ISO country code (case-insensitive).
    ///
    /// # Errors
    ///
    /// Returns [`PhoneInputError::InvalidCountryCode`] when the string is not
    /// exactly two ASCII alphabetic characters.
    pub fn parse(code: &str) -> Result<Self, PhoneInputError> {
        let bytes = code.as_bytes();
        if bytes.len() != 2 || !bytes[0].is_ascii_alphabetic() || !bytes[1].is_ascii_alphabetic() {
            return Err(PhoneInputError::InvalidCountryCode);
        }
        Ok(Self([
            bytes[0].to_ascii_uppercase(),
            bytes[1].to_ascii_uppercase(),
        ]))
    }

    /// Returns the two-letter code as a `&str`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        // SAFETY: bytes are always ASCII A–Z from construction.
        std::str::from_utf8(&self.0).unwrap_or("ZZ")
    }

    /// Returns the raw ASCII bytes.
    #[must_use]
    pub const fn as_bytes(self) -> [u8; 2] {
        self.0
    }
}

impl fmt::Debug for CountryCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CountryCode").field(&self.as_str()).finish()
    }
}

impl fmt::Display for CountryCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<&str> for CountryCode {
    type Error = PhoneInputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl AsRef<str> for CountryCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// One country row matching svelte-tel-input's `Country` shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhoneCountry {
    /// ISO 3166-1 alpha-2 code.
    pub iso2: CountryCode,
    /// English display name.
    pub name: &'static str,
    /// E.164 calling / dial code (e.g. `1` for US, `44` for GB).
    pub dial_code: u16,
}

impl PhoneCountry {
    /// Regional-indicator flag emoji for this country (`US` → 🇺🇸).
    #[must_use]
    pub fn flag_emoji(self) -> String {
        flag_emoji(self.iso2)
    }

    /// Dial label as shown in the country list (`+1`, `+44`, …).
    #[must_use]
    pub fn dial_label(self) -> String {
        format!("+{}", self.dial_code)
    }
}

impl fmt::Display for PhoneCountry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (+{})", self.name, self.dial_code)
    }
}

/// Detailed parse snapshot mirroring svelte-tel-input `DetailedValue`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DetailedPhoneValue {
    /// Selected / detected country.
    pub country_code: Option<CountryCode>,
    /// E.164 calling code digits without `+` (e.g. `"1"`).
    pub country_calling_code: Option<String>,
    /// National significant number digits.
    pub national_number: Option<String>,
    /// Canonical E.164 string including `+`.
    pub number: Option<String>,
    /// Whether [`number`](Self::number) is a valid phone number.
    pub valid: bool,
}

/// Options mirroring svelte-tel-input `TelInputOptions`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhoneInputOptions {
    /// Insert spaces into the displayed / emitted value (international form).
    pub spaces: bool,
    /// Fill the input placeholder from an example number for the country.
    pub auto_placeholder: bool,
}

impl Default for PhoneInputOptions {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl PhoneInputOptions {
    /// Defaults matching the web `defaultOptions` (`spaces` + `autoPlaceholder`).
    pub const DEFAULT: Self = Self {
        spaces: true,
        auto_placeholder: true,
    };
}

/// Fallible phone-input operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PhoneInputError {
    /// The country code string was not two ASCII letters.
    InvalidCountryCode,
    /// The ISO code is not in the shipped country table.
    UnknownCountry,
    /// The phone string could not be parsed / normalised.
    InvalidNumber,
}

impl fmt::Display for PhoneInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCountryCode => f.write_str("invalid country code"),
            Self::UnknownCountry => f.write_str("unknown country"),
            Self::InvalidNumber => f.write_str("invalid phone number"),
        }
    }
}

impl std::error::Error for PhoneInputError {}

/// Alphabetical-by-name country ordering used by the web country selector.
#[must_use]
pub fn default_country_order(a: &PhoneCountry, b: &PhoneCountry) -> Ordering {
    a.name.cmp(b.name).then_with(|| a.iso2.cmp(&b.iso2))
}

/// Returns every supported country from the phonelib table.
#[must_use]
pub fn phone_countries() -> Vec<PhoneCountry> {
    phonelib_countries()
        .iter()
        .filter_map(|country| {
            // Skip multi-prefix overlays that reuse the same ISO with a
            // non-canonical long prefix (e.g. DO 1809) — keep the shortest
            // calling code for dial-code display parity with svelte-tel-input.
            let dial = u16::try_from(country.prefix).ok()?;
            let iso2 = CountryCode::parse(country.code).ok()?;
            Some(PhoneCountry {
                iso2,
                name: country.name,
                dial_code: dial,
            })
        })
        .collect()
}

/// Looks up a country by ISO code.
#[must_use]
pub fn phone_country(code: CountryCode) -> Option<PhoneCountry> {
    let country = country_by_code(code.as_str())?;
    let dial = u16::try_from(country.prefix).ok()?;
    Some(PhoneCountry {
        iso2: code,
        name: country.name,
        dial_code: dial,
    })
}

/// Sorts a country slice with `order`, defaulting to [`default_country_order`].
pub fn sort_countries(
    countries: &mut [PhoneCountry],
    order: Option<fn(&PhoneCountry, &PhoneCountry) -> Ordering>,
) {
    let cmp = order.unwrap_or(default_country_order);
    countries.sort_by(cmp);
}

/// Builds the regional-indicator flag emoji for an ISO code.
#[must_use]
pub fn flag_emoji(code: CountryCode) -> String {
    let bytes = code.as_bytes();
    let mut out = String::with_capacity(8);
    for byte in bytes {
        let letter = byte.to_ascii_uppercase();
        if letter.is_ascii_uppercase() {
            let scalar = 0x1F1E6 + u32::from(letter - b'A');
            if let Some(ch) = char::from_u32(scalar) {
                out.push(ch);
            }
        }
    }
    out
}

/// Formats `value` for display / storage according to `options.spaces`.
///
/// When the number is valid, returns international (with spaces) or E.164.
/// Invalid / empty input is returned unchanged (trimmed).
#[must_use]
pub fn format_phone_value(value: &str, options: PhoneInputOptions) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let Some(e164) = normalize_to_e164(trimmed, None) else {
        return trimmed.to_owned();
    };
    if options.spaces {
        format_phone_number(&e164, PhoneFormat::International).unwrap_or(e164)
    } else {
        e164
    }
}

/// Example placeholder for `country` when `auto_placeholder` is on.
///
/// Matches svelte-tel-input's example formatting (international with spaces
/// when that is how the field usually displays numbers).
#[must_use]
pub fn auto_placeholder(country: Option<CountryCode>) -> Option<String> {
    let code = country?;
    let country = phone_country(code)?;
    let candidate = format!(
        "+{}{}",
        country.dial_code,
        example_national_digits(country.dial_code)
    );
    format_phone_number(&candidate, PhoneFormat::International)
        .or_else(|| format_phone_number(&candidate, PhoneFormat::National))
        .or(Some(format!("+{}", country.dial_code)))
}

/// Parses `raw` against an optional country hint into a detailed snapshot.
#[must_use]
pub fn parse_phone_input(raw: &str, country: Option<CountryCode>) -> DetailedPhoneValue {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return DetailedPhoneValue {
            country_code: country,
            country_calling_code: country
                .and_then(phone_country)
                .map(|c| c.dial_code.to_string()),
            national_number: None,
            number: None,
            valid: false,
        };
    }

    let parsed = match country {
        Some(code) => PhoneNumber::try_parse_with_country(trimmed, code.as_str())
            .or_else(|_| PhoneNumber::try_parse(trimmed)),
        None => PhoneNumber::try_parse(trimmed),
    };

    match parsed {
        Ok(number) => detailed_from_number(&number, country),
        Err(_) => DetailedPhoneValue {
            country_code: country,
            country_calling_code: country
                .and_then(phone_country)
                .map(|c| c.dial_code.to_string()),
            national_number: None,
            number: None,
            valid: false,
        },
    }
}

/// Applies a country change: keeps national digits when possible, else clears.
#[must_use]
pub fn apply_country_change(
    current_value: &str,
    previous: Option<CountryCode>,
    next: CountryCode,
    options: PhoneInputOptions,
) -> DetailedPhoneValue {
    let detailed = parse_phone_input(current_value, previous.or(Some(next)));
    if let Some(national) = detailed.national_number.as_deref() {
        let dial = phone_country(next).map(|c| c.dial_code).unwrap_or(0);
        let candidate = format!("+{dial}{national}");
        let mut next_detailed = parse_phone_input(&candidate, Some(next));
        if let Some(number) = next_detailed.number.as_ref() {
            next_detailed.number = Some(format_phone_value(number, options));
        }
        next_detailed.country_code = Some(next);
        return next_detailed;
    }

    DetailedPhoneValue {
        country_code: Some(next),
        country_calling_code: phone_country(next).map(|c| c.dial_code.to_string()),
        national_number: None,
        number: None,
        valid: false,
    }
}

/// Updates the controlled value after the user edits the text field.
#[must_use]
pub fn apply_input_change(
    raw: &str,
    country: Option<CountryCode>,
    options: PhoneInputOptions,
) -> DetailedPhoneValue {
    let mut detailed = parse_phone_input(raw, country);
    if detailed.valid {
        if let Some(number) = detailed.number.as_ref() {
            detailed.number = Some(format_phone_value(number, options));
        }
    } else if !raw.trim().is_empty() {
        // Keep the user's raw text as the working value until it becomes valid.
        detailed.number = Some(raw.to_owned());
    }
    if detailed.country_code.is_none() {
        detailed.country_code = country;
    }
    detailed
}

/// Whether `value` is a valid phone number (E.164 or formatted).
#[must_use]
pub fn is_phone_valid(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty() && is_valid_phone_number(trimmed)
}

/// Normalises to E.164 when possible.
#[must_use]
pub fn normalize_to_e164(value: &str, country: Option<CountryCode>) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(code) = country
        && let Ok(number) = PhoneNumber::try_parse_with_country(trimmed, code.as_str())
    {
        return Some(number.e164().to_owned());
    }
    PhoneNumber::try_parse(trimmed)
        .ok()
        .map(|number| number.e164().to_owned())
}

fn detailed_from_number(number: &PhoneNumber, hint: Option<CountryCode>) -> DetailedPhoneValue {
    let e164 = number.e164().to_owned();
    let valid = is_valid_phone_number(&e164);
    let country_code = hint
        .or_else(|| phonelib::extract_country(&e164).and_then(|c| CountryCode::parse(c.code).ok()));
    let country_calling_code = country_code
        .and_then(phone_country)
        .map(|c| c.dial_code.to_string())
        .or_else(|| {
            e164.strip_prefix('+')
                .map(|rest| {
                    rest.chars()
                        .take_while(|c| c.is_ascii_digit())
                        .take(3)
                        .collect::<String>()
                })
                .filter(|s| !s.is_empty())
        });

    DetailedPhoneValue {
        country_code,
        country_calling_code,
        national_number: Some(number.national_number().to_owned()),
        number: Some(e164),
        valid,
    }
}

fn example_national_digits(dial: u16) -> &'static str {
    match dial {
        1 => "2015550123",
        44 => "2079460958",
        33 => "123456789",
        49 => "3012345678",
        81 => "901234567",
        _ => "123456789",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn country_code_parse_roundtrip() {
        let code = CountryCode::parse("us").expect("parse");
        assert_eq!(code.as_str(), "US");
        assert_eq!(code.to_string(), "US");
        assert!(CountryCode::parse("U").is_err());
        assert!(CountryCode::parse("USA").is_err());
    }

    #[test]
    fn countries_include_us_and_sort_alpha() {
        let mut list = phone_countries();
        assert!(list.iter().any(|c| c.iso2.as_str() == "US"));
        sort_countries(&mut list, None);
        assert!(
            list.windows(2)
                .all(|w| default_country_order(&w[0], &w[1]) != Ordering::Greater)
        );
    }

    #[test]
    fn flag_emoji_us() {
        let code = CountryCode::parse("US").expect("parse");
        assert_eq!(flag_emoji(code), "🇺🇸");
    }

    #[test]
    fn parse_us_number() {
        let us = CountryCode::parse("US").expect("parse");
        let detailed = parse_phone_input("+1 418 543 8090", Some(us));
        assert!(detailed.valid);
        assert_eq!(detailed.number.as_deref(), Some("+14185438090"));
        assert_eq!(detailed.country_code, Some(us));
    }

    #[test]
    fn format_with_spaces() {
        let formatted = format_phone_value("+14185438090", PhoneInputOptions::DEFAULT);
        assert!(formatted.contains(' '));
        assert!(formatted.starts_with('+'));
    }

    #[test]
    fn apply_input_keeps_invalid_raw() {
        let us = CountryCode::parse("US").expect("parse");
        let detailed = apply_input_change("12", Some(us), PhoneInputOptions::DEFAULT);
        assert!(!detailed.valid);
        assert_eq!(detailed.number.as_deref(), Some("12"));
    }

    #[test]
    fn custom_order_pins_us_first() {
        fn us_first(a: &PhoneCountry, b: &PhoneCountry) -> Ordering {
            match (a.iso2.as_str() == "US", b.iso2.as_str() == "US") {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => default_country_order(a, b),
            }
        }
        let mut list = phone_countries();
        sort_countries(&mut list, Some(us_first));
        assert_eq!(list[0].iso2.as_str(), "US");
    }
}
