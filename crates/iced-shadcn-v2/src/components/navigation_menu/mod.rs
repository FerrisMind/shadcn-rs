//! Builder-first navigation-menu component.
//!
//! Port of the shadcn-svelte navigation menu (`NavigationMenu.Root` / `List` /
//! `Item` / `Trigger` / `Content` / `Link` / `Indicator` / `Viewport`) as an
//! iced builder. Hover opens a trigger after `delayDuration`, switching
//! between open items uses a shortened delay, leaving both the trigger list
//! and the floating panel arms `closeDelay`, click toggles, Esc closes, and
//! arrow keys move roving focus. Content is painted either in a shared
//! viewport panel (default) or as a per-item floating surface
//! (`viewport=false`). Timing, placement, and style recipes live in
//! [`shadcn_common`] so egui can share the same behaviour layer.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{
//!     NavigationMenu, NavigationMenuItem, Theme, navigation_menu_content,
//! };
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     OpenChanged(String),
//! }
//!
//! fn view(theme: &Theme) -> Element<'_, Message> {
//!     NavigationMenu::new(theme)
//!         .item(NavigationMenuItem::trigger("home", "Home").content(
//!             navigation_menu_content(iced::widget::text("Welcome"), theme),
//!         ))
//!         .on_value_change(Message::OpenChanged)
//!         .into()
//! }
//! ```

mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use style::NavigationMenuViewportStyle;
pub use types::{
    NavigationMenuContentProps, NavigationMenuJustify, NavigationMenuLinkProps,
    NavigationMenuLinkVariant, NavigationMenuListProps, NavigationMenuOrientation,
    NavigationMenuProps, NavigationMenuSize, NavigationMenuWrap,
};

pub use shadcn_common::{NavigationMenuAlign, NavigationMenuSide, NavigationMenuTiming};

use std::fmt;

use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::widget::{container, text};
use crate::iced_compat::{Element, Length};
use crate::theme::Theme;

use render::{
    NavItemKind, NavItemMeta, NavigationMenuLinkWidget, NavigationMenuTriggerWidget,
    NavigationMenuWidget,
};
use style::{recipe, resolve_content_style, resolve_viewport_style};
use types::NavigationMenuLinkVariant as LinkVariant;

/// Trigger / content pair or top-level link entry.
pub enum NavigationMenuItem<'a, Message> {
    /// Trigger that opens floating content.
    Trigger {
        /// Stable value used for controlled `value` / `on_value_change`.
        value: String,
        /// Trigger label or custom element.
        trigger: NavigationMenuTriggerContent<'a, Message>,
        /// Whether to show the chevron icon.
        show_chevron: bool,
        /// Disables the trigger.
        disabled: bool,
        /// Floating content panel.
        content: NavigationMenuContent<'a, Message>,
    },
    /// Top-level link without a panel.
    Link {
        /// Stable value (not used as open state).
        value: String,
        /// Link content.
        content: Element<'a, Message>,
        /// Press message.
        on_press: Option<Message>,
        /// Link props.
        props: NavigationMenuLinkProps,
    },
}

impl<Message> fmt::Debug for NavigationMenuItem<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Trigger {
                value,
                show_chevron,
                disabled,
                ..
            } => formatter
                .debug_struct("Trigger")
                .field("value", value)
                .field("show_chevron", show_chevron)
                .field("disabled", disabled)
                .finish_non_exhaustive(),
            Self::Link { value, props, .. } => formatter
                .debug_struct("Link")
                .field("value", value)
                .field("props", props)
                .finish_non_exhaustive(),
        }
    }
}

impl<'a, Message> NavigationMenuItem<'a, Message> {
    /// Creates a trigger item with a text label.
    pub fn trigger(
        value: impl Into<String>,
        label: impl IntoFragment<'a>,
    ) -> NavigationMenuTriggerBuilder<'a, Message> {
        NavigationMenuTriggerBuilder {
            value: value.into(),
            trigger: NavigationMenuTriggerContent::Text(label.into_fragment()),
            show_chevron: true,
            disabled: false,
            content_props: NavigationMenuContentProps::new(),
        }
    }

    /// Creates a trigger item with custom content.
    pub fn trigger_with(
        value: impl Into<String>,
        content: impl Into<Element<'a, Message>>,
    ) -> NavigationMenuTriggerBuilder<'a, Message> {
        NavigationMenuTriggerBuilder {
            value: value.into(),
            trigger: NavigationMenuTriggerContent::Element(content.into()),
            show_chevron: true,
            disabled: false,
            content_props: NavigationMenuContentProps::new(),
        }
    }

    /// Creates a top-level link item.
    pub fn link(
        value: impl Into<String>,
        content: impl Into<Element<'a, Message>>,
        on_press: Option<Message>,
    ) -> Self {
        Self::Link {
            value: value.into(),
            content: content.into(),
            on_press,
            props: NavigationMenuLinkProps::new().variant(LinkVariant::Trigger),
        }
    }
}

