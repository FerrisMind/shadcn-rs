use std::borrow::Cow;

use iced::advanced::Renderer as _;
use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::svg;
use iced::advanced::text;
use iced::advanced::text::Renderer as _;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::border::Border;
use iced::font::Weight;
use iced::keyboard;
use iced::mouse;
use iced::touch;
use iced::{
    Background, Color, Element, Event, Font, Length, Point, Rectangle, Shadow, Size, Vector,
};
use lucide_icons::Icon as LucideIcon;

use crate::overlay::keyboard as overlay_keyboard;
use crate::switch as shadcn_switch;
use crate::theme::Theme;
use crate::tokens::{
    AccentColor, accent_color, accent_foreground, accent_high, accent_soft, accent_soft_foreground,
};

static NOVA_SHIELD_TERMINAL_SVG: &[u8] = include_bytes!("../assets/icons/shield-terminal.svg");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuContentSize {
    Size1,
    Size2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuContentVariant {
    Solid,
    Soft,
}

#[derive(Clone, Copy, Debug)]
pub struct MenuContentProps {
    pub size: MenuContentSize,
    pub variant: MenuContentVariant,
    pub color: AccentColor,
    pub high_contrast: bool,
    pub show_shadow: bool,
    /// Custom corner radius for the menu surface; falls back to the
    /// size-derived metric when unset.
    pub radius: Option<f32>,
    /// Custom corner radius for item hover highlights; falls back to the
    /// size-derived metric when unset.
    pub item_radius: Option<f32>,
}

impl Default for MenuContentProps {
    fn default() -> Self {
        Self {
            size: MenuContentSize::Size2,
            variant: MenuContentVariant::Solid,
            color: AccentColor::Gray,
            high_contrast: false,
            show_shadow: true,
            radius: None,
            item_radius: None,
        }
    }
}

impl MenuContentProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: MenuContentSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: MenuContentVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = color;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn show_shadow(mut self, show_shadow: bool) -> Self {
        self.show_shadow = show_shadow;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius.max(0.0));
        self
    }

    pub fn item_radius(mut self, item_radius: f32) -> Self {
        self.item_radius = Some(item_radius.max(0.0));
        self
    }
}

#[derive(Clone, Debug)]
pub struct MenuItemProps<'a> {
    pub disabled: bool,
    pub inset: bool,
    pub color: Option<AccentColor>,
    pub shortcut: Option<Cow<'a, str>>,
    pub leading_icon: Option<MenuLeadingIcon>,
    pub leading_icon_color: Option<Color>,
    pub trailing_check: bool,
    pub trailing_switch: Option<bool>,
    pub close_on_select: bool,
}

impl<'a> Default for MenuItemProps<'a> {
    fn default() -> Self {
        Self {
            disabled: false,
            inset: false,
            color: None,
            shortcut: None,
            leading_icon: None,
            leading_icon_color: None,
            trailing_check: false,
            trailing_switch: None,
            close_on_select: true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MenuLeadingIcon {
    Lucide(LucideIcon),
    ShieldTerminal,
}

impl<'a> MenuItemProps<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<Cow<'a, str>>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn leading_icon(mut self, icon: LucideIcon) -> Self {
        self.leading_icon = Some(MenuLeadingIcon::Lucide(icon));
        self
    }

    pub fn leading_shield_terminal(mut self) -> Self {
        self.leading_icon = Some(MenuLeadingIcon::ShieldTerminal);
        self
    }

    pub fn leading_icon_color(mut self, color: Color) -> Self {
        self.leading_icon_color = Some(color);
        self
    }

    pub fn trailing_check(mut self, checked: bool) -> Self {
        self.trailing_check = checked;
        self
    }

    pub fn trailing_switch(mut self, checked: bool) -> Self {
        self.trailing_switch = Some(checked);
        self
    }

    pub fn close_on_select(mut self, close_on_select: bool) -> Self {
        self.close_on_select = close_on_select;
        self
    }
}

#[derive(Clone, Debug)]
pub struct MenuItem<'a, Message> {
    pub label: Cow<'a, str>,
    pub on_select: Option<Message>,
    pub props: MenuItemProps<'a>,
}

impl<'a, Message> MenuItem<'a, Message> {
    pub fn new(label: impl Into<Cow<'a, str>>, on_select: Option<Message>) -> Self {
        Self {
            label: label.into(),
            on_select,
            props: MenuItemProps::new(),
        }
    }

    pub fn props(mut self, props: MenuItemProps<'a>) -> Self {
        self.props = props;
        self
    }
}

#[derive(Clone, Debug)]
pub struct MenuCheckboxItem<'a, Message> {
    pub label: Cow<'a, str>,
    pub checked: bool,
    pub on_toggle: Option<Message>,
    pub props: MenuItemProps<'a>,
}

impl<'a, Message> MenuCheckboxItem<'a, Message> {
    pub fn new(label: impl Into<Cow<'a, str>>, checked: bool, on_toggle: Option<Message>) -> Self {
        Self {
            label: label.into(),
            checked,
            on_toggle,
            props: MenuItemProps::new(),
        }
    }

    pub fn props(mut self, props: MenuItemProps<'a>) -> Self {
        self.props = props;
        self
    }
}

#[derive(Clone, Debug)]
pub struct MenuRadioItem<'a, Message> {
    pub label: Cow<'a, str>,
    pub selected: bool,
    pub on_select: Option<Message>,
    pub props: MenuItemProps<'a>,
}

