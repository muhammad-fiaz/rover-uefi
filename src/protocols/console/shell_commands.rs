//! # UEFI Shell Built-in Commands Framework
//!
//! Provides the command registry, canonical descriptor table, execution status codes,
//! and profile classifications for standardized UEFI Shell commands defined across
//! Support Levels 1-3, Driver diagnostics, Debug, Network, and Configuration profiles.

use crate::base::EfiStatus;

/// Execution status codes returned by UEFI Shell commands and applications.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum ShellStatus {
    /// The shell command completed successfully.
    Success = 0,
    /// The application or driver failed to load.
    LoadError = 1,
    /// One or more command-line parameters were invalid.
    InvalidParameter = 2,
    /// The requested operation or parameter syntax is unsupported.
    Unsupported = 3,
    /// The buffer was not of the expected size.
    BadBufferSize = 4,
    /// The output buffer was too small to receive the data.
    BufferTooSmall = 5,
    /// The requested device or resource is not ready.
    NotReady = 6,
    /// The underlying physical hardware encountered a device error.
    DeviceError = 7,
    /// Target storage media is write-protected.
    WriteProtected = 8,
    /// Out of system memory or execution resources.
    OutOfResources = 9,
    /// File system volume structure is corrupted.
    VolumeCorrupted = 10,
    /// Storage volume has no remaining free capacity.
    VolumeFull = 11,
    /// Target drive contains no removable media.
    NoMedia = 12,
    /// Media was changed during an operation.
    MediaChanged = 13,
    /// File, directory, device, or variable was not found.
    NotFound = 14,
    /// Access was denied due to permissions or locking.
    AccessDenied = 15,
    /// Operation timed out before completion.
    Timeout = 18,
    /// The subsystem or protocol is not started.
    NotStarted = 19,
    /// The command or handler is already started.
    AlreadyStarted = 20,
    /// Command execution was aborted by user or system break.
    Aborted = 21,
    /// Incompatible firmware or specification version encountered.
    IncompatibleVersion = 25,
    /// Security violation prevented command execution.
    SecurityViolation = 26,
    /// Comparison produced unequal results.
    NotEqual = 27,
}

impl From<EfiStatus> for ShellStatus {
    fn from(status: EfiStatus) -> Self {
        match status {
            EfiStatus::SUCCESS => Self::Success,
            EfiStatus::LOAD_ERROR => Self::LoadError,
            EfiStatus::INVALID_PARAMETER => Self::InvalidParameter,
            EfiStatus::UNSUPPORTED => Self::Unsupported,
            EfiStatus::BAD_BUFFER_SIZE => Self::BadBufferSize,
            EfiStatus::BUFFER_TOO_SMALL => Self::BufferTooSmall,
            EfiStatus::NOT_READY => Self::NotReady,
            EfiStatus::WRITE_PROTECTED => Self::WriteProtected,
            EfiStatus::OUT_OF_RESOURCES => Self::OutOfResources,
            EfiStatus::VOLUME_CORRUPTED => Self::VolumeCorrupted,
            EfiStatus::VOLUME_FULL => Self::VolumeFull,
            EfiStatus::NO_MEDIA => Self::NoMedia,
            EfiStatus::MEDIA_CHANGED => Self::MediaChanged,
            EfiStatus::NOT_FOUND => Self::NotFound,
            EfiStatus::ACCESS_DENIED => Self::AccessDenied,
            EfiStatus::TIMEOUT => Self::Timeout,
            EfiStatus::NOT_STARTED => Self::NotStarted,
            EfiStatus::ALREADY_STARTED => Self::AlreadyStarted,
            EfiStatus::ABORTED => Self::Aborted,
            EfiStatus::INCOMPATIBLE_VERSION => Self::IncompatibleVersion,
            EfiStatus::SECURITY_VIOLATION => Self::SecurityViolation,
            _ => Self::DeviceError,
        }
    }
}

