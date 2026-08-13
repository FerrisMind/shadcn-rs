//! Builder-first command palette component.
//!
//! Port of the shadcn-svelte command (`Command.Root` / `Input` / `List` /
//! `Empty` / `Group` / `Item` / `Separator` / `Shortcut` / `Loading` /
//! `Dialog`) as iced builders. Filtering and highlight stepping live in
//! [`shadcn_common`] so egui can share them. The dialog variant composes
//! [`crate::Dialog`] with `p-0` padding and `top-1/3` placement.
//!
//! **Style packs:** unlike Form (identical `form.json` across packs), Command
//! ships distinct `.cn-command*` recipes per style — radii, input chrome,
//! item padding, separator bleed, etc. via
//! [`shadcn_common::command_recipe`] and `theme.style_id()`.
//!
//! Composed parts still follow the same [`Theme`]: [`CommandDialog`] → Dialog
//! recipe (with command radius override), loading row → [`crate::Spinner`],
//! list rules → [`crate::Separator`], search field → Input / InputGroup
//! builders. Picking Rhea on the theme therefore paints Rhea Command chrome
//! *and* Rhea Dialog / Spinner / Separator / Button trigger recipes — the
//! same composite rule as Form, plus Command’s own pack deltas.

mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use style::CommandStyle;
pub use types::{
    CommandEmpty, CommandEntry, CommandGlyph, CommandGroup, CommandItem, CommandLoading,
    CommandRadius,
};

use std::fmt;

use shadcn_common::{
    COMMAND_DIALOG_VERTICAL_ANCHOR, COMMAND_LIST_MAX_HEIGHT_PX, CommandFilter,
    default_command_filter,
};

use crate::components::dialog::{Dialog, DialogStyle};
use crate::iced_compat::{Element, Length};
use crate::theme::Theme;

/// Builder-first command palette styled from `shadcn-common` recipes.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Command, CommandGroup, CommandItem, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Query(String),
///     Run(&'static str),
/// }
///
/// fn palette<'a>(theme: &'a Theme, query: &'a str) -> Element<'a, Message> {
///     Command::new(theme)
///         .query(query)
///         .on_query_change(Message::Query)
///         .group(
///             CommandGroup::new("Suggestions")
///                 .item(CommandItem::new("calendar", "Calendar"))
///                 .item(CommandItem::new("emoji", "Search Emoji")),
///         )
///         .empty("No results found.")
///         .on_select(Message::Run)
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Command<'a, T, Message>
where
    T: Clone,
{
    theme: &'a Theme,
    query: &'a str,
    placeholder: String,
    empty: Option<CommandEmpty>,
    rows: Vec<CommandEntry<T>>,
    radius: Option<CommandRadius>,
    width: Length,
    max_height: f32,
    should_filter: bool,
    filter: CommandFilter,
    show_search_icon: bool,
    show_border: bool,
    show_shadow: bool,
    in_dialog: bool,
    loop_highlight: bool,
    highlighted: Option<usize>,
    on_query_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_select: Option<Box<dyn Fn(T) -> Message + 'a>>,
    on_highlight_change: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    style_override: Option<Box<dyn Fn(CommandStyle) -> CommandStyle + 'a>>,
    input_leading: Option<Element<'a, Message>>,
    input_trailing: Option<Element<'a, Message>>,
    /// Hit size of leading/trailing chrome controls; drives addon inset math.
    input_adornment_size: Option<f32>,
    input_id: Option<crate::iced_compat::widget::Id>,
}

impl<T, Message> fmt::Debug for Command<'_, T, Message>
where
    T: Clone + fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Command")
            .field("theme", &self.theme)
            .field("query", &self.query)
            .field("placeholder", &self.placeholder)
            .field("empty", &self.empty)
            .field("rows", &self.rows.len())
            .field("radius", &self.radius)
            .field("width", &self.width)
            .field("max_height", &self.max_height)
            .field("should_filter", &self.should_filter)
            .field("show_search_icon", &self.show_search_icon)
            .field("show_border", &self.show_border)
            .field("show_shadow", &self.show_shadow)
            .field("in_dialog", &self.in_dialog)
            .field("loop_highlight", &self.loop_highlight)
            .field("highlighted", &self.highlighted)
            .field("on_query_change", &self.on_query_change.is_some())
            .field("on_select", &self.on_select.is_some())
            .field("on_highlight_change", &self.on_highlight_change.is_some())
            .field("style_override", &self.style_override.is_some())
            .field("input_leading", &self.input_leading.is_some())
            .field("input_trailing", &self.input_trailing.is_some())
            .field("input_adornment_size", &self.input_adornment_size)
            .finish()
    }
}

