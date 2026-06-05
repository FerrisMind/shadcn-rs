use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::text;
use iced::advanced::text::Paragraph;
use iced::advanced::text::paragraph;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::alignment;
use iced::border::Border;
use iced::border::Radius;
use iced::keyboard;
use iced::mouse;
use iced::time::Instant;
use iced::touch;
use iced::window;
use iced::{
    Background, Color, Element, Event, Font, Length, Padding, Pixels, Point, Rectangle, Shadow,
    Size, Vector,
};
use lucide_icons::Icon as LucideIcon;

use crate::button::ButtonRadius;
use crate::overlay::{keyboard as overlay_keyboard, positioning};
use crate::profiling::profile_span;
use crate::theme::Theme as ShadcnTheme;
use crate::tokens::{
    AccentColor, accent_color, accent_foreground, accent_high, accent_soft, accent_text, is_dark,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectSize {
    Size1,
    Size2,
    Size3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriggerVariant {
    Classic,
    Surface,
    Soft,
    Ghost,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContentVariant {
    Solid,
    Soft,
}

#[derive(Clone, Copy, Debug)]
pub struct SelectProps {
    pub size: SelectSize,
    pub variant: TriggerVariant,
    pub content_variant: ContentVariant,
    pub color: AccentColor,
    pub content_color: Option<AccentColor>,
    pub radius: Option<ButtonRadius>,
    pub high_contrast: bool,
    pub disabled: bool,
}

impl Default for SelectProps {
    fn default() -> Self {
        Self {
            size: SelectSize::Size2,
            variant: TriggerVariant::Surface,
            content_variant: ContentVariant::Solid,
            color: AccentColor::Gray,
            content_color: None,
            radius: None,
            high_contrast: false,
            disabled: false,
        }
    }
}

impl SelectProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: SelectSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: TriggerVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn content_variant(mut self, content_variant: ContentVariant) -> Self {
        self.content_variant = content_variant;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = color;
        self
    }

    pub fn content_color(mut self, color: AccentColor) -> Self {
        self.content_color = Some(color);
        self
    }

    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug)]
pub struct SelectItem<T> {
    pub value: T,
    pub label: String,
    pub disabled: bool,
}

impl<T> SelectItem<T> {
    pub fn new(value: T, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug)]
pub struct SelectGroup<T> {
    pub label: Option<String>,
    pub items: Vec<SelectItem<T>>,
}

impl<T> SelectGroup<T> {
    pub fn new(label: impl Into<String>, items: Vec<SelectItem<T>>) -> Self {
        Self {
            label: Some(label.into()),
            items,
        }
    }

    pub fn unnamed(items: Vec<SelectItem<T>>) -> Self {
        Self { label: None, items }
    }
}

#[derive(Clone, Debug)]
pub enum SelectEntry<T> {
    Item(SelectItem<T>),
    Label(String),
    Separator,
    Group(SelectGroup<T>),
}

impl<T> SelectEntry<T> {
    pub fn label(label: impl Into<String>) -> Self {
        Self::Label(label.into())
    }

    pub fn separator() -> Self {
        Self::Separator
    }
}

impl<T> From<SelectItem<T>> for SelectEntry<T> {
    fn from(item: SelectItem<T>) -> Self {
        Self::Item(item)
    }
}

impl<T> From<SelectGroup<T>> for SelectEntry<T> {
    fn from(group: SelectGroup<T>) -> Self {
        Self::Group(group)
    }
}

#[derive(Clone, Copy, Debug)]
struct SelectMetrics {
    trigger_height: f32,
    trigger_padding_x: f32,
    trigger_padding_y: f32,
    text_size: u32,
    chevron_size: f32,
    check_size: f32,
    icon_gap: f32,
    item_height: f32,
    item_padding_left: f32,
    item_padding_right: f32,
    label_text_size: u32,
    label_padding_y: f32,
    content_padding: f32,
    separator_height: f32,
    separator_margin_y: f32,
    indicator_size: f32,
    scroll_button_height: f32,
}

impl SelectSize {
    fn metrics(self) -> SelectMetrics {
        match self {
            SelectSize::Size1 => SelectMetrics {
                trigger_height: 32.0,
                trigger_padding_x: 12.0,
                trigger_padding_y: 6.0,
                text_size: 14,
                chevron_size: 16.0,
                check_size: 16.0,
                icon_gap: 8.0,
                item_height: 32.0,
                item_padding_left: 8.0,
                item_padding_right: 32.0,
                label_text_size: 12,
                label_padding_y: 6.0,
                content_padding: 4.0,
                separator_height: 1.0,
                separator_margin_y: 4.0,
                indicator_size: 14.0,
                scroll_button_height: 24.0,
            },
            SelectSize::Size2 => SelectMetrics {
                trigger_height: 36.0,
                trigger_padding_x: 12.0,
                trigger_padding_y: 8.0,
                text_size: 14,
                chevron_size: 16.0,
                check_size: 16.0,
                icon_gap: 8.0,
                item_height: 32.0,
                item_padding_left: 8.0,
                item_padding_right: 32.0,
                label_text_size: 12,
                label_padding_y: 6.0,
                content_padding: 4.0,
                separator_height: 1.0,
                separator_margin_y: 4.0,
                indicator_size: 14.0,
                scroll_button_height: 24.0,
            },
            SelectSize::Size3 => SelectMetrics {
                trigger_height: 40.0,
                trigger_padding_x: 14.0,
                trigger_padding_y: 10.0,
                text_size: 16,
                chevron_size: 16.0,
                check_size: 16.0,
                icon_gap: 10.0,
                item_height: 36.0,
                item_padding_left: 8.0,
                item_padding_right: 32.0,
                label_text_size: 12,
                label_padding_y: 6.0,
                content_padding: 4.0,
                separator_height: 1.0,
                separator_margin_y: 4.0,
                indicator_size: 14.0,
                scroll_button_height: 24.0,
            },
        }
    }
}

#[derive(Clone, Copy)]
enum SelectEntries<'a, T> {
    Plain(&'a [T]),
    Entries(&'a [SelectEntry<T>]),
}

pub fn select<'a, Message: Clone + 'a, T, F>(
    options: &'a [T],
    selected: Option<T>,
    placeholder: &'a str,
    on_select: F,
    props: SelectProps,
    theme: &ShadcnTheme,
) -> Select<'a, T, Message>
where
    T: Clone + PartialEq + ToString + 'a,
    F: Fn(T) -> Message + 'a,
{
    Select::new(
        SelectEntries::Plain(options),
        selected,
        placeholder,
        on_select,
        props,
        theme,
    )
}

pub fn select_entries<'a, Message: Clone + 'a, T, F>(
    entries: &'a [SelectEntry<T>],
    selected: Option<T>,
    placeholder: &'a str,
    on_select: F,
    props: SelectProps,
    theme: &ShadcnTheme,
) -> Select<'a, T, Message>
where
    T: Clone + PartialEq + ToString + 'a,
    F: Fn(T) -> Message + 'a,
{
    Select::new(
        SelectEntries::Entries(entries),
        selected,
        placeholder,
        on_select,
        props,
        theme,
    )
}

pub struct Select<'a, T, Message> {
    entries: SelectEntries<'a, T>,
    selected: Option<T>,
    placeholder: Option<String>,
    on_select: Box<dyn Fn(T) -> Message + 'a>,
    props: SelectProps,
    theme: ShadcnTheme,
    width: Length,
    menu_height: Length,
    on_open: Option<Message>,
    on_close: Option<Message>,
    font: Option<Font>,
    text_line_height: text::LineHeight,
    text_shaping: text::Shaping,
    last_status: Option<SelectStatus>,
}

impl<'a, T, Message> Select<'a, T, Message>
where
    T: Clone + PartialEq + ToString + 'a,
    Message: Clone + 'a,
{
    fn new<F>(
        entries: SelectEntries<'a, T>,
        selected: Option<T>,
        placeholder: &'a str,
        on_select: F,
        props: SelectProps,
        theme: &ShadcnTheme,
    ) -> Self
    where
        F: Fn(T) -> Message + 'a,
    {
        Self {
            entries,
            selected,
            placeholder: Some(placeholder.to_string()),
            on_select: Box::new(on_select),
            props,
            theme: theme.clone(),
            width: Length::Shrink,
            menu_height: Length::Shrink,
            on_open: None,
            on_close: None,
            font: None,
            text_line_height: text::LineHeight::default(),
            text_shaping: text::Shaping::Basic,
            last_status: None,
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn menu_height(mut self, height: impl Into<Length>) -> Self {
        self.menu_height = height.into();
        self
    }

    pub fn on_open(mut self, on_open: Message) -> Self {
        self.on_open = Some(on_open);
        self
    }

    pub fn on_close(mut self, on_close: Message) -> Self {
        self.on_close = Some(on_close);
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn text_line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.text_line_height = line_height.into();
        self
    }

    pub fn text_shaping(mut self, shaping: text::Shaping) -> Self {
        self.text_shaping = shaping;
        self
    }
}

impl<Message, AppTheme, Renderer, T> Widget<Message, AppTheme, Renderer> for Select<'_, T, Message>
where
    T: Clone + PartialEq + ToString,
    Message: Clone,
    Renderer: renderer::Renderer + text::Renderer<Font = Font>,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<SelectState<Renderer::Paragraph>>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(SelectState::<Renderer::Paragraph>::new())
    }

    fn size(&self) -> Size<Length> {
        let metrics = self.props.size.metrics();
        Size {
            width: self.width,
            height: Length::Fixed(metrics.trigger_height),
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let state = tree
            .state
            .downcast_mut::<SelectState<Renderer::Paragraph>>();
        let metrics = self.props.size.metrics();
        let font = self.font.unwrap_or_else(|| renderer.default_font());
        let text_size: Pixels = metrics.text_size.into();
        let labels = collect_item_labels(self.entries.clone());

        state
            .option_labels
            .resize_with(labels.len(), Default::default);

        let option_text = text::Text {
            content: "",
            bounds: Size::new(
                f32::INFINITY,
                self.text_line_height.to_absolute(text_size).into(),
            ),
            size: text_size,
            line_height: self.text_line_height,
            font,
            align_x: text::Alignment::Default,
            align_y: alignment::Vertical::Center,
            shaping: self.text_shaping,
            wrapping: text::Wrapping::default(),
        };

        for (label, paragraph) in labels.iter().zip(state.option_labels.iter_mut()) {
            let _ = paragraph.update(text::Text {
                content: label,
                ..option_text
            });
        }

        if let Some(placeholder) = &self.placeholder {
            let _ = state.placeholder.update(text::Text {
                content: placeholder,
                ..option_text
            });
        }

        let max_width = match self.width {
            Length::Shrink => {
                let labels_width = state.option_labels.iter().fold(0.0, |width, paragraph| {
                    f32::max(width, paragraph.min_width())
                });
                labels_width.max(
                    self.placeholder
                        .as_ref()
                        .map(|_| state.placeholder.min_width())
                        .unwrap_or(0.0),
                )
            }
            _ => 0.0,
        };

        let padding = trigger_padding(metrics);
        let intrinsic = Size::new(
            max_width + padding.left + padding.right,
            metrics.trigger_height,
        );

        layout::Node::new(limits.resolve(
            self.width,
            Length::Fixed(metrics.trigger_height),
            intrinsic,
        ))
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree
            .state
            .downcast_mut::<SelectState<Renderer::Paragraph>>();
        let disabled = self.props.disabled;

        if disabled && state.is_open {
            state.is_open = false;
        }

        if !disabled {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    if state.is_open {
                        let over_trigger = cursor.is_over(layout.bounds());
                        let over_menu = state
                            .menu
                            .menu_bounds
                            .map(|bounds| cursor.is_over(bounds))
                            .unwrap_or(false);

                        if over_trigger || !over_menu {
                            state.is_open = false;
                            if let Some(on_close) = &self.on_close {
                                shell.publish(on_close.clone());
                            }
                            shell.capture_event();
                        }
                    } else if cursor.is_over(layout.bounds()) {
                        state.is_open = true;
                        state.hovered_row = selected_row_index(
                            self.entries.clone(),
                            self.selected.as_ref(),
                            self.props.size.metrics(),
                        );
                        if let Some(on_open) = &self.on_open {
                            shell.publish(on_open.clone());
                        }
                        shell.capture_event();
                    }
                }
                Event::Keyboard(keyboard::Event::KeyPressed { .. })
                    if matches!(
                        overlay_keyboard::command(event),
                        Some(overlay_keyboard::OverlayCommand::Close)
                    ) =>
                {
                    state.is_open = false;
                    if let Some(on_close) = &self.on_close {
                        shell.publish(on_close.clone());
                    }
                    shell.capture_event();
                }
                Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                    state.keyboard_modifiers = *modifiers;
                }
                _ => {}
            }
        }

        if !state.is_open {
            state.menu.menu_bounds = None;
        }

        let status = if disabled {
            SelectStatus::Disabled
        } else {
            let is_hovered = cursor.is_over(layout.bounds());
            if state.is_open {
                SelectStatus::Opened { is_hovered }
            } else if is_hovered {
                SelectStatus::Hovered
            } else {
                SelectStatus::Active
            }
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.last_status = Some(status);
        } else if self.last_status.is_some_and(|last| last != status) {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if self.props.disabled {
            return mouse::Interaction::default();
        }

        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &AppTheme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let metrics = self.props.size.metrics();
        let font = self.font.unwrap_or_else(|| renderer.default_font());
        let status = self.last_status.unwrap_or(SelectStatus::Active);
        let trigger_style = select_trigger_style(&self.theme, self.props, status);
        let padding = trigger_padding(metrics);

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: trigger_style.border,
                shadow: trigger_style.shadow,
                ..renderer::Quad::default()
            },
            trigger_style.background,
        );

        let chevron = LucideIcon::ChevronDown;
        let mut chevron_bytes = [0; 4];
        let chevron_icon = char::from(chevron).encode_utf8(&mut chevron_bytes);
        renderer.fill_text(
            text::Text {
                content: chevron_icon.to_string(),
                size: metrics.chevron_size.into(),
                line_height: text::LineHeight::Absolute(metrics.chevron_size.into()),
                font: Font::with_name("lucide"),
                bounds: Size::new(metrics.chevron_size, metrics.trigger_height),
                align_x: text::Alignment::Center,
                align_y: alignment::Vertical::Center,
                shaping: text::Shaping::Basic,
                wrapping: text::Wrapping::default(),
            },
            Point::new(
                bounds.x + bounds.width - metrics.trigger_padding_x - metrics.chevron_size / 2.0,
                bounds.center_y(),
            ),
            trigger_style.handle_color,
            *viewport,
        );

        let selected_label = selected_label(self.entries.clone(), self.selected.as_ref());
        let label = selected_label.or_else(|| self.placeholder.clone());

        if let Some(label) = label {
            let text_size: Pixels = metrics.text_size.into();
            let text_bounds = Size::new(
                (bounds.width - padding.left - padding.right).max(0.0),
                self.text_line_height.to_absolute(text_size).into(),
            );

            let current_color = if self.selected.is_some() {
                trigger_style.text_color
            } else {
                trigger_style.placeholder_color
            };

            if let Some((part1, part2)) = label.split_once("  ") {
                let temp_paragraph = Renderer::Paragraph::with_text(text::Text {
                    content: part1,
                    size: text_size,
                    line_height: self.text_line_height,
                    font,
                    bounds: Size::new(f32::INFINITY, text_bounds.height),
                    align_x: text::Alignment::Default,
                    align_y: alignment::Vertical::Center,
                    shaping: self.text_shaping,
                    wrapping: text::Wrapping::default(),
                });
                let font_width = temp_paragraph.min_width();

                let text_color_muted = apply_opacity(current_color, 0.6);

                renderer.fill_text(
                    text::Text {
                        content: part1.to_string(),
                        size: text_size,
                        line_height: self.text_line_height,
                        font,
                        bounds: text_bounds,
                        align_x: text::Alignment::Default,
                        align_y: alignment::Vertical::Center,
                        shaping: self.text_shaping,
                        wrapping: text::Wrapping::default(),
                    },
                    Point::new(bounds.x + padding.left, bounds.center_y()),
                    current_color,
                    *viewport,
                );

                renderer.fill_text(
                    text::Text {
                        content: part2.to_string(),
                        size: (text_size.0 * 0.85).into(),
                        line_height: self.text_line_height,
                        font,
                        bounds: Size::new(
                            (text_bounds.width - font_width - 8.0).max(0.0),
                            text_bounds.height,
                        ),
                        align_x: text::Alignment::Default,
                        align_y: alignment::Vertical::Center,
                        shaping: self.text_shaping,
                        wrapping: text::Wrapping::default(),
                    },
                    Point::new(
                        bounds.x + padding.left + font_width + 8.0,
                        bounds.center_y(),
                    ),
                    text_color_muted,
                    *viewport,
                );
            } else {
                renderer.fill_text(
                    text::Text {
                        content: label,
                        size: text_size,
                        line_height: self.text_line_height,
                        font,
                        bounds: text_bounds,
                        align_x: text::Alignment::Default,
                        align_y: alignment::Vertical::Center,
                        shaping: self.text_shaping,
                        wrapping: text::Wrapping::default(),
                    },
                    Point::new(bounds.x + padding.left, bounds.center_y()),
                    current_color,
                    *viewport,
                );
            }
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<iced::overlay::Element<'b, Message, AppTheme, Renderer>> {
        let state = tree
            .state
            .downcast_mut::<SelectState<Renderer::Paragraph>>();
        let font = self.font.unwrap_or_else(|| renderer.default_font());

        if state.is_open {
            let bounds = layout.bounds();
            let on_select = &self.on_select;
            let props = self.props;
            let metrics = self.props.size.metrics();

            let menu = SelectMenu {
                state: &mut state.menu,
                entries: self.entries.clone(),
                hovered_row: &mut state.hovered_row,
                selected: self.selected.clone(),
                on_selected: Box::new(|option| {
                    state.is_open = false;
                    (on_select)(option)
                }),
                props,
                metrics,
                font,
                text_shaping: self.text_shaping,
                theme: self.theme.clone(),
                width: bounds.width,
            };

            Some(menu.overlay::<Renderer, AppTheme>(
                layout.position() + translation,
                *viewport,
                bounds.width,
                bounds.height,
                self.menu_height,
            ))
        } else {
            None
        }
    }
}