impl From<ShellStatus> for EfiStatus {
    fn from(status: ShellStatus) -> Self {
        match status {
            ShellStatus::Success => Self::SUCCESS,
            ShellStatus::LoadError => Self::LOAD_ERROR,
            ShellStatus::InvalidParameter => Self::INVALID_PARAMETER,
            ShellStatus::Unsupported => Self::UNSUPPORTED,
            ShellStatus::BadBufferSize => Self::BAD_BUFFER_SIZE,
            ShellStatus::BufferTooSmall => Self::BUFFER_TOO_SMALL,
            ShellStatus::NotReady => Self::NOT_READY,
            ShellStatus::DeviceError => Self::DEVICE_ERROR,
            ShellStatus::WriteProtected => Self::WRITE_PROTECTED,
            ShellStatus::OutOfResources => Self::OUT_OF_RESOURCES,
            ShellStatus::VolumeCorrupted => Self::VOLUME_CORRUPTED,
            ShellStatus::VolumeFull => Self::VOLUME_FULL,
            ShellStatus::NoMedia => Self::NO_MEDIA,
            ShellStatus::MediaChanged => Self::MEDIA_CHANGED,
            ShellStatus::NotFound | ShellStatus::NotEqual => Self::NOT_FOUND,
            ShellStatus::AccessDenied => Self::ACCESS_DENIED,
            ShellStatus::Timeout => Self::TIMEOUT,
            ShellStatus::NotStarted => Self::NOT_STARTED,
            ShellStatus::AlreadyStarted => Self::ALREADY_STARTED,
            ShellStatus::Aborted => Self::ABORTED,
            ShellStatus::IncompatibleVersion => Self::INCOMPATIBLE_VERSION,
            ShellStatus::SecurityViolation => Self::SECURITY_VIOLATION,
        }
    }
}

/// Minimum Shell Support Level 0 (bare environment).
pub const SHELL_SUPPORT_LEVEL_0: u32 = 0;

/// Shell Support Level 1 (batch scripts, branching, loop constructs).
pub const SHELL_SUPPORT_LEVEL_1: u32 = 1;

/// Shell Support Level 2 (filesystem operations, device mapping, environment).
pub const SHELL_SUPPORT_LEVEL_2: u32 = 2;

/// Shell Support Level 3 (interactive tools, help, paging, console utilities).
pub const SHELL_SUPPORT_LEVEL_3: u32 = 3;

/// Profile name for Level 1 command set.
pub const SHELL_PROFILE_LEVEL1: &str = "Level1";

/// Profile name for Level 2 command set.
pub const SHELL_PROFILE_LEVEL2: &str = "Level2";

/// Profile name for Level 3 command set.
pub const SHELL_PROFILE_LEVEL3: &str = "Level3";

/// Profile name for driver management commands.
pub const SHELL_PROFILE_DRIVER1: &str = "Driver1";

/// Profile name for hardware and firmware debugging commands.
pub const SHELL_PROFILE_DEBUG1: &str = "Debug1";

/// Profile name for network diagnostics commands.
pub const SHELL_PROFILE_NETWORK1: &str = "Network1";

/// Profile name for boot configuration commands.
pub const SHELL_PROFILE_BCFG: &str = "Bcfg";

/// Metadata descriptor for a UEFI Shell built-in or registered command.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ShellCommandDescriptor {
    /// Canonical command name matching command-line invocation.
    pub name: &'static str,
    /// Brief human-readable summary of command operation.
    pub summary: &'static str,
    /// Minimum required shell support level (1, 2, or 3).
    pub support_level: u32,
    /// Functional profile classification (`"Level1"`, `"Driver1"`, etc.).
    pub profile: &'static str,
    /// Whether command outcome modifies the `%LASTERROR%` environment variable.
    pub can_affect_last_error: bool,
}

