//! A composable, controlled emoji picker for `iced-shadcn-v2`.
//!
//! The picker follows the structure and behavior of the shadcn-svelte-extra
//! component: a searchable six-column catalog, category headings, optional
//! frecency-sorted recents, controlled skin tone, an active-value footer slot,
//! and a standalone skin-tone selector. Unicode data comes from the complete
//! `emojis` catalog rather than a hand-maintained subset.
//!
//! Iced applications own their state. Feed [`EmojiPicker::value`] and
//! [`EmojiPicker::query`] from application state and update them from the
//! callbacks. For recents, call [`EmojiPickerRecents::record`] in the same
//! `on_select` handler and persist the value using the application's storage.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{EmojiPicker, SelectedEmoji, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     QueryChanged(String),
//!     EmojiSelected(SelectedEmoji),
//! }
//!
//! fn view<'a>(theme: &'a Theme, value: &'a str, query: &'a str) -> Element<'a, Message> {
//!     EmojiPicker::new(theme)
//!         .value(value)
//!         .query(query)
//!         .on_query_change(Message::QueryChanged)
//!         .on_select(Message::EmojiSelected)
//!         .into()
//! }
//! ```

mod render;
mod types;

#[cfg(test)]
mod tests;

pub use types::{
    EmojiPickerCategory, EmojiPickerData, EmojiPickerRecent, EmojiPickerRecents, EmojiPickerSkin,
    SelectedEmoji,
};

use std::fmt;
use std::sync::Arc;

use crate::iced_compat::{Element, Length};
use crate::theme::Theme;

/// Callback used by the controlled picker root.
type SelectCallback<'a, Message> = Arc<dyn Fn(SelectedEmoji) -> Message + 'a>;
type QueryCallback<'a, Message> = Box<dyn Fn(String) -> Message + 'a>;
type SkinCallback<'a, Message> = Box<dyn Fn(EmojiPickerSkin) -> Message + 'a>;
type FooterCallback<'a, Message> =
    Box<dyn FnOnce(Option<SelectedEmoji>) -> Element<'a, Message> + 'a>;

/// Builder-first controlled emoji picker.
///
/// The default footprint mirrors the reference component (`232px` wide with
/// a `200px` scrolling list). All mutable values are supplied by the caller;
/// the picker does not hide state in widget-local storage.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct EmojiPicker<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) value: String,
    pub(super) query: String,
    pub(super) skin: EmojiPickerSkin,
    pub(super) show_recents: bool,
    pub(super) recents: Option<&'a EmojiPickerRecents>,
    pub(super) max_recents: usize,
    pub(super) empty_message: String,
    pub(super) search_placeholder: String,
    pub(super) preview_emoji: String,
    pub(super) width: Length,
    pub(super) max_height: f32,
    pub(super) disabled: bool,
    pub(super) on_query_change: Option<QueryCallback<'a, Message>>,
    pub(super) on_select: Option<SelectCallback<'a, Message>>,
    pub(super) on_skin_change: Option<SkinCallback<'a, Message>>,
    pub(super) footer: Option<FooterCallback<'a, Message>>,
}

impl<Message> fmt::Debug for EmojiPicker<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EmojiPicker")
            .field("theme", &self.theme)
            .field("value", &self.value)
            .field("query", &self.query)
            .field("skin", &self.skin)
            .field("show_recents", &self.show_recents)
            .field("recents", &self.recents.is_some())
            .field("max_recents", &self.max_recents)
            .field("empty_message", &self.empty_message)
            .field("search_placeholder", &self.search_placeholder)
            .field("preview_emoji", &self.preview_emoji)
            .field("width", &self.width)
            .field("max_height", &self.max_height)
            .field("disabled", &self.disabled)
            .field("on_query_change", &self.on_query_change.is_some())
            .field("on_select", &self.on_select.is_some())
            .field("on_skin_change", &self.on_skin_change.is_some())
            .field("footer", &self.footer.is_some())
            .finish()
    }
}