impl<'a, T, Message, AppTheme, Renderer> From<Select<'a, T, Message>>
    for Element<'a, Message, AppTheme, Renderer>
where
    T: Clone + PartialEq + ToString + 'a,
    Message: Clone + 'a,
    Renderer: renderer::Renderer + text::Renderer<Font = Font> + 'a,
{
    fn from(select: Select<'a, T, Message>) -> Element<'a, Message, AppTheme, Renderer> {
        Element::new(select)
    }
}

#[derive(Debug)]
struct SelectState<P: text::Paragraph> {
    menu: SelectMenuState,
    keyboard_modifiers: keyboard::Modifiers,
    is_open: bool,
    hovered_row: Option<usize>,
    option_labels: Vec<paragraph::Plain<P>>,
    placeholder: paragraph::Plain<P>,
}

impl<P: text::Paragraph> SelectState<P> {
    fn new() -> Self {
        Self {
            menu: SelectMenuState::default(),
            keyboard_modifiers: keyboard::Modifiers::default(),
            is_open: false,
            hovered_row: None,
            option_labels: Vec::new(),
            placeholder: paragraph::Plain::default(),
        }
    }
}

impl<P: text::Paragraph> Default for SelectState<P> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct SelectMenuState {
    tree: Tree,
    menu_bounds: Option<Rectangle>,
}

impl SelectMenuState {
    fn new() -> Self {
        Self {
            tree: Tree::empty(),
            menu_bounds: None,
        }
    }
}

impl Default for SelectMenuState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectStatus {
    Active,
    Hovered,
    Opened { is_hovered: bool },
    Disabled,
}

struct SelectMenu<'a, T, Message> {
    state: &'a mut SelectMenuState,
    entries: SelectEntries<'a, T>,
    hovered_row: &'a mut Option<usize>,
    selected: Option<T>,
    on_selected: Box<dyn FnMut(T) -> Message + 'a>,
    props: SelectProps,
    metrics: SelectMetrics,
    font: Font,
    text_shaping: text::Shaping,
    theme: ShadcnTheme,
    width: f32,
}