impl<'a, Message> MenuRadioItem<'a, Message> {
    pub fn new(label: impl Into<Cow<'a, str>>, selected: bool, on_select: Option<Message>) -> Self {
        Self {
            label: label.into(),
            selected,
            on_select,
            props: MenuItemProps::new(),
        }
    }

    pub fn props(mut self, props: MenuItemProps<'a>) -> Self {
        self.props = props;
        self
    }
}

#[derive(Clone, Debug)]
pub struct MenuSubMenu<'a, Message> {
    pub label: Cow<'a, str>,
    pub props: MenuItemProps<'a>,
    pub entries: Vec<MenuEntry<'a, Message>>,
}

impl<'a, Message> MenuSubMenu<'a, Message> {
    pub fn new(label: impl Into<Cow<'a, str>>, entries: Vec<MenuEntry<'a, Message>>) -> Self {
        Self {
            label: label.into(),
            props: MenuItemProps::new(),
            entries,
        }
    }

    pub fn props(mut self, props: MenuItemProps<'a>) -> Self {
        self.props = props;
        self
    }
}

#[derive(Clone, Debug)]
pub enum MenuEntry<'a, Message> {
    Label(Cow<'a, str>),
    Separator,
    Item(MenuItem<'a, Message>),
    CheckboxItem(MenuCheckboxItem<'a, Message>),
    RadioItem(MenuRadioItem<'a, Message>),
    SubMenu(MenuSubMenu<'a, Message>),
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum MenuKind {
    Dropdown,
    Context,
}

#[derive(Clone, Debug)]
pub(crate) struct MenuOverlayProps<Message> {
    pub kind: MenuKind,
    pub width: Option<u32>,
    pub offset: f32,
    pub disabled: bool,
    pub on_close: Option<Message>,
}

impl<Message> Default for MenuOverlayProps<Message> {
    fn default() -> Self {
        Self {
            kind: MenuKind::Dropdown,
            width: None,
            offset: 4.0,
            disabled: false,
            on_close: None,
        }
    }
}

#[derive(Debug, Default)]
struct MenuState {
    is_open: bool,
    open_submenu: Option<usize>,
    opened_at: Option<Point>,
    overlay_bounds: Option<Rectangle>,
    submenu_bounds: Option<Rectangle>,
    keyboard_modifiers: keyboard::Modifiers,
    hovered_row: Option<usize>,
    hovered_sub_row: Option<usize>,
    overlay: MenuOverlayState,
}

#[derive(Debug)]
struct MenuOverlayState {
    main_tree: Tree,
    submenu_tree: Tree,
}

impl Default for MenuOverlayState {
    fn default() -> Self {
        Self {
            main_tree: Tree::empty(),
            submenu_tree: Tree::empty(),
        }
    }
}

pub(crate) fn menu<'a, Message: Clone + 'a>(
    trigger: impl Into<Element<'a, Message>>,
    entries: Vec<MenuEntry<'a, Message>>,
    content: MenuContentProps,
    overlay: MenuOverlayProps<Message>,
    theme: &Theme,
) -> Menu<'a, Message> {
    Menu {
        trigger: trigger.into(),
        entries,
        content,
        overlay,
        theme: theme.clone(),
    }
}

pub(crate) struct Menu<'a, Message> {
    trigger: Element<'a, Message>,
    entries: Vec<MenuEntry<'a, Message>>,
    content: MenuContentProps,
    overlay: MenuOverlayProps<Message>,
    theme: Theme,
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer> for Menu<'_, Message>
where
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.trigger)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.trigger.as_widget()]);
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(MenuState::default())
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<MenuState>()
    }

    fn size(&self) -> Size<Length> {
        self.trigger.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.trigger
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<MenuState>();
        let was_open = state.is_open;
        let was_open_submenu = state.open_submenu;
        let was_opened_at = state.opened_at;

        self.trigger.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if self.overlay.disabled {
            state.is_open = false;
            state.open_submenu = None;
            state.overlay_bounds = None;
            state.submenu_bounds = None;
            return;
        }

        let trigger_bounds = layout.bounds();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. })
                if matches!(self.overlay.kind, MenuKind::Dropdown) =>
            {
                let over_trigger = cursor.is_over(trigger_bounds);
                let over_menu = state
                    .overlay_bounds
                    .map(|b| cursor.is_over(b))
                    .unwrap_or(false);
                let over_submenu = state
                    .submenu_bounds
                    .map(|b| cursor.is_over(b))
                    .unwrap_or(false);

                if state.is_open {
                    if over_trigger || (!over_menu && !over_submenu) {
                        state.is_open = false;
                        state.open_submenu = None;
                        shell.capture_event();
                    }
                } else if over_trigger {
                    state.is_open = true;
                    state.opened_at = None;
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right))
                if matches!(self.overlay.kind, MenuKind::Context) =>
            {
                if cursor.is_over(trigger_bounds) {
                    state.is_open = true;
                    state.open_submenu = None;
                    state.opened_at = cursor.position();
                    shell.capture_event();
                } else if state.is_open {
                    let over_menu = state
                        .overlay_bounds
                        .map(|b| cursor.is_over(b))
                        .unwrap_or(false);
                    let over_submenu = state
                        .submenu_bounds
                        .map(|b| cursor.is_over(b))
                        .unwrap_or(false);
                    if !over_menu && !over_submenu {
                        state.is_open = false;
                        state.open_submenu = None;
                        shell.capture_event();
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(_))
            | Event::Touch(touch::Event::FingerPressed { .. })
                if state.is_open =>
            {
                let over_menu = state
                    .overlay_bounds
                    .map(|b| cursor.is_over(b))
                    .unwrap_or(false);
                let over_submenu = state
                    .submenu_bounds
                    .map(|b| cursor.is_over(b))
                    .unwrap_or(false);
                if !over_menu && !over_submenu {
                    state.is_open = false;
                    state.open_submenu = None;
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
                state.open_submenu = None;
                shell.capture_event();
            }
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                state.keyboard_modifiers = *modifiers;
            }
            _ => {}
        }

        if !state.is_open {
            state.overlay_bounds = None;
            state.submenu_bounds = None;
        }

        if was_open
            && !state.is_open
            && let Some(on_close) = self.overlay.on_close.clone()
        {
            shell.publish(on_close);
        }

        if was_open != state.is_open
            || was_open_submenu != state.open_submenu
            || was_opened_at != state.opened_at
        {
            shell.request_redraw();
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<iced::overlay::Element<'b, Message, iced::Theme, iced::Renderer>> {
        let state = tree.state.downcast_mut::<MenuState>();
        if !state.is_open {
            return None;
        }

        let font = renderer.default_font();
        let bounds = layout.bounds();
        let anchor_position = layout.position() + translation;

        Some(iced::overlay::Element::new(Box::new(MenuOverlay {
            entries: &self.entries,
            state,
            theme: self.theme.clone(),
            content: self.content,
            overlay: self.overlay.clone(),
            viewport: *viewport,
            font,
            anchor_position,
            target_size: Size::new(bounds.width, bounds.height),
        })))
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.trigger.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.trigger.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }
}

