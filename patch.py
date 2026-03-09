import sys

with open("H:/Nova-Code/tp/shadcn-rs/crates/iced-shadcn/src/tree_viewer/widget.rs", "r", encoding="utf-8") as f:
    code = f.read()

# Replace Default Props
code = code.replace("""    fn default() -> Self {
        Self {
            row_height: 32.0,
            indent: 16.0,
            icon_size: 16.0,
            text_size: 14.0,
            content_offset: 8.0,
        }
    }""", """    fn default() -> Self {
        Self {
            row_height: 28.0,
            indent: 16.0,
            icon_size: 16.0,
            text_size: 13.0,
            content_offset: 0.0,
        }
    }""")

# Replace draw and update
draw_start = code.find("    fn draw(")
mouse_interact_start = code.find("    fn mouse_interaction(")

new_methods = """    fn draw(
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

            // Draw vertical guides for ancestor levels
            for d in 0..node.depth {
                let ancestor_left_pad = self.props.content_offset + d as f32 * self.props.indent;
                let guide_x = bounds.x + 4.0 + ancestor_left_pad + self.props.icon_size * 0.5;
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

            let clickable_bounds = Rectangle {
                x: bounds.x + 4.0,
                y: row_bounds.y,
                width: (bounds.width - 8.0).max(0.0),
                height: row_height,
            };

            let is_selected = self.state.is_selected(&node.path);
            let is_hovered = cursor.position_over(clickable_bounds).is_some();

            // Background
            let bg_color = if is_selected {
                Some(self.theme.palette.accent)
            } else if is_hovered {
                Some(self.theme.palette.accent)
            } else {
                None
            };

            if let Some(bg) = bg_color {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: clickable_bounds,
                        border: Border {
                            radius: self.theme.radius.sm.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    bg,
                );
            }

            // Icons and Text
            let base_pad = self.props.content_offset + node.depth as f32 * self.props.indent;
            let left_pad = if node.is_folder { base_pad } else { base_pad + 3.0 };
            
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

            let icon_color = if is_selected {
                self.theme.palette.accent_foreground
            } else {
                self.theme.palette.muted_foreground
            };

            let text_color = if is_selected {
                self.theme.palette.accent_foreground
            } else {
                self.theme.palette.foreground
            };

            // Draw Icon
            renderer.fill_text(
                iced::advanced::text::Text {
                    content: char::from(icon).to_string(),
                    bounds: Size::new(self.props.icon_size * 2.0, row_height),
                    size: iced::Pixels(self.props.icon_size),
                    line_height: iced::advanced::text::LineHeight::default(),
                    font: Font::with_name("lucide"),
                    align_x: iced::advanced::text::Alignment::Center,
                    align_y: iced::alignment::Vertical::Center,
                    shaping: iced::advanced::text::Shaping::Basic,
                    wrapping: iced::advanced::text::Wrapping::default(),
                },
                Point::new(icon_center_x, row_bounds.y + row_height / 2.0),
                icon_color,
                *viewport,
            );

            // Draw Text
            renderer.fill_text(
                iced::advanced::text::Text {
                    content: node.name.clone(),
                    bounds: Size::new(
                        (clickable_bounds.width - (text_x - clickable_bounds.x)).max(0.0),
                        row_height,
                    ),
                    size: iced::Pixels(self.props.text_size),
                    line_height: iced::advanced::text::LineHeight::default(),
                    font: Font::DEFAULT,
                    align_x: iced::advanced::text::Alignment::Left,
                    align_y: iced::alignment::Vertical::Center,
                    shaping: iced::advanced::text::Shaping::Basic,
                    wrapping: iced::advanced::text::Wrapping::default(),
                },
                Point::new(text_x, row_bounds.y + row_height / 2.0),
                text_color,
                *viewport,
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
        _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event
            && let Some(cursor_pos) = cursor.position_over(layout.bounds())
        {
            let bounds = layout.bounds();
            let relative_y = cursor_pos.y - bounds.y;
            let index = (relative_y / self.props.row_height).floor() as usize;

            let y_offset = index as f32 * self.props.row_height;
            let clickable_bounds = Rectangle {
                x: bounds.x + 4.0,
                y: bounds.y + y_offset,
                width: (bounds.width - 8.0).max(0.0),
                height: self.props.row_height,
            };

            if clickable_bounds.contains(cursor_pos) {
                if let Some(node) = self.state.nodes.get(index) {
                    if node.is_folder {
                        if node.folder_state == FolderState::Unloaded {
                            _shell.publish((self.on_load)(node.path.clone()));
                        } else {
                            _shell.publish((self.on_toggle)(node.path.clone()));
                        }
                    } else {
                        _shell.publish((self.on_select)(node.path.clone()));
                    }
                }
            }
        }
    }

"""

code = code[:draw_start] + new_methods + code[mouse_interact_start:]

with open("H:/Nova-Code/tp/shadcn-rs/crates/iced-shadcn/src/tree_viewer/widget.rs", "w", encoding="utf-8") as f:
    f.write(code)

print("Patch applied successfully.")
