//! Custom widget + default trigger chrome for the file drop zone.

use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::fonts::iced_font;
use crate::iced_compat::advanced::layout;
use crate::iced_compat::advanced::renderer;
use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::Operation;
use crate::iced_compat::advanced::widget::tree::{self, Tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget};
use crate::iced_compat::alignment::Horizontal;
use crate::iced_compat::widget::canvas::{self, Frame, Geometry, Path as CanvasPath, Stroke};
use crate::iced_compat::widget::text::{LineHeight, Wrapping};
use crate::iced_compat::widget::{
    canvas as canvas_widget, column, container, stack, text as iced_text,
};
use crate::iced_compat::{
    Background, Border, Color, Element, Event, Length, Padding, Point, Rectangle, Size, mouse,
    window,
};
use crate::theme::Theme;

use shadcn_common::{
    DEFAULT_TRIGGER_LABEL, FileCandidate, FileDropZoneConfig, FileRejectedReason,
    file_drop_zone_can_upload, file_drop_zone_default_hint, guess_mime, should_accept_file,
};

use super::geometry;
use super::style::{self, FileDropZoneStyle};
use super::types::{FileDropZoneAction, FileDropZoneFile, FileDropZoneState, FileDropZoneVariant};

static NEXT_DROP_ZONE_ID: AtomicU64 = AtomicU64::new(1);

thread_local! {
    static ACTIVE_DROP_TARGET: std::cell::RefCell<Option<u64>> =
        const { std::cell::RefCell::new(None) };
}

#[derive(Debug, Default)]
struct WidgetLocalState {
    instance_id: u64,
    hovering_files: bool,
    drop_batch_count: usize,
    last_cursor_over: bool,
}

/// Wraps child content with dashed-border painting and file-drop interaction.
pub(super) struct FileDropZoneWidget<'a, Message> {
    content: Element<'a, Message>,
    theme: Theme,
    config: FileDropZoneConfig,
    state: FileDropZoneState,
    variant: FileDropZoneVariant,
    width: Length,
    height: Length,
    on_action: Rc<dyn Fn(FileDropZoneAction) -> Message + 'a>,
}

impl<'a, Message> FileDropZoneWidget<'a, Message> {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        content: Element<'a, Message>,
        theme: &Theme,
        config: FileDropZoneConfig,
        state: FileDropZoneState,
        variant: FileDropZoneVariant,
        width: Length,
        height: Length,
        on_action: Rc<dyn Fn(FileDropZoneAction) -> Message + 'a>,
    ) -> Self {
        Self {
            content,
            theme: theme.clone(),
            config,
            state,
            variant,
            width,
            height,
            on_action,
        }
    }
}

