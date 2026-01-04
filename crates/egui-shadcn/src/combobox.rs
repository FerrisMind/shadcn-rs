use crate::button::{Button, ButtonJustify, ButtonSize, ButtonVariant};
use crate::icons::{icon_check, icon_chevrons_up_down};
use crate::input::{InputProps, InputSize, InputVariant, text_input_with_props};
use crate::popover::{PopoverAlign, PopoverProps, PopoverSide, popover};
use crate::select::{SelectItem, SelectSize};
use crate::theme::Theme;
use crate::tokens::ControlSize;
use egui::{
    Color32, CornerRadius, FontId, Key, Margin, Modifiers, Rect, Response, RichText, ScrollArea,
    Sense, Ui, pos2, vec2,
};
use log::trace;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ComboboxSize {
    Size1,

    #[default]
    Size2,

    Size3,
}

impl From<ControlSize> for ComboboxSize {
    fn from(size: ControlSize) -> Self {
        match size {
            ControlSize::Sm | ControlSize::IconSm => ComboboxSize::Size1,
            ControlSize::Md | ControlSize::Icon => ComboboxSize::Size2,
            ControlSize::Lg | ControlSize::IconLg => ComboboxSize::Size3,
        }
    }
}

impl From<ComboboxSize> for InputSize {
    fn from(size: ComboboxSize) -> Self {
        match size {
            ComboboxSize::Size1 => InputSize::Size1,
            ComboboxSize::Size2 => InputSize::Size2,
            ComboboxSize::Size3 => InputSize::Size3,
        }
    }
}

impl From<ComboboxSize> for SelectSize {
    fn from(size: ComboboxSize) -> Self {
        match size {
            ComboboxSize::Size1 => SelectSize::Size1,
            ComboboxSize::Size2 => SelectSize::Size2,
            ComboboxSize::Size3 => SelectSize::Size3,
        }
    }
}

impl From<ComboboxSize> for ButtonSize {
    fn from(size: ComboboxSize) -> Self {
        match size {
            ComboboxSize::Size1 => ButtonSize::Sm,
            ComboboxSize::Size2 => ButtonSize::Default,
            ComboboxSize::Size3 => ButtonSize::Lg,
        }
    }
}

pub struct ComboboxProps<'a, Id> {
    pub id_source: Id,
    pub value: &'a mut Option<String>,
    pub search_value: &'a mut String,
    pub items: &'a [SelectItem],
    pub placeholder: &'a str,
    pub search_placeholder: &'a str,
    pub empty_text: &'a str,
    pub size: ComboboxSize,
    pub variant: InputVariant,
    pub trigger_variant: ButtonVariant,
    pub trigger_justify: ButtonJustify,
    pub disabled: bool,
    pub width: Option<f32>,
    pub on_value_change: Option<Box<dyn FnMut(Option<String>) + 'a>>,
}

