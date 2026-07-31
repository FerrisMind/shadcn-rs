//! Interactive playground for `iced-shadcn-v2::EmojiPicker`.
//!
//! Run with:
//! `cargo run -p iced-shadcn-v2 --example emoji_picker`

use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Background, Border, Element, Length, Task};

use iced_shadcn_v2::{
    Button, ButtonVariant, EmojiPicker, EmojiPickerList, EmojiPickerRecents, EmojiPickerSearch,
    EmojiPickerSkin, FontId, SelectedEmoji, StyleId, Theme, ThemeMode, fonts, iced_font,
};

fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PickerId {
    Basic,
    SearchAndSkin,
    Recents,
    Footer,
    Popover,
}

impl PickerId {
    const fn index(self) -> usize {
        match self {
            Self::Basic => 0,
            Self::SearchAndSkin => 1,
            Self::Recents => 2,
            Self::Footer => 3,
            Self::Popover => 4,
        }
    }

    const fn title(self) -> &'static str {
        match self {
            Self::Basic => "Basic",
            Self::SearchAndSkin => "Search + skin",
            Self::Recents => "Recents",
            Self::Footer => "Footer + tone selector",
            Self::Popover => "Popover composition",
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    ToggleMode,
    CycleStyle,
    Reset,
    QueryChanged(PickerId, String),
    EmojiSelected(PickerId, SelectedEmoji),
    SkinChanged(PickerId, EmojiPickerSkin),
    PopoverTriggerPressed,
    PopoverChanged(bool),
}

struct Example {
    theme: Theme,
    values: [String; 5],
    queries: [String; 5],
    skins: [EmojiPickerSkin; 5],
    recents: EmojiPickerRecents,
    popover_open: bool,
    last_selection: String,
    selection_count: u32,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            values: Default::default(),
            queries: [
                String::new(),
                "hand".to_owned(),
                String::new(),
                String::new(),
                String::new(),
            ],
            skins: [
                EmojiPickerSkin::Default,
                EmojiPickerSkin::Medium,
                EmojiPickerSkin::Default,
                EmojiPickerSkin::Default,
                EmojiPickerSkin::Default,
            ],
            recents: EmojiPickerRecents::new(),
            popover_open: false,
            last_selection: "Nothing selected yet".to_owned(),
            selection_count: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Emoji Picker".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleMode => {
                let mode = if self.theme.is_dark() {
                    ThemeMode::Light
                } else {
                    ThemeMode::Dark
                };
                self.theme = self.theme.clone().with_mode(mode);
            }
            Message::CycleStyle => {
                self.theme = self
                    .theme
                    .clone()
                    .with_style(next_style(self.theme.style_id()));
            }
            Message::Reset => {
                self.values = Default::default();
                self.queries = [
                    String::new(),
                    "hand".to_owned(),
                    String::new(),
                    String::new(),
                    String::new(),
                ];
                self.skins = [
                    EmojiPickerSkin::Default,
                    EmojiPickerSkin::Medium,
                    EmojiPickerSkin::Default,
                    EmojiPickerSkin::Default,
                    EmojiPickerSkin::Default,
                ];
                self.recents.clear();
                self.popover_open = false;
                self.last_selection = "Nothing selected yet".to_owned();
                self.selection_count = 0;
            }
            Message::QueryChanged(id, query) => {
                self.queries[id.index()] = query;
            }
            Message::EmojiSelected(id, selected) => {
                self.values[id.index()] = selected.emoji().to_owned();
                self.skins[id.index()] = selected.skin();
                self.last_selection = format!(
                    "{} · {} · skin {}",
                    selected.emoji(),
                    selected.data().name(),
                    selected.skin().label()
                );
                self.selection_count += 1;

                if id == PickerId::Recents {
                    self.recents.record(&selected);
                }
                if id == PickerId::Popover {
                    self.popover_open = false;
                }
            }
            Message::SkinChanged(id, skin) => {
                self.skins[id.index()] = skin;
            }
            // `Popover` owns the open-state transition. The inert message
            // keeps the nested button enabled without toggling the state a
            // second time.
            Message::PopoverTriggerPressed => {}
            Message::PopoverChanged(open) => {
                self.popover_open = open;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;

        let controls = row![
            Button::text(
                if theme.is_dark() {
                    "Use light mode"
                } else {
                    "Use dark mode"
                },
                theme,
            )
            .variant(ButtonVariant::Outline)
            .on_press(Message::ToggleMode),
            Button::text(format!("Style: {}", theme.style_id().as_str()), theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::CycleStyle),
            Button::text("Reset demo", theme)
                .variant(ButtonVariant::Ghost)
                .on_press(Message::Reset),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .wrap();

        let cards = column![
            picker_card(
                PickerId::Basic,
                "Full catalog, controlled value and query",
                self.basic_picker(theme),
                self,
            ),
            picker_card(
                PickerId::SearchAndSkin,
                "Starts with query=hand and skin=Medium",
                self.search_skin_picker(theme),
                self,
            ),
            picker_card(
                PickerId::Recents,
                "The app owns and records frecency data",
                self.recents_picker(theme),
                self,
            ),
            picker_card(
                PickerId::Footer,
                "Active selection is passed to the footer slot",
                self.footer_picker(theme),
                self,
            ),
            picker_card(
                PickerId::Popover,
                "The same controlled picker composes inside Popover",
                self.popover_picker(theme),
                self,
            ),
        ]
        .spacing(18)
        .width(Length::Fill);

        let content = column![
            text("iced-shadcn-v2 Emoji Picker")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("Complete Unicode catalog · 6-column grid · search · recents · skin tones")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            row![
                text(format!("Last selection: {}", self.last_selection))
                    .size(13)
                    .font(iced_font(theme.font_pack().mono))
                    .color(palette.foreground),
                text(format!("Selected {} times", self.selection_count))
                    .size(13)
                    .font(iced_font(theme.font_pack().mono))
                    .color(palette.muted_foreground),
            ]
            .spacing(16)
            .wrap(),
            controls,
            cards,
        ]
        .spacing(16)
        .max_width(960)
        .padding(8);

        container(scrollable(
            container(content)
                .width(Length::Fill)
                .center_x(Length::Fill)
                .padding(24),
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.foreground),
            ..iced::widget::container::Style::default()
        })
        .into()
    }