impl<'a, T, Message> SelectMenu<'a, T, Message>
where
    T: Clone + PartialEq + ToString,
    Message: Clone + 'a,
{
    fn overlay<Renderer, AppTheme>(
        self,
        position: Point,
        viewport: Rectangle,
        target_width: f32,
        target_height: f32,
        menu_height: Length,
    ) -> iced::overlay::Element<'a, Message, AppTheme, Renderer>
    where
        Renderer: renderer::Renderer + text::Renderer<Font = Font>,
    {
        iced::overlay::Element::new(Box::new(SelectOverlay::new::<Renderer, AppTheme>(
            position,
            viewport,
            self,
            target_width,
            target_height,
            menu_height,
        )))
    }
}

struct SelectOverlay<'a, T, Message> {
    position: Point,
    viewport: Rectangle,
    tree: &'a mut Tree,
    list: SelectList<'a, T, Message>,
    width: f32,
    target_width: f32,
    target_height: f32,
    props: SelectProps,
    theme: ShadcnTheme,
    menu_bounds: &'a mut Option<Rectangle>,
}

impl<'a, T, Message> SelectOverlay<'a, T, Message>
where
    T: Clone + PartialEq + ToString,
    Message: Clone + 'a,
{
    fn new<Renderer, AppTheme>(
        position: Point,
        viewport: Rectangle,
        menu: SelectMenu<'a, T, Message>,
        target_width: f32,
        target_height: f32,
        menu_height: Length,
    ) -> Self
    where
        Renderer: renderer::Renderer + text::Renderer<Font = Font>,
    {
        let _profile = profile_span("select.overlay.new");

        let SelectMenu {
            state,
            entries,
            hovered_row,
            selected,
            on_selected,
            props,
            metrics,
            font,
            text_shaping,
            width,
            theme,
        } = menu;
        let width = width.max(128.0);
        let menu_bounds = &mut state.menu_bounds;
        let rows = build_rows(entries.clone(), selected.as_ref(), metrics);
        let content_height = rows.iter().map(|row| row.height).sum::<f32>();

        let list = SelectList {
            rows,
            content_height,
            hovered_row,
            on_selected,
            props,
            metrics,
            font,
            text_shaping,
            menu_height,
            theme: theme.clone(),
        };

        state
            .tree
            .diff::<Message, AppTheme, Renderer>(&list as &dyn Widget<_, _, _>);

        Self {
            position,
            viewport,
            tree: &mut state.tree,
            list,
            width,
            target_width,
            target_height,
            props,
            theme,
            menu_bounds,
        }
    }
}

