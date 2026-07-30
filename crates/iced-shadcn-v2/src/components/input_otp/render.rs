//! Custom iced widget: focus, keyboard editing, paste, and slot rendering.
//!
//! The web component hides one real `<input>` under purely visual slots; this
//! widget plays both roles itself. It takes focus on click (or through
//! [`iced_core::widget::operation::focusable`] with [`super::InputOtp::id`]),
//! consumes typed characters, Backspace, and clipboard shortcuts, and draws
//! every group, divider, ring, character, fake caret, and separator directly,
//! so the visuals always match the `.cn-input-otp*` recipe of the active pack.

use iced_core::Renderer as CoreRenderer;
use iced_core::text::Renderer as TextRenderer;
use iced_core::{clipboard, keyboard, text};

use crate::iced_compat::advanced::layout::{self, Layout};
use crate::iced_compat::advanced::widget::{Operation, Tree, operation, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, renderer};
use crate::iced_compat::time::{Duration, Instant};
use crate::iced_compat::{
    Border, Color, Element, Event, Length, Pixels, Point, Rectangle, Size, alignment, border,
    mouse, touch, widget, window,
};

use shadcn_common::AccentColor;

use super::geometry::{OtpMetrics, SLOT_BORDER_WIDTH};
use super::style;
use super::types::{InputOtpPattern, InputOtpRadius, InputOtpStatus, InputOtpStyle};
use super::{InputOtp, OnComplete, OnInput, PasteTransformer, StyleOverride};
use crate::fonts::iced_font;
use crate::theme::Theme;

/// Redraw cadence of the continuous caret-blink animation.
const FRAME_INTERVAL: Duration = Duration::from_millis(16);
/// `animate-caret-blink` cycle (`duration-1000`).
const CARET_BLINK_CYCLE_MILLIS: u128 = 1000;
/// `.cn-input-otp-caret-line` is `h-4 w-px`.
const CARET_SIZE: Size = Size::new(1.0, 16.0);

pub(super) fn build<'a, Message>(otp: InputOtp<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let InputOtp {
        theme,
        value,
        max_length,
        groups,
        pattern,
        radius,
        color,
        slot_size,
        text_size,
        disabled,
        invalid,
        id,
        on_input,
        on_complete,
        on_submit,
        paste_transformer,
        style_override,
    } = otp;

    Element::new(OtpWidget {
        theme,
        value: value.into_owned(),
        max_length,
        groups: super::geometry::normalize_groups(max_length, &groups),
        pattern,
        radius,
        color,
        slot_size,
        text_size,
        disabled,
        invalid,
        id,
        on_input,
        on_complete,
        on_submit,
        paste_transformer,
        style_override,
        last_status: None,
    })
}

struct OtpWidget<'a, Message> {
    theme: &'a Theme,
    value: String,
    max_length: usize,
    groups: Vec<usize>,
    pattern: InputOtpPattern,
    radius: Option<InputOtpRadius>,
    color: Option<AccentColor>,
    slot_size: Option<f32>,
    text_size: Option<f32>,
    disabled: bool,
    invalid: bool,
    id: Option<widget::Id>,
    on_input: Option<OnInput<'a, Message>>,
    on_complete: Option<OnComplete<'a, Message>>,
    on_submit: Option<Message>,
    paste_transformer: Option<PasteTransformer<'a>>,
    style_override: Option<StyleOverride<'a>>,
    last_status: Option<InputOtpStatus>,
}

/// Runtime interaction state of one [`OtpWidget`].
#[derive(Debug, Default)]
struct OtpState {
    focused: bool,
    focus_started: Option<Instant>,
    now: Option<Instant>,
    hovered: bool,
    keyboard_modifiers: keyboard::Modifiers,
}

impl operation::Focusable for OtpState {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
        self.focus_started = Some(Instant::now());
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}

