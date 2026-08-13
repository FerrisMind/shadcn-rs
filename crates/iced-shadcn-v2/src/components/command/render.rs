//! Element construction and keyboard shell for [`super::Command`].

use std::cell::Cell;
use std::rc::Rc;

use shadcn_common::{
    COMMAND_DISABLED_OPACITY, COMMAND_INPUT_ICON_OPACITY, CommandFilter, Direction, NavAction,
    NavKey, Orientation, command_matches, default_command_filter, first_selectable_index,
    last_selectable_index, resolve_nav_action, step_selectable_index,
};

use iced_core::keyboard;

use crate::components::input::Input;
use crate::components::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign};
use crate::components::scroll_area::{ScrollArea, ScrollAreaOrientation, ScrollAreaScrollbar};
use crate::components::separator::Separator;
use crate::components::spinner::{Spinner, SpinnerSize};
use crate::fonts::iced_font;
use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Operation, Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, layout, overlay, renderer};
use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::text::{self as text_style, LineHeight};
use crate::iced_compat::widget::{Space, button, column, container, row, text};
use crate::iced_compat::{
    Background, Border, Color, Element, Event, Length, Padding, Pixels, Point, Rectangle, Renderer,
    Shadow, Size, Theme as IcedTheme, Vector, alignment, border, mouse, touch,
};
use crate::recipes::iced_font_weight;
use crate::theme::Theme;

use super::style::{self, CommandStyle, item_background, item_foreground, typography_size};
use super::types::{
    CommandEmpty, CommandEntry, CommandGlyph, CommandItem, CommandLoading, CommandRadius,
};

/// Builds the command widget.
#[allow(clippy::too_many_arguments)]
pub(super) fn build<'a, T, Message>(
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
    input_adornment_size: Option<f32>,
    input_id: Option<crate::iced_compat::widget::Id>,
) -> Element<'a, Message>
where
    T: Clone + 'a,
    Message: Clone + 'a,
{
    let recipe = style::recipe(theme);
    let mut resolved = style::resolve_style(theme, radius, in_dialog, show_border, show_shadow);
    if let Some(style_override) = style_override.as_ref() {
        resolved = style_override(resolved);
    }

    let input = build_input(
        theme,
        query,
        &placeholder,
        show_search_icon,
        resolved,
        recipe,
        on_query_change,
        input_leading,
        input_trailing,
        input_adornment_size,
        input_id,
    );

    Element::new(CommandWidget {
        theme,
        input,
        query: query.to_owned(),
        empty,
        rows,
        style: resolved,
        recipe,
        width,
        max_height,
        should_filter,
        filter,
        loop_highlight,
        highlighted,
        on_select,
        on_highlight_change,
    })
}

struct CommandWidget<'a, T, Message> {
    theme: &'a Theme,
    input: Element<'a, Message>,
    query: String,
    empty: Option<CommandEmpty>,
    rows: Vec<CommandEntry<T>>,
    style: CommandStyle,
    recipe: shadcn_common::CommandRecipe,
    width: Length,
    max_height: f32,
    should_filter: bool,
    filter: CommandFilter,
    loop_highlight: bool,
    highlighted: Option<usize>,
    on_select: Option<Box<dyn Fn(T) -> Message + 'a>>,
    on_highlight_change: Option<Box<dyn Fn(usize) -> Message + 'a>>,
}

#[derive(Debug)]
struct State {
    highlighted: Option<usize>,
    /// Written by item rows on pointer hover; drained in [`CommandWidget::update`].
    hover_bus: Rc<Cell<Option<usize>>>,
    /// Last fitted list viewport (`min(content, max-h-72)`).
    list_viewport_h: f32,
}

impl State {
    fn new(highlighted: Option<usize>, max_height: f32) -> Self {
        Self {
            highlighted,
            hover_bus: Rc::new(Cell::new(None)),
            list_viewport_h: max_height.max(1.0),
        }
    }
}

struct FlatItem<'a, T> {
    value: &'a T,
    label: &'a str,
    keywords: &'a [String],
    disabled: bool,
    force_mount: bool,
}

fn flatten<'a, T>(entries: &'a [CommandEntry<T>], out: &mut Vec<FlatItem<'a, T>>) {
    for entry in entries {
        match entry {
            CommandEntry::Item(item) => out.push(FlatItem {
                value: &item.value,
                label: &item.label,
                keywords: &item.keywords,
                disabled: item.disabled,
                force_mount: item.force_mount,
            }),
            CommandEntry::Group(group) => flatten(&group.entries, out),
            CommandEntry::Separator { .. } | CommandEntry::Loading(_) => {}
        }
    }
}