impl<Message, AppTheme, Renderer, T> iced::advanced::Overlay<Message, AppTheme, Renderer>
    for SelectOverlay<'_, T, Message>
where
    T: Clone + PartialEq + ToString,
    Message: Clone,
    Renderer: renderer::Renderer + text::Renderer<Font = Font>,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let _profile = profile_span("select.overlay.layout");

        let space_below = bounds.height - (self.position.y + self.target_height);
        let space_above = self.position.y;
        let gap = 4.0;

        let max_available = if space_below > space_above {
            space_below - gap
        } else {
            space_above - gap
        };

        let max_height = max_available.clamp(0.0, 384.0);

        let limits = layout::Limits::new(
            Size::ZERO,
            Size::new(bounds.width - self.position.x, max_height),
        )
        .width(self.width);

        let node = <SelectList<'_, T, Message> as Widget<Message, AppTheme, Renderer>>::layout(
            &mut self.list,
            self.tree,
            renderer,
            &limits,
        );
        let size = node.size();
        let placement = positioning::place_overlay_centered(
            self.position,
            Size::new(self.target_width, self.target_height),
            size,
            bounds,
            gap,
        );
        let node = node.move_to(placement.position);

        *self.menu_bounds = Some(node.bounds());

        node
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let _profile = profile_span("select.overlay.update");

        let bounds = layout.bounds();
        <SelectList<'_, T, Message> as Widget<Message, AppTheme, Renderer>>::update(
            &mut self.list,
            self.tree,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &bounds,
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        <SelectList<'_, T, Message> as Widget<Message, AppTheme, Renderer>>::mouse_interaction(
            &self.list,
            self.tree,
            layout,
            cursor,
            &self.viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _theme: &AppTheme,
        defaults: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let _profile = profile_span("select.overlay.draw");

        let bounds = layout.bounds();
        let style = select_menu_style(&self.theme, self.props);

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: style.border,
                shadow: style.shadow,
                ..renderer::Quad::default()
            },
            style.background,
        );

        self.list.draw(
            self.tree, renderer, _theme, defaults, layout, cursor, &bounds,
        );
    }
}

struct SelectList<'a, T, Message> {
    rows: Vec<Row<T>>,
    content_height: f32,
    hovered_row: &'a mut Option<usize>,
    on_selected: Box<dyn FnMut(T) -> Message + 'a>,
    props: SelectProps,
    metrics: SelectMetrics,
    font: Font,
    text_shaping: text::Shaping,
    menu_height: Length,
    theme: ShadcnTheme,
}

impl<Message, AppTheme, Renderer, T> Widget<Message, AppTheme, Renderer>
    for SelectList<'_, T, Message>
