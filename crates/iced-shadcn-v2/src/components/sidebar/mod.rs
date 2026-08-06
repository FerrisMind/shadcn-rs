//! Builder-first sidebar component.
//!
//! Port of the shadcn-svelte sidebar (`Sidebar.Provider` / `Root` / slots /
//! menu primitives). Shared open/collapse/mobile behaviour and geometry live
//! in [`shadcn_common::SidebarController`] and [`shadcn_common::sidebar_recipe`]
//! so egui can reuse the same tokens.
//!
//! ```rust,no_run
//! use iced::widget::row;
//! use iced::Element;
//! use iced_shadcn_v2::{
//!     Sidebar, SidebarCollapsible, SidebarContent, SidebarController, SidebarGroup,
//!     SidebarGroupContent, SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarMenu,
//!     SidebarMenuButton, SidebarMenuItem, SidebarProvider, SidebarTrigger, Theme,
//! };
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Toggle,
//!     Nav(&'static str),
//! }
//!
//! fn view<'a>(theme: &'a Theme, ctrl: &'a SidebarController) -> Element<'a, Message> {
//!     SidebarProvider::new(theme)
//!         .push(row![
//!             Sidebar::new(ctrl, theme)
//!                 .collapsible(SidebarCollapsible::Icon)
//!                 .header(SidebarHeader::new(theme).push(iced::widget::text("Acme")))
//!                 .content(
//!                     SidebarContent::new(theme).push(
//!                         SidebarGroup::new(theme)
//!                             .label(SidebarGroupLabel::text("Platform", theme))
//!                             .content(
//!                                 SidebarGroupContent::new(theme).push(
//!                                     SidebarMenu::new(theme).push(
//!                                         SidebarMenuItem::new(theme).push(
//!                                             SidebarMenuButton::text("Playground", ctrl, theme)
//!                                                 .active(true)
//!                                                 .tooltip("Playground")
//!                                                 .on_press(Message::Nav("playground")),
//!                                         ),
//!                                     ),
//!                                 ),
//!                             ),
//!                     ),
//!                 ),
//!             SidebarInset::new(theme)
//!                 .header(SidebarTrigger::new(theme).on_press(Message::Toggle))
//!                 .push(iced::widget::text("Main")),
//!         ])
//!         .into()
//! }
//! ```

mod animate;
mod icon;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use style::SidebarStyle;
pub use types::{
    SidebarCollapsible, SidebarController, SidebarDisplayState, SidebarMenuButtonSize,
    SidebarMenuButtonVariant, SidebarMenuSubButtonSize, SidebarSide, SidebarVariant,
};

use std::fmt;

use crate::iced_compat::widget::text::IntoFragment;
use crate::iced_compat::{Element, Length};

use crate::theme::Theme;

/// Full-viewport wrapper (`Sidebar.Provider` / `sidebar-wrapper`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarProvider<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) on_viewport_change: Option<Box<dyn Fn(f32) -> Message + 'a>>,
}

impl<Message> fmt::Debug for SidebarProvider<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarProvider")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("on_viewport_change", &self.on_viewport_change.is_some())
            .finish()
    }
}

impl<'a, Message> SidebarProvider<'a, Message> {
    /// Creates an empty provider.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            on_viewport_change: None,
        }
    }

    /// Appends a child (typically a row of [`Sidebar`] + [`SidebarInset`]).
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Emits the window width on resize so the app can call
    /// [`SidebarController::set_viewport_width`] (md breakpoint = 768px).
    pub fn on_viewport_change<F>(mut self, on_viewport_change: F) -> Self
    where
        F: Fn(f32) -> Message + 'a,
    {
        self.on_viewport_change = Some(Box::new(on_viewport_change));
        self
    }

    /// Builds the provider.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_provider(self)
    }
}

impl<'a, Message> From<SidebarProvider<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarProvider<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Desktop / mobile sidebar root (`Sidebar.Root`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Sidebar<'a, Message> {
    pub(super) controller: &'a SidebarController,
    pub(super) theme: &'a Theme,
    pub(super) side: SidebarSide,
    pub(super) variant: SidebarVariant,
    pub(super) collapsible: SidebarCollapsible,
    pub(super) animated: bool,
    pub(super) header: Option<SidebarHeader<'a, Message>>,
    pub(super) content: Option<SidebarContent<'a, Message>>,
    pub(super) footer: Option<SidebarFooter<'a, Message>>,
    pub(super) rail: Option<SidebarRail<'a, Message>>,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) on_mobile_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    pub(super) style_override: Option<Box<dyn Fn(SidebarStyle) -> SidebarStyle + 'a>>,
}

