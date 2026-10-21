//! # Supervisor binary interface
//!
//! The calling convention and the extension identifiers of the
//! supervisor binary interface.
//!
//! The interface lets a supervisor program ask the machine
//! firmware for services such as setting a timer, starting a
//! hart or resetting the system. The module only describes the
//! protocol: the extension identifiers, the function identifiers
//! of the individual extensions, the layout of the call and the
//! encoding of the returned status.
//!
//! No privileged instruction is executed here. The supervisor
//! performs the call itself and hands the result to the decoder.

/// The identifier of the legacy set timer extension.
pub const SBI_EXTENSION_LEGACY_SET_TIMER: u64 = 0x00;

/// The identifier of the legacy console putchar extension.
pub const SBI_EXTENSION_LEGACY_CONSOLE_PUTCHAR: u64 = 0x01;

/// The identifier of the legacy console getchar extension.
pub const SBI_EXTENSION_LEGACY_CONSOLE_GETCHAR: u64 = 0x02;

/// The identifier of the legacy clear inter-processor interrupt
/// extension.
pub const SBI_EXTENSION_LEGACY_CLEAR_IPI: u64 = 0x03;

/// The identifier of the legacy send inter-processor interrupt
/// extension.
pub const SBI_EXTENSION_LEGACY_SEND_IPI: u64 = 0x04;

/// The identifier of the legacy remote instruction fence
/// extension.
pub const SBI_EXTENSION_LEGACY_REMOTE_FENCE_I: u64 = 0x05;

/// The identifier of the legacy remote address translation fence
/// extension.
pub const SBI_EXTENSION_LEGACY_REMOTE_SFENCE_VMA: u64 = 0x06;

/// The identifier of the legacy remote address translation fence
/// extension with an address space identifier.
pub const SBI_EXTENSION_LEGACY_REMOTE_SFENCE_VMA_ASID: u64 = 0x07;

/// The identifier of the legacy system shutdown extension.
pub const SBI_EXTENSION_LEGACY_SHUTDOWN: u64 = 0x08;

/// The identifier of the base extension.
pub const SBI_EXTENSION_BASE: u64 = 0x10;

/// The identifier of the timer extension.
pub const SBI_EXTENSION_TIME: u64 = 0x5449_4D45;

/// The identifier of the inter-processor interrupt extension.
pub const SBI_EXTENSION_IPI: u64 = 0x0073_5049;

/// The identifier of the remote fence extension.
pub const SBI_EXTENSION_RFENCE: u64 = 0x5246_4E43;

/// The identifier of the hart start extension.
pub const SBI_EXTENSION_HS_START: u64 = 0x4853;

/// The identifier of the system reset extension.
pub const SBI_EXTENSION_SRST: u64 = 0x5352_5354;

/// The identifier of the debug console extension.
pub const SBI_EXTENSION_DBCN: u64 = 0x4442_434E;

/// The function that reports the version of the interface.
pub const SBI_BASE_FN_GET_SPEC_VERSION: u64 = 0x00;

/// The function that reports the identifier of the
/// implementation.
pub const SBI_BASE_FN_GET_IMPL_ID: u64 = 0x01;

/// The function that reports the version of the implementation.
pub const SBI_BASE_FN_GET_IMPL_VERSION: u64 = 0x02;

/// The function that probes for an extension.
pub const SBI_BASE_FN_PROBE_EXTENSION: u64 = 0x03;

/// The function that reports the vendor identifier.
pub const SBI_BASE_FN_GET_MVENDORID: u64 = 0x04;

/// The function that reports the architecture identifier.
pub const SBI_BASE_FN_GET_MARCHID: u64 = 0x05;

/// The function that reports the implementation identifier.
pub const SBI_BASE_FN_GET_MIMPID: u64 = 0x06;

/// The function that sets the timer of the previous time base.
pub const SBI_TIME_FN_SET_TIMER: u64 = 0x00;

/// The function that sets the timer of the supervisor time base.
pub const SBI_TIME_FN_SET_TIMER_STIME: u64 = 0x01;