where
    T: Clone + PartialEq + ToString,
    Message: Clone,
    Renderer: renderer::Renderer + text::Renderer<Font = Font>,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<SelectListState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(SelectListState::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Shrink)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let intrinsic = Size::new(
            0.0,
            self.content_height + self.metrics.content_padding * 2.0,
        );

        layout::Node::new(limits.resolve(Length::Fill, self.menu_height, intrinsic))
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let _profile = profile_span("select.list.update");

        let bounds = layout.bounds();
        let state = tree.state.downcast_mut::<SelectListState>();
        let mut layout_info = list_layout(
            bounds,
            &self.rows,
            self.content_height,
            self.metrics,
            state.scroll_offset,
        );

        if layout_info.max_scroll > 0.0 {
            state.scroll_offset = state.scroll_offset.clamp(0.0, layout_info.max_scroll);
        } else {
            state.scroll_offset = 0.0;
        }
        layout_info = list_layout(
            bounds,
            &self.rows,
            self.content_height,
            self.metrics,
            state.scroll_offset,
        );

        match event {
            Event::Mouse(mouse::Event::WheelScrolled { delta }) if cursor.is_over(bounds) => {
                let scroll_delta = match delta {
                    mouse::ScrollDelta::Lines { y, .. } => -y * self.metrics.item_height,
                    mouse::ScrollDelta::Pixels { y, .. } => -y,
                };
                if scroll_delta.abs() > f32::EPSILON {
                    state.scroll_offset =
                        (state.scroll_offset + scroll_delta).clamp(0.0, layout_info.max_scroll);
                    shell.request_redraw();
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let mut direction = 0;
                if let Some(top_bounds) = layout_info.top_button_bounds
                    && cursor.is_over(top_bounds)
                {
                    direction = -1;
                } else if let Some(bottom_bounds) = layout_info.bottom_button_bounds
                    && cursor.is_over(bottom_bounds)
                {
                    direction = 1;
                }
                if state.auto_scroll_direction != direction {
                    state.auto_scroll_direction = direction;
                    state.last_auto_scroll = None;
                    shell.request_redraw();
                }

                if let Some(cursor_position) = cursor.position_in(layout_info.list_bounds) {
                    let y = cursor_position.y + state.scroll_offset;
                    if let Some(index) = row_at(&self.rows, y) {
                        if matches!(&self.rows[index].kind, RowKind::Item { disabled: true, .. }) {
                            let old_hovered = *self.hovered_row;
                            *self.hovered_row = None;
                            if old_hovered.is_some() {
                                shell.request_redraw();
                            }
                        } else {
                            let old_hovered = *self.hovered_row;
                            *self.hovered_row = Some(index);
                            if old_hovered != Some(index) {
                                shell.request_redraw();
                            }
                        }
                    } else {
                        let old_hovered = *self.hovered_row;
                        *self.hovered_row = None;
                        if old_hovered.is_some() {
                            shell.request_redraw();
                        }
                    }
                } else {
                    let old_hovered = *self.hovered_row;
                    *self.hovered_row = None;
                    if old_hovered.is_some() {
                        shell.request_redraw();
                    }
                }
            }
            Event::Window(window::Event::RedrawRequested(now)) => {
                let mut direction = state.auto_scroll_direction;
                if direction == 0 {
                    if let Some(top_bounds) = layout_info.top_button_bounds
                        && cursor.is_over(top_bounds)
                    {
                        direction = -1;
                    } else if let Some(bottom_bounds) = layout_info.bottom_button_bounds
                        && cursor.is_over(bottom_bounds)
                    {
                        direction = 1;
                    }
                }

                if direction != 0 && layout_info.max_scroll > 0.0 {
                    let previous = state.scroll_offset;
                    let now = *now;
                    let elapsed = state
                        .last_auto_scroll
                        .map(|last| now.saturating_duration_since(last).as_secs_f32())
                        .unwrap_or(0.0);
                    let base_step = self.metrics.item_height * 0.3;
                    let speed = self.metrics.item_height * 6.0;
                    let step = (elapsed * speed).max(base_step);
                    let delta = if direction < 0 { -step } else { step };

                    state.scroll_offset =
                        (state.scroll_offset + delta).clamp(0.0, layout_info.max_scroll);
                    state.last_auto_scroll = Some(now);

                    if (state.scroll_offset - previous).abs() > f32::EPSILON {
                        shell.request_redraw();
                    }
                } else {
                    state.last_auto_scroll = None;
                }

                state.is_hovered = Some(cursor.is_over(bounds));
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(bounds) = layout_info.top_button_bounds
                    && cursor.is_over(bounds)
                {
                    state.scroll_offset = (state.scroll_offset - self.metrics.item_height)
                        .clamp(0.0, layout_info.max_scroll);
                    shell.request_redraw();
                    shell.capture_event();
                    return;
                }

                if let Some(bounds) = layout_info.bottom_button_bounds
                    && cursor.is_over(bounds)
                {
                    state.scroll_offset = (state.scroll_offset + self.metrics.item_height)
                        .clamp(0.0, layout_info.max_scroll);
                    shell.request_redraw();
                    shell.capture_event();
                    return;
                }

                if let Some(cursor_position) = cursor.position_in(layout_info.list_bounds) {
                    let y = cursor_position.y + state.scroll_offset;
                    if let Some(index) = row_at(&self.rows, y)
                        && let RowKind::Item {
                            value,
                            disabled: false,
                            ..
                        } = &self.rows[index].kind
                    {
                        shell.publish((self.on_selected)(value.clone()));
                        shell.capture_event();
                    }
                } else if let Some(index) = *self.hovered_row
                    && let RowKind::Item {
                        value,
                        disabled: false,
                        ..
                    } = &self.rows[index].kind
                {
                    shell.publish((self.on_selected)(value.clone()));
                    shell.capture_event();
                }
            }
            _ => {}
        }

        if !matches!(event, Event::Window(window::Event::RedrawRequested(_)))
            && state
                .is_hovered
                .is_some_and(|is_hovered| is_hovered != cursor.is_over(bounds))
        {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        if cursor.is_over(bounds) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &AppTheme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let _profile = profile_span("select.list.draw");

        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let state = tree.state.downcast_ref::<SelectListState>();
        let initial_layout = list_layout(
            bounds,
            &self.rows,
            self.content_height,
            self.metrics,
            state.scroll_offset,
        );
        let scroll_offset = state.scroll_offset.clamp(0.0, initial_layout.max_scroll);
        let layout_info = list_layout(
            bounds,
            &self.rows,
            self.content_height,
            self.metrics,
            scroll_offset,
        );
        let menu_style = select_menu_style(&self.theme, self.props);
        let scroll_overlay_background = select_scroll_overlay_background(&self.theme, self.props);
        let item_radius = item_radius(&self.theme);
        let disabled_text_color = apply_opacity(menu_style.text_color, 0.5);
        let content_clip_bounds = select_content_clip_bounds(&layout_info);

        let mut y = layout_info.list_bounds.y - scroll_offset;
        for (index, row) in self.rows.iter().enumerate() {
            let row_bounds = Rectangle {
                x: layout_info.list_bounds.x,
                y,
                width: layout_info.list_bounds.width,
                height: row.height,
            };
            y += row.height;

            if row_bounds.y > content_clip_bounds.y + content_clip_bounds.height {
                break;
            }
            if row_bounds.y + row_bounds.height < content_clip_bounds.y {
                continue;
            }

            let row_bounds = intersect_rectangles(row_bounds, content_clip_bounds);

            match &row.kind {
                RowKind::Item {
                    label,
                    disabled,
                    selected,
                    ..
                } => {
                    let is_hovered = *self.hovered_row == Some(index);

                    // Draw background for selected item
                    if *selected && !*disabled {
                        renderer.fill_quad(
                            renderer::Quad {
                                bounds: row_bounds,
                                border: Border {
                                    radius: item_radius.into(),
                                    width: 0.0,
                                    color: Color::TRANSPARENT,
                                },
                                ..renderer::Quad::default()
                            },
                            menu_style.selected_background,
                        );
                    }

                    // Draw hover background (on top of selected if both)
                    if is_hovered && !*disabled {
                        renderer.fill_quad(
                            renderer::Quad {
                                bounds: row_bounds,
                                border: Border {
                                    radius: item_radius.into(),
                                    width: 0.0,
                                    color: Color::TRANSPARENT,
                                },
                                ..renderer::Quad::default()
                            },
                            menu_style.hover_background,
                        );
                    }

                    let text_color = if *disabled {
                        disabled_text_color
                    } else if is_hovered {
                        menu_style.hover_text_color
                    } else if *selected {
                        menu_style.selected_text_color
                    } else {
                        menu_style.text_color
                    };

                    if let Some((part1, part2)) = label.split_once("  ") {
                        let temp_paragraph = Renderer::Paragraph::with_text(text::Text {
                            content: part1,
                            size: self.metrics.text_size.into(),
                            line_height: text::LineHeight::Absolute(
                                (self.metrics.text_size as f32 + 6.0).into(),
                            ),
                            font: self.font,
                            bounds: Size::new(f32::INFINITY, row_bounds.height),
                            align_x: text::Alignment::Default,
                            align_y: alignment::Vertical::Center,
                            shaping: self.text_shaping,
                            wrapping: text::Wrapping::default(),
                        });
                        let font_width = temp_paragraph.min_width();
                        let text_color_muted =
                            apply_opacity(text_color, if is_hovered { 0.7 } else { 0.5 });

                        renderer.fill_text(
                            text::Text {
                                content: part1.to_string(),
                                size: self.metrics.text_size.into(),
                                line_height: text::LineHeight::Absolute(
                                    (self.metrics.text_size as f32 + 6.0).into(),
                                ),
                                font: self.font,
                                bounds: Size::new(
                                    (row_bounds.width
                                        - self.metrics.item_padding_left
                                        - self.metrics.item_padding_right)
                                        .max(0.0),
                                    row_bounds.height,
                                ),
                                align_x: text::Alignment::Default,
                                align_y: alignment::Vertical::Center,
                                shaping: self.text_shaping,
                                wrapping: text::Wrapping::default(),
                            },
                            Point::new(
                                row_bounds.x + self.metrics.item_padding_left,
                                row_bounds.center_y(),
                            ),
                            text_color,
                            content_clip_bounds,
                        );

                        renderer.fill_text(
                            text::Text {
                                content: part2.to_string(),
                                size: (self.metrics.text_size as f32 * 0.85).into(),
                                line_height: text::LineHeight::Absolute(
                                    (self.metrics.text_size as f32 + 6.0).into(),
                                ),
                                font: self.font,
                                bounds: Size::new(
                                    (row_bounds.width
                                        - self.metrics.item_padding_left
                                        - self.metrics.item_padding_right
                                        - font_width
                                        - 8.0)
                                        .max(0.0),
                                    row_bounds.height,
                                ),
                                align_x: text::Alignment::Default,
                                align_y: alignment::Vertical::Center,
                                shaping: self.text_shaping,
                                wrapping: text::Wrapping::default(),
                            },
                            Point::new(
                                row_bounds.x + self.metrics.item_padding_left + font_width + 8.0,
                                row_bounds.center_y(),
                            ),
                            text_color_muted,
                            content_clip_bounds,
                        );
                    } else {
                        renderer.fill_text(
                            text::Text {
                                content: label.clone(),
                                size: self.metrics.text_size.into(),
                                line_height: text::LineHeight::Absolute(
                                    (self.metrics.text_size as f32 + 6.0).into(),
                                ),
                                font: self.font,
                                bounds: Size::new(
                                    (row_bounds.width
                                        - self.metrics.item_padding_left
                                        - self.metrics.item_padding_right)
                                        .max(0.0),
                                    row_bounds.height,
                                ),
                                align_x: text::Alignment::Default,
                                align_y: alignment::Vertical::Center,
                                shaping: self.text_shaping,
                                wrapping: text::Wrapping::default(),
                            },
                            Point::new(
                                row_bounds.x + self.metrics.item_padding_left,
                                row_bounds.center_y(),
                            ),
                            text_color,
                            content_clip_bounds,
                        );
                    }

                    if *selected {
                        let icon_color = if *disabled {
                            disabled_text_color
                        } else if is_hovered {
                            menu_style.hover_text_color
                        } else if *selected {
                            menu_style.selected_text_color
                        } else {
                            menu_style.text_color
                        };
                        let icon_center = Point::new(
                            row_bounds.x + row_bounds.width - self.metrics.item_padding_right / 2.0,
                            row_bounds.center_y(),
                        );
                        let mut icon_bytes = [0; 4];
                        let check_icon = char::from(LucideIcon::Check).encode_utf8(&mut icon_bytes);
                        renderer.fill_text(
                            text::Text {
                                content: check_icon.to_string(),
                                size: self.metrics.check_size.into(),
                                line_height: text::LineHeight::Absolute(
                                    self.metrics.check_size.into(),
                                ),
                                font: Font::with_name("lucide"),
                                bounds: Size::new(
                                    self.metrics.indicator_size,
                                    self.metrics.indicator_size,
                                ),
                                align_x: text::Alignment::Center,
                                align_y: alignment::Vertical::Center,
                                shaping: text::Shaping::Basic,
                                wrapping: text::Wrapping::default(),
                            },
                            icon_center,
                            icon_color,
                            content_clip_bounds,
                        );
                    }
                }
                RowKind::Label { text: label } => {
                    renderer.fill_text(
                        text::Text {
                            content: label.clone(),
                            size: self.metrics.label_text_size.into(),
                            line_height: text::LineHeight::Absolute(
                                (self.metrics.label_text_size as f32 + 4.0).into(),
                            ),
                            font: self.font,
                            bounds: Size::new(row_bounds.width, row_bounds.height),
                            align_x: text::Alignment::Default,
                            align_y: alignment::Vertical::Center,
                            shaping: self.text_shaping,
                            wrapping: text::Wrapping::default(),
                        },
                        Point::new(
                            row_bounds.x + self.metrics.item_padding_left,
                            row_bounds.center_y(),
                        ),
                        menu_style.muted_text_color,
                        content_clip_bounds,
                    );
                }
                RowKind::Separator => {
                    let line_bounds = Rectangle {
                        x: layout_info.list_bounds.x,
                        y: row_bounds.center_y() - self.metrics.separator_height / 2.0,
                        width: layout_info.list_bounds.width,
                        height: self.metrics.separator_height,
                    };
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: line_bounds,
                            border: Border::default(),
                            ..renderer::Quad::default()
                        },
                        Background::Color(menu_style.separator_color),
                    );
                }
            }
        }

        if layout_info.show_buttons {
            let up_enabled = scroll_offset > 0.0;
            let down_enabled = scroll_offset < layout_info.max_scroll;
            let overlay_clip_bounds = intersect_rectangles(*viewport, layout_info.list_bounds);

            renderer.with_layer(layout_info.list_bounds, |renderer| {
                if let Some(bounds) = layout_info.top_button_bounds {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds,
                            border: Border {
                                color: Color::TRANSPARENT,
                                width: 0.0,
                                radius: Radius {
                                    top_left: menu_style.border.radius.top_left,
                                    top_right: menu_style.border.radius.top_right,
                                    ..Radius::default()
                                },
                            },
                            shadow: Shadow::default(),
                            ..renderer::Quad::default()
                        },
                        scroll_overlay_background,
                    );

                    let color = if up_enabled {
                        menu_style.muted_text_color
                    } else {
                        apply_opacity(menu_style.muted_text_color, 0.4)
                    };
                    let mut icon_bytes = [0; 4];
                    let up_icon = char::from(LucideIcon::ChevronUp).encode_utf8(&mut icon_bytes);
                    renderer.fill_text(
                        text::Text {
                            content: up_icon.to_string(),
                            size: self.metrics.chevron_size.into(),
                            line_height: text::LineHeight::Absolute(
                                self.metrics.chevron_size.into(),
                            ),
                            font: Font::with_name("lucide"),
                            bounds: Size::new(bounds.width, bounds.height),
                            align_x: text::Alignment::Center,
                            align_y: alignment::Vertical::Center,
                            shaping: text::Shaping::Basic,
                            wrapping: text::Wrapping::default(),
                        },
                        Point::new(bounds.center_x(), bounds.center_y()),
                        color,
                        overlay_clip_bounds,
                    );
                }

                if let Some(bounds) = layout_info.bottom_button_bounds {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds,
                            border: Border {
                                color: Color::TRANSPARENT,
                                width: 0.0,
                                radius: Radius {
                                    bottom_right: menu_style.border.radius.bottom_right,
                                    bottom_left: menu_style.border.radius.bottom_left,
                                    ..Radius::default()
                                },
                            },
                            shadow: Shadow::default(),
                            ..renderer::Quad::default()
                        },
                        scroll_overlay_background,
                    );

                    let color = if down_enabled {
                        menu_style.muted_text_color
                    } else {
                        apply_opacity(menu_style.muted_text_color, 0.4)
                    };
                    let mut icon_bytes = [0; 4];
                    let down_icon =
                        char::from(LucideIcon::ChevronDown).encode_utf8(&mut icon_bytes);
                    renderer.fill_text(
                        text::Text {
                            content: down_icon.to_string(),
                            size: self.metrics.chevron_size.into(),
                            line_height: text::LineHeight::Absolute(
                                self.metrics.chevron_size.into(),
                            ),
                            font: Font::with_name("lucide"),
                            bounds: Size::new(bounds.width, bounds.height),
                            align_x: text::Alignment::Center,
                            align_y: alignment::Vertical::Center,
                            shaping: text::Shaping::Basic,
                            wrapping: text::Wrapping::default(),
                        },
                        Point::new(bounds.center_x(), bounds.center_y()),
                        color,
                        overlay_clip_bounds,
                    );
                }
            });
        }
    }
}

