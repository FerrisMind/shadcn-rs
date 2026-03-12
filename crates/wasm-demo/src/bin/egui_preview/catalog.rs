use lucide_icons::Icon;

pub const COMPONENT_SLUGS: [&str; 51] = [
    "accordion",
    "alert",
    "alert_dialog",
    "aspect_ratio",
    "avatar",
    "badge",
    "breadcrumb",
    "button",
    "button_group",
    "calendar",
    "card",
    "carousel",
    "chart",
    "checkbox",
    "collapsible",
    "combobox",
    "command",
    "context_menu",
    "data_table",
    "date_picker",
    "dialog",
    "dropdown_menu",
    "form",
    "hover_card",
    "input",
    "input_otp",
    "kbd",
    "label",
    "navigation_menu",
    "pagination",
    "popover",
    "progress",
    "radio",
    "resizable",
    "scroll_area",
    "select",
    "separator",
    "sheet",
    "sidebar",
    "skeleton",
    "slider",
    "spinner",
    "switch",
    "table",
    "tabs",
    "textarea",
    "toast",
    "toggle",
    "toggle_group",
    "tooltip",
    "typography",
];

#[cfg(target_arch = "wasm32")]
pub fn component_index_by_slug(slug: &str) -> Option<usize> {
    COMPONENT_SLUGS.iter().position(|entry| *entry == slug)
}

