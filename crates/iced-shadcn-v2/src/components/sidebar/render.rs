//! Element builders for sidebar slots.

use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::button as iced_button;
use crate::iced_compat::widget::text::{IntoFragment, LineHeight};
use crate::iced_compat::widget::{Space, button, column, container, row, scrollable, stack, text};
use crate::iced_compat::{Background, Border, Color, Element, Length, Padding};

use shadcn_common::{
    SIDEBAR_FLOATING_PAD_PX, SIDEBAR_ICON_SIZE_PX, SIDEBAR_RAIL_WIDTH_PX, SIDEBAR_WIDTH_MOBILE_PX,
    SheetSide, SidebarCollapsible, SidebarController, SidebarSide, SidebarVariant,
};

use crate::button::{Button, ButtonSize, ButtonVariant};
use crate::fonts::iced_font;
use crate::input::{Input, InputSize};
use crate::recipes::{component_radius_px, iced_font_weight};
use crate::separator::{Separator, SeparatorOrientation};
use crate::sheet::Sheet;
use crate::skeleton::Skeleton;
use crate::theme::Theme;
use crate::tooltip::{Tooltip, TooltipSide};

use super::icon::panel_left_icon;
use super::style::{SidebarStyle, disabled_color, group_label_color, resolve_style};
use super::types::{SidebarMenuButtonSize, SidebarMenuButtonVariant, SidebarMenuSubButtonSize};
use super::{
    Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupAction, SidebarGroupContent,
    SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuAction,
    SidebarMenuBadge, SidebarMenuButton, SidebarMenuItem, SidebarMenuSkeleton, SidebarMenuSub,
    SidebarMenuSubButton, SidebarMenuSubItem, SidebarProvider, SidebarRail, SidebarSeparator,
    SidebarTrigger,
};

pub(super) fn build_provider<'a, Message: Clone + 'a>(
    provider: SidebarProvider<'a, Message>,
) -> Element<'a, Message> {
    let background = provider.theme.palette.background;
    let foreground = provider.theme.palette.foreground;
    let body = column(provider.children)
        .spacing(0.0)
        .width(Length::Fill)
        .height(Length::Fill);

    let root: Element<'a, Message> = container(body)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(background)),
            text_color: Some(foreground),
            ..container::Style::default()
        })
        .into();

    if let Some(on_viewport) = provider.on_viewport_change {
        return super::animate::ViewportProbe::new(root, on_viewport).into();
    }

    root
}

pub(super) fn build_sidebar<'a, Message: Clone + 'a>(
    sidebar: Sidebar<'a, Message>,
) -> Element<'a, Message> {
    let Sidebar {
        controller,
        theme,
        side,
        variant,
        collapsible,
        animated,
        header,
        content,
        footer,
        rail,
        children,
        on_mobile_open_change,
        style_override,
    } = sidebar;

    let style = apply_style_override(resolve_style(theme), style_override.as_deref());

    if controller.is_mobile() {
        return build_mobile_sidebar(
            theme,
            side,
            controller.open_mobile(),
            header,
            content,
            footer,
            children,
            on_mobile_open_change,
            style,
        );
    }

    if collapsible == SidebarCollapsible::None {
        let width = controller.gap_width(collapsible, variant);
        return build_desktop_panel(
            theme,
            controller,
            side,
            variant,
            collapsible,
            true,
            header,
            content,
            footer,
            rail,
            children,
            style,
            Length::Fixed(width.max(0.0)),
        );
    }

    // Always keep the animated gap widget mounted — including when offcanvas is
    // fully closed (gap → 0) — so open/close can interpolate `duration-200`.
    let panel = build_desktop_panel(
        theme,
        controller,
        side,
        variant,
        collapsible,
        controller.open(),
        header,
        content,
        footer,
        rail,
        children,
        style,
        Length::Fill,
    );

    super::animate::AnimatedGap::new(
        panel,
        controller.open(),
        collapsible,
        variant,
        controller.width_px(),
        controller.width_icon_px(),
        animated,
    )
    .into()
}