impl<Message> fmt::Debug for Sidebar<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sidebar")
            .field("controller", &self.controller)
            .field("theme", &self.theme)
            .field("side", &self.side)
            .field("variant", &self.variant)
            .field("collapsible", &self.collapsible)
            .field("animated", &self.animated)
            .field("header", &self.header.is_some())
            .field("content", &self.content.is_some())
            .field("footer", &self.footer.is_some())
            .field("rail", &self.rail.is_some())
            .field("children", &self.children.len())
            .field(
                "on_mobile_open_change",
                &self.on_mobile_open_change.is_some(),
            )
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Sidebar<'a, Message> {
    /// Creates an empty sidebar bound to `controller`.
    pub fn new(controller: &'a SidebarController, theme: &'a Theme) -> Self {
        Self {
            controller,
            theme,
            side: SidebarSide::Left,
            variant: SidebarVariant::Sidebar,
            collapsible: SidebarCollapsible::Offcanvas,
            animated: true,
            header: None,
            content: None,
            footer: None,
            rail: None,
            children: Vec::new(),
            on_mobile_open_change: None,
            style_override: None,
        }
    }

    /// Sets the dock edge.
    pub fn side(mut self, side: SidebarSide) -> Self {
        self.side = side;
        self
    }

    /// Sets the visual variant.
    pub fn variant(mut self, variant: SidebarVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the collapse behaviour.
    pub fn collapsible(mut self, collapsible: SidebarCollapsible) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Enables or disables the `duration-200` width transition (default: on).
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets the header slot.
    pub fn header(mut self, header: SidebarHeader<'a, Message>) -> Self {
        self.header = Some(header);
        self
    }

    /// Sets the scrollable content slot.
    pub fn content(mut self, content: SidebarContent<'a, Message>) -> Self {
        self.content = Some(content);
        self
    }

    /// Sets the footer slot.
    pub fn footer(mut self, footer: SidebarFooter<'a, Message>) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Sets the collapse rail.
    pub fn rail(mut self, rail: SidebarRail<'a, Message>) -> Self {
        self.rail = Some(rail);
        self
    }

    /// Appends arbitrary root children.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Emits when the mobile sheet open state changes.
    pub fn on_mobile_open_change(
        mut self,
        on_mobile_open_change: impl Fn(bool) -> Message + 'a,
    ) -> Self {
        self.on_mobile_open_change = Some(Box::new(on_mobile_open_change));
        self
    }

    /// Patches the resolved sidebar style.
    pub fn style_override(
        mut self,
        style_override: impl Fn(SidebarStyle) -> SidebarStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the sidebar.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_sidebar(self)
    }
}

impl<'a, Message> From<Sidebar<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: Sidebar<'a, Message>) -> Self {
        value.into_element()
    }
}

macro_rules! section_builder {
    ($name:ident, $doc:literal, $build:path) => {
        #[doc = $doc]
        #[must_use = "builders do nothing unless turned into an iced Element"]
        pub struct $name<'a, Message> {
            pub(super) theme: &'a Theme,
            pub(super) children: Vec<Element<'a, Message>>,
        }

        impl<Message> fmt::Debug for $name<'_, Message> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("theme", &self.theme)
                    .field("children", &self.children.len())
                    .finish()
            }
        }

        impl<'a, Message> $name<'a, Message> {
            /// Creates an empty section.
            pub fn new(theme: &'a Theme) -> Self {
                Self {
                    theme,
                    children: Vec::new(),
                }
            }

            /// Appends a child.
            pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
                self.children.push(child.into());
                self
            }

            /// Builds the section.
            pub fn into_element(self) -> Element<'a, Message>
            where
                Message: Clone + 'a,
            {
                let style = style::resolve_style(self.theme);
                $build(self, false, style)
            }
        }

        impl<'a, Message> From<$name<'a, Message>> for Element<'a, Message>
        where
            Message: Clone + 'a,
        {
            fn from(value: $name<'a, Message>) -> Self {
                value.into_element()
            }
        }
    };
}