impl<'a, Message: Clone + 'a> From<Menu<'a, Message>> for Element<'a, Message> {
    fn from(widget: Menu<'a, Message>) -> Element<'a, Message> {
        Element::new(widget)
    }
}

struct MenuOverlay<'a, 'b, Message> {
    entries: &'a [MenuEntry<'b, Message>],
    state: &'a mut MenuState,
    theme: Theme,
    content: MenuContentProps,
    overlay: MenuOverlayProps<Message>,
    viewport: Rectangle,
    font: Font,
    anchor_position: Point,
    target_size: Size,
}

impl<Message> iced::advanced::Overlay<Message, iced::Theme, iced::Renderer>
    for MenuOverlay<'_, '_, Message>
where
    Message: Clone,
{
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        let metrics = menu_metrics(&self.theme, self.content);
        let menu_width = self
            .overlay
            .width
            .map(|w| w as f32)
            .unwrap_or(self.target_size.width.max(128.0));
        let overlay_bounds = Size::new(
            bounds.width.max(self.viewport.width),
            bounds.height.max(self.viewport.height),
        );

        let limits = layout::Limits::new(Size::ZERO, overlay_bounds).width(menu_width);

        let mut main_list = MenuList {
            entries: self.entries,
            hovered_row: &mut self.state.hovered_row,
            open_submenu: Some(&mut self.state.open_submenu),
            is_open: Some(&mut self.state.is_open),
            metrics,
            font: self.font,
            content: self.content,
            theme: self.theme.clone(),
        };

        self.state
            .overlay
            .main_tree
            .diff::<Message, iced::Theme, iced::Renderer>(&main_list as &dyn Widget<_, _, _>);

        let main_node = main_list.layout(&mut self.state.overlay.main_tree, renderer, &limits);
        let main_size = main_node.size();

        let collision_padding = 10.0;
        let min_x = self.viewport.x + collision_padding;
        let max_x = (self.viewport.x + self.viewport.width - main_size.width - collision_padding)
            .max(min_x);
        let min_y = self.viewport.y + collision_padding;
        let max_y = (self.viewport.y + self.viewport.height - main_size.height - collision_padding)
            .max(min_y);
        let space_below = bounds.height - (self.anchor_position.y + self.target_size.height);
        let space_above = self.anchor_position.y;

        let x = match self.overlay.kind {
            MenuKind::Dropdown => self.anchor_position.x,
            MenuKind::Context => self.state.opened_at.unwrap_or(self.anchor_position).x,
        };

        let x = x.clamp(min_x, max_x);

        let y = match self.overlay.kind {
            MenuKind::Dropdown => {
                if space_below >= space_above {
                    self.anchor_position.y + self.target_size.height + self.overlay.offset
                } else {
                    self.anchor_position.y - main_size.height - self.overlay.offset
                }
            }
            MenuKind::Context => self.state.opened_at.unwrap_or(self.anchor_position).y,
        }
        .clamp(min_y, max_y);

        let mut children = Vec::new();
        let main_node = main_node.move_to(Point::new(x, y));
        self.state.overlay_bounds = Some(main_node.bounds());
        children.push(main_node);

        if let Some(submenu_index) = self.state.open_submenu {
            if let Some(MenuEntry::SubMenu(submenu)) = self.entries.get(submenu_index) {
                let mut submenu_list = MenuList {
                    entries: &submenu.entries,
                    hovered_row: &mut self.state.hovered_sub_row,
                    open_submenu: None,
                    is_open: Some(&mut self.state.is_open),
                    metrics,
                    font: self.font,
                    content: self.content,
                    theme: self.theme.clone(),
                };

                self.state
                    .overlay
                    .submenu_tree
                    .diff::<Message, iced::Theme, iced::Renderer>(
                        &submenu_list as &dyn Widget<_, _, _>,
                    );

                let submenu_node =
                    submenu_list.layout(&mut self.state.overlay.submenu_tree, renderer, &limits);

                let submenu_size = submenu_node.size();
                let submenu_gap = 4.0;
                let right_x = x + main_size.width + submenu_gap;
                let left_x = x - submenu_size.width - submenu_gap;
                let submenu_min_x = self.viewport.x + collision_padding;
                let submenu_max_x = (self.viewport.x + self.viewport.width
                    - submenu_size.width
                    - collision_padding)
                    .max(submenu_min_x);
                let submenu_x = if right_x <= submenu_max_x {
                    right_x
                } else {
                    left_x.clamp(submenu_min_x, submenu_max_x)
                };
                let submenu_min_y = self.viewport.y + collision_padding;
                let submenu_max_y = (self.viewport.y + self.viewport.height
                    - submenu_size.height
                    - collision_padding)
                    .max(submenu_min_y);
                let submenu_y = y.clamp(submenu_min_y, submenu_max_y);

                let submenu_node = submenu_node.move_to(Point::new(submenu_x, submenu_y));
                self.state.submenu_bounds = Some(submenu_node.bounds());
                children.push(submenu_node);
            } else {
                self.state.open_submenu = None;
                self.state.submenu_bounds = None;
            }
        } else {
            self.state.submenu_bounds = None;
        }

        layout::Node::with_children(overlay_bounds, children)
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let metrics = menu_metrics(&self.theme, self.content);
        let bounds = layout.bounds();

        let mut children = layout.children();
        let Some(main_layout) = children.next() else {
            return;
        };

        let mut main_list = MenuList {
            entries: self.entries,
            hovered_row: &mut self.state.hovered_row,
            open_submenu: Some(&mut self.state.open_submenu),
            is_open: Some(&mut self.state.is_open),
            metrics,
            font: self.font,
            content: self.content,
            theme: self.theme.clone(),
        };

        main_list.update(
            &mut self.state.overlay.main_tree,
            event,
            main_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &bounds,
        );

        if let Some(submenu_layout) = children.next()
            && let Some(submenu_index) = self.state.open_submenu
            && let Some(MenuEntry::SubMenu(submenu)) = self.entries.get(submenu_index)
        {
            let mut submenu_list = MenuList {
                entries: &submenu.entries,
                hovered_row: &mut self.state.hovered_sub_row,
                open_submenu: None,
                is_open: Some(&mut self.state.is_open),
                metrics,
                font: self.font,
                content: self.content,
                theme: self.theme.clone(),
            };

            submenu_list.update(
                &mut self.state.overlay.submenu_tree,
                event,
                submenu_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                &bounds,
            );
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        _theme: &iced::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let overlay_viewport = layout.bounds();
        let metrics = menu_metrics(&self.theme, self.content);

        for (index, child_layout) in layout.children().enumerate() {
            let bounds = child_layout.bounds();
            let menu_style = menu_style(&self.theme, self.content);

            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: menu_style.border(metrics.radius),
                    shadow: menu_style.shadow,
                    ..renderer::Quad::default()
                },
                menu_style.background,
            );

            if index == 0 {
                let mut hovered_row = self.state.hovered_row;
                let list = MenuList {
                    entries: self.entries,
                    hovered_row: &mut hovered_row,
                    open_submenu: None,
                    is_open: None,
                    metrics,
                    font: self.font,
                    content: self.content,
                    theme: self.theme.clone(),
                };

                <MenuList<'_, '_, Message> as Widget<Message, iced::Theme, iced::Renderer>>::draw(
                    &list,
                    &self.state.overlay.main_tree,
                    renderer,
                    _theme,
                    style,
                    child_layout,
                    cursor,
                    &overlay_viewport,
                );
            } else if let Some(submenu_index) = self.state.open_submenu
                && let Some(MenuEntry::SubMenu(submenu)) = self.entries.get(submenu_index)
            {
                let mut hovered_row = self.state.hovered_sub_row;
                let list = MenuList {
                    entries: &submenu.entries,
                    hovered_row: &mut hovered_row,
                    open_submenu: None,
                    is_open: None,
                    metrics,
                    font: self.font,
                    content: self.content,
                    theme: self.theme.clone(),
                };

                <MenuList<'_, '_, Message> as Widget<Message, iced::Theme, iced::Renderer>>::draw(
                    &list,
                    &self.state.overlay.submenu_tree,
                    renderer,
                    _theme,
                    style,
                    child_layout,
                    cursor,
                    &overlay_viewport,
                );
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct MenuMetrics {
    content_padding: f32,
    item_height: f32,
    label_height: f32,
    separator_height: f32,
    font_size: f32,
    label_font_size: f32,
    shortcut_font_size: f32,
    indicator_size: f32,
    base_padding_x: f32,
    inset_padding_x: f32,
    radius: f32,
    item_radius: f32,
}

fn menu_metrics(theme: &Theme, content: MenuContentProps) -> MenuMetrics {
    let mut metrics = match content.size {
        MenuContentSize::Size1 => MenuMetrics {
            content_padding: theme.spacing.xs,
            item_height: 28.0,
            label_height: 28.0,
            separator_height: 9.0,
            font_size: 12.0,
            label_font_size: 12.0,
            shortcut_font_size: 10.0,
            indicator_size: 12.0,
            base_padding_x: theme.spacing.sm,
            inset_padding_x: 20.0,
            radius: theme.radius.md,
            item_radius: theme.radius.md,
        },
        MenuContentSize::Size2 => MenuMetrics {
            content_padding: theme.spacing.xs,
            item_height: 32.0,
            label_height: 32.0,
            separator_height: 9.0,
            font_size: 14.0,
            label_font_size: 14.0,
            shortcut_font_size: 12.0,
            indicator_size: 14.0,
            base_padding_x: theme.spacing.sm,
            inset_padding_x: 24.0,
            radius: theme.radius.md,
            item_radius: theme.radius.md,
        },
    };
    if let Some(radius) = content.radius {
        metrics.radius = radius;
    }
    if let Some(item_radius) = content.item_radius {
        metrics.item_radius = item_radius;
    }
    metrics
}

#[derive(Clone, Copy)]
struct ResolvedMenuStyle {
    background: Background,
    border_color: Color,
    shadow: Shadow,
    text_color: Color,
    muted_text_color: Color,
    disabled_text_color: Color,
}

impl ResolvedMenuStyle {
    fn border(&self, radius: f32) -> Border {
        Border {
            color: self.border_color,
            width: crate::theme::ThemeStyles::default().menu.border_width,
            radius: radius.into(),
        }
    }
}

fn apply_opacity(mut color: Color, opacity: f32) -> Color {
    color.a *= opacity;
    color
}

fn switch_bounds(theme: &Theme, metrics: MenuMetrics, row_bounds: Rectangle) -> Rectangle {
    let switch_width = shadcn_switch::size1_width(theme);
    shadcn_switch::size1_bounds(
        theme,
        row_bounds.x + row_bounds.width - metrics.base_padding_x - switch_width,
        row_bounds.center_y(),
    )
}

fn menu_style(theme: &Theme, props: MenuContentProps) -> ResolvedMenuStyle {
    let shadow = if props.show_shadow {
        Shadow {
            color: Color {
                a: theme.styles.menu.shadow.opacity,
                ..theme.palette.foreground
            },
            offset: Vector::new(0.0, theme.styles.menu.shadow.offset_y),
            blur_radius: theme.styles.menu.shadow.blur_radius,
        }
    } else {
        Shadow::default()
    };

    ResolvedMenuStyle {
        background: Background::Color(theme.palette.popover),
        border_color: theme.palette.border,
        shadow,
        text_color: theme.palette.popover_foreground,
        muted_text_color: theme.palette.muted_foreground,
        disabled_text_color: apply_opacity(theme.palette.popover_foreground, 0.45),
    }
}

fn hovered_colors(
    theme: &Theme,
    content: MenuContentProps,
    item_color: AccentColor,
) -> (Background, Color) {
    let is_gray = item_color == AccentColor::Gray;
    match content.variant {
        MenuContentVariant::Solid => {
            if content.high_contrast {
                let bg = if is_gray {
                    theme.palette.foreground
                } else {
                    accent_high(&theme.palette, item_color)
                };
                let fg = if is_gray {
                    theme.palette.background
                } else {
                    accent_foreground(&theme.palette, item_color)
                };
                (Background::Color(bg), fg)
            } else {
                let bg = if is_gray {
                    theme.palette.accent
                } else {
                    accent_color(&theme.palette, item_color)
                };
                let fg = if is_gray {
                    theme.palette.accent_foreground
                } else {
                    accent_foreground(&theme.palette, item_color)
                };
                (Background::Color(bg), fg)
            }
        }
        MenuContentVariant::Soft => {
            let bg = if is_gray {
                theme.palette.accent
            } else {
                accent_soft(&theme.palette, item_color)
            };
            let fg = if is_gray {
                theme.palette.accent_foreground
            } else {
                accent_soft_foreground(&theme.palette, item_color)
            };
            (Background::Color(bg), fg)
        }
    }
}

#[derive(Debug, Default)]
struct MenuListState {
    is_hovered: Option<bool>,
}

struct MenuRow<'a, Message> {
    height: f32,
    kind: MenuRowKind<'a, Message>,
}

#[derive(Clone, Copy, Debug)]
enum MenuIndicator {
    Check,
    Radio,
}

enum MenuRowKind<'a, Message> {
    Label(Cow<'a, str>),
    Separator,
    Item {
        entry_index: usize,
        label: Cow<'a, str>,
        disabled: bool,
        inset: bool,
        shortcut: Option<Cow<'a, str>>,
        leading_icon: Option<MenuLeadingIcon>,
        leading_icon_color: Option<Color>,
        indicator: Option<MenuIndicator>,
        trailing_check: bool,
        trailing_switch: Option<bool>,
        close_on_select: bool,
        submenu: bool,
        on_select: Option<Message>,
        color: Option<AccentColor>,
    },
}

fn build_rows<'a, Message: Clone>(
    entries: &'a [MenuEntry<'a, Message>],
    metrics: MenuMetrics,
) -> Vec<MenuRow<'a, Message>> {
    let mut rows = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        match entry {
            MenuEntry::Label(text) => rows.push(MenuRow {
                height: metrics.label_height,
                kind: MenuRowKind::Label(text.clone()),
            }),
            MenuEntry::Separator => rows.push(MenuRow {
                height: metrics.separator_height,
                kind: MenuRowKind::Separator,
            }),
            MenuEntry::Item(item) => rows.push(MenuRow {
                height: metrics.item_height,
                kind: MenuRowKind::Item {
                    entry_index: index,
                    label: item.label.clone(),
                    disabled: item.props.disabled,
                    inset: item.props.inset,
                    shortcut: item.props.shortcut.clone(),
                    leading_icon: item.props.leading_icon,
                    leading_icon_color: item.props.leading_icon_color,
                    indicator: None,
                    trailing_check: item.props.trailing_check,
                    trailing_switch: item.props.trailing_switch,
                    close_on_select: item.props.close_on_select,
                    submenu: false,
                    on_select: item.on_select.clone(),
                    color: item.props.color,
                },
            }),
            MenuEntry::CheckboxItem(item) => rows.push(MenuRow {
                height: metrics.item_height,
                kind: MenuRowKind::Item {
                    entry_index: index,
                    label: item.label.clone(),
                    disabled: item.props.disabled,
                    inset: item.props.inset,
                    shortcut: item.props.shortcut.clone(),
                    leading_icon: item.props.leading_icon,
                    leading_icon_color: item.props.leading_icon_color,
                    indicator: item.checked.then_some(MenuIndicator::Check),
                    trailing_check: item.props.trailing_check,
                    trailing_switch: item.props.trailing_switch,
                    close_on_select: item.props.close_on_select,
                    submenu: false,
                    on_select: item.on_toggle.clone(),
                    color: item.props.color,
                },
            }),
            MenuEntry::RadioItem(item) => rows.push(MenuRow {
                height: metrics.item_height,
                kind: MenuRowKind::Item {
                    entry_index: index,
                    label: item.label.clone(),
                    disabled: item.props.disabled,
                    inset: item.props.inset,
                    shortcut: item.props.shortcut.clone(),
                    leading_icon: item.props.leading_icon,
                    leading_icon_color: item.props.leading_icon_color,
                    indicator: item.selected.then_some(MenuIndicator::Radio),
                    trailing_check: item.props.trailing_check,
                    trailing_switch: item.props.trailing_switch,
                    close_on_select: item.props.close_on_select,
                    submenu: false,
                    on_select: item.on_select.clone(),
                    color: item.props.color,
                },
            }),
            MenuEntry::SubMenu(item) => rows.push(MenuRow {
                height: metrics.item_height,
                kind: MenuRowKind::Item {
                    entry_index: index,
                    label: item.label.clone(),
                    disabled: item.props.disabled,
                    inset: item.props.inset,
                    shortcut: item.props.shortcut.clone(),
                    leading_icon: item.props.leading_icon,
                    leading_icon_color: item.props.leading_icon_color,
                    indicator: None,
                    trailing_check: item.props.trailing_check,
                    trailing_switch: item.props.trailing_switch,
                    close_on_select: item.props.close_on_select,
                    submenu: true,
                    on_select: None,
                    color: item.props.color,
                },
            }),
        }
    }
    rows
}

