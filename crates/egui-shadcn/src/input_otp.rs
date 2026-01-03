use crate::theme::Theme;
use egui::{
    Color32, CornerRadius, Event, Id, Key, Response, Sense, Stroke, StrokeKind, Ui, Vec2, vec2,
};
use regex::Regex;
use std::fmt::{self, Debug};

pub struct InputOTPOnComplete<'a>(pub Box<dyn FnMut(&str) + 'a>);

impl<'a> Debug for InputOTPOnComplete<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InputOTPOnComplete").finish()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InputOTPState {
    pub cursor: usize,
    pub focused: bool,
}

#[derive(Debug)]
pub struct InputOTPProps<'a> {
    pub max_length: usize,
    pub pattern: Option<&'a Regex>,
    pub on_complete: Option<InputOTPOnComplete<'a>>,
}

impl<'a> Default for InputOTPProps<'a> {
    fn default() -> Self {
        Self {
            max_length: 6,
            pattern: None,
            on_complete: None,
        }
    }
}

impl<'a> InputOTPProps<'a> {
    pub fn new(max_length: usize) -> Self {
        Self {
            max_length,
            pattern: None,
            on_complete: None,
        }
    }

    pub fn pattern(mut self, pattern: &'a Regex) -> Self {
        self.pattern = Some(pattern);
        self
    }

    pub fn on_complete(mut self, callback: impl FnMut(&str) + 'a) -> Self {
        self.on_complete = Some(InputOTPOnComplete(Box::new(callback)));
        self
    }
}

pub struct InputOTPContext<'a> {
    pub id: Id,
    pub value: &'a str,
    pub max_length: usize,
    pub cursor: usize,
    pub focused: bool,
    pub theme: &'a Theme,
    pub enabled: bool,
    group_slot_count: std::cell::Cell<usize>,
    group_first_index: std::cell::Cell<Option<usize>>,
}

