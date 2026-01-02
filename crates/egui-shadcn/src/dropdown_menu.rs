//! Dropdown Menu component - displays a menu on trigger click.
//!
//! Built on top of egui's Popup::menu() with shadcn styling.

pub use crate::menu_primitives::{
    MenuCheckboxItemProps as DropdownMenuCheckboxItemProps, MenuItemProps as DropdownMenuItemProps,
    MenuItemVariant as DropdownMenuItemVariant, MenuLabelProps as DropdownMenuLabelProps,
    MenuRadioGroupProps as DropdownMenuRadioGroupProps,
    MenuRadioItemProps as DropdownMenuRadioItemProps, MenuSubProps as DropdownMenuSubProps,
    MenuTokens as DropdownMenuTokens, menu_label as dropdown_menu_label,
    menu_radio_group as dropdown_menu_radio_group, menu_separator as dropdown_menu_separator,
    menu_shortcut as dropdown_menu_shortcut, menu_tokens as dropdown_menu_tokens,
};
use crate::menu_primitives::{
    menu_checkbox_item as base_menu_checkbox_item, menu_item as base_menu_item,
    menu_radio_item as base_menu_radio_item, menu_sub as base_menu_sub,
};
use crate::theme::Theme;
use egui::{Context, Frame, Id, Popup, Response, Stroke, Ui, Vec2};

const DROPDOWN_MENU_STACK_KEY: &str = "egui_shadcn_dropdown_menu_stack";

#[derive(Clone, Default)]
struct DropdownMenuScope {
    first_focusable: Option<Id>,
}

struct DropdownMenuScopeGuard {
    ctx: Context,
    menu_id: Id,
}

impl DropdownMenuScopeGuard {
    fn new(ctx: &Context, menu_id: Id) -> Self {
        push_menu_scope(ctx, menu_id);
        Self {
            ctx: ctx.clone(),
            menu_id,
        }
    }
}

impl Drop for DropdownMenuScopeGuard {
    fn drop(&mut self) {
        pop_menu_scope(&self.ctx, self.menu_id);
    }
}

fn dropdown_menu_stack_id() -> Id {
    Id::new(DROPDOWN_MENU_STACK_KEY)
}

fn dropdown_menu_scope_id(menu_id: Id) -> Id {
    menu_id.with("dropdown-menu-scope")
}

fn dropdown_menu_state_id(menu_id: Id) -> Id {
    menu_id.with("dropdown-menu-open")
}

fn push_menu_scope(ctx: &Context, menu_id: Id) {
    ctx.data_mut(|data| {
        let stack_id = dropdown_menu_stack_id();
        let mut stack = data.get_temp::<Vec<Id>>(stack_id).unwrap_or_default();
        stack.push(menu_id);
        data.insert_temp(stack_id, stack);
        data.insert_temp(
            dropdown_menu_scope_id(menu_id),
            DropdownMenuScope::default(),
        );
    });
}

fn pop_menu_scope(ctx: &Context, menu_id: Id) {
    ctx.data_mut(|data| {
        let stack_id = dropdown_menu_stack_id();
        if let Some(mut stack) = data.get_temp::<Vec<Id>>(stack_id)
            && stack.last().copied() == Some(menu_id)
        {
            stack.pop();
            data.insert_temp(stack_id, stack);
        }
    });
}

fn current_menu_id(ctx: &Context) -> Option<Id> {
    ctx.data(|data| {
        data.get_temp::<Vec<Id>>(dropdown_menu_stack_id())
            .and_then(|stack| stack.last().copied())
    })
}

fn register_focusable(ctx: &Context, response: &Response, disabled: bool) {
    if disabled {
        return;
    }

    let Some(menu_id) = current_menu_id(ctx) else {
        return;
    };

    ctx.data_mut(|data| {
        let scope_id = dropdown_menu_scope_id(menu_id);
        let mut scope = data
            .get_temp::<DropdownMenuScope>(scope_id)
            .unwrap_or_default();
        if scope.first_focusable.is_none() {
            scope.first_focusable = Some(response.id);
        }
        data.insert_temp(scope_id, scope);
    });
}