struct MenuList<'a, 'b, Message> {
    entries: &'a [MenuEntry<'b, Message>],
    hovered_row: &'a mut Option<usize>,
    open_submenu: Option<&'a mut Option<usize>>,
    is_open: Option<&'a mut bool>,
    metrics: MenuMetrics,
    font: Font,
    content: MenuContentProps,
    theme: Theme,
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer> for MenuList<'_, '_, Message>
where
    Message: Clone,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<MenuListState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(MenuListState::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Shrink)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let rows = build_rows(self.entries, self.metrics);
        let content_height = rows.iter().map(|row| row.height).sum::<f32>();
        let intrinsic = Size::new(0.0, content_height + self.metrics.content_padding * 2.0);
        layout::Node::new(limits.resolve(Length::Fill, Length::Shrink, intrinsic))
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let rows = build_rows(self.entries, self.metrics);
        let state = tree.state.downcast_mut::<MenuListState>();

        let list_bounds = Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width,
            height: bounds.height,
        };

        fn row_at(rows: &[MenuRow<'_, impl Clone>], y: f32) -> Option<usize> {
            let mut cursor = 0.0;
            for (idx, row) in rows.iter().enumerate() {
                let next = cursor + row.height;
                if y >= cursor && y < next {
                    return Some(idx);
                }
                cursor = next;
            }
            None
        }

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some(pos) = cursor.position_in(list_bounds) {
                    let y = (pos.y - self.metrics.content_padding).max(0.0);
                    if let Some(index) = row_at(&rows, y) {
                        match &rows[index].kind {
                            MenuRowKind::Item { disabled: true, .. } => *self.hovered_row = None,
                            MenuRowKind::Item { .. } => *self.hovered_row = Some(index),
                            _ => *self.hovered_row = None,
                        }
                    } else {
                        *self.hovered_row = None;
                    }
                } else {
                    *self.hovered_row = None;
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(pos) = cursor.position_in(list_bounds) {
                    let y = (pos.y - self.metrics.content_padding).max(0.0);
                    if let Some(index) = row_at(&rows, y)
                        && let MenuRowKind::Item {
                            entry_index,
                            disabled: false,
                            close_on_select,
                            trailing_switch,
                            submenu,
                            on_select,
                            ..
                        } = &rows[index].kind
                    {
                        if *submenu {
                            if let Some(open_submenu) = self.open_submenu.as_deref_mut() {
                                if open_submenu.as_ref() == Some(entry_index) {
                                    *open_submenu = None;
                                } else {
                                    *open_submenu = Some(*entry_index);
                                }
                            }
                        } else {
                            let row_y = rows.iter().take(index).map(|row| row.height).sum::<f32>();
                            let row_bounds = Rectangle {
                                x: bounds.x + self.metrics.content_padding,
                                y: bounds.y + self.metrics.content_padding + row_y,
                                width: (bounds.width - self.metrics.content_padding * 2.0).max(0.0),
                                height: rows[index].height,
                            };
                            let over_switch = trailing_switch.is_some_and(|_| {
                                cursor.position().is_some_and(|point| {
                                    switch_bounds(&self.theme, self.metrics, row_bounds)
                                        .contains(point)
                                })
                            });
                            if *close_on_select && !over_switch {
                                if let Some(is_open) = self.is_open.as_deref_mut() {
                                    *is_open = false;
                                }
                                if let Some(open_submenu) = self.open_submenu.as_deref_mut() {
                                    *open_submenu = None;
                                }
                            }
                            if let Some(message) = on_select.clone() {
                                shell.publish(message);
                            }
                        }
                        shell.capture_event();
                    }
                }
            }
            _ => {}
        }

        if let Event::Window(iced::window::Event::RedrawRequested(_now)) = event {
            state.is_hovered = Some(cursor.is_over(bounds));
        } else if state
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
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut iced::Renderer,
        _theme: &iced::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let rows = build_rows(self.entries, self.metrics);
        let menu_style = menu_style(&self.theme, self.content);

        let mut y = bounds.y + self.metrics.content_padding;
        for (index, row) in rows.iter().enumerate() {
            let row_bounds = Rectangle {
                x: bounds.x + self.metrics.content_padding,
                y,
                width: (bounds.width - self.metrics.content_padding * 2.0).max(0.0),
                height: row.height,
            };
            y += row.height;

            match &row.kind {
                MenuRowKind::Separator => {
                    let line_y = row_bounds.y + row_bounds.height / 2.0;
                    let separator_inset = 12.0;
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: Rectangle {
                                x: bounds.x + separator_inset,
                                y: line_y,
                                width: (bounds.width - separator_inset * 2.0).max(0.0),
                                height: 1.0,
                            },
                            ..renderer::Quad::default()
                        },
                        Background::Color(menu_style.border_color),
                    );
                }
                MenuRowKind::Label(label) => {
                    let label_font = Font {
                        weight: Weight::Medium,
                        ..self.font
                    };
                    let label_x = row_bounds.x;
                    renderer.fill_text(
                        text::Text {
                            content: label.to_string(),
                            size: self.metrics.label_font_size.into(),
                            line_height: text::LineHeight::Absolute(
                                self.metrics.label_height.into(),
                            ),
                            font: label_font,
                            bounds: Size::new(row_bounds.width, row_bounds.height),
                            align_x: text::Alignment::Left,
                            align_y: iced::alignment::Vertical::Center,
                            shaping: text::Shaping::Basic,
                            wrapping: text::Wrapping::default(),
                        },
                        Point::new(label_x, row_bounds.center_y()),
                        menu_style.muted_text_color,
                        *viewport,
                    );
                }
                MenuRowKind::Item {
                    label,
                    disabled,
                    inset,
                    shortcut,
                    leading_icon,
                    leading_icon_color,
                    indicator,
                    trailing_check,
                    trailing_switch,
                    submenu,
                    color,
                    ..
                } => {
                    let is_hovered = self.hovered_row.is_some_and(|hovered| hovered == index);
                    let item_color = color.unwrap_or(self.content.color);
                    let icon_font = Font::with_name("lucide");

                    let mut text_color = menu_style.text_color;
                    if is_hovered && !disabled {
                        let (bg, fg) = hovered_colors(&self.theme, self.content, item_color);
                        text_color = fg;
                        renderer.fill_quad(
                            renderer::Quad {
                                bounds: row_bounds,
                                border: Border {
                                    radius: self.metrics.item_radius.into(),
                                    ..Border::default()
                                },
                                ..renderer::Quad::default()
                            },
                            bg,
                        );
                    }

                    if *disabled {
                        text_color = menu_style.disabled_text_color;
                    }

                    let icon_x = row_bounds.x
                        + self.metrics.base_padding_x
                        + self.metrics.indicator_size / 2.0;
                    let icon_column_width =
                        self.metrics.indicator_size + self.metrics.base_padding_x * 2.0;
                    let needs_inset = *inset || indicator.is_some();
                    let label_x = if leading_icon.is_some() {
                        row_bounds.x + icon_column_width
                    } else {
                        row_bounds.x
                            + self.metrics.base_padding_x
                            + if needs_inset {
                                self.metrics.inset_padding_x
                            } else {
                                0.0
                            }
                    };

                    if let Some(indicator) = indicator {
                        let (icon, icon_size) = match indicator {
                            MenuIndicator::Check => {
                                (LucideIcon::Check, self.metrics.indicator_size)
                            }
                            MenuIndicator::Radio => (
                                LucideIcon::Circle,
                                (self.metrics.indicator_size * 0.6).max(8.0),
                            ),
                        };
                        renderer.fill_text(
                            text::Text {
                                content: char::from(icon).to_string(),
                                size: icon_size.into(),
                                line_height: text::LineHeight::Absolute(icon_size.into()),
                                font: icon_font,
                                bounds: Size::new(icon_size, row_bounds.height),
                                align_x: text::Alignment::Center,
                                align_y: iced::alignment::Vertical::Center,
                                shaping: text::Shaping::Basic,
                                wrapping: text::Wrapping::default(),
                            },
                            Point::new(icon_x, row_bounds.center_y()),
                            text_color,
                            *viewport,
                        );
                    }

                    if let Some(icon) = leading_icon {
                        let icon_size = self.metrics.indicator_size;
                        let icon_color = leading_icon_color.unwrap_or(text_color);
                        match icon {
                            MenuLeadingIcon::Lucide(icon) => {
                                renderer.fill_text(
                                    text::Text {
                                        content: char::from(*icon).to_string(),
                                        size: icon_size.into(),
                                        line_height: text::LineHeight::Absolute(icon_size.into()),
                                        font: icon_font,
                                        bounds: Size::new(icon_size, row_bounds.height),
                                        align_x: text::Alignment::Center,
                                        align_y: iced::alignment::Vertical::Center,
                                        shaping: text::Shaping::Basic,
                                        wrapping: text::Wrapping::default(),
                                    },
                                    Point::new(icon_x, row_bounds.center_y()),
                                    icon_color,
                                    *viewport,
                                );
                            }
                            MenuLeadingIcon::ShieldTerminal => {
                                let handle =
                                    svg::Handle::from_memory(NOVA_SHIELD_TERMINAL_SVG.to_vec());
                                let bounds = Rectangle {
                                    x: icon_x - icon_size / 2.0,
                                    y: row_bounds.center_y() - icon_size / 2.0,
                                    width: icon_size,
                                    height: icon_size,
                                };
                                svg::Renderer::draw_svg(
                                    renderer,
                                    svg::Svg::new(handle).color(icon_color),
                                    bounds,
                                    *viewport,
                                );
                            }
                        }
                    }

                    renderer.fill_text(
                        text::Text {
                            content: label.to_string(),
                            size: self.metrics.font_size.into(),
                            line_height: text::LineHeight::Absolute(row_bounds.height.into()),
                            font: self.font,
                            bounds: Size::new(row_bounds.width, row_bounds.height),
                            align_x: text::Alignment::Left,
                            align_y: iced::alignment::Vertical::Center,
                            shaping: text::Shaping::Basic,
                            wrapping: text::Wrapping::default(),
                        },
                        Point::new(label_x, row_bounds.center_y()),
                        text_color,
                        *viewport,
                    );

                    if let Some(shortcut) = shortcut {
                        let shortcut_color = if *disabled {
                            menu_style.disabled_text_color
                        } else {
                            menu_style.muted_text_color
                        };
                        let shortcut_bounds = Size::new(
                            (row_bounds.width - self.metrics.base_padding_x * 2.0).max(0.0),
                            row_bounds.height,
                        );
                        renderer.fill_text(
                            text::Text {
                                content: shortcut.to_string(),
                                size: self.metrics.shortcut_font_size.into(),
                                line_height: text::LineHeight::Absolute(row_bounds.height.into()),
                                font: self.font,
                                bounds: shortcut_bounds,
                                align_x: text::Alignment::Right,
                                align_y: iced::alignment::Vertical::Center,
                                shaping: text::Shaping::Basic,
                                wrapping: text::Wrapping::default(),
                            },
                            Point::new(
                                row_bounds.x + self.metrics.base_padding_x,
                                row_bounds.center_y(),
                            ),
                            shortcut_color,
                            *viewport,
                        );
                    }

                    if *trailing_check {
                        let icon_size = self.metrics.indicator_size;
                        renderer.fill_text(
                            text::Text {
                                content: char::from(LucideIcon::Check).to_string(),
                                size: icon_size.into(),
                                line_height: text::LineHeight::Absolute(icon_size.into()),
                                font: icon_font,
                                bounds: Size::new(icon_size, row_bounds.height),
                                align_x: text::Alignment::Center,
                                align_y: iced::alignment::Vertical::Center,
                                shaping: text::Shaping::Basic,
                                wrapping: text::Wrapping::default(),
                            },
                            Point::new(
                                row_bounds.x + row_bounds.width
                                    - self.metrics.base_padding_x
                                    - icon_size / 2.0,
                                row_bounds.center_y(),
                            ),
                            text_color,
                            *viewport,
                        );
                    }

                    if let Some(is_checked) = trailing_switch {
                        shadcn_switch::draw_size1_switch(
                            &self.theme,
                            renderer,
                            switch_bounds(&self.theme, self.metrics, row_bounds),
                            *is_checked,
                            *disabled,
                            color.unwrap_or(self.content.color),
                        );
                    }

                    if *submenu {
                        let icon_size = self.metrics.indicator_size;
                        renderer.fill_text(
                            text::Text {
                                content: char::from(LucideIcon::ChevronRight).to_string(),
                                size: icon_size.into(),
                                line_height: text::LineHeight::Absolute(icon_size.into()),
                                font: icon_font,
                                bounds: Size::new(icon_size, row_bounds.height),
                                align_x: text::Alignment::Center,
                                align_y: iced::alignment::Vertical::Center,
                                shaping: text::Shaping::Basic,
                                wrapping: text::Wrapping::default(),
                            },
                            Point::new(
                                row_bounds.x + row_bounds.width
                                    - self.metrics.base_padding_x
                                    - icon_size / 2.0,
                                row_bounds.center_y(),
                            ),
                            menu_style.muted_text_color,
                            *viewport,
                        );
                    }
                }
            }
        }
    }
}