impl<'a, T, Message> Command<'a, T, Message>
where
    T: Clone,
{
    /// Creates an empty command palette.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            query: "",
            placeholder: "Type a command or search...".to_owned(),
            empty: None,
            rows: Vec::new(),
            radius: None,
            width: Length::Fill,
            max_height: COMMAND_LIST_MAX_HEIGHT_PX,
            should_filter: true,
            filter: default_command_filter,
            show_search_icon: true,
            show_border: true,
            show_shadow: true,
            in_dialog: false,
            loop_highlight: true,
            highlighted: None,
            on_query_change: None,
            on_select: None,
            on_highlight_change: None,
            style_override: None,
            input_leading: None,
            input_trailing: None,
            input_adornment_size: None,
            input_id: None,
        }
    }

    /// Sets the controlled search query (`bind:value`).
    pub fn query(mut self, query: &'a str) -> Self {
        self.query = query;
        self
    }

    /// Sets the input placeholder.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Sets the empty-state copy (`Command.Empty`).
    pub fn empty(mut self, empty: impl Into<CommandEmpty>) -> Self {
        self.empty = Some(empty.into());
        self
    }

    /// Appends a selectable item.
    pub fn item(mut self, item: impl Into<CommandItem<T>>) -> Self {
        self.rows.push(CommandEntry::Item(item.into()));
        self
    }

    /// Appends a labelled group.
    pub fn group(mut self, group: CommandGroup<T>) -> Self {
        self.rows.push(CommandEntry::Group(group));
        self
    }

    /// Appends a separator.
    pub fn separator(mut self) -> Self {
        self.rows
            .push(CommandEntry::Separator { force_mount: false });
        self
    }

    /// Appends a force-mounted separator.
    pub fn separator_force_mount(mut self) -> Self {
        self.rows
            .push(CommandEntry::Separator { force_mount: true });
        self
    }

    /// Appends a loading row.
    pub fn loading(mut self, loading: CommandLoading) -> Self {
        self.rows.push(CommandEntry::Loading(loading));
        self
    }

    /// Appends an arbitrary entry.
    pub fn entry(mut self, entry: CommandEntry<T>) -> Self {
        self.rows.push(entry);
        self
    }

    /// Overrides the surface corner radius.
    pub fn radius(mut self, radius: CommandRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Sets the command width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the list max height (`max-h-72` by default).
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height.max(1.0);
        self
    }

    /// Enables or disables client-side filtering.
    pub fn should_filter(mut self, should_filter: bool) -> Self {
        self.should_filter = should_filter;
        self
    }

    /// Replaces the default fuzzy filter.
    pub fn filter(mut self, filter: CommandFilter) -> Self {
        self.filter = filter;
        self
    }

    /// Shows or hides the leading search glyph.
    pub fn show_search_icon(mut self, show: bool) -> Self {
        self.show_search_icon = show;
        self
    }

    /// Shows or hides the surface hairline.
    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Shows or hides the surface shadow.
    pub fn show_shadow(mut self, show: bool) -> Self {
        self.show_shadow = show;
        self
    }

    /// Applies dialog item-radius tokens (`in-data-[slot=dialog-content]`).
    pub fn in_dialog(mut self, in_dialog: bool) -> Self {
        self.in_dialog = in_dialog;
        self
    }

    /// Enables wrapping when arrowing past the ends.
    pub fn loop_highlight(mut self, looping: bool) -> Self {
        self.loop_highlight = looping;
        self
    }

    /// Controls the highlighted selectable index.
    pub fn highlighted(mut self, index: usize) -> Self {
        self.highlighted = Some(index);
        self
    }

    /// Controls the highlighted index when `Some`.
    pub fn highlighted_maybe(mut self, index: Option<usize>) -> Self {
        self.highlighted = index;
        self
    }

    /// Notifies when the search query changes.
    pub fn on_query_change(mut self, on_query_change: impl Fn(String) -> Message + 'a) -> Self {
        self.on_query_change = Some(Box::new(on_query_change));
        self
    }

    /// Notifies when an item is activated.
    pub fn on_select(mut self, on_select: impl Fn(T) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// Notifies when the highlight moves.
    pub fn on_highlight_change(
        mut self,
        on_highlight_change: impl Fn(usize) -> Message + 'a,
    ) -> Self {
        self.on_highlight_change = Some(Box::new(on_highlight_change));
        self
    }

    /// Patches the resolved [`CommandStyle`] after theme resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(CommandStyle) -> CommandStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Places an arbitrary element at the start of the command input row.
    ///
    /// When set, this replaces the default search glyph from
    /// [`Self::show_search_icon`].
    pub fn input_leading(mut self, leading: impl Into<Element<'a, Message>>) -> Self {
        self.input_leading = Some(leading.into());
        self
    }

    /// Places an arbitrary element at the end of the command input row.
    pub fn input_trailing(mut self, trailing: impl Into<Element<'a, Message>>) -> Self {
        self.input_trailing = Some(trailing.into());
        self
    }

    /// Sets the leading/trailing control size used for addon inset math.
    ///
    /// Defaults to the pack `control_height_sm` when unset.
    pub fn input_adornment_size(mut self, size: f32) -> Self {
        self.input_adornment_size = Some(size.max(1.0));
        self
    }

    /// Sets the search field widget id for focus and selection operations.
    pub fn input_id(mut self, id: impl Into<crate::iced_compat::widget::Id>) -> Self {
        self.input_id = Some(id.into());
        self
    }
}

