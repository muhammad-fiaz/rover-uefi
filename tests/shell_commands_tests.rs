//! Integration tests for UEFI Shell Built-in Commands Framework.

use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::console::shell_commands::*;

#[test]
fn test_shell_status_conversions() {
    assert_eq!(ShellStatus::from(EfiStatus::SUCCESS), ShellStatus::Success);
    assert_eq!(
        ShellStatus::from(EfiStatus::INVALID_PARAMETER),
        ShellStatus::InvalidParameter
    );
    assert_eq!(
        ShellStatus::from(EfiStatus::UNSUPPORTED),
        ShellStatus::Unsupported
    );
    assert_eq!(
        ShellStatus::from(EfiStatus::OUT_OF_RESOURCES),
        ShellStatus::OutOfResources
    );
    assert_eq!(
        ShellStatus::from(EfiStatus::SECURITY_VIOLATION),
        ShellStatus::SecurityViolation
    );

    assert_eq!(EfiStatus::from(ShellStatus::Success), EfiStatus::SUCCESS);
    assert_eq!(
        EfiStatus::from(ShellStatus::LoadError),
        EfiStatus::LOAD_ERROR
    );
    assert_eq!(EfiStatus::from(ShellStatus::NotFound), EfiStatus::NOT_FOUND);
    assert_eq!(EfiStatus::from(ShellStatus::Timeout), EfiStatus::TIMEOUT);
}

#[test]
fn test_builtin_command_lookup_and_inspection() {
    let help_cmd = lookup_command("help");
    assert!(help_cmd.is_some());
    if let Some(cmd) = help_cmd {
        assert_eq!(cmd.name, "help");
        assert_eq!(cmd.support_level, SHELL_SUPPORT_LEVEL_3);
        assert_eq!(cmd.profile, SHELL_PROFILE_LEVEL3);
        assert!(!cmd.can_affect_last_error);
    }

    let bcfg_cmd = lookup_command("BCFG");
    assert!(bcfg_cmd.is_some());
    if let Some(cmd) = bcfg_cmd {
        assert_eq!(cmd.name, "bcfg");
        assert_eq!(cmd.profile, SHELL_PROFILE_BCFG);
        assert!(cmd.can_affect_last_error);
    }

    let dh_cmd = lookup_command("dh");
    assert!(dh_cmd.is_some());
    if let Some(cmd) = dh_cmd {
        assert_eq!(cmd.name, "dh");
        assert_eq!(cmd.profile, SHELL_PROFILE_DRIVER1);
    }

    assert!(is_builtin_command("ls"));
    assert!(is_builtin_command("cd"));
    assert!(is_builtin_command("pci"));
    assert!(is_builtin_command("memmap"));
    assert!(is_builtin_command("reset"));
    assert!(is_builtin_command("ifconfig"));
    assert!(is_builtin_command("ping"));
    assert!(!is_builtin_command("nonexistent_command_xyz"));
}

#[test]
fn test_commands_by_level_and_profile() {
    let lvl1_cmds: Vec<&str> = commands_for_level(SHELL_SUPPORT_LEVEL_1)
        .map(|c| c.name)
        .collect();
    assert!(lvl1_cmds.contains(&"exit"));
    assert!(lvl1_cmds.contains(&"for"));
    assert!(lvl1_cmds.contains(&"if"));
    assert!(!lvl1_cmds.contains(&"ls")); // ls is Level 2

    let driver_cmds: Vec<&str> = commands_for_profile(SHELL_PROFILE_DRIVER1)
        .map(|c| c.name)
        .collect();
    assert!(driver_cmds.contains(&"connect"));
    assert!(driver_cmds.contains(&"disconnect"));
    assert!(driver_cmds.contains(&"devices"));
    assert!(driver_cmds.contains(&"drivers"));
    assert!(driver_cmds.contains(&"dh"));
    assert!(!driver_cmds.contains(&"help"));

    let net_cmds: Vec<&str> = commands_for_profile(SHELL_PROFILE_NETWORK1)
        .map(|c| c.name)
        .collect();
    assert!(net_cmds.contains(&"ifconfig"));
    assert!(net_cmds.contains(&"ping"));
}

#[test]
fn test_shell_command_registry() {
    let mut registry = ShellCommandRegistry::new();
    assert_eq!(registry.count(), 0);

    let test_cmd = ShellCommandDescriptor {
        name: "mytool",
        summary: "Custom platform diagnosis tool",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: "Custom",
        can_affect_last_error: true,
    };

    assert_eq!(registry.register(test_cmd), Ok(()));
    assert_eq!(registry.count(), 1);

    // Duplicate registration should fail
    assert_eq!(
        registry.register(test_cmd),
        Err(ShellStatus::AlreadyStarted)
    );

    let found = registry.find("MYTOOL");
    assert!(found.is_some());
    if let Some(cmd) = found {
        assert_eq!(cmd.name, "mytool");
        assert_eq!(cmd.summary, "Custom platform diagnosis tool");
    }

    assert_eq!(registry.unregister("mytool"), Ok(()));
    assert_eq!(registry.count(), 0);
    assert_eq!(registry.unregister("mytool"), Err(ShellStatus::NotFound));

    let builtins_reg = ShellCommandRegistry::with_builtins();
    assert_eq!(builtins_reg.count(), BUILTIN_COMMANDS.len());
    assert!(builtins_reg.find("help").is_some());
    assert!(builtins_reg.find("bcfg").is_some());
}