fn visible_flags(
    items: &[FlatItem<'_, impl Clone>],
    query: &str,
    should_filter: bool,
    filter: CommandFilter,
) -> Vec<bool> {
    items
        .iter()
        .map(|item| {
            let kw: Vec<&str> = item.keywords.iter().map(String::as_str).collect();
            item.force_mount || command_matches(query, item.label, &kw, should_filter, filter)
        })
        .collect()
}

impl<'a, T, Message> CommandWidget<'a, T, Message>
where
    T: Clone + 'a,
    Message: Clone + 'a,
{
    fn enabled_flags(&self) -> (Vec<FlatItem<'_, T>>, Vec<bool>, Vec<bool>) {
        let mut items = Vec::new();
        flatten(&self.rows, &mut items);
        let visible = visible_flags(&items, &self.query, self.should_filter, self.filter);
        let enabled: Vec<bool> = items
            .iter()
            .zip(visible.iter())
            .map(|(item, vis)| *vis && !item.disabled)
            .collect();
        (items, visible, enabled)
    }

    /// Builds the list body column (no scrollport) — used to measure intrinsic height.
    fn build_list_body(
        &self,
        highlighted: Option<usize>,
        hover_bus: Rc<Cell<Option<usize>>>,
    ) -> Element<'a, Message> {
        let mut flat = Vec::new();
        flatten(&self.rows, &mut flat);
        let visible = visible_flags(&flat, &self.query, self.should_filter, self.filter);
        let visible_count = visible.iter().filter(|v| **v).count();

        let mut children = Vec::new();
        let mut selectable_index = 0usize;
        push_entries(
            self.theme,
            &self.rows,
            &visible,
            &mut selectable_index,
            highlighted,
            self.style,
            self.recipe,
            self.on_select.as_deref(),
            hover_bus,
            &mut children,
        );

        if let Some(empty) = &self.empty
            && (empty.force_mount || visible_count == 0)
        {
            children.push(build_empty(self.theme, empty, self.style, self.recipe));
        }

        // `.cn-command-list`: `scroll-py-1`
        let scroll_py = self.recipe.list_scroll_pad_y_px;
        column(children)
            .width(Length::Fill)
            .padding(Padding {
                top: scroll_py,
                bottom: scroll_py,
                left: 0.0,
                right: 0.0,
            })
            .into()
    }

    /// `.cn-command-list`: `max-h-72 overflow-y-auto` — viewport is the
    /// measured content height capped at [`Self::max_height`].
    fn build_list(
        &self,
        highlighted: Option<usize>,
        hover_bus: Rc<Cell<Option<usize>>>,
        viewport_h: f32,
    ) -> Element<'a, Message> {
        ScrollArea::new(self.build_list_body(highlighted, hover_bus), self.theme)
            .orientation(ScrollAreaOrientation::Vertical)
            .width(Length::Fill)
            .height(Length::Fixed(viewport_h.max(1.0)))
            // `.cn-command-list`: `no-scrollbar`
            .vertical_scrollbar(ScrollAreaScrollbar::hidden())
            .bordered(false)
            .into()
    }
}

/// Extra outer-edge inset for chrome adornments relative to vertical pad.
const ADORNMENT_INLINE_EXTRA_PX: f32 = 2.0;

