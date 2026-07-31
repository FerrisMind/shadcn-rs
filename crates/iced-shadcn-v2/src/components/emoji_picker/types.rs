//! Public value types used by the emoji picker.

/// The six Fitzpatrick skin tones supported by the picker.
///
/// The value is controlled by the application, just like the Svelte
/// component's `skin` prop. `Default` is the unmodified emoji skin.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EmojiPickerSkin {
    /// The emoji's default presentation.
    #[default]
    Default,
    /// Light skin tone.
    Light,
    /// Medium-light skin tone.
    MediumLight,
    /// Medium skin tone.
    Medium,
    /// Medium-dark skin tone.
    MediumDark,
    /// Dark skin tone.
    Dark,
}

impl EmojiPickerSkin {
    /// All six picker tones in their cycling order.
    pub const ALL: [Self; 6] = [
        Self::Default,
        Self::Light,
        Self::MediumLight,
        Self::Medium,
        Self::MediumDark,
        Self::Dark,
    ];

    /// Returns the zero-based index used by the Svelte component.
    pub const fn index(self) -> usize {
        match self {
            Self::Default => 0,
            Self::Light => 1,
            Self::MediumLight => 2,
            Self::Medium => 3,
            Self::MediumDark => 4,
            Self::Dark => 5,
        }
    }

    /// Converts the Svelte-compatible zero-based index into a skin tone.
    pub const fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Default),
            1 => Some(Self::Light),
            2 => Some(Self::MediumLight),
            3 => Some(Self::Medium),
            4 => Some(Self::MediumDark),
            5 => Some(Self::Dark),
            _ => None,
        }
    }

    /// Returns the next tone, wrapping back to [`Self::Default`].
    pub const fn next(self) -> Self {
        Self::ALL[(self.index() + 1) % Self::ALL.len()]
    }

    /// Returns a stable label suitable for demos and accessibility text.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::Light => "Light",
            Self::MediumLight => "Medium light",
            Self::Medium => "Medium",
            Self::MediumDark => "Medium dark",
            Self::Dark => "Dark",
        }
    }

    pub(crate) const fn as_emojis_tone(self) -> emojis::SkinTone {
        match self {
            Self::Default => emojis::SkinTone::Default,
            Self::Light => emojis::SkinTone::Light,
            Self::MediumLight => emojis::SkinTone::MediumLight,
            Self::Medium => emojis::SkinTone::Medium,
            Self::MediumDark => emojis::SkinTone::MediumDark,
            Self::Dark => emojis::SkinTone::Dark,
        }
    }

    pub(crate) fn from_emojis_tone(tone: emojis::SkinTone) -> Option<Self> {
        match tone {
            emojis::SkinTone::Default => Some(Self::Default),
            emojis::SkinTone::Light => Some(Self::Light),
            emojis::SkinTone::MediumLight => Some(Self::MediumLight),
            emojis::SkinTone::Medium => Some(Self::Medium),
            emojis::SkinTone::MediumDark => Some(Self::MediumDark),
            emojis::SkinTone::Dark => Some(Self::Dark),
            _ => None,
        }
    }
}

/// The visual category headings used by the picker.
///
/// `SmileysAndEmotion` and `PeopleAndBody` are intentionally combined into
/// `People`, matching the eight categories in `@emoji-mart/data`.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EmojiPickerCategory {
    /// Faces, people, and body gestures.
    People,
    /// Animals, plants, and weather.
    Nature,
    /// Food and drinks.
    Foods,
    /// Sports, games, and celebrations.
    Activity,
    /// Travel and places.
    Places,
    /// Tools, clothing, and other objects.
    Objects,
    /// Punctuation, signs, and symbols.
    Symbols,
    /// Country and regional flags.
    Flags,
}

impl EmojiPickerCategory {
    /// All categories in the same order as the reference picker.
    pub const ALL: [Self; 8] = [
        Self::People,
        Self::Nature,
        Self::Foods,
        Self::Activity,
        Self::Places,
        Self::Objects,
        Self::Symbols,
        Self::Flags,
    ];

