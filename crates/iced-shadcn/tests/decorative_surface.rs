use iced::Element;
use iced::widget::text;
use iced_shadcn::new_api::{Button, ButtonVariant};
use iced_shadcn::{DecorativeSurfaceProps, Theme, decorative_surface};
use twill::prelude::{SemanticThemeVars, ThemeVariant};

#[test]
fn decorative_surface_public_api_is_available() {
    let theme = Theme::from_semantic_theme(SemanticThemeVars::shadcn_neutral(), ThemeVariant::Dark);
    let props = DecorativeSurfaceProps::new()
        .themed()
        .clip(true)
        .border_width(2.0);

    let _surface: Element<'_, ()> = decorative_surface(
        text("content"),
        vec![text("underlay").into()],
        vec![text("overlay").into()],
        props,
        &theme,
    );
}

#[test]
fn decorative_surface_can_wrap_component_content() {
    let theme = Theme::light();
    let button = Button::text("Action", &theme)
        .variant(ButtonVariant::Default)
        .into_button();

    let _surface: Element<'_, ()> = decorative_surface(
        button,
        vec![text("underlay").into()],
        vec![text("overlay").into()],
        DecorativeSurfaceProps::new().themed(),
        &theme,
    );
}