impl<Message> OtpWidget<'_, Message>
where
    Message: Clone,
{
    fn metrics(&self) -> OtpMetrics {
        let pack = style::pack_recipe(self.theme.style_id());
        OtpMetrics {
            slot_size: self.slot_size.unwrap_or(pack.slot_size_px).max(1.0),
            slot_gap: pack.slot_gap,
            ring_width: pack.ring_width,
            separator: self.groups.len() > 1,
        }
    }

    fn is_interactive(&self) -> bool {
        !self.disabled && self.on_input.is_some()
    }

    fn status(&self, state: &OtpState) -> InputOtpStatus {
        InputOtpStatus {
            focused: state.focused && self.is_interactive(),
            hovered: state.hovered,
            disabled: self.disabled,
            invalid: self.invalid,
        }
    }

    fn resolve_style(&self, status: InputOtpStatus) -> InputOtpStyle {
        let mut resolved = style::resolve_style(self.theme, self.radius, self.color, status);
        if let Some(override_fn) = self.style_override.as_ref() {
            resolved = override_fn(resolved, status);
        }
        resolved
    }

    /// Publishes the edited value plus `on_complete` once every slot fills.
    fn publish_value(&self, next: String, shell: &mut Shell<'_, Message>) {
        let is_complete = char_count(&next, self.max_length) >= self.max_length;

        if let Some(on_input) = self.on_input.as_ref() {
            shell.publish(on_input(next.clone()));
        }
        if is_complete && let Some(on_complete) = self.on_complete.as_ref() {
            shell.publish(on_complete(next));
        }
    }
}