#[allow(clippy::too_many_arguments)]
fn build_mobile_sidebar<'a, Message: Clone + 'a>(
    theme: &'a Theme,
    side: SidebarSide,
    open: bool,
    header: Option<SidebarHeader<'a, Message>>,
    content: Option<SidebarContent<'a, Message>>,
    footer: Option<SidebarFooter<'a, Message>>,
    children: Vec<Element<'a, Message>>,
    on_mobile_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    style: SidebarStyle,
) -> Element<'a, Message> {
    let inner = build_inner_column(theme, header, content, footer, children, false, style);
    let sheet_side = match side {
        SidebarSide::Left => SheetSide::Left,
        SidebarSide::Right => SheetSide::Right,
        _ => SheetSide::Left,
    };

    let mut sheet = Sheet::new(
        Space::new().width(0).height(0),
        container(inner)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(style.background)),
                text_color: Some(style.foreground),
                ..container::Style::default()
            }),
        theme,
    )
    .side(sheet_side)
    .max_width(SIDEBAR_WIDTH_MOBILE_PX)
    .open(open)
    .show_close_button(false);

    if let Some(on_change) = on_mobile_open_change {
        sheet = sheet.on_open_change(on_change);
    }

    sheet.into()
}

#[allow(clippy::too_many_arguments)]
fn build_desktop_panel<'a, Message: Clone + 'a>(
    theme: &'a Theme,
    controller: &'a SidebarController,
    side: SidebarSide,
    variant: SidebarVariant,
    collapsible: SidebarCollapsible,
    open: bool,
    header: Option<SidebarHeader<'a, Message>>,
    content: Option<SidebarContent<'a, Message>>,
    footer: Option<SidebarFooter<'a, Message>>,
    rail: Option<SidebarRail<'a, Message>>,
    children: Vec<Element<'a, Message>>,
    style: SidebarStyle,
    layout_width: Length,
) -> Element<'a, Message> {
    let _ = controller;
    let icon_mode = !open && collapsible == SidebarCollapsible::Icon;
    let inner = build_inner_column(theme, header, content, footer, children, icon_mode, style);

    let radius = if variant.is_padded() {
        component_radius_px(theme, style.recipe.floating_radius)
    } else {
        0.0
    };

    // One solid panel. Edge border for `variant=sidebar` is a 1px padding reveal
    // (not a `row![content, hairline]` — that split the sidebar into two tracks
    // and let labels/chevrons paint past the divider).
    let surface: Element<'a, Message> = if variant.is_padded() {
        container(inner)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::from(SIDEBAR_FLOATING_PAD_PX))
            .clip(true)
            .style(move |_| container::Style {
                background: Some(Background::Color(style.background)),
                text_color: Some(style.foreground),
                border: Border {
                    color: style.border,
                    width: 1.0,
                    radius: radius.into(),
                },
                ..container::Style::default()
            })
            .into()
    } else if variant == SidebarVariant::Sidebar {
        let edge = match side {
            SidebarSide::Right => Padding {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 1.0,
            },
            _ => Padding {
                top: 0.0,
                right: 1.0,
                bottom: 0.0,
                left: 0.0,
            },
        };
        // Outer paints `sidebar-border`; 1px padding reveals it as border-e/s.
        container(
            container(inner)
                .width(Length::Fill)
                .height(Length::Fill)
                .clip(true)
                .style(move |_| container::Style {
                    background: Some(Background::Color(style.background)),
                    text_color: Some(style.foreground),
                    ..container::Style::default()
                }),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(edge)
        .clip(true)
        .style(move |_| container::Style {
            background: Some(Background::Color(style.border)),
            ..container::Style::default()
        })
        .into()
    } else {
        container(inner)
            .width(Length::Fill)
            .height(Length::Fill)
            .clip(true)
            .style(move |_| container::Style {
                background: Some(Background::Color(style.background)),
                text_color: Some(style.foreground),
                ..container::Style::default()
            })
            .into()
    };

    // Rail: web is `absolute w-4 -translate-x-1/2` on the trailing edge — hit
    // target only, never a layout column. Hover shows the 2px `after` line,
    // not an opaque fill (opaque fill was painting over chevrons).
    let body: Element<'a, Message> = if let Some(rail) = rail {
        let rail_hit = build_rail(rail, style);
        let rail_layer = container(rail_hit)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(match side {
                SidebarSide::Right => Horizontal::Left,
                _ => Horizontal::Right,
            });
        stack![surface, rail_layer]
            .width(layout_width)
            .height(Length::Fill)
            .into()
    } else {
        container(surface)
            .width(layout_width)
            .height(Length::Fill)
            .into()
    };

    body
}