#[derive(Debug, Default)]
struct SelectListState {
    scroll_offset: f32,
    is_hovered: Option<bool>,
    auto_scroll_direction: i8,
    last_auto_scroll: Option<Instant>,
}

struct Row<T> {
    kind: RowKind<T>,
    height: f32,
}

enum RowKind<T> {
    Item {
        value: T,
        label: String,
        disabled: bool,
        selected: bool,
    },
    Label {
        text: String,
    },
    Separator,
}

struct ListLayout {
    show_buttons: bool,
    list_bounds: Rectangle,
    top_button_bounds: Option<Rectangle>,
    bottom_button_bounds: Option<Rectangle>,
    max_scroll: f32,
}

fn select_radius(theme: &ShadcnTheme, props: SelectProps) -> f32 {
    match props.radius {
        Some(ButtonRadius::None) => 0.0,
        Some(ButtonRadius::Small) => theme.radius.sm,
        Some(ButtonRadius::Medium) => theme.radius.md,
        Some(ButtonRadius::Large) => theme.radius.lg,
        Some(ButtonRadius::Full) => 9999.0,
        None => theme.radius.md,
    }
}

fn trigger_padding(metrics: SelectMetrics) -> Padding {
    Padding {
        top: metrics.trigger_padding_y,
        bottom: metrics.trigger_padding_y,
        left: metrics.trigger_padding_x,
        right: metrics.trigger_padding_x + metrics.chevron_size + metrics.icon_gap,
    }
}