impl<Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for OtpWidget<'_, Message>
where
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<OtpState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(OtpState::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &crate::iced_compat::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let intrinsic = self.metrics().total_size(&self.groups);
        layout::Node::new(limits.resolve(Length::Shrink, Length::Shrink, intrinsic))
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &crate::iced_compat::Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<OtpState>();
        operation.focusable(self.id.as_ref(), layout.bounds(), state);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &crate::iced_compat::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<OtpState>();
        let interactive = self.is_interactive();
        let bounds = layout.bounds();

        match event {
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                state.keyboard_modifiers = *modifiers;
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(bounds) && interactive {
                    if !state.focused {
                        state.focused = true;
                        state.focus_started = Some(Instant::now());
                    }
                    shell.capture_event();
                    shell.request_redraw();
                } else if state.focused {
                    state.focused = false;
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                state.hovered = cursor.is_over(bounds);
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                modified_key,
                physical_key,
                text,
                ..
            }) if state.focused && interactive => {
                let modifiers = state.keyboard_modifiers;

                match key.to_latin(*physical_key) {
                    Some('v') if modifiers.command() && !modifiers.alt() => {
                        let pasted: String = clipboard
                            .read(clipboard::Kind::Standard)
                            .unwrap_or_default()
                            .chars()
                            .filter(|c| !c.is_control())
                            .collect();
                        let pasted = match self.paste_transformer.as_ref() {
                            Some(transform) => transform(pasted),
                            None => pasted,
                        };

                        if let Some(next) =
                            apply_paste(&self.value, &pasted, self.pattern, self.max_length)
                        {
                            self.publish_value(next, shell);
                            state.focus_started = Some(Instant::now());
                        }
                        shell.capture_event();
                        return;
                    }
                    Some('c') if modifiers.command() => {
                        if !self.value.is_empty() {
                            clipboard.write(clipboard::Kind::Standard, self.value.clone());
                        }
                        shell.capture_event();
                        return;
                    }
                    Some('x') if modifiers.command() => {
                        if !self.value.is_empty() {
                            clipboard.write(clipboard::Kind::Standard, self.value.clone());
                            self.publish_value(String::new(), shell);
                        }
                        shell.capture_event();
                        return;
                    }
                    _ => {}
                }

                if let Some(text) = text
                    && text.chars().any(|c| !c.is_control())
                {
                    if let Some(next) =
                        append_text(&self.value, text, self.pattern, self.max_length)
                    {
                        self.publish_value(next, shell);
                        state.focus_started = Some(Instant::now());
                    }

                    // Pattern-rejected characters are swallowed too, like the
                    // web input.
                    shell.capture_event();
                    return;
                }

                match modified_key.as_ref() {
                    keyboard::Key::Named(keyboard::key::Named::Backspace) => {
                        if !self.value.is_empty() {
                            let next = if modifiers.command() || modifiers.jump() {
                                String::new()
                            } else {
                                without_last_char(&self.value)
                            };
                            self.publish_value(next, shell);
                            state.focus_started = Some(Instant::now());
                        }
                        shell.capture_event();
                    }
                    keyboard::Key::Named(keyboard::key::Named::Enter) => {
                        if let Some(on_submit) = self.on_submit.clone() {
                            shell.publish(on_submit);
                            shell.capture_event();
                        }
                    }
                    keyboard::Key::Named(keyboard::key::Named::Escape) => {
                        state.focused = false;
                        shell.request_redraw();
                        shell.capture_event();
                    }
                    _ => {}
                }
            }
            Event::Window(window::Event::RedrawRequested(now)) => {
                state.now = Some(*now);
                self.last_status = Some(self.status(state));

                // The fake caret fades continuously, so keep frames coming
                // while it is on screen.
                if state.focused
                    && interactive
                    && char_count(&self.value, self.max_length) < self.max_length
                {
                    shell.request_redraw_at(*now + FRAME_INTERVAL);
                }
                return;
            }
            _ => {}
        }

        let status = self.status(state);
        if self
            .last_status
            .is_some_and(|last_status| last_status != status)
        {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &crate::iced_compat::Renderer,
    ) -> mouse::Interaction {
        if !cursor.is_over(layout.bounds()) {
            mouse::Interaction::default()
        } else if self.disabled {
            // `disabled:cursor-not-allowed` on the web root.
            mouse::Interaction::NotAllowed
        } else if self.is_interactive() {
            mouse::Interaction::Text
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut crate::iced_compat::Renderer,
        _iced_theme: &crate::iced_compat::Theme,
        _defaults: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let state = tree.state.downcast_ref::<OtpState>();
        let status = self.last_status.unwrap_or_else(|| self.status(state));
        let resolved = self.resolve_style(status);
        let metrics = self.metrics();
        let regions = metrics.regions(bounds, &self.groups);

        let chars: Vec<char> = self.value.chars().take(self.max_length).collect();
        let active_slot = (status.focused && !status.disabled)
            .then(|| chars.len().min(self.max_length.saturating_sub(1)));
        let show_caret = active_slot.is_some() && chars.len() < self.max_length;

        let text_size = self
            .text_size
            .unwrap_or(style::pack_recipe(self.theme.style_id()).text_size_px);
        let font = iced_font(self.theme.font_pack().sans);

        let mut first_index = 0;
        for (group, count) in regions.group_bounds.iter().zip(&self.groups) {
            draw_group(
                renderer,
                &resolved,
                &metrics,
                *group,
                *count,
                first_index,
                active_slot,
            );

            for index_in_group in 0..*count {
                let slot = metrics.slot_bounds(*group, index_in_group);
                let index = first_index + index_in_group;

                if let Some(entered) = chars.get(index) {
                    renderer.fill_text(
                        text::Text {
                            content: entered.to_string(),
                            bounds: slot.size(),
                            size: Pixels(text_size),
                            line_height: text::LineHeight::default(),
                            font,
                            align_x: text::Alignment::Center,
                            align_y: alignment::Vertical::Center,
                            shaping: text::Shaping::default(),
                            wrapping: text::Wrapping::default(),
                        },
                        slot.center(),
                        resolved.slot_text,
                        *viewport,
                    );
                } else if show_caret && active_slot == Some(index) {
                    draw_caret(renderer, &resolved, state, slot);
                }
            }

            first_index += count;
        }

        for separator in &regions.separator_bounds {
            draw_separator(renderer, &resolved, *separator);
        }
    }
}

