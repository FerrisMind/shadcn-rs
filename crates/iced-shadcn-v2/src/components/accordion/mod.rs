//! Builder-first accordion component ported from shadcn-svelte.
//!
//! The component mirrors the source `Root`, `Item`, `Trigger`, and `Content`
//! composition. The root is controlled: the application supplies an
//! [`AccordionValue`] and receives the next value through
//! [`Accordion::on_value_change`]. Both single and multiple selection modes,
//! disabled items, orientation, loop policy, custom content, force mounting,
//! and the source trigger/content styling contract are covered.
//!
//! Reveal measurement and frame animation are delegated to the existing
//! [`crate::Collapsible`] primitive. The accordion trigger is rendered locally
//! so its down/up indicator and `hover:underline` treatment match the
//! shadcn-svelte component instead of inheriting the collapsible file-tree
//! chevron.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{
//!     Accordion, AccordionContent, AccordionItem, AccordionTrigger, AccordionValue, Theme,
//! };
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     OpenChanged(AccordionValue),
//! }
//!
//! fn view(theme: &Theme, value: AccordionValue) -> Element<'_, Message> {
//!     Accordion::new(theme)
//!         .value(value)
//!         .push(AccordionItem::text(
//!             "item-1",
//!             "Is it accessible?",
//!             "Yes. It follows the WAI-ARIA accordion pattern.",
//!             theme,
//!         ))
//!         .on_value_change(Message::OpenChanged)
//!         .into()
//! }
//! ```

mod error;
mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use error::AccordionBuildError;
pub use types::{
    Accordion, AccordionContent, AccordionHeaderLevel, AccordionItem, AccordionLoop, AccordionMode,
    AccordionOrientation, AccordionSelection, AccordionTrigger, AccordionType, AccordionValue,
};

use std::fmt;
use std::rc::Rc;
use std::time::Duration;

use crate::components::collapsible::Collapsible;
use crate::iced_compat::widget::button as button_widget;
use crate::iced_compat::widget::container;
use crate::iced_compat::widget::text::IntoFragment;
use crate::iced_compat::{Element, Length};
use crate::theme::Theme;

use shadcn_common::AccentColor;
use twill_core::prelude::Padding;
use twill_core::prelude::theme::SemanticColor;

use self::types::AccordionTriggerContent;

/// Duration of one content reveal transition.
const DEFAULT_TRANSITION: Duration = Duration::from_millis(200);

