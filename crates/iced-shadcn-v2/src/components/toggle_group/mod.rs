//! Toggle-group component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! A group is a controlled set of [`crate::Toggle`] controls. It supports the
//! same two selection modes as shadcn-svelte (`single` and `multiple`),
//! horizontal or vertical orientation, `default` and `outline` variants,
//! `sm`/`default`/`lg` sizes, spacing tokens, root/item disabled states, and
//! controlled value callbacks. The application owns the selected value and
//! feeds it back through [`ToggleGroup::value`], [`ToggleGroup::values`], or
//! [`ToggleGroup::selection`].
//!
//! The visual and state contract follows the upstream
//! [shadcn-svelte Toggle Group](https://github.com/huntabyte/shadcn-svelte/tree/next/sites/docs/src/lib/registry/ui/toggle-group)
//! registry component.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{
//!     Theme, ToggleGroup, ToggleGroupItem, ToggleGroupSelection, ToggleGroupType,
//! };
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Changed(ToggleGroupSelection),
//! }
//!
//! fn filters(theme: &Theme, selected: ToggleGroupSelection) -> Element<'_, Message> {
//!     ToggleGroup::new(theme)
//!         .group_type(ToggleGroupType::Multiple)
//!         .selection(selected)
//!         .push(ToggleGroupItem::text("bold", "Bold", theme))
//!         .push(ToggleGroupItem::text("italic", "Italic", theme))
//!         .on_selection_change(Message::Changed)
//!         .into()
//! }
//! ```

mod geometry;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{ToggleGroupOrientation, ToggleGroupSelection, ToggleGroupType};

pub use crate::components::toggle::{ToggleRadius, ToggleSize, ToggleVariant};

/// Alias for the variant enum inherited from [`crate::Toggle`].
pub type ToggleGroupVariant = ToggleVariant;

/// Alias for the size enum inherited from [`crate::Toggle`].
pub type ToggleGroupSize = ToggleSize;

/// Alias for the radius enum inherited from [`crate::Toggle`].
pub type ToggleGroupRadius = ToggleRadius;

/// Alias for the controlled value emitted by a toggle group.
pub type ToggleGroupValue = ToggleGroupSelection;

/// Alias for [`ToggleGroupType`] using the shorter mode terminology.
pub type ToggleGroupMode = ToggleGroupType;

use std::fmt;
use std::rc::Rc;

use crate::components::toggle::Toggle;
use crate::iced_compat::widget::button as button_widget;
use crate::iced_compat::widget::container;
use crate::iced_compat::widget::text::IntoFragment;
use crate::iced_compat::{Background, Element, Length};
use crate::theme::Theme;

type SelectionCallback<'a, Message> = Rc<dyn Fn(ToggleGroupSelection) -> Message + 'a>;

/// Builder-first root for a controlled toggle group.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct ToggleGroup<'a, Message> {
    theme: &'a Theme,
    items: Vec<ToggleGroupItem<'a, Message>>,
    group_type: ToggleGroupType,
    selection: ToggleGroupSelection,
    orientation: ToggleGroupOrientation,
    variant: ToggleVariant,
    size: ToggleSize,
    spacing: f32,
    disabled: bool,
    width: Length,
    height: Length,
    on_selection_change: Option<SelectionCallback<'a, Message>>,
    on_press: Option<Message>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for ToggleGroup<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ToggleGroup")
            .field("theme", &self.theme)
            .field("items", &self.items.len())
            .field("group_type", &self.group_type)
            .field("selection", &self.selection)
            .field("orientation", &self.orientation)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("spacing", &self.spacing)
            .field("disabled", &self.disabled)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("on_selection_change", &self.on_selection_change.is_some())
            .field("on_press", &self.on_press.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

/// One value-bearing control inside a [`ToggleGroup`].
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct ToggleGroupItem<'a, Message> {
    value: String,
    toggle: Toggle<'a, Message>,
    inherit_group_defaults: bool,
    variant: Option<ToggleVariant>,
    size: Option<ToggleSize>,
    radius: Option<ToggleRadius>,
    disabled: bool,
}

