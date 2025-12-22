use crate::theme::Theme;
use crate::tokens::ease_out_cubic;
use egui::{
    Align, Context, Id, InnerResponse, Response, Sense, Ui, UiBuilder, UiKind, UiStackInfo, vec2,
};
use log::trace;

#[derive(Clone, Copy, Debug, Default)]
struct CollapsibleInnerState {
    /// Height of the content region when fully open. Used for animations.
    open_height: Option<f32>,
}

fn load_inner_state(ctx: &Context, id: Id) -> CollapsibleInnerState {
    ctx.data_mut(|d| d.get_persisted::<CollapsibleInnerState>(id))
        .unwrap_or_default()
}

fn store_inner_state(ctx: &Context, id: Id, state: CollapsibleInnerState) {
    ctx.data_mut(|d| d.insert_persisted(id, state));
}

fn compute_animated_max_height(
    openness: f32,
    open: bool,
    known_open_height: Option<f32>,
) -> f32 {
    if openness <= 0.0 {
        0.0
    } else if openness >= 1.0 {
        known_open_height.unwrap_or_default().max(0.0)
    } else if open && known_open_height.is_none() {
        // First frame of expansion: show some motion while we measure.
        10.0
    } else {
        let full_height = known_open_height.unwrap_or_default().max(0.0);
        let h = egui::emath::remap_clamp(openness, 0.0..=1.0, 0.0..=full_height);
        h.round()
    }
}

pub struct CollapsibleProps<'a> {
    pub id_source: Id,
    pub open: &'a mut bool,
    pub default_open: bool,
    pub on_open_change: Option<&'a mut dyn FnMut(bool)>,
    pub disabled: bool,
    pub animate: bool,
    pub animation_ms: Option<f32>,
}

impl std::fmt::Debug for CollapsibleProps<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CollapsibleProps")
            .field("id_source", &self.id_source)
            .field("open", &self.open)
            .field("default_open", &self.default_open)
            .field("disabled", &self.disabled)
            .field("animate", &self.animate)
            .field("animation_ms", &self.animation_ms)
            .field("on_open_change", &self.on_open_change.is_some())
            .finish()
    }
}

impl<'a> CollapsibleProps<'a> {
    pub fn new(id_source: Id, open: &'a mut bool) -> Self {
        Self {
            id_source,
            open,
            default_open: false,
            on_open_change: None,
            disabled: false,
            animate: false,
            animation_ms: None,
        }
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_animation(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    pub fn animation_ms(mut self, ms: f32) -> Self {
        self.animation_ms = Some(ms.max(0.0));
        self
    }

    pub fn on_open_change(mut self, cb: &'a mut dyn FnMut(bool)) -> Self {
        self.on_open_change = Some(cb);
        self
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CollapsibleContentProps {
    pub force_mount: bool,
}

impl CollapsibleContentProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }
}

pub struct CollapsibleContext<'a> {
    id_source: Id,
    open: &'a mut bool,
    on_open_change: Option<&'a mut dyn FnMut(bool)>,
    disabled: bool,
    animate: bool,
    animation_ms: Option<f32>,
    theme: &'a Theme,
}

impl<'a> CollapsibleContext<'a> {
    pub fn is_open(&self) -> bool {
        *self.open
    }

    pub fn set_open(&mut self, ui: &Ui, open: bool) {
        if self.disabled || open == *self.open {
            trace!(
                "collapsible set_open ignored disabled={} current_open={} next_open={}",
                self.disabled,
                *self.open,
                open
            );
            return;
        }

        trace!("collapsible set_open current_open={} next_open={}", *self.open, open);
        *self.open = open;
        if let Some(cb) = self.on_open_change.as_mut() {
            cb(open);
        }
        ui.ctx().request_repaint();
    }

    pub fn toggle(&mut self, ui: &Ui) {
        let next = !*self.open;
        self.set_open(ui, next);
    }

    /// Attach a trigger widget. If it is clicked, the collapsible toggles.
    pub fn trigger(
        &mut self,
        ui: &mut Ui,
        add_trigger: impl FnOnce(&mut Ui) -> Response,
    ) -> Response {
        let response = ui
            .push_id(self.id_source.with("trigger"), |ui| add_trigger(ui))
            .inner;
        let (primary_clicked, interact_pos) = ui.ctx().input(|i| (i.pointer.primary_clicked(), i.pointer.interact_pos()));
        let hit = interact_pos.is_some_and(|p| response.rect.contains(p));
        trace!(
            "collapsible trigger: open={} clicked={} hovered={} primary_clicked={} interact_pos={:?} hit={} rect={:?} enabled={}",
            *self.open,
            response.clicked(),
            response.hovered(),
            primary_clicked,
            interact_pos,
            hit,
            response.rect,
            ui.is_enabled()
        );
        if response.clicked() {
            self.toggle(ui);
        }
        response
    }

    /// Render collapsible content with Radix-like mount/animation semantics.
    ///
    /// Returns `None` when the content is unmounted (`open == false`, `openness == 0`, and
    /// `force_mount == false`).
    pub fn content<R>(
        &mut self,
        ui: &mut Ui,
        add_body: impl FnOnce(&mut Ui) -> R,
    ) -> Option<InnerResponse<R>> {
        self.content_with_props(ui, CollapsibleContentProps::default(), add_body)
    }

