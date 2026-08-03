//! Package-manager command display with selectable agent tabs.
//!
//! This is the iced port of the shadcn-svelte-extra `PMCommand` component.
//! It keeps the reference component's package-manager aliases, custom agent
//! list, controlled selected agent, secondary variant, tooltip copy button,
//! and horizontal overflow behavior. Clipboard writes remain application
//! owned, following [`super::copy_button::CopyButton`]'s controlled API.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{PmCommand, PmCommandAgent, PmCommandVerb, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     AgentChanged(PmCommandAgent),
//!     Copy,
//! }
//!
//! fn view(theme: &Theme) -> Element<'_, Message> {
//!     PmCommand::new(
//!         PmCommandVerb::Execute,
//!         ["jsrepo", "add", "ui/pm-command"],
//!         theme,
//!     )
//!     .agents([PmCommandAgent::Npm, PmCommandAgent::Pnpm, PmCommandAgent::Yarn])
//!     .on_agent_change(Message::AgentChanged)
//!     .on_copy(Message::Copy)
//!     .max_width(480.0)
//!     .into()
//! }
//! ```

mod icon;
mod render;
mod types;

#[cfg(test)]
mod tests;

pub use types::{
    PmCommandAgent, PmCommandRadius, PmCommandResolution, PmCommandVariant, PmCommandVerb,
    resolve_pm_command, try_resolve_pm_command,
};

use std::fmt;
use std::time::Duration;

use crate::components::copy_button::{CopyButtonAction, CopyButtonStatus};
use crate::iced_compat::widget::container;
use crate::iced_compat::{Element, Length};
use crate::theme::Theme;

/// Rust-style spelling of the PMCommand component.
///
/// The [`PMCommand`] alias is also exported for source compatibility with the
/// upstream component name.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct PmCommand<'a, Message> {
    command: PmCommandVerb,
    args: Vec<String>,
    theme: &'a Theme,
    agents: Vec<PmCommandAgent>,
    agent: PmCommandAgent,
    variant: PmCommandVariant,
    radius: PmCommandRadius,
    width: Length,
    max_width: Option<f32>,
    copy_status: CopyButtonStatus,
    copy_animation_duration: Duration,
    on_agent_change: Option<Box<dyn Fn(PmCommandAgent) -> Message + 'a>>,
    on_copy: Option<PmCommandOnCopy<'a, Message>>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

/// Upstream-compatible acronym spelling for [`PmCommand`].
pub type PMCommand<'a, Message> = PmCommand<'a, Message>;

enum PmCommandOnCopy<'a, Message> {
    Message(Message),
    Callback(Box<dyn Fn(CopyButtonAction) -> Message + 'a>),
}

