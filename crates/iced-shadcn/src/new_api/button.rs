use std::borrow::Cow;
use iced::widget::{
    button as iced_button, text as iced_text,
};
use iced::{Element, Length};
use crate::theme::Theme;
use crate::spinner::{Spinner, SpinnerSize, spinner};
use crate::button::{ButtonVariant, ButtonSize, ButtonRadius, button_style};

/// Helper trait to unify icon types.
pub trait ButtonIcon<'a, Message> {
    fn into_element(self) -> Option<Element<'a, Message>>;
}

impl<'a, Message> ButtonIcon<'a, Message> for () {
    fn into_element(self) -> Option<Element<'a, Message>> {
        None
    }
}

pub struct AsElement<'a, Message>(Element<'a, Message>);

impl<'a, Message> ButtonIcon<'a, Message> for AsElement<'a, Message> {
    fn into_element(self) -> Option<Element<'a, Message>> {
        Some(self.0)
    }
}

/// A high-quality button component with a friendly Rust API.
/// Follows Rust API Guidelines and provides a zero-friction experience.
pub struct Button<'a, Message, I = ()>
where
    Message: Clone + 'a,
{
    label: Cow<'static, str>,
    on_press: Option<Message>,
    variant: ButtonVariant,
    size: ButtonSize,
    color: crate::tokens::AccentColor,
    radius: Option<ButtonRadius>,
    justify: crate::button::ButtonJustify,
    high_contrast: bool,
    disabled: bool,
    loading: bool,
    progress: f32,
    href: Option<Cow<'static, str>>,
    icon: I,
    icon_position: IconPosition,
    class: String,
    width: Length,
    height: Length,
    phantom: std::marker::PhantomData<&'a Message>,
}

impl<'a, Message, I> std::fmt::Debug for Button<'a, Message, I>
where
    Message: Clone + 'a,
    I: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Button")
            .field("label", &self.label)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("loading", &self.loading)
            .field("disabled", &self.disabled)
            .finish_non_exhaustive()
    }
}