impl<Message> fmt::Debug for AccordionTrigger<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AccordionTrigger")
            .field("theme", &self.theme)
            .field("content", &self.content.kind())
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("color", &self.color)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("full_width", &self.full_width)
            .field("disabled", &self.disabled)
            .field("level", &self.level)
            .field("padding", &self.padding.is_some())
            .field("gap", &self.gap)
            .field("on_press", &self.on_press.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<Message> fmt::Debug for AccordionContent<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AccordionContent")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .field("padding", &self.padding)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("background", &self.background)
            .field("bordered", &self.bordered)
            .field("radius", &self.radius)
            .field("force_mount", &self.force_mount)
            .field("hidden_until_found", &self.hidden_until_found)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<Message> fmt::Debug for AccordionItem<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AccordionItem")
            .field("theme", &self.theme)
            .field("value", &self.value)
            .field("trigger", &self.trigger)
            .field("content", &self.content)
            .field("disabled", &self.disabled)
            .field("padding", &self.padding)
            .field("background", &self.background)
            .field("bordered", &self.bordered)
            .field("radius", &self.radius)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<Message> fmt::Debug for Accordion<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Accordion")
            .field("theme", &self.theme)
            .field("items", &self.items.len())
            .field("accordion_type", &self.accordion_type)
            .field("value", &self.value)
            .field("orientation", &self.orientation)
            .field("loop_navigation", &self.loop_navigation)
            .field("spacing", &self.spacing)
            .field("disabled", &self.disabled)
            .field("animated", &self.animated)
            .field("duration", &self.duration)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("background", &self.background)
            .field("bordered", &self.bordered)
            .field("radius", &self.radius)
            .field("on_value_change", &self.on_value_change.is_some())
            .field("on_press", &self.on_press.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message: 'a> AccordionTrigger<'a, Message> {
    /// Creates a trigger from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            theme,
            content: AccordionTriggerContent::Element(content.into()),
            variant: crate::ButtonVariant::Ghost,
            size: crate::ButtonSize::Default,
            radius: None,
            color: None,
            width: Length::Fill,
            height: Length::Shrink,
            full_width: true,
            disabled: false,
            level: AccordionHeaderLevel::default(),
            padding: None,
            gap: None,
            on_press: None,
            style_override: None,
        }
    }

    /// Creates a trigger with style-pack-aware text.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            theme,
            content: AccordionTriggerContent::Label(label.into_fragment()),
            ..Self::new(crate::iced_compat::widget::space(), theme)
        }
    }

    /// Creates an icon-leading trigger with the standard accordion indicator.
    pub fn icon(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: AccordionTriggerContent::Icon(content.into()),
            ..Self::new(crate::iced_compat::widget::space(), theme)
        }
    }

    /// Sets the button treatment used by the trigger.
    pub fn variant(mut self, variant: crate::ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the trigger size.
    pub fn size(mut self, size: crate::ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the trigger corner radius.
    pub fn radius(mut self, radius: crate::ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Applies an accent color overlay to the trigger's theme tokens.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets a custom trigger width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets a custom trigger height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Makes the trigger fill its item's available width.
    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    /// Disables this trigger independently of the item and root state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the semantic heading level of the trigger header.
    pub fn level(mut self, level: AccordionHeaderLevel) -> Self {
        self.level = level;
        self
    }

    /// Sets the semantic heading level from a number, clamping it to `1..=6`.
    pub fn level_number(self, level: u8) -> Self {
        self.level(AccordionHeaderLevel::from_number(level))
    }

    /// Returns the semantic heading level configured for this trigger.
    ///
    /// Iced does not expose an ARIA tree, so native rendering cannot attach an
    /// `aria-level` attribute. The value remains available to an application
    /// accessibility adapter or a custom renderer.
    #[must_use]
    pub const fn header_level(&self) -> AccordionHeaderLevel {
        self.level
    }

    /// Sets trigger padding using the same twill values as [`crate::Button`].
    ///
    /// # Errors
    ///
    /// Returns [`AccordionBuildError`] for `auto` or unresolved custom
    /// property values.
    pub fn padding(mut self, padding: Padding) -> Result<Self, AccordionBuildError> {
        geometry::resolve_padding(padding)?;
        self.padding = Some(padding);
        Ok(self)
    }

    /// Sets the gap between trigger content and its indicator in pixels.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = Some(geometry::normalize_px(gap));
        self
    }

    /// Sets a message emitted when this trigger is pressed.
    ///
    /// This overrides the root callback for this item, matching the normal
    /// event-handler override behavior of the source primitive.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets or clears the per-trigger press message.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Applies an iced button-style override after semantic trigger styling.
    pub fn style_override(
        mut self,
        style_override: impl Fn(button_widget::Style, button_widget::Status) -> button_widget::Style
        + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds this trigger as an inert standalone iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_trigger(self, false, false, None)
    }
}

impl<'a, Message> From<AccordionTrigger<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(trigger: AccordionTrigger<'a, Message>) -> Self {
        trigger.into_element()
    }
}

impl<'a, Message: 'a> AccordionContent<'a, Message> {
    /// Creates an empty content panel.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            spacing: 0.0,
            padding: None,
            width: Length::Fill,
            height: Length::Shrink,
            background: None,
            bordered: false,
            radius: None,
            force_mount: false,
            hidden_until_found: false,
            style_override: None,
        }
    }

    /// Creates a content panel containing one paragraph-sized text node.
    pub fn text(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        let content = render::paragraph_text(content.into_fragment(), theme);
        Self::new(theme).push(content)
    }

    /// Creates a content panel from an iterator of iced elements.
    pub fn with_children(
        theme: &'a Theme,
        children: impl IntoIterator<Item = Element<'a, Message>>,
    ) -> Self {
        Self {
            children: children.into_iter().collect(),
            ..Self::new(theme)
        }
    }

    /// Appends one child to the content panel.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends all children from an iterator.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets the gap between content children in pixels.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = geometry::normalize_px(spacing);
        self
    }

    /// Sets content padding using twill spacing values.
    ///
    /// # Errors
    ///
    /// Returns [`AccordionBuildError`] for `auto` or unresolved custom
    /// property values.
    pub fn padding(mut self, padding: Padding) -> Result<Self, AccordionBuildError> {
        self.padding = Some(geometry::resolve_padding(padding)?);
        Ok(self)
    }

    /// Sets the content width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the natural content height revealed when fully open.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Paints content on a semantic surface.
    pub fn background(mut self, background: SemanticColor) -> Self {
        self.background = Some(background);
        self
    }

    /// Draws a one-pixel border around the content surface.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Sets the content corner radius in pixels.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(geometry::normalize_px(radius));
        self
    }

    /// Keeps closed content mounted in the widget tree.
    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    /// Returns whether closed content is kept mounted.
    #[must_use]
    pub const fn is_force_mounted(&self) -> bool {
        self.force_mount
    }

    /// Retains the browser `hiddenUntilFound` intent for API parity.
    ///
    /// iced has no browser find-in-page engine, so this value is stored for
    /// configuration/debugging but does not alter native rendering.
    pub fn hidden_until_found(mut self, hidden_until_found: bool) -> Self {
        self.hidden_until_found = hidden_until_found;
        self
    }

    /// Returns the browser find-in-page preference carried by this panel.
    #[must_use]
    pub const fn hidden_until_found_enabled(&self) -> bool {
        self.hidden_until_found
    }

    /// Applies an iced container-style override after semantic content styling.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds this content panel as an inert standalone iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let theme = self.theme;
        Collapsible::new(theme)
            .content(render::build_content(self))
            .into()
    }
}

