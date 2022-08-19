//!
//! Command builders and output parsers
//!
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
//!       > server options which do not apply to any particular window or session or pane
//!         * absolute (no global/local differentiation)
//!           * get
//!             * single
//!               * name value
//!               * value
//!             * all
//!               * name value
//!               * value
//!           * set
//!           * toggle (on/off {default}/off) if no value specified
//!     * Session Options (otherwise ``)
//!       > Sessions which do not have a particular option configured inherit the value from the global session options
//!         * local (``)
//!           * get
//!             * single
//!               * name value
//!               * value
//!             * all
//!               * name value
//!               * value
//!           * set
//!             * name value
//!         * global (`-g`)
//!           * get
//!             * single
//!               * name value
//!               * value
//!             * all
//!               * name value
//!               * value
//!           * set
//!             * name value
//!     * Window Options (`-w`)
//!       > There is also a set of global window options from which any unset window or pane options are inherited
//!         * local (``)
//!           * get
//!             * single
//!               * name value
//!               * value
//!             * all
//!               * name value
//!               * value
//!           * set
//!             * name value
//!         * global (`-g`)
//!           * get
//!             * single
//!               * name value
//!               * value
//!             * all
//!               * name value
//!               * value
//!           * set
//!             * name value
//!     * Pane Options (`-p`)
//!       > Pane options inherit from window options
//!         * absolute (no global/local differentiation)
//!           * get
//!             * single
//!               * name value
//!               * value
//!             * all
//!               * name value
//!               * value
//!           * set
//!             * name value
//!
//! * User Option (`@name`)
//!
//! Get:
//! * inherited from parent (`*`)
//!
//! Set:
//! * single one with name and value
//!
//! 1. need subclassing global / local options -> custom Deref trait
//! 2. need user methods abstraction (so user can't access options of wrong object) -> newtype wrapper type
//! 3. need builder / parser for single option
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
pub struct Options<'a> {
    pub server_options: ServerOptions<'a>,
    pub session_options: SessionOptions,
    pub global_session_options: SessionOptions,
    pub window_options: WindowOptions,
    pub global_window_options: WindowOptions,
    #[cfg(feature = "tmux_3_1")]
    pub pane_options: PaneOptions,
}

//pub struct OptionsController {
//}
//pub fn set_server
//}
//
