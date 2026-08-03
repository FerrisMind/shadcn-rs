//! Composition rendering for [`super::Combobox`].

use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::command::{Command, CommandEntry, CommandItem};
use crate::components::popover::Popover;
use crate::fonts::iced_font;
use crate::iced_compat::widget::button as button_widget;
use crate::iced_compat::widget::text::LineHeight;
use crate::iced_compat::widget::{container, row, text};
use crate::iced_compat::{Element, Length, Pixels};
use crate::recipes::iced_font_weight;
use crate::theme::Theme;

use shadcn_common::ControlSize;
use twill_core::prelude::theme::SemanticColor;

use super::icon;
use super::{Combobox, SelectSelection};

/// Builds the composed button → popover → command element.
pub(super) fn build<'a, T, Message>(combobox: Combobox<'a, T, Message>) -> Element<'a, Message>
where
    T: Clone + PartialEq + 'a,
    Message: Clone + 'a,
{
    let Combobox {
        theme,
        mut rows,
        selection,
        select_type,
        placeholder,
        search_placeholder,
        query,
        empty,
        trigger_variant,
        trigger_size,
        trigger_radius,
        trigger_color,
        trigger_width,
        command_radius,
        command_width,
        command_max_height,
        command_should_filter,
        command_filter,
        command_show_search_icon,
        command_show_border,
        command_show_shadow,
        command_loop_highlight,
        highlighted,
        popover_width,
        popover_content_padding,
        popover_side,
        popover_align,
        popover_side_offset,
        popover_align_offset,
        popover_animated,
        popover_close_on_click_outside,
        popover_close_on_escape,
        disabled,
        invalid,
        deselectable,
        open,
        default_open,
        on_open_change,
        on_query_change,
        on_select,
        on_selection_change,
        on_highlight_change,
        trigger_style_override,
        command_style_override,
        popover_style_override,
    } = combobox;

    mark_selection(&mut rows, &selection);

    let trigger_open_message = on_open_change.as_ref().map(|callback| callback(true));
    let trigger = build_trigger(
        theme,
        selected_text(&rows, &selection, &placeholder),
        trigger_variant,
        trigger_size,
        trigger_radius,
        trigger_color,
        trigger_width,
        disabled,
        invalid,
        trigger_open_message,
        trigger_style_override,
    );

    let mut command = Command::new(theme)
        .query(query)
        .placeholder(search_placeholder)
        .width(command_width)
        .max_height(command_max_height)
        .should_filter(command_should_filter)
        .filter(command_filter)
        .show_search_icon(command_show_search_icon)
        .show_border(command_show_border)
        .show_shadow(command_show_shadow)
        .loop_highlight(command_loop_highlight)
        .highlighted_maybe(highlighted);

    for entry in rows {
        command = command.entry(entry);
    }

    if let Some(empty) = empty {
        command = command.empty(empty);
    }
    if let Some(radius) = command_radius {
        command = command.radius(radius);
    }
    if let Some(on_query_change) = on_query_change {
        command = command.on_query_change(on_query_change);
    }
    if let Some(on_highlight_change) = on_highlight_change {
        command = command.on_highlight_change(on_highlight_change);
    }

    // Command has one activation message per item. Prefer the controlled
    // selection snapshot when both callback forms are configured; this keeps
    // a single activation from producing two potentially conflicting app
    // updates while retaining the direct-value API for simple cases.
    if let Some(on_selection_change) = on_selection_change {
        let current = selection.clone();
        command = command.on_select(move |value| {
            on_selection_change(current.clone().toggled(select_type, &value, deselectable))
        });
    } else if let Some(on_select) = on_select {
        command = command.on_select(on_select);
    }

    if let Some(style_override) = command_style_override {
        command = command.style_override(style_override);
    }

    let content: Element<'a, Message> = command.into();
    let mut popover = Popover::new(trigger, content, theme)
        .side(popover_side)
        .align(popover_align)
        .side_offset(popover_side_offset)
        .align_offset(popover_align_offset)
        .content_padding(popover_content_padding)
        .animated(popover_animated)
        .disabled(disabled)
        .close_on_click_outside(popover_close_on_click_outside)
        .close_on_escape(popover_close_on_escape);

    let resolved_popover_width = popover_width.or_else(|| match trigger_width {
        Length::Fixed(width) if width.is_finite() => Some(width.max(0.0)),
        _ => None,
    });
    if let Some(width) = resolved_popover_width {
        popover = popover.width(width);
    }
    if let Some(open) = open {
        popover = popover.open(open);
    }
    if default_open {
        popover = popover.default_open(true);
    }
    if let Some(on_open_change) = on_open_change {
        popover = popover.on_open_change(on_open_change);
    }
    if let Some(style_override) = popover_style_override {
        popover = popover.style_override(style_override);
    }

    popover.into()
}