#[allow(clippy::too_many_arguments)]
fn build_input<'a, Message: Clone + 'a>(
    theme: &'a Theme,
    query: &'a str,
    placeholder: &str,
    show_search_icon: bool,
    style: CommandStyle,
    recipe: shadcn_common::CommandRecipe,
    on_query_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    input_leading: Option<Element<'a, Message>>,
    input_trailing: Option<Element<'a, Message>>,
    input_adornment_size: Option<f32>,
    input_id: Option<crate::iced_compat::widget::Id>,
) -> Element<'a, Message> {
    let mut input = Input::new(theme)
        .value(query)
        .placeholder(placeholder.to_owned())
        .width(Length::Fill);
    if let Some(id) = input_id {
        input = input.id(id);
    }
    if let Some(on_input) = on_query_change {
        input = input.on_input(on_input);
    }

    let fill = style.input_background;
    let border_color = style.input_border;
    let radius = style.input_radius;
    let underlined = recipe.input_underline_only;
    let bordered = recipe.input_bordered;

    let adornment_size = input_adornment_size.unwrap_or(theme.style.control_height_sm_px);

    // Keep InputGroup chrome fully transparent so command owns a single
    // surface (avoids pack-default radius stacking under command radius).
    let mut group = InputGroup::new(theme)
        .height(Length::Fixed(recipe.input_height_px))
        .style_override(|mut resolved| {
            resolved.background = Some(Background::Color(Color::TRANSPARENT));
            resolved.border = Border::default();
            resolved.shadow = Shadow::default();
            resolved
        });

    if let Some(leading) = input_leading {
        group = group.push(
            InputGroupAddon::empty(theme)
                .align(InputGroupAddonAlign::InlineStart)
                .padding_uniform_around_child_extra_inline(
                    adornment_size,
                    recipe.input_height_px,
                    ADORNMENT_INLINE_EXTRA_PX,
                )
                .push(leading),
        );
    } else if show_search_icon {
        group = group.push(
            InputGroupAddon::empty(theme)
                .align(InputGroupAddonAlign::InlineStart)
                .push(glyph_canvas(
                    CommandGlyph::Search,
                    recipe.input_icon_size_px,
                    Color {
                        a: style.muted_foreground.a * COMMAND_INPUT_ICON_OPACITY,
                        ..style.muted_foreground
                    },
                )),
        );
    }
    group = group.push(input);
    if let Some(trailing) = input_trailing {
        group = group.push(
            InputGroupAddon::empty(theme)
                .align(InputGroupAddonAlign::InlineEnd)
                .padding_uniform_around_child_extra_inline(
                    adornment_size,
                    recipe.input_height_px,
                    ADORNMENT_INLINE_EXTRA_PX,
                )
                .push(trailing),
        );
    }

    let mut pad = Padding::new(recipe.input_wrapper_pad_px);
    if recipe.input_wrapper_border_bottom {
        // Hairline is drawn below the wrapper; avoid double gap under the field.
        pad.bottom = 0.0;
    }

    let field = container(group)
        .width(Length::Fill)
        .height(Length::Fixed(recipe.input_height_px))
        .style(move |_| container::Style {
            background: Some(Background::Color(fill)),
            border: Border {
                color: if underlined || !bordered {
                    Color::TRANSPARENT
                } else {
                    border_color
                },
                width: if bordered && !underlined { 1.0 } else { 0.0 },
                radius: border::radius(radius),
            },
            shadow: Shadow::default(),
            ..container::Style::default()
        });

    let wrapper = container(field).padding(pad).width(Length::Fill);

    if recipe.input_wrapper_border_bottom || underlined {
        let hairline = style.border;
        return column![
            wrapper,
            container(Space::new().width(Length::Fill).height(Length::Fixed(1.0))).style(
                move |_| container::Style {
                    background: Some(Background::Color(hairline)),
                    ..container::Style::default()
                }
            )
        ]
        .width(Length::Fill)
        .into();
    }

    wrapper.into()
}

#[allow(clippy::too_many_arguments)]
fn push_entries<'a, T, Message>(
    theme: &'a Theme,
    entries: &[CommandEntry<T>],
    visibility: &[bool],
    selectable_index: &mut usize,
    highlighted: Option<usize>,
    style: CommandStyle,
    recipe: shadcn_common::CommandRecipe,
    on_select: Option<&(dyn Fn(T) -> Message + 'a)>,
    hover_bus: Rc<Cell<Option<usize>>>,
    out: &mut Vec<Element<'a, Message>>,
) where
    T: Clone + 'a,
    Message: Clone + 'a,
{
    for entry in entries {
        match entry {
            CommandEntry::Item(item) => {
                let index = *selectable_index;
                *selectable_index += 1;
                let visible = visibility.get(index).copied().unwrap_or(true);
                if !visible && !item.force_mount {
                    continue;
                }
                out.push(build_item(
                    theme,
                    item,
                    index,
                    highlighted == Some(index),
                    style,
                    recipe,
                    on_select.map(|f| f(item.value.clone())),
                    Rc::clone(&hover_bus),
                ));
            }
            CommandEntry::Group(group) => {
                let start = *selectable_index;
                let mut peek = *selectable_index;
                let mut any = false;
                peek_group_visibility(&group.entries, visibility, &mut peek, &mut any);
                if !any && !group.force_mount {
                    *selectable_index = peek;
                    continue;
                }
                let _ = start;
                if let Some(heading) = &group.heading {
                    out.push(build_heading(theme, heading, style, recipe));
                }
                let mut nested = Vec::new();
                push_entries(
                    theme,
                    &group.entries,
                    visibility,
                    selectable_index,
                    highlighted,
                    style,
                    recipe,
                    on_select,
                    Rc::clone(&hover_bus),
                    &mut nested,
                );
                out.push(
                    container(column(nested).width(Length::Fill))
                        .padding(recipe.group_pad_px)
                        .width(Length::Fill)
                        .into(),
                );
            }
            CommandEntry::Separator { force_mount } => {
                if *force_mount || visibility.iter().any(|v| *v) {
                    out.push(build_separator(theme, style, recipe));
                }
            }
            CommandEntry::Loading(loading) => {
                out.push(build_loading(theme, loading, style, recipe));
            }
        }
    }
}

