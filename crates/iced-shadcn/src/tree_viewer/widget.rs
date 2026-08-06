use crate::theme::Theme;
use crate::tree_viewer::state::{FolderState, TreeViewerState};
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::event::Event;
use iced::mouse::{self, Cursor};
use iced::{Border, Color, Element, Font, Length, Point, Rectangle, Size};
use lucide_icons::Icon as LucideIcon;

pub struct TreeViewer<'a, Message> {
    state: &'a TreeViewerState,
    context_path: Option<String>,
    handlers: TreeViewerHandlers<'a, Message>,
    props: TreeViewerProps,
    theme: &'a Theme,
}

pub struct TreeViewerHandlers<'a, Message> {
    pub on_toggle: Box<dyn Fn(String) -> Message + 'a>,
    pub on_select: Box<dyn Fn(String) -> Message + 'a>,
    pub on_load: Box<dyn Fn(String) -> Message + 'a>,
    pub on_hover: Box<dyn Fn(Option<String>) -> Message + 'a>,
    pub on_context: Box<dyn Fn(String) -> Message + 'a>,
}

impl<'a, Message> TreeViewerHandlers<'a, Message> {
    pub fn new(
        on_toggle: impl Fn(String) -> Message + 'a,
        on_select: impl Fn(String) -> Message + 'a,
        on_load: impl Fn(String) -> Message + 'a,
        on_hover: impl Fn(Option<String>) -> Message + 'a,
        on_context: impl Fn(String) -> Message + 'a,
    ) -> Self {
        Self {
            on_toggle: Box::new(on_toggle),
            on_select: Box::new(on_select),
            on_load: Box::new(on_load),
            on_hover: Box::new(on_hover),
            on_context: Box::new(on_context),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TreeViewerProps {
    pub row_height: f32,
    pub indent: f32,
    pub icon_size: f32,
    pub text_size: f32,
    pub content_offset: f32,
    pub max_label_chars: usize,
}

impl Default for TreeViewerProps {
    fn default() -> Self {
        Self {
            row_height: 28.0,
            indent: 16.0,
            icon_size: 16.0,
            text_size: 13.0,
            content_offset: 8.0,
            max_label_chars: 44,
        }
    }
}

fn truncate_ellipsis(value: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }
    if max_chars <= 3 {
        return ".".repeat(max_chars);
    }
    let char_count = value.chars().count();
    if char_count <= max_chars {
        return value.to_string();
    }

    let truncated: String = value.chars().take(max_chars - 3).collect();
    format!("{truncated}...")
}

fn single_line_text(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            '\r' | '\n' | '\t' => ' ',
            _ => ch,
        })
        .collect()
}

fn max_chars_for_width(width: f32, text_size: f32) -> usize {
    if width <= 0.0 || text_size <= 0.0 {
        return 1;
    }
    // Conservative estimate to guarantee one-line fit even with narrow glyphs.
    let avg_glyph_width = text_size * 0.56;
    let estimated = (width / avg_glyph_width).floor() as usize;
    estimated.max(1)
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}

fn active_row_hover_bg(theme: &Theme) -> Color {
    theme.palette.accent
}

fn inactive_row_hover_bg(theme: &Theme) -> Color {
    apply_opacity(active_row_hover_bg(theme), 0.70)
}

impl<'a, Message> TreeViewer<'a, Message> {
    pub fn new(
        state: &'a TreeViewerState,
        context_path: Option<String>,
        handlers: TreeViewerHandlers<'a, Message>,
        props: TreeViewerProps,
        theme: &'a Theme,
    ) -> Self {
        Self {
            state,
            context_path,
            handlers,
            props,
            theme,
        }
    }

    fn total_height(&self) -> f32 {
        // Keep one extra row-height of free space after the last item.
        (self.state.nodes.len() as f32 + 1.0) * self.props.row_height
    }

