//! Interactive playground for `iced-shadcn-v2::Textarea` + `shadcn-common` theme knobs.
//!
//! Mirrors the `button` example's theme control panel and the shadcn-svelte
//! textarea showcase (default, disabled, with label, with helper text, with
//! button, and form layouts).
//!
//! Run: `cargo run -p iced-shadcn-v2 --example textarea`

use std::fmt;

use iced::advanced::text::Wrapping;
use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonVariant, FontHeading, FontId, FontPack, Label, RadiusId,
    StyleId, Textarea, TextareaResize, TextareaSize, Theme, ThemeMode, fonts, iced_font,
    textarea_apply_action,
};

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
    content_default: iced::widget::text_editor::Content,
    content_disabled: iced::widget::text_editor::Content,
    content_label: iced::widget::text_editor::Content,
    content_text: iced::widget::text_editor::Content,
    content_button: iced::widget::text_editor::Content,
    content_form: iced::widget::text_editor::Content,
    content_invalid: iced::widget::text_editor::Content,
    content_read_only: iced::widget::text_editor::Content,
    content_max_len: iced::widget::text_editor::Content,
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
    EditDefault(iced::widget::text_editor::Action),
    EditLabel(iced::widget::text_editor::Action),
    EditText(iced::widget::text_editor::Action),
    EditButton(iced::widget::text_editor::Action),
    EditForm(iced::widget::text_editor::Action),
    EditInvalid(iced::widget::text_editor::Action),
    EditReadOnly(iced::widget::text_editor::Action),
    EditMaxLen(iced::widget::text_editor::Action),
    Send,
    Submit,
}

