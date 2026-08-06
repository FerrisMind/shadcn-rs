//! Radio-group component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! A radio group is a controlled set of items where at most one value is
//! selected. The application owns the selected value and receives the next one
//! through [`RadioGroup::on_change`], exactly like the `bind:value` contract of
//! the web component. Indicator, dot, ring, and gap geometry come from the
//! active style pack (`.cn-radio-group*`), so a group changes shape together
//! with [`crate::Theme`].
//!
//! Beyond the web component's `value`, `orientation`, `disabled`, `readonly`,
//! `required`, `name`, and `loop` props, the builder exposes the states the
//! pack CSS defines but the web props do not (focus ring, `aria-invalid` ring),
//! size and radius presets, per-item descriptions, and a style escape hatch.
//!
//! Because iced has no accessibility tree and no roving tabindex, keyboard
//! navigation is app-driven: paint the ring with [`RadioGroup::focused`] and ask
//! [`RadioGroup::next_value`] / [`RadioGroup::previous_value`] what an arrow key
//! should select. Both honour [`RadioGroup::loop_navigation`] and skip disabled
//! items, mirroring bits-ui's `loop` behaviour.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{RadioGroup, RadioGroupItem, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     SpacingChanged(String),
//! }
//!
//! fn spacing<'a>(theme: &'a Theme, selected: &str) -> Element<'a, Message> {
//!     RadioGroup::new(theme)
//!         .value(selected)
//!         .push(RadioGroupItem::text("default", "Default"))
//!         .push(RadioGroupItem::text("comfortable", "Comfortable"))
//!         .push(RadioGroupItem::text("compact", "Compact"))
//!         .on_change(Message::SpacingChanged)
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
    RadioGroupOrientation, RadioGroupRadius, RadioGroupSize, RadioGroupStatus, RadioGroupStyle,
};

use std::borrow::Cow;
use std::fmt;
use std::rc::Rc;

use crate::iced_compat::widget::container;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Element, Length};

use crate::theme::Theme;

type ValueCallback<'a, Message> = Rc<dyn Fn(String) -> Message + 'a>;
type StyleOverride<'a> = Rc<dyn Fn(RadioGroupStyle, RadioGroupStatus) -> RadioGroupStyle + 'a>;

/// Label content of one item — the iced stand-in for the `children` snippet.
enum ItemContent<'a, Message> {
    Empty,
    Text(Fragment<'a>),
    Element(Element<'a, Message>),
}

/// One value-bearing item inside a [`RadioGroup`].
///
/// Items do not borrow the theme: the group resolves colors and geometry for
/// every item it renders, so one item can be reused under any [`Theme`].
///
/// ```rust
/// use iced_shadcn_v2::RadioGroupItem;
///
/// # #[derive(Debug, Clone)]
/// # enum Message {}
/// let item = RadioGroupItem::<Message>::text("yearly", "Yearly ($99.99/year)")
///     .description("Save 17% compared to monthly billing");
/// assert_eq!(item.value(), "yearly");
/// ```
#[must_use = "items do nothing unless pushed into a RadioGroup"]
pub struct RadioGroupItem<'a, Message> {
    value: String,
    content: ItemContent<'a, Message>,
    description: Option<Fragment<'a>>,
    id: Option<Cow<'a, str>>,
    disabled: bool,
    invalid: bool,
    focused: bool,
    width: Option<Length>,
    style_override: Option<StyleOverride<'a>>,
}