#[allow(clippy::too_many_arguments)]
fn build_trigger<'a, Message>(
    theme: &'a Theme,
    label: String,
    variant: ButtonVariant,
    size: ButtonSize,
    radius: Option<crate::components::button::ButtonRadius>,
    color: Option<shadcn_common::AccentColor>,
    width: Length,
    disabled: bool,
    invalid: bool,
    on_press: Option<Message>,
    style_override: Option<
        Box<dyn Fn(button_widget::Style, button_widget::Status) -> button_widget::Style + 'a>,
    >,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let control_size = button_control_size(size);
    let button_recipe = theme.style.button_size(control_size);
    let button_type = theme.style.button_type();
    let mut font = iced_font(theme.font_pack().sans);
    font.weight = iced_font_weight(button_type.typography.weight);
    let label = if button_type.typography.uppercase {
        label.to_uppercase()
    } else {
        label
    };

    let icon_size = button_recipe.icon_px;
    // `Button` keeps ordinary content shrink-wrapped so intrinsic buttons do
    // not accidentally become full-width. A fixed-width Combobox trigger is
    // the `justify-between` case from shadcn-svelte, however: its content row
    // must occupy the space between the button's horizontal padding edges so
    // the chevron sits at the same right offset as the web trigger.
    let content_width = match width {
        Length::Fixed(width) if width.is_finite() => {
            Length::Fixed((width - 2.0 * button_recipe.pad_x_px).max(0.0))
        }
        _ => Length::Fill,
    };
    let mut icon_color = theme.semantic_color(SemanticColor::MutedForeground);
    icon_color.a *= 0.5;
    if disabled {
        icon_color.a *= 0.5;
    }

    let trigger_content: Element<'a, Message> = row![
        container(
            text(label)
                .size(Pixels(button_recipe.text_size_px))
                .line_height(LineHeight::Absolute(Pixels(
                    button_type.typography.line_height_px,
                )))
                .font(font),
        )
        .width(Length::Fill),
        icon::chevrons_up_down(icon_size, icon_color),
    ]
    .spacing(button_recipe.gap_px)
    .align_y(crate::iced_compat::alignment::Vertical::Center)
    .width(content_width)
    .into();

    let mut button = Button::new(trigger_content, theme)
        .variant(variant)
        .size(size)
        .width(width)
        .disabled(disabled)
        .on_press_maybe(on_press);

    if let Some(radius) = radius {
        button = button.radius(radius);
    }
    if let Some(color) = color {
        button = button.color(color);
    }

    if invalid || style_override.is_some() {
        button = button.style_override(move |mut resolved, status| {
            if invalid {
                let mut destructive = theme.semantic_color(SemanticColor::Destructive);
                if theme.is_dark() {
                    destructive.a *= 0.5;
                }
                if disabled {
                    destructive.a *= 0.5;
                }
                resolved.border.color = destructive;
                resolved.border.width = 1.0;
            }
            if let Some(style_override) = style_override.as_ref() {
                resolved = style_override(resolved, status);
            }
            resolved
        });
    }

    button.into()
}

fn button_control_size(size: ButtonSize) -> ControlSize {
    match size {
        ButtonSize::Xs | ButtonSize::IconXs => ControlSize::Xs,
        ButtonSize::Sm | ButtonSize::IconSm => ControlSize::Sm,
        ButtonSize::Default | ButtonSize::Icon => ControlSize::Md,
        ButtonSize::Lg | ButtonSize::IconLg => ControlSize::Lg,
    }
}

pub(super) fn selected_text<T>(
    rows: &[CommandEntry<T>],
    selection: &SelectSelection<T>,
    placeholder: &str,
) -> String
where
    T: PartialEq,
{
    match selection {
        SelectSelection::Single(Some(value)) => find_label(rows, value)
            .map(str::to_owned)
            .unwrap_or_else(|| placeholder.to_owned()),
        SelectSelection::Multiple(values) if values.len() == 1 => find_label(rows, &values[0])
            .map(str::to_owned)
            .unwrap_or_else(|| placeholder.to_owned()),
        SelectSelection::Multiple(values) if !values.is_empty() => {
            format!("{} selected", values.len())
        }
        _ => placeholder.to_owned(),
    }
}

fn find_label<'a, T: PartialEq>(rows: &'a [CommandEntry<T>], value: &T) -> Option<&'a str> {
    rows.iter().find_map(|entry| match entry {
        CommandEntry::Item(item) if &item.value == value => Some(item.label.as_str()),
        CommandEntry::Group(group) => find_label(&group.entries, value),
        _ => None,
    })
}

pub(super) fn mark_selection<T: PartialEq>(
    rows: &mut [CommandEntry<T>],
    selection: &SelectSelection<T>,
) {
    for entry in rows {
        match entry {
            CommandEntry::Item(CommandItem {
                value,
                checked,
                leading_check,
                ..
            }) => {
                *checked = selection.is_selected(value);
                *leading_check = true;
            }
            CommandEntry::Group(group) => mark_selection(&mut group.entries, selection),
            CommandEntry::Separator { .. } | CommandEntry::Loading(_) => {}
        }
    }
}