/// The function that reads the current time.
pub const SBI_TIME_FN_GET_TIME: u64 = 0x02;

/// The function that sets the current time.
pub const SBI_TIME_FN_SET_TIME: u64 = 0x03;

/// The function that resets the system.
pub const SBI_SRST_FN_SYSTEM_RESET: u64 = 0x00;

/// The function that shuts the system down.
pub const SBI_SRST_FN_SYSTEM_SHUTDOWN: u64 = 0x01;

/// The index of the first argument register of a call.
pub const SBI_ECALL_ARGUMENT_0: usize = 0;

/// The index of the last argument register of a call.
pub const SBI_ECALL_ARGUMENT_5: usize = 5;

/// The index of the register that holds the function identifier.
pub const SBI_ECALL_FUNCTION: usize = 6;

/// The index of the register that holds the extension identifier.
pub const SBI_ECALL_EXTENSION: usize = 7;

/// The number of registers that a call uses.
pub const SBI_ECALL_REGISTER_COUNT: usize = 8;

/// The status of a call that succeeded.
pub const SBI_SUCCESS: isize = 0;

/// The status of a call that failed for an unspecified reason.
pub const SBI_ERR_FAILED: isize = -1;

/// The status of a call for an unsupported extension or
/// function.
pub const SBI_ERR_NOT_SUPPORTED: isize = -2;

/// The status of a call with an invalid parameter.
pub const SBI_ERR_INVALID_PARAM: isize = -3;

/// The status of a call that the implementation refused.
pub const SBI_ERR_DENIED: isize = -4;

/// The status of a call with an invalid address.
pub const SBI_ERR_INVALID_ADDRESS: isize = -5;

/// The status of a call for a resource that is already
/// available.
pub const SBI_ERR_ALREADY_AVAILABLE: isize = -6;

/// The status of a call for a hart that is already running.
pub const SBI_ERR_ALREADY_STARTED: isize = -7;

/// The status of a call for a hart that is already stopped.
pub const SBI_ERR_ALREADY_STOPPED: isize = -8;

/// The error of a supervisor binary interface call.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SbiError {
    /// The call failed for an unspecified reason.
    Failed,
    /// The extension or the function is not supported.
    NotSupported,
    /// A parameter of the call was invalid.
    InvalidParameter,
    /// The implementation refused the call.
    Denied,
    /// An address of the call was invalid.
    InvalidAddress,
    /// The requested resource is already available.
    AlreadyAvailable,
    /// The hart is already running.
    AlreadyStarted,
    /// The hart is already stopped.
    AlreadyStopped,
    /// The implementation reported an error that the interface
    /// does not define.
    Unknown(isize),
}

impl SbiError {
    /// Decodes an error status.
    ///
    /// A zero status is not an error and decodes to `None`.
    #[must_use]
    pub const fn from_code(code: isize) -> Option<Self> {
        match code {
            SBI_SUCCESS => None,
            SBI_ERR_FAILED => Some(Self::Failed),
            SBI_ERR_NOT_SUPPORTED => Some(Self::NotSupported),
            SBI_ERR_INVALID_PARAM => Some(Self::InvalidParameter),
            SBI_ERR_DENIED => Some(Self::Denied),
            SBI_ERR_INVALID_ADDRESS => Some(Self::InvalidAddress),
            SBI_ERR_ALREADY_AVAILABLE => Some(Self::AlreadyAvailable),
            SBI_ERR_ALREADY_STARTED => Some(Self::AlreadyStarted),
            SBI_ERR_ALREADY_STOPPED => Some(Self::AlreadyStopped),
            _ => Some(Self::Unknown(code)),
        }
    }