/// Intermediate builder for a trigger + content item.
#[must_use = "builders do nothing unless turned into a NavigationMenuItem"]
pub struct NavigationMenuTriggerBuilder<'a, Message> {
    value: String,
    trigger: NavigationMenuTriggerContent<'a, Message>,
    show_chevron: bool,
    disabled: bool,
    content_props: NavigationMenuContentProps,
}

impl<Message> fmt::Debug for NavigationMenuTriggerBuilder<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NavigationMenuTriggerBuilder")
            .field("value", &self.value)
            .field("show_chevron", &self.show_chevron)
            .field("disabled", &self.disabled)
            .field("content_props", &self.content_props)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> NavigationMenuTriggerBuilder<'a, Message> {
    /// Attaches floating content.
    pub fn content(
        self,
        content: impl Into<NavigationMenuContent<'a, Message>>,
    ) -> NavigationMenuItem<'a, Message> {
        let mut content = content.into();
        if content.props == NavigationMenuContentProps::default() {
            content.props = self.content_props;
        }
        NavigationMenuItem::Trigger {
            value: self.value,
            trigger: self.trigger,
            show_chevron: self.show_chevron,
            disabled: self.disabled,
            content,
        }
    }

    /// Sets content props before attaching content.
    pub fn content_props(mut self, props: NavigationMenuContentProps) -> Self {
        self.content_props = props;
        self
    }

    /// Shows or hides the chevron.
    pub fn show_chevron(mut self, show: bool) -> Self {
        self.show_chevron = show;
        self
    }

    /// Disables the trigger.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Trigger label content.
pub enum NavigationMenuTriggerContent<'a, Message> {
    /// Plain text label.
    Text(Fragment<'a>),
    /// Custom element.
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for NavigationMenuTriggerContent<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(_) => formatter.write_str("Text(..)"),
            Self::Element(_) => formatter.write_str("Element(..)"),
        }
    }
}

/// Floating content panel (`NavigationMenu.Content`).
pub struct NavigationMenuContent<'a, Message> {
    /// Panel body.
    pub content: Element<'a, Message>,
    /// Placement / size props.
    pub props: NavigationMenuContentProps,
}

impl<Message> fmt::Debug for NavigationMenuContent<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NavigationMenuContent")
            .field("props", &self.props)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> NavigationMenuContent<'a, Message> {
    /// Creates content with default props.
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            props: NavigationMenuContentProps::new(),
        }
    }

    /// Overrides content props.
    pub fn props(mut self, props: NavigationMenuContentProps) -> Self {
        self.props = props;
        self
    }
}

impl<'a, Message> From<Element<'a, Message>> for NavigationMenuContent<'a, Message> {
    fn from(content: Element<'a, Message>) -> Self {
        Self::new(content)
    }
}

/// Builder-first navigation menu styled directly with iced types.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct NavigationMenu<'a, Message> {
    theme: &'a Theme,
    items: Vec<NavigationMenuItem<'a, Message>>,
    root_props: NavigationMenuProps,
    list_props: NavigationMenuListProps,
    value: Option<String>,
    on_value_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
}

impl<Message> fmt::Debug for NavigationMenu<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NavigationMenu")
            .field("theme", &self.theme)
            .field("items", &self.items.len())
            .field("root_props", &self.root_props)
            .field("list_props", &self.list_props)
            .field("value", &self.value)
            .field("on_value_change", &self.on_value_change.is_some())
            .finish()
    }
}

impl<'a, Message> NavigationMenu<'a, Message> {
    /// Creates an empty navigation menu.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            items: Vec::new(),
            root_props: NavigationMenuProps::new(),
            list_props: NavigationMenuListProps::new(),
            value: None,
            on_value_change: None,
        }
    }

    /// Appends a list item.
    pub fn item(mut self, item: NavigationMenuItem<'a, Message>) -> Self {
        self.items.push(item);
        self
    }

    /// Replaces root props.
    pub fn props(mut self, props: NavigationMenuProps) -> Self {
        self.root_props = props;
        self
    }

    /// Replaces list props.
    pub fn list_props(mut self, props: NavigationMenuListProps) -> Self {
        self.list_props = props;
        self
    }

    /// Enables or disables the shared viewport.
    pub fn viewport(mut self, viewport: bool) -> Self {
        self.root_props.viewport = viewport;
        self
    }

    /// Enables or disables the indicator.
    pub fn indicator(mut self, indicator: bool) -> Self {
        self.root_props.indicator = indicator;
        self
    }

    /// Sets timing knobs.
    pub fn timing(mut self, timing: NavigationMenuTiming) -> Self {
        self.root_props.timing = timing;
        self
    }

    /// Sets the controlled open value (`bind:value`). Empty string closes.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        let value = value.into();
        self.value = (!value.is_empty()).then_some(value);
        self
    }

    /// Sets the controlled open value from an optional string.
    pub fn value_maybe(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = value.map(Into::into).filter(|value| !value.is_empty());
        self
    }

    /// Uncontrolled initial open value.
    pub fn default_value(mut self, value: &'static str) -> Self {
        self.root_props.default_value = Some(value);
        self
    }

    /// Notifies when the open value changes (`onValueChange`).
    pub fn on_value_change(mut self, on_value_change: impl Fn(String) -> Message + 'a) -> Self {
        self.on_value_change = Some(Box::new(on_value_change));
        self
    }
}