fn peek_group_visibility<T>(
    entries: &[CommandEntry<T>],
    visibility: &[bool],
    selectable_index: &mut usize,
    any: &mut bool,
) {
    for entry in entries {
        match entry {
            CommandEntry::Item(item) => {
                let visible = visibility.get(*selectable_index).copied().unwrap_or(true);
                *selectable_index += 1;
                if visible || item.force_mount {
                    *any = true;
                }
            }
            CommandEntry::Group(group) => {
                let mut nested = false;
                peek_group_visibility(&group.entries, visibility, selectable_index, &mut nested);
                if nested || group.force_mount {
                    *any = true;
                }
            }
            CommandEntry::Separator { .. } | CommandEntry::Loading(_) => {}
        }
    }
}

fn build_heading<'a, Message: 'a>(
    theme: &'a Theme,
    heading: &str,
    style: CommandStyle,
    recipe: shadcn_common::CommandRecipe,
) -> Element<'a, Message> {
    let ty = recipe.heading_typography;
    let mut label = heading.to_owned();
    if ty.uppercase {
        label = label.to_uppercase();
    }
    let color = style.muted_foreground;
    let mut font = iced_font(theme.font_pack().sans);
    font.weight = iced_font_weight(ty.weight);
    container(
        text(label)
            .size(Pixels(typography_size(ty)))
            .line_height(LineHeight::Absolute(Pixels(ty.line_height_px)))
            .font(font)
            .style(move |_| text_style::Style { color: Some(color) }),
    )
    .padding(Padding {
        top: recipe.heading_pad_y_px,
        bottom: recipe.heading_pad_y_px,
        left: recipe.heading_pad_x_px,
        right: recipe.heading_pad_x_px,
    })
    .width(Length::Fill)
    .into()
}