impl<'a, Message, I> Clone for Button<'a, Message, I>
where
    Message: Clone + 'a,
    I: Clone,
{
    fn clone(&self) -> Self {
        Self {
            label: self.label.clone(),
            on_press: self.on_press.clone(),
            variant: self.variant,
            size: self.size,
            color: self.color,
            radius: self.radius,
            justify: self.justify,
            high_contrast: self.high_contrast,
            disabled: self.disabled,
            loading: self.loading,
            progress: self.progress,
            href: self.href.clone(),
            icon: self.icon.clone(),
            icon_position: self.icon_position,
            class: self.class.clone(),
            width: self.width,
            height: self.height,
            phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum IconPosition {
    #[default]
    Left,
    Right,
}

impl<'a, Message> Button<'a, Message, ()>
where
    Message: Clone + 'a,
{
    /// Creates a new button with the specified label.
    pub fn new<S>(label: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self {
            label: label.into(),
            on_press: None,
            variant: ButtonVariant::Default,
            size: ButtonSize::Size2,
            color: crate::tokens::AccentColor::Gray,
            radius: None,
            justify: crate::button::ButtonJustify::Center,
            high_contrast: false,
            disabled: false,
            loading: false,
            progress: 0.0,
            href: None,
            icon: (),
            icon_position: IconPosition::Left,
            class: String::new(),
            width: Length::Shrink,
            height: Length::Fixed(36.0),
            phantom: std::marker::PhantomData,
        }
    }

    /// Shorthand constructor for creating a link button.
    pub fn link<L, H>(label: L, href: H) -> Self
    where
        L: Into<Cow<'static, str>>,
        H: Into<Cow<'static, str>>,
    {
        Self::new(label).variant(ButtonVariant::Link).href(href)
    }
}

impl<'a, Message> Default for Button<'a, Message, ()>
where
    Message: Clone + 'a,
{
    fn default() -> Self {
        Self::new("")
    }
}

impl<'a, Message, I> Button<'a, Message, I>
where
    Message: Clone + 'a,
{
    /// Sets the action to be performed when the button is pressed.
    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets the button variant (e.g., Default, Outline, Ghost).
    ///
    /// # Examples
    /// ```rust,no_run
    /// # use iced_shadcn::new_api::Button;
    /// let _ = Button::new("Click").outline();
    /// ```
    #[must_use]
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the button size and its corresponding default height.
    #[must_use]
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self.height = Length::Fixed(Self::size_to_height_val(size));
        self
    }

    /// Sets a custom width for the button.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets a custom height for the button.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the button width to fill the available horizontal space.
    #[must_use]
    pub fn full_width(mut self) -> Self {
        self.width = Length::Fill;
        self
    }

    /// Sets the button width and height to fill the available space.
    #[must_use]
    pub fn fill(mut self) -> Self {
        self.width = Length::Fill;
        self.height = Length::Fill;
        self
    }

    /// Sets the accent color for the button.
    #[must_use]
    pub fn color(mut self, color: crate::tokens::AccentColor) -> Self {
        self.color = color;
        self
    }

    /// Sets the border radius for the button.
    #[must_use]
    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Sets the content justification within the button.
    #[must_use]
    pub fn justify(mut self, justify: crate::button::ButtonJustify) -> Self {
        self.justify = justify;
        self
    }

    /// Enables high contrast mode for the button.
    #[must_use]
    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    /// Sets the button to a disabled state.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Enables the loading state (shows a spinner).
    #[must_use]
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Sets the progress for the loading spinner animation (from 0.0 to 1.0).
    #[must_use]
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress;
        self
    }

    /// Turns the button into a link.
    #[must_use]
    pub fn href<S: Into<Cow<'static, str>>>(mut self, href: S) -> Self {
        self.href = Some(href.into());
        self
    }

    /// Adds custom CSS classes (metadata for the renderer).
    #[must_use]
    pub fn class<S: AsRef<str>>(mut self, class: S) -> Self {
        if !self.class.is_empty() {
            self.class.push(' ');
        }
        self.class.push_str(class.as_ref());
        self
    }

    /// Adds an icon to the button and returns a new button type.
    #[must_use]
    pub fn icon<NewI>(self, icon: NewI) -> Button<'a, Message, AsElement<'a, Message>> 
    where NewI: Into<Element<'a, Message>>
    {
        Button {
            label: self.label,
            on_press: self.on_press,
            variant: self.variant,
            size: self.size,
            color: self.color,
            radius: self.radius,
            justify: self.justify,
            high_contrast: self.high_contrast,
            disabled: self.disabled,
            loading: self.loading,
            progress: self.progress,
            href: self.href,
            icon: AsElement(icon.into()),
            icon_position: self.icon_position,
            class: self.class,
            width: self.width,
            height: self.height,
            phantom: self.phantom,
        }
    }

    /// Places the icon to the right of the text.
    #[must_use]
    pub fn icon_right(mut self) -> Self {
        self.icon_position = IconPosition::Right;
        self
    }

    // --- Shorthand Helpers (Friendly API) ---

    #[must_use] pub fn outline(self) -> Self { self.variant(ButtonVariant::Outline) }
    #[must_use] pub fn ghost(self) -> Self { self.variant(ButtonVariant::Ghost) }
    #[must_use] pub fn destructive(self) -> Self { self.variant(ButtonVariant::Destructive) }
    #[must_use] pub fn secondary(self) -> Self { self.variant(ButtonVariant::Secondary) }
    #[must_use] pub fn link_variant(self) -> Self { self.variant(ButtonVariant::Link) }

    #[must_use] pub fn sm(self) -> Self { self.size(ButtonSize::Size1) }
    #[must_use] pub fn lg(self) -> Self { self.size(ButtonSize::Size3) }
    #[must_use] pub fn xl(self) -> Self { self.size(ButtonSize::Size4) }
}

