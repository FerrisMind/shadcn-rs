use std::path::PathBuf;

use iced::widget::{checkbox, column, container, scrollable, text};
use iced::{Background, Element, Length, Task};

#[cfg(feature = "rfd")]
use iced_shadcn::file_drop_zone_pick_files_task;
use iced_shadcn::{
    ACCEPT_IMAGE, CardProps, CardSize, CardVariant, FileDropZoneAction, FileDropZoneFile,
    FileDropZoneProps, FileDropZoneRejectedReason, FileDropZoneState, Theme, card, display_size,
    file_drop_zone_load_files_task, file_drop_zone_root, file_drop_zone_surface,
    file_drop_zone_textarea,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .title("File Drop Zone demo")
        .run()
}

struct Example {
    theme: Theme,
    path_state: FileDropZoneState,
    bytes_state: FileDropZoneState,
    disabled: bool,
    enforce_max_files: bool,
    accept_images_only: bool,
    path_only_paths: Vec<PathBuf>,
    loaded_files: Vec<FileDropZoneFile>,
    rejected: Vec<(PathBuf, FileDropZoneRejectedReason)>,
}

#[derive(Debug, Clone)]
enum Message {
    PathZone(FileDropZoneAction),
    BytesZone(FileDropZoneAction),
    BytesLoaded(Vec<FileDropZoneFile>),
    ToggleDisabled(bool),
    ToggleMaxFiles(bool),
    ToggleAcceptImages(bool),
    #[cfg(feature = "rfd")]
    Picked(Vec<PathBuf>),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            path_state: FileDropZoneState::new(),
            bytes_state: FileDropZoneState::new(),
            disabled: false,
            enforce_max_files: true,
            accept_images_only: true,
            path_only_paths: Vec::new(),
            loaded_files: Vec::new(),
            rejected: Vec::new(),
        }
    }
}