impl<Message> fmt::Debug for ToggleGroupItem<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ToggleGroupItem")
            .field("value", &self.value)
            .field("toggle", &self.toggle)
            .field("inherit_group_defaults", &self.inherit_group_defaults)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("disabled", &self.disabled)
            .finish()
    }
}

impl<'a, Message> ToggleGroupItem<'a, Message> {
    /// Creates an item from arbitrary iced content.
    pub fn new(
        value: impl Into<String>,
        content: impl Into<Element<'a, Message>>,
        theme: &'a Theme,
    ) -> Self {
        Self {
            value: value.into(),
            toggle: Toggle::new(content, theme),
            inherit_group_defaults: true,
            variant: None,
            size: None,
            radius: None,
            disabled: false,
        }
    }

    /// Creates a text item using the active toggle typography.
    pub fn text(value: impl Into<String>, label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            value: value.into(),
            toggle: Toggle::text(label, theme),
            inherit_group_defaults: true,
            variant: None,
            size: None,
            radius: None,
            disabled: false,
        }
    }

    /// Creates an icon-only item.
    pub fn icon(
        value: impl Into<String>,
        content: impl Into<Element<'a, Message>>,
        theme: &'a Theme,
    ) -> Self {
        Self {
            value: value.into(),
            toggle: Toggle::icon(content, theme),
            inherit_group_defaults: true,
            variant: None,
            size: None,
            radius: None,
            disabled: false,
        }
    }

    /// Wraps a preconfigured [`crate::Toggle`] without replacing its variant,
    /// size, or other per-control settings with root defaults.
    pub fn from_toggle(value: impl Into<String>, toggle: Toggle<'a, Message>) -> Self {
        Self {
            value: value.into(),
            toggle,
            inherit_group_defaults: false,
            variant: None,
            size: None,
            radius: None,
            disabled: false,
        }
    }

    /// Returns the stable value used in group selections.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Overrides the inherited visual variant for this item.
    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = Some(variant);
        self.toggle = self.toggle.variant(variant);
        self
    }

    /// Overrides the inherited visual size for this item.
    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = Some(size);
        self.toggle = self.toggle.size(size);
        self
    }

    /// Sets this item's corner radius.
    pub fn radius(mut self, radius: ToggleRadius) -> Self {
        self.radius = Some(radius);
        self.toggle = self.toggle.radius(radius);
        self
    }

    /// Adds a leading icon slot.
    pub fn icon_start(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.toggle = self.toggle.icon_start(icon);
        self
    }

    /// Adds a trailing icon slot.
    pub fn icon_end(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.toggle = self.toggle.icon_end(icon);
        self
    }

    /// Paints the item's invalid/destructive state.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.toggle = self.toggle.invalid(invalid);
        self
    }

    /// Disables this item while leaving the rest of the group interactive.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self.toggle = self.toggle.disabled(disabled);
        self
    }

    /// Sets a custom item width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.toggle = self.toggle.width(width);
        self
    }

    /// Sets a custom item height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.toggle = self.toggle.height(height);
        self
    }

    /// Sets custom item padding.
    pub fn padding(mut self, padding: impl Into<crate::iced_compat::Padding>) -> Self {
        self.toggle = self.toggle.padding(padding);
        self
    }

    /// Makes this item fill its parent width.
    pub fn full_width(mut self) -> Self {
        self.toggle = self.toggle.full_width();
        self
    }

    /// Applies an iced button-style override after toggle-group styling.
    pub fn style_override(
        mut self,
        style_override: impl Fn(button_widget::Style, button_widget::Status) -> button_widget::Style
        + 'a,
    ) -> Self {
        self.toggle = self.toggle.style_override(style_override);
        self
    }
}

