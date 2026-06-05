use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use iced::advanced::Renderer as _;
use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::Operation;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::border::Border;
use iced::widget::{column, container, text};
use iced::{Background, Color, Element, Event, Length, Rectangle, Shadow, Task, mouse, window};

use crate::theme::Theme;

pub const BYTE: u64 = 1;
pub const KILOBYTE: u64 = 1000;
pub const MEGABYTE: u64 = 1000 * KILOBYTE;
pub const GIGABYTE: u64 = 1000 * MEGABYTE;

pub const ACCEPT_IMAGE: &str = "image/*";
pub const ACCEPT_VIDEO: &str = "video/*";
pub const ACCEPT_AUDIO: &str = "audio/*";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FileDropZoneSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl FileDropZoneSize {
    fn min_height(self) -> f32 {
        match self {
            FileDropZoneSize::Sm => 96.0,
            FileDropZoneSize::Md => 128.0,
            FileDropZoneSize::Lg => 168.0,
        }
    }

    fn padding(self) -> f32 {
        match self {
            FileDropZoneSize::Sm => 12.0,
            FileDropZoneSize::Md => 16.0,
            FileDropZoneSize::Lg => 20.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FileDropZoneVariant {
    Ghost,
    #[default]
    Surface,
    Soft,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileDropZoneRejectedReason {
    MaximumFileSizeExceeded,
    FileTypeNotAllowed,
    MaximumFilesUploaded,
}

#[derive(Clone, Debug)]
pub enum FileDropZoneAction {
    PickerRequested,
    DropPaths(Vec<PathBuf>),
    Rejected {
        path: PathBuf,
        reason: FileDropZoneRejectedReason,
    },
    Hovered(bool),
}

#[derive(Clone, Debug)]
pub struct FileDropZoneFile {
    pub name: String,
    pub path: PathBuf,
    pub bytes: Vec<u8>,
    pub mime: String,
}

#[derive(Clone, Debug)]
pub struct FileDropZoneProps {
    pub id: Option<String>,
    pub disabled: bool,
    pub max_files: Option<usize>,
    pub file_count: Option<usize>,
    pub max_file_size: Option<u64>,
    pub accept: Option<String>,
    pub size: FileDropZoneSize,
    pub variant: FileDropZoneVariant,
}

impl Default for FileDropZoneProps {
    fn default() -> Self {
        Self {
            id: None,
            disabled: false,
            max_files: None,
            file_count: None,
            max_file_size: None,
            accept: None,
            size: FileDropZoneSize::Md,
            variant: FileDropZoneVariant::Surface,
        }
    }
}

impl FileDropZoneProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn max_files(mut self, max_files: usize) -> Self {
        self.max_files = Some(max_files);
        self
    }

    pub fn file_count(mut self, file_count: usize) -> Self {
        self.file_count = Some(file_count);
        self
    }

    pub fn max_file_size(mut self, max_file_size: u64) -> Self {
        self.max_file_size = Some(max_file_size);
        self
    }

    pub fn accept(mut self, accept: impl Into<String>) -> Self {
        self.accept = Some(accept.into());
        self
    }

    pub fn size(mut self, size: FileDropZoneSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: FileDropZoneVariant) -> Self {
        self.variant = variant;
        self
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FileDropZoneState {
    pub hovered: bool,
}

impl FileDropZoneState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&mut self, action: &FileDropZoneAction) {
        if let FileDropZoneAction::Hovered(value) = action {
            self.hovered = *value;
        }
    }

    pub fn can_upload(&self, props: &FileDropZoneProps) -> bool {
        if props.disabled {
            return false;
        }

        if let (Some(max), Some(count)) = (props.max_files, props.file_count)
            && count >= max
        {
            return false;
        }

        true
    }
}

pub struct FileDropZoneContext<'a, Message> {
    pub props: FileDropZoneProps,
    pub state: &'a FileDropZoneState,
    pub theme: &'a Theme,
    pub on_action: Rc<dyn Fn(FileDropZoneAction) -> Message + 'a>,
}

impl<'a, Message> FileDropZoneContext<'a, Message> {
    pub fn message(&self, action: FileDropZoneAction) -> Message {
        (self.on_action)(action)
    }
}

pub fn file_drop_zone_root<'a, Message: Clone + 'a>(
    props: FileDropZoneProps,
    state: &'a FileDropZoneState,
    on_action: impl Fn(FileDropZoneAction) -> Message + 'a,
    theme: &'a Theme,
    content: impl FnOnce(FileDropZoneContext<'a, Message>) -> Element<'a, Message>,
) -> Element<'a, Message> {
    let ctx = FileDropZoneContext {
        props,
        state,
        theme,
        on_action: Rc::new(on_action),
    };

    content(ctx)
}

pub fn file_drop_zone_trigger<'a, Message: Clone + 'a>(
    ctx: &FileDropZoneContext<'a, Message>,
    child: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let widget = FileDropZoneWidget::new(
        child.into(),
        ctx.props.clone(),
        ctx.state,
        ctx.theme.clone(),
        Rc::clone(&ctx.on_action),
    );
    Element::new(widget)
}

pub fn file_drop_zone_trigger_default<'a, Message: Clone + 'a>(
    ctx: &FileDropZoneContext<'a, Message>,
) -> Element<'a, Message> {
    let hint = match (ctx.props.max_files, ctx.props.file_count) {
        (Some(max), Some(current)) => format!("Files: {current}/{max}"),
        (Some(max), None) => format!("Max files: {max}"),
        _ => String::from("Drop files here or click to choose"),
    };

    let support = if let Some(accept) = &ctx.props.accept {
        format!("Accepted: {accept}")
    } else {
        String::from("Accepted: any file type")
    };

    let child = column![
        text("File Drop Zone").size(16),
        text(hint).size(13),
        text(support).size(12),
    ]
    .spacing(4);

    file_drop_zone_trigger(ctx, child)
}

pub fn file_drop_zone_textarea<'a, Message: Clone + 'a>(
    ctx: &FileDropZoneContext<'a, Message>,
    child: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let widget = FileDropZoneWidget::new(
        child.into(),
        ctx.props.clone(),
        ctx.state,
        ctx.theme.clone(),
        Rc::clone(&ctx.on_action),
    );
    Element::new(widget)
}

pub fn display_size(bytes: u64) -> String {
    if bytes < KILOBYTE {
        return format!("{bytes} B");
    }

    if bytes < MEGABYTE {
        return format!("{} KB", (bytes as f64 / KILOBYTE as f64).round() as u64);
    }

    if bytes < GIGABYTE {
        return format!("{} MB", (bytes as f64 / MEGABYTE as f64).round() as u64);
    }

    format!("{} GB", (bytes as f64 / GIGABYTE as f64).round() as u64)
}

pub fn file_drop_zone_load_files_task<Message: Send + 'static>(
    paths: Vec<PathBuf>,
    map: impl Fn(Vec<FileDropZoneFile>) -> Message + Send + 'static,
) -> Task<Message> {
    Task::perform(async move { load_files(paths.as_slice()) }, map)
}

