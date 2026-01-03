//! Sidebar component - collapsible navigation rail.

use crate::theme::Theme;
use egui::RichText;
use egui::{
    Align, Align2, Color32, CornerRadius, FontId, Frame, Id, Layout, Margin, Response, Sense,
    Stroke, StrokeKind, Ui, Vec2, WidgetText, pos2, vec2,
};

const DEFAULT_EXPANDED_WIDTH: f32 = 240.0;
const DEFAULT_COLLAPSED_WIDTH: f32 = 64.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

pub struct SidebarProviderProps<'a> {
    pub id_source: Id,
    pub open: &'a mut bool,
    pub default_open: bool,
    pub expanded_width: f32,
    pub collapsed_width: f32,
    pub animate: bool,
    pub on_open_change: Option<&'a mut dyn FnMut(bool)>,
}

impl<'a> SidebarProviderProps<'a> {
    pub fn new(id_source: Id, open: &'a mut bool) -> Self {
        Self {
            id_source,
            open,
            default_open: true,
            expanded_width: DEFAULT_EXPANDED_WIDTH,
            collapsed_width: DEFAULT_COLLAPSED_WIDTH,
            animate: true,
            on_open_change: None,
        }
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn expanded_width(mut self, width: f32) -> Self {
        self.expanded_width = width;
        self
    }

    pub fn collapsed_width(mut self, width: f32) -> Self {
        self.collapsed_width = width;
        self
    }

    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    pub fn on_open_change(mut self, cb: &'a mut dyn FnMut(bool)) -> Self {
        self.on_open_change = Some(cb);
        self
    }
}

pub struct SidebarContext<'a> {
    pub id_source: Id,
    pub open: &'a mut bool,
    pub expanded_width: f32,
    pub collapsed_width: f32,
    pub animate: bool,
    on_open_change: Option<&'a mut dyn FnMut(bool)>,
}

impl<'a> SidebarContext<'a> {
    pub fn is_collapsed(&self) -> bool {
        !*self.open
    }

    pub fn set_open(&mut self, open: bool) {
        if *self.open == open {
            return;
        }
        *self.open = open;
        if let Some(cb) = self.on_open_change.as_mut() {
            cb(open);
        }
    }