impl<Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for FileDropZoneWidget<'_, Message>
where
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<WidgetLocalState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(WidgetLocalState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(self.content.as_widget())]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.content.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &crate::iced_compat::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let metrics = geometry::metrics(&self.theme);
        let min_height = metrics.recipe.height_px;
        let child_limits = (*limits).width(self.width).min_height(min_height);
        let child =
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &child_limits);
        let size = limits.resolve(self.width, self.height, child.size());
        layout::Node::with_children(size, vec![child])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: layout::Layout<'_>,
        renderer: &crate::iced_compat::Renderer,
        operation: &mut dyn Operation,
    ) {
        if let Some(child_layout) = layout.children().next() {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                child_layout,
                renderer,
                operation,
            );
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &crate::iced_compat::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let Some(child_layout) = layout.children().next() else {
            return;
        };

        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            child_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        let can_upload = file_drop_zone_can_upload(&self.config);
        let bounds = layout.bounds();
        let is_over = cursor.is_over(bounds);
        let local = tree.state.downcast_mut::<WidgetLocalState>();
        if local.instance_id == 0 {
            local.instance_id = NEXT_DROP_ZONE_ID.fetch_add(1, Ordering::Relaxed);
        }

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                local.last_cursor_over = is_over;
                if is_over != self.state.hovered {
                    shell.publish((self.on_action)(FileDropZoneAction::Hovered(is_over)));
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                if is_over && can_upload =>
            {
                shell.publish((self.on_action)(FileDropZoneAction::PickerRequested));
                shell.capture_event();
            }
            Event::Window(window::Event::FileHovered(_path)) => {
                let cursor_known = cursor.position().is_some();
                let prefer_target = if cursor_known {
                    is_over
                } else {
                    local.last_cursor_over
                };
                let is_target =
                    can_upload && claim_or_match_drop_target(local.instance_id, prefer_target);
                if is_target {
                    local.hovering_files = true;
                    shell.publish((self.on_action)(FileDropZoneAction::Hovered(true)));
                }
            }
            Event::Window(window::Event::FileDropped(path)) => {
                let is_target =
                    can_upload && (cursor.is_over(bounds) || is_drop_target(local.instance_id));
                if is_target {
                    let file_number =
                        self.config.file_count.unwrap_or(0) + local.drop_batch_count + 1;
                    if let Some(reason) = validate_path(path, file_number, &self.config) {
                        shell.publish((self.on_action)(FileDropZoneAction::Rejected {
                            path: path.clone(),
                            reason,
                        }));
                    } else {
                        local.drop_batch_count += 1;
                        shell.publish((self.on_action)(FileDropZoneAction::DropPaths(vec![
                            path.clone(),
                        ])));
                    }
                    shell.capture_event();
                    if is_drop_target(local.instance_id) {
                        clear_drop_target(local.instance_id);
                    }
                }
            }
            Event::Window(window::Event::FilesHoveredLeft) => {
                let was_hovering = local.hovering_files;
                local.hovering_files = false;
                local.drop_batch_count = 0;
                local.last_cursor_over = false;
                if was_hovering || self.state.hovered {
                    shell.publish((self.on_action)(FileDropZoneAction::Hovered(false)));
                }
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &crate::iced_compat::Renderer,
    ) -> mouse::Interaction {
        let Some(child_layout) = layout.children().next() else {
            return mouse::Interaction::default();
        };

        if !file_drop_zone_can_upload(&self.config) {
            return self.content.as_widget().mouse_interaction(
                &tree.children[0],
                child_layout,
                cursor,
                viewport,
                renderer,
            );
        }

        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            self.content.as_widget().mouse_interaction(
                &tree.children[0],
                child_layout,
                cursor,
                viewport,
                renderer,
            )
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut crate::iced_compat::Renderer,
        theme: &crate::iced_compat::Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let metrics = geometry::metrics(&self.theme);
        let resolved = style::resolve(&self.theme, self.variant, &self.state, &self.config);
        let bg = resolved.background.scale_alpha(resolved.opacity);
        let border = resolved.border.scale_alpha(resolved.opacity);

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    radius: metrics.radius_px.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Default::default(),
                ..renderer::Quad::default()
            },
            Background::Color(bg),
        );

        if let Some(child_layout) = layout.children().next() {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
            );
        }

        // Dashed outline is drawn by the stacked canvas in the default trigger
        // builder; for custom children the border is painted here as a solid
        // fallback when no overlay canvas is present. The default trigger
        // always supplies its own dashed overlay, so this solid stroke stays
        // transparent there (border width 0 above). For custom surfaces we
        // still want a dashed look — see [`build_surface`].
        let _ = border;
    }
}

/// Builds the default extras trigger (icon ring + label + optional hint).
pub(super) fn build_default_trigger<'a, Message>(
    theme: &'a Theme,
    config: &FileDropZoneConfig,
    state: &FileDropZoneState,
    variant: FileDropZoneVariant,
    width: Length,
    on_action: Rc<dyn Fn(FileDropZoneAction) -> Message + 'a>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let metrics = geometry::metrics(theme);
    let recipe = metrics.recipe;
    let resolved = style::resolve(theme, variant, state, config);
    let font = iced_font(theme.font_id());

    let title = iced_text(DEFAULT_TRIGGER_LABEL)
        .size(recipe.title.size_px)
        .line_height(LineHeight::Absolute(recipe.title.line_height_px.into()))
        .font(font)
        .color(resolved.foreground.scale_alpha(resolved.opacity))
        .wrapping(Wrapping::Word)
        .width(Length::Fill);

    let mut text_col = column![title]
        .spacing(recipe.text_gap_px)
        .width(Length::Fill)
        .align_x(Horizontal::Center);

    if let Some(hint) = file_drop_zone_default_hint(config) {
        text_col = text_col.push(
            iced_text(hint)
                .size(recipe.hint.size_px)
                .line_height(LineHeight::Absolute(recipe.hint.line_height_px.into()))
                .font(font)
                .color(resolved.hint.scale_alpha(resolved.opacity))
                .wrapping(Wrapping::Word)
                .width(Length::Fill),
        );
    }

    let icon = canvas_widget(UploadIcon {
        color: resolved.foreground.scale_alpha(resolved.opacity),
        ring: resolved.icon_ring.scale_alpha(resolved.opacity),
        circle_px: recipe.icon_circle_px,
        icon_px: recipe.icon_px,
        stroke_viewbox: shadcn_common::FILE_DROP_ZONE_ICON_STROKE_VIEWBOX,
        viewbox: shadcn_common::FILE_DROP_ZONE_ICON_VIEWBOX,
    })
    .width(Length::Fixed(recipe.icon_circle_px))
    .height(Length::Fixed(recipe.icon_circle_px));

    let body = column![icon, text_col]
        .spacing(recipe.gap_px)
        .width(Length::Fill)
        .align_x(Horizontal::Center);

    let padded = container(body)
        .padding(Padding::from(recipe.padding_px))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill);

    let dashed = canvas_widget(DashedBorder {
        color: resolved.border.scale_alpha(resolved.opacity),
        width: recipe.border_width_px,
        radius: metrics.radius_px,
    })
    .width(Length::Fill)
    .height(Length::Fill);

    let stacked = stack![padded, dashed]
        .width(Length::Fill)
        .height(Length::Fill);

    let widget = FileDropZoneWidget::new(
        stacked.into(),
        theme,
        config.clone(),
        *state,
        variant,
        width,
        Length::Fixed(recipe.height_px),
        on_action,
    );
    Element::new(widget)
}

