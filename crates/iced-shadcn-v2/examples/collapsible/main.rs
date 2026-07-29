//! Interactive playground for `iced-shadcn-v2::Collapsible`.
//!
//! Run with:
//! `cargo run -p iced-shadcn-v2 --example collapsible`

use std::collections::BTreeSet;

use iced::widget::{column, container, scrollable, text};
use iced::{Alignment, Background, Element, Length, Task};

use iced_shadcn_v2::{
    ButtonSize, ButtonVariant, Collapsible, CollapsibleAlignment, CollapsibleContent,
    CollapsibleEasing, CollapsibleIndicator, CollapsibleIndicatorPlacement, CollapsibleOrientation,
    CollapsibleTrigger, Padding, SemanticColor, Spacing, StyleId, Theme, fonts, iced_font,
};

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .default_font(iced_font(iced_shadcn_v2::FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

/// Folders of the file-tree section, keyed by name.
const FOLDERS: [(&str, &[&str]); 3] = [
    ("src", &["app.css", "app.html", "hooks.server.ts"]),
    ("lib", &["utils.ts", "stores.ts", "types.ts"]),
    (
        "routes",
        &["+layout.svelte", "+page.svelte", "+error.svelte"],
    ),
];

struct Example {
    theme: Theme,
    faq: bool,
    settings: bool,
    inline: bool,
    linear: bool,
    open_folders: BTreeSet<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Faq(bool),
    Settings(bool),
    Inline(bool),
    Linear(bool),
    Folder(&'static str),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light().with_style(StyleId::Nova),
            faq: true,
            settings: false,
            inline: false,
            linear: false,
            open_folders: BTreeSet::from(["src"]),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Collapsible".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Faq(open) => self.faq = open,
            Message::Settings(open) => self.settings = open,
            Message::Inline(open) => self.inline = open,
            Message::Linear(open) => self.linear = open,
            Message::Folder(name) => {
                if !self.open_folders.remove(name) {
                    self.open_folders.insert(name);
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;

        let faq = Collapsible::new(theme)
            .open(self.faq)
            .width(Length::Fixed(420.0))
            .trigger(
                CollapsibleTrigger::text("Can I use this in my project?", theme)
                    .indicator(CollapsibleIndicator::Chevron)
                    .full_width(true),
            )
            .content(
                CollapsibleContent::new(theme)
                    .background(SemanticColor::Muted)
                    .radius(8.0)
                    .padding(Padding::all(Spacing::S3))
                    .expect("scale padding is supported")
                    .push(text(
                        "Yes. Free to use for personal and commercial projects. \
                         No attribution required.",
                    )),
            )
            .on_open_change(Message::Faq);

        let settings = Collapsible::new(theme)
            .open(self.settings)
            .orientation(CollapsibleOrientation::Horizontal)
            .align(CollapsibleAlignment::Start)
            .width(Length::Fixed(420.0))
            .push(
                column![
                    text("Radius").size(15).color(palette.foreground),
                    text("Set the corner radius of the element.")
                        .size(13)
                        .color(palette.muted_foreground),
                ]
                .spacing(4)
                .width(Length::Fill),
            )
            .content(
                CollapsibleContent::new(theme)
                    .width(Length::Fixed(120.0))
                    .push(text("Radius X: 0").size(13))
                    .push(text("Radius Y: 0").size(13))
                    .spacing(4.0),
            )
            .trigger(
                CollapsibleTrigger::chevron(theme)
                    .variant(ButtonVariant::Outline)
                    .indicator(CollapsibleIndicator::ChevronDown),
            )
            .on_open_change(Message::Settings);

        let inline = Collapsible::new(theme)
            .open(self.inline)
            .orientation(CollapsibleOrientation::Horizontal)
            .align(CollapsibleAlignment::Center)
            .trigger(
                CollapsibleTrigger::text("Details", theme)
                    .variant(ButtonVariant::Secondary)
                    .size(ButtonSize::Sm)
                    .indicator(CollapsibleIndicator::Chevron)
                    .indicator_placement(CollapsibleIndicatorPlacement::Trailing),
            )
            .content(
                CollapsibleContent::new(theme)
                    .width(Length::Shrink)
                    .push(text("width reveals instead of height").size(13)),
            )
            .on_open_change(Message::Inline);

        let linear = Collapsible::new(theme)
            .open(self.linear)
            .width(Length::Fixed(420.0))
            .easing(CollapsibleEasing::Linear)
            .duration_ms(600)
            .bordered(true)
            .radius(10.0)
            .padding(Padding::all(Spacing::S3))
            .expect("scale padding is supported")
            .trigger(
                CollapsibleTrigger::text("Slow linear reveal", theme)
                    .indicator(CollapsibleIndicator::ChevronDown)
                    .full_width(true),
            )
            .content(
                CollapsibleContent::new(theme)
                    .push(text("600 ms, constant speed.").size(13))
                    .push(text("Compare it with the smoothstep default above.").size(13))
                    .spacing(4.0),
            )
            .on_open_change(Message::Linear);

        let tree = column(FOLDERS.map(|(name, files)| self.folder(name, files)))
            .spacing(4)
            .width(Length::Fixed(280.0));

        let disabled = Collapsible::<Message>::new(theme)
            .disabled(true)
            .width(Length::Fixed(420.0))
            .trigger(
                CollapsibleTrigger::text("Unavailable", theme)
                    .indicator(CollapsibleIndicator::Chevron)
                    .full_width(true),
            )
            .content(CollapsibleContent::new(theme).push(text("Never revealed.")));

        let content = column![
            text("iced-shadcn-v2 Collapsible")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("shadcn-svelte parity: controlled open, disabled, animated reveal, chevron")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            section("FAQ panel", faq, theme),
            section("Settings row (horizontal root)", settings, theme),
            section("Inline width reveal", inline, theme),
            section("Linear easing, 600 ms", linear, theme),
            section("File tree", tree, theme),
            section("Disabled", disabled, theme),
        ]
        .spacing(18)
        .max_width(900)
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
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.foreground),
            ..container::Style::default()
        })
        .into()
    }

    fn folder(&self, name: &'static str, files: &'static [&'static str]) -> Element<'_, Message> {
        let theme = &self.theme;

        Collapsible::new(theme)
            .open(self.open_folders.contains(name))
            .spacing(2.0)
            .trigger(
                CollapsibleTrigger::text(name, theme)
                    .size(ButtonSize::Sm)
                    .indicator(CollapsibleIndicator::Chevron)
                    .full_width(true),
            )
            .content(
                CollapsibleContent::with_children(
                    theme,
                    files
                        .iter()
                        .map(|file| text(*file).size(13).into())
                        .collect::<Vec<_>>(),
                )
                .spacing(2.0)
                .padding(Padding::individual(
                    Spacing::S1,
                    Spacing::S0,
                    Spacing::S0,
                    Spacing::S5,
                ))
                .expect("scale padding is supported"),
            )
            .on_press(Message::Folder(name))
            .into()
    }
}

fn section<'a>(
    label: &'static str,
    body: impl Into<Element<'a, Message>>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    column![
        text(label)
            .size(17)
            .font(iced_font(theme.font_pack().heading))
            .color(theme.palette.muted_foreground),
        body.into(),
    ]
    .spacing(8)
    .align_x(Alignment::Start)
    .into()
}
