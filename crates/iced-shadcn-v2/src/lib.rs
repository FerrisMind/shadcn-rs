//! Builder-first shadcn-inspired component kit for iced — v2 API.
//!
//! Successor of `iced-shadcn::new_api`. Theme tokens come from
//! [`shadcn_common`]; iced styles are built directly from `twill-core`
//! tokens, without the `twill` style-composition facade. The crate
//! intentionally does not depend on `iced-shadcn` v1.
//!
//! # Theming model
//!
//! Store a [`Theme`] in app state and pass `&Theme` into components. Style
//! packs (`StyleId::Vega`, …) live on [`Theme`]; overrides via `Theme::with_*`
//! beat pack defaults; per-widget knobs (e.g. [`Button::color`]) beat that
//! theme for one control. Two looks on one screen ⇒ two [`Theme`] values (or
//! one theme + different button variants). See the crate README “Theming”
//! section for the three common patterns.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Button, ButtonVariant, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Save,
//! }
//!
//! fn view(theme: &Theme) -> Element<'_, Message> {
//!     Button::text("Save", theme)
//!         .variant(ButtonVariant::Default)
//!         .on_press(Message::Save)
//!         .into()
//! }
//! ```
//!
//! # Feature flags
//!
//! - `wgpu` *(default)* — forwards the GPU renderer backend to iced.
//! - `tiny-skia` *(default)* — forwards the software renderer backend (plus
//!   the X11/Wayland integrations it needs on Linux).
//! - `serde` — derives `serde::Serialize` / `serde::Deserialize` for the
//!   configuration enums (variants, sizes, radii, orientations, states), so
//!   app settings that reference them can be persisted. The one exception is
//!   [`SkeletonFill`], whose variants wrap non-serializable foreign types
//!   (`iced` colors and `twill-core` semantic slots). Disabled by default.
//! - `rfd` — enables [`file_drop_zone_pick_files`] for native click-to-pick
//!   file dialogs used by [`FileDropZone`].
//!
//! The library depends on the granular `iced_core` / `iced_widget` crates
//! only — not the full `iced` facade with its window/runtime stack — so use
//! `default-features = false` and pick a single renderer (or none, if the
//! app depends on `iced` directly) to trim the dependency tree.
//!
//! # Panics
//!
//! The public builder API never panics: invalid numeric inputs are clamped
//! or normalized, and unsupported values (e.g. `auto` padding) are reported
//! through `*BuildError` results instead. Rendering internals `expect` on
//! layout invariants guaranteed by iced (every custom widget lays out the
//! child it created); those panics are unreachable unless iced itself
//! violates its layout contract.
//!
//! # Thread safety
//!
//! [`Theme`], [`Palette`], every configuration enum, and every error type are
//! `Send + Sync`. Widget builders (e.g. [`Button`]) borrow the theme and may
//! hold non-`Send` style closures; build them inside `view` instead of
//! sharing them across threads.

#![deny(missing_docs)]
#![warn(missing_debug_implementations)]

pub mod components;
mod display;
pub(crate) mod floating_surface;
pub mod fonts;
pub(crate) mod iced_compat;
pub mod recipes;
pub mod theme;