/// Paints one group surface: fill, outer border, and inner slot dividers —
/// or the per-slot underlines on Sera — plus the active ring treatment.
fn draw_group(
    renderer: &mut crate::iced_compat::Renderer,
    resolved: &InputOtpStyle,
    metrics: &OtpMetrics,
    group: Rectangle,
    count: usize,
    first_index: usize,
    active_slot: Option<usize>,
) {
    let corner = resolved.radius.clamp(0.0, metrics.slot_size / 2.0);

    if resolved.underline_only {
        for index_in_group in 0..count {
            let slot = metrics.slot_bounds(group, index_in_group);
            let is_active = active_slot == Some(first_index + index_in_group);

            if resolved.slot_background.a > f32::EPSILON {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: slot,
                        ..renderer::Quad::default()
                    },
                    resolved.slot_background,
                );
            }

            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle::new(
                        Point::new(slot.x, slot.y + slot.height - SLOT_BORDER_WIDTH),
                        Size::new(slot.width, SLOT_BORDER_WIDTH),
                    ),
                    ..renderer::Quad::default()
                },
                if is_active {
                    resolved.active_border
                } else {
                    resolved.slot_border
                },
            );
        }
        return;
    }

    renderer.fill_quad(
        renderer::Quad {
            bounds: group,
            border: Border {
                color: resolved.slot_border,
                width: SLOT_BORDER_WIDTH,
                radius: corner.into(),
            },
            ..renderer::Quad::default()
        },
        resolved.slot_background,
    );

    for divider in 1..count {
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle::new(
                    Point::new(
                        group.x + metrics.slot_size * divider as f32 - SLOT_BORDER_WIDTH / 2.0,
                        group.y,
                    ),
                    Size::new(SLOT_BORDER_WIDTH, group.height),
                ),
                ..renderer::Quad::default()
            },
            resolved.slot_border,
        );
    }

    let Some(active) = active_slot else { return };
    if active < first_index || active >= first_index + count {
        return;
    }

    let index_in_group = active - first_index;
    let slot = metrics.slot_bounds(group, index_in_group);
    let radius = slot_corner_radius(corner, index_in_group, count);

    // The translucent `ring-*` halo: a border-only quad around the slot.
    if resolved.ring_width > 0.0 && resolved.ring.a > f32::EPSILON {
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle::new(
                    Point::new(slot.x - resolved.ring_width, slot.y - resolved.ring_width),
                    Size::new(
                        slot.width + resolved.ring_width * 2.0,
                        slot.height + resolved.ring_width * 2.0,
                    ),
                ),
                border: Border {
                    color: resolved.ring,
                    width: resolved.ring_width,
                    radius: expand_radius(radius, resolved.ring_width),
                },
                ..renderer::Quad::default()
            },
            Color::TRANSPARENT,
        );
    }

    // `data-[active=true]:border-ring` with `z-10` — outline the whole slot.
    renderer.fill_quad(
        renderer::Quad {
            bounds: slot,
            border: Border {
                color: resolved.active_border,
                width: SLOT_BORDER_WIDTH,
                radius,
            },
            ..renderer::Quad::default()
        },
        Color::TRANSPARENT,
    );
}

/// `first:rounded-l-* last:rounded-r-*`: only the outer corners round.
fn slot_corner_radius(corner: f32, index_in_group: usize, count: usize) -> border::Radius {
    let mut radius = border::Radius::from(0.0);
    if index_in_group == 0 {
        radius.top_left = corner;
        radius.bottom_left = corner;
    }
    if index_in_group + 1 == count {
        radius.top_right = corner;
        radius.bottom_right = corner;
    }
    radius
}