fn build_inner_column<'a, Message: Clone + 'a>(
    theme: &'a Theme,
    header: Option<SidebarHeader<'a, Message>>,
    content: Option<SidebarContent<'a, Message>>,
    footer: Option<SidebarFooter<'a, Message>>,
    children: Vec<Element<'a, Message>>,
    icon_mode: bool,
    style: SidebarStyle,
) -> Element<'a, Message> {
    let mut items = Vec::new();
    if let Some(header) = header {
        items.push(build_header(header, icon_mode, style));
    }
    if let Some(content) = content {
        items.push(build_content(content, icon_mode, style));
    }
    items.extend(children);
    if let Some(footer) = footer {
        items.push(build_footer(footer, icon_mode, style));
    }

    let _ = theme;
    column(items)
        .spacing(0.0)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub(super) fn build_header<'a, Message: Clone + 'a>(
    header: SidebarHeader<'a, Message>,
    icon_mode: bool,
    style: SidebarStyle,
) -> Element<'a, Message> {
    let _ = icon_mode;
    let recipe = style.recipe;
    container(
        column(header.children)
            .spacing(recipe.section_gap_px)
            .width(Length::Fill),
    )
    .padding(recipe.section_pad_px)
    .width(Length::Fill)
    .into()
}

pub(super) fn build_content<'a, Message: Clone + 'a>(
    content: SidebarContent<'a, Message>,
    icon_mode: bool,
    style: SidebarStyle,
) -> Element<'a, Message> {
    let recipe = style.recipe;
    let body = column(content.children)
        .spacing(recipe.content_gap_px)
        .width(Length::Fill);

    let scrolled: Element<'a, Message> = if icon_mode {
        container(body)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    } else {
        scrollable(body)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    };

    container(scrolled)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub(super) fn build_footer<'a, Message: Clone + 'a>(
    footer: SidebarFooter<'a, Message>,
    icon_mode: bool,
    style: SidebarStyle,
) -> Element<'a, Message> {
    let _ = icon_mode;
    let recipe = style.recipe;
    container(
        column(footer.children)
            .spacing(recipe.section_gap_px)
            .width(Length::Fill),
    )
    .padding(recipe.section_pad_px)
    .width(Length::Fill)
    .into()
}

pub(super) fn build_inset<'a, Message: Clone + 'a>(
    inset: SidebarInset<'a, Message>,
) -> Element<'a, Message> {
    let style = resolve_style(inset.theme);
    let recipe = style.recipe;
    let mut items = Vec::new();
    if let Some(header) = inset.header {
        items.push(header);
    }
    items.extend(inset.children);

    let body = column(items)
        .spacing(0.0)
        .width(Length::Fill)
        .height(Length::Fill);
    let mut root = container(body)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(inset.theme.palette.background)),
            text_color: Some(inset.theme.palette.foreground),
            ..container::Style::default()
        });

    if inset.variant == SidebarVariant::Inset {
        let radius = component_radius_px(inset.theme, recipe.inset_radius);
        root = root
            .padding(recipe.inset_margin_px)
            .style(move |_| container::Style {
                background: Some(Background::Color(inset.theme.palette.background)),
                text_color: Some(inset.theme.palette.foreground),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: radius.into(),
                },
                shadow: Default::default(),
                ..container::Style::default()
            });
    }

    root.into()
}