/// Convenience constructor matching other components (`dropdown_menu`, …).
pub fn navigation_menu<'a, Message>(theme: &'a Theme) -> NavigationMenu<'a, Message> {
    NavigationMenu::new(theme)
}

/// Wraps arbitrary content as a [`NavigationMenuContent`] with recipe padding.
pub fn navigation_menu_content<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> NavigationMenuContent<'a, Message> {
    let pad = recipe(theme).content_pad_px;
    NavigationMenuContent::new(container(content.into()).padding(pad))
        .props(NavigationMenuContentProps::new().padding(pad))
}

/// Builds an in-content or top-level navigation-menu link.
pub fn navigation_menu_link<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    on_press: Option<Message>,
    props: NavigationMenuLinkProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let width = if props.full_width {
        Length::Fill
    } else {
        props.width
    };

    Element::new(NavigationMenuLinkWidget {
        content: content.into(),
        on_press,
        props,
        theme: theme.clone(),
        width,
        height: props.height,
    })
}

/// Trigger-styled link props helper (matches `navigationMenuTriggerStyle()`).
#[must_use]
pub fn navigation_menu_trigger_style() -> NavigationMenuLinkProps {
    NavigationMenuLinkProps::new().variant(LinkVariant::Trigger)
}

impl<'a, Message> From<NavigationMenu<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(menu: NavigationMenu<'a, Message>) -> Element<'a, Message> {
        let theme = menu.theme.clone();
        let list_props = menu.list_props;
        let root_props = menu.root_props;
        let recipe = recipe(&theme);

        let mut triggers = Vec::new();
        let mut contents = Vec::new();
        let mut metas = Vec::new();

        for item in menu.items {
            match item {
                NavigationMenuItem::Trigger {
                    value,
                    trigger,
                    show_chevron,
                    disabled,
                    content,
                } => {
                    let content_index = contents.len();
                    let resolved_props = content.props;
                    let mut wrapper = container(content.content).padding(resolved_props.padding);
                    if let Some(width) = resolved_props.width {
                        wrapper = wrapper.width(Length::Fixed(width));
                    }
                    if let Some(max_height) = resolved_props.max_height {
                        wrapper = wrapper.max_height(max_height);
                    }
                    contents.push(wrapper.into());

                    let label = match trigger {
                        NavigationMenuTriggerContent::Text(label) => text(label)
                            .size(
                                list_props
                                    .size
                                    .text_size()
                                    .max(recipe.trigger_typography.size_px),
                            )
                            .into(),
                        NavigationMenuTriggerContent::Element(element) => element,
                    };

                    let link_props = NavigationMenuLinkProps::new()
                        .variant(LinkVariant::Trigger)
                        .size(list_props.size)
                        .disabled(disabled);

                    triggers.push(
                        NavigationMenuTriggerWidget {
                            content: label,
                            show_chevron,
                            icon_size: list_props.size.icon_size(),
                            pad_y: recipe.trigger_pad_y_px,
                            pad_x: recipe.trigger_pad_x_px,
                            disabled,
                            theme: theme.clone(),
                            link_props,
                        }
                        .into(),
                    );

                    metas.push(NavItemMeta {
                        value,
                        kind: NavItemKind::Trigger,
                        disabled,
                        content_index: Some(content_index),
                        content_props: resolved_props,
                        link_message: None,
                    });
                }
                NavigationMenuItem::Link {
                    value,
                    content,
                    on_press,
                    props,
                } => {
                    triggers.push(navigation_menu_link(
                        content,
                        on_press.clone(),
                        props,
                        &theme,
                    ));
                    metas.push(NavItemMeta {
                        value,
                        kind: NavItemKind::Link,
                        disabled: props.disabled,
                        content_index: None,
                        content_props: NavigationMenuContentProps::new(),
                        link_message: on_press,
                    });
                }
            }
        }

        Element::new(NavigationMenuWidget {
            triggers,
            contents,
            items: metas,
            value: menu.value,
            on_value_change: menu.on_value_change,
            root_props,
            list_props,
            theme,
            viewport_style: resolve_viewport_style(menu.theme),
            content_style: resolve_content_style(menu.theme),
        })
    }
}

impl<'a, Message: Clone + 'a> From<NavigationMenuTriggerWidget<'a, Message>>
    for Element<'a, Message>
{
    fn from(widget: NavigationMenuTriggerWidget<'a, Message>) -> Self {
        Element::new(widget)
    }
}