section_builder!(
    SidebarHeader,
    "Sidebar header slot (`Sidebar.Header`).",
    render::build_header
);
section_builder!(
    SidebarContent,
    "Scrollable sidebar body (`Sidebar.Content`).",
    render::build_content
);
section_builder!(
    SidebarFooter,
    "Sidebar footer slot (`Sidebar.Footer`).",
    render::build_footer
);

/// Main content area beside the sidebar (`Sidebar.Inset`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarInset<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) variant: SidebarVariant,
    pub(super) header: Option<Element<'a, Message>>,
    pub(super) children: Vec<Element<'a, Message>>,
}

impl<Message> fmt::Debug for SidebarInset<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarInset")
            .field("theme", &self.theme)
            .field("variant", &self.variant)
            .field("header", &self.header.is_some())
            .field("children", &self.children.len())
            .finish()
    }
}

impl<'a, Message> SidebarInset<'a, Message> {
    /// Creates an empty inset.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            variant: SidebarVariant::Sidebar,
            header: None,
            children: Vec::new(),
        }
    }

    /// Mirrors the sidebar variant so inset rounding activates for `Inset`.
    pub fn variant(mut self, variant: SidebarVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets a top header row (typically [`SidebarTrigger`] + breadcrumbs).
    pub fn header(mut self, header: impl Into<Element<'a, Message>>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Appends body content.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Builds the inset.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_inset(self)
    }
}

impl<'a, Message> From<SidebarInset<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarInset<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Ghost icon trigger that toggles the sidebar (`Sidebar.Trigger`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarTrigger<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) on_press: Option<Message>,
}

impl<Message> fmt::Debug for SidebarTrigger<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarTrigger")
            .field("theme", &self.theme)
            .field("on_press", &self.on_press.is_some())
            .finish()
    }
}

impl<'a, Message> SidebarTrigger<'a, Message> {
    /// Creates a trigger.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            on_press: None,
        }
    }

    /// Sets the press message (typically `controller.toggle()` in `update`).
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Builds the trigger.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_trigger(self)
    }
}

impl<'a, Message> From<SidebarTrigger<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarTrigger<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Edge hit-target that toggles the sidebar (`Sidebar.Rail`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarRail<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) on_press: Option<Message>,
}

impl<Message> fmt::Debug for SidebarRail<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarRail")
            .field("theme", &self.theme)
            .field("on_press", &self.on_press.is_some())
            .finish()
    }
}

impl<'a, Message> SidebarRail<'a, Message> {
    /// Creates a rail.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            on_press: None,
        }
    }

    /// Sets the press message.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Builds the rail.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let style = style::resolve_style(self.theme);
        render::build_rail(self, style)
    }
}

impl<'a, Message> From<SidebarRail<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarRail<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Sidebar-styled search input (`Sidebar.Input`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarInput<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) value: &'a str,
    pub(super) placeholder: &'a str,
    pub(super) on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
}

impl<Message> fmt::Debug for SidebarInput<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarInput")
            .field("theme", &self.theme)
            .field("value", &self.value)
            .field("placeholder", &self.placeholder)
            .field("on_input", &self.on_input.is_some())
            .finish()
    }
}

impl<'a, Message> SidebarInput<'a, Message> {
    /// Creates an input bound to `value`.
    pub fn new(theme: &'a Theme, value: &'a str) -> Self {
        Self {
            theme,
            value,
            placeholder: "",
            on_input: None,
        }
    }

    /// Sets the placeholder.
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    /// Sets the input callback.
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// Builds the input.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_input(self)
    }
}

impl<'a, Message> From<SidebarInput<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarInput<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Sidebar separator (`Sidebar.Separator`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarSeparator<'a, Message> {
    pub(super) theme: &'a Theme,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> fmt::Debug for SidebarSeparator<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarSeparator")
            .field("theme", &self.theme)
            .finish()
    }
}

impl<'a, Message> SidebarSeparator<'a, Message> {
    /// Creates a separator.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            _message: std::marker::PhantomData,
        }
    }

    /// Builds the separator.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_separator(self)
    }
}

impl<'a, Message> From<SidebarSeparator<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(value: SidebarSeparator<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Navigation group (`Sidebar.Group`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarGroup<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) label: Option<SidebarGroupLabel<'a, Message>>,
    pub(super) action: Option<SidebarGroupAction<'a, Message>>,
    pub(super) content: Option<SidebarGroupContent<'a, Message>>,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) icon_mode: bool,
}