    fn basic_picker<'a>(&self, theme: &'a Theme) -> Element<'a, Message> {
        let id = PickerId::Basic;
        EmojiPicker::new(theme)
            .value(&self.values[id.index()])
            .query(&self.queries[id.index()])
            .skin(self.skins[id.index()])
            .on_query_change(move |query| Message::QueryChanged(id, query))
            .on_select(move |selected| Message::EmojiSelected(id, selected))
            .into()
    }

    fn search_skin_picker<'a>(&self, theme: &'a Theme) -> Element<'a, Message> {
        let id = PickerId::SearchAndSkin;
        EmojiPicker::new(theme)
            .value(&self.values[id.index()])
            .query(&self.queries[id.index()])
            .skin(self.skins[id.index()])
            .on_query_change(move |query| Message::QueryChanged(id, query))
            .on_select(move |selected| Message::EmojiSelected(id, selected))
            .on_skin_change(move |skin| Message::SkinChanged(id, skin))
            .into()
    }

    fn recents_picker<'a>(&'a self, theme: &'a Theme) -> Element<'a, Message> {
        let id = PickerId::Recents;
        EmojiPicker::new(theme)
            .value(&self.values[id.index()])
            .query(&self.queries[id.index()])
            .skin(self.skins[id.index()])
            .show_recents(true)
            .recents(&self.recents)
            .max_recents(8)
            .on_query_change(move |query| Message::QueryChanged(id, query))
            .on_select(move |selected| Message::EmojiSelected(id, selected))
            .into()
    }

    fn footer_picker<'a>(&self, theme: &'a Theme) -> Element<'a, Message> {
        let id = PickerId::Footer;
        let footer_theme = theme;
        EmojiPicker::new(theme)
            .value(&self.values[id.index()])
            .query(&self.queries[id.index()])
            .skin(self.skins[id.index()])
            .on_query_change(move |query| Message::QueryChanged(id, query))
            .on_select(move |selected| Message::EmojiSelected(id, selected))
            .on_skin_change(move |skin| Message::SkinChanged(id, skin))
            .footer(move |active| {
                let label = active
                    .map(|selected| format!("{}  {}", selected.emoji(), selected.data().name()))
                    .unwrap_or_else(|| "Choose an emoji".to_owned());

                text(label)
                    .size(12)
                    .font(iced_font(footer_theme.font_pack().sans))
                    .color(footer_theme.palette.muted_foreground)
                    .into()
            })
            .into()
    }

    fn popover_picker<'a>(&self, theme: &'a Theme) -> Element<'a, Message> {
        let id = PickerId::Popover;
        let trigger_label = if self.values[id.index()].is_empty() {
            "Open emoji picker".to_owned()
        } else {
            format!("{}  Pick another", self.values[id.index()])
        };

        let trigger = Button::text(trigger_label, theme)
            .variant(ButtonVariant::Outline)
            .on_press(Message::PopoverTriggerPressed);

        let search = EmojiPickerSearch::new(&self.queries[id.index()], theme)
            .on_input(move |query| Message::QueryChanged(id, query));
        let list = EmojiPickerList::new(theme)
            .query(&self.queries[id.index()])
            .skin(self.skins[id.index()])
            .on_select(move |selected| Message::EmojiSelected(id, selected));
        let picker = column![
            container(search.into_element())
                .width(Length::Fill)
                .padding(8),
            list.into_element(),
        ]
        .width(Length::Fill);

        iced_shadcn_v2::Popover::new(trigger, picker, theme)
            .open(self.popover_open)
            .on_open_change(Message::PopoverChanged)
            .width(232.0)
            .content_padding(0.0)
            .into()
    }
}

