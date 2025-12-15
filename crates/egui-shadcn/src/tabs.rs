use crate::scroll_area::{ScrollAreaProps, ScrollDirection, scroll_area};
use crate::theme::Theme;
use crate::tokens::{ColorPalette, DEFAULT_FOCUS, ease_out_cubic, mix};
use egui::scroll_area::ScrollBarVisibility;
use egui::{
    Color32, CornerRadius, FontId, Frame, Id, Margin, Pos2, Rect, Response, Sense, Stroke,
    StrokeKind, TextStyle, TextWrapMode, Ui, Vec2, WidgetText, vec2,
};
use log::trace;
use std::fmt::{self, Debug};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsVariant {
    Underline,

    Soft,

    Outline,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsOrientation {
    Horizontal,
    Vertical,
}

pub type TabsDirection = TabsOrientation;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsDirectionality {
    Ltr,
    Rtl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsActivationMode {
    Automatic,
    Manual,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsListLoop {
    Enabled,
    Disabled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsContentForceMount {
    Off,
    On,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsSize {
    Size1,
    Size2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsJustify {
    Start,
    Center,
    End,
}

#[derive(Clone, Debug)]
pub struct TabItem {
    pub id: String,
    pub label: WidgetText,
    pub disabled: bool,
    pub force_mount: TabsContentForceMount,
    pub trigger_as_child: bool,
}

impl TabItem {
    pub fn new(id: impl Into<String>, label: impl Into<WidgetText>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
            force_mount: TabsContentForceMount::Off,
            trigger_as_child: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn force_mount(mut self, force: bool) -> Self {
        self.force_mount = if force {
            TabsContentForceMount::On
        } else {
            TabsContentForceMount::Off
        };
        self
    }

    pub fn trigger_as_child(mut self, as_child: bool) -> Self {
        self.trigger_as_child = as_child;
        self
    }
}

#[derive(Debug)]
pub struct TabsProps<'a> {
    pub id_source: Id,
    pub items: &'a [TabItem],
    pub active: &'a mut String,
    pub default_value: Option<String>,
    pub on_value_change: Option<OnValueChange<'a>>,
    pub variant: TabsVariant,
    pub orientation: TabsDirection,
    pub activation_mode: TabsActivationMode,
    pub dir: Option<TabsDirectionality>,
    pub list_loop: TabsListLoop,
    pub size: TabsSize,
    pub wrap: TabsWrap,
    pub justify: TabsJustify,

    pub root_as_child: bool,
    pub list_as_child: bool,
    pub trigger_as_child: bool,
    pub content_as_child: bool,
    pub content_force_mount: TabsContentForceMount,

    pub full_width: bool,

    pub scrollable: bool,

    pub accent_color: Option<Color32>,

    pub high_contrast: bool,

    pub animate: bool,
    pub compact: bool,
}

pub struct OnValueChange<'a>(pub Box<dyn FnMut(&str) + 'a>);

impl<'a> Debug for OnValueChange<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OnValueChange").finish()
    }
}

impl<'a> TabsProps<'a> {
    pub fn new(id_source: Id, items: &'a [TabItem], active: &'a mut String) -> Self {
        Self {
            id_source,
            items,
            active,
            default_value: None,
            on_value_change: None,
            variant: TabsVariant::Underline,
            orientation: TabsDirection::Horizontal,
            activation_mode: TabsActivationMode::Automatic,
            dir: None,
            list_loop: TabsListLoop::Enabled,
            size: TabsSize::Size2,
            wrap: TabsWrap::NoWrap,
            justify: TabsJustify::Start,
            root_as_child: false,
            list_as_child: false,
            trigger_as_child: false,
            content_as_child: false,
            content_force_mount: TabsContentForceMount::Off,
            full_width: false,
            scrollable: true,
            accent_color: None,
            high_contrast: false,
            animate: true,
            compact: false,
        }
    }

    pub fn with_variant(mut self, variant: TabsVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_orientation(mut self, orientation: TabsDirection) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_activation_mode(mut self, mode: TabsActivationMode) -> Self {
        self.activation_mode = mode;
        self
    }

    pub fn with_dir(mut self, dir: TabsDirectionality) -> Self {
        self.dir = Some(dir);
        self
    }

    pub fn with_default_value(mut self, value: String) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn with_on_value_change(mut self, callback: impl FnMut(&str) + 'a) -> Self {
        self.on_value_change = Some(OnValueChange(Box::new(callback)));
        self
    }

    pub fn with_list_loop(mut self, loop_focus: TabsListLoop) -> Self {
        self.list_loop = loop_focus;
        self
    }

    pub fn with_size(mut self, size: TabsSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_wrap(mut self, wrap: TabsWrap) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn with_justify(mut self, justify: TabsJustify) -> Self {
        self.justify = justify;
        self
    }

    pub fn root_as_child(mut self, as_child: bool) -> Self {
        self.root_as_child = as_child;
        self
    }

    pub fn list_as_child(mut self, as_child: bool) -> Self {
        self.list_as_child = as_child;
        self
    }

    pub fn trigger_as_child(mut self, as_child: bool) -> Self {
        self.trigger_as_child = as_child;
        self
    }

    pub fn content_as_child(mut self, as_child: bool) -> Self {
        self.content_as_child = as_child;
        self
    }

    pub fn with_content_force_mount(mut self, force: TabsContentForceMount) -> Self {
        self.content_force_mount = force;
        self
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.scrollable = scrollable;
        self
    }

    pub fn with_accent_color(mut self, color: Color32) -> Self {
        self.accent_color = Some(color);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }
}

pub struct TabsResult<R> {
    pub bar_response: Response,
    pub content: R,
}

fn apply_default_value(ui: &Ui, props: &mut TabsProps<'_>) {
    if let Some(default) = props.default_value.clone() {
        let applied_id = props.id_source.with("default_applied");
        let already_applied = ui
            .ctx()
            .memory_mut(|mem| mem.data.get_persisted::<bool>(applied_id).unwrap_or(false));
        if !already_applied {
            *props.active = default;
            if let Some(cb) = props.on_value_change.as_mut() {
                (cb.0)(props.active);
            }
            ui.ctx()
                .memory_mut(|mem| mem.data.insert_persisted(applied_id, true));
        }
    }
}

fn set_active_if_changed(props: &mut TabsProps<'_>, next: &str) {
    if props.active != next {
        *props.active = next.to_string();
        if let Some(cb) = props.on_value_change.as_mut() {
            (cb.0)(props.active);
        }
    }
}

pub fn tabs<'a, R>(
    ui: &mut Ui,
    theme: &Theme,
    mut props: TabsProps<'a>,
    render_content: impl FnOnce(&mut Ui, &TabItem) -> R,
) -> TabsResult<R> {
    let palette = &theme.palette;

    apply_default_value(ui, &mut props);

    if props.items.is_empty() {
        let dummy = ui.allocate_response(Vec2::ZERO, Sense::hover());
        return TabsResult {
            bar_response: dummy,
            content: render_content(ui, &TabItem::new("empty", WidgetText::from("Empty"))),
        };
    }

    let first_enabled = props
        .items
        .iter()
        .find(|t| !t.disabled)
        .unwrap_or(&props.items[0]);

    if !props
        .items
        .iter()
        .any(|t| t.id == *props.active && !t.disabled)
    {
        set_active_if_changed(&mut props, &first_enabled.id);
    }

    trace!("render tabs {:?}", props.id_source);

    let tokens = resolve_tabs_tokens(palette, &props);
    let anim_duration = theme.motion.base_ms / 1000.0;
    let mut current_active = props.active.clone();
    let mut pending_active: Option<String> = None;

    let mut bar_rect: Rect = Rect::NOTHING;
    let bar_response = Frame::default()
        .fill(tokens.list_fill)
        .stroke(tokens.list_stroke)
        .corner_radius(tokens.list_rounding)
        .inner_margin(Margin::same(tokens.list_padding.round() as i8))
        .show(ui, |bar_ui| {
            bar_rect = bar_ui.max_rect();

            let mut scoped_style = bar_ui.style().as_ref().clone();
            scoped_style
                .text_styles
                .insert(TextStyle::Button, tokens.font.clone());
            bar_ui.set_style(scoped_style);

            let mut add_triggers = |triggers_ui: &mut Ui| {
                triggers_ui.spacing_mut().item_spacing = vec2(tokens.gap, tokens.gap);

                let available_width = triggers_ui.available_width();
                let trigger_ids: Vec<Id> = props
                    .items
                    .iter()
                    .map(|tab| props.id_source.with("trigger").with(&tab.id))
                    .collect();

                let measured_widths: Vec<f32> = props
                    .items
                    .iter()
                    .map(|tab| {
                        let galley = tab.label.clone().into_galley(
                            triggers_ui,
                            Some(TextWrapMode::Extend),
                            f32::INFINITY,
                            TextStyle::Button,
                        );
                        galley.size().x + tokens.trigger_padding.x * 2.0
                    })
                    .collect();

                let trigger_widths: Vec<f32> = if props.full_width
                    && !props.scrollable
                    && props.wrap == TabsWrap::NoWrap
                    && props.orientation == TabsOrientation::Horizontal
                {
                    let total_gap = tokens.gap * (props.items.len().saturating_sub(1) as f32);
                    let width_each =
                        ((available_width - total_gap) / props.items.len().max(1) as f32).max(0.0);
                    vec![width_each; props.items.len()]
                } else if props.full_width && props.orientation == TabsOrientation::Vertical {
                    vec![available_width.max(0.0); props.items.len()]
                } else {
                    measured_widths
                };

                let total_width = trigger_widths.iter().sum::<f32>()
                    + tokens.gap * (props.items.len().saturating_sub(1) as f32);
                let leading_space = if props.scrollable || props.wrap != TabsWrap::NoWrap {
                    0.0
                } else {
                    match props.justify {
                        TabsJustify::Start => 0.0,
                        TabsJustify::Center => ((available_width - total_width) * 0.5).max(0.0),
                        TabsJustify::End => (available_width - total_width).max(0.0),
                    }
                };
                if leading_space > 0.0 {
                    triggers_ui.add_space(leading_space);
                }

                let mut focused_index: Option<usize> = None;

                let mut render_trigger = |trigger_ui: &mut Ui,
                                          index: usize,
                                          tab: &TabItem|
                 -> Response {
                    let trigger_id = trigger_ids[index];
                    let is_active = current_active == tab.id;
                    let width = trigger_widths[index];

                    let (rect, _) = trigger_ui
                        .allocate_exact_size(vec2(width, tokens.trigger_height), Sense::hover());
                    let response = if tab.disabled {
                        trigger_ui.interact(rect, trigger_id, Sense::hover())
                    } else {
                        trigger_ui.interact(rect, trigger_id, Sense::click())
                    };

                    if response.has_focus() {
                        focused_index = Some(index);
                    }

                    let effectively_disabled = tab.disabled;
                    let is_hovered = response.hovered() && !effectively_disabled;
                    let is_pressed = response.is_pointer_button_down_on() && !effectively_disabled;
                    let has_focus = response.has_focus() && !effectively_disabled;

                    let active_t = if props.animate {
                        trigger_ui.ctx().animate_bool_with_time_and_easing(
                            response.id.with("active"),
                            is_active,
                            anim_duration,
                            ease_out_cubic,
                        )
                    } else if is_active {
                        1.0
                    } else {
                        0.0
                    };
                    let hover_t = if props.animate {
                        trigger_ui.ctx().animate_bool_with_time_and_easing(
                            response.id.with("hover"),
                            is_hovered,
                            anim_duration,
                            ease_out_cubic,
                        )
                    } else if is_hovered {
                        1.0
                    } else {
                        0.0
                    };
                    let press_t = if props.animate {
                        trigger_ui.ctx().animate_bool_with_time_and_easing(
                            response.id.with("pressed"),
                            is_pressed,
                            anim_duration,
                            ease_out_cubic,
                        )
                    } else if is_pressed {
                        1.0
                    } else {
                        0.0
                    };

                    let mut bg_color = blended_color(
                        tokens.trigger_idle_bg,
                        tokens.trigger_hover_bg,
                        tokens.trigger_active_bg,
                        hover_t,
                        active_t,
                        press_t,
                    );
                    let mut text_color = blended_color(
                        tokens.trigger_idle_text,
                        tokens.trigger_hover_text,
                        tokens.trigger_active_text,
                        hover_t,
                        active_t,
                        press_t,
                    );

                    if effectively_disabled {
                        bg_color = mix(bg_color, palette.background, 0.6);
                        text_color = mix(text_color, palette.background, 0.6);
                    }

                    if props.variant == TabsVariant::Soft && active_t > 0.01 {
                        let shadow_rect = rect.translate(vec2(0.0, 1.0));
                        trigger_ui.painter().rect_filled(
                            shadow_rect,
                            tokens.trigger_rounding,
                            Color32::from_black_alpha((20.0 * active_t) as u8),
                        );
                    }

                    trigger_ui
                        .painter()
                        .rect_filled(rect, tokens.trigger_rounding, bg_color);
                    if tokens.trigger_stroke.width > 0.0 {
                        let stroke = if props.variant == TabsVariant::Soft {
                            Stroke::new(
                                tokens.trigger_stroke.width,
                                apply_opacity(tokens.trigger_stroke.color, active_t),
                            )
                        } else {
                            tokens.trigger_stroke
                        };
                        trigger_ui.painter().rect_stroke(
                            rect,
                            tokens.trigger_rounding,
                            stroke,
                            StrokeKind::Inside,
                        );
                    }

                    if has_focus {
                        trigger_ui.painter().rect_stroke(
                            rect.expand(1.0),
                            tokens.trigger_rounding,
                            tokens.focus_ring,
                            StrokeKind::Outside,
                        );
                    }

                    if props.variant == TabsVariant::Underline {
                        let thickness = tokens.indicator_thickness;
                        let indicator_rect = match props.orientation {
                            TabsOrientation::Horizontal => Rect::from_min_size(
                                Pos2::new(rect.left(), rect.bottom() - thickness),
                                vec2(rect.width(), thickness),
                            ),
                            TabsOrientation::Vertical => Rect::from_min_size(
                                Pos2::new(rect.right() - thickness, rect.top()),
                                vec2(thickness, rect.height()),
                            ),
                        };
                        let indicator_color = apply_opacity(tokens.indicator_color, active_t);
                        trigger_ui.painter().rect_filled(
                            indicator_rect,
                            tokens.indicator_rounding,
                            indicator_color,
                        );
                    }

                    let galley = tab.label.clone().into_galley(
                        trigger_ui,
                        Some(TextWrapMode::Extend),
                        rect.width() - tokens.trigger_padding.x * 2.0,
                        TextStyle::Button,
                    );
                    let text_pos = rect.center() - 0.5 * galley.size();
                    trigger_ui.painter().galley(text_pos, galley, text_color);

                    if (response.clicked()
                        || (has_focus
                            && trigger_ui.input(|i| {
                                i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Space)
                            })))
                        && current_active != tab.id
                    {
                        current_active = tab.id.clone();
                        pending_active = Some(tab.id.clone());
                    }

                    let mut response = response;
                    if !effectively_disabled {
                        response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
                    }
                    response
                };

                match props.orientation {
                    TabsOrientation::Horizontal => {
                        if props.wrap == TabsWrap::NoWrap {
                            triggers_ui.horizontal(|row_ui| {
                                for (index, tab) in props.items.iter().enumerate() {
                                    render_trigger(row_ui, index, tab);
                                }
                            });
                        } else {
                            triggers_ui.horizontal_wrapped(|row_ui| {
                                for (index, tab) in props.items.iter().enumerate() {
                                    render_trigger(row_ui, index, tab);
                                }
                            });
                        }
                    }
                    TabsOrientation::Vertical => {
                        triggers_ui.vertical(|col_ui| {
                            for (index, tab) in props.items.iter().enumerate() {
                                render_trigger(col_ui, index, tab);
                            }
                        });
                    }
                };

                if let Some(current) = focused_index {
                    let (next_key, prev_key) = match props.orientation {
                        TabsDirection::Horizontal => {
                            let (mut next, mut prev) =
                                (egui::Key::ArrowRight, egui::Key::ArrowLeft);
                            if matches!(props.dir, Some(TabsDirectionality::Rtl)) {
                                std::mem::swap(&mut next, &mut prev);
                            }
                            (next, prev)
                        }
                        TabsDirection::Vertical => (egui::Key::ArrowDown, egui::Key::ArrowUp),
                    };

                    let direction = if triggers_ui.input(|i| i.key_pressed(next_key)) {
                        Some(1_i32)
                    } else if triggers_ui.input(|i| i.key_pressed(prev_key)) {
                        Some(-1_i32)
                    } else {
                        None
                    };

                    if let Some(delta) = direction
                        && let Some(next_index) =
                            next_enabled_index(current, delta, props.items, props.list_loop)
                    {
                        let next_tab = &props.items[next_index];
                        if props.activation_mode == TabsActivationMode::Automatic
                            && current_active != next_tab.id
                        {
                            current_active = next_tab.id.clone();
                            pending_active = Some(next_tab.id.clone());
                        }
                        let next_id = trigger_ids[next_index];
                        triggers_ui.memory_mut(|m| m.request_focus(next_id));
                    }
                }
            };

            if props.scrollable {
                let scroll_props = ScrollAreaProps::default()
                    .with_id(props.id_source.with("scroll"))
                    .with_direction(match props.orientation {
                        TabsOrientation::Horizontal => ScrollDirection::Horizontal,
                        TabsOrientation::Vertical => ScrollDirection::Vertical,
                    })
                    .with_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                    .with_auto_shrink([true, true]);
                scroll_area(bar_ui, theme, scroll_props, add_triggers);
            } else {
                add_triggers(bar_ui);
            }
        })
        .response;

    if props.variant == TabsVariant::Underline {
        let line_color = mix(palette.border, palette.foreground, 0.1);
        match props.orientation {
            TabsOrientation::Horizontal => {
                let y = bar_rect.bottom();
                ui.painter().line_segment(
                    [
                        Pos2::new(bar_rect.left(), y),
                        Pos2::new(bar_rect.right(), y),
                    ],
                    Stroke::new(1.0, line_color),
                );
            }
            TabsOrientation::Vertical => {
                let x = bar_rect.right();
                ui.painter().line_segment(
                    [
                        Pos2::new(x, bar_rect.top()),
                        Pos2::new(x, bar_rect.bottom()),
                    ],
                    Stroke::new(1.0, line_color),
                );
            }
        }
    }

    if let Some(new_active) = pending_active.take() {
        set_active_if_changed(&mut props, &new_active);
    }

    let active_tab = props
        .items
        .iter()
        .find(|t| t.id == *props.active)
        .unwrap_or(first_enabled);

    ui.add_space(8.0);

    let content = render_content(ui, active_tab);

    TabsResult {
        bar_response,
        content,
    }
}

#[derive(Clone, Debug)]
struct TabsTokens {
    list_fill: Color32,
    list_stroke: Stroke,
    list_rounding: CornerRadius,
    list_padding: f32,
    trigger_height: f32,
    trigger_padding: Vec2,
    trigger_rounding: CornerRadius,
    trigger_stroke: Stroke,
    trigger_idle_bg: Color32,
    trigger_hover_bg: Color32,
    trigger_active_bg: Color32,
    trigger_idle_text: Color32,
    trigger_hover_text: Color32,
    trigger_active_text: Color32,
    focus_ring: Stroke,
    indicator_color: Color32,
    indicator_thickness: f32,
    indicator_rounding: CornerRadius,
    gap: f32,
    font: FontId,
}

fn resolve_tabs_tokens(palette: &ColorPalette, props: &TabsProps<'_>) -> TabsTokens {
    let focus_color = Color32::from_rgba_unmultiplied(
        palette.ring.r(),
        palette.ring.g(),
        palette.ring.b(),
        128,
    );
    let focus_ring = DEFAULT_FOCUS.stroke(focus_color);
    let accent = props.accent_color.unwrap_or(palette.accent);

    let (mut trigger_height, mut trigger_padding, font, _list_rounding, trigger_rounding) =
        match props.size {
            TabsSize::Size1 => (
                32.0,
                vec2(8.0, 5.0),
                FontId::proportional(13.0),
                CornerRadius::same(6),
                CornerRadius::same(4),
            ),
            TabsSize::Size2 => (
                36.0,
                vec2(10.0, 6.0),
                FontId::proportional(14.0),
                CornerRadius::same(8),
                CornerRadius::same(6),
            ),
        };

    if props.compact {
        trigger_padding *= 0.85;
        trigger_height -= 2.0;
    }

    match props.variant {
        TabsVariant::Underline => {
            let hover_bg = mix(palette.muted, palette.background, 0.7);
            let indicator_color = if props.high_contrast {
                mix(accent, palette.foreground, 0.6)
            } else {
                mix(accent, palette.foreground, 0.35)
            };
            TabsTokens {
                list_fill: Color32::TRANSPARENT,
                list_stroke: Stroke::NONE,
                list_rounding: CornerRadius::same(0),
                list_padding: 0.0,
                trigger_height,
                trigger_padding,
                trigger_rounding,
                trigger_stroke: Stroke::NONE,
                trigger_idle_bg: Color32::TRANSPARENT,
                trigger_hover_bg: hover_bg,
                trigger_active_bg: Color32::TRANSPARENT,
                trigger_idle_text: palette.muted_foreground,
                trigger_hover_text: palette.foreground,
                trigger_active_text: palette.foreground,
                focus_ring,
                indicator_color,
                indicator_thickness: 2.0,
                indicator_rounding: CornerRadius::same(1),
                gap: 0.0,
                font,
            }
        }
        TabsVariant::Soft => {
            let list_fill = palette.muted;
            let hover_bg = Color32::TRANSPARENT;

            let list_padding = 3.0;
            let trigger_height = (trigger_height - list_padding * 2.0).max(0.0);
            let trigger_padding = vec2(8.0, 4.0);

            let list_rounding = match props.size {
                TabsSize::Size1 => CornerRadius::same(8),
                TabsSize::Size2 => CornerRadius::same(10),
            };
            let trigger_rounding = match props.size {
                TabsSize::Size1 => CornerRadius::same(6),
                TabsSize::Size2 => CornerRadius::same(8),
            };

            let trigger_stroke = Stroke::new(1.0, mix(palette.border, palette.foreground, 0.22));

            let active_bg = palette.background;
            TabsTokens {
                list_fill,
                list_stroke: Stroke::NONE,
                list_rounding,
                list_padding,
                trigger_height,
                trigger_padding,
                trigger_rounding,
                trigger_stroke,
                trigger_idle_bg: Color32::TRANSPARENT,
                trigger_hover_bg: hover_bg,
                trigger_active_bg: active_bg,
                trigger_idle_text: palette.muted_foreground,
                trigger_hover_text: palette.foreground,
                trigger_active_text: palette.foreground,
                focus_ring,
                indicator_color: Color32::TRANSPARENT,
                indicator_thickness: 0.0,
                indicator_rounding: CornerRadius::same(0),
                gap: 0.0,
                font,
            }
        }
        TabsVariant::Outline => {
            let stroke_color = mix(palette.border, palette.foreground, 0.1);
            let hover_bg = mix(palette.input, palette.background, 0.85);
            let active_bg = mix(palette.background, palette.input, 0.6);
            TabsTokens {
                list_fill: Color32::TRANSPARENT,
                list_stroke: Stroke::NONE,
                list_rounding: CornerRadius::same(0),
                list_padding: 0.0,
                trigger_height,
                trigger_padding,
                trigger_rounding,
                trigger_stroke: Stroke::new(1.0, stroke_color),
                trigger_idle_bg: Color32::TRANSPARENT,
                trigger_hover_bg: hover_bg,
                trigger_active_bg: active_bg,
                trigger_idle_text: palette.muted_foreground,
                trigger_hover_text: palette.foreground,
                trigger_active_text: palette.foreground,
                focus_ring,
                indicator_color: Color32::TRANSPARENT,
                indicator_thickness: 0.0,
                indicator_rounding: CornerRadius::same(0),
                gap: 4.0,
                font,
            }
        }
    }
}

fn next_enabled_index(
    start: usize,
    delta: i32,
    items: &[TabItem],
    loop_focus: TabsListLoop,
) -> Option<usize> {
    if items.is_empty() {
        return None;
    }
    let len = items.len();
    let mut index = start as i32;
    let limit = if matches!(loop_focus, TabsListLoop::Enabled) {
        len
    } else {
        len + 1
    };
    for _ in 0..limit {
        index += delta;
        if matches!(loop_focus, TabsListLoop::Disabled) {
            if index < 0 || index >= len as i32 {
                return None;
            }
        } else {
            index = index.rem_euclid(len as i32);
        }
        let candidate = index as usize;
        if !items[candidate].disabled {
            return Some(candidate);
        }
    }
    None
}

fn blended_color(
    idle: Color32,
    hover: Color32,
    active: Color32,
    hover_t: f32,
    active_t: f32,
    press_t: f32,
) -> Color32 {
    let base = lerp_color(idle, hover, hover_t);
    let pressed = lerp_color(base, hover, press_t * 0.5);
    lerp_color(pressed, active, active_t)
}

fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let lerp_u8 = |x: u8, y: u8| -> u8 {
        (x as f32 + (y as f32 - x as f32) * t)
            .round()
            .clamp(0.0, 255.0) as u8
    };
    Color32::from_rgba_unmultiplied(
        lerp_u8(a.r(), b.r()),
        lerp_u8(a.g(), b.g()),
        lerp_u8(a.b(), b.b()),
        lerp_u8(a.a(), b.a()),
    )
}

fn apply_opacity(color: Color32, opacity: f32) -> Color32 {
    let alpha = (color.a() as f32 * opacity.clamp(0.0, 1.0)).round() as u8;
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}