pub fn input_otp(
    ui: &mut Ui,
    theme: &Theme,
    value: &mut String,
    mut props: InputOTPProps<'_>,
    add_contents: impl FnOnce(&mut Ui, &InputOTPContext),
) -> Response {
    let id = ui.next_auto_id();
    let mut state = ui
        .ctx()
        .data(|d| d.get_temp::<InputOTPState>(id))
        .unwrap_or_default();

    let (mut chars, normalized) = normalized_chars(value, props.max_length, props.pattern);
    if normalized {
        *value = chars.iter().collect();
    }

    let mut cursor = state.cursor.min(props.max_length).min(chars.len());
    let mut changed = normalized;

    if state.focused {
        let events = ui.input(|i| i.events.clone());
        for event in events {
            match event {
                Event::Text(text) | Event::Paste(text) => {
                    if !text.is_empty()
                        && apply_text(
                            &mut chars,
                            &mut cursor,
                            &text,
                            props.max_length,
                            props.pattern,
                        )
                    {
                        changed = true;
                    }
                }
                Event::Key {
                    key, pressed: true, ..
                } => match key {
                    Key::Backspace => {
                        if cursor > 0 {
                            cursor -= 1;
                            chars.remove(cursor);
                            changed = true;
                        }
                    }
                    Key::Delete => {
                        if cursor < chars.len() {
                            chars.remove(cursor);
                            changed = true;
                        }
                    }
                    Key::ArrowLeft => {
                        cursor = cursor.saturating_sub(1);
                    }
                    Key::ArrowRight => {
                        if cursor < chars.len().min(props.max_length) {
                            cursor += 1;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    cursor = cursor.min(props.max_length);
    state.cursor = cursor;

    if changed {
        let updated = chars.iter().collect::<String>();
        if *value != updated {
            *value = updated;
        }
    }

    if changed
        && chars.len() == props.max_length
        && let Some(cb) = props.on_complete.as_mut()
    {
        (cb.0)(value.as_str());
    }

    ui.ctx().data_mut(|d| d.insert_temp(id, state));

    let context = InputOTPContext {
        id,
        value,
        max_length: props.max_length,
        cursor: state.cursor,
        focused: state.focused,
        theme,
        enabled: ui.is_enabled(),
        group_slot_count: std::cell::Cell::new(0),
        group_first_index: std::cell::Cell::new(None),
    };

    let inner = ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.spacing_mut().item_spacing = vec2(2.0, 0.0);
        add_contents(ui, &context);
    });

    let response = ui.interact(inner.response.rect, id, Sense::click());
    if response.clicked() {
        ui.memory_mut(|m| m.request_focus(id));
        ui.ctx().data_mut(|d| {
            let mut state = d.get_temp::<InputOTPState>(id).unwrap_or_default();
            state.focused = true;
            d.insert_temp(id, state);
        });
    }

    if response.clicked_elsewhere() || response.lost_focus() {
        ui.ctx().data_mut(|d| {
            let mut state = d.get_temp::<InputOTPState>(id).unwrap_or_default();
            state.focused = false;
            d.insert_temp(id, state);
        });
    }

    if changed {
        ui.ctx().request_repaint();
    }

    response
}

pub fn input_otp_group(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) -> Response {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(-1.0, 0.0);
        add_contents(ui);
    })
    .response
}

pub fn input_otp_slot(ui: &mut Ui, context: &InputOTPContext, index: usize) -> Response {
    input_otp_slot_impl(ui, context, index, false)
}

pub fn input_otp_slot_last(ui: &mut Ui, context: &InputOTPContext, index: usize) -> Response {
    let result = input_otp_slot_impl(ui, context, index, true);
    // Сбрасываем счетчик после последнего слота в группе
    context.group_slot_count.set(0);
    context.group_first_index.set(None);
    result
}

fn input_otp_slot_impl(
    ui: &mut Ui,
    context: &InputOTPContext,
    index: usize,
    is_last_in_group: bool,
) -> Response {
    // Трекинг позиции внутри текущей группы
    let group_position = context.group_slot_count.get();
    context.group_slot_count.set(group_position + 1);

    if context.group_first_index.get().is_none() {
        context.group_first_index.set(Some(index));
    }

    let size = Vec2::splat(36.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    if response.clicked() {
        ui.ctx().data_mut(|d| {
            let mut state = d.get_temp::<InputOTPState>(context.id).unwrap_or_default();
            state.focused = true;
            state.cursor = index.min(context.value.chars().count());
            d.insert_temp(context.id, state);
        });
        ui.memory_mut(|m| m.request_focus(context.id));
    }

    let is_active = context.focused && context.cursor == index;
    let slot_char = context.value.chars().nth(index);

    let palette = &context.theme.palette;
    // Используем позицию внутри группы и флаг is_last
    let rounding = corner_radius_for_slot_in_group(context.theme, group_position, is_last_in_group);

    let mut border_color = if is_active {
        palette.ring
    } else {
        palette.input
    };
    let mut bg = palette.background;
    let mut text_color = palette.foreground;
    let mut caret_color = palette.foreground;

    if !context.enabled {
        let factor = ui.visuals().disabled_alpha;
        border_color = with_alpha(border_color, factor);
        bg = with_alpha(bg, factor);
        text_color = with_alpha(text_color, factor);
        caret_color = with_alpha(caret_color, factor);
    }

    let painter = ui.painter();
    painter.rect_filled(rect, rounding, bg);

    let border_width = if is_active { 2.0 } else { 1.0 };
    painter.rect_stroke(
        rect,
        rounding,
        Stroke::new(border_width, border_color),
        StrokeKind::Inside,
    );

    if is_active {
        let ring_color = Color32::from_rgba_unmultiplied(
            border_color.r(),
            border_color.g(),
            border_color.b(),
            96,
        );
        painter.rect_stroke(
            rect,
            rounding,
            Stroke::new(3.0, ring_color),
            StrokeKind::Outside,
        );
    }

    if let Some(ch) = slot_char {
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            ch.to_string(),
            egui::FontId::proportional(14.0),
            text_color,
        );
    }

    if is_active && context.focused {
        let time = ui.input(|i| i.time);
        if (time * 2.0).fract() < 0.5 {
            painter.rect_filled(
                egui::Rect::from_center_size(rect.center(), Vec2::new(1.5, 20.0)),
                CornerRadius::same(0),
                caret_color,
            );
        }
    }

    response
}

pub fn input_otp_separator(ui: &mut Ui, theme: &Theme) -> Response {
    ui.label(egui::RichText::new("-").color(theme.palette.muted_foreground))
}

fn is_allowed_char(pattern: Option<&Regex>, ch: char) -> bool {
    if ch.is_control() {
        return false;
    }
    match pattern {
        Some(pattern) => {
            let mut buffer = [0u8; 4];
            pattern.is_match(ch.encode_utf8(&mut buffer))
        }
        None => true,
    }
}

fn normalized_chars(value: &str, max_length: usize, pattern: Option<&Regex>) -> (Vec<char>, bool) {
    let mut chars = Vec::new();
    for ch in value.chars() {
        if chars.len() >= max_length {
            break;
        }
        if is_allowed_char(pattern, ch) {
            chars.push(ch);
        }
    }
    let normalized = chars.iter().collect::<String>();
    (chars, normalized != value)
}

fn apply_text(
    chars: &mut Vec<char>,
    cursor: &mut usize,
    text: &str,
    max_length: usize,
    pattern: Option<&Regex>,
) -> bool {
    let mut changed = false;
    for ch in text.chars() {
        if !is_allowed_char(pattern, ch) {
            continue;
        }
        if chars.len() >= max_length {
            break;
        }
        if *cursor < chars.len() {
            chars[*cursor] = ch;
        } else {
            chars.push(ch);
        }
        *cursor += 1;
        changed = true;
    }
    changed
}

fn corner_radius_for_slot_in_group(
    theme: &Theme,
    position_in_group: usize,
    is_last: bool,
) -> CornerRadius {
    let radius = theme.radius.r2.round() as u8;

    match (position_in_group, is_last) {
        (0, true) => {
            // Единственный слот в группе - скругляем все углы
            CornerRadius::same(radius)
        }
        (0, false) => {
            // Первый слот в группе - скругляем только левые углы
            CornerRadius {
                nw: radius,
                sw: radius,
                ne: 0,
                se: 0,
            }
        }
        (_, true) => {
            // Последний слот в группе - скругляем только правые углы
            CornerRadius {
                nw: 0,
                sw: 0,
                ne: radius,
                se: radius,
            }
        }
        _ => {
            // Средние слоты - без скруглений
            CornerRadius::same(0)
        }
    }
}

fn with_alpha(color: Color32, factor: f32) -> Color32 {
    if factor >= 1.0 {
        return color;
    }
    let alpha = (color.a() as f32 * factor).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}
