//! Interactive playground for `iced-shadcn-v2::Checkbox`.
//!
//! The layout mirrors shadcn-svelte's checkbox demo and keeps the control
//! controlled by application state, including its indeterminate state.
//!
//! Run with:
//! `cargo run -p iced-shadcn-v2 --example checkbox`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Element, Length, Task};

use iced_shadcn_v2::AccentColor;
use iced_shadcn_v2::{
    BaseColor, Button, ButtonVariant, Checkbox, CheckboxSize, CheckboxState, CheckboxVariant,
    FontId, StyleId, Theme, ThemeMode, fonts, iced_font,
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
    terms: CheckboxState,
    notifications: CheckboxState,
    desktop_items: [bool; 4],
    variant: VariantOpt,
    size: SizeOpt,
}

#[derive(Debug, Clone)]
enum Message {
    TermsChanged(CheckboxState),
    NotificationsChanged(CheckboxState),
    DesktopItemChanged(usize, CheckboxState),
    Variant(VariantOpt),
    Size(SizeOpt),
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Reset,
    Noop,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            terms: CheckboxState::Unchecked,
            notifications: CheckboxState::Checked,
            desktop_items: [true, false, false, false],
            variant: VariantOpt::Surface,
            size: SizeOpt::Lg,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Checkbox".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TermsChanged(state) => self.terms = state,
            Message::NotificationsChanged(state) => self.notifications = state,
            Message::DesktopItemChanged(index, state) => {
                if let Some(item) = self.desktop_items.get_mut(index) {
                    *item = matches!(state, CheckboxState::Checked);
                }
            }
            Message::Variant(variant) => self.variant = variant,
            Message::Size(size) => self.size = size,
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
            Message::Reset => {
                self.terms = CheckboxState::Unchecked;
                self.notifications = CheckboxState::Checked;
                self.desktop_items = [true, false, false, false];
            }
            Message::Noop => {}
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;

        let controls = column![
            section_label("Theme", theme),
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
            section_label("Checkbox knobs", theme),
            control_select(
                "Variant",
                &VARIANTS,
                Some(self.variant),
                Message::Variant,
                theme,
            ),
            control_select("Size", &SIZES, Some(self.size), Message::Size, theme),
            Button::text("Reset states", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Reset),
        ]
        .spacing(8);

        let basic = Checkbox::new(theme)
            .state(self.terms)
            .variant(self.variant.into())
            .size(self.size.into())
            .label("Accept terms and conditions")
            .on_toggle(Message::TermsChanged);

        let checked_with_description = column![
            Checkbox::new(theme)
                .state(self.notifications)
                .variant(self.variant.into())
                .size(self.size.into())
                .label("Accept terms and conditions")
                .on_toggle(Message::NotificationsChanged),
            text("By clicking this checkbox, you agree to the terms and conditions.")
                .size(13)
                .color(palette.muted_foreground),
        ]
        .spacing(7);

        let disabled = Checkbox::new(theme)
            .state(CheckboxState::Unchecked)
            .variant(self.variant.into())
            .size(self.size.into())
            .label("Enable notifications")
            .disabled(true);

        let indeterminate = Checkbox::new(theme)
            .state(CheckboxState::Indeterminate)
            .variant(self.variant.into())
            .size(self.size.into())
            .label("Partially selected")
            .on_press(Message::Noop);

