use crate::theme::Theme;
use crate::tokens::{ColorPalette, ControlSize, mix};
use egui::{
    Color32, CornerRadius, Event, FontId, Key, LayerId, Order, Painter, Pos2, Rect, Response,
    Sense, Stroke, StrokeKind, Ui, Vec2, pos2, vec2,
};
use log::trace;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SelectSize {
    Size1,

    #[default]
    Size2,

    Size3,

    Sm,

    Default,
}

impl SelectSize {
    fn canonical(self) -> Self {
        match self {
            SelectSize::Sm => SelectSize::Size2,
            SelectSize::Default => SelectSize::Size3,
            other => other,
        }
    }

    pub fn trigger_height(self) -> f32 {
        match self.canonical() {
            SelectSize::Size1 => 24.0,
            SelectSize::Size2 => 32.0,
            _ => 36.0,
        }
    }

    pub fn item_height(self) -> f32 {
        match self.canonical() {
            SelectSize::Size1 => 20.0,
            SelectSize::Size2 => 24.0,
            _ => 28.0,
        }
    }

    pub fn trigger_padding(self) -> Vec2 {
        match self.canonical() {
            SelectSize::Size1 => vec2(8.0, 4.0),
            SelectSize::Size2 => vec2(12.0, 6.0),
            _ => vec2(14.0, 8.0),
        }
    }

    pub fn font_size(self) -> f32 {
        match self.canonical() {
            SelectSize::Size1 => 12.0,
            SelectSize::Size2 => 14.0,
            _ => 16.0,
        }
    }

    pub fn icon_size(self) -> f32 {
        match self.canonical() {
            SelectSize::Size1 => 12.0,
            SelectSize::Size2 => 14.0,
            _ => 16.0,
        }
    }