impl<'a, Id: Hash + Debug> ComboboxProps<'a, Id> {
    pub fn new(
        id_source: Id,
        value: &'a mut Option<String>,
        items: &'a [SelectItem],
        search_value: &'a mut String,
    ) -> Self {
        Self {
            id_source,
            value,
            search_value,
            items,
            placeholder: "Select option...",
            search_placeholder: "Search...",
            empty_text: "No option found.",
            size: ComboboxSize::Size2,
            variant: InputVariant::Surface,
            trigger_variant: ButtonVariant::Outline,
            trigger_justify: ButtonJustify::Between,
            disabled: false,
            width: None,
            on_value_change: None,
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn search_placeholder(mut self, placeholder: &'a str) -> Self {
        self.search_placeholder = placeholder;
        self
    }

    pub fn empty_text(mut self, empty_text: &'a str) -> Self {
        self.empty_text = empty_text;
        self
    }

    pub fn size(mut self, size: ComboboxSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn trigger_variant(mut self, variant: ButtonVariant) -> Self {
        self.trigger_variant = variant;
        self
    }

    pub fn trigger_justify(mut self, justify: ButtonJustify) -> Self {
        self.trigger_justify = justify;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn on_value_change<F>(mut self, callback: F) -> Self
    where
        F: FnMut(Option<String>) + 'a,
    {
        self.on_value_change = Some(Box::new(callback));
        self
    }
}

fn get_selected_label(items: &[SelectItem], value: &Option<String>) -> Option<String> {
    if let Some(val) = value {
        for item in items {
            match item {
                SelectItem::Option {
                    value: item_value,
                    label,
                    ..
                } => {
                    if item_value == val {
                        return Some(label.clone());
                    }
                }
                SelectItem::Group { items, .. } => {
                    if let Some(label) = get_selected_label(items, value) {
                        return Some(label);
                    }
                }
                _ => {}
            }
        }
    }
    None
}

fn filter_items(items: &[SelectItem], search: &str) -> Vec<SelectItem> {
    if search.is_empty() {
        return items.to_vec();
    }

    let search_lower = search.to_lowercase();
    let mut filtered = Vec::new();

    for item in items {
        match item {
            SelectItem::Option {
                value,
                label,
                disabled: _,
                text_value,
            } => {
                let searchable = text_value.as_deref().unwrap_or(label);
                if searchable.to_lowercase().contains(&search_lower)
                    || value.to_lowercase().contains(&search_lower)
                {
                    filtered.push(item.clone());
                }
            }
            SelectItem::Group { label, items } => {
                let filtered_group_items = filter_items(items, search);
                if !filtered_group_items.is_empty() {
                    filtered.push(SelectItem::Group {
                        label: label.clone(),
                        items: filtered_group_items,
                    });
                }
            }
            SelectItem::Separator => {
                if !filtered.is_empty() {
                    filtered.push(SelectItem::Separator);
                }
            }
            SelectItem::Label(text) => {
                filtered.push(SelectItem::Label(text.clone()));
            }
        }
    }

    filtered
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FlatOption {
    value: String,
    label: String,
    disabled: bool,
}

fn flatten_options(items: &[SelectItem], out: &mut Vec<FlatOption>) {
    for item in items {
        match item {
            SelectItem::Option {
                value,
                label,
                disabled,
                ..
            } => out.push(FlatOption {
                value: value.clone(),
                label: label.clone(),
                disabled: *disabled,
            }),
            SelectItem::Group { items, .. } => flatten_options(items, out),
            SelectItem::Separator | SelectItem::Label(_) => {}
        }
    }
}

#[allow(dead_code)]
fn combobox_with_props_legacy<Id>(
    ui: &mut Ui,
    theme: &Theme,
    mut props: ComboboxProps<'_, Id>,
) -> Response
where
    Id: Hash + Debug,
{
    trace!(
        "Rendering combobox size={:?} variant={:?} disabled={} value={:?}",
        props.size, props.variant, props.disabled, props.value
    );

    let id = ui.make_persistent_id(&props.id_source);
    let open_id = id.with("open");
    let is_open = ui
        .ctx()
        .memory_mut(|m| m.data.get_persisted::<bool>(open_id).unwrap_or(false));

    let display_text = if let Some(val) = props.value.as_ref() {
        get_selected_label(props.items, props.value).unwrap_or_else(|| val.clone())
    } else {
        String::new()
    };

    let input_size: InputSize = props.size.into();
    let width = props.width.unwrap_or(200.0);

    let mut input_value = if is_open {
        props.search_value.clone()
    } else {
        display_text
    };

    let input_response = text_input_with_props(
        ui,
        theme,
        InputProps::new(id.with("input"), &mut input_value)
            .placeholder(props.placeholder)
            .variant(props.variant)
            .size(input_size)
            .enabled(!props.disabled)
            .width(width),
    );

    if is_open {
        *props.search_value = input_value.clone();
    }

    let clicked = input_response.clicked();
    let gained_focus = input_response.gained_focus();

    if clicked && !props.disabled {
        ui.memory_mut(|m| m.data.insert_persisted(open_id, true));
    }

    if gained_focus && !props.disabled {
        ui.memory_mut(|m| m.data.insert_persisted(open_id, true));
    }

    let filtered_items = if is_open {
        filter_items(props.items, props.search_value)
    } else {
        Vec::new()
    };

    let mut selected_value = props.value.clone();
    let mut should_close = false;
    let mut final_response = input_response;

    if is_open {
        let mut open_state = is_open;
        let (trigger_resp, content_result) = popover(
            ui,
            theme,
            PopoverProps::new(id.with("popover"), &mut open_state)
                .side(PopoverSide::Bottom)
                .align(PopoverAlign::Start)
                .with_width(width)
                .with_max_height(300.0)
                .with_animation(true),
            |_ui| final_response,
            |ui| {
                let mut clicked_value: Option<String> = None;

                ui.vertical(|ui| {
                    ui.set_width(width);

                    if filtered_items.is_empty() {
                        let painter = ui.painter();
                        let galley = painter.layout_no_wrap(
                            props.empty_text.to_string(),
                            FontId::proportional(input_size.font_size()),
                            theme.palette.muted_foreground,
                        );
                        let galley_size = galley.size();
                        let available = ui.available_rect_before_wrap();
                        let pos = pos2(available.left() + 8.0, available.top() + 8.0);
                        painter.galley(pos, galley, Color32::TRANSPARENT);
                        ui.allocate_space(vec2(width, galley_size.y + 16.0));
                    } else {
                        let select_size: SelectSize = props.size.into();
                        let item_height = select_size.item_height();
                        let padding = 4.0;

                        for item in &filtered_items {
                            match item {
                                SelectItem::Option {
                                    value,
                                    label,
                                    disabled,
                                    ..
                                } => {
                                    let (_item_id, item_rect) =
                                        ui.allocate_space(vec2(width, item_height));
                                    let item_response =
                                        ui.allocate_response(item_rect.size(), Sense::click());
                                    let is_selected = props.value.as_ref() == Some(value);
                                    let is_hovered = item_rect.contains(
                                        ui.input(|i| i.pointer.hover_pos().unwrap_or_default()),
                                    );

                                    let bg_color = if *disabled {
                                        theme.palette.muted
                                    } else if is_selected {
                                        theme.palette.accent
                                    } else if is_hovered {
                                        theme.palette.muted
                                    } else {
                                        Color32::TRANSPARENT
                                    };

                                    let text_color = if *disabled {
                                        theme.palette.muted_foreground
                                    } else if is_selected {
                                        theme.palette.accent_foreground
                                    } else {
                                        theme.palette.foreground
                                    };

                                    let painter = ui.painter();
                                    painter.rect_filled(item_rect, CornerRadius::same(4), bg_color);

                                    let galley = painter.layout_no_wrap(
                                        label.clone(),
                                        FontId::proportional(select_size.font_size()),
                                        text_color,
                                    );
                                    let text_pos = pos2(
                                        item_rect.left() + padding,
                                        item_rect.center().y - galley.size().y * 0.5,
                                    );
                                    painter.galley(text_pos, galley, Color32::TRANSPARENT);

                                    if item_response.clicked() {
                                        clicked_value = Some(value.clone());
                                        should_close = true;
                                    }
                                }
                                SelectItem::Separator => {
                                    let available = ui.available_rect_before_wrap();
                                    let cursor = ui.cursor();
                                    let sep_rect = Rect::from_min_size(
                                        pos2(available.left(), cursor.max.y + 4.0),
                                        vec2(width, 1.0),
                                    );
                                    ui.painter().rect_filled(
                                        sep_rect,
                                        CornerRadius::ZERO,
                                        theme.palette.border,
                                    );
                                    ui.allocate_space(vec2(width, 8.0));
                                }
                                SelectItem::Label(text) => {
                                    let painter = ui.painter();
                                    let galley = painter.layout_no_wrap(
                                        text.clone(),
                                        FontId::proportional(12.0),
                                        theme.palette.muted_foreground,
                                    );
                                    let galley_size = galley.size();
                                    let available = ui.available_rect_before_wrap();
                                    let cursor = ui.cursor();
                                    let pos = pos2(available.left() + padding, cursor.max.y + 4.0);
                                    painter.galley(pos, galley, Color32::TRANSPARENT);
                                    ui.allocate_space(vec2(width, galley_size.y + 8.0));
                                }
                                SelectItem::Group { label, items } => {
                                    let painter = ui.painter();
                                    let galley = painter.layout_no_wrap(
                                        label.clone(),
                                        FontId::proportional(12.0),
                                        theme.palette.muted_foreground,
                                    );
                                    let galley_size = galley.size();
                                    let available = ui.available_rect_before_wrap();
                                    let cursor = ui.cursor();
                                    let pos = pos2(available.left() + padding, cursor.max.y + 4.0);
                                    painter.galley(pos, galley, Color32::TRANSPARENT);
                                    ui.allocate_space(vec2(width, galley_size.y + 4.0));

                                    for sub_item in items {
                                        if let SelectItem::Option {
                                            value,
                                            label,
                                            disabled,
                                            ..
                                        } = sub_item
                                        {
                                            let (_sub_item_id, item_rect) =
                                                ui.allocate_space(vec2(width, item_height));
                                            let item_response = ui.allocate_response(
                                                item_rect.size(),
                                                Sense::click(),
                                            );
                                            let is_selected = props.value.as_ref() == Some(value);
                                            let is_hovered = item_rect.contains(ui.input(|i| {
                                                i.pointer.hover_pos().unwrap_or_default()
                                            }));

                                            let bg_color = if *disabled {
                                                theme.palette.muted
                                            } else if is_selected {
                                                theme.palette.accent
                                            } else if is_hovered {
                                                theme.palette.muted
                                            } else {
                                                Color32::TRANSPARENT
                                            };

                                            let text_color = if *disabled {
                                                theme.palette.muted_foreground
                                            } else if is_selected {
                                                theme.palette.accent_foreground
                                            } else {
                                                theme.palette.foreground
                                            };

                                            let painter = ui.painter();
                                            painter.rect_filled(
                                                item_rect,
                                                CornerRadius::same(4),
                                                bg_color,
                                            );

                                            let galley = painter.layout_no_wrap(
                                                label.clone(),
                                                FontId::proportional(select_size.font_size()),
                                                text_color,
                                            );
                                            let text_pos = pos2(
                                                item_rect.left() + padding * 2.0,
                                                item_rect.center().y - galley.size().y * 0.5,
                                            );
                                            painter.galley(text_pos, galley, Color32::TRANSPARENT);

                                            if item_response.clicked() {
                                                clicked_value = Some(value.clone());
                                                should_close = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                });

                clicked_value
            },
        );

        ui.memory_mut(|m| m.data.insert_persisted(open_id, open_state));

        if let Some(value) = content_result
            && let Some(val) = value
        {
            selected_value = Some(val);
            should_close = true;
        }

        final_response = trigger_resp;

        if should_close {
            ui.memory_mut(|m| m.data.insert_persisted(open_id, false));
            *props.search_value = String::new();
        }
    }

    if selected_value != *props.value {
        *props.value = selected_value.clone();
        if let Some(ref mut cb) = props.on_value_change {
            cb(selected_value);
        }
    }

    final_response
}

pub fn combobox_with_props<Id>(
    ui: &mut Ui,
    theme: &Theme,
    mut props: ComboboxProps<'_, Id>,
) -> Response
where
    Id: Hash + Debug,
{
    trace!(
        "Rendering combobox size={:?} variant={:?} disabled={} value={:?}",
        props.size, props.variant, props.disabled, props.value
    );

    let id = ui.make_persistent_id(&props.id_source);
    let open_id = id.with("open");
    let focus_applied_id = id.with("search_focus_applied");
    let active_index_id = id.with("active_index");

    let width = props.width.unwrap_or(200.0);

    let mut open_state = ui
        .ctx()
        .memory_mut(|m| m.data.get_persisted::<bool>(open_id).unwrap_or(false));
    let was_open = open_state;

    let mut focus_applied = ui.ctx().memory_mut(|m| {
        m.data
            .get_persisted::<bool>(focus_applied_id)
            .unwrap_or(false)
    });

    let selected_label = get_selected_label(props.items, props.value);
    let trigger_text = selected_label.unwrap_or_else(|| props.placeholder.to_string());
    let trigger_widget: egui::WidgetText = if props.value.is_some() {
        trigger_text.into()
    } else {
        RichText::new(trigger_text)
            .color(theme.palette.muted_foreground)
            .into()
    };

    let button_size: ButtonSize = props.size.into();
    let chevrons_icon = |p: &egui::Painter, center: egui::Pos2, size: f32, color: Color32| {
        let muted = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 128);
        icon_chevrons_up_down(p, center, size, muted);
    };

    let (trigger_resp, selection_result) = popover(
        ui,
        theme,
        PopoverProps::new(id.with("popover"), &mut open_state)
            .side(PopoverSide::Bottom)
            .align(PopoverAlign::Start)
            .with_width(width)
            .with_max_height(320.0)
            .with_content_padding(Margin::same(0))
            .with_animation(true),
        |ui| {
            Button::new(trigger_widget)
                .variant(props.trigger_variant)
                .size(button_size)
                .justify(props.trigger_justify)
                .trailing_icon(&chevrons_icon)
                .min_width(width)
                .enabled(!props.disabled)
                .show(ui, theme)
        },
        |ui| {
            if props.disabled {
                return None;
            }

            ui.set_width(width);
            ui.spacing_mut().item_spacing.y = 8.0;

            let input_size: InputSize = props.size.into();
            let select_size: SelectSize = props.size.into();
            let search_id = id.with("search");

            let _ = text_input_with_props(
                ui,
                theme,
                InputProps::new(search_id, props.search_value)
                    .placeholder(props.search_placeholder)
                    .variant(props.variant)
                    .size(input_size)
                    .enabled(true)
                    .width(width),
            );

            let just_opened = !focus_applied;
            if just_opened {
                ui.memory_mut(|m| m.request_focus(search_id));
                focus_applied = true;
            }

            let filtered_items = filter_items(props.items, props.search_value);
            let mut flat = Vec::new();
            flatten_options(&filtered_items, &mut flat);
            let enabled: Vec<&FlatOption> = flat.iter().filter(|opt| !opt.disabled).collect();

            let mut active = ui
                .ctx()
                .memory_mut(|m| m.data.get_persisted::<usize>(active_index_id).unwrap_or(0));
            if active >= enabled.len() {
                active = enabled.len().saturating_sub(1);
            }

            if just_opened
                && let Some(current) = props.value.as_deref()
                && let Some(pos) = enabled.iter().position(|opt| opt.value == current)
            {
                active = pos;
            }

            ui.ctx().input_mut(|i| {
                if i.consume_key(Modifiers::NONE, Key::ArrowDown) && !enabled.is_empty() {
                    active = (active + 1).min(enabled.len() - 1);
                }
                if i.consume_key(Modifiers::NONE, Key::ArrowUp) && !enabled.is_empty() {
                    active = active.saturating_sub(1);
                }
            });

            let mut enter_pressed = false;
            ui.ctx()
                .input_mut(|i| enter_pressed = i.consume_key(Modifiers::NONE, Key::Enter));

            let mut chosen: Option<String> = None;

            if enabled.is_empty() {
                ui.label(
                    RichText::new(props.empty_text)
                        .color(theme.palette.muted_foreground)
                        .size(input_size.font_size()),
                );
            } else {
                let item_height = select_size.item_height();
                let padding_left = 10.0;
                let right_gutter = 18.0;

                ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                    ui.set_width(width);
                    let mut enabled_pos = 0usize;

                    for item in &filtered_items {
                        match item {
                            SelectItem::Label(text) => {
                                ui.label(
                                    RichText::new(text.clone())
                                        .color(theme.palette.muted_foreground)
                                        .size(select_size.font_size() * 0.9),
                                );
                            }
                            SelectItem::Separator => {
                                let available = ui.available_rect_before_wrap();
                                let cursor = ui.cursor();
                                let sep_rect = Rect::from_min_size(
                                    pos2(available.left(), cursor.max.y),
                                    vec2(width, 1.0),
                                );
                                ui.painter().rect_filled(
                                    sep_rect,
                                    CornerRadius::same(0),
                                    theme.palette.border,
                                );
                                ui.allocate_space(vec2(width, 8.0));
                            }
                            SelectItem::Group { label, items } => {
                                ui.label(
                                    RichText::new(label.clone())
                                        .color(theme.palette.muted_foreground)
                                        .size(select_size.font_size() * 0.9),
                                );
                                for nested in items {
                                    if let SelectItem::Option {
                                        value,
                                        label,
                                        disabled,
                                        ..
                                    } = nested
                                    {
                                        let (_id, item_rect) =
                                            ui.allocate_space(vec2(width, item_height));
                                        let resp =
                                            ui.allocate_response(item_rect.size(), Sense::click());

                                        let mut is_active = false;
                                        if !*disabled {
                                            is_active = enabled_pos == active;
                                            enabled_pos += 1;
                                        }

                                        let is_hovered =
                                            item_rect.contains(ui.input(|i| {
                                                i.pointer.hover_pos().unwrap_or_default()
                                            }));
                                        let is_selected = props.value.as_deref() == Some(value);

                                        let bg_color = if *disabled {
                                            Color32::TRANSPARENT
                                        } else if is_active || is_hovered {
                                            theme.palette.muted
                                        } else {
                                            Color32::TRANSPARENT
                                        };

                                        let text_color = if *disabled {
                                            theme.palette.muted_foreground
                                        } else {
                                            theme.palette.foreground
                                        };

                                        let painter = ui.painter();
                                        painter.rect_filled(
                                            item_rect,
                                            CornerRadius::same(6),
                                            bg_color,
                                        );

                                        let galley = painter.layout_no_wrap(
                                            label.clone(),
                                            FontId::proportional(select_size.font_size()),
                                            text_color,
                                        );
                                        let text_pos = pos2(
                                            item_rect.left() + padding_left,
                                            item_rect.center().y - galley.size().y * 0.5,
                                        );
                                        painter.galley(text_pos, galley, Color32::TRANSPARENT);

                                        if is_selected {
                                            let icon_center = pos2(
                                                item_rect.right() - right_gutter,
                                                item_rect.center().y,
                                            );
                                            icon_check(
                                                painter,
                                                icon_center,
                                                select_size.font_size() * 1.1,
                                                theme.palette.foreground,
                                            );
                                        }

                                        if resp.clicked() && !*disabled {
                                            chosen = Some(value.clone());
                                        }
                                    }
                                }
                            }
                            SelectItem::Option {
                                value,
                                label,
                                disabled,
                                ..
                            } => {
                                let (_id, item_rect) = ui.allocate_space(vec2(width, item_height));
                                let resp = ui.allocate_response(item_rect.size(), Sense::click());

                                let mut is_active = false;
                                if !*disabled {
                                    is_active = enabled_pos == active;
                                    enabled_pos += 1;
                                }

                                let is_hovered = item_rect.contains(
                                    ui.input(|i| i.pointer.hover_pos().unwrap_or_default()),
                                );
                                let is_selected = props.value.as_deref() == Some(value);

                                let bg_color = if *disabled {
                                    Color32::TRANSPARENT
                                } else if is_active || is_hovered {
                                    theme.palette.muted
                                } else {
                                    Color32::TRANSPARENT
                                };

                                let text_color = if *disabled {
                                    theme.palette.muted_foreground
                                } else {
                                    theme.palette.foreground
                                };

                                let painter = ui.painter();
                                painter.rect_filled(item_rect, CornerRadius::same(6), bg_color);

                                let galley = painter.layout_no_wrap(
                                    label.clone(),
                                    FontId::proportional(select_size.font_size()),
                                    text_color,
                                );
                                let text_pos = pos2(
                                    item_rect.left() + padding_left,
                                    item_rect.center().y - galley.size().y * 0.5,
                                );
                                painter.galley(text_pos, galley, Color32::TRANSPARENT);

                                if is_selected {
                                    let icon_center = pos2(
                                        item_rect.right() - right_gutter,
                                        item_rect.center().y,
                                    );
                                    icon_check(
                                        painter,
                                        icon_center,
                                        select_size.font_size() * 1.1,
                                        theme.palette.foreground,
                                    );
                                }

                                if resp.clicked() && !*disabled {
                                    chosen = Some(value.clone());
                                }
                            }
                        }
                    }
                });
            }

            if enter_pressed && !enabled.is_empty() {
                chosen = Some(enabled[active].value.clone());
            }

            ui.ctx()
                .memory_mut(|m| m.data.insert_persisted(active_index_id, active));

            chosen
        },
    );

    ui.memory_mut(|m| m.data.insert_persisted(open_id, open_state));
    ui.memory_mut(|m| m.data.insert_persisted(focus_applied_id, focus_applied));

    if was_open && !open_state {
        ui.memory_mut(|m| m.data.insert_persisted(focus_applied_id, false));
        *props.search_value = String::new();
    }

    let mut selected_value = props.value.clone();
    let mut should_close = false;

    if let Some(Some(chosen)) = selection_result {
        if props.value.as_ref() == Some(&chosen) {
            selected_value = None;
        } else {
            selected_value = Some(chosen);
        }
        should_close = true;
    }

    if should_close {
        ui.memory_mut(|m| m.data.insert_persisted(open_id, false));
        ui.memory_mut(|m| m.data.insert_persisted(focus_applied_id, false));
        *props.search_value = String::new();
    }

    if selected_value != *props.value {
        *props.value = selected_value.clone();
        if let Some(ref mut cb) = props.on_value_change {
            cb(selected_value);
        }
    }

    trigger_resp
}

pub fn combobox<Id>(
    ui: &mut Ui,
    theme: &Theme,
    id_source: Id,
    value: &mut Option<String>,
    items: &[SelectItem],
) -> Response
where
    Id: Hash + Debug,
{
    let mut search = String::new();
    combobox_with_props(
        ui,
        theme,
        ComboboxProps::new(id_source, value, items, &mut search),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::select::SelectItem;

    #[test]
    fn get_selected_label_searches_nested_groups() {
        let items = vec![SelectItem::group(
            "Group",
            vec![SelectItem::option("a", "Alpha")],
        )];
        let label = get_selected_label(&items, &Some("a".to_string()));
        assert_eq!(label, Some("Alpha".to_string()));
    }

    #[test]
    fn filter_items_matches_text_value_and_returns_group() {
        let items = vec![
            SelectItem::option("a", "Alpha"),
            SelectItem::group(
                "Group",
                vec![SelectItem::option_with_text_value(
                    "b",
                    "Beta",
                    "Beta Extra",
                )],
            ),
        ];
        let filtered = filter_items(&items, "extra");
        assert_eq!(filtered.len(), 1);
        match &filtered[0] {
            SelectItem::Group { label, items } => {
                assert_eq!(label, "Group");
                assert_eq!(items.len(), 1);
            }
            _ => panic!("Expected group"),
        }
    }

    #[test]
    fn flatten_options_collects_only_options() {
        let items = vec![
            SelectItem::label("Label"),
            SelectItem::separator(),
            SelectItem::option("a", "Alpha"),
            SelectItem::group("Group", vec![SelectItem::option_disabled("b", "Beta")]),
        ];
        let mut out = Vec::new();
        flatten_options(&items, &mut out);
        assert_eq!(
            out,
            vec![
                FlatOption {
                    value: "a".to_string(),
                    label: "Alpha".to_string(),
                    disabled: false,
                },
                FlatOption {
                    value: "b".to_string(),
                    label: "Beta".to_string(),
                    disabled: true,
                },
            ]
        );
    }
}
