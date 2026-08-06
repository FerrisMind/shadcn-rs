//! Rendering for pagination built on top of the button component.

use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::text::{Fragment, LineHeight};
use crate::iced_compat::widget::{container, row, text as iced_text};
use crate::iced_compat::{Element, Font, Length};

use shadcn_common::AccentColor;

use super::geometry;
use super::types::{self, PaginationItem};
use super::{NavDirection, Pagination};
use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;
use crate::theme::Theme;

/// Text stand-ins for the lucide `ChevronLeft` / `ChevronRight` /
/// `MoreHorizontal` glyphs, consistent with the glyph-based icons used
/// elsewhere in the crate.
const PREVIOUS_GLYPH: &str = "‹";
const NEXT_GLYPH: &str = "›";
const ELLIPSIS_GLYPH: &str = "…";

pub(super) fn build_pagination<'a, Message>(root: Pagination<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let Pagination {
        theme,
        count,
        per_page,
        page,
        sibling_count,
        link_size,
        controls_size,
        active_variant,
        inactive_variant,
        color,
        spacing,
        show_controls,
        show_links,
        show_labels,
        previous_label,
        next_label,
        disabled,
        width,
        on_page_change,
    } = root;

    let total = types::total_pages(count, per_page);
    let page = page.clamp(1, total);
    let emit = |target: usize| on_page_change.as_ref().map(|callback| callback(target));

    let mut children: Vec<Element<'a, Message>> = Vec::new();

    if show_controls {
        let at_start = page == 1;
        children.push(build_nav(NavConfig {
            theme,
            direction: NavDirection::Previous,
            label: previous_label,
            show_label: show_labels,
            icon: None,
            size: controls_size,
            variant: inactive_variant,
            color,
            disabled: disabled || at_start,
            on_press: (!disabled && !at_start).then(|| emit(page - 1)).flatten(),
        }));
    }

    if show_links {
        for item in types::page_items(page, total, sibling_count) {
            match item {
                PaginationItem::Page(target) => children.push(build_link(LinkConfig {
                    theme,
                    page: target,
                    content: None,
                    active: target == page,
                    size: link_size,
                    active_variant,
                    inactive_variant,
                    color,
                    disabled,
                    on_press: (!disabled).then(|| emit(target)).flatten(),
                })),
                PaginationItem::Ellipsis => children.push(build_ellipsis(theme, link_size)),
                _ => {}
            }
        }
    }

    if show_controls {
        let at_end = page == total;
        children.push(build_nav(NavConfig {
            theme,
            direction: NavDirection::Next,
            label: next_label,
            show_label: show_labels,
            icon: None,
            size: controls_size,
            variant: inactive_variant,
            color,
            disabled: disabled || at_end,
            on_press: (!disabled && !at_end).then(|| emit(page + 1)).flatten(),
        }));
    }

    let content = row(children)
        .spacing(geometry::spacing_px(theme, spacing))
        .align_y(Vertical::Center);

    container(content)
        .width(width)
        .align_x(Horizontal::Center)
        .into()
}

pub(super) struct LinkConfig<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) page: usize,
    pub(super) content: Option<Element<'a, Message>>,
    pub(super) active: bool,
    pub(super) size: ButtonSize,
    pub(super) active_variant: ButtonVariant,
    pub(super) inactive_variant: ButtonVariant,
    pub(super) color: Option<AccentColor>,
    pub(super) disabled: bool,
    pub(super) on_press: Option<Message>,
}

pub(super) fn build_link<'a, Message>(config: LinkConfig<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let LinkConfig {
        theme,
        page,
        content,
        active,
        size,
        active_variant,
        inactive_variant,
        color,
        disabled,
        on_press,
    } = config;

    let mut button = match content {
        Some(element) => Button::new(element, theme),
        None => Button::text(page.to_string(), theme),
    };

    button = button
        .variant(if active {
            active_variant
        } else {
            inactive_variant
        })
        .size(size)
        .disabled(disabled)
        .on_press_maybe(on_press);

    if let Some(color) = color {
        button = button.color(color);
    }

    button.into()
}

pub(super) struct NavConfig<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) direction: NavDirection,
    pub(super) label: Fragment<'a>,
    pub(super) show_label: bool,
    pub(super) icon: Option<Element<'a, Message>>,
    pub(super) size: ButtonSize,
    pub(super) variant: ButtonVariant,
    pub(super) color: Option<AccentColor>,
    pub(super) disabled: bool,
    pub(super) on_press: Option<Message>,
}

pub(super) fn build_nav<'a, Message>(config: NavConfig<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let NavConfig {
        theme,
        direction,
        label,
        show_label,
        icon,
        size,
        variant,
        color,
        disabled,
        on_press,
    } = config;

    let recipe = geometry::size_recipe(theme, size);
    let font = button_label_font(theme);
    let icon: Element<'a, Message> = icon.unwrap_or_else(|| {
        let glyph = match direction {
            NavDirection::Previous => PREVIOUS_GLYPH,
            NavDirection::Next => NEXT_GLYPH,
        };

        iced_text(glyph)
            .size(recipe.text_size_px + 3.0)
            .font(font)
            .into()
    });

    let mut button = if show_label {
        let type_recipe = theme.style.button_type();
        let text = if type_recipe.typography.uppercase {
            label.as_ref().to_uppercase()
        } else {
            label.into_owned()
        };
        let label = iced_text(text)
            .size(recipe.text_size_px)
            .line_height(LineHeight::Absolute(recipe.text_size_px.into()))
            .font(font);

        let content = match direction {
            NavDirection::Previous => row![icon, label],
            NavDirection::Next => row![label, icon],
        }
        .spacing(recipe.gap_px)
        .align_y(Vertical::Center);

        Button::new(content, theme)
    } else {
        Button::icon(icon, theme)
    };

    button = button
        .variant(variant)
        .size(size)
        .disabled(disabled)
        .on_press_maybe(on_press);

    if let Some(color) = color {
        button = button.color(color);
    }

    button.into()
}

pub(super) fn build_ellipsis<'a, Message>(
    theme: &'a Theme,
    size: ButtonSize,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let recipe = geometry::size_recipe(theme, size);
    let color = theme.palette.muted_foreground;

    container(
        iced_text(ELLIPSIS_GLYPH)
            .size(recipe.text_size_px + 2.0)
            .font(button_label_font(theme))
            .color(color),
    )
    .width(Length::Fixed(recipe.height_px))
    .height(Length::Fixed(recipe.height_px))
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .into()
}

fn button_label_font(theme: &Theme) -> Font {
    let mut font = iced_font(theme.font_pack().sans);
    font.weight = iced_font_weight(theme.style.button_type().typography.weight);
    font
}