pub(super) fn build_trigger<'a, Message: Clone + 'a>(
    trigger: SidebarTrigger<'a, Message>,
) -> Element<'a, Message> {
    let style = resolve_style(trigger.theme);
    let icon = panel_left_icon(SIDEBAR_ICON_SIZE_PX, style.foreground);
    let mut button = Button::icon(icon, trigger.theme)
        .variant(ButtonVariant::Ghost)
        .size(ButtonSize::IconSm);
    if let Some(message) = trigger.on_press {
        button = button.on_press(message);
    }
    button.into()
}

pub(super) fn build_rail<'a, Message: Clone + 'a>(
    rail: SidebarRail<'a, Message>,
    style: SidebarStyle,
) -> Element<'a, Message> {
    let _ = style;
    // Web: transparent `w-4` absolute hit target (`-translate-x-1/2` on the
    // trailing edge). Hover only tints `after:w-[2px]` — never an opaque fill.
    let content = Space::new()
        .width(Length::Fixed(SIDEBAR_RAIL_WIDTH_PX))
        .height(Length::Fill);

    let mut btn = button(content)
        .width(Length::Fixed(SIDEBAR_RAIL_WIDTH_PX))
        .height(Length::Fill)
        .padding(0)
        .style(move |_theme, _status| iced_button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: Color::TRANSPARENT,
            border: Border::default(),
            ..iced_button::Style::default()
        });

    if let Some(message) = rail.on_press {
        btn = btn.on_press(message);
    }

    btn.into()
}

pub(super) fn build_input<'a, Message: Clone + 'a>(
    input: super::SidebarInput<'a, Message>,
) -> Element<'a, Message> {
    let mut builder = Input::new(input.theme)
        .size(InputSize::Default)
        .width(Length::Fill)
        .placeholder(input.placeholder)
        .value(input.value);
    if let Some(on_input) = input.on_input {
        builder = builder.on_input(on_input);
    }
    builder.into()
}

pub(super) fn build_separator<'a, Message: 'a>(
    separator: SidebarSeparator<'a, Message>,
) -> Element<'a, Message> {
    let style = resolve_style(separator.theme);
    let recipe = style.recipe;
    container(Separator::new(separator.theme).orientation(SeparatorOrientation::Horizontal))
        .padding(Padding {
            top: 0.0,
            right: recipe.separator_margin_x_px,
            bottom: 0.0,
            left: recipe.separator_margin_x_px,
        })
        .width(Length::Fill)
        .into()
}

pub(super) fn build_group<'a, Message: Clone + 'a>(
    group: SidebarGroup<'a, Message>,
    icon_mode: bool,
) -> Element<'a, Message> {
    let style = resolve_style(group.theme);
    let recipe = style.recipe;
    let mut items = Vec::new();
    if let Some(label) = group.label {
        items.push(build_group_label(label, icon_mode));
    }
    if let Some(action) = group.action {
        items.push(build_group_action(action, icon_mode));
    }
    if let Some(content) = group.content {
        items.push(build_group_content(content, icon_mode));
    }
    items.extend(group.children);

    container(column(items).spacing(0.0).width(Length::Fill))
        .padding(Padding {
            top: recipe.group_pad_y_px,
            right: recipe.group_pad_x_px,
            bottom: recipe.group_pad_y_px,
            left: recipe.group_pad_x_px,
        })
        .width(Length::Fill)
        .into()
}