pub fn file_drop_zone_partition_paths(
    paths: Vec<PathBuf>,
    props: &FileDropZoneProps,
) -> (Vec<PathBuf>, Vec<(PathBuf, FileDropZoneRejectedReason)>) {
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();
    let base_count = props.file_count.unwrap_or(0);

    for (index, path) in paths.into_iter().enumerate() {
        let file_number = base_count + index + 1;
        if let Some(reason) = validate_path(path.as_path(), file_number, props) {
            rejected.push((path, reason));
        } else {
            accepted.push(path);
        }
    }

    (accepted, rejected)
}

#[cfg(feature = "rfd")]
pub fn file_drop_zone_pick_files_task<Message: Send + 'static>(
    map: impl Fn(Vec<PathBuf>) -> Message + Send + 'static,
) -> Task<Message> {
    Task::perform(
        async move { rfd::FileDialog::new().pick_files().unwrap_or_default() },
        map,
    )
}

fn load_files(paths: &[PathBuf]) -> Vec<FileDropZoneFile> {
    paths
        .iter()
        .filter_map(|path| {
            let bytes = std::fs::read(path).ok()?;
            let name = path.file_name()?.to_string_lossy().to_string();
            Some(FileDropZoneFile {
                name,
                path: path.clone(),
                bytes,
                mime: guess_mime(path),
            })
        })
        .collect()
}

static NEXT_DROP_ZONE_ID: AtomicU64 = AtomicU64::new(1);

thread_local! {
    static ACTIVE_DROP_TARGET: std::cell::RefCell<Option<u64>> = const { std::cell::RefCell::new(None) };
}

#[derive(Debug, Default)]
struct FileDropZoneWidgetState {
    instance_id: u64,
    hovering_files: bool,
    drop_batch_count: usize,
    last_cursor_over: bool,
}

struct FileDropZoneWidget<'a, Message> {
    content: Element<'a, Message>,
    props: FileDropZoneProps,
    state: &'a FileDropZoneState,
    theme: Theme,
    on_action: Rc<dyn Fn(FileDropZoneAction) -> Message + 'a>,
}