    /// Returns the category heading shown in the list.
    pub const fn title(self) -> &'static str {
        match self {
            Self::People => "People",
            Self::Nature => "Nature",
            Self::Foods => "Foods",
            Self::Activity => "Activity",
            Self::Places => "Places",
            Self::Objects => "Objects",
            Self::Symbols => "Symbols",
            Self::Flags => "Flags",
        }
    }

    pub(crate) const fn matches_group(self, group: emojis::Group) -> bool {
        match self {
            Self::People => matches!(
                group,
                emojis::Group::SmileysAndEmotion | emojis::Group::PeopleAndBody
            ),
            Self::Nature => matches!(group, emojis::Group::AnimalsAndNature),
            Self::Foods => matches!(group, emojis::Group::FoodAndDrink),
            Self::Activity => matches!(group, emojis::Group::Activities),
            Self::Places => matches!(group, emojis::Group::TravelAndPlaces),
            Self::Objects => matches!(group, emojis::Group::Objects),
            Self::Symbols => matches!(group, emojis::Group::Symbols),
            Self::Flags => matches!(group, emojis::Group::Flags),
        }
    }

    pub(crate) fn from_group(group: emojis::Group) -> Self {
        match group {
            emojis::Group::SmileysAndEmotion | emojis::Group::PeopleAndBody => Self::People,
            emojis::Group::AnimalsAndNature => Self::Nature,
            emojis::Group::FoodAndDrink => Self::Foods,
            emojis::Group::Activities => Self::Activity,
            emojis::Group::TravelAndPlaces => Self::Places,
            emojis::Group::Objects => Self::Objects,
            emojis::Group::Symbols => Self::Symbols,
            emojis::Group::Flags => Self::Flags,
        }
    }
}

/// Metadata for the selected emoji.
///
/// This is an owned wrapper around the catalog entry. The public API does not
/// expose the `emojis` crate's internal catalog type, leaving room to change
/// catalogs without breaking callers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmojiPickerData {
    id: String,
    native: String,
    name: String,
    category: EmojiPickerCategory,
    shortcodes: Vec<String>,
    skin_count: usize,
    has_skin_tones: bool,
}

impl EmojiPickerData {
    /// Returns the stable catalog identifier (the first shortcode when one
    /// exists, otherwise the native Unicode value).
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the catalog's native Unicode representation.
    pub fn native(&self) -> &str {
        &self.native
    }

    /// Alias for [`Self::native`] matching the Svelte `SelectedEmoji` shape.
    pub fn emoji(&self) -> &str {
        self.native()
    }

    /// Returns the Unicode CLDR name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the visual category.
    pub const fn category(&self) -> EmojiPickerCategory {
        self.category
    }

    /// Returns all GitHub-compatible shortcodes known for this emoji.
    pub fn shortcodes(&self) -> &[String] {
        &self.shortcodes
    }

    /// Returns the searchable keyword set exposed by the catalog wrapper.
    pub fn keywords(&self) -> impl Iterator<Item = &str> + '_ {
        std::iter::once(self.name.as_str()).chain(self.shortcodes.iter().map(String::as_str))
    }

    /// Returns the number of native skin variants in the catalog.
    pub const fn skin_count(&self) -> usize {
        self.skin_count
    }

    /// Returns whether the catalog has skin-tone variants for this emoji.
    pub const fn has_skin_tones(&self) -> bool {
        self.has_skin_tones
    }

    pub(crate) fn from_catalog(emoji: &emojis::Emoji) -> Self {
        let shortcodes: Vec<String> = emoji.shortcodes().map(str::to_owned).collect();
        Self {
            id: shortcodes
                .first()
                .cloned()
                .unwrap_or_else(|| emoji.as_str().to_owned()),
            native: emoji.as_str().to_owned(),
            name: emoji.name().to_owned(),
            category: EmojiPickerCategory::from_group(emoji.group()),
            skin_count: emoji.skin_tones().map(|tones| tones.count()).unwrap_or(1),
            shortcodes,
            has_skin_tones: emoji.skin_tones().is_some(),
        }
    }
}

/// The value emitted when a user selects an emoji.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectedEmoji {
    emoji: String,
    data: EmojiPickerData,
    skin: EmojiPickerSkin,
}

impl SelectedEmoji {
    /// Returns the selected native Unicode string.
    pub fn emoji(&self) -> &str {
        &self.emoji
    }

    /// Returns the selected emoji metadata.
    pub const fn data(&self) -> &EmojiPickerData {
        &self.data
    }

    /// Returns the active skin tone used for the selection.
    pub const fn skin(&self) -> EmojiPickerSkin {
        self.skin
    }

    /// Consumes the selection and returns its native Unicode string.
    pub fn into_emoji(self) -> String {
        self.emoji
    }

    pub(crate) fn from_catalog(base: &emojis::Emoji, skin: EmojiPickerSkin) -> Self {
        let displayed = base.with_skin_tone(skin.as_emojis_tone()).unwrap_or(base);

        Self {
            emoji: displayed.as_str().to_owned(),
            data: EmojiPickerData::from_catalog(base),
            skin,
        }
    }