pub(super) fn build_group_label<'a, Message: Clone + 'a>(
    label: SidebarGroupLabel<'a, Message>,
    icon_mode: bool,
) -> Element<'a, Message> {
    if icon_mode {
        return Space::new().width(Length::Fill).height(0).into();
    }
    let style = resolve_style(label.theme);
    let recipe = style.recipe;
    let color = group_label_color(&style);
    let mut font = iced_font(label.theme.font_pack().sans);
    font.weight = iced_font_weight(recipe.group_label.weight);

    container(
        text(label.text)
            .size(recipe.group_label.size_px)
            .line_height(LineHeight::Absolute(
                recipe.group_label.line_height_px.into(),
            ))
            .font(font)
            .color(color),
    )
    .width(Length::Fill)
    .height(Length::Fixed(recipe.group_label_height_px))
    .padding(Padding {
        top: 0.0,
        right: recipe.group_label_pad_x_px,
        bottom: 0.0,
        left: recipe.group_label_pad_x_px,
    })
    .align_y(Vertical::Center)
    .into()
}

pub(super) fn build_group_action<'a, Message: Clone + 'a>(
    action: SidebarGroupAction<'a, Message>,
    icon_mode: bool,
) -> Element<'a, Message> {
    if icon_mode {
        return Space::new().width(0).height(0).into();
    }
    let style = resolve_style(action.theme);
    let recipe = style.recipe;
    let content = action
        .content
        .unwrap_or_else(|| text("···").size(12).color(style.foreground).into());

    let mut btn = button(
        container(content)
            .width(Length::Fixed(recipe.group_action_size_px))
            .height(Length::Fixed(recipe.group_action_size_px))
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    )
    .style(move |_theme, status| menu_chrome_button_style(style, status, false));

    if let Some(message) = action.on_press {
        btn = btn.on_press(message);
    }
    btn.into()
}

pub(super) fn build_group_content<'a, Message: Clone + 'a>(
    content: SidebarGroupContent<'a, Message>,
    icon_mode: bool,
) -> Element<'a, Message> {
    let _ = icon_mode;
    column(content.children)
        .spacing(0.0)
        .width(Length::Fill)
        .into()
}

pub(super) fn build_menu<'a, Message: Clone + 'a>(
    menu: SidebarMenu<'a, Message>,
) -> Element<'a, Message> {
    let style = resolve_style(menu.theme);
    column(menu.children)
        .spacing(style.recipe.menu_gap_px)
        .width(Length::Fill)
        .into()
}

pub(super) fn build_menu_item<'a, Message: Clone + 'a>(
    item: SidebarMenuItem<'a, Message>,
) -> Element<'a, Message> {
    container(column(item.children).spacing(0.0).width(Length::Fill))
        .width(Length::Fill)
        .into()
}