#[allow(clippy::too_many_arguments)]
fn build_item<'a, T, Message: Clone + 'a>(
    theme: &'a Theme,
    item: &CommandItem<T>,
    index: usize,
    selected: bool,
    style: CommandStyle,
    recipe: shadcn_common::CommandRecipe,
    on_press: Option<Message>,
    hover_bus: Rc<Cell<Option<usize>>>,
) -> Element<'a, Message> {
    let ty = recipe.item_typography;
    let fg_idle = item_foreground(style, false, item.disabled);
    let fg_hot = item_foreground(style, true, item.disabled);
    let bg_idle = item_background(style, false, item.disabled);
    let bg_hot = item_background(style, true, item.disabled);
    let muted_idle = {
        let mut muted = style.muted_foreground;
        if item.disabled {
            muted.a *= COMMAND_DISABLED_OPACITY;
        }
        muted
    };
    let muted_hot = {
        let mut muted = style.selected_foreground;
        if item.disabled {
            muted.a *= COMMAND_DISABLED_OPACITY;
        }
        muted
    };

    let mut content = row![]
        .spacing(recipe.item_gap_px)
        .align_y(alignment::Vertical::Center)
        .width(Length::Fill);

    if item.leading_check {
        content = content.push(glyph_canvas(
            CommandGlyph::Check,
            recipe.item_icon_size_px,
            if item.checked {
                if selected { fg_hot } else { fg_idle }
            } else {
                Color::TRANSPARENT
            },
        ));
    }

    if let Some(icon) = item.icon {
        content = content.push(glyph_canvas(
            icon,
            recipe.item_icon_size_px,
            if selected { fg_hot } else { fg_idle },
        ));
    }

    let mut label = item.label.clone();
    if ty.uppercase {
        label = label.to_uppercase();
    }
    let mut font = iced_font(theme.font_pack().sans);
    font.weight = iced_font_weight(ty.weight);
    let label_color = if selected { fg_hot } else { fg_idle };
    let label = text(label)
        .size(Pixels(typography_size(ty)))
        .line_height(LineHeight::Absolute(Pixels(ty.line_height_px)))
        .font(font)
        .style(move |_| text_style::Style {
            color: Some(label_color),
        });

    if let Some(description) = &item.description {
        let description_style = recipe.shortcut_typography;
        let description_color = if selected { muted_hot } else { muted_idle };
        content = content.push(
            column![
                label,
                text(description.clone())
                    .size(Pixels(typography_size(description_style)))
                    .line_height(LineHeight::Absolute(Pixels(
                        description_style.line_height_px,
                    )))
                    .font(iced_font(theme.font_pack().sans))
                    .style(move |_| text_style::Style {
                        color: Some(description_color),
                    }),
            ]
            .spacing(0)
            .width(Length::Fill),
        );
    } else {
        content = content.push(label);
    }
    content = content.push(Space::new().width(Length::Fill));

    if let Some(shortcut) = &item.shortcut {
        let sty = recipe.shortcut_typography;
        let shortcut_color = if selected { muted_hot } else { muted_idle };
        content = content.push(
            text(shortcut.clone())
                .size(Pixels(typography_size(sty)))
                .line_height(LineHeight::Absolute(Pixels(sty.line_height_px)))
                .style(move |_| text_style::Style {
                    color: Some(shortcut_color),
                }),
        );
    } else if item.checked && !item.leading_check {
        content = content.push(glyph_canvas(
            CommandGlyph::Check,
            recipe.item_icon_size_px,
            if selected { fg_hot } else { fg_idle },
        ));
    }

    let description_height = item
        .description
        .as_ref()
        .map(|_| recipe.shortcut_typography.line_height_px)
        .unwrap_or(0.0);
    let min_h = recipe
        .item_min_height_px
        .unwrap_or(ty.line_height_px + description_height + recipe.item_pad_y_px * 2.0)
        .max(ty.line_height_px + description_height + recipe.item_pad_y_px * 2.0);

    let padded = container(content)
        .padding(Padding {
            top: recipe.item_pad_y_px,
            bottom: recipe.item_pad_y_px,
            left: recipe.item_pad_x_px,
            right: recipe.item_pad_x_px,
        })
        .width(Length::Fill)
        .height(Length::Fixed(min_h));

    let item_radius = style.item_radius;
    let disabled = item.disabled;
    let mut widget = button(padded)
        .padding(0)
        .width(Length::Fill)
        .style(move |_, status| {
            let hot = !disabled
                && (selected
                    || matches!(status, button::Status::Hovered | button::Status::Pressed));
            button::Style {
                background: Some(Background::Color(if hot { bg_hot } else { bg_idle })),
                text_color: if hot { fg_hot } else { fg_idle },
                border: Border {
                    radius: border::radius(item_radius),
                    ..Border::default()
                },
                ..button::Style::default()
            }
        });
    if let Some(message) = on_press.filter(|_| !item.disabled) {
        widget = widget.on_press(message);
    }

    Element::new(HoverItem {
        content: widget.into(),
        index,
        disabled: item.disabled,
        hover_bus,
    })
}

/// Forwards pointer hover into the command highlight bus (cmdk `data-selected`).
struct HoverItem<'a, Message> {
    content: Element<'a, Message>,
    index: usize,
    disabled: bool,
    hover_bus: Rc<Cell<Option<usize>>>,
}

impl<Message> Widget<Message, IcedTheme, Renderer> for HoverItem<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if self.disabled {
            return;
        }

        if matches!(
            event,
            Event::Mouse(mouse::Event::CursorMoved { .. })
                | Event::Mouse(mouse::Event::CursorEntered)
                | Event::Touch(touch::Event::FingerMoved { .. })
        ) && cursor.is_over(layout.bounds())
        {
            self.hover_bus.set(Some(self.index));
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &IcedTheme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }
}

fn build_separator<'a, Message: 'a>(
    theme: &'a Theme,
    _style: CommandStyle,
    recipe: shadcn_common::CommandRecipe,
) -> Element<'a, Message> {
    container(Separator::new(theme))
        .padding(Padding {
            top: recipe.separator_margin_y_px,
            bottom: recipe.separator_margin_y_px,
            left: recipe.separator_margin_x_px,
            right: recipe.separator_margin_x_px,
        })
        .width(Length::Fill)
        .into()
}