    pub(crate) fn from_native(native: &str, fallback_skin: EmojiPickerSkin) -> Option<Self> {
        let selected = emojis::get(native)?;
        let (base, skin) = match selected.skin_tone() {
            Some(tone) => (
                selected
                    .with_skin_tone(emojis::SkinTone::Default)
                    .unwrap_or(selected),
                EmojiPickerSkin::from_emojis_tone(tone).unwrap_or(fallback_skin),
            ),
            None => (selected, EmojiPickerSkin::Default),
        };

        Some(Self::from_catalog(base, skin))
    }
}

/// One app-controlled recent emoji entry.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmojiPickerRecent {
    emoji: String,
    uses: u32,
    last_used: u64,
}

impl EmojiPickerRecent {
    /// Returns the native emoji string.
    pub fn emoji(&self) -> &str {
        &self.emoji
    }

    /// Returns the number of times the entry has been recorded.
    pub const fn uses(&self) -> u32 {
        self.uses
    }
}

/// Application-owned frecency data for [`super::EmojiPicker`].
///
/// The web component persists this information under `recentsKey` in browser
/// storage. A native iced component cannot safely own browser storage, so the
/// caller owns this small value and can serialize it wherever appropriate.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmojiPickerRecents {
    entries: Vec<EmojiPickerRecent>,
    next_tick: u64,
}

impl EmojiPickerRecents {
    /// Creates an empty recent-emoji store.
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_tick: 0,
        }
    }

    /// Returns recent entries ordered by use count and then recency.
    pub fn entries(&self) -> &[EmojiPickerRecent] {
        &self.entries
    }

    /// Returns the number of stored recent entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether no recent entries are stored.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Removes all recent entries.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.next_tick = 0;
    }

    /// Records a selection using its native Unicode value.
    pub fn record(&mut self, selected: &SelectedEmoji) {
        let _ = self.record_emoji(selected.emoji());
    }

    /// Records a native Unicode emoji and returns `false` for invalid input.
    ///
    /// Invalid values are ignored rather than being rendered as broken recent
    /// buttons. This keeps persisted app state safe to load and migrate.
    pub fn record_emoji(&mut self, emoji: impl Into<String>) -> bool {
        let emoji = emoji.into();
        if emojis::get(&emoji).is_none() {
            return false;
        }

        let latest_tick = self
            .entries
            .iter()
            .map(|entry| entry.last_used)
            .max()
            .unwrap_or_default();
        self.next_tick = self.next_tick.max(latest_tick).wrapping_add(1);

        if let Some(entry) = self.entries.iter_mut().find(|entry| entry.emoji == emoji) {
            entry.uses = entry.uses.saturating_add(1);
            entry.last_used = self.next_tick;
        } else {
            self.entries.push(EmojiPickerRecent {
                emoji,
                uses: 1,
                last_used: self.next_tick,
            });
        }

        self.entries.sort_by(|left, right| {
            right
                .uses
                .cmp(&left.uses)
                .then_with(|| right.last_used.cmp(&left.last_used))
                .then_with(|| left.emoji.cmp(&right.emoji))
        });
        true
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &EmojiPickerRecent> {
        self.entries.iter()
    }
}

pub(crate) fn catalog_entry(native: &str) -> Option<&'static emojis::Emoji> {
    emojis::get(native)
}

pub(crate) fn base_catalog_entry(emoji: &'static emojis::Emoji) -> &'static emojis::Emoji {
    emoji
        .with_skin_tone(emojis::SkinTone::Default)
        .unwrap_or(emoji)
}

pub(crate) fn display_catalog_entry(
    emoji: &'static emojis::Emoji,
    skin: EmojiPickerSkin,
) -> &'static emojis::Emoji {
    base_catalog_entry(emoji)
        .with_skin_tone(skin.as_emojis_tone())
        .unwrap_or_else(|| base_catalog_entry(emoji))
}

pub(crate) fn category_emojis(category: EmojiPickerCategory) -> Vec<&'static emojis::Emoji> {
    emojis::iter()
        .filter(|emoji| category.matches_group(emoji.group()))
        .collect()
}

pub(crate) fn matches_query(emoji: &emojis::Emoji, query: &str) -> bool {
    let query = query.trim();
    if query.is_empty() {
        return true;
    }

    keyword_matches(emoji.name(), query)
        || emoji
            .shortcodes()
            .any(|shortcode| keyword_matches(shortcode, query))
}

fn keyword_matches(keyword: &str, query: &str) -> bool {
    starts_with_ascii_case(keyword, query)
        || keyword
            .split([' ', '_', '-'])
            .any(|word| starts_with_ascii_case(word, query))
}

fn starts_with_ascii_case(candidate: &str, query: &str) -> bool {
    candidate.len() >= query.len()
        && candidate
            .bytes()
            .zip(query.bytes())
            .all(|(candidate, query)| candidate.eq_ignore_ascii_case(&query))
}