/// Table of all standardized built-in UEFI Shell commands.
pub static BUILTIN_COMMANDS: &[ShellCommandDescriptor] = &[
    // Level 1: Scripting and execution control
    ShellCommandDescriptor {
        name: "exit",
        summary: "Exit the current script or the UEFI Shell",
        support_level: SHELL_SUPPORT_LEVEL_1,
        profile: SHELL_PROFILE_LEVEL1,
        can_affect_last_error: false,
    },
    ShellCommandDescriptor {
        name: "for",
        summary: "Execute commands repeatedly for set of items or filenames",
        support_level: SHELL_SUPPORT_LEVEL_1,
        profile: SHELL_PROFILE_LEVEL1,
        can_affect_last_error: false,
    },
    ShellCommandDescriptor {
        name: "goto",
        summary: "Jump execution to a specified script label",
        support_level: SHELL_SUPPORT_LEVEL_1,
        profile: SHELL_PROFILE_LEVEL1,
        can_affect_last_error: false,
    },
    ShellCommandDescriptor {
        name: "if",
        summary: "Perform conditional command execution in batch scripts",
        support_level: SHELL_SUPPORT_LEVEL_1,
        profile: SHELL_PROFILE_LEVEL1,
        can_affect_last_error: false,
    },
    ShellCommandDescriptor {
        name: "shift",
        summary: "Shift batch argument positions leftward",
        support_level: SHELL_SUPPORT_LEVEL_1,
        profile: SHELL_PROFILE_LEVEL1,
        can_affect_last_error: false,
    },
    ShellCommandDescriptor {
        name: "stall",
        summary: "Stall processor execution for specified microseconds",
        support_level: SHELL_SUPPORT_LEVEL_1,
        profile: SHELL_PROFILE_LEVEL1,
        can_affect_last_error: false,
    },
    // Level 2: Filesystem, device mappings, and execution
    ShellCommandDescriptor {
        name: "attrib",
        summary: "Display or modify attributes of files and directories",
        support_level: SHELL_SUPPORT_LEVEL_2,
        profile: SHELL_PROFILE_LEVEL2,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "cd",
        summary: "Change current directory or display working directory",
        support_level: SHELL_SUPPORT_LEVEL_2,
        profile: SHELL_PROFILE_LEVEL2,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "cp",
        summary: "Copy files or directory trees to destination",
        support_level: SHELL_SUPPORT_LEVEL_2,
        profile: SHELL_PROFILE_LEVEL2,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "date",
        summary: "Display or configure the system calendar date",
        support_level: SHELL_SUPPORT_LEVEL_2,
        profile: SHELL_PROFILE_LEVEL2,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "load",
        summary: "Load specified UEFI driver image into system memory",
        support_level: SHELL_SUPPORT_LEVEL_2,
        profile: SHELL_PROFILE_LEVEL2,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "ls",
        summary: "List directory contents or file attributes",
        support_level: SHELL_SUPPORT_LEVEL_2,
        profile: SHELL_PROFILE_LEVEL2,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "map",
        summary: "Display or modify short device mapping mappings (fs0, blk0)",
        support_level: SHELL_SUPPORT_LEVEL_2,
        profile: SHELL_PROFILE_LEVEL2,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "mkdir",
        summary: "Create one or more new directories",
        support_level: SHELL_SUPPORT_LEVEL_2,
        profile: SHELL_PROFILE_LEVEL2,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "mv",
        summary: "Move or rename files and directories",
        support_level: SHELL_SUPPORT_LEVEL_2,
        profile: SHELL_PROFILE_LEVEL2,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "parse",
        summary: "Parse and extract tokens from script outputs",
        support_level: SHELL_SUPPORT_LEVEL_2,
        profile: SHELL_PROFILE_LEVEL2,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "reset",
        summary: "Reset the platform (cold, warm, shutdown)",
        support_level: SHELL_SUPPORT_LEVEL_2,
        profile: SHELL_PROFILE_LEVEL2,
        can_affect_last_error: false,
    },
    ShellCommandDescriptor {
        name: "rm",
        summary: "Delete one or more files or directory hierarchies",
        support_level: SHELL_SUPPORT_LEVEL_2,
        profile: SHELL_PROFILE_LEVEL2,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "set",
        summary: "Display, define, or delete shell environment variables",
        support_level: SHELL_SUPPORT_LEVEL_2,
        profile: SHELL_PROFILE_LEVEL2,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "time",
        summary: "Display or configure the real-time system clock",
        support_level: SHELL_SUPPORT_LEVEL_2,
        profile: SHELL_PROFILE_LEVEL2,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "vol",
        summary: "Display volume information and label for a file system",
        support_level: SHELL_SUPPORT_LEVEL_2,
        profile: SHELL_PROFILE_LEVEL2,
        can_affect_last_error: true,
    },
    // Level 3: Interactive tools and console utilities
    ShellCommandDescriptor {
        name: "alias",
        summary: "Display, create, or delete command aliases",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_LEVEL3,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "cls",
        summary: "Clear console display surface and reset cursor position",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_LEVEL3,
        can_affect_last_error: false,
    },
    ShellCommandDescriptor {
        name: "echo",
        summary: "Display text messages or control command echo in scripts",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_LEVEL3,
        can_affect_last_error: false,
    },
    ShellCommandDescriptor {
        name: "help",
        summary: "Display interactive help and syntax for shell commands",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_LEVEL3,
        can_affect_last_error: false,
    },
    ShellCommandDescriptor {
        name: "pause",
        summary: "Pause batch script execution and wait for keystroke",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_LEVEL3,
        can_affect_last_error: false,
    },
    ShellCommandDescriptor {
        name: "touch",
        summary: "Update timestamp of a file or create empty file",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_LEVEL3,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "type",
        summary: "Display contents of text or ASCII files to console",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_LEVEL3,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "ver",
        summary: "Display UEFI firmware, shell version, and vendor banner",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_LEVEL3,
        can_affect_last_error: false,
    },
    // Driver1: Controller and driver management
    ShellCommandDescriptor {
        name: "connect",
        summary: "Bind UEFI drivers to device controllers recursively",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DRIVER1,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "devices",
        summary: "Display list of devices managed by UEFI drivers",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DRIVER1,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "devtree",
        summary: "Display hierarchical device tree of active controllers",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DRIVER1,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "dh",
        summary: "Dump handle database and inspect installed protocols",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DRIVER1,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "disconnect",
        summary: "Disconnect driver binding from specified device handle",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DRIVER1,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "drivers",
        summary: "Display list of registered UEFI drivers and images",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DRIVER1,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "drvcfg",
        summary: "Invoke driver configuration protocol for controller",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DRIVER1,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "drvdiag",
        summary: "Invoke driver diagnostics protocol for controller",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DRIVER1,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "openinfo",
        summary: "Display open protocol agents and attributes for a handle",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DRIVER1,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "reconnect",
        summary: "Disconnect and immediately reconnect drivers to a controller",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DRIVER1,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "unload",
        summary: "Unload a driver or application image from memory",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DRIVER1,
        can_affect_last_error: true,
    },
    // Debug1: Hardware and system inspection
    ShellCommandDescriptor {
        name: "dmem",
        summary: "Dump memory contents in hexadecimal and ASCII",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DEBUG1,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "dmpstore",
        summary: "Dump UEFI non-volatile and volatile variable store",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DEBUG1,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "memmap",
        summary: "Display system memory map and physical descriptors",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DEBUG1,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "mm",
        summary: "Inspect or modify memory, I/O ports, or PCI configuration space",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DEBUG1,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "pci",
        summary: "Enumerate and display PCI bus devices and configuration registers",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DEBUG1,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "smbiosview",
        summary: "Display SMBIOS system and hardware structures",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_DEBUG1,
        can_affect_last_error: true,
    },
    // Network1: Network stack configuration and testing
    ShellCommandDescriptor {
        name: "ifconfig",
        summary: "Display or configure network interfaces, IP addresses, and DNS",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_NETWORK1,
        can_affect_last_error: true,
    },
    ShellCommandDescriptor {
        name: "ping",
        summary: "Send ICMP echo request packets to network destination",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_NETWORK1,
        can_affect_last_error: true,
    },
    // Bcfg: Boot and Driver Option Configuration
    ShellCommandDescriptor {
        name: "bcfg",
        summary: "Configure boot and driver load options in NVRAM (BootOrder, DriverOrder)",
        support_level: SHELL_SUPPORT_LEVEL_3,
        profile: SHELL_PROFILE_BCFG,
        can_affect_last_error: true,
    },
];