/// Backwards-compatible access to the accordion component.
pub use components::accordion;
/// Backwards-compatible access to the alert component.
pub use components::alert;
/// Backwards-compatible access to the alert-dialog component.
pub use components::alert_dialog;
/// Backwards-compatible access to the aspect-ratio component.
pub use components::aspect_ratio;
/// Backwards-compatible access to the avatar component.
pub use components::avatar;
/// Backwards-compatible access to the badge component.
pub use components::badge;
/// Backwards-compatible access to the breadcrumb component.
pub use components::breadcrumb;
/// Backwards-compatible access to the button component.
pub use components::button;
/// Backwards-compatible access to the button-group component.
pub use components::button_group;
/// Backwards-compatible access to the calendar component.
pub use components::calendar;
/// Backwards-compatible access to the card component.
pub use components::card;
/// Backwards-compatible access to the carousel component.
pub use components::carousel;
/// Backwards-compatible access to the chart component.
pub use components::chart;
/// Backwards-compatible access to the checkbox component.
pub use components::checkbox;
/// Backwards-compatible access to the code component.
pub use components::code;
/// Backwards-compatible access to the collapsible component.
pub use components::collapsible;
/// Backwards-compatible access to the combobox component.
pub use components::combobox;
/// Backwards-compatible access to the command component.
pub use components::command;
/// Backwards-compatible access to the context-menu component.
pub use components::context_menu;
/// Backwards-compatible access to the copy-button component.
pub use components::copy_button;
/// Backwards-compatible access to the data-table component.
pub use components::data_table;
/// Backwards-compatible access to the date-picker component.
pub use components::date_picker;
/// Backwards-compatible access to the dialog component.
pub use components::dialog;
/// Backwards-compatible access to the drawer component.
pub use components::drawer;
/// Backwards-compatible access to the dropdown-menu component.
pub use components::dropdown_menu;
/// Backwards-compatible access to the emoji-picker component.
pub use components::emoji_picker;
/// Backwards-compatible access to the empty-state component.
pub use components::empty;
/// Backwards-compatible access to the field component.
pub use components::field;
/// Backwards-compatible access to the file-drop-zone component.
pub use components::file_drop_zone;
/// Backwards-compatible access to the form component.
pub use components::form;
/// Backwards-compatible access to the hover-card component.
pub use components::hover_card;
/// Backwards-compatible access to the input component.
pub use components::input;
/// Backwards-compatible access to the input-group component.
pub use components::input_group;
/// Backwards-compatible access to the input-otp component.
pub use components::input_otp;
/// Backwards-compatible access to the item component.
pub use components::item;
/// Backwards-compatible access to the kbd component.
pub use components::kbd;
/// Backwards-compatible access to the label component.
pub use components::label;
/// Backwards-compatible access to the menubar component.
pub use components::menubar;
/// Backwards-compatible access to the meter component.
pub use components::meter;
/// Backwards-compatible access to the native-select component.
pub use components::native_select;
/// Backwards-compatible access to the navigation-menu component.
pub use components::navigation_menu;
/// Backwards-compatible access to the pagination component.
pub use components::pagination;
/// Backwards-compatible access to the password component.
pub use components::password;
/// Backwards-compatible access to the phone-input component.
pub use components::phone_input;
/// Backwards-compatible access to the PMCommand component.
pub use components::pm_command;
/// Backwards-compatible access to the popover component.
pub use components::popover;
/// Backwards-compatible access to the progress component.
pub use components::progress;
/// Backwards-compatible access to the radio-group component.
pub use components::radio_group;
/// Backwards-compatible access to the range-calendar component.
pub use components::range_calendar;
/// Backwards-compatible access to the rename component.
pub use components::rename;
/// Backwards-compatible access to the resizable component.
pub use components::resizable;
/// Backwards-compatible access to the scroll-area component.
pub use components::scroll_area;
/// Backwards-compatible access to the select component.
pub use components::select;
/// Backwards-compatible access to the separator component.
pub use components::separator;
/// Backwards-compatible access to the sheet component.
pub use components::sheet;
/// Backwards-compatible access to the sidebar component.
pub use components::sidebar;
/// Backwards-compatible access to the skeleton component.
pub use components::skeleton;
/// Backwards-compatible access to the slider component.
pub use components::slider;
/// Backwards-compatible access to the snippet component.
pub use components::snippet;
/// Backwards-compatible access to the sonner toast component.
pub use components::sonner;
/// Backwards-compatible access to the spinner component.
pub use components::spinner;
/// Backwards-compatible access to the star-rating component.
pub use components::star_rating;
/// Backwards-compatible access to the stepper component.
pub use components::stepper;
/// Backwards-compatible access to the switch component.
pub use components::switch;
/// Backwards-compatible access to the table component.
pub use components::table;
/// Backwards-compatible access to the tabs component.
pub use components::tabs;
/// Backwards-compatible access to the textarea component.
pub use components::textarea;
/// Backwards-compatible access to the toggle component.
pub use components::toggle;
/// Backwards-compatible access to the toggle-group component.
pub use components::toggle_group;
/// Backwards-compatible access to the tooltip component.
pub use components::tooltip;
/// Backwards-compatible access to the typography component.
pub use components::typography;