/// Wraps arbitrary content with drop / click behaviour and a dashed border.
#[allow(clippy::too_many_arguments)]
pub(super) fn build_surface<'a, Message>(
    theme: &'a Theme,
    config: &FileDropZoneConfig,
    state: &FileDropZoneState,
    variant: FileDropZoneVariant,
    width: Length,
    height: Length,
    child: Element<'a, Message>,
    on_action: Rc<dyn Fn(FileDropZoneAction) -> Message + 'a>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let metrics = geometry::metrics(theme);
    let recipe = metrics.recipe;
    let resolved = style::resolve(theme, variant, state, config);

    let padded = container(child)
        .padding(Padding::from(recipe.padding_px))
        .width(Length::Fill)
        .height(Length::Fill);

    let dashed = canvas_widget(DashedBorder {
        color: resolved.border.scale_alpha(resolved.opacity),
        width: recipe.border_width_px,
        radius: metrics.radius_px,
    })
    .width(Length::Fill)
    .height(Length::Fill);

    let stacked = stack![padded, dashed]
        .width(Length::Fill)
        .height(Length::Fill);

    let widget = FileDropZoneWidget::new(
        stacked.into(),
        theme,
        config.clone(),
        *state,
        variant,
        width,
        height,
        on_action,
    );
    Element::new(widget)
}

/// Reads file bytes for accepted paths.
#[must_use]
pub fn load_files(paths: &[PathBuf]) -> Vec<FileDropZoneFile> {
    paths
        .iter()
        .filter_map(|path| {
            let bytes = std::fs::read(path).ok()?;
            let name = path.file_name()?.to_string_lossy().into_owned();
            Some(FileDropZoneFile {
                name,
                path: path.clone(),
                bytes,
                mime: guess_mime(path).to_owned(),
            })
        })
        .collect()
}

/// Partitions paths with the shared accept / size / count rules.
#[must_use]
pub fn partition_paths(
    paths: Vec<PathBuf>,
    config: &FileDropZoneConfig,
) -> (Vec<PathBuf>, Vec<(PathBuf, FileRejectedReason)>) {
    let base = config.file_count.unwrap_or(0);
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();

    for (index, path) in paths.into_iter().enumerate() {
        let file_number = base + index + 1;
        if let Some(reason) = validate_path(&path, file_number, config) {
            rejected.push((path, reason));
        } else {
            accepted.push(path);
        }
    }

    (accepted, rejected)
}

/// Opens a native multi-file picker when the `rfd` feature is enabled.
#[cfg(feature = "rfd")]
#[must_use]
pub fn pick_files() -> Vec<PathBuf> {
    rfd::FileDialog::new().pick_files().unwrap_or_default()
}

fn validate_path(
    path: &Path,
    file_number: usize,
    config: &FileDropZoneConfig,
) -> Option<FileRejectedReason> {
    should_accept_file(&FileCandidate::from_path(path), file_number, config)
}

fn claim_or_match_drop_target(instance_id: u64, prefer_target: bool) -> bool {
    ACTIVE_DROP_TARGET.with(|target| {
        let mut active = target.borrow_mut();
        match *active {
            Some(current) if current == instance_id => true,
            Some(_) if prefer_target => {
                *active = Some(instance_id);
                true
            }
            Some(_) => false,
            None if prefer_target => {
                *active = Some(instance_id);
                true
            }
            None => false,
        }
    })
}

