//!     show-options [-AgHpqsvw] [-t target-pane] [option]
//!                   (alias: show)
//!             Show the pane options (or a single option if option is
//!             provided) with -p, the window options with -w, the server
//!             options with -s, otherwise the session options.  If the
//!             option is not a user option, -w or -s may be unnecessary -
//!             tmux will infer the type from the option name, assuming -w
//!             for pane options.  Global session or window options are
//!             listed if -g is used.  -v shows only the option value, not
//!             the name.  If -q is set, no error will be returned if
//!             option is unset.  -H includes hooks (omitted by default).
//!             -A includes options inherited from a parent set of options,
//!             such options are marked with an asterisk.
//!
//! Tmux boundary conditions
//!
//! * Tmux Option:
//!     * Server Options (`-s`)
//!     * Session Options (otherwise ``)
//!         * global (`-g`)
//!     * Window Options (`-w`)
//!         * global (`-g`)
//!     * Pane Options (`-p`)
//!
//! * User Option (`@name`)
//!
//! Get:
//! * all
//! * single one
//! * value with name
//! * value without name
//! * inherited from parent (`*`)
//!
//! Set:
//! * single one with name and value
//!
// mb separated crate later, and tmux_commands as underlying layer
//
pub mod common;

#[cfg(feature = "tmux_3_1")]
pub mod pane;
#[cfg(feature = "tmux_1_2")]
pub mod server;
#[cfg(feature = "tmux_1_0")]
pub mod session;
#[cfg(feature = "tmux_1_2")]
pub mod window;

pub use crate::options::common::*;

#[cfg(feature = "tmux_3_1")]
pub use crate::options::pane::*;
#[cfg(feature = "tmux_1_2")]
pub use crate::options::server::*;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session::*;
#[cfg(feature = "tmux_1_2")]
pub use crate::options::window::*;

#[cfg(feature = "tmux_1_0")]
pub struct Options {
    pub server_options: ServerOptions,
    pub session_options: SessionOptions,
    pub window_options: WindowOptions,
    #[cfg(feature = "tmux_3_1")]
    pub pane_options: PaneOptions,
}
