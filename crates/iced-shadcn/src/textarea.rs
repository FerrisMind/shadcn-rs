use iced::Background;
use iced::advanced::text::Wrapping;
use iced::border::Border;
use iced::widget::text_editor;

use crate::button::ButtonRadius;
use crate::theme::Theme;
use crate::tokens::{
    AccentColor, accent_color, accent_soft, accent_text, ensure_contrast, is_dark,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextareaSize {
    Size1,
    Size2,
    Size3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextareaVariant {
    Classic,
    Surface,
    Soft,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextareaResize {
    None,
    Vertical,
    Horizontal,
    Both,
}

#[derive(Clone, Debug)]
pub struct TextareaProps {
    pub id: Option<iced::widget::Id>,
    pub size: TextareaSize,
    pub variant: TextareaVariant,
    pub resize: TextareaResize,
    pub wrapping: Wrapping,
    pub padding: Option<[f32; 2]>,
    pub color: AccentColor,
    pub radius: Option<ButtonRadius>,
    pub text_color: Option<iced::Color>,
    pub placeholder_color: Option<iced::Color>,
    pub read_only: bool,
    pub max_len: Option<usize>,
    pub rows: Option<usize>,
    pub max_rows: Option<usize>,
    pub invalid: bool,
    pub disabled: bool,
    pub borderless: bool,
}

impl Default for TextareaProps {
    fn default() -> Self {
        Self {
            id: None,
            size: TextareaSize::Size2,
            variant: TextareaVariant::Surface,
            resize: TextareaResize::None,
            wrapping: Wrapping::WordOrGlyph,
            padding: None,
            color: AccentColor::Gray,
            radius: None,
            text_color: None,
            placeholder_color: None,
            read_only: false,
            max_len: None,
            rows: None,
            max_rows: None,
            invalid: false,
            disabled: false,
            borderless: false,
        }
    }
}

impl TextareaProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: TextareaSize) -> Self {
        self.size = size;
        self
    }

    pub fn id(mut self, id: impl Into<iced::widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn variant(mut self, variant: TextareaVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn resize(mut self, resize: TextareaResize) -> Self {
        self.resize = resize;
        self
    }

    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    pub fn padding(mut self, padding: [f32; 2]) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = color;
        self
    }

    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn text_color(mut self, color: iced::Color) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn placeholder_color(mut self, color: iced::Color) -> Self {
        self.placeholder_color = Some(color);
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn max_len(mut self, max_len: usize) -> Self {
        self.max_len = Some(max_len);
        self
    }

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = Some(rows);
        self
    }

    pub fn max_rows(mut self, max_rows: usize) -> Self {
        self.max_rows = Some(max_rows.max(1));
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }
}

impl TextareaSize {
    fn padding(self) -> [f32; 2] {
        match self {
            TextareaSize::Size1 => [6.0, 10.0],
            TextareaSize::Size2 => [8.0, 12.0],
            TextareaSize::Size3 => [10.0, 14.0],
        }
    }

    fn text_size(self) -> u32 {
        match self {
            TextareaSize::Size1 => 14,
            TextareaSize::Size2 => 14,
            TextareaSize::Size3 => 16,
        }
    }

    fn min_height(self) -> f32 {
        match self {
            TextareaSize::Size1 => 64.0,
            TextareaSize::Size2 => 96.0,
            TextareaSize::Size3 => 128.0,
        }
    }
}

fn textarea_radius(theme: &Theme, props: &TextareaProps) -> f32 {
    match props.radius {
        Some(ButtonRadius::None) => 0.0,
        Some(ButtonRadius::Small) => theme.radius.sm,
        Some(ButtonRadius::Medium) => theme.radius.md,
        Some(ButtonRadius::Large) => theme.radius.lg,
        Some(ButtonRadius::Full) => 9999.0,
        None => theme.radius.sm,
    }
}

pub fn textarea<'a, Message: Clone + 'a, F>(
    content: &'a text_editor::Content,
    placeholder: &'a str,
    on_action: Option<F>,
    props: TextareaProps,
    theme: &Theme,
) -> text_editor::TextEditor<'a, iced::advanced::text::highlighter::PlainText, Message>
where
    F: Fn(text_editor::Action) -> Message + 'a,
{
    let theme = theme.clone();
    let padding = props.padding.unwrap_or_else(|| props.size.padding());
    let text_size = props.size.text_size();
    let min_height = textarea_min_height(&props, text_size, padding);
    let max_height = textarea_max_height(&props, text_size, padding);
    let style_props = props.clone();
    let mut widget = text_editor::TextEditor::new(content)
        .placeholder(placeholder)
        .padding(padding)
        .size(text_size)
        .min_height(min_height)
        .wrapping(props.wrapping)
        .style(move |_iced_theme, status| textarea_style(&theme, &style_props, status));

    if let Some(id) = props.id.clone() {
        widget = widget.id(id);
    }

    if let Some(max_height) = max_height {
        widget = widget.max_height(max_height);
    }

    if props.resize == TextareaResize::None {
        widget = widget.height(iced::Length::Fixed(min_height));
    }

    if !props.disabled
        && let Some(on_action) = on_action
    {
        widget = widget.on_action(on_action);
    }

    widget
}

fn textarea_style(
    theme: &Theme,
    props: &TextareaProps,
    status: text_editor::Status,
) -> text_editor::Style {
    let palette = theme.palette;
    let radius = textarea_radius(theme, props);
    let accent = accent_color(&palette, props.color);
    let text_color = accent_text(&palette, props.color);
    let soft_bg = accent_soft(&palette, props.color);

    let mut border = Border {
        radius: radius.into(),
        width: 1.0,
        color: palette.input,
    };
    let base_bg = if is_dark(&palette) {
        Background::Color(palette.input)
    } else {
        Background::Color(iced::Color::TRANSPARENT)
    };
    let mut background = match props.variant {
        TextareaVariant::Classic | TextareaVariant::Surface => base_bg,
        TextareaVariant::Soft => Background::Color(soft_bg),
    };
    let mut value = match props.variant {
        TextareaVariant::Soft => text_color,
        _ => palette.foreground,
    };
    let mut placeholder = match props.variant {
        TextareaVariant::Soft => text_color,
        _ => palette.muted_foreground,
    };
    let mut selection = accent;
    let value_overridden = props.text_color.is_some();
    let placeholder_overridden = props.placeholder_color.is_some();
    if let Some(color) = props.text_color {
        value = color;
    }
    if let Some(color) = props.placeholder_color {
        placeholder = color;
    }

    match status {
        text_editor::Status::Hovered => {
            border.color = if props.invalid {
                palette.destructive
            } else {
                palette.ring
            };
        }
        text_editor::Status::Focused { .. } => {
            border.color = if props.invalid {
                palette.destructive
            } else {
                palette.ring
            };
            border.width = 1.5;
        }
        text_editor::Status::Disabled => {
            background = Background::Color(palette.muted);
            value = palette.muted_foreground;
            placeholder = palette.muted_foreground;
            selection = palette.muted;
        }
        text_editor::Status::Active => {}
    }

    if props.read_only && !matches!(status, text_editor::Status::Disabled) {
        background = Background::Color(palette.muted);
        value = palette.muted_foreground;
        placeholder = palette.muted_foreground;
        border.color = palette.border;
        selection = palette.muted;
    }

    if props.invalid && matches!(status, text_editor::Status::Active) {
        border.color = palette.destructive;
    }

    if props.borderless {
        border.width = 0.0;
        border.color = iced::Color::TRANSPARENT;
        if !matches!(status, text_editor::Status::Disabled) {
            background = Background::Color(iced::Color::TRANSPARENT);
        }
    }

    let is_disabled = matches!(status, text_editor::Status::Disabled) || props.read_only;
    if !is_disabled {
        if !value_overridden {
            let fallback_bg = palette.background;
            value = ensure_contrast(background, fallback_bg, value);
        }
        if !placeholder_overridden {
            let fallback_bg = palette.background;
            placeholder = ensure_contrast(background, fallback_bg, placeholder);
        }
    }

    text_editor::Style {
        background,
        border,
        placeholder,
        value,
        selection,
    }
}

pub fn textarea_apply_action(
    content: &mut text_editor::Content,
    action: text_editor::Action,
    props: TextareaProps,
) -> bool {
    if props.disabled {
        return false;
    }

    if props.read_only && action.is_edit() {
        return false;
    }

    if let Some(max_len) = props.max_len
        && !can_apply_edit(content, &action, max_len)
    {
        return false;
    }

    content.perform(action);
    true
}

fn textarea_min_height(props: &TextareaProps, text_size: u32, padding: [f32; 2]) -> f32 {
    if let Some(rows) = props.rows {
        let rows = rows.max(1) as f32;
        let line_height = text_size as f32 * 1.4;
        return line_height * rows + padding[0] * 2.0;
    }

    props.size.min_height()
}

fn textarea_max_height(props: &TextareaProps, text_size: u32, padding: [f32; 2]) -> Option<f32> {
    let max_rows = props.max_rows?;
    let line_height = text_size as f32 * 1.4;
    let rows = max_rows.max(1) as f32;
    Some(line_height * rows + padding[0] * 2.0)
}

fn can_apply_edit(
    content: &text_editor::Content,
    action: &text_editor::Action,
    max_len: usize,
) -> bool {
    let edit = match action {
        text_editor::Action::Edit(edit) => edit,
        _ => return true,
    };

    let current_len = content.text().chars().count();
    let selection_len = selection_len(content);
    let insert_len = match edit {
        text_editor::Edit::Insert(_) => 1,
        text_editor::Edit::Paste(text) => text.chars().count(),
        text_editor::Edit::Enter => content
            .line_ending()
            .unwrap_or_default()
            .as_str()
            .chars()
            .count(),
        text_editor::Edit::Indent | text_editor::Edit::Unindent => 0,
        text_editor::Edit::Backspace | text_editor::Edit::Delete => 0,
    };

    if insert_len == 0 {
        return true;
    }

    current_len.saturating_sub(selection_len) + insert_len <= max_len
}

fn selection_len(content: &text_editor::Content) -> usize {
    let cursor = content.cursor();
    let selection = match cursor.selection {
        Some(selection) => selection,
        None => return 0,
    };

    let start = position_to_index(content, cursor.position);
    let end = position_to_index(content, selection);
    start.abs_diff(end)
}

fn position_to_index(content: &text_editor::Content, position: text_editor::Position) -> usize {
    let mut index = 0usize;
    for (line_index, line) in content.lines().enumerate() {
        if line_index == position.line {
            let column = position.column.min(line.text.chars().count());
            index += line.text.chars().take(column).count();
            return index;
        }
        index += line.text.chars().count();
        index += line.ending.as_str().chars().count();
    }
    index
}