impl<Message> fmt::Debug for RadioGroupItem<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.content {
            ItemContent::Empty => "empty",
            ItemContent::Text(_) => "text",
            ItemContent::Element(_) => "element",
        };

        formatter
            .debug_struct("RadioGroupItem")
            .field("value", &self.value)
            .field("content", &content)
            .field("description", &self.description.is_some())
            .field("id", &self.id.as_deref())
            .field("disabled", &self.disabled)
            .field("invalid", &self.invalid)
            .field("focused", &self.focused)
            .field("width", &self.width)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> RadioGroupItem<'a, Message> {
    /// Creates an indicator-only item carrying `value`.
    pub fn new(value: impl Into<String>) -> Self {
        Self::from_content(value, ItemContent::Empty)
    }

    /// Creates an item with a themed label beside the indicator.
    pub fn text(value: impl Into<String>, label: impl IntoFragment<'a>) -> Self {
        Self::from_content(value, ItemContent::Text(label.into_fragment()))
    }

    /// Creates an item with arbitrary iced content beside the indicator.
    pub fn with_content(
        value: impl Into<String>,
        content: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self::from_content(value, ItemContent::Element(content.into()))
    }

    fn from_content(value: impl Into<String>, content: ItemContent<'a, Message>) -> Self {
        Self {
            value: value.into(),
            content,
            description: None,
            id: None,
            disabled: false,
            invalid: false,
            focused: false,
            width: None,
            style_override: None,
        }
    }

    /// Returns the value this item selects.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the control id, if one was set.
    #[must_use]
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Replaces the label with themed text.
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.content = ItemContent::Text(label.into_fragment());
        self
    }

    /// Adds a muted second line under the label.
    pub fn description(mut self, description: impl IntoFragment<'a>) -> Self {
        self.description = Some(description.into_fragment());
        self
    }

    /// Sets the control id (`id` on the web, paired with a label's `for`).
    ///
    /// iced does not yet expose an accessibility tree, so the id is carried for
    /// API parity and for apps that wire focus manually — the same contract as
    /// [`crate::Label::for_id`].
    pub fn id_attr(mut self, id: impl Into<Cow<'a, str>>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Disables this item while the rest of the group stays interactive.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Paints this item's `aria-invalid` destructive border and ring.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Paints this item's `focus-visible` ring.
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Overrides the width inherited from [`RadioGroup::item_width`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Patches this item's resolved [`RadioGroupStyle`] right before it is
    /// painted, after any [`RadioGroup::item_style_override`].
    pub fn style_override(
        mut self,
        style_override: impl Fn(RadioGroupStyle, RadioGroupStatus) -> RadioGroupStyle + 'a,
    ) -> Self {
        self.style_override = Some(Rc::new(style_override));
        self
    }
}

/// Builder-first radio group styled from `shadcn-common` theme tokens.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{RadioGroup, RadioGroupItem, RadioGroupOrientation, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Changed(String),
/// }
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     RadioGroup::new(theme)
///         .value("all")
///         .name("notify")
///         .orientation(RadioGroupOrientation::Horizontal)
///         .push(RadioGroupItem::text("all", "All new messages"))
///         .push(RadioGroupItem::text("mentions", "Mentions"))
///         .push(RadioGroupItem::text("none", "Nothing").disabled(true))
///         .on_change(Message::Changed)
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct RadioGroup<'a, Message> {
    theme: &'a Theme,
    items: Vec<RadioGroupItem<'a, Message>>,
    value: Option<String>,
    focused: Option<String>,
    orientation: RadioGroupOrientation,
    size: RadioGroupSize,
    radius: Option<RadioGroupRadius>,
    spacing: Option<f32>,
    label_spacing: Option<f32>,
    disabled: bool,
    readonly: bool,
    invalid: bool,
    required: bool,
    loop_navigation: bool,
    name: Option<Cow<'a, str>>,
    width: Length,
    height: Length,
    item_width: Length,
    on_change: Option<ValueCallback<'a, Message>>,
    on_press: Option<Message>,
    item_style_override: Option<StyleOverride<'a>>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for RadioGroup<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RadioGroup")
            .field("theme", &self.theme)
            .field("items", &self.items.len())
            .field("value", &self.value)
            .field("focused", &self.focused)
            .field("orientation", &self.orientation)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("spacing", &self.spacing)
            .field("label_spacing", &self.label_spacing)
            .field("disabled", &self.disabled)
            .field("readonly", &self.readonly)
            .field("invalid", &self.invalid)
            .field("required", &self.required)
            .field("loop_navigation", &self.loop_navigation)
            .field("name", &self.name.as_deref())
            .field("width", &self.width)
            .field("height", &self.height)
            .field("item_width", &self.item_width)
            .field("on_change", &self.on_change.is_some())
            .field("on_press", &self.on_press.is_some())
            .field("item_style_override", &self.item_style_override.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> RadioGroup<'a, Message> {
    /// Creates an empty vertical group with no selected value.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{RadioGroup, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let group = RadioGroup::<Message>::new(&theme);
    /// assert!(group.is_empty());
    /// ```
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            items: Vec::new(),
            value: None,
            focused: None,
            orientation: RadioGroupOrientation::Vertical,
            size: RadioGroupSize::Default,
            radius: None,
            spacing: None,
            label_spacing: None,
            disabled: false,
            readonly: false,
            invalid: false,
            required: false,
            loop_navigation: true,
            name: None,
            width: Length::Shrink,
            height: Length::Shrink,
            item_width: Length::Shrink,
            on_change: None,
            on_press: None,
            item_style_override: None,
            style_override: None,
        }
    }

    /// Creates a group containing `items`.
    pub fn with_items(
        theme: &'a Theme,
        items: impl IntoIterator<Item = RadioGroupItem<'a, Message>>,
    ) -> Self {
        Self::new(theme).extend(items)
    }

    /// Appends one item.
    pub fn push(mut self, item: RadioGroupItem<'a, Message>) -> Self {
        self.items.push(item);
        self
    }

    /// Appends all items from an iterator.
    pub fn extend(mut self, items: impl IntoIterator<Item = RadioGroupItem<'a, Message>>) -> Self {
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

    /// Returns the values of every item, in layout order.
    pub fn values(&self) -> impl Iterator<Item = &str> {
        self.items.iter().map(RadioGroupItem::value)
    }

    /// Sets the controlled selected value.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Sets or clears the controlled selected value.
    pub fn value_maybe(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = value.map(Into::into);
        self
    }

    /// Clears the controlled selected value.
    pub fn clear_value(mut self) -> Self {
        self.value = None;
        self
    }

    /// Returns the controlled selected value, if any.
    #[must_use]
    pub fn selected_value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Returns whether `value` is the selected one.
    #[must_use]
    pub fn is_selected(&self, value: &str) -> bool {
        self.value.as_deref() == Some(value)
    }

    /// Paints the `focus-visible` ring on the item carrying `value`.
    ///
    /// A canvas-free composite cannot take keyboard focus in iced, so the ring
    /// is driven from application state — the same contract as
    /// [`crate::Switch::focused`].
    pub fn focused(mut self, value: impl Into<String>) -> Self {
        self.focused = Some(value.into());
        self
    }

    /// Sets or clears the focused item.
    pub fn focused_maybe(mut self, value: Option<impl Into<String>>) -> Self {
        self.focused = value.map(Into::into);
        self
    }

    /// Sets the layout axis.
    pub fn orientation(mut self, orientation: RadioGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the indicator footprint preset.
    pub fn size(mut self, size: RadioGroupSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the indicator corner radius.
    pub fn radius(mut self, radius: RadioGroupRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Sets the gap between items in style-pack spacing units.
    ///
    /// `spacing(3.0)` matches the web component's `gap-3`. Without this call the
    /// group uses the active pack's own `.cn-radio-group` gap.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(geometry::normalize_spacing(spacing));
        self
    }

    /// Sets the gap between an indicator and its label in spacing units.
    ///
    /// `label_spacing(2.0)` matches the web component's `space-x-2`.
    pub fn label_spacing(mut self, spacing: f32) -> Self {
        self.label_spacing = Some(geometry::normalize_spacing(spacing));
        self
    }

    /// Suppresses interaction and dims every item.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Makes the group inert while keeping its normal colors.
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Paints the `aria-invalid` destructive border and ring on every item.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Marks the group as required.
    ///
    /// iced has no form submission, so the flag is carried for API parity and
    /// read back through [`Self::is_required`] by app-side validation.
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Returns whether the group was marked required.
    #[must_use]
    pub fn is_required(&self) -> bool {
        self.required
    }

    /// Sets the form field name.
    ///
    /// Carried for API parity with the web component's hidden input; read it
    /// back through [`Self::name`] when serializing app state.
    pub fn name_attr(mut self, name: impl Into<Cow<'a, str>>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Alias for [`Self::name_attr`].
    pub fn name(self, name: impl Into<Cow<'a, str>>) -> Self {
        self.name_attr(name)
    }

    /// Returns the form field name, if one was set.
    #[must_use]
    pub fn field_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Enables or disables wrap-around for [`Self::next_value`] and
    /// [`Self::previous_value`] (bits-ui's `loop`, enabled by default).
    pub fn loop_navigation(mut self, loop_navigation: bool) -> Self {
        self.loop_navigation = loop_navigation;
        self
    }

    /// Value an arrow key toward the end of the group should select.
    ///
    /// Disabled items are skipped. With no current selection the first enabled
    /// item is returned; at the end of the group the answer wraps only when
    /// [`Self::loop_navigation`] is enabled. Returns `None` for an empty group,
    /// or when the whole group is disabled or read-only.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{RadioGroup, RadioGroupItem, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let group = RadioGroup::<Message>::new(&theme)
    ///     .push(RadioGroupItem::new("a"))
    ///     .push(RadioGroupItem::new("b").disabled(true))
    ///     .push(RadioGroupItem::new("c"))
    ///     .value("a");
    ///
    /// assert_eq!(group.next_value(), Some("c"));
    /// assert_eq!(group.previous_value(), Some("c"));
    /// ```
    #[must_use]
    pub fn next_value(&self) -> Option<&str> {
        self.step_value(true)
    }

    /// Value an arrow key toward the start of the group should select.
    ///
    /// Mirrors [`Self::next_value`] in the opposite direction.
    #[must_use]
    pub fn previous_value(&self) -> Option<&str> {
        self.step_value(false)
    }

    fn step_value(&self, forward: bool) -> Option<&str> {
        if self.disabled || self.readonly {
            return None;
        }

        let current_index = self.value.as_deref().and_then(|value| {
            self.items
                .iter()
                .position(|item| !item.disabled && item.value == value)
        });

        let index = shadcn_common::step_index(
            &self.items,
            current_index,
            if forward { 1 } else { -1 },
            self.loop_navigation,
            |item| !item.disabled,
        )
        .or(current_index)?;

        Some(self.items[index].value.as_str())
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

    /// Sets the width every item inherits unless it overrides it.
    pub fn item_width(mut self, width: impl Into<Length>) -> Self {
        self.item_width = width.into();
        self
    }

    /// Makes the group and its items fill the available width.
    ///
    /// This is the iced equivalent of the web component's `w-full` root.
    pub fn full_width(mut self) -> Self {
        self.width = Length::Fill;
        self.item_width = Length::Fill;
        self
    }

    /// Sets the callback invoked with the next selected value.
    ///
    /// The group stays controlled: it keeps painting [`Self::value`] until the
    /// application stores the new one. Pressing the already selected item emits
    /// nothing, because radios never deselect.
    pub fn on_change<F>(mut self, on_change: F) -> Self
    where
        F: Fn(String) -> Message + 'a,
    {
        self.on_change = Some(Rc::new(on_change));
        self.on_press = None;
        self
    }

    /// Sets or clears the selected-value callback.
    ///
    /// A group without a callback is inert but keeps its normal colors, which is
    /// how read-only previews are rendered.
    pub fn on_change_maybe<F>(mut self, on_change: Option<F>) -> Self
    where
        F: Fn(String) -> Message + 'a,
    {
        self.on_change = on_change.map(|callback| Rc::new(callback) as _);
        self.on_press = None;
        self
    }

    /// Alias for [`Self::on_change`], matching bits-ui's `onValueChange`.
    pub fn on_value_change<F>(self, on_value_change: F) -> Self
    where
        F: Fn(String) -> Message + 'a,
    {
        self.on_change(on_value_change)
    }

    /// Emits a cloned message on every enabled press, ignoring the value.
    ///
    /// Unlike [`Self::on_change`], this also fires for the already selected
    /// item.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self.on_change = None;
        self
    }

    /// Sets or clears the message emitted by every enabled press.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self.on_change = None;
        self
    }

    /// Patches the resolved [`RadioGroupStyle`] of every item before painting.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{RadioGroup, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let group = RadioGroup::<Message>::new(&theme).item_style_override(|mut style, status| {
    ///     if status.checked {
    ///         style.border_width += 1.0;
    ///     }
    ///
    ///     style
    /// });
    /// ```
    pub fn item_style_override(
        mut self,
        style_override: impl Fn(RadioGroupStyle, RadioGroupStatus) -> RadioGroupStyle + 'a,
    ) -> Self {
        self.item_style_override = Some(Rc::new(style_override));
        self
    }

    /// Applies an iced container-style override to the group root.
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
        let RadioGroup {
            theme,
            items,
            value,
            focused,
            orientation,
            size,
            radius,
            spacing,
            label_spacing,
            disabled,
            readonly,
            invalid,
            required: _,
            loop_navigation: _,
            name: _,
            width,
            height,
            item_width,
            on_change,
            on_press,
            item_style_override,
            style_override,
        } = self;

        let metrics = geometry::resolve_metrics(theme, size, radius);
        let gap = geometry::gap_px(theme, spacing);
        let layout = render::ItemLayout {
            footprint: metrics.footprint(),
            label_gap: geometry::label_gap_px(theme, metrics, label_spacing),
            width: item_width,
        };
        let mut rendered = Vec::with_capacity(items.len());

        for item in items {
            let checked = value.as_deref() == Some(item.value.as_str());
            let item_disabled = disabled || item.disabled;
            let status = RadioGroupStatus {
                checked,
                disabled: item_disabled,
                readonly,
                focused: item.focused || focused.as_deref() == Some(item.value.as_str()),
                invalid: invalid || item.invalid,
            };

            let mut style = style::resolve_style(theme, metrics, status);
            if let Some(override_fn) = item_style_override.as_ref() {
                style = override_fn(style, status);
            }
            if let Some(override_fn) = item.style_override.as_ref() {
                style = override_fn(style, status);
            }

            let press = if item_disabled || readonly {
                None
            } else if let Some(callback) = on_change.as_ref() {
                (!checked).then(|| callback(item.value.clone()))
            } else {
                on_press.clone()
            };

            rendered.push(render::build_item(
                theme,
                item.content,
                item.description,
                style,
                render::ItemLayout {
                    width: item.width.unwrap_or(layout.width),
                    ..layout
                },
                press,
            ));
        }

        render::build_group(rendered, orientation, gap, width, height, style_override)
    }
}

/// Creates an empty [`RadioGroup`] for `theme`.
///
/// ```rust
/// use iced_shadcn_v2::{RadioGroupItem, Theme, radio_group};
///
/// # #[derive(Debug, Clone)]
/// # enum Message {}
/// let theme = Theme::light();
/// let group = radio_group::<Message>(&theme).push(RadioGroupItem::text("a", "A"));
/// assert_eq!(group.len(), 1);
/// ```
pub fn radio_group<Message>(theme: &Theme) -> RadioGroup<'_, Message> {
    RadioGroup::new(theme)
}

impl<'a, Message> From<RadioGroup<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(group: RadioGroup<'a, Message>) -> Self {
        group.into_element()
    }
}
