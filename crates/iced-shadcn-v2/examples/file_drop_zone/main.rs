//! Interactive playground for `iced-shadcn-v2::FileDropZone`.
//!
//! FileDropZone has no pack-specific tokens in shadcn-svelte-extras (hard-coded
//! Tailwind, same idea as Form). Choosing a style pack on the shared [`Theme`]
//! therefore styles the zone via pack radius / palette / fonts, and styles
//! composed Button rows through that pack's own recipes (e.g. Rhea button).
//!
//! Drop-only (no native picker):
//! `cargo run -p iced-shadcn-v2 --example file_drop_zone`
//!
//! Click-to-pick + drop (`rfd`):
//! `cargo run -p iced-shadcn-v2 --example file_drop_zone --features rfd`

use std::fmt;
use std::path::PathBuf;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    ACCEPT_IMAGE, AccentColor, BaseColor, Button, ButtonVariant, FileDropZone, FileDropZoneAction,
    FileDropZoneFile, FileDropZoneState, FileDropZoneVariant, FileRejectedReason, FontHeading,
    FontId, MEGABYTE, RadiusId, StyleId, Theme, ThemeMode, display_size, file_drop_zone_load_files,
    fonts, iced_font,
};

#[cfg(feature = "rfd")]
use iced_shadcn_v2::file_drop_zone_pick_files;

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    state: FileDropZoneState,
    files: Vec<FileDropZoneFile>,
    rejected: Vec<(PathBuf, FileRejectedReason)>,
    disabled: bool,
    uploading: bool,
    variant: FileDropZoneVariant,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Font(Labelled<FontId>),
    Heading(Labelled<FontHeading>),
    Radius(Labelled<RadiusId>),
    Variant(LabelledVariant),
    ToggleDisabled,
    Zone(FileDropZoneAction),
    #[cfg(feature = "rfd")]
    Picked(Vec<PathBuf>),
    Loaded(Vec<FileDropZoneFile>),
    Remove(usize),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            state: FileDropZoneState::new(),
            files: Vec::new(),
            rejected: Vec::new(),
            disabled: false,
            uploading: false,
            variant: FileDropZoneVariant::Default,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 File Drop Zone".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => {
                self.theme = self.theme.clone().with_style(style.0);
            }
            Message::Base(base) => {
                self.theme = self.theme.clone().with_base(base.0);
            }
            Message::Accent(accent) => {
                self.theme = self.theme.clone().with_accent(accent.into_option());
            }
            Message::Mode(mode) => {
                self.theme = self.theme.clone().with_mode(mode.0);
            }
            Message::Font(font) => {
                self.theme = self.theme.clone().with_font(font.0);
            }
            Message::Heading(heading) => {
                self.theme = self.theme.clone().with_font_heading(heading.0);
            }
            Message::Radius(radius) => {
                self.theme = self.theme.clone().with_radius(radius.0);
            }
            Message::Variant(variant) => {
                self.variant = variant.0;
            }
            Message::ToggleDisabled => {
                self.disabled = !self.disabled;
            }
            Message::Zone(action) => {
                self.state.apply(&action);
                return self.handle_zone(action);
            }
            #[cfg(feature = "rfd")]
            Message::Picked(paths) => {
                self.uploading = false;
                let (accepted, rejected) = iced_shadcn_v2::file_drop_zone_partition_paths(
                    paths,
                    &shadcn_common::FileDropZoneConfig::new()
                        .with_accept(ACCEPT_IMAGE)
                        .with_max_files(4)
                        .with_file_count(self.files.len())
                        .with_max_file_size(3 * MEGABYTE),
                );
                self.rejected.extend(rejected);
                if accepted.is_empty() {
                    return Task::none();
                }
                return Task::perform(
                    async move { file_drop_zone_load_files(&accepted) },
                    Message::Loaded,
                );
            }
            Message::Loaded(files) => {
                for file in files {
                    if self.files.iter().any(|existing| existing.name == file.name) {
                        continue;
                    }
                    if self.files.len() >= 4 {
                        break;
                    }
                    self.files.push(file);
                }
            }
            Message::Remove(index) => {
                if index < self.files.len() {
                    self.files.remove(index);
                }
            }
        }

        Task::none()
    }

    fn handle_zone(&mut self, action: FileDropZoneAction) -> Task<Message> {
        match action {
            FileDropZoneAction::DropPaths(paths) => {
                let (accepted, rejected) = iced_shadcn_v2::file_drop_zone_partition_paths(
                    paths,
                    &shadcn_common::FileDropZoneConfig::new()
                        .with_accept(ACCEPT_IMAGE)
                        .with_max_files(4)
                        .with_file_count(self.files.len())
                        .with_max_file_size(3 * MEGABYTE),
                );
                self.rejected.extend(rejected);
                if accepted.is_empty() {
                    return Task::none();
                }
                Task::perform(
                    async move { file_drop_zone_load_files(&accepted) },
                    Message::Loaded,
                )
            }
            FileDropZoneAction::Rejected { path, reason } => {
                self.rejected.push((path, reason));
                Task::none()
            }
            #[cfg(feature = "rfd")]
            FileDropZoneAction::PickerRequested => {
                self.uploading = true;
                Task::perform(async { file_drop_zone_pick_files() }, Message::Picked)
            }
            #[cfg(not(feature = "rfd"))]
            FileDropZoneAction::PickerRequested => Task::none(),
            FileDropZoneAction::Hovered(_) => Task::none(),
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;

        let controls = column![
            section_label("Theme (shadcn-common)", p.muted_foreground, theme),
            control_select(
                "Style",
                &STYLES,
                Some(Labelled(theme.style_id())),
                Message::Style,
                theme,
            ),
            control_select(
                "Base",
                &BASES,
                Some(Labelled(theme.base())),
                Message::Base,
                theme,
            ),
            control_select(
                "Accent",
                &ACCENTS,
                Some(AccentOpt::from_option(theme.accent())),
                Message::Accent,
                theme,
            ),
            control_select(
                "Mode",
                &MODES,
                Some(Labelled(theme.mode())),
                Message::Mode,
                theme,
            ),
            control_select(
                "Heading",
                &HEADINGS,
                Some(Labelled(theme.font_heading())),
                Message::Heading,
                theme,
            ),
            control_select(
                "Font",
                &FONTS,
                Some(Labelled(theme.font_id())),
                Message::Font,
                theme,
            ),
            control_select(
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme,
            ),
            control_select(
                "Variant",
                &VARIANTS,
                Some(LabelledVariant(self.variant)),
                Message::Variant,
                theme,
            ),
            text(format!(
                "style={} · zone recipe shared · lg radius={:.0}px · Button uses this pack",
                theme.style_id().as_str(),
                theme.radius_scale().lg_px,
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(p.muted_foreground),
            Button::text(
                if self.disabled {
                    "Enable zone"
                } else {
                    "Disable zone"
                },
                theme,
            )
            .variant(ButtonVariant::Outline)
            .on_press(Message::ToggleDisabled),
        ]
        .spacing(8);

        let zone = FileDropZone::new(theme, &self.state)
            .max_files(4)
            .file_count(self.files.len())
            .max_file_size(3 * MEGABYTE)
            .accept(ACCEPT_IMAGE)
            .disabled(self.disabled)
            .uploading(self.uploading)
            .variant(self.variant)
            .on_action(Message::Zone)
            .trigger();

        let mut file_rows = column![].spacing(8);
        for (index, file) in self.files.iter().enumerate() {
            file_rows = file_rows.push(
                row![
                    column![
                        text(file.name.clone()).size(14).color(p.foreground),
                        text(display_size(file.bytes.len() as u64))
                            .size(12)
                            .color(p.muted_foreground),
                    ]
                    .spacing(2)
                    .width(Length::Fill),
                    Button::text("Remove", theme)
                        .variant(ButtonVariant::Outline)
                        .on_press(Message::Remove(index)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            );
        }

        let rejected = if self.rejected.is_empty() {
            text("No rejections").size(12).color(p.muted_foreground)
        } else {
            text(
                self.rejected
                    .iter()
                    .rev()
                    .take(6)
                    .map(|(path, reason)| format!("{} — {reason}", path.display()))
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
            .size(12)
            .color(p.destructive)
        };

        let content = column![
            text("File Drop Zone")
                .size(30)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text(
                "No FileDropZone style variants in extras — pick Style (e.g. Rhea) and the zone radius / palette plus composed Button follow that pack."
            )
            .size(14)
            .color(p.muted_foreground),
            controls,
            section_label("FileDropZone.Trigger", p.foreground, theme),
            text(picker_hint())
                .size(12)
                .color(p.muted_foreground),
            zone,
            section_label("Uploaded", p.foreground, theme),
            file_rows,
            section_label("Rejected", p.foreground, theme),
            rejected,
        ]
        .spacing(16)
        .max_width(720)
        .padding(24);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(p.background)),
                text_color: Some(p.foreground),
                ..container::Style::default()
            })
            .into()
    }
}

fn control_select<'a, T>(
    label: &'static str,
    options: &'static [T],
    selected: Option<T>,
    on_select: impl Fn(T) -> Message + 'a,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    T: Clone + fmt::Display + PartialEq + 'static,
{
    let p = &theme.palette;
    let font = iced_font(theme.font_id());
    row![
        text(label)
            .size(13)
            .font(font)
            .color(p.muted_foreground)
            .width(Length::Fixed(72.0)),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(200.0))
            .style(move |_theme, _status| pick_list::Style {
                background: Background::Color(p.background),
                text_color: p.foreground,
                placeholder_color: p.muted_foreground,
                handle_color: p.muted_foreground,
                border: Border {
                    color: p.input,
                    width: 1.0,
                    radius: 6.0.into(),
                },
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn section_label<'a>(label: &'static str, color: Color, theme: &Theme) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(theme.font_pack().heading))
        .color(color)
        .into()
}

fn picker_hint() -> &'static str {
    #[cfg(feature = "rfd")]
    {
        "accept=image/* · maxFiles=4 · maxFileSize=3 MB · click or drop"
    }
    #[cfg(not(feature = "rfd"))]
    {
        "accept=image/* · maxFiles=4 · maxFileSize=3 MB · drop files (enable --features rfd for click-to-pick)"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Labelled<T>(T);

impl fmt::Display for Labelled<StyleId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<BaseColor> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<ThemeMode> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<FontId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<FontHeading> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<RadiusId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.label())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LabelledVariant(FileDropZoneVariant);

impl fmt::Display for LabelledVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            FileDropZoneVariant::Default => "default",
            FileDropZoneVariant::Surface => "surface",
            FileDropZoneVariant::Soft => "soft",
            _ => "default",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AccentOpt {
    None,
    Color(AccentColor),
}

impl AccentOpt {
    const fn from_option(accent: Option<AccentColor>) -> Self {
        match accent {
            None => Self::None,
            Some(color) => Self::Color(color),
        }
    }

    const fn into_option(self) -> Option<AccentColor> {
        match self {
            Self::None => None,
            Self::Color(color) => Some(color),
        }
    }
}

impl fmt::Display for AccentOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => f.write_str("none"),
            Self::Color(color) => f.write_str(color.as_str()),
        }
    }
}