impl<Message> fmt::Debug for PmCommand<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PmCommand")
            .field("command", &self.command)
            .field("args", &self.args)
            .field("theme", &self.theme)
            .field("agents", &self.agents)
            .field("agent", &self.agent)
            .field("variant", &self.variant)
            .field("radius", &self.radius)
            .field("width", &self.width)
            .field("max_width", &self.max_width)
            .field("copy_status", &self.copy_status)
            .field("copy_animation_duration", &self.copy_animation_duration)
            .field("on_agent_change", &self.on_agent_change.is_some())
            .field("on_copy", &self.on_copy.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> PmCommand<'a, Message> {
    /// Creates a PMCommand from a detector command and its arguments.
    ///
    /// `command` and each argument accept either the typed enums/strings or a
    /// custom value via their `From` implementations.
    pub fn new(
        command: impl Into<PmCommandVerb>,
        args: impl IntoIterator<Item = impl Into<String>>,
        theme: &'a Theme,
    ) -> Self {
        Self {
            command: command.into(),
            args: args.into_iter().map(Into::into).collect(),
            theme,
            agents: PmCommandAgent::defaults(),
            agent: PmCommandAgent::default(),
            variant: PmCommandVariant::default(),
            radius: PmCommandRadius::default(),
            width: Length::Fill,
            max_width: None,
            copy_status: CopyButtonStatus::Idle,
            copy_animation_duration: Duration::from_millis(500),
            on_agent_change: None,
            on_copy: None,
            style_override: None,
        }
    }

    /// Replaces the command verb after construction.
    #[must_use = "builder methods return the modified PMCommand"]
    pub fn command(mut self, command: impl Into<PmCommandVerb>) -> Self {
        self.command = command.into();
        self
    }

    /// Replaces the command arguments after construction.
    #[must_use = "builder methods return the modified PMCommand"]
    pub fn args<I, A>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<String>,
    {
        self.args = args.into_iter().map(Into::into).collect();
        self
    }

    /// Replaces the available package-manager tabs.
    #[must_use = "builder methods return the modified PMCommand"]
    pub fn agents<I, A>(mut self, agents: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<PmCommandAgent>,
    {
        self.agents = agents.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the controlled package manager used to resolve and display the
    /// command.
    #[must_use = "builder methods return the modified PMCommand"]
    pub fn agent(mut self, agent: impl Into<PmCommandAgent>) -> Self {
        self.agent = agent.into();
        self
    }

    /// Returns the selected agent held by this builder.
    #[must_use]
    pub fn selected_agent(&self) -> &PmCommandAgent {
        &self.agent
    }

    /// Returns the configured agent list.
    #[must_use]
    pub fn agent_list(&self) -> &[PmCommandAgent] {
        &self.agents
    }

    /// Returns the configured command verb.
    #[must_use]
    pub fn command_verb(&self) -> &PmCommandVerb {
        &self.command
    }

    /// Returns the configured command arguments.
    #[must_use]
    pub fn arguments(&self) -> &[String] {
        &self.args
    }

    /// Resolves the command using the selected agent, falling back to the
    /// first visible tab when the controlled value is not in `agents`.
    #[must_use]
    pub fn resolved_command(&self) -> PmCommandResolution {
        resolve_pm_command(&self.effective_agent(), &self.command, &self.args)
    }

    fn effective_agent(&self) -> PmCommandAgent {
        self.agents
            .iter()
            .find(|candidate| *candidate == &self.agent)
            .cloned()
            .or_else(|| self.agents.first().cloned())
            .unwrap_or_else(|| self.agent.clone())
    }

    /// Returns the display string used by the copy button and command body.
    #[must_use]
    pub fn command_text(&self) -> String {
        self.resolved_command().command_text()
    }

    /// Sets the root visual variant.
    #[must_use = "builder methods return the modified PMCommand"]
    pub fn variant(mut self, variant: PmCommandVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the root corner-radius policy.
    #[must_use = "builder methods return the modified PMCommand"]
    pub fn radius(mut self, radius: PmCommandRadius) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the root width (`Fill` by default).
    #[must_use = "builder methods return the modified PMCommand"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Caps the root width, corresponding to the Svelte `class="max-w-*"`
    /// usage in the demos.
    #[must_use = "builder methods return the modified PMCommand"]
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width.max(0.0));
        self
    }

    /// Sets the controlled copy feedback status.
    #[must_use = "builder methods return the modified PMCommand"]
    pub fn copy_status(mut self, status: CopyButtonStatus) -> Self {
        self.copy_status = status;
        self
    }

    /// Sets the copy icon animation duration.
    #[must_use = "builder methods return the modified PMCommand"]
    pub fn copy_animation_duration(mut self, duration: Duration) -> Self {
        self.copy_animation_duration = duration;
        self
    }

    /// Publishes a message when an agent tab is selected.
    #[must_use = "builder methods return the modified PMCommand"]
    pub fn on_agent_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(PmCommandAgent) -> Message + 'a,
    {
        self.on_agent_change = Some(Box::new(callback));
        self
    }

    /// Publishes a fixed message when the copy button is pressed.
    #[must_use = "builder methods return the modified PMCommand"]
    pub fn on_copy(mut self, message: Message) -> Self {
        self.on_copy = Some(PmCommandOnCopy::Message(message));
        self
    }

    /// Clears or sets the fixed copy message.
    #[must_use = "builder methods return the modified PMCommand"]
    pub fn on_copy_maybe(mut self, message: Option<Message>) -> Self {
        self.on_copy = message.map(PmCommandOnCopy::Message);
        self
    }

    /// Maps the full copy action cycle to an application message.
    #[must_use = "builder methods return the modified PMCommand"]
    pub fn on_copy_action<F>(mut self, callback: F) -> Self
    where
        F: Fn(CopyButtonAction) -> Message + 'a,
    {
        self.on_copy = Some(PmCommandOnCopy::Callback(Box::new(callback)));
        self
    }

    /// Applies an iced container-style override to the resolved root style.
    #[must_use = "builder methods return the modified PMCommand"]
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the styled iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let PmCommand {
            command,
            args,
            theme,
            agents,
            agent,
            variant,
            radius,
            width,
            max_width,
            copy_status,
            copy_animation_duration,
            on_copy,
            on_agent_change,
            style_override,
        } = self;

        let active_agent = agents
            .iter()
            .find(|candidate| *candidate == &agent)
            .cloned()
            .or_else(|| agents.first().cloned())
            .unwrap_or(agent);
        let command_text = resolve_pm_command(&active_agent, &command, &args).command_text();
        let on_copy = on_copy.map(|source| match source {
            PmCommandOnCopy::Message(message) => message,
            PmCommandOnCopy::Callback(callback) => callback(CopyButtonAction::Pressed),
        });
        let on_agent_change = on_agent_change
            .as_ref()
            .map(|callback| callback.as_ref() as &dyn Fn(PmCommandAgent) -> Message);

        render::build(
            theme,
            variant,
            radius,
            width,
            max_width,
            &agents,
            &active_agent,
            &command_text,
            copy_status,
            copy_animation_duration,
            on_copy,
            on_agent_change,
            style_override,
        )
    }
}

impl<'a, Message> From<PmCommand<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(command: PmCommand<'a, Message>) -> Self {
        command.into_element()
    }
}
