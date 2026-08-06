//! Catalog showcase for `iced-shadcn-v2::Tabs` + `shadcn-common` theme knobs.
//!
//! Run with `cargo run -p iced-shadcn-v2 --example tabs`.

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonVariant, FontHeading, FontId, FontPack, RadiusId,
    StyleId, Tabs, TabsActivationMode, TabsContent, TabsHover, TabsJustify, TabsList, TabsListLoop,
    TabsListVariant, TabsOrientation, TabsSize, TabsTrigger, TabsWrap, Theme, ThemeMode, fonts,
    iced_font,
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
    default_active: String,
    line_active: String,
    vertical_active: String,
    preview_active: String,
    size_active: String,
    state_active: String,
    composition_active: String,
    actions: u32,
    preview_variant: TabsListVariant,
    preview_orientation: TabsOrientation,
    preview_activation: TabsActivationMode,
    preview_loop: TabsListLoop,
    preview_size: TabsSize,
    preview_wrap: TabsWrap,
    preview_justify: TabsJustify,
    preview_hover: TabsHover,
    preview_full_width: bool,
    preview_root_disabled: bool,
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
    DefaultActive(String),
    LineActive(String),
    VerticalActive(String),
    PreviewActive(String),
    SizeActive(String),
    StateActive(String),
    CompositionActive(String),
    PreviewVariant(Labelled<TabsListVariant>),
    PreviewOrientation(Labelled<TabsOrientation>),
    PreviewActivation(Labelled<TabsActivationMode>),
    PreviewLoop(Labelled<TabsListLoop>),
    PreviewSize(Labelled<TabsSize>),
    PreviewWrap(Labelled<TabsWrap>),
    PreviewJustify(Labelled<TabsJustify>),
    PreviewHover(Labelled<TabsHover>),
    ToggleFullWidth,
    ToggleRootDisabled,
    Action,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            default_active: "account".to_owned(),
            line_active: "overview".to_owned(),
            vertical_active: "account".to_owned(),
            preview_active: "overview".to_owned(),
            size_active: "home".to_owned(),
            state_active: "enabled".to_owned(),
            composition_active: "activity".to_owned(),
            actions: 0,
            preview_variant: TabsListVariant::Default,
            preview_orientation: TabsOrientation::Horizontal,
            preview_activation: TabsActivationMode::Automatic,
            preview_loop: TabsListLoop::Enabled,
            preview_size: TabsSize::Default,
            preview_wrap: TabsWrap::NoWrap,
            preview_justify: TabsJustify::Start,
            preview_hover: TabsHover::Subtle,
            preview_full_width: false,
            preview_root_disabled: false,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Tabs".to_owned()
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
            Message::DefaultActive(value) => self.default_active = value,
            Message::LineActive(value) => self.line_active = value,
            Message::VerticalActive(value) => self.vertical_active = value,
            Message::PreviewActive(value) => self.preview_active = value,
            Message::SizeActive(value) => self.size_active = value,
            Message::StateActive(value) => self.state_active = value,
            Message::CompositionActive(value) => self.composition_active = value,
            Message::PreviewVariant(value) => self.preview_variant = value.0,
            Message::PreviewOrientation(value) => self.preview_orientation = value.0,
            Message::PreviewActivation(value) => self.preview_activation = value.0,
            Message::PreviewLoop(value) => self.preview_loop = value.0,
            Message::PreviewSize(value) => self.preview_size = value.0,
            Message::PreviewWrap(value) => self.preview_wrap = value.0,
            Message::PreviewJustify(value) => self.preview_justify = value.0,
            Message::PreviewHover(value) => self.preview_hover = value.0,
            Message::ToggleFullWidth => self.preview_full_width = !self.preview_full_width,
            Message::ToggleRootDisabled => {
                self.preview_root_disabled = !self.preview_root_disabled;
            }
            Message::Action => self.actions += 1,
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = theme.palette;

        let controls = column![
            section_label("Theme (shadcn-common)", p.muted_foreground, theme.font_pack()),
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
                "radius lg={:.0}px · tabs control h={:.0}/{:.0}/{:.0} · sans={} · heading={}",
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
            section_label("Interactive Tabs knobs", p.muted_foreground, theme.font_pack()),
            control_select(
                "Variant",
                &TABS_VARIANTS,
                Some(Labelled(self.preview_variant)),
                Message::PreviewVariant,
                theme,
            ),
            control_select(
                "Orientation",
                &TABS_ORIENTATIONS,
                Some(Labelled(self.preview_orientation)),
                Message::PreviewOrientation,
                theme,
            ),
            control_select(
                "Activation",
                &TABS_ACTIVATIONS,
                Some(Labelled(self.preview_activation)),
                Message::PreviewActivation,
                theme,
            ),
            control_select(
                "Loop",
                &TABS_LOOPS,
                Some(Labelled(self.preview_loop)),
                Message::PreviewLoop,
                theme,
            ),
            control_select(
                "Size",
                &TABS_SIZES,
                Some(Labelled(self.preview_size)),
                Message::PreviewSize,
                theme,
            ),
            control_select(
                "Wrap",
                &TABS_WRAPS,
                Some(Labelled(self.preview_wrap)),
                Message::PreviewWrap,
                theme,
            ),
            control_select(
                "Justify",
                &TABS_JUSTIFY,
                Some(Labelled(self.preview_justify)),
                Message::PreviewJustify,
                theme,
            ),
            control_select(
                "Hover",
                &TABS_HOVERS,
                Some(Labelled(self.preview_hover)),
                Message::PreviewHover,
                theme,
            ),
            row![
                Button::text(
                    if self.preview_full_width {
                        "Full width on"
                    } else {
                        "Full width off"
                    },
                    theme,
                )
                .variant(ButtonVariant::Outline)
                .on_press(Message::ToggleFullWidth),
                Button::text(
                    if self.preview_root_disabled {
                        "Root disabled"
                    } else {
                        "Root enabled"
                    },
                    theme,
                )
                .variant(ButtonVariant::Outline)
                .on_press(Message::ToggleRootDisabled),
            ]
            .spacing(12)
            .wrap(),
            text(format!(
                "{} · {} · activation={} · loop={} · size={} · wrap={} · justify={} · hover={} · full_width={} · root_disabled={}",
                self.preview_variant,
                self.preview_orientation,
                self.preview_activation,
                self.preview_loop,
                self.preview_size,
                self.preview_wrap,
                self.preview_justify,
                self.preview_hover,
                self.preview_full_width,
                self.preview_root_disabled,
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(p.muted_foreground),
        ]
        .spacing(8);

        let swatches = row![
            swatch("bg", p.background, p.border),
            swatch("fg", p.foreground, p.border),
            swatch("primary", p.primary, p.border),
            swatch("secondary", p.secondary, p.border),
            swatch("muted", p.muted, p.border),
            swatch("accent", p.accent, p.border),
            swatch("destructive", p.destructive, p.border),
            swatch("border", p.border, p.foreground),
        ]
        .spacing(8)
        .wrap();

        let default_tabs = Tabs::new(theme)
            .value(&self.default_active)
            .list(
                TabsList::new(theme)
                    .push(TabsTrigger::text("account", "Account", theme))
                    .push(TabsTrigger::text("password", "Password", theme))
                    .push(TabsTrigger::text("notifications", "Notifications", theme))
                    .push(TabsTrigger::text("disabled", "Disabled", theme).disabled(true)),
            )
            .push(TabsContent::new(
                "account",
                panel(
                    "Account settings",
                    "Update your profile, email address, and personal details.",
                    theme,
                ),
                theme,
            ))
            .push(TabsContent::new(
                "password",
                panel(
                    "Password",
                    "Use a strong password with a mix of letters, numbers, and symbols.",
                    theme,
                ),
                theme,
            ))
            .push(TabsContent::new(
                "notifications",
                panel(
                    "Notifications",
                    "Choose which product updates and alerts should reach you.",
                    theme,
                ),
                theme,
            ))
            .on_value_change(Message::DefaultActive);

        let line_tabs = Tabs::new(theme)
            .value(&self.line_active)
            .list(
                TabsList::new(theme)
                    .variant(TabsListVariant::Line)
                    .push(TabsTrigger::text("overview", "Overview", theme))
                    .push(TabsTrigger::text("analytics", "Analytics", theme))
                    .push(TabsTrigger::text("reports", "Reports", theme)),
            )
            .push(TabsContent::new(
                "overview",
                panel(
                    "Overview",
                    "The line variant keeps the list transparent and marks the active tab with an underline.",
                    theme,
                ),
                theme,
            ))
            .push(TabsContent::new(
                "analytics",
                panel(
                    "Analytics",
                    "The active panel follows the controlled value supplied to Tabs::value.",
                    theme,
                ),
                theme,
            ))
            .push(TabsContent::new(
                "reports",
                panel(
                    "Reports",
                    "Line tabs work with the same disabled and keyboard navigation behavior.",
                    theme,
                ),
                theme,
            ))
            .on_value_change(Message::LineActive);

        let vertical_tabs = Tabs::new(theme)
            .value(&self.vertical_active)
            .orientation(TabsOrientation::Vertical)
            .activation_mode(TabsActivationMode::Manual)
            .list_loop(TabsListLoop::Disabled)
            .spacing(12.0)
            .list(
                TabsList::new(theme)
                    .variant(TabsListVariant::Line)
                    .width(Length::Fixed(148.0))
                    .push(TabsTrigger::text("account", "Account", theme))
                    .push(TabsTrigger::text("security", "Security", theme))
                    .push(TabsTrigger::text("billing", "Billing", theme)),
            )
            .push(TabsContent::new(
                "account",
                panel(
                    "Account",
                    "Vertical roots place the list beside the active panel.",
                    theme,
                ),
                theme,
            ))
            .push(TabsContent::new(
                "security",
                panel(
                    "Security",
                    "Manual activation moves focus with arrows and commits with Enter or Space.",
                    theme,
                ),
                theme,
            ))
            .push(TabsContent::new(
                "billing",
                panel(
                    "Billing",
                    "Home and End move to the first and last enabled triggers.",
                    theme,
                ),
                theme,
            ))
            .on_value_change(Message::VerticalActive);

        let mut preview_list = TabsList::new(theme)
            .variant(self.preview_variant)
            .size(self.preview_size)
            .wrap(self.preview_wrap)
            .justify(self.preview_justify)
            .hover(self.preview_hover)
            .push(TabsTrigger::text("overview", "Overview", theme))
            .push(TabsTrigger::text("activity", "Activity", theme))
            .push(TabsTrigger::text("settings", "Settings", theme))
            .push(TabsTrigger::text("disabled", "Disabled", theme).disabled(true))
            .push(TabsTrigger::text("help", "Help", theme));

        preview_list = if self.preview_orientation.is_vertical() {
            preview_list.width(Length::Fixed(172.0))
        } else if self.preview_full_width {
            preview_list.full_width()
        } else {
            preview_list.width(Length::Fixed(420.0))
        };

        let preview_tabs = Tabs::new(theme)
            .value(&self.preview_active)
            .orientation(self.preview_orientation)
            .activation_mode(self.preview_activation)
            .list_loop(self.preview_loop)
            .disabled(self.preview_root_disabled)
            .list(preview_list)
            .push(TabsContent::new(
                "overview",
                panel(
                    "Overview",
                    "Change the controls above to exercise the public Tabs builder surface.",
                    theme,
                ),
                theme,
            ))
            .push(TabsContent::new(
                "activity",
                panel(
                    "Activity",
                    "Controlled selection, disabled triggers, wrapping, and list alignment are all live here.",
                    theme,
                ),
                theme,
            ))
            .push(TabsContent::new(
                "settings",
                panel(
                    "Settings",
                    "Try vertical orientation, manual activation, and disabled root state together.",
                    theme,
                ),
                theme,
            ))
            .push(TabsContent::new(
                "help",
                panel(
                    "Help",
                    "Arrow keys skip disabled triggers and obey the selected loop policy.",
                    theme,
                ),
                theme,
            ))
            .on_value_change(Message::PreviewActive);

        let size_gallery = row![
            container(size_demo("sm", TabsSize::Sm, &self.size_active, theme))
                .width(Length::FillPortion(1)),
            container(size_demo(
                "default",
                TabsSize::Default,
                &self.size_active,
                theme,
            ))
            .width(Length::FillPortion(1)),
            container(size_demo("lg", TabsSize::Lg, &self.size_active, theme))
                .width(Length::FillPortion(1)),
        ]
        .spacing(16)
        .wrap();

        let disabled_trigger_tabs = Tabs::new(theme)
            .value(&self.state_active)
            .list(
                TabsList::new(theme)
                    .push(TabsTrigger::text("enabled", "Enabled", theme))
                    .push(TabsTrigger::text("disabled", "Disabled", theme).disabled(true)),
            )
            .push(TabsContent::text(
                "enabled",
                "The disabled trigger is visible but cannot be selected or focused.",
                theme,
            ))
            .on_value_change(Message::StateActive);

        let disabled_root_tabs = Tabs::new(theme)
            .value("account")
            .disabled(true)
            .list(
                TabsList::new(theme)
                    .push(TabsTrigger::text("account", "Account", theme))
                    .push(TabsTrigger::text("password", "Password", theme)),
            )
            .push(TabsContent::text(
                "account",
                "Tabs::disabled(true) retains the selected visual state while disabling the root.",
                theme,
            ));

        let custom_tabs = Tabs::new(theme)
            .value(&self.composition_active)
            .list(
                TabsList::new(theme)
                    .push(
                        TabsTrigger::new(
                            "activity",
                            row![
                                text("●").color(p.primary),
                                text("Activity").font(iced_font(theme.font_pack().sans)),
                            ]
                            .spacing(6)
                            .align_y(Alignment::Center),
                            theme,
                        )
                        .style_override(|mut style, _| {
                            style.border.width = 1.0;
                            style
                        }),
                    )
                    .push(TabsTrigger::new(
                        "security",
                        row![
                            text("◆").color(p.accent),
                            text("Security").font(iced_font(theme.font_pack().sans)),
                        ]
                        .spacing(6)
                        .align_y(Alignment::Center),
                        theme,
                    )),
            )
            .push(
                TabsContent::new(
                    "activity",
                    column![
                        text("Arbitrary trigger and panel content")
                            .font(iced_font(theme.font_pack().heading))
                            .color(p.foreground),
                        text("TabsTrigger::new and TabsContent::new accept normal iced elements.")
                            .size(13)
                            .color(p.muted_foreground),
                        Button::text("Run action", theme)
                            .variant(ButtonVariant::Outline)
                            .on_press(Message::Action),
                    ]
                    .spacing(12),
                    theme,
                )
                .padding(Padding::from([12.0, 16.0]))
                .style_override(|mut style| {
                    style.border.width = 1.0;
                    style
                }),
            )
            .push(TabsContent::new(
                "security",
                column![
                    text("Compositional content")
                        .font(iced_font(theme.font_pack().heading))
                        .color(p.foreground),
                    text(format!(
                        "The action counter is shared by the example: {}.",
                        self.actions
                    ))
                    .size(13)
                    .color(p.muted_foreground),
                ]
                .spacing(12),
                theme,
            ))
            .on_value_change(Message::CompositionActive);

        let content = column![
            text("iced-shadcn-v2 Tabs")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("Controlled tabs with shadcn-svelte-compatible variants, keyboard behavior, and composition.")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            text(format!(
                "default={} · line={} · vertical={} · preview={} · actions={}",
                self.default_active,
                self.line_active,
                self.vertical_active,
                self.preview_active,
                self.actions,
            ))
            .size(14)
            .font(iced_font(theme.font_pack().sans))
            .color(p.foreground),
            controls,
            section_label("Palette", p.muted_foreground, theme.font_pack()),
            swatches,
            demo_section(
                "Default variant · controlled value",
                "The standard segmented list, disabled trigger, and arbitrary content panels.",
                default_tabs,
                theme,
            ),
            demo_section(
                "Line variant",
                "A transparent list with an active underline and the same controlled callback contract.",
                line_tabs,
                theme,
            ),
            demo_section(
                "Vertical orientation · manual activation",
                "Arrow keys move focus; Enter or Space commits the focused trigger.",
                vertical_tabs,
                theme,
            ),
            demo_section(
                "Interactive builder preview",
                "Use the knobs above to inspect orientation, activation, loop, size, wrap, alignment, hover, and disabled-root behavior.",
                preview_tabs,
                theme,
            ),
            section_label("Sizes", p.muted_foreground, theme.font_pack()),
            text("The same controlled root rendered with the compact, default, and large trigger footprints.")
                .size(13)
                .color(p.muted_foreground),
            preview_shell(size_gallery, theme),
            section_label("States / disabled behavior", p.muted_foreground, theme.font_pack()),
            preview_shell(
                column![
                    text("Disabled trigger").size(13).color(p.muted_foreground),
                    disabled_trigger_tabs,
                    text("Disabled root").size(13).color(p.muted_foreground),
                    disabled_root_tabs,
                ]
                .spacing(12),
                theme,
            ),
            demo_section(
                "Composition",
                "Text helpers are convenient, while new accepts arbitrary iced content and style overrides.",
                custom_tabs,
                theme,
            ),
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

fn demo_section<'a>(
    title: &'static str,
    description: &'static str,
    content: impl Into<Element<'a, Message>>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    column![
        section_label(title, theme.palette.muted_foreground, theme.font_pack()),
        text(description)
            .size(13)
            .font(iced_font(theme.font_pack().sans))
            .color(theme.palette.muted_foreground),
        preview_shell(content, theme),
    ]
    .spacing(8)
    .into()
}

fn preview_shell<'a>(
    content: impl Into<Element<'a, Message>>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let p = theme.palette;

    container(content)
        .width(Length::Fill)
        .padding(16)
        .style(move |_| container::Style {
            background: Some(Background::Color(p.card)),
            border: Border {
                color: p.border,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..container::Style::default()
        })
        .into()
}

fn panel<'a>(heading: &'static str, body: &'static str, theme: &'a Theme) -> Element<'a, Message> {
    let p = theme.palette;

    container(
        column![
            text(heading)
                .font(iced_font(theme.font_pack().heading))
                .color(p.card_foreground),
            text(body)
                .size(13)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
        ]
        .spacing(6),
    )
    .width(Length::Fill)
    .padding(16)
    .style(move |_| container::Style {
        background: Some(Background::Color(p.card)),
        border: Border {
            color: p.border,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..container::Style::default()
    })
    .into()
}

