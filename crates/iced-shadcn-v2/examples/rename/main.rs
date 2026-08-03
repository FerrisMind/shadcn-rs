//! Interactive playground for `iced-shadcn-v2::Rename`.
//!
//! Run with:
//! `cargo run -p iced-shadcn-v2 --example rename`

use std::fmt;
use std::rc::Rc;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonVariant, FontHeading, FontId, FontPack, RadiusId, Rename,
    RenameAction, RenameActionHandler, RenameBlurBehavior, RenameButtonProps,
    RenameFallbackSelectionBehavior, RenameInputTag, RenameMode, RenameProviderProps,
    RenameRootProps, RenameSelectionRequest, RenameState, StyleId, Theme, ThemeMode, fonts,
    iced_font, rename_apply_action, rename_cancel, rename_edit, rename_provider, rename_save,
};

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .window_size(iced::Size::new(1180.0, 820.0))
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    state: RenameState,
    root_props: RenameRootProps,
    input_tag: RenameInputTag,
    blur_behavior: RenameBlurBehavior,
    selection: RenameFallbackSelectionBehavior,
    text_size: f32,
    last_action: Option<String>,
    disabled: bool,
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
    InputTag(Labelled<RenameInputTag>),
    Blur(Labelled<RenameBlurBehavior>),
    Selection(Labelled<RenameFallbackSelectionBehavior>),
    TextSize(TextSizeOpt),
    ToggleDisabled,
    Rename(RenameAction),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            state: RenameState::new("chore: bump deps"),
            root_props: RenameRootProps::new(),
            input_tag: RenameInputTag::Input,
            blur_behavior: RenameBlurBehavior::None,
            selection: RenameFallbackSelectionBehavior::End,
            text_size: 20.0,
            last_action: None,
            disabled: false,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Rename".to_owned()
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
            Message::InputTag(input_tag) => {
                self.input_tag = input_tag.0;
            }
            Message::Blur(blur) => {
                self.blur_behavior = blur.0;
            }
            Message::Selection(selection) => {
                self.selection = selection.0;
            }
            Message::TextSize(size) => {
                self.text_size = size.0 as f32;
            }
            Message::ToggleDisabled => {
                self.disabled = !self.disabled;
            }
            Message::Rename(action) => return self.apply_action(action),
        }

        Task::none()
    }

    fn apply_action(&mut self, action: RenameAction) -> Task<Message> {
        self.last_action = Some(action_name(&action).to_owned());

        let update = rename_apply_action(
            &mut self.state,
            action,
            self.input_tag,
            self.selection,
            self.blur_behavior,
            Self::validate,
        );

        if update.request_focus() {
            self.focus_task(update)
        } else {
            Task::none()
        }
    }

    fn focus_task(&self, update: iced_shadcn_v2::RenameUpdate) -> Task<Message> {
        let input_id = self.configured_props(&self.root_props).input_id_value();
        let mut tasks: Vec<Task<Message>> = vec![iced::widget::operation::focus(input_id.clone())];

        if self.input_tag == RenameInputTag::Input
            && let Some(selection) = update.selection()
        {
            let selection_task = match selection {
                RenameSelectionRequest::Start => {
                    iced::widget::operation::move_cursor_to_front(input_id.clone())
                }
                RenameSelectionRequest::End => {
                    iced::widget::operation::move_cursor_to_end(input_id.clone())
                }
                RenameSelectionRequest::All => {
                    iced::widget::operation::select_all(input_id.clone())
                }
                _ => iced::widget::operation::move_cursor_to_end(input_id),
            };
            tasks.push(selection_task);
        }

        Task::batch(tasks)
    }

    fn validate(value: &str) -> bool {
        !value.trim().is_empty()
    }

    fn configured_props(&self, props: &RenameRootProps) -> RenameRootProps {
        props
            .clone()
            .input_tag(self.input_tag)
            .blur_behavior(self.blur_behavior)
            .fallback_selection_behavior(self.selection)
            .text_size(self.text_size)
            .width(Length::Fixed(175.0))
            .disabled(self.disabled)
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = &theme.palette;

        let controls = column![
            section_label(
                "Theme (shadcn-common)",
                palette.muted_foreground,
                theme.font_pack()
            ),
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
                "Font",
                &FONTS,
                Some(Labelled(theme.font_id())),
                Message::Font,
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
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme,
            ),
            section_label("Rename knobs", palette.muted_foreground, theme.font_pack()),
            control_select(
                "Editor",
                &INPUT_TAGS,
                Some(Labelled(self.input_tag)),
                Message::InputTag,
                theme,
            ),
            control_select(
                "Blur",
                &BLUR_BEHAVIORS,
                Some(Labelled(self.blur_behavior)),
                Message::Blur,
                theme,
            ),
            control_select(
                "Selection",
                &SELECTIONS,
                Some(Labelled(self.selection)),
                Message::Selection,
                theme,
            ),
            control_select(
                "Text size",
                &TEXT_SIZES,
                Some(TextSizeOpt(self.text_size as u32)),
                Message::TextSize,
                theme,
            ),
            Button::text(
                if self.disabled {
                    "Enable rename"
                } else {
                    "Disable rename"
                },
                theme,
            )
            .variant(ButtonVariant::Outline)
            .on_press(Message::ToggleDisabled),
        ]
        .spacing(8);

        let provider_state = &self.state;
        let provider_theme = theme;
        let provider_props = self.configured_props(&self.root_props);
        let provider_handler: RenameActionHandler<'_, Message> = Rc::new(Message::Rename);
        let provider_preview = rename_provider(
            provider_state,
            Some(provider_handler.clone()),
            RenameProviderProps::new(),
            move |context| {
                let root = Rename::new(provider_state, provider_theme)
                    .props(context.root_props(provider_props))
                    .on_action(Message::Rename);
                let buttons: Element<'_, Message> = if context.mode() == RenameMode::Edit {
                    row![
                        rename_save(context, provider_theme, RenameButtonProps::default()),
                        rename_cancel(context, provider_theme, RenameButtonProps::default()),
                    ]
                    .spacing(8)
                    .into()
                } else {
                    rename_edit(context, provider_theme, RenameButtonProps::default())
                };

                row![root, buttons]
                    .spacing(12)
                    .align_y(Alignment::Center)
                    .into()
            },
        );

        let status = if self.state.is_invalid() {
            "invalid value"
        } else if self.state.mode().is_edit() {
            "editing"
        } else {
            "view"
        };

        let preview = column![
            text("iced-shadcn-v2 Rename")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("Controlled inline editing with Input, Textarea, and Button composition")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            section_label(
                "Builder + provider controls",
                palette.muted_foreground,
                theme.font_pack(),
            ),
            container(provider_preview)
                .padding(16)
                .width(Length::Fill)
                .style(move |_| card_style(palette.card, palette.border)),
            text(format!(
                "accepted: {:?} · editing: {:?} · state={} · last action={}",
                self.state.value(),
                self.state.editing_value(),
                status,
                self.last_action.as_deref().unwrap_or("none"),
            ))
            .size(13)
            .font(iced_font(theme.font_pack().mono))
            .color(if self.state.is_invalid() {
                palette.destructive
            } else {
                palette.muted_foreground
            }),
            text("Click the value or press Edit. Enter saves, Escape cancels, and the selection control changes the fallback caret behavior.")
                .size(13)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
        ]
        .spacing(16)
        .max_width(760)
        .padding(8);

        let left_pane = container(scrollable(container(controls).padding(4)))
            .width(Length::Fixed(360.0))
            .height(Length::Fill)
            .padding(16)
            .style(move |_| card_style(palette.card, palette.border));

        let right_pane = container(scrollable(container(preview).padding(8)))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(16);

        container(
            row![left_pane, right_pane]
                .spacing(16)
                .align_y(Alignment::Start)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(16)
        .style(move |_| container::Style {
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.foreground),
            ..container::Style::default()
        })
        .into()
    }
}

