//! Widget construction for the emoji picker.

use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::input::Input;
use crate::fonts::iced_font;
use crate::iced_compat::widget::text_input::{Icon, Side};
use crate::iced_compat::widget::{self, container, row, scrollable, text};
use crate::iced_compat::{Background, Border, Element, Length, Padding, Pixels};

use super::types::{
    EmojiPickerCategory, SelectedEmoji, base_catalog_entry, catalog_entry, category_emojis,
    display_catalog_entry, matches_query,
};
use super::{
    EmojiPicker, EmojiPickerFooter, EmojiPickerList, EmojiPickerSearch,
    EmojiPickerSkinToneSelector, EmojiPickerViewport,
};
use crate::theme::Theme;

const SEARCH_ICON: char = '⌕';
const CELL_SIZE: f32 = 36.0;
const SEARCH_PADDING: f32 = 8.0;
const GROUP_HEADING_HEIGHT: f32 = 24.0;
const EMOJI_TEXT_SIZE: f32 = 18.0;
const DEFAULT_WIDTH: f32 = 232.0;
const DEFAULT_MAX_HEIGHT: f32 = 200.0;

pub(super) fn build_picker<'a, Message: Clone + 'a>(
    picker: EmojiPicker<'a, Message>,
) -> Element<'a, Message> {
    let EmojiPicker {
        theme,
        value,
        query,
        skin,
        show_recents,
        recents,
        max_recents,
        empty_message,
        search_placeholder,
        preview_emoji,
        width,
        max_height,
        disabled,
        on_query_change,
        on_select,
        on_skin_change,
        footer,
        ..
    } = picker;

    let mut search = EmojiPickerSearch::new(query, theme)
        .placeholder(search_placeholder)
        .disabled(disabled);
    if let Some(on_query_change) = on_query_change {
        search = search.on_input(on_query_change);
    }

    let mut list = EmojiPickerList::new(theme)
        .query(search.value())
        .skin(skin)
        .show_recents(show_recents)
        .recents_maybe(recents)
        .max_recents(max_recents)
        .empty_message(empty_message)
        .max_height(max_height)
        .disabled(disabled);
    if let Some(on_select) = on_select {
        list.on_select = Some(on_select);
    }

    let mut content = widget::column![
        container(search.into_element()).padding(SEARCH_PADDING),
        list.into_element(),
    ];

    if let Some(footer) = footer {
        let active = SelectedEmoji::from_native(&value, skin);
        let footer_content = footer(active);
        if let Some(on_skin_change) = on_skin_change {
            let selector = EmojiPickerSkinToneSelector::new(theme)
                .skin(skin)
                .preview_emoji(preview_emoji)
                .on_skin_change(on_skin_change);
            content = content.push(footer_element(
                row![footer_content, selector.into_element()]
                    .width(Length::Fill)
                    .into(),
                theme,
                SEARCH_PADDING,
            ));
        } else {
            content = content.push(footer_element(footer_content, theme, SEARCH_PADDING));
        }
    } else if let Some(on_skin_change) = on_skin_change {
        let selector = EmojiPickerSkinToneSelector::new(theme)
            .skin(skin)
            .preview_emoji(preview_emoji)
            .on_skin_change(on_skin_change);
        content = content.push(footer_element(
            selector.into_element(),
            theme,
            SEARCH_PADDING,
        ));
    }

    EmojiPickerViewport::new(content, theme)
        .width(width)
        .into_element()
}

fn footer_element<'a, Message: 'a>(
    content: Element<'a, Message>,
    theme: &'a Theme,
    padding: f32,
) -> Element<'a, Message> {
    let divider = container(widget::Space::new())
        .width(Length::Fill)
        .height(1.0)
        .style(move |_| container::Style {
            background: Some(Background::Color(theme.palette.border)),
            ..container::Style::default()
        });

    widget::column![divider, container(content).padding(Padding::new(padding))]
        .width(Length::Fill)
        .into()
}

pub(super) fn build_search<'a, Message: Clone + 'a>(
    search: EmojiPickerSearch<'a, Message>,
) -> Element<'a, Message> {
    let EmojiPickerSearch {
        theme,
        value,
        placeholder,
        width,
        disabled,
        on_input,
    } = search;

    let mut input = Input::new(theme)
        .value(value)
        .placeholder(placeholder)
        .width(width)
        .disabled(disabled)
        .icon(Icon {
            font: iced_font(theme.font_pack().sans),
            code_point: SEARCH_ICON,
            size: Some(Pixels(16.0)),
            spacing: 8.0,
            side: Side::Left,
        });

    if let Some(on_input) = on_input {
        input = input.on_input(on_input);
    }

    input.into()
}

