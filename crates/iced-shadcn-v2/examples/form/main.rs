//! Interactive playground for `iced-shadcn-v2::Form` + `shadcn-common::FormState`.
//!
//! Form has no pack-specific tokens in shadcn-svelte (`form.json` is identical
//! across Vega…Rhea). Choosing a style pack on the shared [`Theme`] therefore
//! styles every composed part — Label, Input, Button — through that pack's
//! own recipes (e.g. Rhea button/input/label).
//!
//! Run: `cargo run -p iced-shadcn-v2 --example form`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, ButtonVariant, FontHeading, FontId, FontPack, Form, FormButton,
    FormControlExt, FormControlProps, FormDescription, FormField, FormFieldErrors, FormFieldset,
    FormLabel, FormLegend, Input, RadiusId, StyleId, Theme, ThemeMode, fonts, iced_font,
};
use shadcn_common::{
    FieldConstraints, FieldValue, FormState, ValidationMode, compose, max_length, min_length,
    required,
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
    form: FormState,
    toast: Option<String>,
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
    UsernameChanged(String),
    Submit,
    Reset,
}

impl Default for Example {
    fn default() -> Self {
        let mut form = FormState::new(ValidationMode::OnSubmit);
        // Matches the shadcn-svelte demo schema: z.string().min(2).max(50)
        form.field_with_constraints(
            "username",
            compose(vec![
                required("Username is required"),
                min_length(2, "String must contain at least 2 character(s)"),
                max_length(50, "String must contain at most 50 character(s)"),
            ]),
            FieldConstraints::new()
                .required(true)
                .min_length(2)
                .max_length(50),
        );

        Self {
            theme: Theme::light(),
            form,
            toast: None,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Form".to_owned()
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
            Message::UsernameChanged(value) => {
                self.form.set_text("username", value);
                self.form.set_submitting(false);
                self.toast = None;
            }
            Message::Submit => {
                if self.form.validate() {
                    let username = self.username().to_owned();
                    self.form.set_submitting(true);
                    self.toast = Some(format!(
                        "You submitted {{\n  \"username\": \"{username}\"\n}}"
                    ));
                } else {
                    self.form.set_submitting(false);
                    self.toast = Some("Please fix the errors in the form.".to_owned());
                }
            }
            Message::Reset => {
                self.form.reset();
                self.toast = None;
            }
        }

        Task::none()
    }

    fn username(&self) -> &str {
        match self.form.value("username") {
            Some(FieldValue::Text(value)) => value,
            _ => "",
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = &theme.palette;

        let field = self
            .form
            .field_state("username")
            .expect("the example registers username before rendering");
        let control_props = FormControlProps::from_field(field);

        // One Theme → Form and every child resolve Rhea/Nova/… from theme.style_id().
        let demo_form = Form::new(theme)
            .width(Length::Fill)
            .push(
                FormField::from_state("username", &self.form, theme)
                    .push(FormLabel::from_field("Username", field, theme))
                    .push(
                        Input::new(theme)
                            .value(self.username())
                            .placeholder("shadcn")
                            .form_control(&control_props)
                            .on_input(Message::UsernameChanged),
                    )
                    .push(FormDescription::text(
                        "This is your public display name.",
                        theme,
                    ))
                    .push(FormFieldErrors::from_field(field, theme)),
            )
            .push(
                FormButton::text("Submit", theme)
                    .variant(ButtonVariant::Default)
                    .loading(self.form.is_submitting())
                    .on_press(Message::Submit),
            );

        let fieldset_demo = FormFieldset::new(theme)
            .push(FormLegend::text("Profile", theme).invalid(field.is_invalid()))
            .push(FormDescription::text(
                "Same Theme styles Label, Input, and Button for the active pack.",
                theme,
            ));

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
            text(format!(
                "style={} · Form gaps shared · Label/Input/Button use this pack",
                theme.style_id().as_str(),
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(palette.muted_foreground),
        ]
        .spacing(8);

        let status = self
            .toast
            .as_deref()
            .unwrap_or(if self.form.submit_attempted() {
                "Validation runs when Submit is pressed."
            } else {
                "Fill the username (2–50 chars) and submit."
            });

        let content = column![
            text("Form")
                .size(30)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text(
                "No Form style variants in the registry — pick Style (e.g. Rhea) and composed Label / Input / Button follow that pack."
            )
            .size(14)
            .color(palette.muted_foreground),
            controls,
            section_label(
                "Demo (shadcn-svelte form)",
                palette.muted_foreground,
                theme.font_pack()
            ),
            container(demo_form)
                .width(Length::Fixed(420.0))
                .padding(24)
                .style(move |_| container::Style {
                    background: Some(Background::Color(palette.card)),
                    border: Border {
                        color: palette.border,
                        width: 1.0,
                        radius: 12.0.into(),
                    },
                    ..container::Style::default()
                }),
            section_label("Fieldset", palette.muted_foreground, theme.font_pack()),
            container(fieldset_demo)
                .width(Length::Fixed(420.0))
                .padding(16)
                .style(move |_| container::Style {
                    background: Some(Background::Color(palette.card)),
                    border: Border {
                        color: palette.border,
                        width: 1.0,
                        radius: 12.0.into(),
                    },
                    ..container::Style::default()
                }),
            text(status).size(13).color(if field.is_invalid() {
                palette.destructive
            } else {
                palette.muted_foreground
            }),
            row![
                FormButton::text("Reset", theme)
                    .variant(ButtonVariant::Outline)
                    .on_press(Message::Reset),
            ]
            .spacing(8),
        ]
        .spacing(16)
        .padding(24)
        .width(Length::Fill)
        .align_x(Alignment::Start);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(palette.background)),
                ..container::Style::default()
            })
            .into()
    }
}

fn section_label<'a>(label: &'static str, color: Color, fonts: FontPack) -> Element<'a, Message> {
    text(label)
        .size(12)
        .font(iced_font(fonts.sans))
        .color(color)
        .into()
}

fn control_select<'a, T>(
    label: &'a str,
    options: &'a [T],
    selected: Option<T>,
    on_select: impl Fn(T) -> Message + 'a,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    T: Clone + fmt::Display + PartialEq + 'static,
{
    let palette = &theme.palette;
    column![
        text(label).size(12).color(palette.muted_foreground),
        pick_list(options, selected, on_select).width(Length::Fixed(220.0)),
    ]
    .spacing(4)
    .into()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
