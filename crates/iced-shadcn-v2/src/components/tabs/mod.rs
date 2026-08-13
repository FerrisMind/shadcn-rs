//! Builder-first tabs component ported from shadcn-svelte.
//!
//! The public surface mirrors `Tabs.Root`, `Tabs.List`, `Tabs.Trigger`, and
//! `Tabs.Content`: the application owns the selected string and receives the
//! next value through [`Tabs::on_value_change`]. The list is backed by a small
//! custom iced widget because arrow-key navigation and the active line need to
//! coordinate several trigger children at once.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Tabs, TabsContent, TabsList, TabsTrigger, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     TabChanged(String),
//! }
//!
//! fn view<'a>(theme: &'a Theme, active: &'a str) -> Element<'a, Message> {
//!     Tabs::new(theme)
//!         .value(active)
//!         .list(
//!             TabsList::new(theme)
//!                 .push(TabsTrigger::text("account", "Account", theme))
//!                 .push(TabsTrigger::text("password", "Password", theme)),
//!         )
//!         .push(TabsContent::text(
//!             "account",
//!             "Manage your account here.",
//!             theme,
//!         ))
//!         .push(TabsContent::text(
//!             "password",
//!             "Change your password here.",
//!             theme,
//!         ))
//!         .on_value_change(Message::TabChanged)
//!         .into()
//! }
//! ```

mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{
    Tabs, TabsActivationMode, TabsContent, TabsDirection, TabsHover, TabsJustify, TabsList,
    TabsListLoop, TabsListVariant, TabsOrientation, TabsSize, TabsTrigger, TabsWrap,
};

use crate::iced_compat::widget::text::IntoFragment;
use crate::iced_compat::widget::{button, container};
use crate::iced_compat::{Element, Length, Padding};

use crate::theme::Theme;

use self::types::{TabsContentValue, TabsTriggerContent};

impl<'a, Message> TabsTrigger<'a, Message> {
    /// Creates a trigger from arbitrary iced content and a string value.
    pub fn new(
        value: impl Into<String>,
        content: impl Into<Element<'a, Message>>,
        theme: &'a Theme,
    ) -> Self {
        Self {
            theme,
            value: value.into(),
            content: TabsTriggerContent::Element(content.into()),
            disabled: false,
            width: None,
            height: None,
            padding: None,
            style_override: None,
        }
    }

    /// Creates a style-pack-aware text trigger.
    pub fn text(value: impl Into<String>, label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            theme,
            value: value.into(),
            content: TabsTriggerContent::Label(label.into_fragment()),
            disabled: false,
            width: None,
            height: None,
            padding: None,
            style_override: None,
        }
    }

    /// Returns the trigger's controlled value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Disables the trigger and excludes it from keyboard navigation.
    #[must_use = "use the returned tabs trigger builder"]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets a custom trigger width.
    #[must_use = "use the returned tabs trigger builder"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets a custom trigger height.
    #[must_use = "use the returned tabs trigger builder"]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Overrides the trigger padding.
    #[must_use = "use the returned tabs trigger builder"]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(geometry::normalize_padding(padding.into()));
        self
    }

    /// Applies an iced button-style override after tabs state is resolved.
    #[must_use = "use the returned tabs trigger builder"]
    pub fn style_override(
        mut self,
        style_override: impl Fn(button::Style, button::Status) -> button::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds this trigger as a standalone, inactive iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_standalone_trigger(self)
    }
}

impl<'a, Message> From<TabsTrigger<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(trigger: TabsTrigger<'a, Message>) -> Self {
        trigger.into_element()
    }
}

impl<'a, Message> TabsContent<'a, Message> {
    /// Creates a panel from arbitrary iced content and a trigger value.
    pub fn new(
        value: impl Into<String>,
        content: impl Into<Element<'a, Message>>,
        theme: &'a Theme,
    ) -> Self {
        Self {
            theme,
            value: value.into(),
            content: TabsContentValue::Element(content.into()),
            width: Length::Fill,
            height: Length::Shrink,
            padding: Padding::ZERO,
            style_override: None,
        }
    }

    /// Creates a style-pack-aware text panel.
    pub fn text(value: impl Into<String>, label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            theme,
            value: value.into(),
            content: TabsContentValue::Label(label.into_fragment()),
            width: Length::Fill,
            height: Length::Shrink,
            padding: Padding::ZERO,
            style_override: None,
        }
    }

    /// Returns the value associated with this panel.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Sets the panel width.
    #[must_use = "use the returned tabs content builder"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the panel height.
    #[must_use = "use the returned tabs content builder"]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the panel padding, normalizing negative and non-finite sides.
    #[must_use = "use the returned tabs content builder"]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = geometry::normalize_padding(padding.into());
        self
    }

    /// Applies an iced container-style override to the panel.
    #[must_use = "use the returned tabs content builder"]
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the panel without applying root selection filtering.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_content(self)
    }
}

