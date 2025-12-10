use crate::theme::Theme;
use crate::tokens::{
    ColorPalette, ControlSize, ControlVariant, StateColors, checkbox_metrics, checkbox_tokens, mix,
};
use egui::style::Widgets;
use egui::{Color32, CursorIcon, Response, Sense, Stroke, TextStyle, Ui, Vec2, WidgetText, vec2};
use log::trace;
use std::fmt::Debug;
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

    pub icon: Option<WidgetText>,

    pub accent_color: Option<Color32>,
}

impl<T> RadioOption<T> {
    pub fn new(value: T, label: impl Into<WidgetText>) -> Self {
        Self {
            value,
            label: label.into(),
            description: None,
            disabled: false,
            icon: None,
            accent_color: None,
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

    pub fn icon(mut self, icon: impl Into<WidgetText>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn accent_color(mut self, color: Color32) -> Self {
        self.accent_color = Some(color);
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
        let mut tokens = checkbox_tokens(palette, variant);
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

        if high_contrast {
            tokens.on.idle.bg_fill = mix(tokens.on.idle.bg_fill, palette.foreground, 0.25);
            tokens.on.hovered.bg_fill = mix(tokens.on.hovered.bg_fill, palette.foreground, 0.2);
            tokens.on.active.bg_fill = mix(tokens.on.active.bg_fill, palette.foreground, 0.2);
            tokens.off.idle.bg_fill = mix(tokens.off.idle.bg_fill, palette.background, 0.2);
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

pub fn radio_group<Id, T>(ui: &mut Ui, theme: &Theme, props: RadioGroupProps<'_, T, Id>) -> Response
where
    T: Clone + PartialEq + Debug,
    Id: Hash + Debug,
{
    trace!(
        "Rendering radio group variant={:?} size={:?} options={} card_variant={:?}",
        props.variant,
        props.size,
        props.options.len(),
        props.card_variant
    );

    match props.card_variant {
        RadioCardVariant::Button => render_button_group(ui, theme, props),
        RadioCardVariant::Card => render_card_group(ui, theme, props),
    }
}

fn render_button_group<Id, T>(
    ui: &mut Ui,
    theme: &Theme,
    props: RadioGroupProps<'_, T, Id>,
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

        scope_ui.style_mut().visuals.widgets = widgets.clone();

        let mut render_items = |ui_container: &mut Ui, combined: &mut Option<Response>| {
            for (idx, option) in props.options.iter().enumerate() {
                let option_enabled = !props.disabled && !option.disabled;
                let item_id = id.with(idx);
                let label_text = option.label.clone().color(style.label);

                let response = ui_container
                    .horizontal(|row| {
                        let (icon_rect, icon_response) =
                            row.allocate_exact_size(icon_size, Sense::click());
                        let label_response = row.add_enabled(
                            option_enabled,
                            egui::Label::new(label_text.clone()).wrap(),
                        );

                        let clicked =
                            option_enabled && (icon_response.clicked() || label_response.clicked());
                        if clicked {
                            *props.value = option.value.clone();
                        }

                        let selected = *props.value == option.value;
                        let anim_value = row.ctx().animate_bool(item_id, selected);

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

                        let painter = row.painter_at(icon_rect.expand(style.disabled.border.width));
                        let circle_rect =
                            icon_rect.expand((style.disabled.border.width * 0.5).max(1.0));
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

                        if anim_value > 0.0 {
                            let indicator_color = Color32::from_rgba_unmultiplied(
                                style.indicator.r(),
                                style.indicator.g(),
                                style.indicator.b(),
                                (style.indicator.a() as f32 * anim_value) as u8,
                            );
                            painter.circle_filled(
                                circle_rect.center(),
                                indicator_radius * anim_value.max(0.3),
                                indicator_color,
                            );
                        }

                        if focus && option_enabled {
                            let focus_ring_radius =
                                circle_rect.width().min(circle_rect.height()) * 0.55 + 2.0;
                            painter.circle_stroke(
                                circle_rect.center(),
                                focus_ring_radius,
                                Stroke::new(2.0, style.focus_ring),
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
    props: RadioGroupProps<'_, T, Id>,
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
    let visuals = theme.control(props.variant, props.size);
    let _spacing = props.custom_spacing.unwrap_or(8.0);

    let grid_layout = props.grid_layout.unwrap_or(GridLayout::new(2));

    let mut aggregate_response: Option<Response> = None;

    let inner = ui.scope(|scope_ui| {
        let mut scoped_style = scope_ui.style().as_ref().clone();
        scoped_style
            .text_styles
            .insert(TextStyle::Body, visuals.text_style.clone());
        scope_ui.set_style(scoped_style);

        let mut col_count = 0;
        let mut row_container_response: Option<Response> = None;

        scope_ui.vertical(|vert_ui| {
            for (idx, option) in props.options.iter().enumerate() {
                if col_count == 0 {
                    row_container_response = None;
                }

                let option_enabled = !props.disabled && !option.disabled;
                let item_id = id.with(idx);
                let selected = *props.value == option.value;
                let anim_value = vert_ui.ctx().animate_bool(item_id, selected);

                let card_response = vert_ui.horizontal(|row| {
                    render_radio_card(
                        row,
                        option,
                        option_enabled,
                        selected,
                        anim_value,
                        &style,
                        props.value,
                    )
                });

                if selected && option_enabled {
                    card_response.inner.clone().mark_changed();
                }

                if option_enabled && card_response.inner.clicked() {
                    *props.value = option.value.clone();
                }

                row_container_response = Some(match row_container_response.take() {
                    Some(acc) => acc | card_response.inner,
                    None => card_response.inner,
                });

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

fn render_radio_card<T: PartialEq + Clone + Debug>(
    ui: &mut Ui,
    option: &RadioOption<T>,
    enabled: bool,
    _selected: bool,
    _anim_value: f32,
    style: &RadioStyle,
    _current_value: &T,
) -> Response {
    let _card_bg = if _selected {
        Color32::from_rgba_unmultiplied(
            style.on_idle.bg_fill.r(),
            style.on_idle.bg_fill.g(),
            style.on_idle.bg_fill.b(),
            (255.0 * _anim_value) as u8,
        )
    } else {
        style.off_idle.bg_fill
    };

    let _border_stroke = if _selected {
        Stroke::new(2.0, style.on_idle.border.color)
    } else {
        style.off_idle.border
    };

    ui.vertical(|card_ui| {
        let label_str = match &option.label {
            egui::WidgetText::RichText(rt) => rt.text(),
            egui::WidgetText::Galley(_) => "",
            egui::WidgetText::Text(t) => t,
            egui::WidgetText::LayoutJob(_) => "",
        };

        let response = card_ui.button(egui::RichText::new(label_str).color(if enabled {
            style.label
        } else {
            style.description
        }));

        if let Some(desc) = &option.description {
            let desc_str = match desc {
                egui::WidgetText::RichText(rt) => rt.text(),
                egui::WidgetText::Galley(_) => "",
                egui::WidgetText::Text(t) => t,
                egui::WidgetText::LayoutJob(_) => "",
            };
            card_ui.label(
                egui::RichText::new(desc_str)
                    .color(style.description)
                    .small(),
            );
        }

        response
    })
    .inner
}