pub use components::accordion::{
    Accordion, AccordionBuildError, AccordionContent, AccordionHeaderLevel, AccordionItem,
    AccordionLoop, AccordionMode, AccordionOrientation, AccordionSelection, AccordionTrigger,
    AccordionType, AccordionValue, accordion,
};
pub use components::alert::{
    Alert, AlertAction, AlertDescription, AlertRadius, AlertTitle, AlertVariant,
};
pub use components::alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogMedia, AlertDialogSize, AlertDialogStyle, AlertDialogTitle,
};
pub use components::aspect_ratio::{AspectRatio, MIN_ASPECT_RATIO, aspect_ratio};
pub use components::avatar::{
    Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage, AvatarRadius,
    AvatarSize,
};
pub use components::badge::{Badge, BadgeBuildError, BadgeRadius, BadgeVariant};
pub use components::breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbEntry, BreadcrumbItem, BreadcrumbLink,
    BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
pub use components::button::{Button, ButtonBuildError, ButtonRadius, ButtonSize, ButtonVariant};
pub use components::button_group::{
    ButtonGroup, ButtonGroupItem, ButtonGroupOrientation, ButtonGroupText,
};
pub use components::calendar::{
    Calendar, CalendarCaptionLayout, CalendarMonthFormat, CalendarSelection, CalendarWeekdayFormat,
    CalendarYearFormat, calendar,
};
pub use components::card::{
    Card, CardAction, CardBorder, CardContent, CardDescription, CardFooter, CardFooterAlignment,
    CardFooterDirection, CardHeader, CardRadius, CardSize, CardTitle,
};
pub use components::carousel::{
    Carousel, CarouselAlign, CarouselItem, CarouselNext, CarouselOrientation, CarouselPrevious,
    carousel,
};
pub use components::chart::{
    CHART_DEFAULT_HEIGHT_PX, Chart, ChartAxis, ChartColor, ChartCurve, ChartIndicator, ChartKind,
    ChartSeries, chart,
};
pub use components::checkbox::{
    Checkbox, CheckboxConfig, CheckboxSize, CheckboxState, CheckboxVariant,
};
pub use components::code::{Code, CodeCopyButton, CodeOverflow, CodeVariant};
pub use components::collapsible::{
    Collapsible, CollapsibleAlignment, CollapsibleBuildError, CollapsibleContent,
    CollapsibleEasing, CollapsibleIndicator, CollapsibleIndicatorPlacement, CollapsibleOrientation,
    CollapsibleState, CollapsibleTrigger, collapsible,
};
pub use components::combobox::{
    Combobox, ComboboxEmpty, ComboboxEntry, ComboboxGroup, ComboboxItem, ComboboxLoading,
    ComboboxRadius, ComboboxSelection, ComboboxSize, ComboboxType, combobox,
};
pub use components::command::{
    Command, CommandDialog, CommandEmpty, CommandEntry, CommandGlyph, CommandGroup, CommandItem,
    CommandLoading, CommandRadius, CommandStyle, command, command_dialog,
};
pub use components::context_menu::{
    ContextMenu, ContextMenuCheckboxItem, ContextMenuContentStyle, ContextMenuItem,
    ContextMenuItemVariant, ContextMenuLabel, ContextMenuRadioItem, ContextMenuSub, context_menu,
};
pub use components::copy_button::{
    CopyButton, CopyButtonAction, CopyButtonState, CopyButtonStatus, CopyButtonUpdate,
    copy_button_reduce,
};
pub use components::data_table::{DataTable, data_table};
pub use components::date_picker::{
    DatePicker, DatePickerIconPosition, DateRangePicker, date_picker, date_range_picker,
};
pub use components::dialog::{
    Dialog, DialogDescription, DialogFooter, DialogHeader, DialogStyle, DialogTitle,
};
pub use components::drawer::{
    Drawer, DrawerBody, DrawerDescription, DrawerDirection, DrawerFooter, DrawerHeader,
    DrawerStyle, DrawerTitle,
};
pub use components::dropdown_menu::{
    DropdownMenu, DropdownMenuCheckboxItem, DropdownMenuContentStyle, DropdownMenuItem,
    DropdownMenuItemVariant, DropdownMenuLabel, DropdownMenuRadioItem, DropdownMenuSub,
    dropdown_menu, dropdown_menu_content_style,
};
pub use components::emoji_picker::{
    EmojiPicker, EmojiPickerCategory, EmojiPickerData, EmojiPickerFooter, EmojiPickerList,
    EmojiPickerRecent, EmojiPickerRecents, EmojiPickerSearch, EmojiPickerSkin,
    EmojiPickerSkinToneSelector, EmojiPickerViewport, SelectedEmoji,
};
pub use components::empty::{
    Empty, EmptyBorderStyle, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia,
    EmptyMediaVariant, EmptyRadius, EmptyTitle,
};
pub use components::field::{
    DEFAULT_FIELD_RESPONSIVE_BREAKPOINT, Field, FieldContent, FieldDescription, FieldError,
    FieldErrorItem, FieldGroup, FieldLabel, FieldLegend, FieldLegendVariant, FieldOrientation,
    FieldSeparator, FieldSet, FieldTitle,
};
#[cfg(feature = "rfd")]
pub use components::file_drop_zone::pick_files as file_drop_zone_pick_files;
pub use components::file_drop_zone::{
    FileDropZone, FileDropZoneAction, FileDropZoneFile, FileDropZoneMode, FileDropZoneState,
    FileDropZoneVariant, file_drop_zone, load_files as file_drop_zone_load_files,
    partition_paths as file_drop_zone_partition_paths,
};
pub use components::form::{
    Form, FormButton, FormControl, FormControlExt, FormControlProps, FormDescription,
    FormElementField, FormField, FormFieldContent, FormFieldErrors, FormFieldGroup, FormFieldSet,
    FormFieldTitle, FormFieldset, FormLabel, FormLegend, FormLegendVariant,
};
pub use components::hover_card::{HoverCard, HoverCardAlign, HoverCardSide, HoverCardStyle};
pub use components::input::{Input, InputBuildError, InputRadius, InputSize, input};
pub use components::input_group::{
    InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupAddonProps, InputGroupButton,
    InputGroupButtonProps, InputGroupButtonSize, InputGroupInput, InputGroupInputProps,
    InputGroupItem, InputGroupProps, InputGroupRadius, InputGroupText, InputGroupTextarea,
    InputGroupTextareaProps, InputGroupTextareaResize, input_group_addon, input_group_button,
    input_group_control, input_group_input, input_group_text, input_group_textarea,
    input_group_textarea_apply_action,
};
pub use components::input_otp::{
    InputOtp, InputOtpPattern, InputOtpRadius, InputOtpStatus, InputOtpStyle, input_otp,
};
pub use components::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemHeader, ItemMedia,
    ItemMediaVariant, ItemRadius, ItemSeparator, ItemSize, ItemTitle, ItemVariant,
};
pub use components::kbd::{Kbd, KbdBuildError, KbdGroup, KbdRadius, KbdSurface};
pub use components::label::{Label, LabelContext};
pub use components::menubar::{
    Menubar, MenubarBarStyle, MenubarCheckboxItem, MenubarContentStyle, MenubarItem,
    MenubarItemVariant, MenubarLabel, MenubarMenu, MenubarRadioItem, MenubarSub, menubar,
};
pub use components::meter::{Meter, MeterOrientation, MeterRadius, MeterSize, MeterState, meter};
pub use components::native_select::{
    NativeSelect, NativeSelectGroup, NativeSelectOption, NativeSelectRadius, NativeSelectSize,
    NativeSelectStatus, NativeSelectStyle, native_select,
};
pub use components::navigation_menu::{
    NavigationMenu, NavigationMenuAlign, NavigationMenuContent, NavigationMenuContentProps,
    NavigationMenuItem, NavigationMenuJustify, NavigationMenuLinkProps, NavigationMenuLinkVariant,
    NavigationMenuListProps, NavigationMenuOrientation, NavigationMenuProps, NavigationMenuSide,
    NavigationMenuSize, NavigationMenuTiming, NavigationMenuTriggerBuilder,
    NavigationMenuTriggerContent, NavigationMenuViewportStyle, NavigationMenuWrap, navigation_menu,
    navigation_menu_content, navigation_menu_link, navigation_menu_trigger_style,
};
pub use components::pagination::{
    Pagination, PaginationEllipsis, PaginationItem, PaginationLink, PaginationNext,
    PaginationPrevious, pagination,
};
pub use components::password::{
    Password, PasswordActionSlot, PasswordCopy, PasswordInput, PasswordStrength,
    PasswordToggleVisibility, password,
};
pub use components::phone_input::{PhoneInput, PhoneInputChange, phone_input};
pub use components::pm_command::{
    PMCommand, PmCommand, PmCommandAgent, PmCommandRadius, PmCommandResolution, PmCommandVariant,
    PmCommandVerb, resolve_pm_command, try_resolve_pm_command,
};
pub use components::popover::{
    Popover, PopoverAlign, PopoverDescription, PopoverHeader, PopoverSide, PopoverStyle,
    PopoverTitle,
};
pub use components::progress::{
    Progress, ProgressOrientation, ProgressRadius, ProgressSize, ProgressState, ProgressVariant,
    progress,
};
pub use components::radio_group::{
    RadioGroup, RadioGroupItem, RadioGroupOrientation, RadioGroupRadius, RadioGroupSize,
    RadioGroupStatus, RadioGroupStyle, radio_group,
};
pub use components::range_calendar::{DateRange, RangeCalendar, RangeDayPosition, range_calendar};
pub use components::rename::{
    Rename, RenameAction, RenameActionHandler, RenameBlurBehavior, RenameButtonProps,
    RenameContext, RenameFallbackSelectionBehavior, RenameInputTag, RenameMode,
    RenameProviderProps, RenameRootProps, RenameSelectionRequest, RenameState, RenameUpdate,
    rename_apply_action, rename_cancel, rename_edit, rename_provider, rename_root, rename_save,
};
pub use components::resizable::{
    ResizableBuildError, ResizableDirection, ResizableHandle, ResizableLayout, ResizablePane,
    ResizablePaneGroup, ResizableRadius, resizable_pane_group,
};
pub use components::scroll_area::{
    ScrollArea, ScrollAreaAnchor, ScrollAreaBuildError, ScrollAreaOrientation, ScrollAreaRadius,
    ScrollAreaScrollbar,
};
pub use components::select::{
    Select, SelectContentStyle, SelectGroup, SelectItem, SelectRadius, SelectSelection, SelectSize,
    SelectStatus, SelectTriggerStyle, SelectType, select,
};
pub use components::separator::{Separator, SeparatorOrientation, separator};
pub use components::sheet::{
    Sheet, SheetBody, SheetDescription, SheetFooter, SheetHeader, SheetSide, SheetStyle, SheetTitle,
};
pub use components::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarController, SidebarDisplayState,
    SidebarFooter, SidebarGroup, SidebarGroupAction, SidebarGroupContent, SidebarGroupLabel,
    SidebarHeader, SidebarInput, SidebarInset, SidebarMenu, SidebarMenuAction, SidebarMenuBadge,
    SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuButtonVariant, SidebarMenuItem,
    SidebarMenuSkeleton, SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubButtonSize,
    SidebarMenuSubItem, SidebarProvider, SidebarRail, SidebarSeparator, SidebarSide, SidebarStyle,
    SidebarTrigger, SidebarVariant,
};
pub use components::skeleton::{
    Skeleton, SkeletonAnimation, SkeletonFill, SkeletonRadius, SkeletonShape,
};
pub use components::slider::{
    Slider, SliderOrientation, SliderRadius, SliderState, SliderStatus, SliderStyle, slider,
};
pub use components::snippet::{Snippet, SnippetRadius, SnippetText, SnippetVariant};
pub use components::sonner::{
    SonnerToast, Toast, ToastAction, ToastCallback, ToastId, ToastOptions, ToastPosition,
    ToastPromise, ToastStyle, ToastType, Toaster, active_toast_count, dismiss_all_toasts,
    dismiss_toast, toast, toast_error, toast_immediate, toast_info, toast_loading, toast_promise,
    toast_success, toast_warning, toast_with_id, update_toast,
};
pub use components::spinner::{Spinner, SpinnerSize, SpinnerVariant, spinner};
pub use components::star_rating::{
    StarRating, StarRatingOrientation, StarRatingSize, StarRatingState, StarRatingStatus,
    StarRatingStyle, star_rating,
};
pub use components::stepper::{
    Stepper, StepperDescription, StepperIndicator, StepperItem, StepperItemState, StepperNav,
    StepperNext, StepperOrientation, StepperPrevious, StepperSeparator, StepperTitle,
    StepperTrigger, stepper, stepper_description, stepper_indicator, stepper_item, stepper_next,
    stepper_previous, stepper_separator, stepper_title, stepper_trigger,
};
pub use components::switch::{
    Switch, SwitchRadius, SwitchSize, SwitchState, SwitchStatus, SwitchStyle, switch,
};
pub use components::table::{
    Table, TableBody, TableCaption, TableCell, TableCellAlignment, TableFooter, TableHead,
    TableHeader, TableRow, TableRowCell, TableSection,
};
pub use components::tabs::{
    Tabs, TabsActivationMode, TabsContent, TabsDirection, TabsHover, TabsJustify, TabsList,
    TabsListLoop, TabsListVariant, TabsOrientation, TabsSize, TabsTrigger, TabsWrap, tabs,
    tabs_content, tabs_trigger,
};
pub use components::textarea::{
    Textarea, TextareaRadius, TextareaResize, TextareaSize, textarea, textarea_apply_action,
};
pub use components::toggle::{Toggle, ToggleRadius, ToggleSize, ToggleVariant};
pub use components::toggle_group::{
    ToggleGroup, ToggleGroupItem, ToggleGroupMode, ToggleGroupOrientation, ToggleGroupRadius,
    ToggleGroupSelection, ToggleGroupSize, ToggleGroupType, ToggleGroupValue, ToggleGroupVariant,
};
pub use components::tooltip::{Tooltip, TooltipAlign, TooltipSide, TooltipStyle};
pub use components::tree_view::{
    TreeIconRenderer, TreeNavigationPolicy, TreeScrollbarPolicy, TreeSelectionMode, TreeView,
    TreeViewBuildError, TreeViewMeasurement, TreeViewRenderMode, tree_view,
};
pub use components::typography::{Typography, TypographyList, TypographyTable, TypographyVariant};
pub use fonts::{ALL_FACES, iced_font};
pub use theme::{Palette, Theme};