fn size_demo<'a>(
    label: &'static str,
    size: TabsSize,
    value: &str,
    theme: &'a Theme,
) -> Element<'a, Message> {
    column![
        text(label)
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(theme.palette.muted_foreground),
        Tabs::new(theme)
            .value(value)
            .list(
                TabsList::new(theme)
                    .size(size)
                    .push(TabsTrigger::text("home", "Home", theme))
                    .push(TabsTrigger::text("profile", "Profile", theme))
                    .push(TabsTrigger::text("files", "Files", theme)),
            )
            .push(TabsContent::text("home", "Compact size preview.", theme))
            .push(TabsContent::text(
                "profile",
                "Controlled size preview.",
                theme
            ))
            .push(TabsContent::text("files", "Large size preview.", theme))
            .on_value_change(Message::SizeActive),
    ]
    .spacing(8)
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
    let p = theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(96)
            .font(font)
            .color(p.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(220.0))
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

fn swatch<'a>(label: &'static str, fill: Color, border: Color) -> Element<'a, Message> {
    column![
        container(text(""))
            .width(36)
            .height(36)
            .style(move |_| container::Style {
                background: Some(Background::Color(fill)),
                border: Border {
                    color: border,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..container::Style::default()
            }),
        text(label).size(10).color(border),
    ]
    .spacing(4)
    .align_x(Alignment::Center)
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

macro_rules! display_tabs_label {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl fmt::Display for Labelled<$ty> {
                fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    self.0.fmt(formatter)
                }
            }
        )+
    };
}