fn action_name(action: &RenameAction) -> &'static str {
    match action {
        RenameAction::StartEdit => "start-edit",
        RenameAction::InputChanged(_) => "input-changed",
        RenameAction::TextareaEdited(_) => "textarea-edited",
        RenameAction::SaveRequested => "save-requested",
        RenameAction::CancelRequested => "cancel-requested",
        RenameAction::EscapePressed => "escape-pressed",
        RenameAction::BlurDetected => "blur-detected",
        _ => "other",
    }
}

fn card_style(background: Color, border: Color) -> container::Style {
    container::Style {
        background: Some(Background::Color(background)),
        border: Border {
            color: border,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..container::Style::default()
    }
}

fn control_select<'a, T, F>(
    label: &'static str,
    options: &'a [T],
    selected: Option<T>,
    on_select: F,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    T: Clone + PartialEq + fmt::Display + 'a,
    F: Fn(T) -> Message + 'a,
{
    let palette = &theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(82)
            .font(font)
            .color(palette.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(220.0))
            .style(move |_theme, _status| pick_list::Style {
                background: Background::Color(palette.background),
                text_color: palette.foreground,
                placeholder_color: palette.muted_foreground,
                handle_color: palette.muted_foreground,
                border: Border {
                    color: palette.input,
                    width: 1.0,
                    radius: 6.0.into(),
                },
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn section_label<'a>(label: &'static str, color: Color, fonts: FontPack) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(fonts.heading))
        .color(color)
        .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Labelled<T>(T);

impl fmt::Display for Labelled<StyleId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<BaseColor> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<ThemeMode> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<FontId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<FontHeading> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<RadiusId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.label())
    }
}