impl<'a, Message> EmojiPicker<'a, Message> {
    /// Creates a picker with the default skin and the complete emoji catalog.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            value: String::new(),
            query: String::new(),
            skin: EmojiPickerSkin::Default,
            show_recents: false,
            recents: None,
            max_recents: 12,
            empty_message: "No results.".to_owned(),
            search_placeholder: "Search".to_owned(),
            preview_emoji: "👋".to_owned(),
            width: render::default_width(),
            max_height: render::default_max_height(),
            disabled: false,
            on_query_change: None,
            on_select: None,
            on_skin_change: None,
            footer: None,
        }
    }

    /// Sets the controlled selected native emoji value used by the footer.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Sets the controlled search query.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }

    /// Sets the default skin used for emoji variants and the tone selector.
    pub fn skin(mut self, skin: EmojiPickerSkin) -> Self {
        self.skin = skin;
        self
    }

    /// Enables or disables the recent-emojis group.
    pub fn show_recents(mut self, show: bool) -> Self {
        self.show_recents = show;
        self
    }

    /// Supplies caller-owned recent-emojis data.
    pub fn recents(mut self, recents: &'a EmojiPickerRecents) -> Self {
        self.recents = Some(recents);
        self
    }

    /// Sets or clears caller-owned recent-emojis data.
    pub fn recents_maybe(mut self, recents: Option<&'a EmojiPickerRecents>) -> Self {
        self.recents = recents;
        self
    }

    /// Limits the number of recent entries rendered at the top of the list.
    pub fn max_recents(mut self, max_recents: usize) -> Self {
        self.max_recents = max_recents;
        self
    }

    /// Sets the empty-search result message.
    pub fn empty_message(mut self, message: impl Into<String>) -> Self {
        self.empty_message = message.into();
        self
    }

    /// Sets the search placeholder.
    pub fn search_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.search_placeholder = placeholder.into();
        self
    }

    /// Sets the emoji preview used by the built-in tone selector when
    /// [`Self::on_skin_change`] is configured.
    pub fn preview_emoji(mut self, emoji: impl Into<String>) -> Self {
        self.preview_emoji = emoji.into();
        self
    }

    /// Sets the picker width. The default is `232px`, matching shadcn-svelte.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the scrolling list height in pixels. The default is `200px`.
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height.max(0.0);
        self
    }

    /// Disables search and emoji selection while retaining the controlled view.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the controlled search-change callback.
    pub fn on_query_change(mut self, on_query_change: impl Fn(String) -> Message + 'a) -> Self {
        self.on_query_change = Some(Box::new(on_query_change));
        self
    }

    /// Sets or clears the controlled search-change callback.
    pub fn on_query_change_maybe(
        mut self,
        on_query_change: Option<impl Fn(String) -> Message + 'a>,
    ) -> Self {
        self.on_query_change = on_query_change.map(|callback| Box::new(callback) as _);
        self
    }

    /// Sets the callback emitted after an emoji is selected.
    pub fn on_select(mut self, on_select: impl Fn(SelectedEmoji) -> Message + 'a) -> Self {
        self.on_select = Some(Arc::new(on_select));
        self
    }

    /// Sets or clears the emoji-selection callback.
    pub fn on_select_maybe(
        mut self,
        on_select: Option<impl Fn(SelectedEmoji) -> Message + 'a>,
    ) -> Self {
        self.on_select = on_select.map(|callback| Arc::new(callback) as _);
        self
    }

    /// Sets the callback emitted by the picker's built-in skin-tone selector.
    /// When configured, the selector is appended to the footer slot (or a
    /// compact footer is created when no custom footer was supplied).
    pub fn on_skin_change(
        mut self,
        on_skin_change: impl Fn(EmojiPickerSkin) -> Message + 'a,
    ) -> Self {
        self.on_skin_change = Some(Box::new(on_skin_change));
        self
    }

    /// Sets or clears the built-in skin-tone selector callback.
    pub fn on_skin_change_maybe(
        mut self,
        on_skin_change: Option<impl Fn(EmojiPickerSkin) -> Message + 'a>,
    ) -> Self {
        self.on_skin_change = on_skin_change.map(|callback| Box::new(callback) as _);
        self
    }

    /// Adds a footer render slot. It receives an owned active selection or
    /// `None` when the controlled [`Self::value`] is empty/unknown.
    pub fn footer(
        mut self,
        footer: impl FnOnce(Option<SelectedEmoji>) -> Element<'a, Message> + 'a,
    ) -> Self {
        self.footer = Some(Box::new(footer));
        self
    }

    /// Adds static footer content without needing an active-value callback.
    pub fn footer_element(mut self, footer: impl Into<Element<'a, Message>>) -> Self
    where
        Message: 'a,
    {
        let footer = footer.into();
        self.footer = Some(Box::new(move |_| footer));
        self
    }
}

impl<'a, Message: Clone + 'a> EmojiPicker<'a, Message> {
    /// Builds the picker as an iced element.
    pub fn into_element(self) -> Element<'a, Message> {
        render::build_picker(self)
    }
}

impl<'a, Message: Clone + 'a> From<EmojiPicker<'a, Message>> for Element<'a, Message> {
    fn from(picker: EmojiPicker<'a, Message>) -> Self {
        picker.into_element()
    }
}

/// Standalone controlled search field from the composable reference API.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct EmojiPickerSearch<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) value: String,
    pub(super) placeholder: String,
    pub(super) width: Length,
    pub(super) disabled: bool,
    pub(super) on_input: Option<QueryCallback<'a, Message>>,
}

