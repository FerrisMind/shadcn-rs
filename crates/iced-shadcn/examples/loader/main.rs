use iced::border::Border;
use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    ButtonProps, ButtonSize, ButtonVariant, PromptLoaderProps, PromptLoaderSize,
    PromptLoaderVariant, Theme, button, prompt_loader,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

struct Example {
    theme: Theme,
    selected_size: PromptLoaderSize,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SetSize(PromptLoaderSize),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::dark(),
            selected_size: PromptLoaderSize::Md,
        }
    }
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::SetSize(size) => {
                self.selected_size = size;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let basic = section(
            theme,
            "Basic Loader",
            loaders_grid(theme, PromptLoaderSize::Md),
        );

        let sizes_controls = size_controls(theme, self.selected_size);
        let sizes = section(
            theme,
            "Loader Sizes",
            column![sizes_controls, loaders_grid(theme, self.selected_size)].spacing(16),
        );

        app(theme, column![basic, sizes].spacing(20).into())
    }
}

fn size_controls<'a>(theme: &Theme, selected: PromptLoaderSize) -> Element<'a, Message> {
    let button_for = |label: &'static str, size: PromptLoaderSize| {
        let variant = if selected == size {
            ButtonVariant::Solid
        } else {
            ButtonVariant::Outline
        };

        button(
            label,
            Some(Message::SetSize(size)),
            ButtonProps::new().size(ButtonSize::Size1).variant(variant),
            theme,
        )
    };

    row![
        button_for("Small", PromptLoaderSize::Sm),
        button_for("Medium", PromptLoaderSize::Md),
        button_for("Large", PromptLoaderSize::Lg),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn loaders_grid<'a>(theme: &Theme, size: PromptLoaderSize) -> Element<'a, Message> {
    let variants = PromptLoaderVariant::ALL;
    let mut grid = column![].spacing(12);

    for chunk in variants.chunks(4) {
        let mut line = row![].spacing(12).align_y(Alignment::Start);
        for variant in chunk {
            line = line.push(loader_tile(theme, *variant, size));
        }
        grid = grid.push(line);
    }

    grid.into()
}

fn loader_tile<'a>(
    theme: &Theme,
    variant: PromptLoaderVariant,
    size: PromptLoaderSize,
) -> Element<'a, Message> {
    let bg = theme.palette.background;
    let border_c = theme.palette.border;
    let radius = theme.radius.md;
    let muted = theme.palette.muted_foreground;

    container(
        column![
            container(prompt_loader(
                PromptLoaderProps::new().variant(variant).size(size),
                theme
            ))
            .height(Length::Fixed(56.0))
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
            text(variant.label())
                .size(12)
                .style(move |_| iced::widget::text::Style { color: Some(muted) }),
        ]
        .spacing(8)
        .align_x(Alignment::Center),
    )
    .padding(12)
    .width(Length::Fixed(180.0))
    .style(move |_| iced::widget::container::Style {
        background: Some(Background::Color(bg)),
        border: Border {
            color: border_c,
            width: 1.0,
            radius: radius.into(),
        },
        ..iced::widget::container::Style::default()
    })
    .into()
}

fn section<'a>(
    theme: &Theme,
    title: &'a str,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let fg = theme.palette.foreground;
    let card = theme.palette.card;
    let border_c = theme.palette.border;
    let radius = theme.radius.md;

    container(
        column![
            text(title)
                .size(24)
                .style(move |_| iced::widget::text::Style { color: Some(fg) }),
            container(content.into())
                .padding(16)
                .width(Length::Fill)
                .style(move |_| iced::widget::container::Style {
                    background: Some(Background::Color(card)),
                    border: Border {
                        color: border_c,
                        width: 1.0,
                        radius: radius.into(),
                    },
                    ..iced::widget::container::Style::default()
                }),
        ]
        .spacing(10),
    )
    .width(Length::Fill)
}

fn app<'a>(theme: &Theme, content: Element<'a, Message>) -> Element<'a, Message> {
    let background = theme.palette.background;

    container(scrollable(content).height(Length::Fill))
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        })
        .into()
}