pub fn component_title(slug: &str) -> String {
    slug.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => {
                    format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn component_icon(slug: &str) -> Icon {
    match slug {
        "accordion" => Icon::ChevronDown,
        "alert" | "alert_dialog" => Icon::TriangleAlert,
        "aspect_ratio" => Icon::Scan,
        "avatar" => Icon::CircleUserRound,
        "badge" => Icon::BadgeCheck,
        "breadcrumb" => Icon::ChevronRight,
        "button" | "button_group" => Icon::MousePointer2,
        "calendar" | "date_picker" => Icon::Calendar,
        "card" => Icon::CreditCard,
        "carousel" => Icon::GalleryHorizontalEnd,
        "chart" => Icon::ChartColumn,
        "checkbox" => Icon::Check,
        "collapsible" => Icon::PanelTopClose,
        "combobox" | "command" => Icon::Search,
        "context_menu" | "dropdown_menu" | "navigation_menu" => Icon::Menu,
        "data_table" | "table" => Icon::TableProperties,
        "dialog" | "sheet" | "sidebar" => Icon::PanelsTopLeft,
        "form" => Icon::FileText,
        "hover_card" => Icon::MousePointerClick,
        "input" | "input_otp" | "textarea" => Icon::TextCursorInput,
        "kbd" => Icon::Keyboard,
        "label" => Icon::Tag,
        "pagination" => Icon::ChevronsLeftRight,
        "popover" | "tooltip" => Icon::MessageCircleMore,
        "progress" | "spinner" => Icon::LoaderCircle,
        "radio" => Icon::CircleDot,
        "resizable" => Icon::PanelLeftClose,
        "scroll_area" => Icon::ScrollText,
        "select" => Icon::ListFilter,
        "separator" => Icon::Minus,
        "skeleton" => Icon::Bone,
        "slider" => Icon::SlidersHorizontal,
        "switch" | "toggle" | "toggle_group" => Icon::ToggleLeft,
        "tabs" => Icon::Rows3,
        "toast" => Icon::Bell,
        "typography" => Icon::Type,
        _ => Icon::Component,
    }
}

pub fn component_code(slug: &str) -> &'static str {
    match slug {
        "accordion" => include_str!("../../../../egui-shadcn/examples/accordion/main.rs"),
        "alert" => include_str!("../../../../egui-shadcn/examples/alert/main.rs"),
        "alert_dialog" => include_str!("../../../../egui-shadcn/examples/alert_dialog/main.rs"),
        "aspect_ratio" => include_str!("../../../../egui-shadcn/examples/aspect_ratio/main.rs"),
        "avatar" => include_str!("../../../../egui-shadcn/examples/avatar/main.rs"),
        "badge" => include_str!("../../../../egui-shadcn/examples/badge/main.rs"),
        "breadcrumb" => include_str!("../../../../egui-shadcn/examples/breadcrumb/main.rs"),
        "button" => include_str!("../../../../egui-shadcn/examples/button/main.rs"),
        "button_group" => include_str!("../../../../egui-shadcn/examples/button_group/main.rs"),
        "calendar" => include_str!("../../../../egui-shadcn/examples/calendar/main.rs"),
        "card" => include_str!("../../../../egui-shadcn/examples/card/main.rs"),
        "carousel" => include_str!("../../../../egui-shadcn/examples/carousel/main.rs"),
        "chart" => include_str!("../../../../egui-shadcn/examples/chart/main.rs"),
        "checkbox" => include_str!("../../../../egui-shadcn/examples/checkbox/main.rs"),
        "collapsible" => include_str!("../../../../egui-shadcn/examples/collapsible/main.rs"),
        "combobox" => include_str!("../../../../egui-shadcn/examples/combobox/main.rs"),
        "command" => include_str!("../../../../egui-shadcn/examples/command/main.rs"),
        "context_menu" => include_str!("../../../../egui-shadcn/examples/context_menu/main.rs"),
        "data_table" => include_str!("../../../../egui-shadcn/examples/data_table/main.rs"),
        "date_picker" => include_str!("../../../../egui-shadcn/examples/date_picker/main.rs"),
        "dialog" => include_str!("../../../../egui-shadcn/examples/dialog/main.rs"),
        "dropdown_menu" => include_str!("../../../../egui-shadcn/examples/dropdown_menu/main.rs"),
        "form" => include_str!("../../../../egui-shadcn/examples/form/main.rs"),
        "hover_card" => include_str!("../../../../egui-shadcn/examples/hover_card/main.rs"),
        "input" => include_str!("../../../../egui-shadcn/examples/input/main.rs"),
        "input_otp" => include_str!("../../../../egui-shadcn/examples/input_otp/main.rs"),
        "kbd" => include_str!("../../../../egui-shadcn/examples/kbd/main.rs"),
        "label" => include_str!("../../../../egui-shadcn/examples/label/main.rs"),
        "navigation_menu" => {
            include_str!("../../../../egui-shadcn/examples/navigation_menu/main.rs")
        }
        "pagination" => include_str!("../../../../egui-shadcn/examples/pagination/main.rs"),
        "popover" => include_str!("../../../../egui-shadcn/examples/popover/main.rs"),
        "progress" => include_str!("../../../../egui-shadcn/examples/progress/main.rs"),
        "radio" => include_str!("../../../../egui-shadcn/examples/radio/main.rs"),
        "resizable" => include_str!("../../../../egui-shadcn/examples/resizable/main.rs"),
        "scroll_area" => include_str!("../../../../egui-shadcn/examples/scroll_area/main.rs"),
        "select" => include_str!("../../../../egui-shadcn/examples/select/main.rs"),
        "separator" => include_str!("../../../../egui-shadcn/examples/separator/main.rs"),
        "sheet" => include_str!("../../../../egui-shadcn/examples/sheet/main.rs"),
        "sidebar" => include_str!("../../../../egui-shadcn/examples/sidebar/main.rs"),
        "skeleton" => include_str!("../../../../egui-shadcn/examples/skeleton/main.rs"),
        "slider" => include_str!("../../../../egui-shadcn/examples/slider/main.rs"),
        "spinner" => include_str!("../../../../egui-shadcn/examples/spinner/main.rs"),
        "switch" => include_str!("../../../../egui-shadcn/examples/switch/main.rs"),
        "table" => include_str!("../../../../egui-shadcn/examples/table/main.rs"),
        "tabs" => include_str!("../../../../egui-shadcn/examples/tabs/main.rs"),
        "textarea" => include_str!("../../../../egui-shadcn/examples/textarea/main.rs"),
        "toast" => include_str!("../../../../egui-shadcn/examples/toast/main.rs"),
        "toggle" => include_str!("../../../../egui-shadcn/examples/toggle/main.rs"),
        "toggle_group" => include_str!("../../../../egui-shadcn/examples/toggle_group/main.rs"),
        "tooltip" => include_str!("../../../../egui-shadcn/examples/tooltip/main.rs"),
        "typography" => include_str!("../../../../egui-shadcn/examples/typography/main.rs"),
        _ => "",
    }
}