    pub fn toggle(&mut self) {
        let next = !*self.open;
        self.set_open(next);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SidebarProps {
    pub side: SidebarSide,
    pub padding: Margin,
    pub border: bool,
}

impl SidebarProps {
    pub fn new() -> Self {
        Self {
            side: SidebarSide::Left,
            padding: Margin::same(0),
            border: true,
        }
    }

    pub fn side(mut self, side: SidebarSide) -> Self {
        self.side = side;
        self
    }

    pub fn padding(mut self, padding: Margin) -> Self {
        self.padding = padding;
        self
    }

    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }
}

impl Default for SidebarProps {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SidebarResponse<R> {
    pub response: Response,
    pub inner: R,
    pub width: f32,
}

fn apply_default_open(ui: &Ui, props: &mut SidebarProviderProps<'_>) {
    let init_id = props.id_source.with("default-open-initialized");
    let initialized = ui
        .ctx()
        .data(|d| d.get_temp::<bool>(init_id))
        .unwrap_or(false);
    if !initialized {
        *props.open = props.default_open;
        ui.ctx().data_mut(|d| d.insert_temp(init_id, true));
    }
}

pub fn sidebar_provider<R>(
    ui: &mut Ui,
    mut props: SidebarProviderProps<'_>,
    add_contents: impl FnOnce(&mut Ui, &mut SidebarContext) -> R,
) -> R {
    apply_default_open(ui, &mut props);

    let mut ctx = SidebarContext {
        id_source: props.id_source,
        open: props.open,
        expanded_width: props.expanded_width,
        collapsed_width: props.collapsed_width,
        animate: props.animate,
        on_open_change: props.on_open_change,
    };

    add_contents(ui, &mut ctx)
}

pub fn sidebar<R>(
    ui: &mut Ui,
    theme: &Theme,
    ctx: &mut SidebarContext,
    props: SidebarProps,
    add_contents: impl FnOnce(&mut Ui, &mut SidebarContext) -> R,
) -> SidebarResponse<R> {
    let open = *ctx.open;
    let anim_t = if ctx.animate {
        ui.ctx()
            .animate_bool(ctx.id_source.with("sidebar-open"), open)
    } else if open {
        1.0
    } else {
        0.0
    };

    let width = ctx.collapsed_width + (ctx.expanded_width - ctx.collapsed_width) * anim_t;
    let height = ui.available_height().max(1.0);

    let palette = &theme.palette;
    let rounding = CornerRadius::same(theme.radius.r2.round() as u8);
    let border = Stroke::new(1.0, palette.sidebar_border);

    let inner = ui.allocate_ui_with_layout(
        Vec2::new(width, height),
        Layout::top_down(Align::Min),
        |sidebar_ui| {
            sidebar_ui.set_min_height(height);
            sidebar_ui.set_min_width(width);
            sidebar_ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

            let frame = Frame::default()
                .fill(palette.sidebar)
                .stroke(if props.border { border } else { Stroke::NONE })
                .corner_radius(rounding)
                .inner_margin(props.padding);

            frame
                .show(sidebar_ui, |content_ui| {
                    content_ui.visuals_mut().override_text_color = Some(palette.sidebar_foreground);
                    add_contents(content_ui, ctx)
                })
                .inner
        },
    );

    SidebarResponse {
        response: inner.response,
        inner: inner.inner,
        width,
    }
}

pub fn sidebar_trigger(
    ui: &mut Ui,
    theme: &Theme,
    ctx: &mut SidebarContext,
    label: impl Into<WidgetText>,
) -> Response {
    let response = crate::Button::new(label)
        .variant(crate::ButtonVariant::Ghost)
        .size(crate::ButtonSize::Sm)
        .show(ui, theme);
    if response.clicked() {
        ctx.toggle();
    }
    response
}

pub fn sidebar_header<R>(
    ui: &mut Ui,
    ctx: &SidebarContext,
    add_contents: impl FnOnce(&mut Ui, &SidebarContext) -> R,
) -> R {
    sidebar_section(ui, ctx, Margin::symmetric(12, 12), add_contents)
}

pub fn sidebar_content<R>(
    ui: &mut Ui,
    ctx: &SidebarContext,
    add_contents: impl FnOnce(&mut Ui, &SidebarContext) -> R,
) -> R {
    sidebar_section(ui, ctx, Margin::symmetric(12, 8), add_contents)
}

pub fn sidebar_footer<R>(
    ui: &mut Ui,
    ctx: &SidebarContext,
    add_contents: impl FnOnce(&mut Ui, &SidebarContext) -> R,
) -> R {
    sidebar_section(ui, ctx, Margin::symmetric(12, 12), add_contents)
}

fn sidebar_section<R>(
    ui: &mut Ui,
    ctx: &SidebarContext,
    padding: Margin,
    add_contents: impl FnOnce(&mut Ui, &SidebarContext) -> R,
) -> R {
    Frame::default()
        .inner_margin(padding)
        .show(ui, |section_ui| add_contents(section_ui, ctx))
        .inner
}

#[derive(Clone, Copy, Debug)]
pub struct SidebarGroupProps {
    pub spacing: f32,
}

impl SidebarGroupProps {
    pub fn new() -> Self {
        Self { spacing: 8.0 }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl Default for SidebarGroupProps {
    fn default() -> Self {
        Self::new()
    }
}

pub fn sidebar_group<R>(
    ui: &mut Ui,
    _ctx: &SidebarContext,
    props: SidebarGroupProps,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    ui.vertical(|group_ui| {
        group_ui.spacing_mut().item_spacing = vec2(0.0, props.spacing);
        add_contents(group_ui)
    })
    .inner
}

#[derive(Clone, Debug)]
pub struct SidebarGroupLabelProps {
    pub text: WidgetText,
    pub show_when_collapsed: bool,
}

impl SidebarGroupLabelProps {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            show_when_collapsed: false,
        }
    }