fn build_empty<'a, Message: 'a>(
    theme: &'a Theme,
    empty: &CommandEmpty,
    style: CommandStyle,
    recipe: shadcn_common::CommandRecipe,
) -> Element<'a, Message> {
    let ty = recipe.empty_typography;
    let color = style.muted_foreground;
    let mut font = iced_font(theme.font_pack().sans);
    font.weight = iced_font_weight(ty.weight);
    container(
        text(empty.text.clone())
            .size(Pixels(typography_size(ty)))
            .line_height(LineHeight::Absolute(Pixels(ty.line_height_px)))
            .font(font)
            .style(move |_| text_style::Style { color: Some(color) }),
    )
    .padding(Padding {
        top: recipe.empty_pad_y_px,
        bottom: recipe.empty_pad_y_px,
        left: 0.0,
        right: 0.0,
    })
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .into()
}

fn build_loading<'a, Message: 'a>(
    theme: &'a Theme,
    loading: &CommandLoading,
    style: CommandStyle,
    recipe: shadcn_common::CommandRecipe,
) -> Element<'a, Message> {
    let color = style.muted_foreground;
    let ty = recipe.item_typography;
    container(
        row![
            Spinner::new(theme).size(SpinnerSize::Sm),
            text(loading.label.clone())
                .size(Pixels(typography_size(ty)))
                .style(move |_| text_style::Style { color: Some(color) }),
        ]
        .spacing(recipe.item_gap_px)
        .align_y(alignment::Vertical::Center),
    )
    .padding(Padding {
        top: recipe.item_pad_y_px,
        bottom: recipe.item_pad_y_px,
        left: recipe.item_pad_x_px,
        right: recipe.item_pad_x_px,
    })
    .width(Length::Fill)
    .into()
}

fn glyph_canvas<'a, Message: 'a>(
    glyph: CommandGlyph,
    size: f32,
    color: Color,
) -> Element<'a, Message> {
    canvas(GlyphPainter { glyph, size, color })
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .into()
}

#[derive(Debug)]
struct GlyphPainter {
    glyph: CommandGlyph,
    size: f32,
    color: Color,
}

impl<Message> canvas::Program<Message> for GlyphPainter {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &IcedTheme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let size = self.size.min(bounds.width).min(bounds.height);
        let mut frame = canvas::Frame::new(renderer, Size::new(size, size));
        let stroke = canvas::Stroke::default()
            .with_width((size * 0.12).clamp(1.0, 2.0))
            .with_color(self.color)
            .with_line_cap(canvas::LineCap::Round)
            .with_line_join(canvas::LineJoin::Round);

        match self.glyph {
            CommandGlyph::Search => {
                let cx = size * 0.42;
                let cy = size * 0.42;
                let r = size * 0.28;
                frame.stroke(&canvas::Path::circle(Point::new(cx, cy), r), stroke);
                frame.stroke(
                    &canvas::Path::line(
                        Point::new(cx + r * 0.72, cy + r * 0.72),
                        Point::new(size * 0.82, size * 0.82),
                    ),
                    stroke,
                );
            }
            CommandGlyph::Check => {
                frame.translate(Vector::new(size / 2.0, size / 2.0));
                frame.stroke(
                    &canvas::Path::new(|b| {
                        b.move_to(Point::new(-size * 0.28, 0.0));
                        b.line_to(Point::new(-size * 0.06, size * 0.22));
                        b.line_to(Point::new(size * 0.30, -size * 0.24));
                    }),
                    stroke,
                );
            }
            _ => {
                // Generic rounded square mark for remaining glyphs.
                let inset = size * 0.18;
                frame.stroke(
                    &canvas::Path::new(|b| {
                        b.move_to(Point::new(inset, inset * 1.2));
                        b.line_to(Point::new(size - inset, inset * 1.2));
                        b.line_to(Point::new(size - inset, size - inset));
                        b.line_to(Point::new(inset, size - inset));
                        b.close();
                    }),
                    stroke,
                );
            }
        }
        vec![frame.into_geometry()]
    }
}