display_tabs_label!(
    TabsActivationMode,
    TabsHover,
    TabsJustify,
    TabsListLoop,
    TabsListVariant,
    TabsOrientation,
    TabsSize,
    TabsWrap,
);

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

const TABS_VARIANTS: [Labelled<TabsListVariant>; 2] = [
    Labelled(TabsListVariant::Default),
    Labelled(TabsListVariant::Line),
];

const TABS_ORIENTATIONS: [Labelled<TabsOrientation>; 2] = [
    Labelled(TabsOrientation::Horizontal),
    Labelled(TabsOrientation::Vertical),
];

const TABS_ACTIVATIONS: [Labelled<TabsActivationMode>; 2] = [
    Labelled(TabsActivationMode::Automatic),
    Labelled(TabsActivationMode::Manual),
];

const TABS_LOOPS: [Labelled<TabsListLoop>; 2] = [
    Labelled(TabsListLoop::Disabled),
    Labelled(TabsListLoop::Enabled),
];

const TABS_SIZES: [Labelled<TabsSize>; 3] = [
    Labelled(TabsSize::Sm),
    Labelled(TabsSize::Default),
    Labelled(TabsSize::Lg),
];

const TABS_WRAPS: [Labelled<TabsWrap>; 3] = [
    Labelled(TabsWrap::NoWrap),
    Labelled(TabsWrap::Wrap),
    Labelled(TabsWrap::WrapReverse),
];

const TABS_JUSTIFY: [Labelled<TabsJustify>; 3] = [
    Labelled(TabsJustify::Start),
    Labelled(TabsJustify::Center),
    Labelled(TabsJustify::End),
];

const TABS_HOVERS: [Labelled<TabsHover>; 3] = [
    Labelled(TabsHover::None),
    Labelled(TabsHover::Subtle),
    Labelled(TabsHover::Soft),
];