    /// Render collapsible content with Radix-like mount semantics.
    ///
    /// Returns `None` when the content is unmounted (fully closed and `force_mount == false`).
    pub fn content_with_props<R>(
        &mut self,
        ui: &mut Ui,
        content_props: CollapsibleContentProps,
        add_body: impl FnOnce(&mut Ui) -> R,
    ) -> Option<InnerResponse<R>> {
        let ctx = ui.ctx().clone();
        let duration_s = self
            .animation_ms
            .unwrap_or(self.theme.motion.base_ms)
            .max(0.0)
            / 1000.0;
        let openness = if ctx.memory(|m| m.everything_is_visible()) {
            1.0
        } else if !self.animate {
            if *self.open { 1.0 } else { 0.0 }
        } else {
            ctx.animate_bool_with_time_and_easing(
                self.id_source.with("open-anim"),
                *self.open,
                duration_s,
                ease_out_cubic,
            )
        };

        let state_id = self.id_source.with("content-height");
        let mut state = load_inner_state(&ctx, state_id);

        trace!(
            "collapsible content: open={} animate={} force_mount={} openness={} open_height={:?}",
            *self.open,
            self.animate,
            content_props.force_mount,
            openness,
            state.open_height
        );

        let mut add_body = Some(add_body);

        if !self.animate {
            if !*self.open && !content_props.force_mount {
                return None;
            }

            if *self.open {
                let inner_layout = egui::Layout::top_down(Align::Min).with_main_align(Align::Min);
                let inner = ui
                    .push_id(self.id_source.with("content"), |ui| {
                        ui.with_layout(inner_layout, |ui| {
                            if self.disabled {
                                ui.disable();
                            }
                            add_body
                                .take()
                                .expect("collapsible content body should only be called once")(
                                ui,
                            )
                        })
                    })
                    .inner;

                state.open_height = Some(inner.response.rect.height().max(0.0));
                store_inner_state(&ctx, state_id, state);

                return Some(inner);
            }
        }

        if openness <= 0.0 && !content_props.force_mount {
            return None;
        }

        if openness >= 1.0 && state.open_height.is_none() {
            // When the content is fully open but we don't know the full height yet, measure it
            // with a normal pass (no height constraints), store it, and render it.
            let inner_layout = egui::Layout::top_down(Align::Min).with_main_align(Align::Min);
            let inner = ui
                .push_id(self.id_source.with("content"), |ui| {
                    ui.with_layout(inner_layout, |ui| {
                        if self.disabled {
                            ui.disable();
                        }
                        add_body
                            .take()
                            .expect("collapsible content body should only be called once")(ui)
                    })
                })
                .inner;

            state.open_height = Some(inner.response.rect.height().max(0.0));
            store_inner_state(&ctx, state_id, state);
            return Some(inner);
        }

        let max_height = compute_animated_max_height(openness, *self.open, state.open_height);

        let width = ui.available_width();
        let (outer_rect, outer_response) =
            ui.allocate_exact_size(vec2(width, max_height), Sense::hover());

        let measure_height = state.open_height.unwrap_or(4096.0).max(10.0);

        let measure_rect = egui::Rect::from_min_size(
            outer_rect.min,
            vec2(outer_rect.width(), measure_height),
        );

        let child_layout = egui::Layout::top_down(Align::Min).with_main_align(Align::Min);
        let inner = ui
            .scope_builder(
                UiBuilder::new()
                    .max_rect(measure_rect)
                    .layout(child_layout)
                    .ui_stack_info(UiStackInfo::new(UiKind::Collapsible)),
                |child_ui| {
                    child_ui.set_clip_rect(child_ui.clip_rect().intersect(outer_rect));
                    if self.disabled {
                        child_ui.disable();
                    }

                    let inner = child_ui.push_id(self.id_source.with("content"), |ui| {
                        add_body
                            .take()
                            .expect("collapsible content body should only be called once")(ui)
                    });

                    (inner.inner, inner.response.rect.height().max(0.0))
                },
            )
            .inner;

        state.open_height = Some(inner.1);
        store_inner_state(&ctx, state_id, state);

        Some(InnerResponse {
            inner: inner.0,
            response: outer_response,
        })
    }
}

/// Root scope for Radix-like Collapsible.
///
/// This is a composition primitive: call [`CollapsibleContext::trigger`] and
/// [`CollapsibleContext::content`] from inside `add_contents` to build your layout.
pub fn collapsible<'a, R>(
    ui: &mut Ui,
    theme: &'a Theme,
    mut props: CollapsibleProps<'a>,
    add_contents: impl FnOnce(&mut Ui, &mut CollapsibleContext<'a>) -> R,
) -> R {
    let ctx = ui.ctx();
    let init_id = props.id_source.with("default-open-initialized");
    let initialized = ctx.data(|d| d.get_temp::<bool>(init_id)).unwrap_or(false);
    if !initialized {
        if props.default_open {
            *props.open = true;
        }
        ctx.data_mut(|d| d.insert_temp(init_id, true));
    }

    let mut api = CollapsibleContext {
        id_source: props.id_source,
        open: props.open,
        on_open_change: props.on_open_change.take(),
        disabled: props.disabled,
        animate: props.animate,
        animation_ms: props.animation_ms,
        theme,
    };

    if props.disabled {
        ui.scope(|ui| {
            ui.disable();
            add_contents(ui, &mut api)
        })
        .inner
    } else {
        add_contents(ui, &mut api)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animated_max_height_closed_is_zero() {
        assert_eq!(compute_animated_max_height(0.0, false, Some(100.0)), 0.0);
        assert_eq!(compute_animated_max_height(-1.0, true, Some(100.0)), 0.0);
    }

    #[test]
    fn animated_max_height_first_open_frame_uses_placeholder() {
        assert_eq!(compute_animated_max_height(0.25, true, None), 10.0);
    }

    #[test]
    fn animated_max_height_interpolates_to_full_height() {
        assert_eq!(compute_animated_max_height(0.5, false, Some(200.0)), 100.0);
        assert_eq!(compute_animated_max_height(1.0, true, Some(200.0)), 200.0);
    }
}