impl<'a, Message> FileDropZoneWidget<'a, Message> {
    fn new(
        content: Element<'a, Message>,
        props: FileDropZoneProps,
        state: &'a FileDropZoneState,
        theme: Theme,
        on_action: Rc<dyn Fn(FileDropZoneAction) -> Message + 'a>,
    ) -> Self {
        Self {
            content,
            props,
            state,
            theme,
            on_action,
        }
    }
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer> for FileDropZoneWidget<'_, Message>
where
    Message: Clone,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<FileDropZoneWidgetState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(FileDropZoneWidgetState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.content.as_widget()]);
    }

    fn size(&self) -> iced::Size<Length> {
        iced::Size::new(Length::Fill, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let child_limits = (*limits)
            .width(Length::Fill)
            .min_height(self.props.size.min_height());

        let child =
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &child_limits);

        layout::Node::with_children(child.size(), vec![child])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
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
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
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

        let can_upload = self.state.can_upload(&self.props);
        let bounds = layout.bounds();
        let is_over = cursor.is_over(bounds);
        let local = tree.state.downcast_mut::<FileDropZoneWidgetState>();
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
                        self.props.file_count.unwrap_or(0) + local.drop_batch_count + 1;
                    if let Some(reason) = validate_path(path, file_number, &self.props) {
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
                let was_hovering_files = local.hovering_files;
                local.hovering_files = false;
                local.drop_batch_count = 0;
                local.last_cursor_over = false;
                if was_hovering_files || self.state.hovered {
                    shell.publish((self.on_action)(FileDropZoneAction::Hovered(false)));
                }
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let Some(child_layout) = layout.children().next() else {
            return mouse::Interaction::default();
        };

        if !self.state.can_upload(&self.props) {
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
        renderer: &mut iced::Renderer,
        _theme: &iced::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let style = file_drop_zone_style(&self.theme, &self.props, self.state);
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: style.border,
                shadow: style.shadow,
                ..renderer::Quad::default()
            },
            style
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
        );

        if let Some(child_layout) = layout.children().next() {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                _theme,
                _style,
                child_layout,
                cursor,
                viewport,
            );
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct FileDropZoneStyle {
    background: Option<Background>,
    border: Border,
    shadow: Shadow,
}

fn file_drop_zone_style(
    theme: &Theme,
    props: &FileDropZoneProps,
    state: &FileDropZoneState,
) -> FileDropZoneStyle {
    let palette = theme.palette;
    let hovered = state.hovered;
    let disabled = !state.can_upload(props);

    let (mut background, mut border_color) = match props.variant {
        FileDropZoneVariant::Ghost => (Color::TRANSPARENT, palette.border),
        FileDropZoneVariant::Surface => (palette.card, palette.border),
        FileDropZoneVariant::Soft => (palette.muted, palette.border),
    };

    if hovered {
        border_color = palette.ring;
        if !matches!(props.variant, FileDropZoneVariant::Ghost) {
            background = mix_color(background, palette.accent, 0.35);
        }
    }

    if disabled {
        background = mix_color(background, palette.muted, 0.35);
        border_color = palette.muted_foreground;
    }

    FileDropZoneStyle {
        background: Some(Background::Color(background)),
        border: Border {
            radius: theme.radius.md.into(),
            width: if hovered { 1.5 } else { 1.0 },
            color: border_color,
        },
        shadow: Shadow::default(),
    }
}

fn mix_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
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

fn validate_path(
    path: &Path,
    file_number: usize,
    props: &FileDropZoneProps,
) -> Option<FileDropZoneRejectedReason> {
    if let Some(max_file_size) = props.max_file_size
        && std::fs::metadata(path)
            .ok()
            .map(|metadata| metadata.len() > max_file_size)
            .unwrap_or(false)
    {
        return Some(FileDropZoneRejectedReason::MaximumFileSizeExceeded);
    }

    if let Some(max_files) = props.max_files
        && file_number > max_files
    {
        return Some(FileDropZoneRejectedReason::MaximumFilesUploaded);
    }

    if let Some(accept) = props.accept.as_deref() {
        let accepted = accept
            .split(',')
            .map(|value| value.trim().to_ascii_lowercase())
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>();
        if !accepted.is_empty() {
            let file_name = path
                .file_name()
                .map(|value| value.to_string_lossy().to_ascii_lowercase())
                .unwrap_or_default();
            let mime = guess_mime(path).to_ascii_lowercase();

            let is_allowed = accepted.iter().any(|pattern| {
                if pattern.starts_with('.') {
                    file_name.ends_with(pattern)
                } else if let Some(base) = pattern.strip_suffix("/*") {
                    mime.starts_with(&format!("{base}/"))
                } else {
                    mime == *pattern
                }
            });

            if !is_allowed {
                return Some(FileDropZoneRejectedReason::FileTypeNotAllowed);
            }
        }
    }

    None
}

fn guess_mime(path: &Path) -> String {
    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|value| value.to_ascii_lowercase())
        .as_deref()
    {
        Some("png") | Some("apng") => String::from("image/png"),
        Some("jpg") | Some("jpeg") => String::from("image/jpeg"),
        Some("gif") => String::from("image/gif"),
        Some("webp") => String::from("image/webp"),
        Some("avif") => String::from("image/avif"),
        Some("svg") => String::from("image/svg+xml"),
        Some("bmp") => String::from("image/bmp"),
        Some("txt") => String::from("text/plain"),
        Some("pdf") => String::from("application/pdf"),
        Some("json") => String::from("application/json"),
        Some("mp4") => String::from("video/mp4"),
        Some("mov") => String::from("video/quicktime"),
        Some("mp3") => String::from("audio/mpeg"),
        Some("wav") => String::from("audio/wav"),
        _ => String::from("application/octet-stream"),
    }
}