impl Example {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PathZone(action) => {
                self.path_state.apply(&action);
                self.handle_path_zone_action(action)
            }
            Message::BytesZone(action) => {
                self.bytes_state.apply(&action);
                self.handle_bytes_zone_action(action)
            }
            Message::BytesLoaded(files) => {
                self.loaded_files = files;
                Task::none()
            }
            Message::ToggleDisabled(value) => {
                self.disabled = value;
                Task::none()
            }
            Message::ToggleMaxFiles(value) => {
                self.enforce_max_files = value;
                Task::none()
            }
            Message::ToggleAcceptImages(value) => {
                self.accept_images_only = value;
                Task::none()
            }
            #[cfg(feature = "rfd")]
            Message::Picked(paths) => file_drop_zone_load_files_task(paths, Message::BytesLoaded),
        }
    }

    fn handle_path_zone_action(&mut self, action: FileDropZoneAction) -> Task<Message> {
        match action {
            FileDropZoneAction::DropPaths(paths) => {
                self.path_only_paths = paths;
                Task::none()
            }
            FileDropZoneAction::Rejected { path, reason } => {
                self.rejected.push((path, reason));
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn handle_bytes_zone_action(&mut self, action: FileDropZoneAction) -> Task<Message> {
        match action {
            FileDropZoneAction::DropPaths(paths) => {
                file_drop_zone_load_files_task(paths, Message::BytesLoaded)
            }
            FileDropZoneAction::Rejected { path, reason } => {
                self.rejected.push((path, reason));
                Task::none()
            }
            #[cfg(feature = "rfd")]
            FileDropZoneAction::PickerRequested => file_drop_zone_pick_files_task(Message::Picked),
            #[cfg(not(feature = "rfd"))]
            FileDropZoneAction::PickerRequested => Task::none(),
            FileDropZoneAction::Hovered(_) => Task::none(),
        }
    }

    fn accept_value(&self) -> &'static str {
        if self.accept_images_only {
            ACCEPT_IMAGE
        } else {
            "image/*,text/plain"
        }
    }

    fn max_files_value(&self) -> usize {
        2
    }

    fn path_props(&self) -> FileDropZoneProps {
        let mut props = FileDropZoneProps::new()
            .disabled(self.disabled)
            .accept(self.accept_value())
            .file_count(self.path_only_paths.len());

        if self.enforce_max_files {
            props = props.max_files(self.max_files_value());
        }

        props
    }

    fn bytes_props(&self) -> FileDropZoneProps {
        let mut props = FileDropZoneProps::new()
            .disabled(self.disabled)
            .accept(self.accept_value())
            .file_count(self.loaded_files.len());

        if self.enforce_max_files {
            props = props.max_files(self.max_files_value());
        }

        props
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let controls = column![
            text("Controls").size(18),
            checkbox(self.disabled)
                .label("disabled")
                .on_toggle(Message::ToggleDisabled),
            checkbox(self.enforce_max_files)
                .label("max-files = 2")
                .on_toggle(Message::ToggleMaxFiles),
            checkbox(self.accept_images_only)
                .label("accept = image/* (off: image/*,text/plain)")
                .on_toggle(Message::ToggleAcceptImages),
        ]
        .spacing(8);

        let path_only = file_drop_zone_root(
            self.path_props(),
            &self.path_state,
            Message::PathZone,
            theme,
            |ctx| {
                file_drop_zone_surface(
                    &ctx,
                    "Path-only flow",
                    "Uses FileDropZoneAction::DropPaths without reading bytes.",
                )
            },
        );

        let bytes_flow = file_drop_zone_root(
            self.bytes_props(),
            &self.bytes_state,
            Message::BytesZone,
            theme,
            |ctx| {
                let mut content = column![
                    text("Bytes flow").size(16),
                    text(
                        "Drop files or click to pick; update() runs file_drop_zone_load_files_task."
                    )
                    .size(12),
                ]
                .spacing(4);

                #[cfg(feature = "rfd")]
                {
                    content =
                        content.push(text("rfd feature enabled: click-to-pick active.").size(12));
                }

                #[cfg(not(feature = "rfd"))]
                {
                    content = content.push(text("rfd feature disabled: drop-only mode.").size(12));
                }

                file_drop_zone_textarea(&ctx, container(content).padding(12))
            },
        );

        let paths_text = self
            .path_only_paths
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join("\n");

        let loaded_text = self
            .loaded_files
            .iter()
            .map(|file| format!("{} ({})", file.name, display_size(file.bytes.len() as u64)))
            .collect::<Vec<_>>()
            .join("\n");

        let rejected_text = self
            .rejected
            .iter()
            .map(|(path, reason)| format!("{} -> {:?}", path.display(), reason))
            .collect::<Vec<_>>()
            .join("\n");

        let summary = column![
            text("DropPaths output:").size(14),
            text(if paths_text.is_empty() {
                String::from("No paths yet")
            } else {
                paths_text
            })
            .size(12),
            text("Loaded files (bytes flow):").size(14),
            text(if loaded_text.is_empty() {
                String::from("No files loaded yet")
            } else {
                loaded_text
            })
            .size(12),
            text("Rejected reasons:").size(14),
            text(if rejected_text.is_empty() {
                String::from("No rejections")
            } else {
                rejected_text
            })
            .size(12),
        ]
        .spacing(6);

        let content = column![
            panel(theme, controls.into()),
            panel(theme, path_only),
            panel(theme, bytes_flow),
            panel(theme, summary.into()),
        ]
        .spacing(16)
        .max_width(900);

        container(scrollable(content))
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| iced::widget::container::Style {
                background: Some(Background::Color(theme.palette.background)),
                text_color: Some(theme.palette.foreground),
                ..iced::widget::container::Style::default()
            })
            .into()
    }
}

fn panel<'a, Message: 'a>(theme: &'a Theme, content: Element<'a, Message>) -> Element<'a, Message> {
    card(
        container(content).padding(16),
        CardProps::new()
            .variant(CardVariant::Surface)
            .size(CardSize::Size3),
        theme,
    )
    .into()
}
