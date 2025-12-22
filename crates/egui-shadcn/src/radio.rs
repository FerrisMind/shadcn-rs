use crate::theme::Theme;
use crate::tokens::{
    ColorPalette, ControlSize, ControlVariant, StateColors, checkbox_metrics,
    checkbox_tokens_with_high_contrast, ease_out_cubic, mix,
};
use egui::style::Widgets;
use egui::{Color32, CursorIcon, Response, Sense, Stroke, TextStyle, Ui, Vec2, WidgetText, vec2};
use log::trace;
use std::fmt::{self, Debug};
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RadioDirection {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RadioCardVariant {
    #[default]
    Button,
    Card,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextDirection {
    Ltr,
    Rtl,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridLayout {
    pub columns: usize,
    pub spacing: f32,
}

impl GridLayout {
    pub fn new(columns: usize) -> Self {
        Self {
            columns,
            spacing: 8.0,
        }
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

#[derive(Clone, Debug)]
pub struct RadioOption<T> {
    pub value: T,

    pub label: WidgetText,

    pub description: Option<WidgetText>,

    pub disabled: bool,

    pub required: bool,

    pub as_child: bool,

    pub icon: Option<WidgetText>,

    pub accent_color: Option<Color32>,

    pub force_mount_indicator: bool,
}

impl<T> RadioOption<T> {
    pub fn new(value: T, label: impl Into<WidgetText>) -> Self {
        Self {
            value,
            label: label.into(),
            description: None,
            disabled: false,
            required: false,
            as_child: false,
            icon: None,
            accent_color: None,
            force_mount_indicator: false,
        }
    }

    pub fn description(mut self, description: impl Into<WidgetText>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn as_child(mut self, as_child: bool) -> Self {
        self.as_child = as_child;
        self
    }

    pub fn icon(mut self, icon: impl Into<WidgetText>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn accent_color(mut self, color: Color32) -> Self {
        self.accent_color = Some(color);
        self
    }

    pub fn force_mount_indicator(mut self, force: bool) -> Self {
        self.force_mount_indicator = force;
        self
    }
}

#[derive(Debug)]
pub struct RadioGroupProps<'a, T, Id>
where
    T: Clone + PartialEq + Debug,
    Id: Hash + Debug,
{
    pub id_source: Id,

    pub value: &'a mut T,

    pub options: &'a [RadioOption<T>],

    pub default_value: Option<T>,

    pub on_value_change: Option<OnValueChange<'a, T>>,

    pub name: Option<String>,

    pub required: bool,

    pub dir: Option<TextDirection>,

    pub loop_focus: bool,

    pub as_child: bool,

    pub size: ControlSize,

    pub variant: ControlVariant,

    pub disabled: bool,

    pub high_contrast: bool,

    pub accent_color: Option<Color32>,

    pub direction: RadioDirection,

    pub card_variant: RadioCardVariant,

    pub grid_layout: Option<GridLayout>,

    pub show_separators: bool,

    pub custom_spacing: Option<f32>,
}

pub struct OnValueChange<'a, T>(pub Box<dyn FnMut(&T) + 'a>);

impl<'a, T> Debug for OnValueChange<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OnValueChange").finish()
    }
}

impl<'a, T, Id> RadioGroupProps<'a, T, Id>
where
    T: Clone + PartialEq + Debug,
    Id: Hash + Debug,
{
    pub fn new(id_source: Id, value: &'a mut T, options: &'a [RadioOption<T>]) -> Self {
        Self {
            id_source,
            value,
            options,
            default_value: None,
            on_value_change: None,
            name: None,
            required: false,
            dir: None,
            loop_focus: true,
            as_child: false,
            size: ControlSize::Md,
            variant: ControlVariant::Primary,
            disabled: false,
            high_contrast: false,
            accent_color: None,
            direction: RadioDirection::Vertical,
            card_variant: RadioCardVariant::Button,
            grid_layout: None,
            show_separators: false,
            custom_spacing: None,
        }
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: ControlVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn accent_color(mut self, accent: Color32) -> Self {
        self.accent_color = Some(accent);
        self
    }

    pub fn direction(mut self, direction: RadioDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn card_variant(mut self, variant: RadioCardVariant) -> Self {
        self.card_variant = variant;
        self
    }

    pub fn grid_layout(mut self, layout: GridLayout) -> Self {
        self.grid_layout = Some(layout);
        self
    }

    pub fn show_separators(mut self, show: bool) -> Self {
        self.show_separators = show;
        self
    }

    pub fn custom_spacing(mut self, spacing: f32) -> Self {
        self.custom_spacing = Some(spacing);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn dir(mut self, dir: TextDirection) -> Self {
        self.dir = Some(dir);
        self
    }

    pub fn default_value(mut self, value: T) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn on_value_change(mut self, callback: impl FnMut(&T) + 'a) -> Self {
        self.on_value_change = Some(OnValueChange(Box::new(callback)));
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }

    pub fn as_child(mut self, as_child: bool) -> Self {
        self.as_child = as_child;
        self
    }

    pub fn orientation(mut self, direction: RadioDirection) -> Self {
        self.direction = direction;
        self
    }
}

#[derive(Debug)]
pub struct RadioGroup<'a, T, Id>
where
    T: Clone + PartialEq + Debug,
    Id: Hash + Debug,
{
    props: RadioGroupProps<'a, T, Id>,
}

impl<'a, T, Id> RadioGroup<'a, T, Id>
where
    T: Clone + PartialEq + Debug,
    Id: Hash + Debug,
{
    pub fn new(id_source: Id, value: &'a mut T, options: &'a [RadioOption<T>]) -> Self {
        Self {
            props: RadioGroupProps::new(id_source, value, options),
        }
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.props.size = size;
        self
    }

    pub fn variant(mut self, variant: ControlVariant) -> Self {
        self.props.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.props.high_contrast = high_contrast;
        self
    }

    pub fn accent_color(mut self, accent: Color32) -> Self {
        self.props.accent_color = Some(accent);
        self
    }

    pub fn direction(mut self, direction: RadioDirection) -> Self {
        self.props.direction = direction;
        self
    }

    pub fn card_variant(mut self, variant: RadioCardVariant) -> Self {
        self.props.card_variant = variant;
        self
    }

    pub fn grid_layout(mut self, layout: GridLayout) -> Self {
        self.props.grid_layout = Some(layout);
        self
    }

    pub fn show_separators(mut self, show: bool) -> Self {
        self.props.show_separators = show;
        self
    }

    pub fn custom_spacing(mut self, spacing: f32) -> Self {
        self.props.custom_spacing = Some(spacing);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.props.name = Some(name.into());
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.props.required = required;
        self
    }

    pub fn dir(mut self, dir: TextDirection) -> Self {
        self.props.dir = Some(dir);
        self
    }

    pub fn default_value(mut self, value: T) -> Self {
        self.props.default_value = Some(value);
        self
    }

    pub fn on_value_change(mut self, callback: impl FnMut(&T) + 'a) -> Self {
        self.props.on_value_change = Some(OnValueChange(Box::new(callback)));
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.props.loop_focus = loop_focus;
        self
    }

    pub fn as_child(mut self, as_child: bool) -> Self {
        self.props.as_child = as_child;
        self
    }

    pub fn orientation(mut self, direction: RadioDirection) -> Self {
        self.props.direction = direction;
        self
    }

    pub fn show(self, ui: &mut Ui, theme: &Theme) -> Response {
        radio_group(ui, theme, self.props)
    }
}

#[derive(Clone, Debug)]
struct RadioStyle {
    off_idle: StateColors,
    off_hovered: StateColors,
    off_active: StateColors,
    on_idle: StateColors,
    on_hovered: StateColors,
    on_active: StateColors,
    disabled: StateColors,
    focus_ring: Color32,
    label: Color32,
    description: Color32,
    indicator: Color32,
}

impl RadioStyle {
    fn from_palette(
        palette: &ColorPalette,
        variant: ControlVariant,
        high_contrast: bool,
        accent_color: Option<Color32>,
    ) -> Self {
        let mut tokens = checkbox_tokens_with_high_contrast(palette, variant, high_contrast);

        tokens.off.idle.bg_fill = Color32::TRANSPARENT;
        tokens.off.hovered.bg_fill = Color32::TRANSPARENT;
        tokens.off.active.bg_fill = Color32::TRANSPARENT;
        tokens.disabled.bg_fill = Color32::TRANSPARENT;
        if let Some(accent) = accent_color {
            let border = Stroke::new(1.0, mix(accent, palette.foreground, 0.18));
            tokens.on.idle = StateColors::with_border(accent, palette.primary_foreground, border);
            tokens.on.hovered = StateColors::with_border(
                mix(accent, Color32::WHITE, 0.08),
                palette.primary_foreground,
                border,
            );
            tokens.on.active = StateColors::with_border(
                mix(accent, Color32::WHITE, 0.14),
                palette.primary_foreground,
                Stroke::new(border.width * 1.1, border.color),
            );
        }

        let focus_ring = mix(tokens.on.idle.bg_fill, tokens.on.idle.fg_stroke.color, 0.2);

        Self {
            off_idle: tokens.off.idle,
            off_hovered: tokens.off.hovered,
            off_active: tokens.off.active,
            on_idle: tokens.on.idle,
            on_hovered: tokens.on.hovered,
            on_active: tokens.on.active,
            disabled: tokens.disabled,
            focus_ring,
            label: palette.foreground,
            description: mix(palette.muted_foreground, palette.foreground, 0.35),
            indicator: tokens.thumb_on,
        }
    }
}

fn radio_metrics(size: ControlSize) -> (Vec2, f32) {
    let toggle = checkbox_metrics(size);
    let circle_size = vec2(toggle.track_size.x, toggle.track_size.x);
    let indicator_radius = toggle.thumb_size.x * 0.33;
    (circle_size, indicator_radius)
}

fn apply_default_value<Id, T>(ui: &Ui, props: &mut RadioGroupProps<'_, T, Id>)
where
    T: Clone + PartialEq + Debug,
    Id: Hash + Debug,
{
    if let Some(default) = props.default_value.clone() {
        let default_id = ui.make_persistent_id((&props.id_source, "default_applied"));
        let already_applied = ui
            .ctx()
            .memory_mut(|mem| mem.data.get_persisted::<bool>(default_id).unwrap_or(false));
        if !already_applied {
            *props.value = default;
            if let Some(cb) = props.on_value_change.as_mut() {
                (cb.0)(props.value);
            }
            ui.ctx()
                .memory_mut(|mem| mem.data.insert_persisted(default_id, true));
        }
    }
}

pub fn radio_group<Id, T>(ui: &mut Ui, theme: &Theme, props: RadioGroupProps<'_, T, Id>) -> Response
where
    T: Clone + PartialEq + Debug,
    Id: Hash + Debug,
{
    let mut props = props;
    trace!(
        "Rendering radio group variant={:?} size={:?} options={} card_variant={:?}",
        props.variant,
        props.size,
        props.options.len(),
        props.card_variant
    );

    apply_default_value(ui, &mut props);

    match props.card_variant {
        RadioCardVariant::Button => render_button_group(ui, theme, props),
        RadioCardVariant::Card => render_card_group(ui, theme, props),
    }
}

fn render_button_group<Id, T>(
    ui: &mut Ui,
    theme: &Theme,
    mut props: RadioGroupProps<'_, T, Id>,
) -> Response
where
    T: Clone + PartialEq + Debug,
    Id: Hash + Debug,
{
    let id = ui.make_persistent_id(&props.id_source);
    let style = RadioStyle::from_palette(
        &theme.palette,
        props.variant,
        props.high_contrast,
        props.accent_color,
    );
    let (icon_size, indicator_radius) = radio_metrics(props.size);
    let visuals = theme.control(props.variant, props.size);
    let anim_duration = theme.motion.base_ms / 1000.0;
    let spacing = visuals.padding;

    let mut aggregate_response: Option<Response> = None;
    let painter_spacing = props.custom_spacing.unwrap_or(spacing.y.max(4.0));
    let widgets = Widgets {
        noninteractive: crate::theme::widget_visuals(
            &style.off_idle,
            visuals.widgets.noninteractive.corner_radius,
            visuals.expansion,
        ),
        inactive: visuals.widgets.inactive,
        hovered: visuals.widgets.hovered,
        active: visuals.widgets.active,
        open: visuals.widgets.open,
    };

    let inner = ui.scope(|scope_ui| {
        let mut scoped_style = scope_ui.style().as_ref().clone();
        scoped_style
            .text_styles
            .insert(TextStyle::Body, visuals.text_style.clone());
        scope_ui.set_style(scoped_style);

        {
            let spacing = scope_ui.spacing_mut();
            spacing.item_spacing.y = painter_spacing;
        }

        let mut no_bg_widgets = widgets.clone();
        let clear_bg = |wv: &mut egui::style::WidgetVisuals| {
            wv.bg_fill = Color32::TRANSPARENT;
            wv.weak_bg_fill = Color32::TRANSPARENT;
            wv.bg_stroke = Stroke::NONE;
            wv.corner_radius = egui::CornerRadius::same(255);
        };
        clear_bg(&mut no_bg_widgets.inactive);
        clear_bg(&mut no_bg_widgets.hovered);
        clear_bg(&mut no_bg_widgets.active);
        clear_bg(&mut no_bg_widgets.open);
        clear_bg(&mut no_bg_widgets.noninteractive);
        scope_ui.style_mut().visuals.widgets = no_bg_widgets;

        let mut render_items = |ui_container: &mut Ui, combined: &mut Option<Response>| {
            for (idx, option) in props.options.iter().cloned().enumerate() {
                let option_enabled = !props.disabled && !option.disabled;
                let item_id = id.with(idx);
                let label_text = option.label.clone().color(style.label);

                let response = ui_container
                    .horizontal(|row| {
                        let mut row_style = row.style().as_ref().clone();
                        {
                            let mut widgets = row_style.visuals.widgets.clone();
                            let clear_bg = |wv: &mut egui::style::WidgetVisuals| {
                                wv.bg_fill = Color32::TRANSPARENT;
                                wv.weak_bg_fill = Color32::TRANSPARENT;
                                wv.bg_stroke = Stroke::NONE;
                                wv.corner_radius = egui::CornerRadius::same(255);
                            };
                            clear_bg(&mut widgets.noninteractive);
                            clear_bg(&mut widgets.inactive);
                            clear_bg(&mut widgets.hovered);
                            clear_bg(&mut widgets.active);
                            clear_bg(&mut widgets.open);
                            row_style.visuals.widgets = widgets;
                        }
                        row.set_style(row_style);

                        let (icon_rect, icon_response) =
                            row.allocate_exact_size(icon_size, Sense::click());
                        let label_response = row.add_enabled(
                            option_enabled,
                            egui::Label::new(label_text.clone()).wrap(),
                        );

                        let clicked =
                            option_enabled && (icon_response.clicked() || label_response.clicked());
                        let mut should_call_change = false;
                        if clicked && *props.value != option.value {
                            *props.value = option.value.clone();
                            should_call_change = true;
                        }

                        let selected = *props.value == option.value;
                        let anim_value = row.ctx().animate_bool_with_time_and_easing(
                            item_id,
                            selected,
                            anim_duration,
                            ease_out_cubic,
                        );

                        let hovered = icon_response.hovered() || label_response.hovered();
                        let pointer_down = icon_response.is_pointer_button_down_on();
                        let focus = icon_response.has_focus();

                        let mut off_state = style.off_idle;
                        let mut on_state = style.on_idle;
                        if !option_enabled {
                            off_state = style.disabled;
                            on_state = style.disabled;
                        } else if pointer_down {
                            off_state = style.off_active;
                            on_state = style.on_active;
                        } else if hovered {
                            off_state = style.off_hovered;
                            on_state = style.on_hovered;
                        }

                        let state = if selected { on_state } else { off_state };

                        let circle_rect =
                            icon_rect.expand((style.disabled.border.width * 0.5).max(1.0));

                        let ring_offset = 1.0;
                        let focus_ring_radius =
                            circle_rect.width().min(circle_rect.height()) * 0.5 + ring_offset;
                        let focus_ring_stroke_width = 3.0;
                        let clip_expand = (focus_ring_radius - circle_rect.width() * 0.5
                            + focus_ring_stroke_width)
                            .max(style.disabled.border.width);
                        let painter = row.painter_at(icon_rect.expand(clip_expand));
                        painter.circle_filled(
                            circle_rect.center(),
                            circle_rect.width().min(circle_rect.height()) * 0.5,
                            state.bg_fill,
                        );
                        if state.border != Stroke::NONE {
                            painter.circle_stroke(
                                circle_rect.center(),
                                circle_rect.width().min(circle_rect.height()) * 0.5,
                                Stroke::new(state.border.width, state.border.color),
                            );
                        }

                        if anim_value > 0.0 || option.force_mount_indicator {
                            let indicator_color = Color32::from_rgba_unmultiplied(
                                style.indicator.r(),
                                style.indicator.g(),
                                style.indicator.b(),
                                (style.indicator.a() as f32 * anim_value) as u8,
                            );
                            painter.circle_filled(
                                circle_rect.center(),
                                indicator_radius
                                    * if anim_value > 0.0 {
                                        anim_value.max(0.3)
                                    } else {
                                        0.3
                                    },
                                indicator_color,
                            );
                        }

                        if focus && option_enabled {
                            painter.circle_stroke(
                                circle_rect.center(),
                                focus_ring_radius,
                                Stroke::new(focus_ring_stroke_width, style.focus_ring),
                            );
                        }

                        if let Some(desc) = option.description.as_ref() {
                            let desc_color = if option_enabled {
                                style.description
                            } else {
                                Color32::from_rgba_unmultiplied(
                                    style.description.r(),
                                    style.description.g(),
                                    style.description.b(),
                                    (style.description.a() as f32 * 0.6) as u8,
                                )
                            };
                            let desc_label =
                                egui::Label::new(desc.clone().color(desc_color)).wrap();
                            row.add(desc_label);
                        }

                        let mut merged = icon_response | label_response;
                        if clicked {
                            merged.mark_changed();
                        }
                        if option_enabled {
                            merged = merged.on_hover_cursor(CursorIcon::PointingHand);
                        }

                        if should_call_change && let Some(cb) = props.on_value_change.as_mut() {
                            (cb.0)(props.value);
                        }

                        merged
                    })
                    .inner;

                *combined = Some(match combined.take() {
                    Some(acc) => acc | response,
                    None => response,
                });

                if props.show_separators && idx < props.options.len() - 1 {
                    ui_container.separator();
                }
            }
        };

        match props.direction {
            RadioDirection::Vertical => {
                scope_ui.vertical(|vert| render_items(vert, &mut aggregate_response))
            }
            RadioDirection::Horizontal => {
                scope_ui.horizontal_wrapped(|horiz| render_items(horiz, &mut aggregate_response))
            }
        }
        .inner
    });

    let scope_response = inner.response;
    if let Some(agg) = aggregate_response {
        agg | scope_response
    } else {
        scope_response
    }
}

fn render_card_group<Id, T>(
    ui: &mut Ui,
    theme: &Theme,
    mut props: RadioGroupProps<'_, T, Id>,
) -> Response
where
    T: Clone + PartialEq + Debug,
    Id: Hash + Debug,
{
    let id = ui.make_persistent_id(&props.id_source);
    let style = RadioStyle::from_palette(
        &theme.palette,
        props.variant,
        props.high_contrast,
        props.accent_color,
    );
    let (icon_size, indicator_radius) = radio_metrics(props.size);
    let visuals = theme.control(props.variant, props.size);
    let anim_duration = theme.motion.base_ms / 1000.0;
    let _spacing = props.custom_spacing.unwrap_or(8.0);

    let grid_layout = props.grid_layout.unwrap_or(GridLayout::new(2));

    let mut aggregate_response: Option<Response> = None;

    let inner = ui.scope(|scope_ui| {
        let mut scoped_style = scope_ui.style().as_ref().clone();
        scoped_style
            .text_styles
            .insert(TextStyle::Body, visuals.text_style.clone());
        let mut widgets = visuals.widgets.clone();
        let clear_bg = |wv: &mut egui::style::WidgetVisuals| {
            wv.bg_fill = Color32::TRANSPARENT;
            wv.weak_bg_fill = Color32::TRANSPARENT;
            wv.bg_stroke = Stroke::NONE;
            wv.corner_radius = egui::CornerRadius::same(255);
        };
        clear_bg(&mut widgets.noninteractive);
        clear_bg(&mut widgets.inactive);
        clear_bg(&mut widgets.hovered);
        clear_bg(&mut widgets.active);
        clear_bg(&mut widgets.open);
        scoped_style.visuals.widgets = widgets;
        scope_ui.set_style(scoped_style);

        let mut col_count = 0;
        let mut row_container_response: Option<Response> = None;

        scope_ui.vertical(|vert_ui| {
            for (idx, option) in props.options.iter().cloned().enumerate() {
                if col_count == 0 {
                    row_container_response = None;
                }

                let option_enabled = !props.disabled && !option.disabled;
                let item_id = id.with(idx);
                let selected = *props.value == option.value;
                let anim_value = vert_ui.ctx().animate_bool_with_time_and_easing(
                    item_id,
                    selected,
                    anim_duration,
                    ease_out_cubic,
                );

                let card_response = vert_ui.horizontal(|row| {
                    let mut row_style = row.style().as_ref().clone();
                    {
                        let mut widgets = row_style.visuals.widgets.clone();
                        let clear_bg = |wv: &mut egui::style::WidgetVisuals| {
                            wv.bg_fill = Color32::TRANSPARENT;
                            wv.weak_bg_fill = Color32::TRANSPARENT;
                            wv.bg_stroke = Stroke::NONE;
                            wv.corner_radius = egui::CornerRadius::same(255);
                        };
                        clear_bg(&mut widgets.noninteractive);
                        clear_bg(&mut widgets.inactive);
                        clear_bg(&mut widgets.hovered);
                        clear_bg(&mut widgets.active);
                        clear_bg(&mut widgets.open);
                        row_style.visuals.widgets = widgets;
                    }
                    row.set_style(row_style);

                    render_radio_card(
                        row,
                        &option,
                        option_enabled,
                        selected,
                        anim_value,
                        &style,
                        icon_size,
                        indicator_radius,
                    )
                });

                if selected && option_enabled {
                    card_response.inner.clone().mark_changed();
                }

                let mut should_call_change = false;
                if option_enabled && card_response.inner.clicked() && *props.value != option.value {
                    *props.value = option.value.clone();
                    should_call_change = true;
                }

                row_container_response = Some(match row_container_response.take() {
                    Some(acc) => acc | card_response.inner,
                    None => card_response.inner,
                });

                if should_call_change && let Some(cb) = props.on_value_change.as_mut() {
                    (cb.0)(props.value);
                }

                col_count += 1;
                if col_count >= grid_layout.columns {
                    col_count = 0;
                }

                if let Some(row_resp) = row_container_response.take() {
                    aggregate_response = Some(match aggregate_response.take() {
                        Some(acc) => acc | row_resp,
                        None => row_resp,
                    });
                }
            }
        });

        if let Some(resp) = row_container_response.take() {
            aggregate_response = Some(match aggregate_response.take() {
                Some(acc) => acc | resp,
                None => resp,
            });
        }
    });

    let scope_response = inner.response;
    if let Some(agg) = aggregate_response {
        agg | scope_response
    } else {
        scope_response
    }
}

#[allow(clippy::too_many_arguments)]
fn render_radio_card<T: PartialEq + Clone + Debug>(
    ui: &mut Ui,
    option: &RadioOption<T>,
    enabled: bool,
    selected: bool,
    anim_value: f32,
    style: &RadioStyle,
    icon_size: Vec2,
    indicator_radius: f32,
) -> Response {
    let card_bg = if selected {
        Color32::from_rgba_unmultiplied(
            style.on_idle.bg_fill.r(),
            style.on_idle.bg_fill.g(),
            style.on_idle.bg_fill.b(),
            (40.0 * anim_value) as u8,
        )
    } else {
        Color32::TRANSPARENT
    };

    let border_stroke = if selected {
        Stroke::new(1.0, style.on_idle.border.color)
    } else {
        Stroke::new(1.0, style.off_idle.border.color)
    };

    let (card_rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), 60.0), Sense::click());

    if ui.is_rect_visible(card_rect) {
        let painter = ui.painter();
        let corner_radius = egui::CornerRadius::same(8);

        painter.rect(
            card_rect,
            corner_radius,
            card_bg,
            border_stroke,
            egui::StrokeKind::Inside,
        );

        let radio_center = egui::pos2(
            card_rect.left() + 20.0 + icon_size.x * 0.5,
            card_rect.center().y,
        );
        let radio_radius = icon_size.x.min(icon_size.y) * 0.5;

        let state = if !enabled {
            style.disabled
        } else if response.hovered() {
            if selected {
                style.on_hovered
            } else {
                style.off_hovered
            }
        } else if selected {
            style.on_idle
        } else {
            style.off_idle
        };

        painter.circle_filled(radio_center, radio_radius, state.bg_fill);
        if state.border != Stroke::NONE {
            painter.circle_stroke(radio_center, radio_radius, state.border);
        }

        if anim_value > 0.0 || option.force_mount_indicator {
            let indicator_color = Color32::from_rgba_unmultiplied(
                style.indicator.r(),
                style.indicator.g(),
                style.indicator.b(),
                (style.indicator.a() as f32 * anim_value) as u8,
            );
            painter.circle_filled(
                radio_center,
                indicator_radius
                    * if anim_value > 0.0 {
                        anim_value.max(0.3)
                    } else {
                        0.3
                    },
                indicator_color,
            );
        }

        if response.has_focus() && enabled {
            let focus_ring_radius = radio_radius * 1.1 + 2.0;
            painter.circle_stroke(
                radio_center,
                focus_ring_radius,
                Stroke::new(2.0, style.focus_ring),
            );
        }

        let text_left = card_rect.left() + 20.0 + icon_size.x + 12.0;
        let text_top = card_rect.top() + 12.0;

        let label_str = match &option.label {
            egui::WidgetText::RichText(rt) => rt.text().to_string(),
            egui::WidgetText::Galley(_) => String::new(),
            egui::WidgetText::Text(t) => t.to_string(),
            egui::WidgetText::LayoutJob(_) => String::new(),
        };

        let label_color = if enabled {
            style.label
        } else {
            style.description
        };
        painter.text(
            egui::pos2(text_left, text_top),
            egui::Align2::LEFT_TOP,
            &label_str,
            egui::FontId::default(),
            label_color,
        );

        if let Some(desc) = &option.description {
            let desc_str = match desc {
                egui::WidgetText::RichText(rt) => rt.text().to_string(),
                egui::WidgetText::Galley(_) => String::new(),
                egui::WidgetText::Text(t) => t.to_string(),
                egui::WidgetText::LayoutJob(_) => String::new(),
            };
            painter.text(
                egui::pos2(text_left, text_top + 18.0),
                egui::Align2::LEFT_TOP,
                &desc_str,
                egui::FontId::proportional(12.0),
                style.description,
            );
        }
    }

    response
}