pub fn file_drop_zone_surface<'a, Message: Clone + 'a>(
    ctx: &FileDropZoneContext<'a, Message>,
    title: impl Into<String>,
    subtitle: impl Into<String>,
) -> Element<'a, Message> {
    let child = container(
        column![text(title.into()).size(16), text(subtitle.into()).size(12),]
            .spacing(6)
            .align_x(iced::Alignment::Center),
    )
    .padding(ctx.props.size.padding())
    .width(Length::Fill)
    .height(Length::Shrink)
    .center_x(Length::Fill)
    .center_y(Length::Shrink);

    file_drop_zone_trigger(ctx, child)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_path(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be valid")
            .as_nanos();
        path.push(format!("iced_shadcn_file_drop_zone_{name}_{nanos}"));
        path
    }

    #[test]
    fn accepts_extension_pattern() {
        let props = FileDropZoneProps::new().accept(".png");
        let path = PathBuf::from("avatar.PNG");
        assert_eq!(validate_path(&path, 1, &props), None);
    }

    #[test]
    fn accepts_wildcard_pattern() {
        let props = FileDropZoneProps::new().accept("image/*");
        let path = PathBuf::from("avatar.jpg");
        assert_eq!(validate_path(&path, 1, &props), None);
    }

    #[test]
    fn accepts_exact_mime_pattern() {
        let props = FileDropZoneProps::new().accept("application/pdf");
        let path = PathBuf::from("file.pdf");
        assert_eq!(validate_path(&path, 1, &props), None);
    }

    #[test]
    fn rejects_unknown_type_by_accept() {
        let props = FileDropZoneProps::new().accept("image/*");
        let path = PathBuf::from("clip.mp4");
        assert_eq!(
            validate_path(&path, 1, &props),
            Some(FileDropZoneRejectedReason::FileTypeNotAllowed)
        );
    }

    #[test]
    fn rejects_when_max_files_exceeded() {
        let props = FileDropZoneProps::new().max_files(2);
        let path = PathBuf::from("file.txt");
        assert_eq!(
            validate_path(&path, 3, &props),
            Some(FileDropZoneRejectedReason::MaximumFilesUploaded)
        );
    }

    #[test]
    fn rejects_when_max_file_size_exceeded() {
        let path = unique_path("size");
        std::fs::write(&path, vec![0_u8; 10]).expect("test file should be writable");

        let props = FileDropZoneProps::new().max_file_size(5);
        assert_eq!(
            validate_path(&path, 1, &props),
            Some(FileDropZoneRejectedReason::MaximumFileSizeExceeded)
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn load_files_collects_name_bytes_and_mime() {
        let path = unique_path("load");
        std::fs::write(&path, b"hello").expect("test file should be writable");

        let loaded = load_files(std::slice::from_ref(&path));
        assert_eq!(loaded.len(), 1);
        assert_eq!(
            loaded[0].name,
            path.file_name().expect("name").to_string_lossy()
        );
        assert_eq!(loaded[0].bytes, b"hello");
        assert_eq!(loaded[0].mime, "application/octet-stream");

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn load_files_skips_unreadable_paths() {
        let missing = unique_path("missing");
        let loaded = load_files(std::slice::from_ref(&missing));
        assert!(loaded.is_empty());
    }
}