        let card_checkbox = Checkbox::new(theme)
            .state(self.notifications)
            .variant(CheckboxVariant::Surface)
            .size(CheckboxSize::Md)
            .on_toggle(Message::NotificationsChanged);
        let card = container(
            row![
                card_checkbox,
                column![
                    text("Enable notifications")
                        .size(14)
                        .font(iced_font(theme.font_pack().sans))
                        .color(palette.foreground),
                    text("You can enable or disable notifications at any time.")
                        .size(13)
                        .color(palette.muted_foreground),
                ]
                .spacing(5),
            ]
            .spacing(12)
            .align_y(Alignment::Start),
        )
        .padding(12)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(palette.card)),
            border: Border {
                color: palette.border,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..container::Style::default()
        });

        let field_group = column![
            text("Show these items on the desktop")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.foreground),
            text("Select the items you want to show on the desktop.")
                .size(13)
                .color(palette.muted_foreground),
            self.desktop_checkbox(0, "Hard disks", theme),
            self.desktop_checkbox(1, "External disks", theme),
            self.desktop_checkbox(2, "CDs, DVDs, and iPods", theme),
            self.desktop_checkbox(3, "Connected servers", theme),
        ]
        .spacing(9);

        let preview = column![
            section_label("Preview", theme),
            basic,
            checked_with_description,
            disabled,
            indeterminate,
            card,
            field_group,
        ]
        .spacing(16);

        let states = row![
            labelled_checkbox(
                "unchecked",
                self.demo_checkbox(theme, CheckboxState::Unchecked),
                theme,
            ),
            labelled_checkbox(
                "checked",
                self.demo_checkbox(theme, CheckboxState::Checked),
                theme,
            ),
            labelled_checkbox(
                "indeterminate",
                self.demo_checkbox(theme, CheckboxState::Indeterminate),
                theme,
            ),
            labelled_checkbox(
                "disabled",
                self.demo_checkbox(theme, CheckboxState::Checked)
                    .disabled(true),
                theme,
            ),
        ]
        .spacing(28)
        .align_y(Alignment::Center)
        .wrap();

        let variants = row![
            labelled_checkbox(
                "surface",
                self.demo_checkbox(theme, CheckboxState::Checked)
                    .variant(CheckboxVariant::Surface),
                theme,
            ),
            labelled_checkbox(
                "classic",
                self.demo_checkbox(theme, CheckboxState::Checked)
                    .variant(CheckboxVariant::Classic),
                theme,
            ),
            labelled_checkbox(
                "soft",
                self.demo_checkbox(theme, CheckboxState::Checked)
                    .variant(CheckboxVariant::Soft),
                theme,
            ),
        ]
        .spacing(28)
        .align_y(Alignment::Center)
        .wrap();

        let sizes = row![
            labelled_checkbox(
                "sm",
                self.demo_checkbox(theme, CheckboxState::Checked)
                    .size(CheckboxSize::Sm),
                theme,
            ),
            labelled_checkbox(
                "md",
                self.demo_checkbox(theme, CheckboxState::Checked)
                    .size(CheckboxSize::Md),
                theme,
            ),
            labelled_checkbox(
                "lg",
                self.demo_checkbox(theme, CheckboxState::Checked)
                    .size(CheckboxSize::Lg),
                theme,
            ),
        ]
        .spacing(28)
        .align_y(Alignment::Center)
        .wrap();

        let style_buttons = row![
            style_button(StyleId::Vega, theme),
            style_button(StyleId::Nova, theme),
            style_button(StyleId::Maia, theme),
            style_button(StyleId::Lyra, theme),
            style_button(StyleId::Mira, theme),
            style_button(StyleId::Luma, theme),
            style_button(StyleId::Sera, theme),
            style_button(StyleId::Rhea, theme),
        ]
        .spacing(8)
        .wrap();

        let content = column![
            text("iced-shadcn-v2 Checkbox")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("shadcn-svelte parity: controlled state, indeterminate, labels, disabled")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            controls,
            preview,
            section_label("States", theme),
            states,
            section_label("Variants", theme),
            variants,
            section_label("Sizes", theme),
            sizes,
            section_label("All style packs", theme),
            style_buttons,
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
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.foreground),
            ..container::Style::default()
        })
        .into()
    }

    fn demo_checkbox<'a>(&self, theme: &'a Theme, state: CheckboxState) -> Checkbox<'a, Message> {
        Checkbox::new(theme)
            .state(state)
            .variant(self.variant.into())
            .size(self.size.into())
            .on_press(Message::Noop)
    }

    fn desktop_checkbox<'a>(
        &self,
        index: usize,
        label: &'static str,
        theme: &'a Theme,
    ) -> Element<'a, Message> {
        let state = if self.desktop_items[index] {
            CheckboxState::Checked
        } else {
            CheckboxState::Unchecked
        };

        Checkbox::new(theme)
            .state(state)
            .size(CheckboxSize::Sm)
            .label(label)
            .on_toggle(move |state| Message::DesktopItemChanged(index, state))
            .into()
    }
}

fn labelled_checkbox<'a>(
    label: &'static str,
    checkbox: Checkbox<'a, Message>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    column![
        checkbox,
        text(label)
            .size(11)
            .font(iced_font(theme.font_pack().mono))
            .color(theme.palette.muted_foreground),
    ]
    .spacing(6)
    .align_x(Alignment::Center)
    .into()
}

fn style_button(style: StyleId, theme: &Theme) -> Element<'_, Message> {
    Button::text(style.as_str(), theme)
        .variant(ButtonVariant::Outline)
        .on_press(Message::Style(Labelled(style)))
        .into()
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
    let palette = theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(80)
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

fn section_label<'a>(label: &'static str, theme: &'a Theme) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(theme.font_pack().heading))
        .color(theme.palette.muted_foreground)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VariantOpt {
    Surface,
    Classic,
    Soft,
}

impl From<VariantOpt> for CheckboxVariant {
    fn from(variant: VariantOpt) -> Self {
        match variant {
            VariantOpt::Surface => Self::Surface,
            VariantOpt::Classic => Self::Classic,
            VariantOpt::Soft => Self::Soft,
        }
    }
}

impl fmt::Display for VariantOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            VariantOpt::Surface => "surface",
            VariantOpt::Classic => "classic",
            VariantOpt::Soft => "soft",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SizeOpt {
    Sm,
    Md,
    Lg,
}

impl From<SizeOpt> for CheckboxSize {
    fn from(size: SizeOpt) -> Self {
        match size {
            SizeOpt::Sm => Self::Sm,
            SizeOpt::Md => Self::Md,
            SizeOpt::Lg => Self::Lg,
        }
    }
}

impl fmt::Display for SizeOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            SizeOpt::Sm => "sm (20px)",
            SizeOpt::Md => "md (24px)",
            SizeOpt::Lg => "lg (28px)",
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

const ACCENTS: [AccentOpt; 6] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Rose),
    AccentOpt::Color(AccentColor::Violet),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

const VARIANTS: [VariantOpt; 3] = [VariantOpt::Surface, VariantOpt::Classic, VariantOpt::Soft];

const SIZES: [SizeOpt; 3] = [SizeOpt::Sm, SizeOpt::Md, SizeOpt::Lg];
