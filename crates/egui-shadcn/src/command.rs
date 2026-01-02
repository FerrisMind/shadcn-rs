//! Command component - command palette with search and grouped actions.
//!
//! Make sure the Lucide font is loaded if you want the icon glyphs to render properly.

use crate::dialog::{DialogProps, dialog};
use crate::theme::Theme;
use crate::tokens::DEFAULT_RADIUS;
use egui::{
    Align, Color32, CornerRadius, Frame, Id, Key, Layout, Margin, Response, RichText, ScrollArea,
    Sense, Stroke, Ui, Vec2, WidgetText, vec2,
};
use lucide_icons::Icon;
use std::fmt::{self, Debug};
use std::hash::Hash;

// =============================================================================
// Props and context
// =============================================================================

pub struct OnCommandSelect<'a>(pub Box<dyn FnMut() + 'a>);

impl<'a> Debug for OnCommandSelect<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OnCommandSelect").finish()
    }
}

#[derive(Debug)]
pub struct CommandProps {
    pub id_source: Id,
    pub min_width: Option<f32>,
    pub show_border: bool,
    pub show_shadow: bool,
}

impl CommandProps {
    pub fn new(id_source: Id) -> Self {
        Self {
            id_source,
            min_width: None,
            show_border: true,
            show_shadow: true,
        }
    }

    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    pub fn show_shadow(mut self, show: bool) -> Self {
        self.show_shadow = show;
        self
    }
}

#[derive(Clone, Debug)]
pub struct CommandInputProps {
    pub placeholder: String,
}

impl CommandInputProps {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CommandListProps {
    pub max_height: f32,
}

impl Default for CommandListProps {
    fn default() -> Self {
        Self { max_height: 300.0 }
    }
}

#[derive(Clone, Debug)]
pub struct CommandGroupProps {
    pub heading: Option<String>,
}

impl CommandGroupProps {
    pub fn new(heading: impl Into<String>) -> Self {
        Self {
            heading: Some(heading.into()),
        }
    }
}

#[derive(Debug)]
pub struct CommandItemProps<'a, IdSource> {
    pub id_source: IdSource,
    pub label: WidgetText,
    pub keywords: Vec<String>,
    pub icon: Option<Icon>,
    pub shortcut: Option<String>,
    pub disabled: bool,
    pub on_select: Option<OnCommandSelect<'a>>,
}

impl<'a, IdSource: Hash> CommandItemProps<'a, IdSource> {
    pub fn new(id_source: IdSource, label: impl Into<WidgetText>) -> Self {
        Self {
            id_source,
            label: label.into(),
            keywords: Vec::new(),
            icon: None,
            shortcut: None,
            disabled: false,
            on_select: None,
        }
    }

    pub fn keywords(mut self, keywords: &[&str]) -> Self {
        self.keywords = keywords.iter().map(|k| k.to_string()).collect();
        self
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_select(mut self, callback: impl FnMut() + 'a) -> Self {
        self.on_select = Some(OnCommandSelect(Box::new(callback)));
        self
    }
}

#[derive(Clone, Debug, Default)]
struct CommandState {
    query: String,
    selected_index: usize,
    selectable_count: usize,
}

#[derive(Clone, Debug, Default)]
struct CommandRenderState {
    visible_count: usize,
    selectable_count: usize,
    empty_text: Option<String>,
    enter_pressed: bool,
}

#[derive(Clone, Copy, Debug)]
struct CommandTokens {
    bg: Color32,
    text: Color32,
    muted: Color32,
    border: Color32,
    accent: Color32,
    accent_text: Color32,
}

#[derive(Clone, Copy, Debug)]
struct CommandMetrics {
    input_height: f32,
    item_height: f32,
    item_padding: Margin,
    group_padding: Margin,
    separator_margin: f32,
}

fn command_tokens(theme: &Theme) -> CommandTokens {
    CommandTokens {
        bg: theme.palette.popover,
        text: theme.palette.popover_foreground,
        muted: theme.palette.muted_foreground,
        border: theme.palette.border,
        accent: theme.palette.accent,
        accent_text: theme.palette.accent_foreground,
    }
}

fn command_metrics() -> CommandMetrics {
    CommandMetrics {
        input_height: 48.0,
        item_height: 36.0,
        item_padding: Margin::symmetric(8, 6),
        group_padding: Margin::symmetric(8, 4),
        separator_margin: 6.0,
    }
}

pub struct CommandContext<'a> {
    state: &'a mut CommandState,
    render: &'a mut CommandRenderState,
    tokens: CommandTokens,
    metrics: CommandMetrics,
}

// =============================================================================
// Root
// =============================================================================