pub(super) fn build_menu_button<'a, Message: Clone + 'a>(
    button_cfg: SidebarMenuButton<'a, Message>,
) -> Element<'a, Message> {
    let style = resolve_style(button_cfg.theme);
    let recipe = style.recipe;
    let icon_mode = button_cfg.controller.is_collapsed()
        && !button_cfg.controller.is_mobile()
        && button_cfg.collapsible == SidebarCollapsible::Icon;

    let (height, type_recipe, pad) = match button_cfg.size {
        SidebarMenuButtonSize::Sm => (
            recipe.menu_button_sm_height_px,
            recipe.menu_button_sm,
            recipe.menu_button_pad_px,
        ),
        SidebarMenuButtonSize::Lg => (
            recipe.menu_button_lg_height_px,
            recipe.menu_button,
            if icon_mode {
                0.0
            } else {
                recipe.menu_button_pad_px
            },
        ),
        SidebarMenuButtonSize::Default => (
            recipe.menu_button_height_px,
            recipe.menu_button,
            recipe.menu_button_pad_px,
        ),
    };

    let radius = component_radius_px(button_cfg.theme, recipe.menu_button_radius);
    let active = button_cfg.active;
    let variant = button_cfg.variant;
    let disabled = button_cfg.disabled;

    let label = button_cfg.label.clone();
    let subtitle = button_cfg.subtitle.clone();
    let mut font = iced_font(button_cfg.theme.font_pack().sans);
    font.weight = iced_font_weight(if active {
        shadcn_common::FontWeight::Medium
    } else {
        type_recipe.weight
    });

    let text_el = text(label)
        .size(type_recipe.size_px)
        .line_height(LineHeight::Absolute(type_recipe.line_height_px.into()))
        .font(font);

    let has_leading = button_cfg.leading_icon.is_some();
    let has_trailing = button_cfg.trailing_icon.is_some() && !icon_mode;
    let mut row_children = Vec::new();
    if let Some(leading) = button_cfg.leading_icon {
        row_children.push(leading);
    }
    if !icon_mode {
        // Label takes remaining space (`flex-1` / truncate on the web). A
        // separate `Space::Fill` before the trailing icon was splitting the
        // row into two tracks and pushing chevrons past the border.
        let label_block: Element<'a, Message> = if let Some(subtitle) = subtitle {
            let mut sub_font = iced_font(button_cfg.theme.font_pack().sans);
            sub_font.weight = iced_font_weight(shadcn_common::FontWeight::Normal);
            column![
                text_el,
                text(subtitle)
                    .size(12.0)
                    .line_height(LineHeight::Absolute(16.0.into()))
                    .font(sub_font),
            ]
            .spacing(0.0)
            .width(Length::Fill)
            .into()
        } else {
            text_el.width(Length::Fill).into()
        };
        row_children.push(label_block);
    } else if !has_leading {
        // Collapsed without icon: show first character.
        let ch = button_cfg
            .label
            .chars()
            .next()
            .map(|c| c.to_string())
            .unwrap_or_default();
        row_children.push(text(ch).size(type_recipe.size_px).font(font).into());
    }
    if let Some(trailing) = button_cfg.trailing_icon
        && has_trailing
    {
        row_children.push(trailing);
    }

    let content = container(
        row(row_children)
            .spacing(recipe.menu_button_gap_px)
            .align_y(Vertical::Center)
            .width(Length::Fill),
    )
    .width(if icon_mode {
        Length::Fixed(height)
    } else {
        Length::Fill
    })
    .height(Length::Fixed(height))
    .padding(if icon_mode {
        Padding::from(pad)
    } else {
        Padding {
            top: 0.0,
            right: pad,
            bottom: 0.0,
            left: pad,
        }
    })
    .align_y(Vertical::Center)
    .align_x(if icon_mode {
        Horizontal::Center
    } else {
        Horizontal::Left
    });

    let mut btn = button(content)
        .width(if icon_mode {
            Length::Fixed(height)
        } else {
            Length::Fill
        })
        .height(Length::Fixed(height))
        .padding(0)
        .clip(true)
        .style(move |_theme, status| {
            menu_button_style(style, status, active, variant, disabled, radius)
        });

    if let Some(message) = button_cfg.on_press
        && !disabled
    {
        btn = btn.on_press(message);
    }

    let element: Element<'a, Message> = btn.into();

    if let Some(tooltip) = button_cfg.tooltip
        && button_cfg.controller.show_menu_tooltip()
    {
        return Tooltip::text(element, tooltip, button_cfg.theme)
            .side(TooltipSide::Right)
            .into();
    }

    element
}

pub(super) fn build_menu_action<'a, Message: Clone + 'a>(
    action: SidebarMenuAction<'a, Message>,
) -> Element<'a, Message> {
    if action.controller.is_collapsed() && !action.controller.is_mobile() {
        return Space::new().width(0).height(0).into();
    }
    let style = resolve_style(action.theme);
    let recipe = style.recipe;
    let content = action
        .content
        .unwrap_or_else(|| text("···").size(12).color(style.foreground).into());

    let mut btn = button(
        container(content)
            .width(Length::Fixed(recipe.menu_action_size_px))
            .height(Length::Fixed(recipe.menu_action_size_px))
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    )
    .style(move |_theme, status| menu_chrome_button_style(style, status, false));

    if let Some(message) = action.on_press {
        btn = btn.on_press(message);
    }
    btn.into()
}