    pub fn gap(self) -> f32 {
        match self.canonical() {
            SelectSize::Size1 => 4.0,
            SelectSize::Size2 => 6.0,
            _ => 8.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SelectRadius {
    None,

    Small,

    #[default]
    Medium,

    Large,

    Full,
}

impl SelectRadius {
    pub fn corner_radius(self) -> CornerRadius {
        match self {
            SelectRadius::None => CornerRadius::same(0),
            SelectRadius::Small => CornerRadius::same(2),
            SelectRadius::Medium => CornerRadius::same(4),
            SelectRadius::Large => CornerRadius::same(6),
            SelectRadius::Full => CornerRadius::same(255),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PopupPosition {
    #[default]
    Popper,

    ItemAligned,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TriggerVariant {
    #[default]
    Surface,

    Classic,

    Soft,

    Ghost,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ContentVariant {
    #[default]
    Soft,

    Solid,
}

impl From<ControlSize> for SelectSize {
    fn from(size: ControlSize) -> Self {
        match size {
            ControlSize::Sm | ControlSize::IconSm => SelectSize::Sm,
            _ => SelectSize::Default,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SelectStyle {
    pub trigger_bg: Color32,
    pub trigger_bg_hover: Color32,
    pub trigger_border: Color32,
    pub trigger_text: Color32,
    pub trigger_placeholder: Color32,
    pub trigger_icon: Color32,
    pub trigger_rounding: CornerRadius,

    pub focus_ring_color: Color32,
    pub focus_ring_width: f32,

    pub invalid_border: Color32,
    pub invalid_ring: Color32,

    pub disabled_opacity: f32,

    pub content_bg: Color32,
    pub content_border: Color32,
    pub content_rounding: CornerRadius,
    pub content_shadow: Color32,
    pub content_padding: f32,

    pub item_bg: Color32,
    pub item_bg_hover: Color32,
    pub item_bg_selected: Color32,
    pub item_text: Color32,
    pub item_text_hover: Color32,
    pub item_rounding: CornerRadius,
    pub item_padding: Vec2,
    pub item_icon_color: Color32,

    pub item_solid_bg_hover: Color32,
    pub item_solid_text_hover: Color32,
    pub item_solid_high_contrast_bg: Color32,
    pub item_solid_high_contrast_text: Color32,

    pub label_text: Color32,

    pub separator_color: Color32,

    pub scroll_button_color: Color32,
}

impl SelectStyle {
    fn base_from_palette(palette: &ColorPalette) -> Self {
        Self {
            trigger_bg: Color32::from_rgba_unmultiplied(
                palette.input.r(),
                palette.input.g(),
                palette.input.b(),
                77,
            ),
            trigger_bg_hover: Color32::from_rgba_unmultiplied(
                palette.input.r(),
                palette.input.g(),
                palette.input.b(),
                128,
            ),
            trigger_border: palette.border,
            trigger_text: palette.foreground,
            trigger_placeholder: palette.muted_foreground,
            trigger_icon: palette.muted_foreground,
            trigger_rounding: CornerRadius::same(6),

            focus_ring_color: Color32::from_rgba_unmultiplied(
                palette.border.r(),
                palette.border.g(),
                palette.border.b(),
                128,
            ),
            focus_ring_width: 3.0,

            invalid_border: palette.destructive,
            invalid_ring: Color32::from_rgba_unmultiplied(
                palette.destructive.r(),
                palette.destructive.g(),
                palette.destructive.b(),
                102,
            ),

            disabled_opacity: 0.5,

            content_bg: Color32::from_rgb(30, 30, 30),
            content_border: palette.border,
            content_rounding: CornerRadius::same(6),
            content_shadow: Color32::from_rgba_unmultiplied(0, 0, 0, 40),
            content_padding: 4.0,

            item_bg: Color32::TRANSPARENT,
            item_bg_hover: Color32::from_rgb(55, 55, 55),
            item_bg_selected: mix(Color32::from_rgb(55, 55, 55), palette.primary, 0.12),
            item_text: palette.foreground,
            item_text_hover: palette.foreground,
            item_rounding: CornerRadius::same(3),
            item_padding: vec2(8.0, 6.0),
            item_icon_color: palette.muted_foreground,

            item_solid_bg_hover: palette.primary,
            item_solid_text_hover: palette.primary_foreground,

            item_solid_high_contrast_bg: palette.foreground,
            item_solid_high_contrast_text: palette.background,

            label_text: palette.muted_foreground,

            separator_color: palette.border,

            scroll_button_color: palette.muted_foreground,
        }
    }

    fn with_accent(mut self, palette: &ColorPalette, accent: Color32) -> Self {
        let accent_tint_soft =
            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 42);
        let accent_tint_hover =
            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 56);
        let accent_border =
            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 160);

        self.trigger_bg = accent_tint_soft;
        self.trigger_bg_hover = accent_tint_hover;
        self.trigger_border = accent_border;
        self.trigger_text = accent;
        self.trigger_placeholder = mix(accent, palette.muted_foreground, 0.35);
        self.trigger_icon = accent;
        self.focus_ring_color =
            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 180);

        self.content_bg = mix(palette.input, accent, 0.15);
        self.content_border = accent_border;
        self.item_bg_hover = mix(accent, Color32::WHITE, 0.12);
        self.item_bg_selected = mix(accent, palette.background, 0.15);
        self.item_text_hover = palette.foreground;
        self.item_icon_color = mix(accent, palette.foreground, 0.15);
        self.item_solid_bg_hover = accent;
        self.item_solid_text_hover = palette.primary_foreground;

        self.separator_color = mix(accent, palette.border, 0.25);
        self.scroll_button_color = mix(accent, palette.muted_foreground, 0.2);

        self
    }

    fn with_trigger_variant(
        mut self,
        variant: TriggerVariant,
        palette: &ColorPalette,
        accent: Color32,
    ) -> Self {
        match variant {
            TriggerVariant::Surface => {}
            TriggerVariant::Classic => {
                let bg = mix(palette.input, palette.background, 0.1);
                self.trigger_bg = bg;
                self.trigger_bg_hover = mix(bg, palette.foreground, 0.08);
                self.trigger_border = mix(palette.border, palette.foreground, 0.25);
                self.trigger_text = palette.foreground;
                self.focus_ring_color = mix(palette.primary, palette.foreground, 0.35);
            }
            TriggerVariant::Soft => {
                let tint = mix(accent, palette.background, 0.85);
                self.trigger_bg = tint;
                self.trigger_bg_hover = mix(tint, accent, 0.22);
                self.trigger_border = Color32::TRANSPARENT;
                self.trigger_text = accent;
                self.trigger_placeholder = mix(accent, palette.muted_foreground, 0.4);
                self.trigger_icon = accent;
                self.focus_ring_color = mix(accent, palette.foreground, 0.35);
            }
            TriggerVariant::Ghost => {
                self.trigger_bg = Color32::TRANSPARENT;
                self.trigger_bg_hover = mix(palette.muted, palette.background, 0.5);
                self.trigger_border = Color32::TRANSPARENT;
                self.trigger_text = mix(accent, palette.foreground, 0.6);
                self.trigger_placeholder = mix(self.trigger_text, palette.muted_foreground, 0.5);
                self.trigger_icon = self.trigger_text;
                self.focus_ring_color = mix(accent, palette.foreground, 0.4);
            }
        }
        self
    }

    fn with_content_variant(
        mut self,
        variant: ContentVariant,
        palette: &ColorPalette,
        accent: Color32,
    ) -> Self {
        match variant {
            ContentVariant::Soft => {
                let tinted = mix(self.item_bg_hover, accent, 0.25);
                self.item_bg_selected =
                    Color32::from_rgba_unmultiplied(tinted.r(), tinted.g(), tinted.b(), 80);
            }
            ContentVariant::Solid => {
                self.content_bg = mix(palette.input, accent, 0.12);
                self.content_border = mix(palette.border, accent, 0.25);
                self.item_bg_hover = self.item_solid_bg_hover;
                let solid_selected = mix(self.item_solid_bg_hover, accent, 0.2);
                self.item_bg_selected = Color32::from_rgba_unmultiplied(
                    solid_selected.r(),
                    solid_selected.g(),
                    solid_selected.b(),
                    200,
                );
                self.item_text_hover = self.item_solid_text_hover;
            }
        }
        self
    }

    pub fn from_palette(palette: &ColorPalette) -> Self {
        Self::from_palette_for_variants(
            palette,
            TriggerVariant::Surface,
            ContentVariant::Soft,
            None,
        )
    }

    pub fn from_palette_for_variants(
        palette: &ColorPalette,
        trigger_variant: TriggerVariant,
        content_variant: ContentVariant,
        accent: Option<Color32>,
    ) -> Self {
        let mut style = Self::base_from_palette(palette);
        let effective_accent = accent.unwrap_or(palette.accent);
        if accent.is_some() {
            style = style.with_accent(palette, effective_accent);
        }
        style = style.with_trigger_variant(trigger_variant, palette, effective_accent);
        style.with_content_variant(content_variant, palette, effective_accent)
    }

    pub fn from_palette_with_accent(palette: &ColorPalette, accent: Color32) -> Self {
        Self::from_palette_for_variants(
            palette,
            TriggerVariant::Surface,
            ContentVariant::Soft,
            Some(accent),
        )
    }

    pub fn with_high_contrast(mut self, palette: &ColorPalette) -> Self {
        self.trigger_bg = mix(self.trigger_bg, palette.foreground, 0.08);
        self.trigger_bg_hover = mix(self.trigger_bg_hover, palette.foreground, 0.12);
        self.trigger_text = palette.foreground;
        self.trigger_icon = palette.foreground;
        self.content_bg = mix(self.content_bg, palette.foreground, 0.06);
        self.content_border = mix(self.content_border, palette.foreground, 0.2);
        self.item_bg_hover = mix(self.item_bg_hover, palette.foreground, 0.1);
        self.item_bg_selected = mix(self.item_bg_selected, palette.foreground, 0.15);
        self.item_text_hover = palette.foreground;
        self
    }
}

impl Default for SelectStyle {
    fn default() -> Self {
        Self::from_palette(&ColorPalette::default())
    }
}

#[derive(Clone, Debug)]
pub enum SelectItem {
    Option {
        value: String,
        label: String,
        disabled: bool,
    },

    Group {
        label: String,
        items: Vec<SelectItem>,
    },

    Separator,

    Label(String),
}

impl SelectItem {
    pub fn option(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self::Option {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn option_disabled(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self::Option {
            value: value.into(),
            label: label.into(),
            disabled: true,
        }
    }

    pub fn group(label: impl Into<String>, items: Vec<SelectItem>) -> Self {
        Self::Group {
            label: label.into(),
            items,
        }
    }

    pub fn separator() -> Self {
        Self::Separator
    }

    pub fn label(text: impl Into<String>) -> Self {
        Self::Label(text.into())
    }
}

#[derive(Debug)]
pub struct SelectProps<'a, Id>
where
    Id: Hash + Debug,
{
    pub id_source: Id,

    pub selected: &'a mut Option<String>,

    pub placeholder: &'a str,

    pub size: SelectSize,

    pub trigger_variant: TriggerVariant,

    pub content_variant: ContentVariant,

    pub enabled: bool,

    pub is_invalid: bool,

    pub width: Option<f32>,

    pub style: Option<SelectStyle>,

    pub accent_color: Option<Color32>,

    pub radius: SelectRadius,

    pub high_contrast: bool,

    pub position: PopupPosition,
}

impl<'a, Id: Hash + Debug> SelectProps<'a, Id> {
    pub fn new(id_source: Id, selected: &'a mut Option<String>) -> Self {
        Self {
            id_source,
            selected,
            placeholder: "Select...",
            size: SelectSize::Size2,
            trigger_variant: TriggerVariant::Surface,
            content_variant: ContentVariant::Soft,
            enabled: true,
            is_invalid: false,
            width: None,
            style: None,
            accent_color: None,
            radius: SelectRadius::Medium,
            high_contrast: false,
            position: PopupPosition::Popper,
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn size(mut self, size: SelectSize) -> Self {
        self.size = size;
        self
    }

    pub fn trigger_variant(mut self, variant: TriggerVariant) -> Self {
        self.trigger_variant = variant;
        self
    }

    pub fn content_variant(mut self, variant: ContentVariant) -> Self {
        self.content_variant = variant;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn invalid(mut self, is_invalid: bool) -> Self {
        self.is_invalid = is_invalid;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn style(mut self, style: SelectStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn accent_color(mut self, color: Color32) -> Self {
        self.accent_color = Some(color);
        self
    }

    pub fn radius(mut self, radius: SelectRadius) -> Self {
        self.radius = radius;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn position(mut self, position: PopupPosition) -> Self {
        self.position = position;
        self
    }
}

#[derive(Debug)]
pub struct SelectPropsSimple<'a, Id>
where
    Id: Hash + Debug,
{
    pub id_source: Id,
    pub selected: &'a mut Option<String>,
    pub options: &'a [String],
    pub placeholder: &'a str,
    pub size: ControlSize,
    pub enabled: bool,
    pub is_invalid: bool,
}

#[derive(Clone, Debug, Default)]
struct SelectState {
    is_open: bool,

    focused_index: Option<usize>,

    scroll_offset: f32,

    show_scroll_up: bool,

    show_scroll_down: bool,

    typed_buffer: String,

    last_type_time: f64,
}

fn draw_chevron_down(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    let half = size * 0.35;
    let stroke = Stroke::new(1.5, color);

    painter.line_segment(
        [
            pos2(center.x - half, center.y - half * 0.5),
            pos2(center.x, center.y + half * 0.5),
        ],
        stroke,
    );
    painter.line_segment(
        [
            pos2(center.x, center.y + half * 0.5),
            pos2(center.x + half, center.y - half * 0.5),
        ],
        stroke,
    );
}

fn draw_chevron_up(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    let half = size * 0.35;
    let stroke = Stroke::new(1.5, color);

    painter.line_segment(
        [
            pos2(center.x - half, center.y + half * 0.5),
            pos2(center.x, center.y - half * 0.5),
        ],
        stroke,
    );
    painter.line_segment(
        [
            pos2(center.x, center.y - half * 0.5),
            pos2(center.x + half, center.y + half * 0.5),
        ],
        stroke,
    );
}

fn draw_check_icon(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    let stroke = Stroke::new(2.0, color);

    let s = size * 0.4;
    painter.line_segment(
        [
            pos2(center.x - s * 0.6, center.y),
            pos2(center.x - s * 0.1, center.y + s * 0.5),
        ],
        stroke,
    );
    painter.line_segment(
        [
            pos2(center.x - s * 0.1, center.y + s * 0.5),
            pos2(center.x + s * 0.6, center.y - s * 0.4),
        ],
        stroke,
    );
}

pub fn select_with_items<Id>(
    ui: &mut Ui,
    theme: &Theme,
    props: SelectProps<'_, Id>,
    items: &[SelectItem],
) -> Response
where
    Id: Hash + Debug,
{
    let style = props.style.clone().unwrap_or_else(|| {
        SelectStyle::from_palette_for_variants(
            &theme.palette,
            props.trigger_variant,
            props.content_variant,
            props.accent_color,
        )
    });
    let style = if props.high_contrast {
        style.with_high_contrast(&theme.palette)
    } else {
        style
    };
    let id = ui.make_persistent_id(&props.id_source);

    trace!(
        "Rendering select size={:?} enabled={} items={}",
        props.size,
        props.enabled,
        items.len()
    );

    let mut state = ui
        .ctx()
        .data_mut(|d| d.get_temp::<SelectState>(id).unwrap_or_default());

    let trigger_height = props.size.trigger_height();
    let trigger_width = props.width.unwrap_or(180.0);
    let icon_size = props.size.icon_size();

    let desired_size = vec2(trigger_width, trigger_height);
    let (trigger_rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

    if response.clicked() && props.enabled {
        state.is_open = !state.is_open;
        if state.is_open {
            let item_height = style.item_padding.y * 2.0 + props.size.font_size();
            let separator_height = 9.0;
            let label_height = style.item_padding.y * 2.0 + 12.0;

            if let Some(selected_value) = props.selected.as_ref() {
                let flat_options = flatten_options(items);
                state.focused_index = flat_options
                    .iter()
                    .position(|(value, _, _)| value == selected_value);

                if let Some((offset, _item_h)) = calculate_selected_offset(
                    items,
                    selected_value,
                    item_height,
                    separator_height,
                    label_height,
                ) {
                    let visible_height = 300.0 - style.content_padding * 2.0 - 36.0;
                    state.scroll_offset =
                        (offset - visible_height / 2.0 + item_height / 2.0).max(0.0);
                } else {
                    state.scroll_offset = 0.0;
                }
            } else {
                state.focused_index = None;
                state.scroll_offset = 0.0;
            }
        }
        response.mark_changed();
    }

    if response.has_focus() && props.enabled {
        let input = ui.input(|i| {
            (
                i.key_pressed(Key::Enter) || i.key_pressed(Key::Space),
                i.key_pressed(Key::Escape),
                i.key_pressed(Key::ArrowDown),
                i.key_pressed(Key::ArrowUp),
            )
        });

        if input.0 && !state.is_open {
            state.is_open = true;
            state.focused_index = None;
        } else if input.1 && state.is_open {
            state.is_open = false;
        }
    }

    let anim_t = ui.ctx().animate_bool_with_time_and_easing(
        id.with("open"),
        state.is_open,
        theme.motion.base_ms / 1000.0,
        crate::tokens::ease_out_cubic,
    );

    let painter = ui.painter();

    let bg_color = if !props.enabled {
        mix(
            style.trigger_bg,
            Color32::TRANSPARENT,
            style.disabled_opacity,
        )
    } else if response.hovered() && !state.is_open {
        style.trigger_bg_hover
    } else {
        style.trigger_bg
    };

    let border_color = if props.is_invalid {
        style.invalid_border
    } else if response.has_focus() || state.is_open {
        style.focus_ring_color
    } else {
        style.trigger_border
    };

    painter.rect_filled(trigger_rect, style.trigger_rounding, bg_color);
    painter.rect_stroke(
        trigger_rect,
        style.trigger_rounding,
        Stroke::new(1.0, border_color),
        StrokeKind::Inside,
    );

    if (response.has_focus() || state.is_open) && props.enabled {
        let ring_rect = trigger_rect.expand(style.focus_ring_width * 0.5);
        let ring_color = if props.is_invalid {
            style.invalid_ring
        } else {
            style.focus_ring_color
        };
        painter.rect_stroke(
            ring_rect,
            style.trigger_rounding,
            Stroke::new(style.focus_ring_width, ring_color),
            StrokeKind::Outside,
        );
    }

    let text_rect = trigger_rect.shrink2(vec2(style.content_padding * 3.0, 0.0));
    let text_color = if !props.enabled {
        mix(
            style.trigger_text,
            Color32::TRANSPARENT,
            style.disabled_opacity,
        )
    } else if props.selected.is_some() {
        style.trigger_text
    } else {
        style.trigger_placeholder
    };

    let display_text = if let Some(selected_value) = props.selected.as_ref() {
        find_label_for_value(items, selected_value).unwrap_or_else(|| selected_value.clone())
    } else {
        props.placeholder.to_string()
    };

    let galley = painter.layout_no_wrap(
        display_text,
        FontId::proportional(props.size.font_size()),
        text_color,
    );
    let text_pos = pos2(
        text_rect.left(),
        trigger_rect.center().y - galley.size().y * 0.5,
    );
    painter.galley(text_pos, galley, Color32::TRANSPARENT);

    let icon_center = pos2(
        trigger_rect.right() - icon_size * 0.75 - style.content_padding,
        trigger_rect.center().y,
    );
    let icon_color = if !props.enabled {
        mix(
            style.trigger_icon,
            Color32::TRANSPARENT,
            style.disabled_opacity,
        )
    } else {
        style.trigger_icon
    };
    draw_chevron_down(painter, icon_center, icon_size, icon_color);

    if anim_t > 0.0 {
        let popup_id = id.with("popup");
        let layer_id = LayerId::new(Order::Foreground, popup_id);

        let flat_options = flatten_options(items);

        let item_height = style.item_padding.y * 2.0 + props.size.font_size();
        let separator_height = 9.0;
        let label_height = style.item_padding.y * 2.0 + 12.0;

        let content_height =
            calculate_content_height(items, item_height, separator_height, label_height);
        let max_popup_height = 300.0;
        let popup_height = content_height.min(max_popup_height) + style.content_padding * 2.0;
        let popup_width = trigger_width.max(128.0);

        let popup_pos = pos2(trigger_rect.left(), trigger_rect.bottom() + 4.0);
        let popup_rect = Rect::from_min_size(popup_pos, vec2(popup_width, popup_height));

        let scale = 0.95 + 0.05 * anim_t;
        let alpha = (anim_t * 255.0) as u8;

        let animated_rect = Rect::from_center_size(popup_rect.center(), popup_rect.size() * scale);

        let pointer_pos = ui.input(|i| i.pointer.interact_pos());
        if let Some(pos) = pointer_pos
            && state.is_open
            && ui.input(|i| i.pointer.any_click())
            && !animated_rect.contains(pos)
            && !trigger_rect.contains(pos)
        {
            state.is_open = false;
        }

        if state.is_open {
            let input = ui.input(|i| {
                (
                    i.key_pressed(Key::ArrowDown),
                    i.key_pressed(Key::ArrowUp),
                    i.key_pressed(Key::Enter),
                    i.key_pressed(Key::Escape),
                )
            });

            let now = ui.input(|i| i.time);
            let mut typed = String::new();
            let events: Vec<Event> = ui.input(|i| i.events.clone());
            for event in events {
                if let Event::Text(text) = event
                    && !text.is_empty()
                    && !text.chars().any(|c| c.is_control())
                {
                    typed.push_str(&text);
                }
            }

            if now - state.last_type_time > 0.8 {
                state.typed_buffer.clear();
            }

            if !typed.is_empty() {
                state.typed_buffer.push_str(&typed);
                state.last_type_time = now;
                if let Some(idx) = find_typeahead_match(items, &state.typed_buffer) {
                    state.focused_index = Some(idx);
                }
            }

            if input.0 {
                state.focused_index = Some(
                    state
                        .focused_index
                        .map(|i| (i + 1).min(flat_options.len().saturating_sub(1)))
                        .unwrap_or(0),
                );
            }
            if input.1 {
                state.focused_index = state.focused_index.map(|i| i.saturating_sub(1)).or(Some(0));
            }
            if input.2
                && let Some(idx) = state.focused_index
                && let Some((value, _, disabled)) = flat_options.get(idx)
                && !disabled
            {
                *props.selected = Some(value.clone());
                state.is_open = false;
                response.mark_changed();
            }
            if input.3 {
                state.is_open = false;
            }
        }

        let popup_painter = ui.ctx().layer_painter(layer_id);

        let content_painter = popup_painter.with_clip_rect(animated_rect);

        let shadow_rect = animated_rect.translate(vec2(0.0, 2.0));
        popup_painter.rect_filled(
            shadow_rect,
            style.content_rounding,
            Color32::from_rgba_unmultiplied(
                style.content_shadow.r(),
                style.content_shadow.g(),
                style.content_shadow.b(),
                (style.content_shadow.a() as f32 * anim_t) as u8,
            ),
        );

        let bg_with_alpha = Color32::from_rgba_unmultiplied(
            style.content_bg.r(),
            style.content_bg.g(),
            style.content_bg.b(),
            alpha,
        );
        content_painter.rect_filled(animated_rect, style.content_rounding, bg_with_alpha);
        content_painter.rect_stroke(
            animated_rect,
            style.content_rounding,
            Stroke::new(
                1.0,
                Color32::from_rgba_unmultiplied(
                    style.content_border.r(),
                    style.content_border.g(),
                    style.content_border.b(),
                    alpha,
                ),
            ),
            StrokeKind::Inside,
        );

        let needs_scroll = content_height > max_popup_height;

        let content_rect = animated_rect.shrink(style.content_padding);

        let scroll_button_h = 18.0;
        let mut items_rect = content_rect;

        let max_scroll = if needs_scroll {
            let eps = 1.0;

            let base_height = content_rect.height();

            let max_scroll_with_both =
                (content_height - (base_height - 2.0 * scroll_button_h)).max(0.0);

            let max_scroll_with_up = (content_height - (base_height - scroll_button_h)).max(0.0);

            let max_scroll_with_down = (content_height - (base_height - scroll_button_h)).max(0.0);

            let max_scroll_no_buttons = (content_height - base_height).max(0.0);

            state.show_scroll_up = state.scroll_offset > eps;

            let visible_height_for_down_check = if state.show_scroll_up {
                base_height - scroll_button_h
            } else {
                base_height
            };
            state.show_scroll_down =
                state.scroll_offset + visible_height_for_down_check < content_height - eps;

            let max_scroll = match (state.show_scroll_up, state.show_scroll_down) {
                (true, true) => max_scroll_with_both,
                (true, false) => max_scroll_with_up,
                (false, true) => max_scroll_with_down,
                (false, false) => max_scroll_no_buttons,
            };

            state.scroll_offset = state.scroll_offset.clamp(0.0, max_scroll);

            state.show_scroll_up = state.scroll_offset > eps;
            let visible_height_for_down_check = if state.show_scroll_up {
                base_height - scroll_button_h
            } else {
                base_height
            };
            state.show_scroll_down =
                state.scroll_offset + visible_height_for_down_check < content_height - eps;

            let top_margin = if state.show_scroll_up {
                scroll_button_h
            } else {
                0.0
            };
            let bottom_margin = if state.show_scroll_down {
                scroll_button_h
            } else {
                0.0
            };
            items_rect = Rect::from_min_max(
                pos2(content_rect.left(), content_rect.top() + top_margin),
                pos2(content_rect.right(), content_rect.bottom() - bottom_margin),
            );

            (content_height - items_rect.height()).max(0.0)
        } else {
            state.show_scroll_up = false;
            state.show_scroll_down = false;
            state.scroll_offset = 0.0;
            0.0
        };

        if needs_scroll
            && let Some(idx) = state.focused_index
            && let Some((offset, item_h)) = calculate_selected_offset(
                items,
                &flat_options[idx].0,
                item_height,
                separator_height,
                label_height,
            )
        {
            let visible_h = items_rect.height();
            let target = (offset - (visible_h - item_h) * 0.5).max(0.0);
            state.scroll_offset = target.clamp(0.0, max_scroll);
        }

        let items_painter = content_painter.with_clip_rect(items_rect);
        let mut y_offset = items_rect.top() - state.scroll_offset;
        let mut option_index = 0;
        let mut clicked_value: Option<String> = None;

        if let Some(pos) = ui.input(|i| i.pointer.hover_pos())
            && items_rect.contains(pos)
        {
            state.focused_index = None;
        }

        let selected_ref = props.selected.clone();

        for item in items {
            let (new_y, clicked) = draw_select_item(
                &items_painter,
                item,
                items_rect,
                y_offset,
                &style,
                props.size,
                alpha,
                selected_ref.as_ref(),
                &mut option_index,
                state.focused_index,
                ui,
                item_height,
                separator_height,
                label_height,
                props.content_variant,
                props.high_contrast,
            );
            y_offset = new_y;
            if clicked.is_some() {
                clicked_value = clicked;
            }
        }

        if let Some(value) = clicked_value {
            *props.selected = Some(value);
            state.is_open = false;
            response.mark_changed();
        }

        if state.show_scroll_up {
            let btn_rect = Rect::from_min_size(
                pos2(content_rect.left(), content_rect.top()),
                vec2(content_rect.width(), scroll_button_h),
            );
            content_painter.rect_filled(btn_rect, CornerRadius::ZERO, style.content_bg);
            draw_chevron_up(
                &content_painter,
                btn_rect.center(),
                16.0,
                style.scroll_button_color,
            );

            if let Some(pos) = ui.input(|i| i.pointer.hover_pos())
                && btn_rect.contains(pos)
            {
                state.scroll_offset = (state.scroll_offset - 4.0).clamp(0.0, max_scroll);
                ui.ctx().request_repaint();
            }
        }

        if state.show_scroll_down {
            let btn_rect = Rect::from_min_size(
                pos2(content_rect.left(), content_rect.bottom() - scroll_button_h),
                vec2(content_rect.width(), scroll_button_h),
            );
            content_painter.rect_filled(btn_rect, CornerRadius::ZERO, style.content_bg);
            draw_chevron_down(
                &content_painter,
                btn_rect.center(),
                16.0,
                style.scroll_button_color,
            );

            if let Some(pos) = ui.input(|i| i.pointer.hover_pos())
                && btn_rect.contains(pos)
            {
                state.scroll_offset = (state.scroll_offset + 4.0).clamp(0.0, max_scroll);
                ui.ctx().request_repaint();
            }
        }

        if needs_scroll {
            let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);
            if animated_rect.contains(ui.input(|i| i.pointer.hover_pos().unwrap_or_default())) {
                state.scroll_offset = (state.scroll_offset - scroll_delta).clamp(0.0, max_scroll);
            }
        }
    }

    if !state.is_open {
        state.typed_buffer.clear();
        state.last_type_time = 0.0;
    }

    ui.ctx().data_mut(|d| d.insert_temp(id, state));

    response
}

#[allow(clippy::too_many_arguments)]
fn draw_select_item(
    painter: &Painter,
    item: &SelectItem,
    content_rect: Rect,
    y_offset: f32,
    style: &SelectStyle,
    size: SelectSize,
    alpha: u8,
    selected: Option<&String>,
    option_index: &mut usize,
    focused_index: Option<usize>,
    ui: &Ui,
    item_height: f32,
    separator_height: f32,
    label_height: f32,
    content_variant: ContentVariant,
    high_contrast: bool,
) -> (f32, Option<String>) {
    match item {
        SelectItem::Option {
            value,
            label,
            disabled,
        } => {
            let item_rect = Rect::from_min_size(
                pos2(content_rect.left(), y_offset),
                vec2(content_rect.width(), item_height),
            );

            if item_rect.bottom() < content_rect.top() || item_rect.top() > content_rect.bottom() {
                *option_index += 1;
                return (y_offset + item_height, None);
            }

            let is_selected = selected.map(|s| s == value).unwrap_or(false);
            let is_focused = focused_index == Some(*option_index);
            let is_hovered = ui.input(|i| {
                i.pointer
                    .hover_pos()
                    .map(|p| item_rect.contains(p))
                    .unwrap_or(false)
            });

            let selected_bg = if is_selected {
                if high_contrast && content_variant == ContentVariant::Solid {
                    style.item_solid_high_contrast_bg
                } else {
                    style.item_bg_selected
                }
            } else {
                Color32::TRANSPARENT
            };

            let (bg, text_base) = if *disabled {
                (Color32::TRANSPARENT, style.item_text)
            } else if is_hovered || is_focused {
                match content_variant {
                    ContentVariant::Solid => {
                        if high_contrast {
                            (
                                style.item_solid_high_contrast_bg,
                                style.item_solid_high_contrast_text,
                            )
                        } else {
                            (style.item_solid_bg_hover, style.item_solid_text_hover)
                        }
                    }
                    ContentVariant::Soft => (style.item_bg_hover, style.item_text_hover),
                }
            } else if is_selected {
                (
                    selected_bg,
                    if high_contrast && content_variant == ContentVariant::Solid {
                        style.item_solid_high_contrast_text
                    } else {
                        style.item_text
                    },
                )
            } else {
                (style.item_bg, style.item_text)
            };
            let bg_with_alpha = Color32::from_rgba_unmultiplied(
                bg.r(),
                bg.g(),
                bg.b(),
                (bg.a() as f32 * alpha as f32 / 255.0) as u8,
            );
            painter.rect_filled(item_rect, style.item_rounding, bg_with_alpha);

            if is_selected {
                let check_center = pos2(item_rect.right() - 20.0, item_rect.center().y);

                let check_color = if (is_hovered || is_focused) && !*disabled {
                    text_base
                } else {
                    style.item_text
                };
                draw_check_icon(
                    painter,
                    check_center,
                    16.0,
                    Color32::from_rgba_unmultiplied(
                        check_color.r(),
                        check_color.g(),
                        check_color.b(),
                        alpha,
                    ),
                );
            }

            let text_color = if *disabled {
                Color32::from_rgba_unmultiplied(
                    style.item_text.r(),
                    style.item_text.g(),
                    style.item_text.b(),
                    (alpha as f32 * 0.5) as u8,
                )
            } else {
                Color32::from_rgba_unmultiplied(text_base.r(), text_base.g(), text_base.b(), alpha)
            };

            let galley = painter.layout_no_wrap(
                label.clone(),
                FontId::proportional(size.font_size()),
                text_color,
            );
            let text_pos = pos2(
                item_rect.left() + style.item_padding.x,
                item_rect.center().y - galley.size().y * 0.5,
            );
            painter.galley(text_pos, galley, Color32::TRANSPARENT);

            let clicked = if !*disabled && is_hovered && ui.input(|i| i.pointer.any_click()) {
                Some(value.clone())
            } else {
                None
            };

            *option_index += 1;
            (y_offset + item_height, clicked)
        }

        SelectItem::Group { label, items } => {
            let label_rect = Rect::from_min_size(
                pos2(content_rect.left(), y_offset),
                vec2(content_rect.width(), label_height),
            );

            let galley = painter.layout_no_wrap(
                label.clone(),
                FontId::proportional(12.0),
                Color32::from_rgba_unmultiplied(
                    style.label_text.r(),
                    style.label_text.g(),
                    style.label_text.b(),
                    alpha,
                ),
            );
            let text_pos = pos2(
                label_rect.left() + style.item_padding.x,
                label_rect.center().y - galley.size().y * 0.5,
            );
            painter.galley(text_pos, galley, Color32::TRANSPARENT);

            let mut next_y = y_offset + label_height;
            let mut clicked_value: Option<String> = None;

            for sub_item in items {
                let (new_y, clicked) = draw_select_item(
                    painter,
                    sub_item,
                    content_rect,
                    next_y,
                    style,
                    size,
                    alpha,
                    selected,
                    option_index,
                    focused_index,
                    ui,
                    item_height,
                    separator_height,
                    label_height,
                    content_variant,
                    high_contrast,
                );
                next_y = new_y;
                if clicked.is_some() {
                    clicked_value = clicked;
                }
            }

            (next_y, clicked_value)
        }

        SelectItem::Separator => {
            let sep_rect = Rect::from_min_size(
                pos2(content_rect.left() - 4.0, y_offset + 4.0),
                vec2(content_rect.width() + 8.0, 1.0),
            );
            painter.rect_filled(
                sep_rect,
                CornerRadius::ZERO,
                Color32::from_rgba_unmultiplied(
                    style.separator_color.r(),
                    style.separator_color.g(),
                    style.separator_color.b(),
                    alpha,
                ),
            );
            (y_offset + separator_height, None)
        }

        SelectItem::Label(text) => {
            let label_rect = Rect::from_min_size(
                pos2(content_rect.left(), y_offset),
                vec2(content_rect.width(), label_height),
            );

            let galley = painter.layout_no_wrap(
                text.clone(),
                FontId::proportional(12.0),
                Color32::from_rgba_unmultiplied(
                    style.label_text.r(),
                    style.label_text.g(),
                    style.label_text.b(),
                    alpha,
                ),
            );
            let text_pos = pos2(
                label_rect.left() + style.item_padding.x,
                label_rect.center().y - galley.size().y * 0.5,
            );
            painter.galley(text_pos, galley, Color32::TRANSPARENT);

            (y_offset + label_height, None)
        }
    }
}

fn calculate_content_height(items: &[SelectItem], item_h: f32, sep_h: f32, label_h: f32) -> f32 {
    let mut height = 0.0;
    for item in items {
        match item {
            SelectItem::Option { .. } => height += item_h,
            SelectItem::Separator => height += sep_h,
            SelectItem::Label(_) => height += label_h,
            SelectItem::Group { items, .. } => {
                height += label_h;
                height += calculate_content_height(items, item_h, sep_h, label_h);
            }
        }
    }
    height
}

fn flatten_options(items: &[SelectItem]) -> Vec<(String, String, bool)> {
    let mut result = Vec::new();
    for item in items {
        match item {
            SelectItem::Option {
                value,
                label,
                disabled,
            } => {
                result.push((value.clone(), label.clone(), *disabled));
            }
            SelectItem::Group { items, .. } => {
                result.extend(flatten_options(items));
            }
            _ => {}
        }
    }
    result
}

pub fn find_typeahead_match(items: &[SelectItem], needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return None;
    }
    let needle_lower = needle.to_lowercase();
    let mut index: usize = 0;

    fn traverse(items: &[SelectItem], needle_lower: &str, index: &mut usize) -> Option<usize> {
        for item in items {
            match item {
                SelectItem::Option {
                    value,
                    label,
                    disabled,
                } => {
                    if !*disabled {
                        let label_lower = label.to_lowercase();
                        let value_lower = value.to_lowercase();
                        if label_lower.starts_with(needle_lower)
                            || value_lower.starts_with(needle_lower)
                        {
                            return Some(*index);
                        }
                    }
                    *index += 1;
                }
                SelectItem::Group { items, .. } => {
                    if let Some(found) = traverse(items, needle_lower, index) {
                        return Some(found);
                    }
                }
                _ => {}
            }
        }
        None
    }

    traverse(items, &needle_lower, &mut index)
}

fn find_label_for_value(items: &[SelectItem], value: &str) -> Option<String> {
    for item in items {
        match item {
            SelectItem::Option {
                value: v, label, ..
            } if v == value => {
                return Some(label.clone());
            }
            SelectItem::Group { items, .. } => {
                if let Some(label) = find_label_for_value(items, value) {
                    return Some(label);
                }
            }
            _ => {}
        }
    }
    None
}

fn calculate_selected_offset(
    items: &[SelectItem],
    selected_value: &str,
    item_h: f32,
    sep_h: f32,
    label_h: f32,
) -> Option<(f32, f32)> {
    fn find_offset(
        items: &[SelectItem],
        selected_value: &str,
        item_h: f32,
        sep_h: f32,
        label_h: f32,
        current_offset: f32,
    ) -> Option<(f32, f32)> {
        let mut offset = current_offset;
        for item in items {
            match item {
                SelectItem::Option { value, .. } => {
                    if value == selected_value {
                        return Some((offset, item_h));
                    }
                    offset += item_h;
                }
                SelectItem::Separator => {
                    offset += sep_h;
                }
                SelectItem::Label(_) => {
                    offset += label_h;
                }
                SelectItem::Group {
                    label: _,
                    items: sub_items,
                } => {
                    offset += label_h;
                    if let Some(result) =
                        find_offset(sub_items, selected_value, item_h, sep_h, label_h, offset)
                    {
                        return Some(result);
                    }

                    for sub_item in sub_items {
                        match sub_item {
                            SelectItem::Option { .. } => offset += item_h,
                            SelectItem::Separator => offset += sep_h,
                            SelectItem::Label(_) => offset += label_h,
                            SelectItem::Group { .. } => {}
                        }
                    }
                }
            }
        }
        None
    }

    find_offset(items, selected_value, item_h, sep_h, label_h, 0.0)
}

pub fn select<Id>(ui: &mut Ui, theme: &Theme, props: SelectPropsSimple<'_, Id>) -> Response
where
    Id: Hash + Debug,
{
    trace!(
        "Rendering select (legacy) size={:?} enabled={} options={}",
        props.size,
        props.enabled,
        props.options.len()
    );

    let items: Vec<SelectItem> = props
        .options
        .iter()
        .map(|opt| SelectItem::option(opt.clone(), opt.clone()))
        .collect();

    let new_props = SelectProps {
        id_source: props.id_source,
        selected: props.selected,
        placeholder: props.placeholder,
        size: props.size.into(),
        trigger_variant: TriggerVariant::Surface,
        content_variant: ContentVariant::Soft,
        enabled: props.enabled,
        is_invalid: props.is_invalid,
        width: None,
        style: None,
        accent_color: None,
        radius: SelectRadius::Medium,
        high_contrast: false,
        position: PopupPosition::Popper,
    };

    select_with_items(ui, theme, new_props, &items)
}