impl<Message> fmt::Debug for SidebarGroup<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarGroup")
            .field("theme", &self.theme)
            .field("label", &self.label.is_some())
            .field("action", &self.action.is_some())
            .field("content", &self.content.is_some())
            .field("children", &self.children.len())
            .field("icon_mode", &self.icon_mode)
            .finish()
    }
}

impl<'a, Message> SidebarGroup<'a, Message> {
    /// Creates an empty group.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            label: None,
            action: None,
            content: None,
            children: Vec::new(),
            icon_mode: false,
        }
    }

    /// Hides the label when the sidebar is in icon-collapsed mode.
    pub fn icon_mode(mut self, icon_mode: bool) -> Self {
        self.icon_mode = icon_mode;
        self
    }

    /// Sets the group label.
    pub fn label(mut self, label: SidebarGroupLabel<'a, Message>) -> Self {
        self.label = Some(label);
        self
    }

    /// Sets the group action.
    pub fn action(mut self, action: SidebarGroupAction<'a, Message>) -> Self {
        self.action = Some(action);
        self
    }

    /// Sets the group content.
    pub fn content(mut self, content: SidebarGroupContent<'a, Message>) -> Self {
        self.content = Some(content);
        self
    }

    /// Appends an arbitrary child.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Builds the group.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let icon_mode = self.icon_mode;
        render::build_group(self, icon_mode)
    }
}

impl<'a, Message> From<SidebarGroup<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarGroup<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Group label (`Sidebar.GroupLabel`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarGroupLabel<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) text: String,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> fmt::Debug for SidebarGroupLabel<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarGroupLabel")
            .field("theme", &self.theme)
            .field("text", &self.text)
            .finish()
    }
}

impl<'a, Message> SidebarGroupLabel<'a, Message> {
    /// Creates a text label.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            theme,
            text: label.into_fragment().into_owned(),
            _message: std::marker::PhantomData,
        }
    }

    /// Builds the label.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_group_label(self, false)
    }
}

impl<'a, Message> From<SidebarGroupLabel<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarGroupLabel<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Group action button (`Sidebar.GroupAction`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarGroupAction<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) content: Option<Element<'a, Message>>,
    pub(super) on_press: Option<Message>,
}

impl<Message> fmt::Debug for SidebarGroupAction<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarGroupAction")
            .field("theme", &self.theme)
            .field("content", &self.content.is_some())
            .field("on_press", &self.on_press.is_some())
            .finish()
    }
}

impl<'a, Message> SidebarGroupAction<'a, Message> {
    /// Creates an action.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            content: None,
            on_press: None,
        }
    }

    /// Sets the action glyph / content.
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Sets the press message.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Builds the action.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_group_action(self, false)
    }
}

impl<'a, Message> From<SidebarGroupAction<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarGroupAction<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Group content wrapper (`Sidebar.GroupContent`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarGroupContent<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) children: Vec<Element<'a, Message>>,
}

impl<Message> fmt::Debug for SidebarGroupContent<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarGroupContent")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .finish()
    }
}

impl<'a, Message> SidebarGroupContent<'a, Message> {
    /// Creates empty content.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
        }
    }

    /// Appends a child.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Builds the content.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_group_content(self, false)
    }
}

impl<'a, Message> From<SidebarGroupContent<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarGroupContent<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Menu list (`Sidebar.Menu`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarMenu<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) children: Vec<Element<'a, Message>>,
}

impl<Message> fmt::Debug for SidebarMenu<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarMenu")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .finish()
    }
}

impl<'a, Message> SidebarMenu<'a, Message> {
    /// Creates an empty menu.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
        }
    }

    /// Appends a menu item.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Builds the menu.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_menu(self)
    }
}

impl<'a, Message> From<SidebarMenu<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarMenu<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Menu item row (`Sidebar.MenuItem`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarMenuItem<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) children: Vec<Element<'a, Message>>,
}

impl<Message> fmt::Debug for SidebarMenuItem<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarMenuItem")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .finish()
    }
}

impl<'a, Message> SidebarMenuItem<'a, Message> {
    /// Creates an empty item.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
        }
    }

    /// Appends a child (button / action / badge / sub-menu).
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Builds the item.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_menu_item(self)
    }
}

