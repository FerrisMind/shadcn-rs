//! Interactive playground for `iced-shadcn-v2::Kbd`.
//!
//! Mirrors shadcn-svelte kbd examples: basic keys, modifier keys, groups,
//! arrow keys, icons, icons with text, and contextual surfaces (tooltip /
//! input-group) — plus theme knobs from `shadcn-common`.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example kbd`

use std::fmt;

use iced::border::Border;
use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Color, Element, Length, Task};

use iced_shadcn_v2::{
    BaseColor, FontHeading, FontId, FontPack, Kbd, KbdGroup, KbdRadius, KbdSurface, RadiusId,
    StyleId, Theme, ThemeMode, fonts, iced_font,
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
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
    Font(Labelled<FontId>),
    Heading(Labelled<FontHeading>),
    Radius(Labelled<RadiusId>),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Kbd".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => {
                self.theme = self.theme.clone().with_style(style.0);
            }
            Message::Base(base) => {
                self.theme = self.theme.clone().with_base(base.0);
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
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;
        let font = iced_font(theme.font_pack().sans);

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
        ]
        .spacing(8);

        // shadcn `kbd-basic`: single keys, incl. min-width squares.
        let basic = row![
            Kbd::text("B", theme),
            Kbd::text("K", theme),
            Kbd::text("Esc", theme),
            Kbd::text("Enter", theme),
            Kbd::text("Space", theme),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .wrap();

        // shadcn `kbd-modifier-keys`.
        let modifiers = row![
            Kbd::text("⌘", theme),
            Kbd::text("⇧", theme),
            Kbd::text("⌥", theme),
            Kbd::text("⌃", theme),
            Kbd::text("Ctrl", theme),
            Kbd::text("Alt", theme),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .wrap();

        // shadcn `kbd-group-example`, `kbd-demo` (`<Kbd.Root>Ctrl</Kbd.Root>
        // <span>+</span> <Kbd.Root>B</Kbd.Root>`), and `kbd-group-demo`
        // (`<Kbd.Root>Ctrl + B</Kbd.Root>` — the plus lives inside one kbd).
        let groups = row![
            KbdGroup::new()
                .push(Kbd::text("Ctrl", theme))
                .push(Kbd::text("Shift", theme))
                .push(Kbd::text("P", theme)),
            KbdGroup::new()
                .push(Kbd::text("Ctrl", theme))
                .push(text("+").size(12).font(font).color(p.muted_foreground))
                .push(Kbd::text("B", theme)),
            KbdGroup::new()
                .push(Kbd::text("Ctrl + B", theme))
                .push(Kbd::text("Ctrl + K", theme)),
        ]
        .spacing(24)
        .align_y(Alignment::Center)
        .wrap();

        // shadcn `kbd-arrow-keys` / `kbd-with-icons`: element content.
        let arrows = KbdGroup::new()
            .push(Kbd::new(text("↑").size(12).font(font), theme))
            .push(Kbd::new(text("↓").size(12).font(font), theme))
            .push(Kbd::new(text("←").size(12).font(font), theme))
            .push(Kbd::new(text("→").size(12).font(font), theme));

        // shadcn `kbd-with-icons-and-text`: icon slots next to a label.
        let icons_and_text = row![
            Kbd::text("Save", theme).icon_start(text("✓").size(10).font(font)),
            Kbd::text("Open", theme).icon_end(text("↗").size(10).font(font)),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .wrap();

        // shadcn `kbd-in-tooltip`: chip on a foreground-colored bubble.
        let tooltip_bubble = container(
            row![
                text("Save Changes").size(12).color(p.background),
                Kbd::text("S", theme).surface(KbdSurface::Tooltip),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .padding(iced::Padding {
            top: 4.0,
            right: 6.0,
            bottom: 4.0,
            left: 12.0,
        })
        .style(move |_| container::Style {
            background: Some(Background::Color(p.foreground)),
            border: Border {
                radius: 6.0.into(),
                ..Border::default()
            },
            ..container::Style::default()
        });

        // shadcn `kbd-in-input-group`: chip on an input-like field.
        let input_group = container(
            row![
                text("Search…")
                    .size(13)
                    .font(font)
                    .color(p.muted_foreground),
                iced::widget::Space::new().width(Length::Fill),
                Kbd::text("⌘", theme).surface(KbdSurface::InputGroup),
                Kbd::text("K", theme).surface(KbdSurface::InputGroup),
            ]
            .spacing(4)
            .align_y(Alignment::Center),
        )
        .width(Length::Fixed(240.0))
        .padding(iced::Padding {
            top: 6.0,
            right: 8.0,
            bottom: 6.0,
            left: 12.0,
        })
        .style(move |_| container::Style {
            background: Some(Background::Color(p.background)),
            border: Border {
                color: p.input,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..container::Style::default()
        });

        let radii = row![
            Kbd::text("None", theme).radius(KbdRadius::None),
            Kbd::text("Small", theme).radius(KbdRadius::Small),
            Kbd::text("Medium", theme).radius(KbdRadius::Medium),
            Kbd::text("Large", theme).radius(KbdRadius::Large),
            Kbd::text("Full", theme).radius(KbdRadius::Full),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .wrap();

        let overrides = row![
            Kbd::text("Override", theme).style_override(move |mut style| {
                style.background = Some(Background::Color(Color::from_rgb(0.55, 0.36, 0.96)));
                style.text_color = Some(Color::WHITE);
                style
            }),
            Kbd::text("Wide", theme).width(Length::Fixed(96.0)),
            Kbd::text("No min", theme).min_width(0.0),
            Kbd::text("Big", theme)
                .text_size(16.0)
                .height(Length::Fixed(28.0)),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .wrap();

        let content = column![
            text("iced-shadcn-v2 Kbd")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("shadcn-svelte parity: keys · groups · icons · tooltip / input-group surfaces")
                .size(14)
                .font(font)
                .color(p.muted_foreground),
            controls,
            section_label("Basic", p.muted_foreground, theme.font_pack()),
            basic,
            section_label("Modifier keys", p.muted_foreground, theme.font_pack()),
            modifiers,
            section_label("Group", p.muted_foreground, theme.font_pack()),
            groups,
            section_label("Arrow keys", p.muted_foreground, theme.font_pack()),
            arrows,
            section_label("With icons and text", p.muted_foreground, theme.font_pack()),
            icons_and_text,
            section_label("In tooltip", p.muted_foreground, theme.font_pack()),
            tooltip_bubble,
            section_label("In input group", p.muted_foreground, theme.font_pack()),
            input_group,
            section_label("Radius", p.muted_foreground, theme.font_pack()),
            radii,
            section_label("Overrides", p.muted_foreground, theme.font_pack()),
            overrides,
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