/// Grows rounded corners by the ring width so the halo hugs the slot shape.
fn expand_radius(radius: border::Radius, by: f32) -> border::Radius {
    let expand = |corner: f32| if corner > 0.0 { corner + by } else { 0.0 };
    border::Radius {
        top_left: expand(radius.top_left),
        top_right: expand(radius.top_right),
        bottom_right: expand(radius.bottom_right),
        bottom_left: expand(radius.bottom_left),
    }
}

fn draw_caret(
    renderer: &mut crate::iced_compat::Renderer,
    resolved: &InputOtpStyle,
    state: &OtpState,
    slot: Rectangle,
) {
    let phase = match (state.focus_started, state.now) {
        (Some(started), Some(now)) => {
            let elapsed = now.saturating_duration_since(started);
            (elapsed.as_millis() % CARET_BLINK_CYCLE_MILLIS) as f32
                / CARET_BLINK_CYCLE_MILLIS as f32
        }
        _ => 0.0,
    };

    let opacity = caret_opacity(phase);
    if opacity <= f32::EPSILON {
        return;
    }

    renderer.fill_quad(
        renderer::Quad {
            bounds: Rectangle::new(
                Point::new(
                    slot.center_x() - CARET_SIZE.width / 2.0,
                    slot.center_y() - CARET_SIZE.height / 2.0,
                ),
                CARET_SIZE,
            ),
            ..renderer::Quad::default()
        },
        Color {
            a: resolved.caret.a * opacity,
            ..resolved.caret
        },
    );
}

/// Minus glyph between groups (Lucide `MinusIcon`: a rounded 14/24 bar).
fn draw_separator(
    renderer: &mut crate::iced_compat::Renderer,
    resolved: &InputOtpStyle,
    bounds: Rectangle,
) {
    let width = bounds.width * 14.0 / 24.0;
    let height = (bounds.height * 2.0 / 24.0).max(1.0);

    renderer.fill_quad(
        renderer::Quad {
            bounds: Rectangle::new(
                Point::new(
                    bounds.center_x() - width / 2.0,
                    bounds.center_y() - height / 2.0,
                ),
                Size::new(width, height),
            ),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: (height / 2.0).into(),
            },
            ..renderer::Quad::default()
        },
        resolved.separator,
    );
}

/// `animate-caret-blink` opacity: `0%,70%,100% → 1` and `20%,50% → 0`.
pub(super) fn caret_opacity(phase: f32) -> f32 {
    let phase = phase.rem_euclid(1.0);
    if phase < 0.2 {
        1.0 - phase / 0.2
    } else if phase < 0.5 {
        0.0
    } else if phase < 0.7 {
        (phase - 0.5) / 0.2
    } else {
        1.0
    }
}

/// Number of entered characters, capped at `max_length`.
pub(super) fn char_count(value: &str, max_length: usize) -> usize {
    value.chars().take(max_length).count()
}

/// Appends the accepted characters of `typed`; `None` when nothing fits.
pub(super) fn append_text(
    current: &str,
    typed: &str,
    pattern: InputOtpPattern,
    max_length: usize,
) -> Option<String> {
    let mut next = current.to_owned();
    let mut count = current.chars().count();
    let mut changed = false;

    for character in typed.chars() {
        if count >= max_length {
            break;
        }
        if !character.is_control() && pattern.accepts(character) {
            next.push(character);
            count += 1;
            changed = true;
        }
    }

    changed.then_some(next)
}

/// Applies pasted text: appends to a partial value, replaces a full one.
pub(super) fn apply_paste(
    current: &str,
    pasted: &str,
    pattern: InputOtpPattern,
    max_length: usize,
) -> Option<String> {
    let base = if current.chars().count() >= max_length {
        ""
    } else {
        current
    };

    let next = append_text(base, pasted, pattern, max_length)?;
    (next != current).then_some(next)
}

/// Removes the last character (Backspace with the caret at the end).
pub(super) fn without_last_char(current: &str) -> String {
    let mut next = current.to_owned();
    let _ = next.pop();
    next
}