const STYLES: [Labelled<StyleId>; 8] = [
    Labelled(StyleId::Vega),
    Labelled(StyleId::Nova),
    Labelled(StyleId::Maia),
    Labelled(StyleId::Lyra),
    Labelled(StyleId::Mira),
    Labelled(StyleId::Luma),
    Labelled(StyleId::Sera),
    Labelled(StyleId::Rhea),
];

const BASES: [Labelled<BaseColor>; 7] = [
    Labelled(BaseColor::Neutral),
    Labelled(BaseColor::Zinc),
    Labelled(BaseColor::Stone),
    Labelled(BaseColor::Mauve),
    Labelled(BaseColor::Mist),
    Labelled(BaseColor::Olive),
    Labelled(BaseColor::Taupe),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

const ACCENTS: [AccentOpt; 6] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Rose),
    AccentOpt::Color(AccentColor::Violet),
];

const FONTS: [Labelled<FontId>; 5] = [
    Labelled(FontId::Geist),
    Labelled(FontId::Inter),
    Labelled(FontId::InstrumentSerif),
    Labelled(FontId::GeistMono),
    Labelled(FontId::JetBrainsMono),
];

const HEADINGS: [Labelled<FontHeading>; 6] = [
    Labelled(FontHeading::Inherit),
    Labelled(FontHeading::Font(FontId::Geist)),
    Labelled(FontHeading::Font(FontId::Inter)),
    Labelled(FontHeading::Font(FontId::InstrumentSerif)),
    Labelled(FontHeading::Font(FontId::GeistMono)),
    Labelled(FontHeading::Font(FontId::JetBrainsMono)),
];

const RADII: [Labelled<RadiusId>; 5] = [
    Labelled(RadiusId::Default),
    Labelled(RadiusId::None),
    Labelled(RadiusId::Small),
    Labelled(RadiusId::Medium),
    Labelled(RadiusId::Large),
];

const VARIANTS: [LabelledVariant; 3] = [
    LabelledVariant(FileDropZoneVariant::Default),
    LabelledVariant(FileDropZoneVariant::Surface),
    LabelledVariant(FileDropZoneVariant::Soft),
];