impl<Message> fmt::Debug for EmojiPickerSearch<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EmojiPickerSearch")
            .field("theme", &self.theme)
            .field("value", &self.value)
            .field("placeholder", &self.placeholder)
            .field("width", &self.width)
            .field("disabled", &self.disabled)
            .field("on_input", &self.on_input.is_some())
            .finish()
    }
}

impl<'a, Message> EmojiPickerSearch<'a, Message> {
    /// Creates a search field with a controlled value.
    pub fn new(value: impl Into<String>, theme: &'a Theme) -> Self {
        Self {
            theme,
            value: value.into(),
            placeholder: "Search".to_owned(),
            width: Length::Fill,
            disabled: false,
            on_input: None,
        }
    }

    /// Returns the controlled query.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Sets the controlled query.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.value = query.into();
        self
    }

    /// Sets the search placeholder.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Sets the search width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Disables the search field.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the controlled input callback.
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// Sets or clears the controlled input callback.
    pub fn on_input_maybe(mut self, on_input: Option<impl Fn(String) -> Message + 'a>) -> Self {
        self.on_input = on_input.map(|callback| Box::new(callback) as _);
        self
    }
}

impl<'a, Message: Clone + 'a> EmojiPickerSearch<'a, Message> {
    /// Builds the search field.
    pub fn into_element(self) -> Element<'a, Message> {
        render::build_search(self)
    }
}

impl<'a, Message: Clone + 'a> From<EmojiPickerSearch<'a, Message>> for Element<'a, Message> {
    fn from(search: EmojiPickerSearch<'a, Message>) -> Self {
        search.into_element()
    }
}

/// Standalone list viewport from the composable reference API.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct EmojiPickerList<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) query: String,
    pub(super) skin: EmojiPickerSkin,
    pub(super) show_recents: bool,
    pub(super) recents: Option<&'a EmojiPickerRecents>,
    pub(super) max_recents: usize,
    pub(super) empty_message: String,
    pub(super) max_height: f32,
    pub(super) disabled: bool,
    pub(super) on_select: Option<SelectCallback<'a, Message>>,
}

impl<Message> fmt::Debug for EmojiPickerList<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EmojiPickerList")
            .field("theme", &self.theme)
            .field("query", &self.query)
            .field("skin", &self.skin)
            .field("show_recents", &self.show_recents)
            .field("recents", &self.recents.is_some())
            .field("max_recents", &self.max_recents)
            .field("empty_message", &self.empty_message)
            .field("max_height", &self.max_height)
            .field("disabled", &self.disabled)
            .field("on_select", &self.on_select.is_some())
            .finish()
    }
}

impl<'a, Message> EmojiPickerList<'a, Message> {
    /// Creates a list with the reference `200px` viewport height.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            query: String::new(),
            skin: EmojiPickerSkin::Default,
            show_recents: false,
            recents: None,
            max_recents: 12,
            empty_message: "No results.".to_owned(),
            max_height: render::default_max_height(),
            disabled: false,
            on_select: None,
        }
    }

    /// Sets the controlled query.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }

    /// Sets the selected skin tone.
    pub fn skin(mut self, skin: EmojiPickerSkin) -> Self {
        self.skin = skin;
        self
    }

    /// Enables the recent group.
    pub fn show_recents(mut self, show: bool) -> Self {
        self.show_recents = show;
        self
    }

    /// Supplies app-controlled recent entries.
    pub fn recents(mut self, recents: &'a EmojiPickerRecents) -> Self {
        self.recents = Some(recents);
        self
    }

    /// Sets or clears app-controlled recent entries.
    pub fn recents_maybe(mut self, recents: Option<&'a EmojiPickerRecents>) -> Self {
        self.recents = recents;
        self
    }

    /// Sets the recent-entry limit.
    pub fn max_recents(mut self, max_recents: usize) -> Self {
        self.max_recents = max_recents;
        self
    }

    /// Sets the empty result message.
    pub fn empty_message(mut self, message: impl Into<String>) -> Self {
        self.empty_message = message.into();
        self
    }

    /// Sets the list viewport height.
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height.max(0.0);
        self
    }

    /// Disables emoji selection.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the selection callback.
    pub fn on_select(mut self, on_select: impl Fn(SelectedEmoji) -> Message + 'a) -> Self {
        self.on_select = Some(Arc::new(on_select));
        self
    }

    /// Sets or clears the selection callback.
    pub fn on_select_maybe(
        mut self,
        on_select: Option<impl Fn(SelectedEmoji) -> Message + 'a>,
    ) -> Self {
        self.on_select = on_select.map(|callback| Arc::new(callback) as _);
        self
    }

    /// Builds the scrollable emoji list.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_list(self)
    }
}