fn collect_item_labels<T: ToString>(entries: SelectEntries<'_, T>) -> Vec<String> {
    let mut labels = Vec::new();
    collect_labels(entries, &mut labels);
    labels
}

fn collect_labels<T: ToString>(entries: SelectEntries<'_, T>, labels: &mut Vec<String>) {
    match entries {
        SelectEntries::Plain(options) => {
            for option in options {
                labels.push(option.to_string());
            }
        }
        SelectEntries::Entries(entries) => {
            for entry in entries {
                match entry {
                    SelectEntry::Item(item) => labels.push(item.label.clone()),
                    SelectEntry::Group(group) => {
                        for item in &group.items {
                            labels.push(item.label.clone());
                        }
                    }
                    SelectEntry::Label(_) | SelectEntry::Separator => {}
                }
            }
        }
    }
}

fn selected_label<T: PartialEq + ToString>(
    entries: SelectEntries<'_, T>,
    selected: Option<&T>,
) -> Option<String> {
    let selected = selected?;
    match entries {
        SelectEntries::Plain(_) => Some(selected.to_string()),
        SelectEntries::Entries(entries) => {
            for entry in entries {
                match entry {
                    SelectEntry::Item(item) => {
                        if &item.value == selected {
                            return Some(item.label.clone());
                        }
                    }
                    SelectEntry::Group(group) => {
                        for item in &group.items {
                            if &item.value == selected {
                                return Some(item.label.clone());
                            }
                        }
                    }
                    SelectEntry::Label(_) | SelectEntry::Separator => {}
                }
            }
            None
        }
    }
}

fn build_rows<T: Clone + PartialEq + ToString>(
    entries: SelectEntries<'_, T>,
    selected: Option<&T>,
    metrics: SelectMetrics,
) -> Vec<Row<T>> {
    let mut rows = Vec::new();
    let label_line_height = metrics.label_text_size as f32 + 4.0;
    match entries {
        SelectEntries::Plain(options) => {
            for option in options {
                let is_selected = selected == Some(option);
                rows.push(Row {
                    kind: RowKind::Item {
                        value: option.clone(),
                        label: option.to_string(),
                        disabled: false,
                        selected: is_selected,
                    },
                    height: metrics.item_height,
                });
            }
        }
        SelectEntries::Entries(entries) => {
            for entry in entries {
                match entry {
                    SelectEntry::Item(item) => {
                        let is_selected = selected == Some(&item.value);
                        rows.push(Row {
                            kind: RowKind::Item {
                                value: item.value.clone(),
                                label: item.label.clone(),
                                disabled: item.disabled,
                                selected: is_selected,
                            },
                            height: metrics.item_height,
                        });
                    }
                    SelectEntry::Label(label) => rows.push(Row {
                        kind: RowKind::Label {
                            text: label.clone(),
                        },
                        height: metrics.label_padding_y * 2.0 + label_line_height,
                    }),
                    SelectEntry::Separator => rows.push(Row {
                        kind: RowKind::Separator,
                        height: metrics.separator_margin_y * 2.0 + metrics.separator_height,
                    }),
                    SelectEntry::Group(group) => {
                        if let Some(label) = &group.label {
                            rows.push(Row {
                                kind: RowKind::Label {
                                    text: label.clone(),
                                },
                                height: metrics.label_padding_y * 2.0 + label_line_height,
                            });
                        }
                        for item in &group.items {
                            let is_selected = selected == Some(&item.value);
                            rows.push(Row {
                                kind: RowKind::Item {
                                    value: item.value.clone(),
                                    label: item.label.clone(),
                                    disabled: item.disabled,
                                    selected: is_selected,
                                },
                                height: metrics.item_height,
                            });
                        }
                    }
                }
            }
        }
    }
    rows
}

fn selected_row_index<T: Clone + PartialEq + ToString>(
    entries: SelectEntries<'_, T>,
    selected: Option<&T>,
    metrics: SelectMetrics,
) -> Option<usize> {
    let rows = build_rows(entries, selected, metrics);
    rows.iter()
        .position(|row| matches!(&row.kind, RowKind::Item { selected: true, .. }))
}

fn row_at<T>(rows: &[Row<T>], y: f32) -> Option<usize> {
    let mut offset = 0.0;
    for (index, row) in rows.iter().enumerate() {
        if y >= offset && y < offset + row.height {
            return Some(index);
        }
        offset += row.height;
    }
    None
}

fn list_layout<T>(
    bounds: Rectangle,
    _rows: &[Row<T>],
    content_height: f32,
    metrics: SelectMetrics,
    scroll_offset: f32,
) -> ListLayout {
    let padding = metrics.content_padding;
    let available_height = (bounds.height - padding * 2.0).max(0.0);
    let can_scroll = content_height > available_height;
    let requested_offset = scroll_offset.max(0.0);

    let mut show_top_button = false;
    let mut show_bottom_button = false;
    let list_height = available_height;
    let mut max_scroll = 0.0;

    if can_scroll {
        show_top_button = requested_offset > 0.0;
        show_bottom_button = true;

        for _ in 0..2 {
            max_scroll = (content_height - list_height).max(0.0);
            show_bottom_button = requested_offset < max_scroll;
        }
    }

    let list_bounds = Rectangle {
        x: bounds.x + padding,
        y: bounds.y + padding,
        width: (bounds.width - padding * 2.0).max(0.0),
        height: list_height,
    };

    let top_button_bounds = show_top_button.then_some(Rectangle {
        x: list_bounds.x,
        y: list_bounds.y,
        width: list_bounds.width,
        height: metrics.scroll_button_height,
    });

    let bottom_button_bounds = show_bottom_button.then_some(Rectangle {
        x: list_bounds.x,
        y: list_bounds.y + list_bounds.height - metrics.scroll_button_height,
        width: list_bounds.width,
        height: metrics.scroll_button_height,
    });

    ListLayout {
        show_buttons: show_top_button || show_bottom_button,
        list_bounds,
        top_button_bounds,
        bottom_button_bounds,
        max_scroll,
    }
}

#[derive(Clone, Copy, Debug)]
struct TriggerStyle {
    text_color: Color,
    placeholder_color: Color,
    handle_color: Color,
    background: Background,
    border: Border,
    shadow: Shadow,
}

#[derive(Clone, Copy, Debug)]
struct MenuStyle {
    background: Background,
    border: Border,
    text_color: Color,
    muted_text_color: Color,
    selected_text_color: Color,
    selected_background: Background,
    hover_background: Background,
    hover_text_color: Color,
    separator_color: Color,
    shadow: Shadow,
}