fn first_focusable_id(ctx: &Context, menu_id: Id) -> Option<Id> {
    ctx.data(|data| {
        data.get_temp::<DropdownMenuScope>(dropdown_menu_scope_id(menu_id))
            .and_then(|scope| scope.first_focusable)
    })
}

#[derive(Clone, Debug)]
pub struct DropdownMenuProps<'a> {
    pub trigger: &'a Response,
    pub width: Option<f32>,
}

impl<'a> DropdownMenuProps<'a> {
    pub fn new(trigger: &'a Response) -> Self {
        Self {
            trigger,
            width: None,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DropdownMenuTriggerProps {
    pub id_source: Id,
    pub disabled: bool,
}

impl DropdownMenuTriggerProps {
    pub fn new(id_source: Id) -> Self {
        Self {
            id_source,
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub struct DropdownMenuTriggerResponse {
    pub response: Response,
    pub is_open: bool,
}

pub fn dropdown_menu_trigger(
    ui: &mut Ui,
    props: DropdownMenuTriggerProps,
    render_trigger: impl FnOnce(&mut Ui) -> Response,
) -> DropdownMenuTriggerResponse {
    let trigger_response = ui.add_enabled_ui(!props.disabled, |ui| {
        ui.push_id(props.id_source, |ui| render_trigger(ui)).inner
    });
    let response = trigger_response.inner;
    let popup_id = Popup::default_response_id(&response);
    let is_open = Popup::is_id_open(ui.ctx(), popup_id);
    DropdownMenuTriggerResponse { response, is_open }
}

pub fn dropdown_menu<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: DropdownMenuProps<'_>,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    let tokens = dropdown_menu_tokens(theme);
    let popup_id = Popup::default_response_id(props.trigger);
    let state_id = dropdown_menu_state_id(popup_id);
    let was_open = ui
        .ctx()
        .data(|d| d.get_temp::<bool>(state_id))
        .unwrap_or(false);
    let menu_width = props.width.unwrap_or(tokens.min_width);

    let inner = Popup::menu(props.trigger)
        .frame(
            Frame::popup(ui.style())
                .fill(tokens.bg)
                .stroke(Stroke::new(1.0, tokens.border))
                .corner_radius(tokens.rounding)
                .inner_margin(tokens.padding),
        )
        .show(|menu_ui| {
            let _guard = DropdownMenuScopeGuard::new(menu_ui.ctx(), popup_id);
            menu_ui.visuals_mut().override_text_color = Some(tokens.text);
            menu_ui.spacing_mut().item_spacing = Vec2::new(0.0, 2.0);
            menu_ui.set_min_width(menu_width);
            add_contents(menu_ui)
        });

    let is_open = inner.is_some();
    if is_open
        && !was_open
        && let Some(first_id) = first_focusable_id(ui.ctx(), popup_id)
    {
        ui.ctx().memory_mut(|mem| mem.request_focus(first_id));
    }
    ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
    inner.map(|response| response.inner)
}

pub fn dropdown_menu_item(
    ui: &mut Ui,
    theme: &Theme,
    props: DropdownMenuItemProps<'_>,
) -> Response {
    let disabled = props.disabled;
    let response = base_menu_item(ui, theme, props);
    register_focusable(ui.ctx(), &response, disabled);
    response
}

pub fn dropdown_menu_checkbox_item(
    ui: &mut Ui,
    theme: &Theme,
    props: DropdownMenuCheckboxItemProps<'_>,
) -> Response {
    let disabled = props.disabled;
    let response = base_menu_checkbox_item(ui, theme, props);
    register_focusable(ui.ctx(), &response, disabled);
    response
}

pub fn dropdown_menu_radio_item(
    ui: &mut Ui,
    theme: &Theme,
    props: DropdownMenuRadioItemProps<'_>,
) -> Response {
    let disabled = props.disabled;
    let response = base_menu_radio_item(ui, theme, props);
    register_focusable(ui.ctx(), &response, disabled);
    response
}

pub fn dropdown_menu_sub<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: DropdownMenuSubProps<'_>,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    let submenu_id = ui.id().with("submenu").with(props.label);
    base_menu_sub(ui, theme, props, |ui| {
        let _guard = DropdownMenuScopeGuard::new(ui.ctx(), submenu_id);
        add_contents(ui)
    })
}

pub fn dropdown_menu_group<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    add_contents(ui)
}