fn picker_card<'a>(
    id: PickerId,
    description: &'static str,
    picker: Element<'a, Message>,
    example: &'a Example,
) -> Element<'a, Message> {
    let theme = &example.theme;
    let p = theme.palette;
    let value = &example.values[id.index()];
    let query = &example.queries[id.index()];

    container(
        column![
            row![
                text(id.title())
                    .size(18)
                    .font(iced_font(theme.font_pack().heading))
                    .color(p.foreground),
                text(format!(
                    "value={}  query={}",
                    if value.is_empty() { "∅" } else { value },
                    if query.is_empty() { "∅" } else { query },
                ))
                .size(11)
                .font(iced_font(theme.font_pack().mono))
                .color(p.muted_foreground),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .wrap(),
            text(description)
                .size(13)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            picker,
        ]
        .spacing(8)
        .padding(16),
    )
    .width(Length::Fill)
    .style(move |_| iced::widget::container::Style {
        background: Some(Background::Color(p.card)),
        text_color: Some(p.card_foreground),
        border: Border {
            color: p.border,
            width: 1.0,
            radius: 10.0.into(),
        },
        ..iced::widget::container::Style::default()
    })
    .into()
}

fn next_style(style: StyleId) -> StyleId {
    const STYLES: [StyleId; 8] = [
        StyleId::Vega,
        StyleId::Nova,
        StyleId::Maia,
        StyleId::Lyra,
        StyleId::Mira,
        StyleId::Luma,
        StyleId::Sera,
        StyleId::Rhea,
    ];

    let index = STYLES
        .iter()
        .position(|candidate| *candidate == style)
        .unwrap_or(0);
    STYLES[(index + 1) % STYLES.len()]
}
