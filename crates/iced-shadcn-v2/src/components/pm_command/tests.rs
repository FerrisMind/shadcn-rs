use super::*;

#[test]
fn defaults_match_svelte_component() {
    let theme = Theme::light();
    let command: PmCommand<'_, ()> = PmCommand::new("execute", ["jsrepo", "add"], &theme);

    assert_eq!(command.variant, PmCommandVariant::Default);
    assert_eq!(command.radius, PmCommandRadius::Default);
    assert_eq!(command.agent, PmCommandAgent::Npm);
    assert_eq!(command.agents, PmCommandAgent::defaults());
    assert_eq!(command.command_text(), "npx jsrepo add");
}

#[test]
fn resolves_npm_pnpm_yarn_bun_and_deno_aliases() {
    let args = vec!["jsrepo".to_owned(), "add".to_owned()];

    assert_eq!(
        resolve_pm_command(&PmCommandAgent::Npm, &PmCommandVerb::Execute, &args).command_text(),
        "npx jsrepo add"
    );
    assert_eq!(
        resolve_pm_command(&PmCommandAgent::Pnpm, &PmCommandVerb::Execute, &args).command_text(),
        "pnpm dlx jsrepo add"
    );
    assert_eq!(
        resolve_pm_command(&PmCommandAgent::Yarn, &PmCommandVerb::Execute, &args).command_text(),
        "npx jsrepo add"
    );
    assert_eq!(
        resolve_pm_command(&PmCommandAgent::Bun, &PmCommandVerb::Execute, &args).command_text(),
        "bun x jsrepo add"
    );
    assert_eq!(
        resolve_pm_command(&PmCommandAgent::Deno, &PmCommandVerb::Execute, &args).command_text(),
        "deno run npm:jsrepo add"
    );
}

#[test]
fn resolves_package_manager_specific_install_and_upgrade_commands() {
    let no_args = Vec::new();

    assert_eq!(
        resolve_pm_command(&PmCommandAgent::Npm, &PmCommandVerb::Install, &no_args).command_text(),
        "npm i"
    );
    assert_eq!(
        resolve_pm_command(&PmCommandAgent::Pnpm, &PmCommandVerb::Frozen, &no_args).command_text(),
        "pnpm i --frozen-lockfile"
    );
    assert_eq!(
        resolve_pm_command(&PmCommandAgent::YarnBerry, &PmCommandVerb::Frozen, &no_args)
            .command_text(),
        "yarn install --immutable"
    );
    assert_eq!(
        resolve_pm_command(
            &PmCommandAgent::Bun,
            &PmCommandVerb::Add,
            &["vite".to_owned()]
        )
        .command_text(),
        "bun add vite"
    );
    assert_eq!(
        resolve_pm_command(
            &PmCommandAgent::Pnpm6,
            &PmCommandVerb::Run,
            &["build".to_owned(), "--watch".to_owned()]
        )
        .command_text(),
        "pnpm run build -- --watch"
    );
}

#[test]
fn unsupported_detector_combinations_are_reported_without_hiding_fallbacks() {
    let no_args = Vec::new();
    assert!(
        try_resolve_pm_command(
            &PmCommandAgent::Npm,
            &PmCommandVerb::UpgradeInteractive,
            &no_args
        )
        .is_none()
    );
    assert_eq!(
        resolve_pm_command(
            &PmCommandAgent::Npm,
            &PmCommandVerb::UpgradeInteractive,
            &no_args
        )
        .command_text(),
        "npm upgrade-interactive"
    );
}

#[test]
fn yarn_local_execute_adds_separator_for_multiple_arguments() {
    let args = vec!["tool".to_owned(), "--version".to_owned()];
    let resolution = resolve_pm_command(&PmCommandAgent::Yarn, &PmCommandVerb::ExecuteLocal, &args);

    assert_eq!(resolution.command(), "yarn");
    assert_eq!(resolution.args(), &["exec", "tool", "--", "--version"]);
}

#[test]
fn custom_agents_and_commands_remain_renderable() {
    let agent = PmCommandAgent::from(" volta ".to_owned());
    let command = PmCommandVerb::from("doctor");
    let args = vec!["--json".to_owned()];
    let resolution = resolve_pm_command(&agent, &command, &args);

    assert_eq!(resolution.command_text(), " volta  doctor --json");
}

#[test]
fn selected_agent_falls_back_to_the_first_visible_tab() {
    let theme = Theme::light();
    let command: PmCommand<'_, ()> = PmCommand::new("execute", ["tool"], &theme)
        .agents(["pnpm", "bun"])
        .agent("deno");

    assert_eq!(command.selected_agent(), &PmCommandAgent::Deno);
    assert_eq!(command.resolved_command().command_text(), "pnpm dlx tool");

    let empty: PmCommand<'_, ()> = PmCommand::new("execute", ["tool"], &theme)
        .agents(std::iter::empty::<PmCommandAgent>())
        .agent("deno");
    assert_eq!(empty.resolved_command().command_text(), "deno run npm:tool");
}

#[test]
fn builders_keep_variant_radius_and_dimensions_in_order() {
    let theme = Theme::light();
    let command: PmCommand<'_, ()> = PmCommand::new("run", ["build"], &theme)
        .variant(PmCommandVariant::Secondary)
        .radius(PmCommandRadius::Large)
        .width(320.0)
        .max_width(480.0)
        .copy_status(CopyButtonStatus::Success);

    assert_eq!(command.variant, PmCommandVariant::Secondary);
    assert_eq!(command.radius, PmCommandRadius::Large);
    assert_eq!(command.width, Length::Fixed(320.0));
    assert_eq!(command.max_width, Some(480.0));
    assert_eq!(command.copy_status, CopyButtonStatus::Success);
}