/// Looks up a built-in shell command by name (case-insensitive).
#[must_use]
pub fn lookup_command(name: &str) -> Option<&'static ShellCommandDescriptor> {
    BUILTIN_COMMANDS
        .iter()
        .find(|cmd| cmd.name.eq_ignore_ascii_case(name))
}

/// Returns `true` if `name` matches a standardized built-in UEFI Shell command.
#[must_use]
pub fn is_builtin_command(name: &str) -> bool {
    lookup_command(name).is_some()
}

/// Returns an iterator yielding all built-in commands supported at or below `level`.
pub fn commands_for_level(level: u32) -> impl Iterator<Item = &'static ShellCommandDescriptor> {
    BUILTIN_COMMANDS
        .iter()
        .filter(move |cmd| cmd.support_level <= level)
}

/// Returns an iterator yielding all built-in commands associated with `profile`.
pub fn commands_for_profile(
    profile: &str,
) -> impl Iterator<Item = &'static ShellCommandDescriptor> {
    BUILTIN_COMMANDS
        .iter()
        .filter(move |cmd| cmd.profile.eq_ignore_ascii_case(profile))
}

/// Static buffer size for dynamic registered shell command entries.
pub const MAX_REGISTERED_COMMANDS: usize = 64;

/// In-memory registry for static built-in and dynamic runtime shell commands.
pub struct ShellCommandRegistry {
    entries: [Option<ShellCommandDescriptor>; MAX_REGISTERED_COMMANDS],
    count: usize,
}

