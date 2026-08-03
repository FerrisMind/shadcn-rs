//! Keyboard-accessible, composable step navigation.
//!
//! This is a port of the `shadcn-svelte-extras` stepper composition. The
//! controlled [`Stepper::step`] value is 1-indexed. A caller supplies an
//! `on_step_change` callback to turn clicks, arrow-key navigation, and the
//! previous/next controls into the application's message type.
//!
//! The indicator, rail, title, and description inherit the active [`Theme`]
//! palette and font pack. The previous and next controls are built from the
//! crate's [`crate::Button`] builder, so style packs such as `Rhea`, `Mira`,
//! and `Sera` are reflected in their size, radius, typography, and colors.
//!
//! # Example
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{
//!     Stepper, StepperIndicator, StepperItem, StepperNext, StepperPrevious,
//!     StepperTrigger, Theme,
//! };
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     StepChanged(usize),
//! }
//!
//! fn view(step: usize, theme: &Theme) -> Element<'_, Message> {
//!     let items = (1..=3).map(|number| {
//!         StepperItem::new(
//!             StepperTrigger::new(theme)
//!                 .indicator(StepperIndicator::text(number.to_string(), theme)),
//!         )
//!     });
//!
//!     Stepper::with_items(theme, items)
//!         .step(step)
//!         .on_step_change(Message::StepChanged)
//!         .previous(StepperPrevious::text("Previous", theme))
//!         .next(StepperNext::text("Next", theme))
//!         .into()
//! }
//! ```

mod geometry;
mod render;
mod types;

#[cfg(test)]
mod tests;

pub use types::{
    Stepper, StepperDescription, StepperIndicator, StepperItem, StepperItemState, StepperNav,
    StepperNext, StepperOrientation, StepperPrevious, StepperSeparator, StepperTitle,
    StepperTrigger,
};

use crate::iced_compat::widget::button;
use crate::iced_compat::widget::container;
use crate::iced_compat::widget::text::IntoFragment;
use crate::iced_compat::{Element, Font, Length, Padding};
use crate::theme::Theme;
use shadcn_common::AccentColor;

use self::geometry::{normalize_min_px, normalize_padding, normalize_px};
use self::types::{StepperButtonContent, StepperContent};
use super::button::{ButtonSize, ButtonVariant};

impl<'a, Message> StepperIndicator<'a, Message> {
    /// Creates an indicator from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(StepperContent::Element(content.into()), theme)
    }

    /// Creates a text indicator using the active theme font.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(StepperContent::Label(label.into_fragment()), theme)
    }

    /// Overrides the indicator diameter in pixels.
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(normalize_min_px(size));
        self
    }

    /// Overrides the indicator foreground color.
    pub fn color(mut self, color: crate::iced_compat::Color) -> Self {
        self.foreground = Some(color);
        self
    }

    /// Overrides the indicator surface color.
    pub fn background(mut self, color: crate::iced_compat::Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Overrides the three-pixel ring color separating the indicator from the rail.
    pub fn ring_color(mut self, color: crate::iced_compat::Color) -> Self {
        self.ring_color = Some(color);
        self
    }

    /// Applies a narrow iced container-style override after state colors resolve.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

impl<'a, Message> StepperTitle<'a, Message> {
    /// Creates a title from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(StepperContent::Element(content.into()), theme)
    }

    /// Creates a style-pack-aware text title.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(StepperContent::Label(label.into_fragment()), theme)
    }

    /// Overrides the title text color.
    pub fn color(mut self, color: crate::iced_compat::Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Overrides the title text size in pixels.
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = Some(normalize_min_px(text_size));
        self
    }

    /// Overrides the title line height in pixels.
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(normalize_min_px(line_height));
        self
    }

    /// Overrides the title font.
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Applies an iced container-style override to the title.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

impl<'a, Message> StepperDescription<'a, Message> {
    /// Creates a description from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(StepperContent::Element(content.into()), theme)
    }

    /// Creates a style-pack-aware text description.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(StepperContent::Label(label.into_fragment()), theme)
    }

    /// Overrides the description text color.
    pub fn color(mut self, color: crate::iced_compat::Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Overrides the description text size in pixels.
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = Some(normalize_min_px(text_size));
        self
    }

    /// Overrides the description line height in pixels.
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(normalize_min_px(line_height));
        self
    }

    /// Overrides the description font.
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Applies an iced container-style override to the description.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