pub(super) fn build_menu_badge<'a, Message: Clone + 'a>(
    badge: SidebarMenuBadge<'a, Message>,
) -> Element<'a, Message> {
    if badge.controller.is_collapsed() && !badge.controller.is_mobile() {
        return Space::new().width(0).height(0).into();
    }
    let style = resolve_style(badge.theme);
    let recipe = style.recipe;
    let mut font = iced_font(badge.theme.font_pack().sans);
    font.weight = iced_font_weight(recipe.menu_badge.weight);

    container(
        text(badge.text)
            .size(recipe.menu_badge.size_px)
            .font(font)
            .color(style.foreground),
    )
    .height(Length::Fixed(recipe.menu_action_size_px))
    .padding(Padding {
        top: 0.0,
        right: 4.0,
        bottom: 0.0,
        left: 4.0,
    })
    .align_y(Vertical::Center)
    .into()
}

pub(super) fn build_menu_skeleton<'a, Message: 'a>(
    skeleton: SidebarMenuSkeleton<'a, Message>,
) -> Element<'a, Message> {
    let style = resolve_style(skeleton.theme);
    let recipe = style.recipe;
    let mut row_children = Vec::new();
    if skeleton.show_icon {
        row_children.push(
            Skeleton::new(skeleton.theme)
                .width(Length::Fixed(SIDEBAR_ICON_SIZE_PX))
                .height(Length::Fixed(SIDEBAR_ICON_SIZE_PX))
                .into(),
        );
    }
    let text_width = 40.0 + (skeleton.width_percent.clamp(1, 100) as f32);
    row_children.push(
        Skeleton::new(skeleton.theme)
            .width(Length::Fixed(text_width))
            .height(Length::Fixed(16.0))
            .into(),
    );

    container(
        row(row_children)
            .spacing(recipe.menu_skeleton_gap_px)
            .align_y(Vertical::Center),
    )
    .width(Length::Fill)
    .height(Length::Fixed(recipe.menu_skeleton_height_px))
    .padding(Padding {
        top: 0.0,
        right: recipe.menu_skeleton_pad_x_px,
        bottom: 0.0,
        left: recipe.menu_skeleton_pad_x_px,
    })
    .align_y(Vertical::Center)
    .into()
}

pub(super) fn build_menu_sub<'a, Message: Clone + 'a>(
    sub: SidebarMenuSub<'a, Message>,
) -> Element<'a, Message> {
    if sub.controller.is_collapsed() && !sub.controller.is_mobile() {
        return Space::new().width(0).height(0).into();
    }
    let style = resolve_style(sub.theme);
    let recipe = style.recipe;

    // Web: `mx-3.5 translate-x-px gap-1 border-l px-2.5 py-0.5`
    let body = column(sub.children)
        .spacing(recipe.menu_sub_gap_px)
        .width(Length::Fill);
    let hairline = container(Space::new().width(1).height(Length::Fill))
        .width(1)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(style.border)),
            ..container::Style::default()
        });
    let padded = container(body)
        .padding(Padding {
            top: recipe.menu_sub_pad_y_px,
            right: recipe.menu_sub_pad_x_px,
            bottom: recipe.menu_sub_pad_y_px,
            left: recipe.menu_sub_pad_x_px,
        })
        .width(Length::Fill);

    container(row![hairline, padded].width(Length::Fill))
        .padding(Padding {
            top: 0.0,
            right: recipe.menu_sub_margin_x_px,
            bottom: 0.0,
            left: recipe.menu_sub_margin_x_px,
        })
        .width(Length::Fill)
        .into()
}

pub(super) fn build_menu_sub_item<'a, Message: Clone + 'a>(
    item: SidebarMenuSubItem<'a, Message>,
) -> Element<'a, Message> {
    column(item.children)
        .spacing(0.0)
        .width(Length::Fill)
        .into()
}

