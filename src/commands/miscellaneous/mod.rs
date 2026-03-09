// auto-generated file
//

/// All functions from man tmux "Miscellaneous" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#MISCELLANEOUS))
///
use crate::TmuxCommand;

#[cfg(feature = "tmux_0_8")]
pub mod clock_mode;
#[cfg(feature = "tmux_0_8")]
pub mod clock_mode_macro;

#[cfg(feature = "tmux_1_5")]
pub mod if_shell;
#[cfg(feature = "tmux_1_5")]
pub mod if_shell_macro;

#[cfg(feature = "tmux_0_8")]
pub mod lock_server;
#[cfg(feature = "tmux_0_8")]
pub mod lock_server_macro;

//#[cfg(feature = "tmux_1_0")]
//pub mod server_info;
//#[cfg(feature = "tmux_1_0")]
//pub mod set_password;

pub mod run_shell;

pub mod run_shell_macro;

#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
pub mod server_info;
#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
pub mod server_info_macro;

#[cfg(feature = "tmux_1_8")]
pub mod wait_for;
#[cfg(feature = "tmux_1_8")]
pub mod wait_for_macro;

#[cfg(feature = "tmux_0_8")]
pub use clock_mode::ClockMode;

#[cfg(feature = "tmux_1_5")]
pub use if_shell::{If, IfShell};

#[cfg(feature = "tmux_0_8")]
pub use lock_server::{Lock, LockServer};

pub use run_shell::{Run, RunShell};

#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
pub use server_info::{Info, ServerInfo};

#[cfg(feature = "tmux_1_8")]
pub use wait_for::{Wait, WaitFor};

#[cfg(test)]
#[path = "."]
mod miscellaneous_tests {

    #[cfg(feature = "tmux_0_8")]
    mod clock_mode_tests;

    #[cfg(feature = "tmux_1_5")]
    mod if_shell_tests;

    #[cfg(feature = "tmux_0_8")]
    mod lock_server_tests;

    mod run_shell_tests;

    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    mod server_info_tests;

    #[cfg(feature = "tmux_1_8")]
    mod wait_for_tests;
}

/// All functions from man tmux "Miscellaneous" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#MISCELLANEOUS))
impl<'a> TmuxCommand<'a> {
    #[cfg(feature = "tmux_0_8")]
    pub fn clock_mode() -> ClockMode<'a> {
        ClockMode::new()
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn if_shell() -> IfShell<'a> {
        IfShell::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn lock_server() -> LockServer {
        LockServer::new()
    }

    pub fn run_shell() -> RunShell<'a> {
        RunShell::new()
    }

    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    pub fn server_info() -> ServerInfo {
        ServerInfo::new()
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn wait_for() -> WaitFor<'a> {
        WaitFor::new()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<ClockMode<'a>> for TmuxCommand<'a> {
    fn from(item: ClockMode<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_1_5")]
impl<'a> From<IfShell<'a>> for TmuxCommand<'a> {
    fn from(item: IfShell<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<LockServer> for TmuxCommand<'a> {
    fn from(item: LockServer) -> Self {
        item.build()
    }
}

impl<'a> From<RunShell<'a>> for TmuxCommand<'a> {
    fn from(item: RunShell<'a>) -> Self {
        item.build()
    }
}

#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
impl<'a> From<ServerInfo> for TmuxCommand<'a> {
    fn from(item: ServerInfo) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_1_8")]
impl<'a> From<WaitFor<'a>> for TmuxCommand<'a> {
    fn from(item: WaitFor<'a>) -> Self {
        item.build()
    }
}