impl<'a, Message> From<AccordionContent<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(content: AccordionContent<'a, Message>) -> Self {
        content.into_element()
    }
}

impl<'a, Message: 'a> AccordionItem<'a, Message> {
    /// Creates an item without an explicit value.
    ///
    /// The root assigns a stable `item-N` value based on insertion order when
    /// the item is rendered, matching bits-ui's generated-value fallback.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            value: None,
            trigger: None,
            content: None,
            disabled: false,
            padding: None,
            background: None,
            bordered: false,
            radius: None,
            style_override: None,
        }
    }

    /// Creates a text item with a text content panel.
    pub fn text(
        value: impl Into<String>,
        trigger: impl IntoFragment<'a>,
        content: impl IntoFragment<'a>,
        theme: &'a Theme,
    ) -> Self {
        Self::new(theme)
            .value(value)
            .trigger(AccordionTrigger::text(trigger, theme))
            .content(AccordionContent::text(content, theme))
    }

    /// Sets the stable item value used in controlled selections.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Returns the explicit item value, if one was configured.
    #[must_use]
    pub fn item_value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Sets the item's trigger.
    pub fn trigger(mut self, trigger: AccordionTrigger<'a, Message>) -> Self {
        self.trigger = Some(trigger);
        self
    }

    /// Sets or clears the item's trigger.
    pub fn trigger_maybe(mut self, trigger: Option<AccordionTrigger<'a, Message>>) -> Self {
        self.trigger = trigger;
        self
    }

    /// Sets the item's content panel.
    pub fn content(mut self, content: AccordionContent<'a, Message>) -> Self {
        self.content = Some(content);
        self
    }

    /// Sets or clears the item's content panel.
    pub fn content_maybe(mut self, content: Option<AccordionContent<'a, Message>>) -> Self {
        self.content = content;
        self
    }

    /// Disables this item without affecting its controlled open state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets item padding using twill spacing values.
    ///
    /// # Errors
    ///
    /// Returns [`AccordionBuildError`] for `auto` or unresolved custom
    /// property values.
    pub fn padding(mut self, padding: Padding) -> Result<Self, AccordionBuildError> {
        self.padding = Some(geometry::resolve_padding(padding)?);
        Ok(self)
    }

    /// Paints the item on a semantic surface.
    pub fn background(mut self, background: SemanticColor) -> Self {
        self.background = Some(background);
        self
    }

    /// Draws a one-pixel border around the item.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Sets the item corner radius in pixels.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(geometry::normalize_px(radius));
        self
    }

    /// Applies an iced container-style override to the item surface.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds this item in a closed, inert accordion root.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let theme = self.theme;
        Accordion::new(theme).push(self).into_element()
    }
}

impl<'a, Message> From<AccordionItem<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(item: AccordionItem<'a, Message>) -> Self {
        item.into_element()
    }
}