    pub fn show_when_collapsed(mut self, show: bool) -> Self {
        self.show_when_collapsed = show;
        self
    }
}

pub fn sidebar_group_label(
    ui: &mut Ui,
    theme: &Theme,
    ctx: &SidebarContext,
    props: SidebarGroupLabelProps,
) -> Response {
    if ctx.is_collapsed() && !props.show_when_collapsed {
        return ui.allocate_response(Vec2::ZERO, Sense::hover());
    }

    let text = RichText::new(props.text.text())
        .color(theme.palette.sidebar_foreground.gamma_multiply(0.6))
        .size(11.0);
    ui.add(egui::Label::new(text).sense(Sense::hover()))
}

pub fn sidebar_group_content<R>(
    ui: &mut Ui,
    _ctx: &SidebarContext,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    ui.vertical(|content_ui| {
        content_ui.spacing_mut().item_spacing = vec2(0.0, 4.0);
        add_contents(content_ui)
    })
    .inner
}

pub fn sidebar_menu<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    ui.vertical(|menu_ui| {
        menu_ui.spacing_mut().item_spacing = vec2(0.0, 4.0);
        add_contents(menu_ui)
    })
    .inner
}

pub fn sidebar_menu_item<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    ui.horizontal(|item_ui| {
        item_ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        add_contents(item_ui)
    })
    .inner
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarMenuButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl SidebarMenuButtonSize {
    fn height(self) -> f32 {
        match self {
            SidebarMenuButtonSize::Sm => 28.0,
            SidebarMenuButtonSize::Md => 32.0,
            SidebarMenuButtonSize::Lg => 40.0,
        }
    }

    fn padding(self) -> Margin {
        match self {
            SidebarMenuButtonSize::Sm => Margin::symmetric(10, 6),
            SidebarMenuButtonSize::Md => Margin::symmetric(12, 8),
            SidebarMenuButtonSize::Lg => Margin::symmetric(12, 10),
        }
    }

    fn text_size(self) -> f32 {
        match self {
            SidebarMenuButtonSize::Sm => 12.0,
            SidebarMenuButtonSize::Md => 13.0,
            SidebarMenuButtonSize::Lg => 14.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SidebarMenuButtonProps {
    pub label: WidgetText,
    pub size: SidebarMenuButtonSize,
    pub active: bool,
    pub disabled: bool,
    pub show_label_when_collapsed: bool,
}

impl SidebarMenuButtonProps {
    pub fn new(label: impl Into<WidgetText>) -> Self {
        Self {
            label: label.into(),
            size: SidebarMenuButtonSize::Md,
            active: false,
            disabled: false,
            show_label_when_collapsed: true,
        }
    }

    pub fn size(mut self, size: SidebarMenuButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn show_label_when_collapsed(mut self, show: bool) -> Self {
        self.show_label_when_collapsed = show;
        self
    }
}

pub fn sidebar_menu_button(
    ui: &mut Ui,
    theme: &Theme,
    ctx: &SidebarContext,
    props: SidebarMenuButtonProps,
) -> Response {
    let collapsed = ctx.is_collapsed();
    let height = props.size.height();
    let padding = props.size.padding();
    let desired = vec2(ui.available_width(), height);
    let sense = if props.disabled {
        Sense::hover()
    } else {
        Sense::click()
    };

    let (rect, response) = ui.allocate_exact_size(desired, sense);
    let hovered = response.hovered() || response.has_focus();

    let palette = &theme.palette;
    let bg = if props.active || hovered {
        palette.sidebar_accent
    } else {
        Color32::TRANSPARENT
    };
    if bg != Color32::TRANSPARENT {
        ui.painter()
            .rect_filled(rect, CornerRadius::same(theme.radius.r2.round() as u8), bg);
    }

    let text_color = if props.active || hovered {
        palette.sidebar_accent_foreground
    } else {
        palette.sidebar_foreground
    };

    let label_text = if collapsed && !props.show_label_when_collapsed {
        let mut short = props.label.text().to_string();
        short.truncate(1);
        WidgetText::from(short)
    } else {
        props.label
    };

    let align = if collapsed && !props.show_label_when_collapsed {
        Align2::CENTER_CENTER
    } else {
        Align2::LEFT_CENTER
    };
    let pos = if collapsed && !props.show_label_when_collapsed {
        rect.center()
    } else {
        pos2(rect.left() + padding.left as f32, rect.center().y)
    };

    ui.painter().text(
        pos,
        align,
        label_text.text(),
        FontId::proportional(props.size.text_size()),
        text_color,
    );

    if response.has_focus() && !props.disabled {
        let focus_color = palette.sidebar_ring;
        ui.painter().rect_stroke(
            rect,
            CornerRadius::same(theme.radius.r2.round() as u8),
            theme.focus.stroke(focus_color),
            StrokeKind::Outside,
        );
    }

    if props.disabled {
        response
    } else {
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    }
}