pub use shadcn_common::{
    ACCEPT_AUDIO, ACCEPT_IMAGE, ACCEPT_VIDEO, AccentColor, BYTE, BaseColor, ComponentRadius,
    ControlSize, CountryCode, DEFAULT_TRIGGER_LABEL, DateParts, DetailedPhoneValue, Direction,
    FileCandidate, FileDropZoneConfig, FileRejectedReason, FloatingPadding, FloatingSticky,
    FolderState, FontHeading, FontId, FontPack, FontWeight, GIGABYTE, KILOBYTE, MEGABYTE,
    MenuActivateKind, MenuItemVariant, MeterConfig, MeterFillTone, Orientation, PasswordAction,
    PasswordScore, PasswordState, PasswordStrength as PasswordStrengthResult, PhoneCountry,
    PhoneInputOptions, RadiusId, RadiusScale, ResolvedTheme, SelectMode, StarRatingConfig,
    StarRatingItem, StarRatingItemState, StarRatingKey, StarRatingKeyEffect, StyleId, StylePack,
    ThemeMode, TreeFile, TreeFolder, TreeIconKey, TreeNode, TreeNodeId, TreeNodeIdError,
    TreeOrdering, TreeValidationError, TreeViewAction, TreeViewState, TypeRecipe, VisibleTreeNode,
    apply_country_change, apply_input_change, clamp_meter_value, default_country_order,
    display_size, estimate_password_strength, file_drop_zone_can_upload,
    file_drop_zone_default_hint, file_drop_zone_recipe, flatten_visible, flatten_visible_ordered,
    format_phone_value, guess_mime, is_phone_valid, matches_sidebar_shortcut, meter_fill_tone,
    meter_ratio, meter_recipe, meter_value_label, parse_phone_input, password_reduce,
    phone_countries, phone_country, phone_input_recipe, should_accept_file, sort_countries,
    truncate_tree_label, validate_tree,
};

/// Semantic color slots resolved by [`Theme::semantic_color`].
pub use twill_core::prelude::theme::SemanticColor;

/// Spacing tokens accepted by [`Button::padding`].
pub use twill_core::prelude::{Padding, PaddingValue, PaddingVar, Spacing};