    /// Returns the numeric status of the error.
    #[must_use]
    pub const fn code(self) -> isize {
        match self {
            Self::Failed => SBI_ERR_FAILED,
            Self::NotSupported => SBI_ERR_NOT_SUPPORTED,
            Self::InvalidParameter => SBI_ERR_INVALID_PARAM,
            Self::Denied => SBI_ERR_DENIED,
            Self::InvalidAddress => SBI_ERR_INVALID_ADDRESS,
            Self::AlreadyAvailable => SBI_ERR_ALREADY_AVAILABLE,
            Self::AlreadyStarted => SBI_ERR_ALREADY_STARTED,
            Self::AlreadyStopped => SBI_ERR_ALREADY_STOPPED,
            Self::Unknown(code) => code,
        }
    }
}

/// The result of a supervisor binary interface call.
///
/// The interface returns an error status and a value. The value
/// carries the result of the call when the call succeeded.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SbiRet {
    /// The error status of the call.
    pub error: isize,
    /// The value that the call returned.
    pub value: usize,
}

impl SbiRet {
    /// Creates a result from its error status and its value.
    #[must_use]
    pub const fn new(error: isize, value: usize) -> Self {
        Self { error, value }
    }

    /// Creates a successful result.
    #[must_use]
    pub const fn success(value: usize) -> Self {
        Self {
            error: SBI_SUCCESS,
            value,
        }
    }

    /// Returns whether the call succeeded.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.error == SBI_SUCCESS
    }

    /// Returns whether the call failed.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        self.error != SBI_SUCCESS
    }

    /// Returns the error of the call.
    ///
    /// A successful call has no error.
    #[must_use]
    pub const fn error_code(&self) -> Option<SbiError> {
        SbiError::from_code(self.error)
    }

    /// Returns the value of a successful call.
    ///
    /// # Errors
    ///
    /// Returns the error of the call when the call failed.
    pub const fn into_result(self) -> Result<usize, SbiError> {
        match SbiError::from_code(self.error) {
            Some(error) => Err(error),
            None => Ok(self.value),
        }
    }
}

/// A supervisor binary interface call.
///
/// The call passes the extension identifier, the function
/// identifier and up to six arguments. The structure describes
/// the call; it does not perform it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SbiEcall {
    /// The identifier of the extension.
    pub extension: u64,
    /// The identifier of the function inside the extension.
    pub function: u64,
    /// The arguments of the call.
    pub args: [u64; 6],
}

impl SbiEcall {
    /// Creates a call without arguments.
    #[must_use]
    pub const fn new(extension: u64, function: u64) -> Self {
        Self {
            extension,
            function,
            args: [0; 6],
        }
    }

    /// Creates a call with arguments.
    #[must_use]
    pub const fn with_args(extension: u64, function: u64, args: [u64; 6]) -> Self {
        Self {
            extension,
            function,
            args,
        }
    }

    /// Returns the registers of the call.
    ///
    /// The registers are returned in the order of the argument
    /// registers, followed by the register of the function
    /// identifier and the register of the extension identifier.
    #[must_use]
    pub const fn encode(&self) -> [u64; SBI_ECALL_REGISTER_COUNT] {
        [
            self.args[0],
            self.args[1],
            self.args[2],
            self.args[3],
            self.args[4],
            self.args[5],
            self.function,
            self.extension,
        ]
    }

    /// Decodes a call from its registers.
    #[must_use]
    pub const fn decode(registers: [u64; SBI_ECALL_REGISTER_COUNT]) -> Self {
        Self {
            extension: registers[SBI_ECALL_EXTENSION],
            function: registers[SBI_ECALL_FUNCTION],
            args: [
                registers[SBI_ECALL_ARGUMENT_0],
                registers[1],
                registers[2],
                registers[3],
                registers[4],
                registers[5],
            ],
        }
    }

    /// Creates a probe for an extension.
    ///
    /// The probe uses the base extension and reports the
    /// extension identifier in its first argument.
    #[must_use]
    pub const fn probe(extension: u64) -> Self {
        Self::with_args(
            SBI_EXTENSION_BASE,
            SBI_BASE_FN_PROBE_EXTENSION,
            [extension, 0, 0, 0, 0, 0],
        )
    }
}