impl<'a, Message> From<TabsContent<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(content: TabsContent<'a, Message>) -> Self {
        content.into_element()
    }
}

impl<'a, Message> TabsList<'a, Message> {
    /// Creates an empty list using the active style pack.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            triggers: Vec::new(),
            variant: TabsListVariant::Default,
            size: TabsSize::Default,
            wrap: TabsWrap::NoWrap,
            justify: TabsJustify::Start,
            hover: TabsHover::Subtle,
            full_width: false,
            width: Length::Shrink,
            height: Length::Shrink,
            gap: None,
            list_padding: None,
            style_override: None,
        }
    }

    /// Creates a list populated from an iterator of triggers.
    pub fn with_children(
        theme: &'a Theme,
        triggers: impl IntoIterator<Item = TabsTrigger<'a, Message>>,
    ) -> Self {
        Self::new(theme).extend(triggers)
    }

    /// Appends a trigger to the list.
    #[must_use = "use the returned tabs list builder"]
    pub fn push(mut self, trigger: TabsTrigger<'a, Message>) -> Self {
        self.triggers.push(trigger);
        self
    }

    /// Appends every trigger from an iterator.
    #[must_use = "use the returned tabs list builder"]
    pub fn extend(mut self, triggers: impl IntoIterator<Item = TabsTrigger<'a, Message>>) -> Self {
        self.triggers.extend(triggers);
        self
    }

    /// Sets the list surface variant.
    #[must_use = "use the returned tabs list builder"]
    pub fn variant(mut self, variant: TabsListVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the trigger size.
    #[must_use = "use the returned tabs list builder"]
    pub fn size(mut self, size: TabsSize) -> Self {
        self.size = size;
        self
    }

    /// Sets horizontal wrapping behavior.
    #[must_use = "use the returned tabs list builder"]
    pub fn wrap(mut self, wrap: TabsWrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Sets horizontal alignment inside each list row.
    #[must_use = "use the returned tabs list builder"]
    pub fn justify(mut self, justify: TabsJustify) -> Self {
        self.justify = justify;
        self
    }

    /// Sets the inactive trigger hover treatment.
    #[must_use = "use the returned tabs list builder"]
    pub fn hover(mut self, hover: TabsHover) -> Self {
        self.hover = hover;
        self
    }

    /// Makes the horizontal list and its triggers fill the available width.
    #[must_use = "use the returned tabs list builder"]
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self.width = Length::Fill;
        self
    }

    /// Sets a custom list width.
    #[must_use = "use the returned tabs list builder"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets a custom list height.
    #[must_use = "use the returned tabs list builder"]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Overrides the gap between triggers (style-pack default otherwise).
    #[must_use = "use the returned tabs list builder"]
    pub fn gap(mut self, gap: impl Into<f32>) -> Self {
        let gap = gap.into();
        self.gap = Some(if gap.is_finite() && gap >= 0.0 {
            gap
        } else {
            0.0
        });
        self
    }

    /// Overrides list inset padding (style-pack default otherwise).
    #[must_use = "use the returned tabs list builder"]
    pub fn list_padding(mut self, padding: impl Into<f32>) -> Self {
        let padding = padding.into();
        self.list_padding = Some(if padding.is_finite() && padding >= 0.0 {
            padding
        } else {
            0.0
        });
        self
    }

    /// Applies an iced container-style override to the list surface.
    #[must_use = "use the returned tabs list builder"]
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Returns the number of triggers in the list.
    #[must_use]
    pub fn len(&self) -> usize {
        self.triggers.len()
    }

    /// Returns `true` when the list has no triggers.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.triggers.is_empty()
    }

    /// Builds the list without a selected value or callback.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_list(
            self,
            String::new(),
            TabsOrientation::Horizontal,
            TabsActivationMode::Automatic,
            TabsListLoop::Disabled,
            None,
            false,
        )
    }
}

impl<'a, Message> From<TabsList<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(list: TabsList<'a, Message>) -> Self {
        list.into_element()
    }
}