impl<'a, Message> StepperSeparator<'a, Message> {
    /// Creates a default state-colored separator.
    pub fn new(theme: &'a Theme) -> Self {
        Self::empty(theme)
    }

    /// Creates a separator whose content replaces the default rail.
    pub fn with_content(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: Some(StepperContent::Element(content.into())),
            ..Self::empty(theme)
        }
    }

    /// Creates a text separator, replacing the default rail.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: Some(StepperContent::Label(label.into_fragment())),
            ..Self::empty(theme)
        }
    }

    /// Returns `true` when this separator uses the default rail.
    #[must_use]
    pub const fn is_default_rail(&self) -> bool {
        self.content.is_none()
    }

    /// Moves the rail start toward the next item in pixels.
    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = normalize_px(offset);
        self
    }

    /// Sets the rail thickness in pixels.
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = normalize_min_px(thickness);
        self
    }

    /// Overrides the unfinished rail color.
    pub fn color(mut self, color: crate::iced_compat::Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Overrides the completed rail color.
    pub fn completed_color(mut self, color: crate::iced_compat::Color) -> Self {
        self.completed_color = Some(color);
        self
    }

    /// Applies an iced container-style override to custom separator content.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

impl<'a, Message> StepperTrigger<'a, Message> {
    /// Creates an empty trigger to be filled with indicator and text slots.
    pub fn new(theme: &'a Theme) -> Self {
        Self::empty(theme)
    }

    /// Creates a trigger with one arbitrary child.
    pub fn with_content(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::empty(theme).push(content)
    }

    /// Sets the circular indicator slot.
    pub fn indicator(mut self, indicator: StepperIndicator<'a, Message>) -> Self {
        self.indicator = Some(indicator);
        self
    }

    /// Sets the title slot.
    pub fn title(mut self, title: StepperTitle<'a, Message>) -> Self {
        self.title = Some(title);
        self
    }

    /// Sets the description slot.
    pub fn description(mut self, description: StepperDescription<'a, Message>) -> Self {
        self.description = Some(description);
        self
    }

    /// Appends arbitrary content after the standard slots.
    pub fn push(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(content.into());
        self
    }

    /// Sets the trigger width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the trigger height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the gap between trigger content in pixels.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = Some(normalize_px(gap));
        self
    }

    /// Disables this trigger while retaining its visual state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets a message emitted when this trigger is clicked.
    ///
    /// This overrides the root [`Stepper::on_step_change`] callback for this
    /// trigger, which is useful when a step has a dedicated action message.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets or clears the per-trigger press message.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Applies an iced button-style override after the transparent trigger style.
    pub fn style_override(
        mut self,
        style_override: impl Fn(button::Style, button::Status) -> button::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

impl<'a, Message> StepperItem<'a, Message> {
    /// Creates an item with an auto-generated stable id.
    pub fn new(trigger: StepperTrigger<'a, Message>) -> Self {
        Self {
            id: None,
            trigger,
            separator: None,
            disabled: false,
        }
    }

    /// Creates an item with an explicit id used by [`Stepper::step_for_id`].
    pub fn with_id(id: impl Into<String>, trigger: StepperTrigger<'a, Message>) -> Self {
        Self {
            id: Some(id.into()),
            ..Self::new(trigger)
        }
    }

    /// Sets the stable item id.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Returns the configured item id, if one was supplied.
    #[must_use]
    pub fn item_id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Sets the visual and behavioral trigger.
    pub fn trigger(mut self, trigger: StepperTrigger<'a, Message>) -> Self {
        self.trigger = trigger;
        self
    }

    /// Sets the separator after this item.
    pub fn separator(mut self, separator: StepperSeparator<'a, Message>) -> Self {
        self.separator = Some(separator);
        self
    }

    /// Sets or clears the separator after this item.
    pub fn separator_maybe(mut self, separator: Option<StepperSeparator<'a, Message>>) -> Self {
        self.separator = separator;
        self
    }

    /// Disables this item in addition to its trigger's disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl StepperNav {
    /// Creates a horizontal, full-width navigation configuration.
    pub fn new() -> Self {
        Self {
            orientation: StepperOrientation::Horizontal,
            width: Length::Fill,
            height: Length::Shrink,
            padding: Padding::default(),
            gap: None,
            style_override: None,
        }
    }

    /// Sets the navigation orientation.
    pub fn orientation(mut self, orientation: StepperOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the navigation width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the navigation height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets navigation padding, clamping non-finite and negative values to zero.
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = normalize_padding(padding);
        self
    }

    /// Sets the gap between vertical items in pixels.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = Some(normalize_px(gap));
        self
    }

    /// Applies an iced container-style override to the navigation surface.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'static,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

impl Default for StepperNav {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> StepperNext<'a, Message> {
    /// Creates a next control from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(StepperButtonContent::Element(content.into()), theme)
    }

    /// Creates a next control with a text label.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(StepperButtonContent::label(label.into_fragment()), theme)
    }

    /// Sets the shadcn button variant.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the shadcn button size.
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Applies an accent color overlay to the button.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Disables the control in addition to the automatic last-step state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets a custom press message instead of the automatic next-step message.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets or clears the custom press message.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Applies the same narrow style escape hatch as [`crate::Button`].
    pub fn style_override(
        mut self,
        style_override: impl Fn(button::Style, button::Status) -> button::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

impl<'a, Message> StepperPrevious<'a, Message> {
    /// Creates a previous control from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(StepperButtonContent::Element(content.into()), theme)
    }

    /// Creates a previous control with a text label.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(StepperButtonContent::label(label.into_fragment()), theme)
    }

    /// Sets the shadcn button variant.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the shadcn button size.
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Applies an accent color overlay to the button.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Disables the control in addition to the automatic first-step state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets a custom press message instead of the automatic previous-step message.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets or clears the custom press message.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Applies the same narrow style escape hatch as [`crate::Button`].
    pub fn style_override(
        mut self,
        style_override: impl Fn(button::Style, button::Status) -> button::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

impl<'a, Message> Stepper<'a, Message> {
    /// Creates an empty controlled stepper.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            items: Vec::new(),
            nav: StepperNav::new(),
            step: 1,
            previous: None,
            next: None,
            spacing: None,
            width: Length::Fill,
            height: Length::Shrink,
            padding: Padding::default(),
            disabled: false,
            on_step_change: None,
            style_override: None,
        }
    }

    /// Creates a stepper populated from an iterator of items.
    pub fn with_items(
        theme: &'a Theme,
        items: impl IntoIterator<Item = StepperItem<'a, Message>>,
    ) -> Self {
        Self::new(theme).extend(items)
    }

    /// Appends one item to the navigation.
    pub fn push(mut self, item: StepperItem<'a, Message>) -> Self {
        self.items.push(item);
        self
    }

    /// Appends all items from an iterator.
    pub fn extend(mut self, items: impl IntoIterator<Item = StepperItem<'a, Message>>) -> Self {
        self.items.extend(items);
        self
    }

    /// Returns the number of items in the stepper.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` when the stepper has no items.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Sets the controlled active step. Values are clamped when rendered.
    pub fn step(mut self, step: usize) -> Self {
        self.step = step;
        self
    }

    /// Returns the controlled step after clamping it to the current item count.
    #[must_use]
    pub fn active_step(&self) -> usize {
        geometry::resolve_step(self.step, self.items.len())
    }

    /// Returns the next step number, if one exists.
    #[must_use]
    pub fn next_step(&self) -> Option<usize> {
        geometry::next_step(self.active_step(), self.items.len())
    }

    /// Returns the previous step number, if one exists.
    #[must_use]
    pub fn previous_step(&self) -> Option<usize> {
        geometry::previous_step(self.active_step())
    }

    /// Returns whether the automatic next control can advance.
    #[must_use]
    pub fn can_increment(&self) -> bool {
        self.next_step().is_some()
    }

    /// Returns whether the automatic previous control can go back.
    #[must_use]
    pub fn can_decrement(&self) -> bool {
        self.previous_step().is_some()
    }

    /// Returns the 1-indexed step for an item id, if it exists and is enabled.
    #[must_use]
    pub fn step_for_id(&self, id: &str) -> Option<usize> {
        self.items.iter().enumerate().find_map(|(index, item)| {
            let matches = item.id.as_deref() == Some(id);
            (!item.disabled && !item.trigger.disabled && matches).then_some(index + 1)
        })
    }

    /// Sets the navigation configuration.
    pub fn nav(mut self, nav: StepperNav) -> Self {
        self.nav = nav;
        self
    }

    /// Sets the navigation orientation.
    pub fn orientation(mut self, orientation: StepperOrientation) -> Self {
        self.nav.orientation = orientation;
        self
    }

    /// Sets the gap between the navigation and its optional controls.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(normalize_px(spacing));
        self
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

    /// Sets root padding, clamping invalid values to zero.
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = normalize_padding(padding);
        self
    }

    /// Disables every trigger and automatic control.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the callback used by trigger and keyboard navigation.
    pub fn on_step_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize) -> Message + 'a,
    {
        self.on_step_change = Some(Box::new(callback));
        self
    }

    /// Sets or clears the step-change callback.
    pub fn on_step_change_maybe<F>(mut self, callback: Option<F>) -> Self
    where
        F: Fn(usize) -> Message + 'a,
    {
        self.on_step_change = callback.map(|callback| Box::new(callback) as _);
        self
    }

    /// Adds the previous-step control.
    pub fn previous(mut self, previous: StepperPrevious<'a, Message>) -> Self {
        self.previous = Some(previous);
        self
    }

    /// Adds the next-step control.
    pub fn next(mut self, next: StepperNext<'a, Message>) -> Self {
        self.next = Some(next);
        self
    }

    /// Applies an iced container-style override to the transparent root.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the stepper as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_stepper(self)
    }
}