impl<'a, Message> Accordion<'a, Message> {
    /// Creates an empty vertical, single-selection accordion.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            items: Vec::new(),
            accordion_type: AccordionType::Single,
            value: AccordionValue::default(),
            orientation: AccordionOrientation::Vertical,
            loop_navigation: AccordionLoop::Enabled,
            spacing: 0.0,
            disabled: false,
            animated: true,
            duration: DEFAULT_TRANSITION,
            width: Length::Fill,
            height: Length::Shrink,
            padding: None,
            background: None,
            bordered: None,
            radius: None,
            on_value_change: None,
            on_press: None,
            style_override: None,
        }
    }

    /// Creates an accordion with a pre-populated item iterator.
    pub fn with_items(
        theme: &'a Theme,
        items: impl IntoIterator<Item = AccordionItem<'a, Message>>,
    ) -> Self {
        Self::new(theme).extend(items)
    }

    /// Appends an item to the root.
    pub fn push(mut self, item: AccordionItem<'a, Message>) -> Self {
        self.items.push(item);
        self
    }

    /// Appends all items from an iterator.
    pub fn extend(mut self, items: impl IntoIterator<Item = AccordionItem<'a, Message>>) -> Self {
        self.items.extend(items);
        self
    }

    /// Returns the number of items in the root.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` when the root has no items.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Sets the single or multiple selection mode.
    pub fn accordion_type(mut self, accordion_type: AccordionType) -> Self {
        self.accordion_type = accordion_type;
        self.value = self.value.for_type(accordion_type);
        self
    }

    /// Alias for [`Self::accordion_type`].
    pub fn mode(self, accordion_type: AccordionType) -> Self {
        self.accordion_type(accordion_type)
    }

    /// Sets single-selection mode while preserving the first open value.
    pub fn single(self) -> Self {
        self.accordion_type(AccordionType::Single)
    }

    /// Sets multiple-selection mode while preserving all open values.
    pub fn multiple(self) -> Self {
        self.accordion_type(AccordionType::Multiple)
    }

    /// Returns the configured selection mode.
    #[must_use]
    pub fn selection_type(&self) -> AccordionType {
        self.accordion_type
    }

    /// Sets a controlled single or multiple value and derives its mode.
    pub fn value(mut self, value: impl Into<AccordionValue>) -> Self {
        self.value = value.into();
        self.accordion_type = self.value.value_type();
        self
    }

    /// Sets an optional controlled single open value.
    pub fn value_maybe(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = AccordionValue::single(value);
        self.accordion_type = AccordionType::Single;
        self
    }

    /// Clears the controlled single open value.
    pub fn clear_value(mut self) -> Self {
        self.value = AccordionValue::Single(None);
        self.accordion_type = AccordionType::Single;
        self
    }

    /// Sets controlled multiple open values, preserving order and uniqueness.
    pub fn values(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.value = AccordionValue::multiple(values);
        self.accordion_type = AccordionType::Multiple;
        self
    }

    /// Sets a controlled value and derives the selection mode.
    pub fn selection(mut self, value: impl Into<AccordionValue>) -> Self {
        self.value = value.into();
        self.accordion_type = self.value.value_type();
        self
    }

    /// Alias for [`Self::selection`].
    pub fn selected(self, value: impl Into<AccordionValue>) -> Self {
        self.selection(value)
    }

    /// Returns the controlled value currently supplied to the root.
    #[must_use]
    pub fn selected_value(&self) -> &AccordionValue {
        &self.value
    }

    /// Returns the next enabled trigger value for arrow-key navigation.
    ///
    /// This mirrors bits-ui's roving-focus order while keeping the accordion
    /// controlled: the returned value is a focus target, not an automatic
    /// selection change. Disabled items and items without triggers are skipped.
    /// The caller can map `ArrowDown`/`ArrowUp` in vertical mode or
    /// `ArrowRight`/`ArrowLeft` in horizontal mode to this helper.
    #[must_use]
    pub fn next_trigger_value(&self, current: Option<&str>) -> Option<String> {
        self.step_trigger_value(current, true)
    }

    /// Returns the previous enabled trigger value for arrow-key navigation.
    ///
    /// Wrap-around follows [`Self::loop_navigation`]. With no current trigger,
    /// this returns the last enabled trigger, matching reverse navigation from
    /// an unfocused root.
    #[must_use]
    pub fn previous_trigger_value(&self, current: Option<&str>) -> Option<String> {
        self.step_trigger_value(current, false)
    }

    fn step_trigger_value(&self, current: Option<&str>, forward: bool) -> Option<String> {
        if self.disabled {
            return None;
        }

        let current_index = current.and_then(|value| {
            self.items.iter().enumerate().position(|(index, item)| {
                let trigger_enabled = item
                    .trigger
                    .as_ref()
                    .is_some_and(|trigger| !trigger.disabled);
                if item.disabled || !trigger_enabled {
                    return false;
                }

                item.value.as_deref() == Some(value)
                    || (item.value.is_none() && value == format!("item-{}", index + 1))
            })
        });

        let index = shadcn_common::step_index(
            &self.items,
            current_index,
            if forward { 1 } else { -1 },
            matches!(self.loop_navigation, AccordionLoop::Enabled),
            |item| {
                !item.disabled
                    && item
                        .trigger
                        .as_ref()
                        .is_some_and(|trigger| !trigger.disabled)
            },
        )
        .or(current_index)?;

        Some(
            self.items[index]
                .value
                .clone()
                .unwrap_or_else(|| format!("item-{}", index + 1)),
        )
    }

    /// Sets the keyboard navigation orientation.
    pub fn orientation(mut self, orientation: AccordionOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Returns the configured arrow-key navigation orientation.
    #[must_use]
    pub const fn navigation_orientation(&self) -> AccordionOrientation {
        self.orientation
    }

    /// Sets the keyboard navigation wrapping policy.
    pub fn loop_navigation(mut self, loop_navigation: AccordionLoop) -> Self {
        self.loop_navigation = loop_navigation;
        self
    }

    /// Returns the configured arrow-key navigation wrapping policy.
    #[must_use]
    pub const fn navigation_loop(&self) -> AccordionLoop {
        self.loop_navigation
    }

    /// Disables every trigger while retaining controlled open content.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the gap between items in pixels.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = geometry::normalize_px(spacing);
        self
    }

    /// Enables or disables the content reveal transition.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets the content reveal duration, clamped to at least 1 ms.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration.max(Duration::from_millis(1));
        self
    }

    /// Sets the content reveal duration in milliseconds.
    pub fn duration_ms(self, duration_ms: u32) -> Self {
        self.duration(Duration::from_millis(u64::from(duration_ms)))
    }

    /// Sets the root width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the root height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets root padding using twill spacing values.
    ///
    /// # Errors
    ///
    /// Returns [`AccordionBuildError`] for `auto` or unresolved custom
    /// property values.
    pub fn padding(mut self, padding: Padding) -> Result<Self, AccordionBuildError> {
        self.padding = Some(geometry::resolve_padding(padding)?);
        Ok(self)
    }

    /// Paints the root on a semantic surface.
    pub fn background(mut self, background: SemanticColor) -> Self {
        self.background = Some(background);
        self
    }

    /// Draws a one-pixel border around the root.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = Some(bordered);
        self
    }

    /// Sets the root corner radius in pixels.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(geometry::normalize_px(radius));
        self
    }

    /// Sets the callback receiving the next controlled open value.
    pub fn on_value_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(AccordionValue) -> Message + 'a,
    {
        self.on_value_change = Some(Rc::new(callback));
        self.on_press = None;
        self
    }

    /// Sets or clears the controlled value callback.
    pub fn on_value_change_maybe<F>(mut self, callback: Option<F>) -> Self
    where
        F: Fn(AccordionValue) -> Message + 'a,
    {
        self.on_value_change = callback.map(|callback| Rc::new(callback) as _);
        self.on_press = None;
        self
    }

    /// Alias for [`Self::on_value_change`].
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: Fn(AccordionValue) -> Message + 'a,
    {
        self.on_value_change(callback)
    }

    /// Sets a callback receiving only the next single open value.
    pub fn on_change_single<F>(mut self, callback: F) -> Self
    where
        F: Fn(Option<String>) -> Message + 'a,
    {
        let callback = Rc::new(callback);
        self.on_value_change = Some(Rc::new(move |value| match value {
            AccordionValue::Single(value) => callback(value),
            AccordionValue::Multiple(values) => callback(values.into_iter().next()),
        }));
        self.on_press = None;
        self
    }

    /// Sets a callback receiving the next ordered multiple open values.
    pub fn on_change_multiple<F>(mut self, callback: F) -> Self
    where
        F: Fn(Vec<String>) -> Message + 'a,
    {
        let callback = Rc::new(callback);
        self.on_value_change = Some(Rc::new(move |value| match value {
            AccordionValue::Single(value) => callback(value.into_iter().collect()),
            AccordionValue::Multiple(values) => callback(values),
        }));
        self.on_press = None;
        self
    }

    /// Emits a cloned message for every enabled item press instead of a value
    /// callback.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self.on_value_change = None;
        self
    }

    /// Sets or clears the message emitted by every item press.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self.on_value_change = None;
        self
    }

    /// Applies an iced container-style override to the root.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the complete accordion as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_accordion(self)
    }
}

impl<'a, Message> From<Accordion<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(accordion: Accordion<'a, Message>) -> Self {
        accordion.into_element()
    }
}

/// Converts an accordion builder into an iced element.
pub fn accordion<'a, Message>(root: Accordion<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    root.into_element()
}