    fn row_index_at(&self, bounds: Rectangle, cursor_pos: Point) -> Option<usize> {
        if !bounds.contains(cursor_pos) {
            return None;
        }

        let relative_y = cursor_pos.y - bounds.y;
        let index = (relative_y / self.props.row_height).floor() as usize;
        (index < self.state.nodes.len()).then_some(index)
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for TreeViewer<'a, Message>
where
    Message: Clone,
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = Font>,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fixed(self.total_height()),
        }
    }

    fn layout(
        &mut self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = limits.resolve(Length::Fill, Length::Fixed(self.total_height()), Size::ZERO);

        layout::Node::new(size)
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let row_height = self.props.row_height;

        let relative_viewport = Rectangle {
            x: viewport.x - bounds.x,
            y: viewport.y - bounds.y,
            width: viewport.width,
            height: viewport.height,
        };

        let first_visible = if relative_viewport.y > 0.0 {
            (relative_viewport.y / row_height).floor() as usize
        } else {
            0
        };

        let last_visible = if relative_viewport.y + relative_viewport.height > 0.0 {
            ((relative_viewport.y + relative_viewport.height) / row_height).ceil() as usize
        } else {
            0
        };

        let last_index = last_visible.min(self.state.nodes.len());

        for i in first_visible..last_index {
            let node = &self.state.nodes[i];
            let y_offset = i as f32 * row_height;
            let row_bounds = Rectangle {
                x: bounds.x,
                y: bounds.y + y_offset,
                width: bounds.width,
                height: row_height,
            };

            if !row_bounds.intersects(viewport) {
                continue;
            }

            let clip_x1 = row_bounds.x.max(viewport.x);
            let clip_y1 = row_bounds.y.max(viewport.y);
            let clip_x2 = (row_bounds.x + row_bounds.width).min(viewport.x + viewport.width);
            let clip_y2 = (row_bounds.y + row_bounds.height).min(viewport.y + viewport.height);
            if clip_x2 <= clip_x1 || clip_y2 <= clip_y1 {
                continue;
            }
            let row_clip = Rectangle {
                x: clip_x1,
                y: clip_y1,
                width: clip_x2 - clip_x1,
                height: clip_y2 - clip_y1,
            };

            let clickable_bounds = Rectangle {
                x: bounds.x,
                y: row_bounds.y,
                width: bounds.width.max(0.0),
                height: row_height,
            };
            let hover_inset_x = self.theme.spacing.xs;
            let highlight_bounds = Rectangle {
                x: clickable_bounds.x + hover_inset_x,
                y: clickable_bounds.y,
                width: (clickable_bounds.width - hover_inset_x * 2.0).max(0.0),
                height: clickable_bounds.height,
            };

            let is_selected = self.state.is_selected(&node.path);
            let has_context = self.context_path.is_some();
            let is_context_active = self.context_path.as_deref() == Some(node.path.as_str());
            let is_hovered = !has_context && cursor.position_over(clickable_bounds).is_some();

            // Background
            let bg_color = if is_selected && is_hovered {
                Some(active_row_hover_bg(self.theme))
            } else if is_selected {
                Some(self.theme.palette.accent)
            } else if is_hovered {
                Some(inactive_row_hover_bg(self.theme))
            } else {
                None
            };

            if let Some(bg) = bg_color {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: highlight_bounds,
                        border: Border {
                            radius: self.theme.radius.md.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    bg,
                );
            }

            if is_context_active {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: highlight_bounds,
                        border: Border {
                            color: self.theme.palette.border,
                            width: 1.0,
                            radius: self.theme.radius.md.into(),
                        },
                        ..Default::default()
                    },
                    iced::Color::TRANSPARENT,
                );
            }

            // Draw vertical guides for ancestor levels (should be on top of background)
            for d in 0..node.depth {
                let ancestor_left_pad = self.props.content_offset + d as f32 * self.props.indent;
                let guide_x = bounds.x + ancestor_left_pad + self.props.icon_size * 0.5;
                let line_bounds = Rectangle {
                    x: guide_x.floor(),
                    y: row_bounds.y,
                    width: 1.0,
                    height: row_height,
                };
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: line_bounds,
                        border: Border::default(),
                        ..Default::default()
                    },
                    self.theme.palette.border,
                );
            }

            // Icons and Text
            let base_pad = self.props.content_offset + node.depth as f32 * self.props.indent;
            let left_pad = base_pad;

            let icon_x = clickable_bounds.x + left_pad;
            let icon_center_x = icon_x + self.props.icon_size / 2.0;

            let text_x = icon_x + self.props.icon_size + 6.0;

            // Render Icon
            let icon = if node.is_folder {
                if node.folder_state == FolderState::Loading {
                    LucideIcon::Loader
                } else if node.is_expanded {
                    node.icon_open.unwrap_or(LucideIcon::FolderOpen)
                } else {
                    node.icon_closed.unwrap_or(LucideIcon::Folder)
                }
            } else {
                node.icon_file.unwrap_or(LucideIcon::File)
            };

            let default_icon_color = if is_selected {
                self.theme.palette.accent_foreground
            } else {
                self.theme.palette.muted_foreground
            };
            let icon_color = if is_selected {
                default_icon_color
            } else {
                node.icon_color.unwrap_or(default_icon_color)
            };

            let text_color = if is_selected {
                self.theme.palette.accent_foreground
            } else {
                self.theme.palette.foreground
            };

            // Draw Icon
            let icon_glyph = node.icon_glyph.unwrap_or(char::from(icon)).to_string();
            let icon_font = node
                .icon_font_family
                .map(Font::with_name)
                .unwrap_or(Font::with_name("lucide"));
            renderer.fill_text(
                iced::advanced::text::Text {
                    content: icon_glyph,
                    bounds: Size::new(self.props.icon_size * 2.0, row_height),
                    size: iced::Pixels(self.props.icon_size),
                    line_height: iced::advanced::text::LineHeight::default(),
                    font: icon_font,
                    align_x: iced::advanced::text::Alignment::Center,
                    align_y: iced::alignment::Vertical::Center,
                    shaping: iced::advanced::text::Shaping::Basic,
                    wrapping: iced::advanced::text::Wrapping::default(),
                },
                Point::new(icon_center_x, row_bounds.y + row_height / 2.0),
                icon_color,
                row_clip,
            );

            // Draw Text
            let text_width = (clickable_bounds.width - (text_x - clickable_bounds.x)).max(0.0);
            let width_limited_max = max_chars_for_width(text_width, self.props.text_size)
                .min(self.props.max_label_chars);
            let display_name = truncate_ellipsis(&single_line_text(&node.name), width_limited_max);
            renderer.fill_text(
                iced::advanced::text::Text {
                    content: display_name,
                    bounds: Size::new(text_width, self.props.text_size.max(1.0)),
                    size: iced::Pixels(self.props.text_size),
                    line_height: iced::advanced::text::LineHeight::Absolute(
                        self.props.text_size.into(),
                    ),
                    font: Font::DEFAULT,
                    align_x: iced::advanced::text::Alignment::Left,
                    align_y: iced::alignment::Vertical::Center,
                    shaping: iced::advanced::text::Shaping::Basic,
                    wrapping: iced::advanced::text::Wrapping::None,
                },
                Point::new(text_x, row_bounds.y + row_height / 2.0),
                text_color,
                row_clip,
            );
        }
    }

    fn update(
        &mut self,
        _tree: &mut widget::Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                // To trigger hover redraws for our custom rendered tree
                if cursor.is_over(layout.bounds()) {
                    shell.request_redraw();
                }
                let bounds = layout.bounds();
                let hovered = cursor.position_over(bounds).and_then(|cursor_pos| {
                    self.row_index_at(bounds, cursor_pos)
                        .and_then(|index| self.state.nodes.get(index).map(|node| node.path.clone()))
                });
                shell.publish((self.handlers.on_hover)(hovered));
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let bounds = layout.bounds();
                if let Some(cursor_pos) = cursor.position_over(bounds) {
                    let Some(index) = self.row_index_at(bounds, cursor_pos) else {
                        return;
                    };
                    let bounds = layout.bounds();
                    let y_offset = index as f32 * self.props.row_height;
                    let clickable_bounds = Rectangle {
                        x: bounds.x,
                        y: bounds.y + y_offset,
                        width: bounds.width.max(0.0),
                        height: self.props.row_height,
                    };

                    if clickable_bounds.contains(cursor_pos)
                        && let Some(node) = self.state.nodes.get(index)
                    {
                        if node.is_folder {
                            if node.folder_state == FolderState::Unloaded {
                                shell.publish((self.handlers.on_load)(node.path.clone()));
                            } else {
                                shell.publish((self.handlers.on_toggle)(node.path.clone()));
                            }
                        } else {
                            shell.publish((self.handlers.on_select)(node.path.clone()));
                        }
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => {
                let bounds = layout.bounds();
                if let Some(cursor_pos) = cursor.position_over(bounds) {
                    let Some(index) = self.row_index_at(bounds, cursor_pos) else {
                        return;
                    };
                    if let Some(node) = self.state.nodes.get(index) {
                        shell.publish((self.handlers.on_context)(node.path.clone()));
                    }
                }
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        _state: &widget::Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        let interactive = cursor
            .position_over(bounds)
            .and_then(|pos| self.row_index_at(bounds, pos))
            .is_some();

        if interactive {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message, Theme, Renderer> From<TreeViewer<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = Font> + 'a,
{
    fn from(tree_viewer: TreeViewer<'a, Message>) -> Self {
        Self::new(tree_viewer)
    }
}

#[cfg(test)]
mod tests {
    use super::{active_row_hover_bg, inactive_row_hover_bg};
    use crate::theme::Theme;

    #[test]
    fn inactive_hover_is_thirty_percent_dimmer_than_active_hover() {
        let theme = Theme::default();
        let active = active_row_hover_bg(&theme);
        let inactive = inactive_row_hover_bg(&theme);

        assert_eq!(inactive.r, active.r);
        assert_eq!(inactive.g, active.g);
        assert_eq!(inactive.b, active.b);
        assert!((inactive.a - active.a * 0.70).abs() < f32::EPSILON);
    }
}
