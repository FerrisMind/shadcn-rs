//! Interactive playground for `iced-shadcn-v2::Breadcrumb`.
//!
//! Run with:
//! `cargo run -p iced-shadcn-v2 --example breadcrumb`

use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Background, Element, Length, Task};

use iced_shadcn_v2::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator, Button, ButtonSize, ButtonVariant, StyleId, Theme, fonts, iced_font,
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

struct Example {
    theme: Theme,
    packs: Vec<Theme>,
    last_action: String,
}

#[derive(Debug, Clone)]
enum Message {
    Navigate(&'static str),
    ToggleCollapsed,
}

/// Every style pack, in the order of the shadcn-svelte create customizer.
const PACKS: [StyleId; 8] = [
    StyleId::Vega,
    StyleId::Nova,
    StyleId::Maia,
    StyleId::Lyra,
    StyleId::Mira,
    StyleId::Luma,
    StyleId::Sera,
    StyleId::Rhea,
];

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light().with_style(StyleId::Vega),
            packs: PACKS
                .iter()
                .map(|pack| Theme::light().with_style(*pack))
                .collect(),
            last_action: "none".to_owned(),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Breadcrumb".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(path) => self.last_action = format!("navigate: {path}"),
            Message::ToggleCollapsed => self.last_action = "toggle collapsed steps".to_owned(),
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;

        // shadcn-svelte "Basic": link / separator / link / separator / page.
        let basic = Breadcrumb::new(theme)
            .push(BreadcrumbLink::text("Home", theme).on_press(Message::Navigate("/")))
            .push_separator()
            .push(
                BreadcrumbLink::text("Components", theme)
                    .on_press(Message::Navigate("/components")),
            )
            .push_separator()
            .push(BreadcrumbPage::text("Breadcrumb", theme));

        // shadcn-svelte "With Dropdown": the ellipsis sits inside a trigger.
        // Overlay menus are not ported yet, so the trigger just reports a press.
        let with_trigger = Breadcrumb::new(theme)
            .push(BreadcrumbLink::text("Home", theme).on_press(Message::Navigate("/")))
            .push_separator()
            .push(
                BreadcrumbItem::new(theme).push_element(
                    Button::icon(BreadcrumbEllipsis::new(theme), theme)
                        .variant(ButtonVariant::Ghost)
                        .size(ButtonSize::IconSm)
                        .on_press(Message::ToggleCollapsed),
                ),
            )
            .push_separator()
            .push(
                BreadcrumbLink::text("Components", theme)
                    .on_press(Message::Navigate("/components")),
            )
            .push_separator()
            .push(BreadcrumbPage::text("Breadcrumb", theme));

        // shadcn-svelte "With Link": a bare ellipsis marks the collapsed steps.
        let with_ellipsis = Breadcrumb::new(theme)
            .push(
                BreadcrumbLink::text("Home", theme)
                    .href("/")
                    .on_press(Message::Navigate("/")),
            )
            .push_separator()
            .push(BreadcrumbEllipsis::new(theme))
            .push_separator()
            .push(
                BreadcrumbLink::text("Components", theme)
                    .href("/components")
                    .on_press(Message::Navigate("/components")),
            )
            .push_separator()
            .push(BreadcrumbPage::text("Breadcrumb", theme));

        // `Breadcrumb.Separator` children replace the default chevron glyph.
        let slashes = Breadcrumb::new(theme)
            .push(BreadcrumbLink::text("Home", theme).on_press(Message::Navigate("/")))
            .push(BreadcrumbSeparator::text("/", theme))
            .push(BreadcrumbLink::text("Docs", theme).on_press(Message::Navigate("/docs")))
            .push(BreadcrumbSeparator::text("/", theme))
            .push(BreadcrumbPage::text("Breadcrumb", theme));

        // A long trail wraps like the web `flex-wrap` list.
        let wrapped = Breadcrumb::new(theme)
            .width(Length::Fixed(260.0))
            .list(
                BreadcrumbList::new(theme)
                    .width(Length::Fill)
                    .line_spacing(4.0),
            )
            .push(BreadcrumbLink::text("Home", theme).on_press(Message::Navigate("/")))
            .push_separator()
            .push(BreadcrumbLink::text("Documentation", theme).on_press(Message::Navigate("/docs")))
            .push_separator()
            .push(
                BreadcrumbLink::text("Components", theme)
                    .on_press(Message::Navigate("/components")),
            )
            .push_separator()
            .push(BreadcrumbPage::text("Breadcrumb", theme));

        let content = column![
            text("iced-shadcn-v2 Breadcrumb")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("shadcn-svelte parity: root, list, item, link, page, separator, ellipsis")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            section("Basic", basic, theme),
            section("With dropdown trigger", with_trigger, theme),
            section("With ellipsis and hrefs", with_ellipsis, theme),
            section("Custom separator", slashes, theme),
            section("Wrapping trail (260 px)", wrapped, theme),
            section("Style packs", style_packs(&self.packs), theme),
            text(format!("last action: {}", self.last_action))
                .size(13)
                .font(iced_font(theme.font_pack().mono))
                .color(palette.muted_foreground),
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
}

/// One trail per style pack, so gaps, type scale, and casing can be compared.
fn style_packs(packs: &[Theme]) -> Element<'_, Message> {
    let rows = packs
        .iter()
        .map(|theme| {
            row![
                container(
                    text(format!("{:?}", theme.style_id()))
                        .size(13)
                        .font(iced_font(theme.font_pack().mono))
                        .color(theme.palette.muted_foreground)
                )
                .width(Length::Fixed(64.0)),
                Breadcrumb::new(theme)
                    .push(BreadcrumbLink::text("Home", theme).on_press(Message::Navigate("/")))
                    .push_separator()
                    .push(BreadcrumbEllipsis::new(theme))
                    .push_separator()
                    .push(BreadcrumbPage::text("Breadcrumb", theme)),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .into()
        })
        .collect::<Vec<_>>();

    column(rows).spacing(10).into()
}

fn section<'a>(
    label: &'static str,
    trail: impl Into<Element<'a, Message>>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    column![
        text(label)
            .size(17)
            .font(iced_font(theme.font_pack().heading))
            .color(theme.palette.muted_foreground),
        trail.into(),
    ]
    .spacing(8)
    .align_x(Alignment::Start)
    .into()
}