impl Default for ShellCommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellCommandRegistry {
    /// Creates an empty command registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: [None; MAX_REGISTERED_COMMANDS],
            count: 0,
        }
    }

    /// Creates a registry pre-populated with all standardized built-in commands.
    #[must_use]
    pub fn with_builtins() -> Self {
        let mut reg = Self::new();
        for &cmd in BUILTIN_COMMANDS {
            let _ = reg.register(cmd);
        }
        reg
    }

    /// Number of active commands in this registry.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.count
    }

    /// Registers a new command descriptor into the registry.
    ///
    /// # Errors
    ///
    /// Returns [`ShellStatus::AlreadyStarted`] if a command with `cmd.name` is already registered,
    /// or [`ShellStatus::OutOfResources`] if the registry capacity is exceeded.
    pub fn register(&mut self, cmd: ShellCommandDescriptor) -> Result<(), ShellStatus> {
        if self.find(cmd.name).is_some() {
            return Err(ShellStatus::AlreadyStarted);
        }
        if self.count >= MAX_REGISTERED_COMMANDS {
            return Err(ShellStatus::OutOfResources);
        }

        for slot in &mut self.entries {
            if slot.is_none() {
                *slot = Some(cmd);
                self.count += 1;
                return Ok(());
            }
        }

        Err(ShellStatus::OutOfResources)
    }

    /// Unregisters a command by name.
    ///
    /// # Errors
    ///
    /// Returns [`ShellStatus::NotFound`] if no command matches `name`.
    pub fn unregister(&mut self, name: &str) -> Result<(), ShellStatus> {
        for slot in &mut self.entries {
            let matches = slot
                .as_ref()
                .is_some_and(|cmd| cmd.name.eq_ignore_ascii_case(name));
            if matches {
                *slot = None;
                self.count -= 1;
                return Ok(());
            }
        }
        Err(ShellStatus::NotFound)
    }

    /// Looks up a registered command by name (case-insensitive).
    #[must_use]
    pub fn find(&self, name: &str) -> Option<&ShellCommandDescriptor> {
        self.entries
            .iter()
            .flatten()
            .find(|cmd| cmd.name.eq_ignore_ascii_case(name))
    }
}