impl<'a, Message> From<SidebarMenuItem<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarMenuItem<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Menu button (`Sidebar.MenuButton`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarMenuButton<'a, Message> {
    pub(super) controller: &'a SidebarController,
    pub(super) theme: &'a Theme,
    pub(super) label: String,
    pub(super) subtitle: Option<String>,
    pub(super) variant: SidebarMenuButtonVariant,
    pub(super) size: SidebarMenuButtonSize,
    pub(super) collapsible: SidebarCollapsible,
    pub(super) active: bool,
    pub(super) disabled: bool,
    pub(super) tooltip: Option<String>,
    pub(super) leading_icon: Option<Element<'a, Message>>,
    pub(super) trailing_icon: Option<Element<'a, Message>>,
    pub(super) on_press: Option<Message>,
}

impl<Message> fmt::Debug for SidebarMenuButton<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarMenuButton")
            .field("controller", &self.controller)
            .field("theme", &self.theme)
            .field("label", &self.label)
            .field("subtitle", &self.subtitle)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("collapsible", &self.collapsible)
            .field("active", &self.active)
            .field("disabled", &self.disabled)
            .field("tooltip", &self.tooltip)
            .field("leading_icon", &self.leading_icon.is_some())
            .field("trailing_icon", &self.trailing_icon.is_some())
            .field("on_press", &self.on_press.is_some())
            .finish()
    }
}

impl<'a, Message> SidebarMenuButton<'a, Message> {
    /// Creates a text menu button.
    pub fn text(
        label: impl IntoFragment<'a>,
        controller: &'a SidebarController,
        theme: &'a Theme,
    ) -> Self {
        Self {
            controller,
            theme,
            label: label.into_fragment().into_owned(),
            subtitle: None,
            variant: SidebarMenuButtonVariant::Default,
            size: SidebarMenuButtonSize::Default,
            collapsible: SidebarCollapsible::Icon,
            active: false,
            disabled: false,
            tooltip: None,
            leading_icon: None,
            trailing_icon: None,
            on_press: None,
        }
    }

    /// Sets the visual variant.
    pub fn variant(mut self, variant: SidebarMenuButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets an optional second line under the label (`text-xs`, team/user rows).
    pub fn subtitle(mut self, subtitle: impl IntoFragment<'a>) -> Self {
        self.subtitle = Some(subtitle.into_fragment().into_owned());
        self
    }

    /// Sets the size.
    pub fn size(mut self, size: SidebarMenuButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the collapse mode used for icon-only rendering.
    pub fn collapsible(mut self, collapsible: SidebarCollapsible) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Marks the button as the active route.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Disables the button.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets a tooltip shown while the desktop sidebar is icon-collapsed.
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Sets a leading icon element.
    pub fn leading_icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    /// Sets a trailing icon element.
    pub fn trailing_icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    /// Sets the press message.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Builds the button.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_menu_button(self)
    }
}

impl<'a, Message> From<SidebarMenuButton<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarMenuButton<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Menu action (`Sidebar.MenuAction`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarMenuAction<'a, Message> {
    pub(super) controller: &'a SidebarController,
    pub(super) theme: &'a Theme,
    pub(super) content: Option<Element<'a, Message>>,
    pub(super) on_press: Option<Message>,
}

impl<Message> fmt::Debug for SidebarMenuAction<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarMenuAction")
            .field("controller", &self.controller)
            .field("theme", &self.theme)
            .field("content", &self.content.is_some())
            .field("on_press", &self.on_press.is_some())
            .finish()
    }
}

impl<'a, Message> SidebarMenuAction<'a, Message> {
    /// Creates an action.
    pub fn new(controller: &'a SidebarController, theme: &'a Theme) -> Self {
        Self {
            controller,
            theme,
            content: None,
            on_press: None,
        }
    }

    /// Sets the action content.
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Sets the press message.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Builds the action.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_menu_action(self)
    }
}

impl<'a, Message> From<SidebarMenuAction<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarMenuAction<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Menu badge (`Sidebar.MenuBadge`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarMenuBadge<'a, Message> {
    pub(super) controller: &'a SidebarController,
    pub(super) theme: &'a Theme,
    pub(super) text: String,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> fmt::Debug for SidebarMenuBadge<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarMenuBadge")
            .field("controller", &self.controller)
            .field("theme", &self.theme)
            .field("text", &self.text)
            .finish()
    }
}

