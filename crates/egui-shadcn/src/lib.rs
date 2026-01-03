#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod accordion;
pub mod alert;
pub mod alert_dialog;
pub mod aspect_ratio;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod button_group;
pub mod calendar;
pub mod card;
pub mod carousel;
#[cfg(feature = "plot")]
pub mod chart;
pub mod checkbox;
pub mod collapsible;
pub mod combobox;
pub mod command;
pub mod context_menu;
pub mod data_table;
pub mod date_picker;
pub mod dialog;
pub mod dropdown_menu;
pub mod form;
pub mod hover_card;
pub mod icons;
pub mod input;
pub mod input_otp;
pub mod label;
pub mod menu_primitives;
pub mod navigation_menu;
pub mod pagination;
pub mod popover;
pub mod progress;
pub mod radio;
pub mod resizable;
pub mod scroll_area;
pub mod select;
pub mod separator;
pub mod sheet;
pub mod sidebar;
pub mod skeleton;
pub mod slider;
pub mod spinner;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod textarea;
pub mod theme;
pub mod toast;
pub mod toggle;
pub mod toggle_group;
pub mod tokens;
pub mod tooltip;
pub mod typography;

pub use accordion::{
    AccordionContext, AccordionItemContext, AccordionItemProps, AccordionProps, AccordionState,
    AccordionType, accordion, accordion_item,
};
pub use alert::{AlertProps, AlertVariant, alert};
pub use alert_dialog::{AlertDialogProps, AlertDialogResult, alert_dialog};
pub use aspect_ratio::{AspectRatioProps, aspect_ratio};
pub use avatar::{AvatarProps, AvatarSize, AvatarVariant, avatar};
pub use badge::{BadgeProps, BadgeSize, BadgeVariant, badge};
pub use breadcrumb::{
    BreadcrumbContext, BreadcrumbMetrics, BreadcrumbProps, BreadcrumbTokens, breadcrumb,
    breadcrumb_ellipsis, breadcrumb_item, breadcrumb_link, breadcrumb_list, breadcrumb_page,
    breadcrumb_separator,
};
pub use button::{
    Button, ButtonJustify, ButtonProps, ButtonRadius, ButtonSize, ButtonStyle, ButtonVariant,
    button,
};
pub use button_group::{ButtonGroup, ButtonGroupOrientation, button_group};
pub use calendar::{
    CalendarCaptionLayout, CalendarMode, CalendarProps, calendar, calendar_with_props,
};
pub use card::{CardProps, CardSize, CardTokens, CardVariant, card, card_tokens_with_options};
pub use carousel::{
    CarouselContentProps, CarouselContentResponse, CarouselContext, CarouselItemProps,
    CarouselItemResponse, CarouselOptions, CarouselOrientation, CarouselProps, CarouselResponse,
    carousel, carousel_content, carousel_item, carousel_next, carousel_previous,
};
#[cfg(feature = "plot")]
pub use chart::{
    BarChart, ChartIndicator, ChartLegend, ChartLegendItem, ChartProps, ChartResponse,
    ChartTooltip, ChartTooltipItem, LineChart, ShadcnChart, chart,
};
pub use checkbox::{
    CheckboxCycle, CheckboxOptions, CheckboxProps, CheckboxSize, CheckboxState, CheckboxVariant,
    checkbox, checkbox_state, checkbox_with_props,
};
pub use collapsible::{CollapsibleContentProps, CollapsibleContext, CollapsibleProps, collapsible};
pub use combobox::{ComboboxProps, ComboboxSize, combobox, combobox_with_props};
pub use command::{
    CommandContext, CommandDialogProps, CommandGroupProps, CommandInputProps, CommandItemProps,
    CommandListProps, CommandProps, OnCommandSelect, command, command_dialog, command_empty,
    command_group, command_input, command_item, command_list, command_separator, command_shortcut,
};
pub use context_menu::{
    ContextMenuCheckboxItemProps, ContextMenuItemProps, ContextMenuItemVariant,
    ContextMenuLabelProps, ContextMenuRadioGroupProps, ContextMenuRadioItemProps,
    ContextMenuSubProps, ContextMenuTokens, context_menu, context_menu_checkbox_item,
    context_menu_item, context_menu_label, context_menu_radio_group, context_menu_radio_item,
    context_menu_separator, context_menu_shortcut, context_menu_sub, context_menu_tokens,
};
pub use data_table::{
    DataTableAlign, DataTableColumn, DataTableProps, DataTableResponse, SortDirection, SortValue,
    data_table,
};
pub use date_picker::{
    DatePickerIconPosition, DatePickerProps, DateRange, DateRangePickerProps, date_picker,
    date_picker_with_props, date_range_picker, date_range_picker_with_props,
};
pub use dialog::{
    DialogAlign, DialogLayoutTokens, DialogProps, DialogSize, DialogTokens, compute_dialog_rect,
    dialog, dialog_layout_tokens, dialog_tokens_with_options,
};
pub use dropdown_menu::{
    DropdownMenuCheckboxItemProps, DropdownMenuItemProps, DropdownMenuItemVariant,
    DropdownMenuLabelProps, DropdownMenuProps, DropdownMenuRadioGroupProps,
    DropdownMenuRadioItemProps, DropdownMenuSubProps, DropdownMenuTokens, DropdownMenuTriggerProps,
    DropdownMenuTriggerResponse, dropdown_menu, dropdown_menu_checkbox_item, dropdown_menu_group,
    dropdown_menu_item, dropdown_menu_label, dropdown_menu_radio_group, dropdown_menu_radio_item,
    dropdown_menu_separator, dropdown_menu_shortcut, dropdown_menu_sub, dropdown_menu_tokens,
    dropdown_menu_trigger,
};
pub use form::{
    FieldState, FieldValue, FormControl, FormDescription, FormDescriptionProps, FormItem,
    FormItemContext, FormItemProps, FormLabel, FormLabelProps, FormMessage, FormMessageProps,
    FormState, ValidationMode, Validator, compose, email, form_control, form_description,
    form_item, form_label, form_message, max_length, min_length, none, pattern, required,
};
pub use hover_card::{HoverCardProps, hover_card, hover_card_content, hover_card_trigger};
pub use icons::{icon_calendar, icon_check, icon_chevrons_up_down};
pub use input::{
    Input, InputConfig, InputProps, InputRadius, InputSize, InputStyle, InputType, InputVariant,
    resolve_input_style, text_input, text_input_with_config, text_input_with_props,
};
pub use input_otp::{
    InputOTPContext, InputOTPProps, input_otp, input_otp_group, input_otp_separator,
    input_otp_slot, input_otp_slot_last,
};
pub use label::{Label, LabelProps, LabelVariant, label, label_with_props};
pub use menu_primitives::{
    MenuCheckboxItemProps, MenuItemProps, MenuItemVariant, MenuLabelProps, MenuRadioGroupProps,
    MenuRadioItemProps, MenuSubProps, MenuTokens, menu_checkbox_item, menu_item, menu_label,
    menu_radio_group, menu_radio_item, menu_separator, menu_shortcut, menu_sub, menu_tokens,
};
pub use navigation_menu::{
    NavigationMenuContentProps, NavigationMenuContext, NavigationMenuItemContext,
    NavigationMenuLinkProps, NavigationMenuLinkResponse, NavigationMenuLinkState,
    NavigationMenuProps, navigation_menu, navigation_menu_content, navigation_menu_item,
    navigation_menu_link, navigation_menu_list, navigation_menu_trigger,
};
pub use pagination::{
    OnPageChange, PaginationLinkProps, PaginationProps, pagination, pagination_content,
    pagination_ellipsis, pagination_item, pagination_link, pagination_next, pagination_previous,
};
pub use popover::{
    PopoverAlign, PopoverCollisionPadding, PopoverPlacement, PopoverPortalContainer, PopoverProps,
    PopoverSide, PopoverSticky, PopoverUpdatePositionStrategy, popover,
};
pub use progress::{ProgressProps, ProgressSize, ProgressVariant, progress};
pub use radio::{
    GridLayout, RadioCardVariant, RadioDirection, RadioGroup, RadioGroupProps, RadioOption,
    radio_group,
};
pub use resizable::{
    ResizableContext, ResizableDirection, ResizableHandleProps, ResizablePanelGroupProps,
    ResizablePanelProps, resizable_handle, resizable_panel, resizable_panel_group,
};
pub use scroll_area::{
    ScrollAreaColors, ScrollAreaDir, ScrollAreaProps, ScrollAreaRadius, ScrollAreaSize,
    ScrollAreaType, ScrollDirection, scroll_area,
};
pub use select::{
    ContentVariant, PopupPosition, SelectAlign, SelectAutoFocusEvent, SelectCollisionPadding,
    SelectDirection, SelectEscapeKeyDownEvent, SelectItem, SelectPointerDownOutsideEvent,
    SelectPortalContainer, SelectProps, SelectPropsSimple, SelectRadius, SelectSide, SelectSize,
    SelectSticky, SelectStyle, SelectUpdatePositionStrategy, TriggerVariant, select,
    select_with_items,
};
pub use separator::{SeparatorOrientation, SeparatorProps, SeparatorSize, separator};
pub use sheet::{
    SheetContext, SheetProps, SheetSide, sheet, sheet_content, sheet_description, sheet_footer,
    sheet_header, sheet_title, sheet_trigger,
};
pub use sidebar::{
    SidebarContext, SidebarGroupLabelProps, SidebarGroupProps, SidebarMenuButtonProps,
    SidebarMenuButtonSize, SidebarProps, SidebarProviderProps, SidebarResponse, SidebarSide,
    sidebar, sidebar_content, sidebar_footer, sidebar_group, sidebar_group_content,
    sidebar_group_label, sidebar_header, sidebar_menu, sidebar_menu_button, sidebar_menu_item,
    sidebar_provider, sidebar_trigger,
};
pub use skeleton::{SkeletonProps, skeleton, skeleton_text};
pub use slider::{
    SliderOrientation, SliderProps, SliderRadius, SliderSize, SliderVariant, slider,
    slider_with_props,
};
pub use spinner::{SpinnerProps, SpinnerSize, SpinnerVariant, spinner, spinner_with_content};
pub use switch::{
    OnCheckedChange, SwitchOptions, SwitchProps, switch, switch_with_options, switch_with_props,
};
pub use table::{
    TableCellProps, TableContext, TableProps, TableRowProps, TableRowResponse, TableSize,
    TableVariant, table, table_body, table_caption, table_cell, table_footer, table_head,
    table_header, table_row,
};
pub use tabs::{
    TabItem, TabsActivationMode, TabsContentForceMount, TabsDirection, TabsDirectionality,
    TabsJustify, TabsListLoop, TabsOrientation, TabsProps, TabsSize, TabsVariant, TabsWrap, tabs,
};
pub use textarea::{
    TextareaBuilder, TextareaBuilder as Textarea, TextareaProps, TextareaRadius, TextareaResize,
    TextareaSize, TextareaStyle, TextareaVariant, textarea_with_props,
};
pub use theme::{ControlVisuals, InputVisuals, Theme};
pub use toast::{Toast, ToastPosition, ToastPromise, ToastVariant, Toaster};
pub use toggle::toggle;
pub use toggle_group::{
    ToggleGroupContext, ToggleGroupProps, toggle_group, toggle_group_item, toggle_group_item_last,
};
pub use tokens::{
    ColorPalette, ControlSize, ControlVariant, DEFAULT_FOCUS, DEFAULT_MOTION, DEFAULT_RADIUS,
    FocusTokens, InputTokens, InputVariant as TokenInputVariant, MotionTokens, RadiusScale,
    ShadcnBaseColor, StateColors, SwitchSize, SwitchTokenOptions, SwitchTokens, SwitchVariant,
    ToggleVariant, VariantTokens, checkbox_metrics, checkbox_tokens, input_tokens, mix,
    switch_metrics, switch_metrics_for_control_size, switch_tokens, switch_tokens_with_options,
    toggle_button_tokens, toggle_metrics, variant_tokens,
};
pub use tooltip::{
    TooltipAlign, TooltipAnimationState, TooltipCollisionPadding, TooltipEscapeKeyDownEvent,
    TooltipOpenState, TooltipPointerDownOutsideEvent, TooltipPortalContainer, TooltipPosition,
    TooltipProps, TooltipSide, TooltipState, TooltipSticky, TooltipStyle,
    TooltipUpdatePositionStrategy, tooltip,
};
pub use typography::{
    BlockquoteProps, CodeProps, CodeVariant, HeadingAs, HeadingProps, KbdProps, LinkProps,
    LinkUnderline, ResolvedTextStyle, ShadcnTypographyVariant, TextAlign, TextAs, TextProps,
    TextTrim, TextWeight, TextWrap, TypographyColor, TypographyProps, blockquote, code, heading,
    kbd, link, resolve_shadcn_style, text, typography,
};