pub fn command<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: CommandProps,
    add_contents: impl FnOnce(&mut Ui, &mut CommandContext) -> R,
) -> R {
    let state_id = ui.make_persistent_id(props.id_source);
    let mut state = ui
        .ctx()
        .data(|data| data.get_temp::<CommandState>(state_id))
        .unwrap_or_default();

    let (up, down, enter) = ui.input(|i| {
        (
            i.key_pressed(Key::ArrowUp),
            i.key_pressed(Key::ArrowDown),
            i.key_pressed(Key::Enter),
        )
    });

    if state.selectable_count > 0 {
        if down {
            state.selected_index = (state.selected_index + 1) % state.selectable_count;
        } else if up {
            state.selected_index = if state.selected_index == 0 {
                state.selectable_count - 1
            } else {
                state.selected_index - 1
            };
        }
    } else {
        state.selected_index = 0;
    }

    let tokens = command_tokens(theme);
    let metrics = command_metrics();
    let rounding = CornerRadius::same(DEFAULT_RADIUS.r3 as u8);
    let shadow = if props.show_shadow {
        ui.style().visuals.popup_shadow
    } else {
        egui::Shadow::NONE
    };
    let stroke = if props.show_border {
        Stroke::new(1.0, tokens.border)
    } else {
        Stroke::NONE
    };

    let mut render = CommandRenderState {
        enter_pressed: enter,
        ..Default::default()
    };

    let inner = Frame::NONE
        .fill(tokens.bg)
        .stroke(stroke)
        .corner_radius(rounding)
        .shadow(shadow)
        .show(ui, |command_ui| {
            if let Some(min_width) = props.min_width {
                command_ui.set_min_width(min_width);
            }
            command_ui.visuals_mut().override_text_color = Some(tokens.text);
            command_ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            let mut ctx = CommandContext {
                state: &mut state,
                render: &mut render,
                tokens,
                metrics,
            };
            add_contents(command_ui, &mut ctx)
        })
        .inner;

    state.selectable_count = render.selectable_count;
    if state.selectable_count == 0 {
        state.selected_index = 0;
    } else if state.selected_index >= state.selectable_count {
        state.selected_index = state.selectable_count - 1;
    }

    ui.ctx().data_mut(|data| data.insert_temp(state_id, state));

    inner
}

// =============================================================================
// Input
// =============================================================================

pub fn command_input(ui: &mut Ui, ctx: &mut CommandContext, props: CommandInputProps) -> Response {
    let desired = vec2(ui.available_width(), ctx.metrics.input_height);
    let inner = ui.allocate_ui_with_layout(desired, Layout::left_to_right(Align::Center), |row| {
        row.spacing_mut().item_spacing = vec2(8.0, 0.0);
        row.visuals_mut().override_text_color = Some(ctx.tokens.muted);

        let icon_text = RichText::new(Icon::Search.unicode()).size(14.0);
        row.label(icon_text);
        row.visuals_mut().override_text_color = Some(ctx.tokens.text);

        let mut edit = egui::TextEdit::singleline(&mut ctx.state.query)
            .hint_text(props.placeholder)
            .frame(false);
        edit = edit.desired_width(f32::INFINITY);
        let response = row.add(edit);

        if response.changed() {
            ctx.state.selected_index = 0;
        }

        response
    });

    let response = inner.inner;
    let rect = inner.response.rect;
    let stroke = Stroke::new(1.0, ctx.tokens.border);
    ui.painter()
        .line_segment([rect.left_bottom(), rect.right_bottom()], stroke);

    response
}

// =============================================================================
// List and groups
// =============================================================================

pub fn command_list<R>(
    ui: &mut Ui,
    ctx: &mut CommandContext,
    props: CommandListProps,
    add_contents: impl FnOnce(&mut Ui, &mut CommandContext) -> R,
) -> R {
    ctx.render.visible_count = 0;
    ctx.render.selectable_count = 0;
    ctx.render.empty_text = None;

    ScrollArea::vertical()
        .max_height(props.max_height)
        .show(ui, |list_ui| {
            list_ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            let inner = add_contents(list_ui, ctx);
            if ctx.render.visible_count == 0
                && let Some(text) = ctx.render.empty_text.take()
            {
                list_ui.add_space(8.0);
                list_ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.label(RichText::new(text).color(ctx.tokens.muted).size(12.0));
                });
                list_ui.add_space(8.0);
            }
            inner
        })
        .inner
}

pub fn command_empty(ui: &mut Ui, ctx: &mut CommandContext, text: &str) -> Response {
    ctx.render.empty_text = Some(text.to_string());
    ui.allocate_response(Vec2::ZERO, Sense::hover())
}

pub fn command_group<R>(
    ui: &mut Ui,
    ctx: &mut CommandContext,
    props: CommandGroupProps,
    add_contents: impl FnOnce(&mut Ui, &mut CommandContext) -> R,
) -> R {
    Frame::NONE
        .inner_margin(ctx.metrics.group_padding)
        .show(ui, |group_ui| {
            group_ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            if let Some(heading) = props.heading {
                group_ui.label(
                    RichText::new(heading)
                        .size(11.0)
                        .color(ctx.tokens.muted)
                        .strong(),
                );
            }
            add_contents(group_ui, ctx)
        })
        .inner
}

