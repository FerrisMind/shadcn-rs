# iced-shadcn

<p align="center">
  <img src="https://raw.githubusercontent.com/FerrisMind/shadcn-rs/master/crates/iced-shadcn/assets/icons/shadcn-iced/icon.svg" width="200" alt="shadcn-iced logo" />
</p>

<p align="center">
  <strong>Shadcn-inspired component kit for iced</strong>
</p>

<p align="center">
  <a href="README.ru.md">Русский</a> · <a href="README.pt-BR.md">Português (Brasil)</a>
</p>

---

## Overview

`iced-shadcn` is planned as a set of components for [iced](https://github.com/iced-rs/iced) styled after [shadcn/ui](https://ui.shadcn.com).  
The goal is to provide a shared visual language and theme tokens that match the rest of the `shadcn-rs` ecosystem.

## Status

This crate is **under active development**. Public API, theming model, and component set are not stable yet and may change at any time.

**Coming soon**:

- Component catalog with parity to `egui-shadcn` where it makes sense
- Theming guide and tokens
- Usage examples and best practices

## Tabs

Minimal example using the new Tabs API:

```rust
use iced::widget::text;
use iced_shadcn::{
    tabs_content, tabs_contents, tabs_list, tabs_root, tabs_trigger, TabsHover, TabsListProps,
    TabsListVariant, TabsRootProps, Theme,
};

fn view<'a, Message: Clone + 'a>(theme: &Theme, active: &'a str) -> iced::Element<'a, Message> {
    let list = tabs_list(
        vec![
            tabs_trigger("account", "Account"),
            tabs_trigger("password", "Password"),
        ],
        active,
        None::<fn(String) -> Message>,
        TabsRootProps::new(),
        TabsListProps::new()
            .variant(TabsListVariant::Pill)
            .transparent_container(true)
            .hover(TabsHover::Soft)
            .hover_intensity(0.75),
        theme,
    );

    let content = tabs_contents(
        vec![
            tabs_content("account", text("Account content")),
            tabs_content("password", text("Password content")),
        ],
        active,
    );

    tabs_root(list, content)
}
```

Examples:
- `crates/iced-shadcn/examples/tabs-demo`
- `crates/iced-shadcn/examples/tabs-line`
- `crates/iced-shadcn/examples/tabs-size`
- `crates/iced-shadcn/examples/tabs-color`
- `crates/iced-shadcn/examples/tabs-disabled`

## Navigation Menu

Minimal example using Navigation Menu API:

```rust
use iced::widget::{column, text};
use iced_shadcn::{
    navigation_menu_content, navigation_menu_item, navigation_menu_link_item, navigation_menu_list,
    navigation_menu_root, navigation_menu_trigger, navigation_menu_viewport,
    NavigationMenuContentProps, NavigationMenuListProps, NavigationMenuProps, Theme,
};

fn view<'a, Message: Clone + 'a>(
    theme: &Theme,
    open: Option<&'a str>,
    on_open: Option<fn(String) -> Message>,
) -> iced::Element<'a, Message> {
    let items = navigation_menu_list(vec![
        navigation_menu_item(
            navigation_menu_trigger("home", "Home"),
            navigation_menu_content(column![text("Intro"), text("Installation")].spacing(6))
                .props(NavigationMenuContentProps::new().width(240.0)),
        ),
        navigation_menu_link_item("docs", text("Docs"), None::<Message>),
    ]);

    navigation_menu_root(
        items,
        open,
        on_open,
        NavigationMenuProps::new().viewport_component(navigation_menu_viewport()),
        NavigationMenuListProps::new(),
        theme,
    )
}
```

Example:
- `crates/iced-shadcn/examples/navigation-menu-demo`

## Breadcrumb

Minimal example using the Breadcrumb API:

```rust
use iced_shadcn::{
    BreadcrumbProps, Theme, breadcrumb, breadcrumb_item, breadcrumb_link, breadcrumb_list,
    breadcrumb_page, breadcrumb_separator,
};

fn view<'a, Message: Clone + 'a>(theme: &Theme) -> iced::Element<'a, Message> {
    breadcrumb(theme, BreadcrumbProps::new(), |ctx| {
        breadcrumb_list(
            ctx,
            vec![
                breadcrumb_item(ctx, vec![breadcrumb_link("Docs", None, ctx)]),
                breadcrumb_separator(ctx, None),
                breadcrumb_item(ctx, vec![breadcrumb_link("Components", None, ctx)]),
                breadcrumb_separator(ctx, None),
                breadcrumb_item(ctx, vec![breadcrumb_page("Breadcrumb", ctx)]),
            ],
        )
    })
}
```

Example:
- `crates/iced-shadcn/examples/breadcrumb`

## Resizable

Example demonstrating both splitter directions, live `Vec<f32>` state management, and handle options:

```rust
use iced_shadcn::{
    ResizableDirection, ResizableHandleProps, ResizablePanelGroupProps, ResizablePanelProps,
    Theme, resizable_handle, resizable_panel, resizable_panel_group,
};

// See crates/iced-shadcn/examples/resizable-demo/main.rs for a complete example.
```

Example:
- `crates/iced-shadcn/examples/resizable-demo`

## Image Cropper

Example demonstrating dialog-based crop flow, PNG export, round/rect preview, and optional picker integration:

```rust
use iced_shadcn::{
    ImageCropShape, ImageCropperAction, ImageCropperProps, ImageCropperState, Theme,
    image_cropper_cancel, image_cropper_canvas, image_cropper_controls, image_cropper_crop,
    image_cropper_dialog, image_cropper_preview, image_cropper_root, image_cropper_upload_trigger,
};

// See crates/iced-shadcn/examples/image-cropper-demo/main.rs for a complete example.
```

Example:
- `crates/iced-shadcn/examples/image-cropper-demo`

## License

MIT

---

**Inspired by** [shadcn/ui](https://ui.shadcn.com) · **Icons by** [Lucide](https://lucide.dev)