impl<'a, Message: Clone + 'a> From<EmojiPickerList<'a, Message>> for Element<'a, Message> {
    fn from(list: EmojiPickerList<'a, Message>) -> Self {
        list.into_element()
    }
}

/// Border and background wrapper corresponding to `EmojiPicker.Viewport`.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct EmojiPickerViewport<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) content: Element<'a, Message>,
    pub(super) width: Length,
    pub(super) radius: Option<f32>,
}

impl<Message> fmt::Debug for EmojiPickerViewport<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EmojiPickerViewport")
            .field("theme", &self.theme)
            .field("width", &self.width)
            .field("radius", &self.radius)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> EmojiPickerViewport<'a, Message> {
    /// Wraps arbitrary picker content in the reference border.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            theme,
            content: content.into(),
            width: render::default_width(),
            radius: None,
        }
    }

    /// Sets the viewport width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Overrides the active style-pack surface radius in pixels.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius.max(0.0));
        self
    }

    /// Builds the viewport.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_viewport(self)
    }
}

impl<'a, Message: 'a> From<EmojiPickerViewport<'a, Message>> for Element<'a, Message> {
    fn from(viewport: EmojiPickerViewport<'a, Message>) -> Self {
        viewport.into_element()
    }
}

/// Footer wrapper corresponding to `EmojiPicker.Footer`.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct EmojiPickerFooter<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) content: Option<Element<'a, Message>>,
    pub(super) padding: f32,
}

impl<Message> fmt::Debug for EmojiPickerFooter<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EmojiPickerFooter")
            .field("theme", &self.theme)
            .field("content", &self.content.is_some())
            .field("padding", &self.padding)
            .finish()
    }
}

impl<'a, Message> EmojiPickerFooter<'a, Message> {
    /// Creates a footer. The active selection is supplied by the caller when
    /// composing the footer, matching the Svelte snippet contract.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            content: None,
            padding: 8.0,
        }
    }

    /// Adds arbitrary footer content.
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Sets the footer padding.
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding.max(0.0);
        self
    }

    /// Builds the footer.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_footer(self)
    }
}

impl<'a, Message: 'a> From<EmojiPickerFooter<'a, Message>> for Element<'a, Message> {
    fn from(footer: EmojiPickerFooter<'a, Message>) -> Self {
        footer.into_element()
    }
}

/// Standalone button corresponding to `EmojiPicker.SkinToneSelector`.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct EmojiPickerSkinToneSelector<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) skin: EmojiPickerSkin,
    pub(super) preview_emoji: String,
    pub(super) disabled: bool,
    pub(super) on_skin_change: Option<SkinCallback<'a, Message>>,
}

impl<Message> fmt::Debug for EmojiPickerSkinToneSelector<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EmojiPickerSkinToneSelector")
            .field("theme", &self.theme)
            .field("skin", &self.skin)
            .field("preview_emoji", &self.preview_emoji)
            .field("disabled", &self.disabled)
            .field("on_skin_change", &self.on_skin_change.is_some())
            .finish()
    }
}

impl<'a, Message> EmojiPickerSkinToneSelector<'a, Message> {
    /// Creates a tone selector with the reference `👋` preview.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            skin: EmojiPickerSkin::Default,
            preview_emoji: "👋".to_owned(),
            disabled: false,
            on_skin_change: None,
        }
    }

    /// Sets the controlled tone.
    pub fn skin(mut self, skin: EmojiPickerSkin) -> Self {
        self.skin = skin;
        self
    }

    /// Sets the native emoji used as the tone preview.
    pub fn preview_emoji(mut self, emoji: impl Into<String>) -> Self {
        self.preview_emoji = emoji.into();
        self
    }

    /// Disables the selector.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the callback emitted with the next tone.
    pub fn on_skin_change(
        mut self,
        on_skin_change: impl Fn(EmojiPickerSkin) -> Message + 'a,
    ) -> Self {
        self.on_skin_change = Some(Box::new(on_skin_change));
        self
    }

    /// Sets or clears the controlled skin-change callback.
    pub fn on_skin_change_maybe(
        mut self,
        on_skin_change: Option<impl Fn(EmojiPickerSkin) -> Message + 'a>,
    ) -> Self {
        self.on_skin_change = on_skin_change.map(|callback| Box::new(callback) as _);
        self
    }

    /// Builds the selector button.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_skin_selector(self)
    }
}

impl<'a, Message: Clone + 'a> From<EmojiPickerSkinToneSelector<'a, Message>>
    for Element<'a, Message>
{
    fn from(selector: EmojiPickerSkinToneSelector<'a, Message>) -> Self {
        selector.into_element()
    }
}