impl<'a, Message> SidebarMenuBadge<'a, Message> {
    /// Creates a text badge.
    pub fn text(
        label: impl IntoFragment<'a>,
        controller: &'a SidebarController,
        theme: &'a Theme,
    ) -> Self {
        Self {
            controller,
            theme,
            text: label.into_fragment().into_owned(),
            _message: std::marker::PhantomData,
        }
    }

    /// Builds the badge.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_menu_badge(self)
    }
}

impl<'a, Message> From<SidebarMenuBadge<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarMenuBadge<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Loading placeholder row (`Sidebar.MenuSkeleton`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarMenuSkeleton<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) show_icon: bool,
    pub(super) width_percent: u16,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> fmt::Debug for SidebarMenuSkeleton<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarMenuSkeleton")
            .field("theme", &self.theme)
            .field("show_icon", &self.show_icon)
            .field("width_percent", &self.width_percent)
            .finish()
    }
}

impl<'a, Message> SidebarMenuSkeleton<'a, Message> {
    /// Creates a skeleton row.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            show_icon: false,
            width_percent: 70,
            _message: std::marker::PhantomData,
        }
    }

    /// Shows a leading icon placeholder.
    pub fn show_icon(mut self, show_icon: bool) -> Self {
        self.show_icon = show_icon;
        self
    }

    /// Sets the text bar width as a percent of a nominal 100px base.
    pub fn width_percent(mut self, width_percent: u16) -> Self {
        self.width_percent = width_percent;
        self
    }

    /// Builds the skeleton.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_menu_skeleton(self)
    }
}

impl<'a, Message> From<SidebarMenuSkeleton<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(value: SidebarMenuSkeleton<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Nested menu (`Sidebar.MenuSub`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarMenuSub<'a, Message> {
    pub(super) controller: &'a SidebarController,
    pub(super) theme: &'a Theme,
    pub(super) children: Vec<Element<'a, Message>>,
}

impl<Message> fmt::Debug for SidebarMenuSub<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarMenuSub")
            .field("controller", &self.controller)
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .finish()
    }
}

impl<'a, Message> SidebarMenuSub<'a, Message> {
    /// Creates an empty sub-menu.
    pub fn new(controller: &'a SidebarController, theme: &'a Theme) -> Self {
        Self {
            controller,
            theme,
            children: Vec::new(),
        }
    }

    /// Appends a sub-item.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Builds the sub-menu.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_menu_sub(self)
    }
}

impl<'a, Message> From<SidebarMenuSub<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarMenuSub<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Nested menu item (`Sidebar.MenuSubItem`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarMenuSubItem<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) children: Vec<Element<'a, Message>>,
}

impl<Message> fmt::Debug for SidebarMenuSubItem<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarMenuSubItem")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .finish()
    }
}

impl<'a, Message> SidebarMenuSubItem<'a, Message> {
    /// Creates an empty sub-item.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
        }
    }

    /// Appends a child.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Builds the sub-item.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_menu_sub_item(self)
    }
}

impl<'a, Message> From<SidebarMenuSubItem<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarMenuSubItem<'a, Message>) -> Self {
        value.into_element()
    }
}

/// Nested menu button (`Sidebar.MenuSubButton`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SidebarMenuSubButton<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) label: String,
    pub(super) size: SidebarMenuSubButtonSize,
    pub(super) active: bool,
    pub(super) disabled: bool,
    pub(super) on_press: Option<Message>,
}

impl<Message> fmt::Debug for SidebarMenuSubButton<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SidebarMenuSubButton")
            .field("theme", &self.theme)
            .field("label", &self.label)
            .field("size", &self.size)
            .field("active", &self.active)
            .field("disabled", &self.disabled)
            .field("on_press", &self.on_press.is_some())
            .finish()
    }
}

impl<'a, Message> SidebarMenuSubButton<'a, Message> {
    /// Creates a text sub-button.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            theme,
            label: label.into_fragment().into_owned(),
            size: SidebarMenuSubButtonSize::Md,
            active: false,
            disabled: false,
            on_press: None,
        }
    }

    /// Sets the size.
    pub fn size(mut self, size: SidebarMenuSubButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Marks the button active.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Disables the button.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the press message.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Builds the sub-button.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_menu_sub_button(self)
    }
}

impl<'a, Message> From<SidebarMenuSubButton<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: SidebarMenuSubButton<'a, Message>) -> Self {
        value.into_element()
    }
}

// Keep Length available for docs / future width helpers.
#[allow(dead_code)]
fn _length_marker(width: Length) -> Length {
    width
}