impl Default for Example {
    fn default() -> Self {
        let mut content_read_only = iced::widget::text_editor::Content::new();
        content_read_only.perform(iced::widget::text_editor::Action::Edit(
            iced::widget::text_editor::Edit::Paste(
                "This textarea is read-only. You can select and copy, but not edit."
                    .to_owned()
                    .into(),
            ),
        ));

        Self {
            theme: Theme::light(),
            content_default: iced::widget::text_editor::Content::new(),
            content_disabled: iced::widget::text_editor::Content::new(),
            content_label: iced::widget::text_editor::Content::new(),
            content_text: iced::widget::text_editor::Content::new(),
            content_button: iced::widget::text_editor::Content::new(),
            content_form: iced::widget::text_editor::Content::new(),
            content_invalid: iced::widget::text_editor::Content::new(),
            content_read_only,
            content_max_len: iced::widget::text_editor::Content::new(),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Textarea".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => self.theme = self.theme.clone().with_style(style.0),
            Message::Base(base) => self.theme = self.theme.clone().with_base(base.0),
            Message::Accent(accent) => {
                self.theme = self.theme.clone().with_accent(accent.into_option());
            }
            Message::Mode(mode) => self.theme = self.theme.clone().with_mode(mode.0),
            Message::Font(font) => self.theme = self.theme.clone().with_font(font.0),
            Message::Heading(heading) => {
                self.theme = self.theme.clone().with_font_heading(heading.0);
            }
            Message::Radius(radius) => self.theme = self.theme.clone().with_radius(radius.0),
            Message::EditDefault(action) => {
                textarea_apply_action(&mut self.content_default, action, false, false, None);
            }
            Message::EditLabel(action) => {
                textarea_apply_action(&mut self.content_label, action, false, false, None);
            }
            Message::EditText(action) => {
                textarea_apply_action(&mut self.content_text, action, false, false, None);
            }
            Message::EditButton(action) => {
                textarea_apply_action(&mut self.content_button, action, false, false, None);
            }
            Message::EditForm(action) => {
                textarea_apply_action(&mut self.content_form, action, false, false, None);
            }
            Message::EditInvalid(action) => {
                textarea_apply_action(&mut self.content_invalid, action, false, false, None);
            }
            Message::EditReadOnly(action) => {
                textarea_apply_action(&mut self.content_read_only, action, false, true, None);
            }
            Message::EditMaxLen(action) => {
                textarea_apply_action(&mut self.content_max_len, action, false, false, Some(280));
            }
            Message::Send | Message::Submit => {}
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;

        let controls = column![
            section_label(
                "Theme (shadcn-common)",
                p.muted_foreground,
                theme.font_pack()
            ),
            control_select(
                "Style",
                &STYLES,
                Some(Labelled(theme.style_id())),
                Message::Style,
                theme
            ),
            control_select(
                "Base",
                &BASES,
                Some(Labelled(theme.base())),
                Message::Base,
                theme
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
                theme
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
                theme
            ),
            control_select(
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme,
            ),
            text(format!(
                "radius lg={:.0}px · control h={:.0}/{:.0}/{:.0} · sans={} · heading={}",
                theme.radius_scale().lg_px,
                theme.style.control_height_sm_px,
                theme.style.control_height_md_px,
                theme.style.control_height_lg_px,
                theme.font_pack().sans.title(),
                theme.font_heading().title(),
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(p.muted_foreground),
        ]
        .spacing(8);

        let showcase = column![
            section_label("Textarea", p.muted_foreground, theme.font_pack()),
            Textarea::new(&self.content_default, theme)
                .placeholder("Type your message here.")
                .wrapping(Wrapping::WordOrGlyph)
                .on_action(Message::EditDefault)
                .width(420.0),
            section_label("Disabled", p.muted_foreground, theme.font_pack()),
            Textarea::new(&self.content_disabled, theme)
                .placeholder("Type your message here.")
                .wrapping(Wrapping::WordOrGlyph)
                .disabled(true)
                .width(420.0),
            section_label("With label", p.muted_foreground, theme.font_pack()),
            column![
                Label::text("Your message", theme),
                Textarea::new(&self.content_label, theme)
                    .placeholder("Type your message here.")
                    .wrapping(Wrapping::WordOrGlyph)
                    .on_action(Message::EditLabel)
                    .width(420.0),
            ]
            .spacing(12),
            section_label("With helper text", p.muted_foreground, theme.font_pack()),
            column![
                Label::text("Your Message", theme),
                Textarea::new(&self.content_text, theme)
                    .placeholder("Type your message here.")
                    .wrapping(Wrapping::WordOrGlyph)
                    .on_action(Message::EditText)
                    .width(420.0),
                text("Your message will be copied to the support team.")
                    .size(14)
                    .font(iced_font(theme.font_pack().sans))
                    .color(p.muted_foreground),
            ]
            .spacing(12),
            section_label("With button", p.muted_foreground, theme.font_pack()),
            column![
                Textarea::new(&self.content_button, theme)
                    .placeholder("Type your message here.")
                    .wrapping(Wrapping::WordOrGlyph)
                    .on_action(Message::EditButton)
                    .width(420.0),
                Button::text("Send message", theme)
                    .variant(ButtonVariant::Default)
                    .on_press(Message::Send),
            ]
            .spacing(12),
            section_label("Form", p.muted_foreground, theme.font_pack()),
            column![
                Label::text("Bio", theme),
                Textarea::new(&self.content_form, theme)
                    .placeholder("Tell us a little bit about yourself")
                    .wrapping(Wrapping::WordOrGlyph)
                    .on_action(Message::EditForm)
                    .width(420.0),
                text("You can @mention other users and organizations.")
                    .size(14)
                    .font(iced_font(theme.font_pack().sans))
                    .color(p.muted_foreground),
                Button::text("Submit", theme)
                    .variant(ButtonVariant::Default)
                    .on_press(Message::Submit),
            ]
            .spacing(12),
            section_label(
                "Invalid (aria-invalid)",
                p.muted_foreground,
                theme.font_pack()
            ),
            Textarea::new(&self.content_invalid, theme)
                .placeholder("This field is required.")
                .wrapping(Wrapping::WordOrGlyph)
                .invalid(true)
                .on_action(Message::EditInvalid)
                .width(420.0),
            section_label("Read-only", p.muted_foreground, theme.font_pack()),
            Textarea::new(&self.content_read_only, theme)
                .placeholder("Read-only textarea.")
                .wrapping(Wrapping::WordOrGlyph)
                .read_only(true)
                .on_action(Message::EditReadOnly)
                .width(420.0),
            section_label(
                "Max length (280 chars)",
                p.muted_foreground,
                theme.font_pack()
            ),
            Textarea::new(&self.content_max_len, theme)
                .placeholder("Type up to 280 characters...")
                .wrapping(Wrapping::WordOrGlyph)
                .max_len(280)
                .on_action(Message::EditMaxLen)
                .width(420.0),
            section_label("Sizes", p.muted_foreground, theme.font_pack()),
            row![
                Textarea::new(&self.content_default, theme)
                    .placeholder("sm")
                    .size(TextareaSize::Sm)
                    .resize(TextareaResize::None)
                    .on_action(Message::EditDefault),
                Textarea::new(&self.content_default, theme)
                    .placeholder("default")
                    .size(TextareaSize::Default)
                    .resize(TextareaResize::None)
                    .on_action(Message::EditDefault),
                Textarea::new(&self.content_default, theme)
                    .placeholder("lg")
                    .size(TextareaSize::Lg)
                    .resize(TextareaResize::None)
                    .on_action(Message::EditDefault),
            ]
            .spacing(12)
            .align_y(Alignment::Start),
        ]
        .spacing(24)
        .max_width(480);

        let title_px = 32u32;

        let content = column![
            text("iced-shadcn-v2 Textarea")
                .size(title_px)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("shadcn-common recipes shared with egui-shadcn (TextareaRecipe)")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            showcase,
        ]
        .spacing(16)
        .max_width(960)
        .padding(8);

        container(
            scrollable(
                container(content)
                    .width(Length::Fill)
                    .center_x(Length::Fill)
                    .padding(24),
            )
            .width(Length::Fill)
            .height(Length::Fill),
        )
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
    let p = theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(72)
            .font(font)
            .color(p.muted_foreground),
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

fn section_label<'a>(label: &'static str, color: Color, pack: FontPack) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(pack.heading))
        .color(color)
        .into()
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