impl<T, Message> Widget<Message, IcedTheme, Renderer> for CommandWidget<'_, T, Message>
where
    T: Clone,
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        let (_, _, enabled) = self.enabled_flags();
        tree::State::new(State::new(
            self.highlighted
                .or_else(|| first_selectable_index(&enabled)),
            self.max_height,
        ))
    }

    fn children(&self) -> Vec<Tree> {
        let list = self.build_list(
            self.highlighted,
            Rc::new(Cell::new(None)),
            self.max_height.max(1.0),
        );
        vec![Tree::new(&self.input), Tree::new(&list)]
    }

    fn diff(&self, tree: &mut Tree) {
        let (_, _, enabled) = self.enabled_flags();
        let state = tree.state.downcast_mut::<State>();
        if let Some(hi) = self.highlighted {
            state.highlighted = Some(hi);
        } else if state
            .highlighted
            .is_some_and(|i| !enabled.get(i).copied().unwrap_or(false))
            || state.highlighted.is_none()
        {
            state.highlighted = first_selectable_index(&enabled);
        }
        let bus = Rc::clone(&state.hover_bus);
        let viewport_h = state.list_viewport_h;
        let hi = state.highlighted;
        let list = self.build_list(hi, bus, viewport_h);
        tree.diff_children(&[&self.input, &list]);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let state = tree.state.downcast_ref::<State>();
        let state_hi = state.highlighted;
        let bus = Rc::clone(&state.hover_bus);
        // `.cn-command`: `p-1` (Lyra `0`).
        let pad = self.recipe.pad_px;
        let limits = limits.width(self.width).loose();
        let inner_w = (limits.max().width - pad * 2.0).max(0.0);

        let input_limits = layout::Limits::new(Size::ZERO, Size::new(inner_w, limits.max().height));
        let input_node =
            self.input
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &input_limits);
        let input_size = input_node.size();

        // Measure list body unconstrained on Y, then apply `max-h-72`.
        let mut body = self.build_list_body(state_hi, Rc::clone(&bus));
        let mut measure_tree = Tree::new(&body);
        let measured = body.as_widget_mut().layout(
            &mut measure_tree,
            renderer,
            &layout::Limits::new(Size::ZERO, Size::new(inner_w, f32::INFINITY)),
        );
        let content_h = measured.size().height;
        let viewport_h = content_h.min(self.max_height.max(1.0)).max(1.0);
        tree.state.downcast_mut::<State>().list_viewport_h = viewport_h;

        let list_limits = layout::Limits::new(Size::ZERO, Size::new(inner_w, viewport_h));
        let mut list = self.build_list(state_hi, bus, viewport_h);
        let list_node = list
            .as_widget_mut()
            .layout(&mut tree.children[1], renderer, &list_limits);
        let list_size = list_node.size();

        let width = input_size.width.max(list_size.width) + pad * 2.0;
        let height = pad + input_size.height + list_size.height + pad;

        layout::Node::with_children(
            Size::new(width, height),
            vec![
                input_node.move_to(Point::new(pad, pad)),
                list_node.move_to(Point::new(pad, pad + input_size.height)),
            ],
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let mut children = layout.children();
        let input_layout = children.next().expect("input layout");
        let list_layout = children.next().expect("list layout");

        self.input.as_widget_mut().operate(
            &mut tree.children[0],
            input_layout,
            renderer,
            operation,
        );

        let (state_hi, bus, viewport_h) = {
            let state = tree.state.downcast_ref::<State>();
            (
                state.highlighted,
                Rc::clone(&state.hover_bus),
                state.list_viewport_h,
            )
        };
        let mut list = self.build_list(state_hi, bus, viewport_h);
        list.as_widget_mut().operate(
            &mut tree.children[1],
            list_layout,
            renderer,
            operation,
        );
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) = event
            && let Some(action) = nav_action(key)
        {
            let (_, _, enabled) = self.enabled_flags();
            let state = tree.state.downcast_mut::<State>();
            let next = match action {
                NavAction::Next => {
                    step_selectable_index(&enabled, state.highlighted, 1, self.loop_highlight)
                }
                NavAction::Previous => {
                    step_selectable_index(&enabled, state.highlighted, -1, self.loop_highlight)
                }
                NavAction::First => first_selectable_index(&enabled),
                NavAction::Last => last_selectable_index(&enabled),
                NavAction::Activate => {
                    if let Some(index) = state.highlighted {
                        let mut items = Vec::new();
                        flatten(&self.rows, &mut items);
                        if let Some(item) = items.get(index)
                            && !item.disabled
                            && let Some(on_select) = &self.on_select
                        {
                            shell.publish(on_select(item.value.clone()));
                        }
                    }
                    shell.capture_event();
                    return;
                }
                _ => None,
            };
            if let Some(index) = next {
                state.highlighted = Some(index);
                if let Some(on_hi) = &self.on_highlight_change {
                    shell.publish(on_hi(index));
                }
                shell.invalidate_layout();
                shell.request_redraw();
                shell.capture_event();
                return;
            }
        }

        let mut children = layout.children();
        let input_layout = children.next().expect("input layout");
        let list_layout = children.next().expect("list layout");

        self.input.as_widget_mut().update(
            &mut tree.children[0],
            event,
            input_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        let (state_hi, bus, viewport_h) = {
            let state = tree.state.downcast_ref::<State>();
            (
                state.highlighted,
                Rc::clone(&state.hover_bus),
                state.list_viewport_h,
            )
        };
        let mut list = self.build_list(state_hi, bus, viewport_h);
        list.as_widget_mut().update(
            &mut tree.children[1],
            event,
            list_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        // Pointer hover moves the cmdk selection (same as arrow keys).
        let hovered = tree.state.downcast_ref::<State>().hover_bus.get();
        if let Some(index) = hovered {
            let state = tree.state.downcast_mut::<State>();
            if state.highlighted != Some(index) {
                state.highlighted = Some(index);
                if let Some(on_hi) = &self.on_highlight_change {
                    shell.publish(on_hi(index));
                }
                shell.invalidate_layout();
                shell.request_redraw();
            }
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &IcedTheme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    color: self.style.border,
                    width: self.style.border_width,
                    radius: border::radius(self.style.radius),
                },
                shadow: self.style.shadow,
                snap: false,
            },
            Background::Color(self.style.background),
        );

        let Some(clipped) = bounds.intersection(viewport) else {
            return;
        };

        let mut children = layout.children();
        let input_layout = children.next().expect("input layout");
        let list_layout = children.next().expect("list layout");

        // `.cn-command { overflow-hidden }` — clip children to the surface box.
        renderer.with_layer(clipped, |renderer| {
            self.input.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                input_layout,
                cursor,
                &clipped,
            );

            let (state_hi, bus, viewport_h) = {
                let state = tree.state.downcast_ref::<State>();
                (
                    state.highlighted,
                    Rc::clone(&state.hover_bus),
                    state.list_viewport_h,
                )
            };
            let list = self.build_list(state_hi, bus, viewport_h);
            list.as_widget().draw(
                &tree.children[1],
                renderer,
                theme,
                style,
                list_layout,
                cursor,
                &clipped,
            );
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let mut children = layout.children();
        let input_layout = children.next().expect("input layout");
        let list_layout = children.next().expect("list layout");

        let input = self.input.as_widget().mouse_interaction(
            &tree.children[0],
            input_layout,
            cursor,
            viewport,
            renderer,
        );
        let (state_hi, bus, viewport_h) = {
            let state = tree.state.downcast_ref::<State>();
            (
                state.highlighted,
                Rc::clone(&state.hover_bus),
                state.list_viewport_h,
            )
        };
        let list = self.build_list(state_hi, bus, viewport_h);
        let list_ix = list.as_widget().mouse_interaction(
            &tree.children[1],
            list_layout,
            cursor,
            viewport,
            renderer,
        );
        input.max(list_ix)
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: layout::Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, IcedTheme, Renderer>> {
        let mut children = layout.children();
        let input_layout = children.next()?;
        self.input.as_widget_mut().overlay(
            &mut tree.children[0],
            input_layout,
            renderer,
            viewport,
            translation,
        )
    }
}

fn nav_action(key: &keyboard::Key) -> Option<NavAction> {
    let nav_key = match key {
        keyboard::Key::Named(keyboard::key::Named::ArrowDown) => NavKey::ArrowDown,
        keyboard::Key::Named(keyboard::key::Named::ArrowUp) => NavKey::ArrowUp,
        keyboard::Key::Named(keyboard::key::Named::Home) => NavKey::Home,
        keyboard::Key::Named(keyboard::key::Named::End) => NavKey::End,
        keyboard::Key::Named(keyboard::key::Named::Enter) => NavKey::Enter,
        keyboard::Key::Named(keyboard::key::Named::Space) => NavKey::Space,
        _ => return None,
    };
    resolve_nav_action(nav_key, Orientation::Vertical, Direction::Ltr)
}

// Silence unused import if default filter is only re-exported via mod.
#[allow(dead_code)]
fn _default_filter() -> CommandFilter {
    default_command_filter
}