pub fn command_separator(ui: &mut Ui, ctx: &mut CommandContext) -> Response {
    ui.add_space(ctx.metrics.separator_margin);
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter().line_segment(
        [rect.left_center(), rect.right_center()],
        Stroke::new(1.0, ctx.tokens.border),
    );
    ui.add_space(ctx.metrics.separator_margin);
    response
}

// =============================================================================
// Items
// =============================================================================

pub fn command_item<'a, IdSource: Hash>(
    ui: &mut Ui,
    ctx: &mut CommandContext,
    mut props: CommandItemProps<'a, IdSource>,
) -> Option<Response> {
    let query = ctx.state.query.trim();
    let label_text = props.label.text().to_string();
    if !command_matches(query, &label_text, &props.keywords) {
        return None;
    }

    ctx.render.visible_count += 1;
    let selectable_index = if props.disabled {
        None
    } else {
        let index = ctx.render.selectable_count;
        ctx.render.selectable_count += 1;
        Some(index)
    };

    let is_selected = selectable_index == Some(ctx.state.selected_index);
    let rounding = CornerRadius::same(4);
    let desired = vec2(ui.available_width(), ctx.metrics.item_height);
    let item_id = ui.make_persistent_id(&props.id_source);

    let inner = ui.allocate_ui_with_layout(desired, Layout::left_to_right(Align::Center), |row| {
        row.spacing_mut().item_spacing = vec2(8.0, 0.0);
        let rect = row.max_rect();
        let response = row.interact(rect, item_id, Sense::click());
        let hovered = response.hovered();

        let fill = if is_selected {
            ctx.tokens.accent
        } else if hovered {
            ctx.tokens.accent.gamma_multiply(0.35)
        } else {
            Color32::TRANSPARENT
        };

        if fill != Color32::TRANSPARENT {
            row.painter().rect_filled(rect, rounding, fill);
        }

        Frame::NONE
            .inner_margin(ctx.metrics.item_padding)
            .show(row, |content| {
                let mut text_color = ctx.tokens.text;
                if props.disabled {
                    text_color = ctx.tokens.muted;
                } else if is_selected {
                    text_color = ctx.tokens.accent_text;
                }

                if let Some(icon) = props.icon {
                    content.label(RichText::new(icon.unicode()).size(16.0).color(text_color));
                }

                content.label(RichText::new(label_text.as_str()).color(text_color));

                if let Some(shortcut) = props.shortcut.take() {
                    content.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        command_shortcut(ui, ctx, &shortcut);
                    });
                }
            });

        response
    });

    let response = inner.inner;
    if let Some(index) = selectable_index {
        if response.hovered() && !props.disabled {
            ctx.state.selected_index = index;
        }
        if (response.clicked() || ctx.render.enter_pressed && is_selected)
            && !props.disabled
            && let Some(callback) = props.on_select.as_mut()
        {
            (callback.0)();
        }
    }

    Some(response)
}

pub fn command_shortcut(ui: &mut Ui, ctx: &CommandContext, text: &str) -> Response {
    ui.label(
        RichText::new(text)
            .size(10.0)
            .color(ctx.tokens.muted)
            .monospace(),
    )
}

fn command_matches(query: &str, label: &str, keywords: &[String]) -> bool {
    if query.is_empty() {
        return true;
    }
    if fuzzy_match(query, label) {
        return true;
    }
    keywords.iter().any(|kw| fuzzy_match(query, kw))
}

fn fuzzy_match(query: &str, text: &str) -> bool {
    let query_lower = query.to_lowercase();
    let mut q = query_lower.chars();
    let mut q_next = q.next();
    if q_next.is_none() {
        return true;
    }
    for ch in text.to_lowercase().chars() {
        if Some(ch) == q_next {
            q_next = q.next();
            if q_next.is_none() {
                return true;
            }
        }
    }
    false
}

// =============================================================================
// Command dialog
// =============================================================================

#[derive(Debug)]
pub struct CommandDialogProps<'a> {
    pub id_source: Id,
    pub open: &'a mut bool,
    pub title: String,
    pub description: String,
    pub show_close_button: bool,
}

impl<'a> CommandDialogProps<'a> {
    pub fn new(id_source: Id, open: &'a mut bool) -> Self {
        Self {
            id_source,
            open,
            title: "Command Palette".to_string(),
            description: "Search for a command to run...".to_string(),
            show_close_button: true,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }
}

pub fn command_dialog<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: CommandDialogProps<'_>,
    add_contents: impl FnOnce(&mut Ui, &mut CommandContext) -> R,
) -> Option<R> {
    let dialog_props = DialogProps::new(props.id_source, props.open)
        .with_title(props.title)
        .with_description(props.description)
        .scrollable(false)
        .show_close_button(props.show_close_button)
        .with_size(vec2(520.0, 0.0));

    dialog(ui, theme, dialog_props, |dialog_ui| {
        command(
            dialog_ui,
            theme,
            CommandProps::new(props.id_source.with("command"))
                .show_border(false)
                .show_shadow(false),
            add_contents,
        )
    })
}
