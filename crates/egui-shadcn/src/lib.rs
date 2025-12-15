//! Shadcn-inspired UI компоненты для `egui`.
//!
//! Предоставляет набор контролов (кнопки, формы, оверлеи, layout) с API, выровненным на Radix
//! и палитрой shadcn. Все публичные элементы документированы в духе rustdoc, примеры пригодны
//! к копированию.
//!
//! # Пример
//! ```rust
//! use egui::Ui;
//! use egui_shadcn::{button, ControlSize, ControlVariant, Theme};
//!
//! fn ui_example(ui: &mut Ui, theme: &Theme) {
//!     button(
//!         ui,
//!         theme,
//!         "Save",
//!         ControlVariant::Primary,
//!         ControlSize::Md,
//!         true,
//!     );
//! }
//! ```
//!
//! Дополнительные примеры см. в `examples/` и `crates/egui-shadcn/README.md`.

pub mod button;
pub mod card;
pub mod checkbox;
pub mod dialog;
pub mod input;
pub mod label;
pub mod popover;
pub mod radio;
pub mod scroll_area;
pub mod select;
pub mod separator;
pub mod switch;
pub mod tabs;
pub mod textarea;
pub mod theme;
pub mod toggle;
pub mod tokens;
pub mod tooltip;

pub use button::{
    Button, ButtonProps, ButtonRadius, ButtonSize, ButtonStyle, ButtonVariant, button,
};
pub use card::{CardProps, CardSize, CardTokens, CardVariant, card, card_tokens_with_options};
pub use checkbox::{
    CheckboxCycle, CheckboxOptions, CheckboxProps, CheckboxSize, CheckboxState, CheckboxVariant,
    checkbox, checkbox_state, checkbox_with_props,
};
pub use dialog::{
    DialogAlign, DialogLayoutTokens, DialogProps, DialogSize, DialogTokens, compute_dialog_rect,
    dialog, dialog_layout_tokens, dialog_tokens_with_options,
};
pub use input::{
    Input, InputConfig, InputProps, InputRadius, InputSize, InputStyle, InputType, InputVariant,
    resolve_input_style, text_input, text_input_with_config, text_input_with_props,
};
pub use label::{Label, LabelProps, LabelVariant, label, label_with_props};
pub use popover::{
    PopoverAlign, PopoverCollisionPadding, PopoverPlacement, PopoverPortalContainer, PopoverProps,
    PopoverSide, PopoverSticky, PopoverUpdatePositionStrategy, popover,
};
pub use radio::{
    GridLayout, RadioCardVariant, RadioDirection, RadioGroup, RadioGroupProps, RadioOption,
    radio_group,
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
pub use switch::{
    OnCheckedChange, SwitchOptions, SwitchProps, switch, switch_with_options, switch_with_props,
};
pub use tabs::{
    TabItem, TabsActivationMode, TabsContentForceMount, TabsDirection, TabsDirectionality,
    TabsJustify, TabsListLoop, TabsOrientation, TabsProps, TabsSize, TabsVariant, TabsWrap, tabs,
};
pub use textarea::{
    TextareaBuilder, TextareaBuilder as Textarea, TextareaProps, TextareaRadius, TextareaSize,
    TextareaStyle, TextareaVariant, textarea_with_props,
};
pub use theme::{ControlVisuals, InputVisuals, Theme};
pub use toggle::toggle;
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