impl<'a, Message> From<ToggleGroupItem<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(item: ToggleGroupItem<'a, Message>) -> Self {
        item.toggle
            .pressed(false)
            .on_toggle_maybe(None::<fn(bool) -> Message>)
            .into()
    }
}

impl<'a, Message> ToggleGroup<'a, Message> {
    /// Creates an empty horizontal single-selection group.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            items: Vec::new(),
            group_type: ToggleGroupType::Single,
            selection: ToggleGroupSelection::default(),
            orientation: ToggleGroupOrientation::Horizontal,
            variant: ToggleVariant::Default,
            size: ToggleSize::Default,
            spacing: 0.0,
            disabled: false,
            width: Length::Shrink,
            height: Length::Shrink,
            on_selection_change: None,
            on_press: None,
            style_override: None,
        }
    }

    /// Creates a group containing `items` with the default single-selection
    /// behavior.
    pub fn with_items(
        theme: &'a Theme,
        items: impl IntoIterator<Item = ToggleGroupItem<'a, Message>>,
    ) -> Self {
        Self::new(theme).extend(items)
    }

    /// Appends one value-bearing item.
    pub fn push(mut self, item: ToggleGroupItem<'a, Message>) -> Self {
        self.items.push(item);
        self
    }

    /// Appends all items from an iterator.
    pub fn extend(mut self, items: impl IntoIterator<Item = ToggleGroupItem<'a, Message>>) -> Self {
        self.items.extend(items);
        self
    }

    /// Returns the number of items in the group.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns whether the group has no items.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Sets the selection mode (`single` or `multiple`).
    pub fn group_type(mut self, group_type: ToggleGroupType) -> Self {
        self.group_type = group_type;
        self.selection = self.selection.for_type(group_type);
        self
    }

    /// Alias for [`Self::group_type`].
    pub fn mode(self, group_type: ToggleGroupType) -> Self {
        self.group_type(group_type)
    }

    /// Sets single-selection mode and keeps the first current value.
    pub fn single(self) -> Self {
        self.group_type(ToggleGroupType::Single)
    }

    /// Sets multiple-selection mode and preserves all current values.
    pub fn multiple(self) -> Self {
        self.group_type(ToggleGroupType::Multiple)
    }

    /// Returns the configured selection mode.
    #[must_use]
    pub fn selection_type(&self) -> ToggleGroupType {
        self.group_type
    }

    /// Sets a controlled selection value.
    ///
    /// Strings and `Option<String>` values select a single item; a
    /// [`ToggleGroupSelection`] can be supplied when the mode should be
    /// explicit. [`Self::values`] is the more readable multiple-value alias.
    pub fn value(mut self, value: impl Into<ToggleGroupSelection>) -> Self {
        self.selection = value.into();
        self.group_type = self.selection.selection_type();
        self
    }

    /// Sets an optional controlled single selected value.
    pub fn value_maybe(mut self, value: Option<impl Into<String>>) -> Self {
        self.selection = ToggleGroupSelection::Single(value.map(Into::into));
        self.group_type = ToggleGroupType::Single;
        self
    }

    /// Clears a controlled single selected value.
    pub fn clear_value(mut self) -> Self {
        self.group_type = ToggleGroupType::Single;
        self.selection = ToggleGroupSelection::Single(None);
        self
    }

    /// Sets controlled multiple selected values, preserving their order and
    /// removing duplicates.
    pub fn values(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.group_type = ToggleGroupType::Multiple;
        self.selection = ToggleGroupSelection::multiple(values);
        self
    }

    /// Sets a controlled selection and derives its selection mode.
    pub fn selection(mut self, selection: impl Into<ToggleGroupSelection>) -> Self {
        self.selection = selection.into();
        self.group_type = self.selection.selection_type();
        self
    }

    /// Alias for [`Self::selection`].
    pub fn selected(self, selection: impl Into<ToggleGroupSelection>) -> Self {
        self.selection(selection)
    }

    /// Sets the group visual variant inherited by its items.
    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the group size inherited by its items.
    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the layout orientation.
    pub fn orientation(mut self, orientation: ToggleGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the spacing token between items.
    ///
    /// The value is multiplied by the active style pack's spacing unit, so
    /// `spacing(2.0)` matches shadcn-svelte's `spacing={2}`.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = geometry::normalize_spacing(spacing);
        self
    }

    /// Disables the whole group, including every item.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the group width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the group height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets a callback receiving the next single selected value.
    ///
    /// For a multiple group use [`Self::on_change_values`] or
    /// [`Self::on_selection_change`].
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(Option<String>) -> Message + 'a,
    {
        let callback = Rc::new(callback);
        self.on_selection_change = Some(Rc::new(move |selection| match selection {
            ToggleGroupSelection::Single(value) => callback(value),
            ToggleGroupSelection::Multiple(values) => callback(values.into_iter().next()),
        }));
        self.on_press = None;
        self
    }

    /// Sets or clears the callback receiving the next single selected value.
    pub fn on_change_maybe<F>(mut self, callback: Option<F>) -> Self
    where
        F: Fn(Option<String>) -> Message + 'a,
    {
        match callback {
            Some(callback) => self.on_change(callback),
            None => {
                self.on_selection_change = None;
                self.on_press = None;
                self
            }
        }
    }

    /// Sets a callback receiving the next ordered multiple selection.
    ///
    /// For a single group use [`Self::on_change`] or
    /// [`Self::on_selection_change`].
    pub fn on_change_values<F>(mut self, callback: F) -> Self
    where
        F: Fn(Vec<String>) -> Message + 'a,
    {
        let callback = Rc::new(callback);
        self.on_selection_change = Some(Rc::new(move |selection| match selection {
            ToggleGroupSelection::Single(value) => callback(value.into_iter().collect()),
            ToggleGroupSelection::Multiple(values) => callback(values),
        }));
        self.on_press = None;
        self
    }

    /// Sets or clears the callback receiving the next multiple selection.
    pub fn on_change_values_maybe<F>(mut self, callback: Option<F>) -> Self
    where
        F: Fn(Vec<String>) -> Message + 'a,
    {
        match callback {
            Some(callback) => self.on_change_values(callback),
            None => {
                self.on_selection_change = None;
                self.on_press = None;
                self
            }
        }
    }

    /// Sets a callback receiving the discriminated next selection value.
    pub fn on_selection_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(ToggleGroupSelection) -> Message + 'a,
    {
        self.on_selection_change = Some(Rc::new(callback));
        self.on_press = None;
        self
    }

    /// Sets or clears the callback receiving the discriminated next value.
    pub fn on_selection_change_maybe<F>(mut self, callback: Option<F>) -> Self
    where
        F: Fn(ToggleGroupSelection) -> Message + 'a,
    {
        self.on_selection_change = callback.map(|callback| Rc::new(callback) as _);
        self.on_press = None;
        self
    }

    /// Alias for [`Self::on_selection_change`], matching bits-ui's
    /// `onValueChange` terminology.
    pub fn on_value_change<F>(self, callback: F) -> Self
    where
        F: Fn(ToggleGroupSelection) -> Message + 'a,
    {
        self.on_selection_change(callback)
    }

    /// Emits a cloned message on every enabled item press instead of a
    /// controlled-value callback.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self.on_selection_change = None;
        self
    }

    /// Sets or clears the message emitted by every item press.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self.on_selection_change = None;
        self
    }

    /// Applies an iced container-style override after group style resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the group as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let ToggleGroup {
            theme,
            items,
            group_type,
            selection,
            orientation,
            variant,
            size,
            spacing,
            disabled,
            width,
            height,
            on_selection_change,
            on_press,
            style_override,
        } = self;

        let selection = selection.for_type(group_type);
        let item_count = items.len();
        let spacing_px = geometry::spacing_px(theme, spacing);
        let merged_borders = geometry::merged_borders(variant, spacing);
        let group_radius = geometry::default_radius(theme);
        let mut rendered = Vec::with_capacity(item_count);

        for (index, item) in items.into_iter().enumerate() {
            let is_first = index == 0;
            let is_last = index + 1 == item_count;
            let pressed = selection.is_selected(&item.value);
            let next_selection = selection.toggled(group_type, &item.value);
            let item_disabled = disabled || item.disabled;
            let item_variant = item.variant.unwrap_or(variant);
            let item_size = item.size.unwrap_or(size);
            let item_radius = if item.inherit_group_defaults {
                item.radius.or({
                    if spacing <= f32::EPSILON {
                        if is_first || is_last {
                            Some(group_radius)
                        } else {
                            Some(ToggleRadius::None)
                        }
                    } else {
                        None
                    }
                })
            } else {
                item.radius
            };

            let mut toggle = item.toggle;
            if item.inherit_group_defaults {
                toggle = toggle.variant(item_variant).size(item_size);
            }
            toggle = toggle
                .pressed(pressed)
                .on_toggle_maybe(None::<fn(bool) -> Message>);

            if let Some(radius) = item_radius {
                toggle = toggle.radius(radius);
            }

            if item_disabled {
                toggle = toggle.disabled(true);
            }

            if geometry::merged_borders(item_variant, spacing) {
                toggle = toggle.chain_style_override(|mut style, _| {
                    style.border.width = 0.0;
                    style.shadow = Default::default();
                    style
                });
            }

            if !item_disabled {
                if let Some(callback) = on_selection_change.as_ref() {
                    let callback = Rc::clone(callback);
                    let next_selection = next_selection.clone();
                    toggle = toggle.on_toggle(move |_| callback(next_selection.clone()));
                } else if let Some(message) = on_press.as_ref() {
                    toggle = toggle.on_press(message.clone());
                }
            }

            let element: Element<'a, Message> = toggle.into();
            if orientation.is_vertical() {
                rendered.push(
                    crate::iced_compat::widget::container(element)
                        .width(Length::Fill)
                        .into(),
                );
            } else {
                rendered.push(element);
            }

            if merged_borders && !is_last {
                let separator_color = {
                    let mut color =
                        theme.semantic_color(twill_core::prelude::theme::SemanticColor::Input);
                    if disabled {
                        color.a *= 0.5;
                    }
                    color
                };
                let separator =
                    crate::iced_compat::widget::container(crate::iced_compat::widget::space())
                        .width(if orientation.is_vertical() {
                            Length::Fill
                        } else {
                            Length::Fixed(1.0)
                        })
                        .height(if orientation.is_vertical() {
                            Length::Fixed(1.0)
                        } else {
                            Length::Fill
                        })
                        .style(move |_| container::Style {
                            background: Some(Background::Color(separator_color)),
                            ..container::Style::default()
                        })
                        .into();
                rendered.push(separator);
            }
        }

        let content: Element<'a, Message> = if orientation.is_vertical() {
            crate::iced_compat::widget::column(rendered)
                .spacing(spacing_px)
                .into()
        } else {
            crate::iced_compat::widget::row(rendered)
                .spacing(spacing_px)
                .align_y(crate::iced_compat::alignment::Vertical::Center)
                .into()
        };

        crate::iced_compat::widget::container(content)
            .width(width)
            .height(height)
            .style(move |_| {
                let mut style = style::resolve_group_style(theme, variant, spacing, disabled);
                if let Some(override_fn) = style_override.as_ref() {
                    style = override_fn(style);
                }
                style
            })
            .into()
    }
}

impl<'a, Message> From<ToggleGroup<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(group: ToggleGroup<'a, Message>) -> Self {
        group.into_element()
    }
}