impl<'a, T, Message> From<Command<'a, T, Message>> for Element<'a, Message>
where
    T: Clone + 'a,
    Message: Clone + 'a,
{
    fn from(command: Command<'a, T, Message>) -> Self {
        render::build(
            command.theme,
            command.query,
            command.placeholder,
            command.empty,
            command.rows,
            command.radius,
            command.width,
            command.max_height,
            command.should_filter,
            command.filter,
            command.show_search_icon,
            command.show_border,
            command.show_shadow,
            command.in_dialog,
            command.loop_highlight,
            command.highlighted,
            command.on_query_change,
            command.on_select,
            command.on_highlight_change,
            command.style_override,
            command.input_leading,
            command.input_trailing,
            command.input_adornment_size,
            command.input_id,
        )
    }
}

/// Convenience constructor mirroring [`Command::new`].
pub fn command<'a, T, Message>(theme: &'a Theme) -> Command<'a, T, Message>
where
    T: Clone,
{
    Command::new(theme)
}

/// Command palette hosted inside a dialog (`Command.Dialog`).
///
/// Mirrors shadcn-svelte: `p-0` content, `top-1/3` placement, sr-only title /
/// description, and `showCloseButton = false` by default.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct CommandDialog<'a, T, Message>
where
    T: Clone,
{
    trigger: Element<'a, Message>,
    command: Command<'a, T, Message>,
    theme: &'a Theme,
    title: String,
    description: String,
    open: Option<bool>,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    show_close_button: bool,
}

impl<T, Message> fmt::Debug for CommandDialog<'_, T, Message>
where
    T: Clone + fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CommandDialog")
            .field("command", &self.command)
            .field("theme", &self.theme)
            .field("title", &self.title)
            .field("description", &self.description)
            .field("open", &self.open)
            .field("on_open_change", &self.on_open_change.is_some())
            .field("show_close_button", &self.show_close_button)
            .finish_non_exhaustive()
    }
}

impl<'a, T, Message> CommandDialog<'a, T, Message>
where
    T: Clone,
{
    /// Creates a command dialog opening `command` from `trigger`.
    pub fn new(
        trigger: impl Into<Element<'a, Message>>,
        command: Command<'a, T, Message>,
        theme: &'a Theme,
    ) -> Self {
        Self {
            trigger: trigger.into(),
            command: command
                .in_dialog(true)
                .show_border(false)
                .show_shadow(false),
            theme,
            title: "Command Palette".to_owned(),
            description: "Search for a command to run...".to_owned(),
            open: None,
            on_open_change: None,
            show_close_button: false,
        }
    }

    /// Sets the sr-only dialog title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets the sr-only dialog description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Controls the open state (`bind:open`).
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Controls the open state when `Some`.
    pub fn open_maybe(mut self, open: Option<bool>) -> Self {
        self.open = open;
        self
    }

    /// Notifies about open-state change requests.
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// Shows or hides the dialog close button (`showCloseButton`).
    pub fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }
}

impl<'a, T, Message> From<CommandDialog<'a, T, Message>> for Element<'a, Message>
where
    T: Clone + 'a,
    Message: Clone + 'a,
{
    fn from(dialog: CommandDialog<'a, T, Message>) -> Self {
        use crate::iced_compat::widget::{Space, column};
        use crate::recipes::component_radius_px;
        use shadcn_common::command_recipe;

        // Keep title/description in the tree for a11y parity with the web
        // `sr-only` Dialog.Header; they occupy no visual space.
        let content: Element<'a, Message> = column![
            Space::new()
                .width(Length::Fixed(0.0))
                .height(Length::Fixed(0.0)),
            dialog.command,
        ]
        .into();
        let _ = (dialog.title, dialog.description);

        // `.cn-command-dialog` overrides Dialog.Content radius to match
        // `.cn-command` (e.g. Rhea `rounded-3xl!` vs dialog `min(4xl,24)`).
        let command_radius = command_recipe(dialog.theme.style_id()).radius;
        let theme = dialog.theme;
        let surface_radius = component_radius_px(theme, command_radius);

        let mut builder = Dialog::new(dialog.trigger, content, theme)
            .content_padding(0.0)
            .vertical_anchor_top(COMMAND_DIALOG_VERTICAL_ANCHOR)
            .show_close_button(dialog.show_close_button)
            .style_override(move |style| DialogStyle {
                radius: surface_radius,
                ..style
            });

        if let Some(open) = dialog.open {
            builder = builder.open(open);
        }
        if let Some(on_open_change) = dialog.on_open_change {
            builder = builder.on_open_change(on_open_change);
        }

        builder.into()
    }
}

/// Convenience constructor mirroring [`CommandDialog::new`].
pub fn command_dialog<'a, T, Message>(
    trigger: impl Into<Element<'a, Message>>,
    command: Command<'a, T, Message>,
    theme: &'a Theme,
) -> CommandDialog<'a, T, Message>
where
    T: Clone,
{
    CommandDialog::new(trigger, command, theme)
}