pub(super) fn build_menu_sub_button<'a, Message: Clone + 'a>(
    button_cfg: SidebarMenuSubButton<'a, Message>,
) -> Element<'a, Message> {
    let style = resolve_style(button_cfg.theme);
    let recipe = style.recipe;
    let type_recipe = match button_cfg.size {
        SidebarMenuSubButtonSize::Sm => recipe.menu_sub_button_sm,
        SidebarMenuSubButtonSize::Md => recipe.menu_sub_button_md,
    };
    let active = button_cfg.active;
    let disabled = button_cfg.disabled;
    let radius = component_radius_px(button_cfg.theme, recipe.menu_button_radius);
    let mut font = iced_font(button_cfg.theme.font_pack().sans);
    font.weight = iced_font_weight(type_recipe.weight);

    let content = container(
        text(button_cfg.label)
            .size(type_recipe.size_px)
            .font(font)
            .line_height(LineHeight::Absolute(type_recipe.line_height_px.into())),
    )
    .width(Length::Fill)
    .height(Length::Fixed(recipe.menu_sub_button_height_px))
    .padding(Padding {
        top: 0.0,
        right: recipe.menu_sub_button_pad_x_px,
        bottom: 0.0,
        left: recipe.menu_sub_button_pad_x_px,
    })
    .align_y(Vertical::Center);

    let mut btn = button(content).style(move |_theme, status| {
        menu_button_style(
            style,
            status,
            active,
            SidebarMenuButtonVariant::Default,
            disabled,
            radius,
        )
    });
    if let Some(message) = button_cfg.on_press
        && !disabled
    {
        btn = btn.on_press(message);
    }
    btn.into()
}

fn menu_button_style(
    style: SidebarStyle,
    status: iced_button::Status,
    active: bool,
    variant: SidebarMenuButtonVariant,
    disabled: bool,
    radius: f32,
) -> iced_button::Style {
    if disabled {
        return iced_button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: disabled_color(style.foreground),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: radius.into(),
            },
            ..iced_button::Style::default()
        };
    }

    let hovered = matches!(
        status,
        iced_button::Status::Hovered | iced_button::Status::Pressed
    );
    let emphasize = active || hovered;

    let (background, text_color, border) = match variant {
        SidebarMenuButtonVariant::Outline => {
            let bg = if emphasize {
                style.accent
            } else {
                // bg-background approximation: use theme background via transparent
                // over sidebar — web uses bg-background; we use transparent + border.
                Color::TRANSPARENT
            };
            let fg = if emphasize {
                style.accent_foreground
            } else {
                style.foreground
            };
            let border_color = if emphasize {
                style.accent
            } else {
                style.border
            };
            (
                bg,
                fg,
                Border {
                    color: border_color,
                    width: 1.0,
                    radius: radius.into(),
                },
            )
        }
        SidebarMenuButtonVariant::Default => {
            let bg = if emphasize {
                style.accent
            } else {
                Color::TRANSPARENT
            };
            let fg = if emphasize {
                style.accent_foreground
            } else {
                style.foreground
            };
            (
                bg,
                fg,
                Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: radius.into(),
                },
            )
        }
    };

    iced_button::Style {
        background: Some(Background::Color(background)),
        text_color,
        border,
        ..iced_button::Style::default()
    }
}

fn menu_chrome_button_style(
    style: SidebarStyle,
    status: iced_button::Status,
    active: bool,
) -> iced_button::Style {
    menu_button_style(
        style,
        status,
        active,
        SidebarMenuButtonVariant::Default,
        false,
        6.0,
    )
}

fn apply_style_override(
    mut style: SidebarStyle,
    override_fn: Option<&(dyn Fn(SidebarStyle) -> SidebarStyle + '_)>,
) -> SidebarStyle {
    if let Some(f) = override_fn {
        style = f(style);
    }
    style
}

// Silence unused import warning if IntoFragment is unused in some builds.
#[allow(dead_code)]
fn _into_fragment_marker<'a>(value: impl IntoFragment<'a>) -> impl IntoFragment<'a> {
    value
}