impl<'a, Message> Tabs<'a, Message> {
    /// Creates an empty controlled tabs root.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            list: TabsList::new(theme),
            contents: Vec::new(),
            value: String::new(),
            orientation: TabsOrientation::Horizontal,
            activation_mode: TabsActivationMode::Automatic,
            list_loop: TabsListLoop::Disabled,
            spacing: 8.0,
            width: Length::Fill,
            height: Length::Shrink,
            padding: Padding::ZERO,
            disabled: false,
            on_value_change: None,
            style_override: None,
        }
    }

    /// Creates a root from a trigger iterator and a content iterator.
    pub fn with_children(
        theme: &'a Theme,
        triggers: impl IntoIterator<Item = TabsTrigger<'a, Message>>,
        contents: impl IntoIterator<Item = TabsContent<'a, Message>>,
    ) -> Self {
        Self::new(theme)
            .list(TabsList::with_children(theme, triggers))
            .extend(contents)
    }

    /// Replaces the trigger list.
    #[must_use = "use the returned tabs root builder"]
    pub fn list(mut self, list: TabsList<'a, Message>) -> Self {
        self.list = list;
        self
    }

    /// Appends a trigger to the root's list.
    #[must_use = "use the returned tabs root builder"]
    pub fn push_trigger(mut self, trigger: TabsTrigger<'a, Message>) -> Self {
        self.list = self.list.push(trigger);
        self
    }

    /// Appends one content panel.
    #[must_use = "use the returned tabs root builder"]
    pub fn push(mut self, content: TabsContent<'a, Message>) -> Self {
        self.contents.push(content);
        self
    }

    /// Appends every content panel from an iterator.
    #[must_use = "use the returned tabs root builder"]
    pub fn extend(mut self, contents: impl IntoIterator<Item = TabsContent<'a, Message>>) -> Self {
        self.contents.extend(contents);
        self
    }

    /// Sets the controlled active trigger value.
    #[must_use = "use the returned tabs root builder"]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Returns the controlled value passed to the root.
    #[must_use]
    pub fn active_value(&self) -> &str {
        &self.value
    }

    /// Sets the root orientation.
    #[must_use = "use the returned tabs root builder"]
    pub fn orientation(mut self, orientation: TabsOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets keyboard activation behavior.
    #[must_use = "use the returned tabs root builder"]
    pub fn activation_mode(mut self, activation_mode: TabsActivationMode) -> Self {
        self.activation_mode = activation_mode;
        self
    }

    /// Enables or disables keyboard navigation wrapping.
    #[must_use = "use the returned tabs root builder"]
    pub fn list_loop(mut self, list_loop: TabsListLoop) -> Self {
        self.list_loop = list_loop;
        self
    }

    /// Sets the gap between the list and active panel in logical pixels.
    #[must_use = "use the returned tabs root builder"]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = geometry::normalize_px(spacing);
        self
    }

    /// Sets the root width.
    #[must_use = "use the returned tabs root builder"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the root height.
    #[must_use = "use the returned tabs root builder"]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets root padding, normalizing negative and non-finite sides.
    #[must_use = "use the returned tabs root builder"]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = geometry::normalize_padding(padding.into());
        self
    }

    /// Disables all triggers while retaining the selected visual state.
    #[must_use = "use the returned tabs root builder"]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the callback receiving the next selected trigger value.
    #[must_use = "use the returned tabs root builder"]
    pub fn on_value_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(String) -> Message + 'a,
    {
        self.on_value_change = Some(Box::new(callback));
        self
    }

    /// Sets or clears the value-change callback.
    #[must_use = "use the returned tabs root builder"]
    pub fn on_value_change_maybe<F>(mut self, callback: Option<F>) -> Self
    where
        F: Fn(String) -> Message + 'a,
    {
        self.on_value_change = callback.map(|callback| Box::new(callback) as _);
        self
    }

    /// Alias for [`Self::on_value_change`] using shadcn-svelte terminology.
    #[must_use = "use the returned tabs root builder"]
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: Fn(String) -> Message + 'a,
    {
        self.on_value_change(callback)
    }

    /// Applies an iced container-style override to the root.
    #[must_use = "use the returned tabs root builder"]
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Returns the number of content panels in the root.
    #[must_use]
    pub fn len(&self) -> usize {
        self.contents.len()
    }

    /// Returns `true` when the root has no content panels.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    /// Builds the complete tabs root as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_tabs(self)
    }
}

impl<'a, Message> From<Tabs<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(tabs: Tabs<'a, Message>) -> Self {
        tabs.into_element()
    }
}

/// Convenience constructor for a text trigger.
pub fn tabs_trigger<'a, Message>(
    value: impl Into<String>,
    label: impl IntoFragment<'a>,
    theme: &'a Theme,
) -> TabsTrigger<'a, Message> {
    TabsTrigger::text(value, label, theme)
}

/// Convenience constructor for a text content panel.
pub fn tabs_content<'a, Message>(
    value: impl Into<String>,
    label: impl IntoFragment<'a>,
    theme: &'a Theme,
) -> TabsContent<'a, Message> {
    TabsContent::text(value, label, theme)
}

/// Converts a configured root into an iced element.
pub fn tabs<'a, Message>(root: Tabs<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    root.into_element()
}