impl fmt::Display for Labelled<RenameInputTag> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self.0 {
            RenameInputTag::Input => "input",
            RenameInputTag::Textarea => "textarea",
            _ => "unknown",
        })
    }
}

impl fmt::Display for Labelled<RenameBlurBehavior> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self.0 {
            RenameBlurBehavior::Exit => "exit",
            RenameBlurBehavior::None => "none",
            _ => "unknown",
        })
    }
}

impl fmt::Display for Labelled<RenameFallbackSelectionBehavior> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self.0 {
            RenameFallbackSelectionBehavior::Start => "start",
            RenameFallbackSelectionBehavior::End => "end",
            RenameFallbackSelectionBehavior::All => "all",
            _ => "unknown",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TextSizeOpt(u32);

impl fmt::Display for TextSizeOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} px", self.0)
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
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => formatter.write_str("none"),
            Self::Color(color) => formatter.write_str(color.as_str()),
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

const ACCENTS: [AccentOpt; 18] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Cyan),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Fuchsia),
    AccentOpt::Color(AccentColor::Green),
    AccentOpt::Color(AccentColor::Indigo),
    AccentOpt::Color(AccentColor::Lime),
    AccentOpt::Color(AccentColor::Orange),
    AccentOpt::Color(AccentColor::Pink),
    AccentOpt::Color(AccentColor::Purple),
    AccentOpt::Color(AccentColor::Red),
    AccentOpt::Color(AccentColor::Rose),
    AccentOpt::Color(AccentColor::Sky),
    AccentOpt::Color(AccentColor::Teal),
    AccentOpt::Color(AccentColor::Violet),
    AccentOpt::Color(AccentColor::Yellow),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

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

const INPUT_TAGS: [Labelled<RenameInputTag>; 2] = [
    Labelled(RenameInputTag::Input),
    Labelled(RenameInputTag::Textarea),
];

const BLUR_BEHAVIORS: [Labelled<RenameBlurBehavior>; 2] = [
    Labelled(RenameBlurBehavior::None),
    Labelled(RenameBlurBehavior::Exit),
];

const SELECTIONS: [Labelled<RenameFallbackSelectionBehavior>; 3] = [
    Labelled(RenameFallbackSelectionBehavior::Start),
    Labelled(RenameFallbackSelectionBehavior::End),
    Labelled(RenameFallbackSelectionBehavior::All),
];

const TEXT_SIZES: [TextSizeOpt; 3] = [TextSizeOpt(16), TextSizeOpt(20), TextSizeOpt(24)];
