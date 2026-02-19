use iced::border::Border;
use iced::widget::{column, container, row, scrollable};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    CheckboxProps, CheckboxSize, CheckboxState, ControlSize, LabelProps, LabelVariant, Theme,
    checkbox, label, label_with_props,
};
use lucide_icons::LUCIDE_FONT_BYTES;

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

struct Example {
    theme: Theme,
    terms_state: CheckboxState,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            terms_state: CheckboxState::Unchecked,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Toggle(CheckboxState),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Toggle(state) => self.terms_state = state,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let mut content = column![].spacing(16).width(Length::Fill);

        // -- With Checkbox --
        content = content.push(section_title("With Checkbox"));
        content = content.push(preview(
            theme,
            column![
                row![
                    checkbox(
                        self.terms_state,
                        Some(Message::Toggle),
                        CheckboxProps::new().size(CheckboxSize::Size2),
                        theme,
                    ),
                    label("Accept terms and conditions", theme),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
                row![
                    checkbox(
                        CheckboxState::Unchecked,
                        None::<fn(CheckboxState) -> Message>,
                        CheckboxProps::new()
                            .size(CheckboxSize::Size2)
                            .disabled(true),
                        theme,
                    ),
                    label_with_props(
                        "Label disabled by a peer control",
                        LabelProps::new().disabled(true),
                        theme,
                    ),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            ]
            .spacing(12),
        ));

        // -- Variants --
        content = content.push(section_title("Variants"));
        content = content.push(preview(
            theme,
            column![
                label_with_props(
                    "Default variant",
                    LabelProps::new().variant(LabelVariant::Default),
                    theme,
                ),
                label_with_props(
                    "Secondary variant",
                    LabelProps::new().variant(LabelVariant::Secondary),
                    theme,
                ),
                label_with_props(
                    "Muted variant",
                    LabelProps::new().variant(LabelVariant::Muted),
                    theme,
                ),
                label_with_props(
                    "Destructive variant",
                    LabelProps::new().variant(LabelVariant::Destructive),
                    theme,
                ),
            ]
            .spacing(8),
        ));

        // -- Sizes --
        content = content.push(section_title("Sizes"));
        content = content.push(preview(
            theme,
            column![
                label_with_props(
                    "Small label",
                    LabelProps::new().size(ControlSize::Sm),
                    theme,
                ),
                label_with_props(
                    "Medium label (default)",
                    LabelProps::new().size(ControlSize::Md),
                    theme,
                ),
                label_with_props(
                    "Large label",
                    LabelProps::new().size(ControlSize::Lg),
                    theme,
                ),
            ]
            .spacing(8),
        ));

        // -- Disabled --
        content = content.push(section_title("Disabled"));
        content = content.push(preview(
            theme,
            label_with_props(
                "This label is disabled",
                LabelProps::new().disabled(true),
                theme,
            ),
        ));

        // -- Required --
        content = content.push(section_title("Required"));
        content = content.push(preview(
            theme,
            label_with_props("Email address", LabelProps::new().required(true), theme),
        ));

        // -- With Description --
        content = content.push(section_title("With Description"));
        content = content.push(preview(
            theme,
            label_with_props(
                "Username",
                LabelProps::new()
                    .required(true)
                    .description("This is your public display name."),
                theme,
            ),
        ));

        app(theme, scrollable(content).into())
    }
}

fn section_title(title: &str) -> Element<'_, Message> {
    iced::widget::text(title).size(16).into()
}

fn app<'a>(theme: &Theme, content: Element<'a, Message>) -> Element<'a, Message> {
    let background = theme.palette.background;
    container(content)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        })
        .into()
}

fn preview<'a>(
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(16)
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                radius: radius.into(),
                width: 1.0,
                color: border,
            },
            ..iced::widget::container::Style::default()
        })
}