pub(super) fn build_list<'a, Message: Clone + 'a>(
    list: EmojiPickerList<'a, Message>,
) -> Element<'a, Message> {
    let EmojiPickerList {
        theme,
        query,
        skin,
        show_recents,
        recents,
        max_recents,
        empty_message,
        max_height,
        disabled,
        on_select,
    } = list;

    let mut groups = Vec::new();

    if show_recents && let Some(recents) = recents {
        let recent_emojis: Vec<_> = recents
            .iter()
            .take(max_recents)
            .filter_map(|recent| catalog_entry(recent.emoji()))
            .filter(|emoji| matches_query(emoji, &query))
            .collect();

        if !recent_emojis.is_empty() {
            groups.push((Some("Recently used"), recent_emojis));
        }
    }

    for category in EmojiPickerCategory::ALL {
        let filtered: Vec<_> = category_emojis(category)
            .into_iter()
            .filter(|emoji| matches_query(emoji, &query))
            .collect();

        if !filtered.is_empty() {
            groups.push((Some(category.title()), filtered));
        }
    }

    if groups.is_empty() {
        return scrollable::Scrollable::with_direction(
            container(
                text(empty_message)
                    .size(12)
                    .font(iced_font(theme.font_pack().sans))
                    .color(theme.palette.muted_foreground),
            )
            .width(Length::Fill)
            .padding(Padding::new(16.0))
            .center_x(Length::Fill),
            scrollable::Direction::Vertical(scrollable::Scrollbar::hidden()),
        )
        .height(Length::Fixed(max_height))
        .width(Length::Fill)
        .into();
    }

    let mut children = Vec::new();
    for (heading, emojis) in groups {
        if let Some(heading) = heading {
            children.push(
                container(
                    text(heading)
                        .size(12)
                        .font(iced_font(theme.font_pack().sans))
                        .color(theme.palette.muted_foreground),
                )
                .width(Length::Fill)
                .height(GROUP_HEADING_HEIGHT)
                .padding(Padding {
                    top: 4.0,
                    right: SEARCH_PADDING,
                    bottom: 4.0,
                    left: SEARCH_PADDING,
                })
                .into(),
            );
        }

        for chunk in emojis.chunks(6) {
            let mut cells = Vec::with_capacity(6);

            for emoji in chunk {
                let base = base_catalog_entry(emoji);
                let displayed = if heading == Some("Recently used") {
                    emoji
                } else {
                    display_catalog_entry(base, skin)
                };
                let selected = if heading == Some("Recently used") {
                    SelectedEmoji::from_native(emoji.as_str(), skin)
                        .unwrap_or_else(|| SelectedEmoji::from_catalog(base, skin))
                } else {
                    SelectedEmoji::from_catalog(base, skin)
                };

                let mut button = Button::icon(
                    text(displayed.as_str())
                        .size(EMOJI_TEXT_SIZE)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center(),
                    theme,
                )
                .size(ButtonSize::Icon)
                .variant(ButtonVariant::Ghost)
                .disabled(disabled);

                if !disabled && let Some(on_select) = on_select.as_ref() {
                    let on_select = std::sync::Arc::clone(on_select);
                    button = button.on_press_with(move || on_select(selected.clone()));
                }

                cells.push(button.into());
            }

            while cells.len() < 6 {
                cells.push(
                    widget::Space::new()
                        .width(CELL_SIZE)
                        .height(CELL_SIZE)
                        .into(),
                );
            }

            children.push(
                container(row(cells).width(Length::Fill).height(CELL_SIZE))
                    .width(Length::Fill)
                    .height(CELL_SIZE)
                    .padding(Padding {
                        top: 0.0,
                        right: SEARCH_PADDING,
                        bottom: 0.0,
                        left: SEARCH_PADDING,
                    })
                    .into(),
            );
        }
    }

    scrollable::Scrollable::with_direction(
        widget::column(children).width(Length::Fill),
        scrollable::Direction::Vertical(scrollable::Scrollbar::hidden()),
    )
    .height(Length::Fixed(max_height))
    .width(Length::Fill)
    .into()
}

pub(super) fn build_viewport<'a, Message: 'a>(
    viewport: EmojiPickerViewport<'a, Message>,
) -> Element<'a, Message> {
    let EmojiPickerViewport {
        theme,
        content,
        width,
        radius,
    } = viewport;

    container(content)
        .width(width)
        .style(move |_| container::Style {
            background: Some(Background::Color(theme.palette.popover)),
            text_color: Some(theme.palette.popover_foreground),
            border: Border {
                color: theme.palette.border,
                width: 1.0,
                radius: radius
                    .unwrap_or_else(|| crate::components::popover::surface_radius(theme))
                    .into(),
            },
            ..container::Style::default()
        })
        .into()
}

pub(super) fn build_footer<'a, Message: 'a>(
    footer: EmojiPickerFooter<'a, Message>,
) -> Element<'a, Message> {
    let EmojiPickerFooter {
        theme,
        content,
        padding,
    } = footer;

    footer_element(
        container(content.unwrap_or_else(|| widget::Space::new().into()))
            .width(Length::Fill)
            .into(),
        theme,
        padding,
    )
}

pub(super) fn build_skin_selector<'a, Message: Clone + 'a>(
    selector: EmojiPickerSkinToneSelector<'a, Message>,
) -> Element<'a, Message> {
    let EmojiPickerSkinToneSelector {
        theme,
        skin,
        preview_emoji,
        disabled,
        on_skin_change,
    } = selector;

    let preview = catalog_entry(&preview_emoji)
        .map(|emoji| {
            display_catalog_entry(base_catalog_entry(emoji), skin)
                .as_str()
                .to_owned()
        })
        .unwrap_or_else(|| preview_emoji.clone());

    let mut button = Button::icon(
        text(preview)
            .size(EMOJI_TEXT_SIZE)
            .width(Length::Fill)
            .height(Length::Fill)
            .center(),
        theme,
    )
    .size(ButtonSize::IconSm)
    .variant(ButtonVariant::Outline)
    .disabled(disabled);

    if !disabled && let Some(on_skin_change) = on_skin_change {
        button = button.on_press(on_skin_change(skin.next()));
    }

    button.into()
}

pub(super) fn default_width() -> Length {
    Length::Fixed(DEFAULT_WIDTH)
}

pub(super) fn default_max_height() -> f32 {
    DEFAULT_MAX_HEIGHT
}