impl<'a, Message, I> Button<'a, Message, I>
where
    Message: Clone + 'a,
    I: ButtonIcon<'a, Message>,
{
    /// Finalizes the builder and returns an `Element` for iced.
    pub fn render(self, theme: &Theme) -> Element<'a, Message> {
        let Button {
            label,
            on_press,
            variant,
            size,
            color,
            radius,
            justify,
            high_contrast,
            disabled,
            loading,
            progress,
            href,
            icon,
            icon_position,
            class: _,
            width,
            height,
            phantom: _,
        } = self;

        let mut content_elements = Vec::new();

        // Handle typed icon or loading spinner
        let icon_element: Option<Element<'a, Message>> = if loading {
            let spinner_size = match size {
                ButtonSize::Size1 => SpinnerSize::Size1,
                ButtonSize::Size2 => SpinnerSize::Size2,
                _ => SpinnerSize::Size3,
            };
            Some(spinner(Spinner::new(theme).size(spinner_size).progress(progress)).into())
        } else {
            icon.into_element()
        };

        let has_label = !label.trim().is_empty();
        let is_icon_only = !has_label && icon_element.is_some() && variant != ButtonVariant::Link;

        match (icon_element, icon_position) {
            (Some(icon), IconPosition::Left) => {
                content_elements.push(icon);
                if has_label {
                    content_elements.push(iced::widget::Space::new().width(8.0).into());
                    content_elements.push(iced_text(label).size(Self::size_to_pixels_val(size)).into());
                }
            }
            (Some(icon), IconPosition::Right) => {
                if has_label {
                    content_elements.push(iced_text(label).size(Self::size_to_pixels_val(size)).into());
                    content_elements.push(iced::widget::Space::new().width(8.0).into());
                }
                content_elements.push(icon);
            }
            (None, _) => {
                if has_label {
                    content_elements.push(iced_text(label).size(Self::size_to_pixels_val(size)).into());
                }
            }
        }

        let height_val = Self::size_to_height_val(size);
        
        // Wrap content in a container for centering
        let row = iced::widget::Row::with_children(content_elements)
            .align_y(iced::Alignment::Center);

        let mut content_container = iced::widget::container(row)
            .height(Length::Fill)
            .center_y(Length::Fill);
            
        if is_icon_only {
            // For icon-only buttons, fill the entire space (limited by button's fixed width)
            content_container = content_container.width(Length::Fill).center_x(Length::Fill);
        }

        let content: Element<'a, Message> = content_container.into();

        let mut widget = iced_button(content)
            .width(width)
            .height(height);

        if is_icon_only {
            widget = widget.width(Length::Fixed(height_val)).padding(0);
        } else {
            widget = widget.padding(Self::size_to_padding_val(size));
        }

        let is_disabled = disabled || loading || (on_press.is_none() && href.is_none());
        
        if let Some(msg) = on_press {
            if !is_disabled {
                widget = widget.on_press(msg);
            }
        }

        // Render using existing style logic in iced-shadcn
        let theme_clone = theme.clone();
        let props = crate::button::ButtonProps {
            variant,
            size,
            color,
            radius,
            justify,
            high_contrast,
            loading,
            disabled,
        };

        widget.style(move |_iced_theme, status| button_style(&theme_clone, props, status)).into()
    }
}

impl<'a, Message, I> Button<'a, Message, I>
where
    Message: Clone + 'a,
{
    fn size_to_pixels_val(size: ButtonSize) -> u32 {
        match size {
            ButtonSize::Size1 => 14,
            ButtonSize::Size4 => 16,
            _ => 14,
        }
    }

    fn size_to_padding_val(size: ButtonSize) -> [f32; 2] {
        match size {
            ButtonSize::Size1 => [6.0, 12.0],
            ButtonSize::Size2 => [8.0, 16.0],
            ButtonSize::Size3 => [10.0, 24.0],
            ButtonSize::Size4 => [12.0, 28.0],
        }
    }

    fn size_to_height_val(size: ButtonSize) -> f32 {
        match size {
            ButtonSize::Size1 => 32.0,
            ButtonSize::Size2 => 36.0,
            ButtonSize::Size3 => 40.0,
            ButtonSize::Size4 => 48.0,
        }
    }
}

/// Shorthand macro for creating buttons.
///
/// # Examples
/// ```
/// # use iced_shadcn::button;
/// # #[derive(Clone)] enum Message { Click }
/// button!("Push me", on_press: Message::Click).outline()
/// # ;
/// ```
#[macro_export]
macro_rules! button {
    ($label:expr) => { $crate::new_api::Button::new($label) };
    ($label:expr, on_press: $msg:expr) => { $crate::new_api::Button::new($label).on_press($msg) };
    ($label:expr, href: $href:expr) => { $crate::new_api::Button::link($label, $href) };
    ($label:expr, $variant:ident) => { $crate::new_api::Button::new($label).$variant() };
}