fn select_trigger_style(
    theme: &ShadcnTheme,
    props: SelectProps,
    status: SelectStatus,
) -> TriggerStyle {
    let palette = theme.palette;
    let radius = select_radius(theme, props);
    let accent = accent_color(&palette, props.color);
    let accent_text_color = accent_text(&palette, props.color);
    let soft_bg = accent_soft(&palette, props.color);
    let dark_mode = is_dark(&palette);
    let surface_bg = if dark_mode {
        Background::Color(apply_opacity(palette.input, 0.3))
    } else {
        Background::Color(Color::TRANSPARENT)
    };
    let classic_bg = if dark_mode {
        Background::Color(apply_opacity(palette.background, 0.94))
    } else {
        Background::Color(palette.background)
    };

    let mut background = match props.variant {
        TriggerVariant::Soft => Background::Color(soft_bg),
        TriggerVariant::Ghost => Background::Color(Color::TRANSPARENT),
        TriggerVariant::Classic => classic_bg,
        TriggerVariant::Surface => surface_bg,
    };
    let mut border_color = match props.variant {
        TriggerVariant::Soft | TriggerVariant::Ghost => Color::TRANSPARENT,
        TriggerVariant::Classic => palette.border,
        TriggerVariant::Surface => palette.input,
    };
    let mut text_color = match props.variant {
        TriggerVariant::Soft | TriggerVariant::Ghost => accent_text_color,
        _ => palette.foreground,
    };
    let mut placeholder_color = match props.variant {
        TriggerVariant::Soft | TriggerVariant::Ghost => apply_opacity(accent_text_color, 0.6),
        _ => palette.muted_foreground,
    };
    let mut handle_color = match props.variant {
        TriggerVariant::Soft | TriggerVariant::Ghost => apply_opacity(accent_text_color, 0.6),
        _ => apply_opacity(palette.muted_foreground, 0.5),
    };
    let mut shadow = match props.variant {
        TriggerVariant::Classic => shadow_xs(1.0),
        TriggerVariant::Surface => shadow_xs(0.6),
        TriggerVariant::Ghost => Shadow::default(),
        TriggerVariant::Soft => shadow_xs(1.0),
    };

    match status {
        SelectStatus::Hovered | SelectStatus::Opened { .. } => match props.variant {
            TriggerVariant::Soft => {
                background = Background::Color(mix(soft_bg, accent, 0.1));
            }
            TriggerVariant::Ghost => {
                background = Background::Color(apply_opacity(soft_bg, 0.5));
            }
            TriggerVariant::Classic | TriggerVariant::Surface => {
                if dark_mode {
                    background = Background::Color(apply_opacity(palette.input, 0.5));
                }
            }
        },
        SelectStatus::Disabled => {
            if let Background::Color(color) = background {
                background = Background::Color(apply_opacity(color, 0.5));
            }
            border_color = apply_opacity(border_color, 0.5);
            text_color = apply_opacity(text_color, 0.5);
            placeholder_color = apply_opacity(placeholder_color, 0.5);
            handle_color = apply_opacity(handle_color, 0.5);
            shadow = match props.variant {
                TriggerVariant::Ghost => Shadow::default(),
                _ => shadow_xs(0.5),
            };
        }
        SelectStatus::Active => {}
    }

    if props.high_contrast {
        let contrast = accent_high(&palette, props.color);
        border_color = contrast;
        handle_color = contrast;
        if matches!(props.variant, TriggerVariant::Soft | TriggerVariant::Ghost) {
            text_color = contrast;
            placeholder_color = apply_opacity(contrast, 0.75);
        }
    }

    TriggerStyle {
        text_color,
        placeholder_color,
        handle_color,
        background,
        border: Border {
            radius: radius.into(),
            width: 1.0,
            color: border_color,
        },
        shadow,
    }
}

fn select_menu_style(theme: &ShadcnTheme, props: SelectProps) -> MenuStyle {
    let palette = theme.palette;
    let radius = select_radius(theme, props);
    let content_color = props.content_color.unwrap_or(props.color);
    let is_gray = matches!(content_color, AccentColor::Gray);
    let accent = if is_gray {
        palette.accent
    } else {
        accent_color(&palette, content_color)
    };
    let accent_fg = if is_gray {
        palette.accent_foreground
    } else {
        accent_foreground(&palette, content_color)
    };
    let accent_soft_bg = if is_gray {
        palette.accent
    } else {
        accent_soft(&palette, content_color)
    };
    let accent_strong = if is_gray {
        palette.foreground
    } else {
        accent_high(&palette, content_color)
    };

    let background = match props.content_variant {
        ContentVariant::Soft => Background::Color(accent_soft_bg),
        ContentVariant::Solid => Background::Color(palette.popover),
    };
    let mut selected_background = match props.content_variant {
        ContentVariant::Soft => {
            let blend = if is_gray { palette.foreground } else { accent };
            let mix_ratio = if is_gray { 0.08 } else { 0.2 };
            Background::Color(mix(accent_soft_bg, blend, mix_ratio))
        }
        ContentVariant::Solid => Background::Color(accent),
    };
    let mut selected_text_color = accent_fg;

    if props.high_contrast {
        selected_background = Background::Color(accent_strong);
        selected_text_color = palette.background;
    }

    let shadow = shadow_md(1.0);

    let hover_background = match props.content_variant {
        ContentVariant::Soft => {
            let blend = if is_gray { palette.foreground } else { accent };
            let mix_ratio = if is_gray { 0.12 } else { 0.25 };
            Background::Color(mix(accent_soft_bg, blend, mix_ratio))
        }
        ContentVariant::Solid => Background::Color(palette.accent),
    };

    let hover_text_color = match props.content_variant {
        ContentVariant::Soft => {
            if is_gray {
                palette.foreground
            } else {
                accent_strong
            }
        }
        ContentVariant::Solid => palette.accent_foreground,
    };

    MenuStyle {
        background,
        border: Border {
            width: 1.0,
            radius: radius.into(),
            color: palette.border,
        },
        text_color: palette.popover_foreground,
        muted_text_color: palette.muted_foreground,
        selected_text_color,
        selected_background,
        hover_background,
        hover_text_color,
        separator_color: palette.border,
        shadow,
    }
}

fn item_radius(theme: &ShadcnTheme) -> f32 {
    theme.radius.sm
}

use crate::tokens::mix;

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: (color.a * opacity).clamp(0.0, 1.0),
        ..color
    }
}

fn select_content_clip_bounds(layout_info: &ListLayout) -> Rectangle {
    let top_inset = layout_info
        .top_button_bounds
        .map(|bounds| bounds.height)
        .unwrap_or(0.0);
    let bottom_inset = layout_info
        .bottom_button_bounds
        .map(|bounds| bounds.height)
        .unwrap_or(0.0);

    Rectangle {
        x: layout_info.list_bounds.x,
        y: layout_info.list_bounds.y + top_inset,
        width: layout_info.list_bounds.width,
        height: (layout_info.list_bounds.height - top_inset - bottom_inset).max(0.0),
    }
}

fn intersect_rectangles(a: Rectangle, b: Rectangle) -> Rectangle {
    let x = a.x.max(b.x);
    let y = a.y.max(b.y);
    let right = (a.x + a.width).min(b.x + b.width);
    let bottom = (a.y + a.height).min(b.y + b.height);

    Rectangle {
        x,
        y,
        width: (right - x).max(0.0),
        height: (bottom - y).max(0.0),
    }
}

fn select_scroll_overlay_background(theme: &ShadcnTheme, props: SelectProps) -> Background {
    match select_menu_style(theme, props).background {
        Background::Color(color) if color.a > 0.0 => Background::Color(Color { a: 1.0, ..color }),
        _ => Background::Color(theme.palette.popover),
    }
}

fn shadow_xs(opacity: f32) -> Shadow {
    Shadow {
        color: apply_opacity(Color::BLACK, 0.05 * opacity),
        offset: Vector::new(0.0, 1.0),
        blur_radius: 2.0,
    }
}

fn shadow_md(opacity: f32) -> Shadow {
    Shadow {
        color: apply_opacity(Color::BLACK, 0.1 * opacity),
        offset: Vector::new(0.0, 4.0),
        blur_radius: 6.0,
    }
}