fn is_drop_target(instance_id: u64) -> bool {
    ACTIVE_DROP_TARGET.with(|target| {
        target
            .borrow()
            .is_some_and(|current| current == instance_id)
    })
}

fn clear_drop_target(instance_id: u64) {
    ACTIVE_DROP_TARGET.with(|target| {
        let mut active = target.borrow_mut();
        if active.is_some_and(|current| current == instance_id) {
            *active = None;
        }
    });
}

#[derive(Clone, Copy, Debug)]
struct DashedBorder {
    color: Color,
    width: f32,
    radius: f32,
}

impl<Message> canvas::Program<Message> for DashedBorder {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &crate::iced_compat::Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let width = if self.width.is_finite() {
            self.width.max(0.0)
        } else {
            0.0
        };
        if width <= 0.0 || bounds.width <= width || bounds.height <= width {
            return Vec::new();
        }

        let inset = width / 2.0;
        let size = Size::new(bounds.width - width, bounds.height - width);
        let max_radius = (size.width.min(size.height) / 2.0).max(0.0);
        let radius = self.radius.min(max_radius).max(0.0);
        let path = CanvasPath::rounded_rectangle(
            Point::new(inset, inset),
            size,
            crate::iced_compat::border::radius(radius),
        );

        let mut stroke = Stroke::default()
            .with_color(self.color)
            .with_width(width)
            .with_line_join(canvas::LineJoin::Round);
        stroke.line_dash = canvas::LineDash {
            segments: &[6.0, 4.0],
            offset: 0,
        };

        let mut frame = Frame::new(renderer, bounds.size());
        frame.stroke(&path, stroke);
        vec![frame.into_geometry()]
    }
}

#[derive(Clone, Copy, Debug)]
struct UploadIcon {
    color: Color,
    ring: Color,
    circle_px: f32,
    icon_px: f32,
    stroke_viewbox: f32,
    viewbox: f32,
}

impl<Message> canvas::Program<Message> for UploadIcon {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &crate::iced_compat::Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let cx = bounds.width / 2.0;
        let cy = bounds.height / 2.0;
        let ring_r = (self.circle_px / 2.0).min(cx).min(cy) - 0.5;

        let mut ring_stroke = Stroke::default()
            .with_color(self.ring)
            .with_width(1.0)
            .with_line_join(canvas::LineJoin::Round);
        ring_stroke.line_dash = canvas::LineDash {
            segments: &[4.0, 3.0],
            offset: 0,
        };
        frame.stroke(&CanvasPath::circle(Point::new(cx, cy), ring_r), ring_stroke);

        // Lucide `upload` (24×24): tray + arrow up.
        let s = self.icon_px / self.viewbox;
        let origin_x = cx - self.icon_px / 2.0;
        let origin_y = cy - self.icon_px / 2.0;
        let pt = |gx: f32, gy: f32| Point::new(origin_x + gx * s, origin_y + gy * s);
        let stroke = Stroke::default()
            .with_color(self.color)
            .with_width(self.stroke_viewbox * s)
            .with_line_cap(canvas::LineCap::Round)
            .with_line_join(canvas::LineJoin::Round);

        // M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4
        let tray = CanvasPath::new(|builder| {
            builder.move_to(pt(21.0, 15.0));
            builder.line_to(pt(21.0, 19.0));
            builder.quadratic_curve_to(pt(21.0, 21.0), pt(19.0, 21.0));
            builder.line_to(pt(5.0, 21.0));
            builder.quadratic_curve_to(pt(3.0, 21.0), pt(3.0, 19.0));
            builder.line_to(pt(3.0, 15.0));
        });
        frame.stroke(&tray, stroke);

        // M17 8l-5-5-5 5
        let arrow = CanvasPath::new(|builder| {
            builder.move_to(pt(17.0, 8.0));
            builder.line_to(pt(12.0, 3.0));
            builder.line_to(pt(7.0, 8.0));
        });
        frame.stroke(&arrow, stroke);

        // M12 3v12
        frame.stroke(&CanvasPath::line(pt(12.0, 3.0), pt(12.0, 15.0)), stroke);

        vec![frame.into_geometry()]
    }
}

/// Convenience re-export used by the public module.
#[allow(dead_code)]
pub(super) type ResolvedStyle = FileDropZoneStyle;