impl<'a, Message> From<Stepper<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(stepper: Stepper<'a, Message>) -> Self {
        stepper.into_element()
    }
}

/// Creates an empty stepper builder.
pub fn stepper<'a, Message>(theme: &'a Theme) -> Stepper<'a, Message> {
    Stepper::new(theme)
}

/// Creates an item from an already configured trigger.
pub fn stepper_item<'a, Message>(trigger: StepperTrigger<'a, Message>) -> StepperItem<'a, Message> {
    StepperItem::new(trigger)
}

/// Creates an empty trigger builder.
pub fn stepper_trigger<'a, Message>(theme: &'a Theme) -> StepperTrigger<'a, Message> {
    StepperTrigger::new(theme)
}

/// Creates a text indicator.
pub fn stepper_indicator<'a, Message>(
    label: impl IntoFragment<'a>,
    theme: &'a Theme,
) -> StepperIndicator<'a, Message> {
    StepperIndicator::text(label, theme)
}

/// Creates a text title.
pub fn stepper_title<'a, Message>(
    label: impl IntoFragment<'a>,
    theme: &'a Theme,
) -> StepperTitle<'a, Message> {
    StepperTitle::text(label, theme)
}

/// Creates a text description.
pub fn stepper_description<'a, Message>(
    label: impl IntoFragment<'a>,
    theme: &'a Theme,
) -> StepperDescription<'a, Message> {
    StepperDescription::text(label, theme)
}

/// Creates a default separator.
pub fn stepper_separator<'a, Message>(theme: &'a Theme) -> StepperSeparator<'a, Message> {
    StepperSeparator::new(theme)
}

/// Creates a text next control.
pub fn stepper_next<'a, Message>(
    label: impl IntoFragment<'a>,
    theme: &'a Theme,
) -> StepperNext<'a, Message> {
    StepperNext::text(label, theme)
}

/// Creates a text previous control.
pub fn stepper_previous<'a, Message>(
    label: impl IntoFragment<'a>,
    theme: &'a Theme,
) -> StepperPrevious<'a, Message> {
    StepperPrevious::text(label, theme)
}
