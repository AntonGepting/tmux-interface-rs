// doc note parts/modules commands builders and parsers
//
//!
//! Command builders and output parsers
//!
//! Get
//! ```text
//! show-options [-AgHpqsvw] [-t target-pane] [option]
//! (alias: show)
//! Show the pane options (or a single option if option is
//! provided) with -p, the window options with -w, the server
//! options with -s, otherwise the session options.  If the
//! option is not a user option, -w or -s may be unnecessary -
//! tmux will infer the type from the option name, assuming -w
//! for pane options.  Global session or window options are
//! listed if -g is used.  -v shows only the option value, not
//! the name.  If -q is set, no error will be returned if
//! option is unset.  -H includes hooks (omitted by default).
//! -A includes options inherited from a parent set of options,
//! such options are marked with an asterisk.
//! ```
//!
//! Set
//! ```text
//! set-option [-aFgopqsuUw] [-t target-pane] option value
//!       (alias: set)
//! Set a pane option with -p, a window option with -w, a
//! server option with -s, otherwise a session option.  If the
//! option is not a user option, -w or -s may be unnecessary -
//! tmux will infer the type from the option name, assuming -w
//! for pane options.  If -g is given, the global session or
//! window option is set.
//!
//! -F expands formats in the option value.  The -u flag unsets
//! an option, so a session inherits the option from the global
//! options (or with -g, restores a global option to the
//! default).  -U unsets an option (like -u) but if the option
//! is a pane option also unsets the option on any panes in the
//! window.  value depends on the option and may be a number, a
//! string, or a flag (on, off, or omitted to toggle).
//!
//! The -o flag prevents setting an option that is already set
//! and the -q flag suppresses errors about unknown or
//! ambiguous options.
//!
//! With -a, and if the option expects a string or a style,
//! value is appended to the existing setting.  For example:
//!
//!       set -g status-left "foo"
//!       set -ag status-left "bar"
//!
//! Will result in ‘foobar’.  And:
//!
//!       set -g status-style "bg=red"
//!       set -ag status-style "fg=blue"
//!
//! Will result in a red background and blue foreground.
//! Without -a, the result would be the default background and
//! a blue foreground.
//! ```
//!
//! * Tmux Options
//!
//! * by [Scope](#scope)
//!     * Global
//!     * Local
//!
//! * by Access
//!     * All
//!     * Single
//!     * Multiple Selective
//!
//! * by Object
//!     * [`Server`](self::server)
//!     * [`Session`](self::session)
//!     * [`Window`](self::window)
//!     * [`Pane`](self::pane)
//!
//! * by Setting Methods
//!     * Set
//!     * Unset
//!     * Toggle (for `on | off | ...` options)
//!
//! Tmux Options
//! * [`Server`](self::server)
//!     * [`GetServerOption`]
//!     * [`GetServerOptionValue`]
//!     * [`SetServerOption`]
//! * [`Session`](self::session)
//!     * [`Builder`](self::session::builder)
//!         * [`global`](self::session::builder::global)
//!             * [`GetGlobalSessionOption`]
//!             * [`GetGlobalSessionOptionValue`]
//!             * [`SetGlobalSessionOption`]
//!         * [`local`](self::session::builder::local)
//!             * [`GetLocalSessionOption`]
//!             * [`GetLocalSessionOptionValue`]
//!             * [`SetLocalSessionOption`]
//!     * [`Parser`](self::session::parser)
//!         * [`SessionOptionsCtl`](self::session::parser::session_options_ctl)
//!         * [`GlobalSessionOptionsCtl`](self::session::parser::global_session_options_ctl)
//!         * [`LocalSessionOptionsCtl`](self::session::parser::local_session_options_ctl)
//! * [`Window`](self::window)
//!     * [`Builder`](self::window::builder)
//!         * [`global`](self::window::builder::global)
//!             * [`GetGlobalWindowOption`]
//!             * [`GetGlobalWindowOptionValue`]
//!             * [`SetGlobalWindowOption`]
//!         * [`local`](self::window::builder::local)
//!             * [`GetLocalWindowOption`]
//!             * [`GetLocalWindowOptionValue`]
//!             * [`SetLocalWindowOption`]
//!     * [`Parser`](self::window::parser)
//!         * [`WindowOptionsCtl`](self::window::parser::window_options_ctl)
//!         * [`GlobalWindowOptionsCtl`](self::window::parser::global_window_options_ctl)
//! * [`Pane`](self::pane)
//!     * [`GetPaneOption`]
//!     * [`GetPaneOptionValue`]
//!     * [`SetPaneOption`]
//!
//!
//! **Table**: Tmux Options
//! <table>
//!   <thead>
//!     <tr>
//!       <th>Tmux Options</th>
//!       <th colspan="2">Scope</th>
//!     </tr>
//!   </thead>
//!   <tbody>
//!     <tr>
//!       <td></td>
//!       <td>Global (`show/set -g`)</td>
//!       <td>Local</td>
//!     </tr>
//!     <tr>
//!       <td>ServerOptions</td>
//!       <td></td>
//!       <td>x (`show/set -s`)</td>
//!     </tr>
//!     <tr>
//!       <td>SessionOptions</td>
//!       <td>x (`show/set -g`) </td>
//!       <td>x (`show/set`)</td>
//!     </tr>
//!     <tr>
//!       <td>WindowOptions</td>
//!       <td>x (`show/set -wg`)</td>
//!       <td>x (`show/set -w`)</td>
//!     </tr>
//!     <tr>
//!       <td>PaneOptions</td>
//!       <td></td>
//!       <td>x (`show/set -p`)</td>
//!     </tr>
//!   </tbody>
//! </table>
//!
//
// Tmux boundary conditions
//
// * Tmux Option:
//     * Server Options (`-s`)
//       > server options which do not apply to any particular window or session or pane
//         * absolute (no global/local differentiation)
//           * get
//             * single
//               * name value
//               * value
//             * all
//               * name value
//               * value
//           * set
//           * toggle (on/off {default}/off) if no value specified
//     * Session Options (otherwise ``)
//       > Sessions which do not have a particular option configured inherit the value from the global session options
//         * local (``)
//           * get
//             * single
//               * name value
//               * value
//             * all
//               * name value
//               * value
//           * set
//             * name value
//         * global (`-g`)
//           * get
//             * single
//               * name value
//               * value
//             * all
//               * name value
//               * value
//           * set
//             * name value
//     * Window Options (`-w`)
//       > There is also a set of global window options from which any unset window or pane options are inherited
//         * local (``)
//           * get
//             * single
//               * name value
//               * value
//             * all
//               * name value
//               * value
//           * set
//             * name value
//         * global (`-g`)
//           * get
//             * single
//               * name value
//               * value
//             * all
//               * name value
//               * value
//           * set
//             * name value
//     * Pane Options (`-p`)
//       > Pane options inherit from window options
//         * absolute (no global/local differentiation)
//           * get
//             * single
//               * name value
//               * value
//             * all
//               * name value
//               * value
//           * set
//             * name value
//
// * User Option (`@name`)
//
// Get:
// * inherited from parent (`*`)
//
// Set:
// * single one with name and value
//
// 1. need subclassing global / local options -> custom Deref trait
// 2. need user methods abstraction (so user can't access options of wrong object) -> newtype wrapper type
// 3. need builder / parser for single option
// mb separated crate later, and tmux_commands as underlying layer
//
//! [`GetOptionExt`] trait implements common getter method for all options
//! [`SetOptionExt`] trait implements common setter methods
//!
//! [`GetUserOption`] traits implements getter method for user option (`@user-option-name value`)
//! [`GetUserOptions`] traits implements getter method for user option (`@user-option-name value`),
//! used by selective options builder
//!
use std::iter::Iterator;

pub mod common;
pub mod tmux_options_map;

#[cfg(feature = "tmux_3_1")]
pub mod pane;
#[cfg(feature = "tmux_1_2")]
pub mod server;
#[cfg(feature = "tmux_1_0")]
pub mod session;
#[cfg(feature = "tmux_1_2")]
pub mod window;

pub use crate::options::common::*;
pub use user_option::*;

#[cfg(feature = "tmux_3_1")]
pub use crate::options::pane::*;
#[cfg(feature = "tmux_1_2")]
pub use crate::options::server::*;
#[cfg(feature = "tmux_1_0")]
pub use crate::options::session::*;
#[cfg(feature = "tmux_1_2")]
pub use crate::options::window::*;

//#[cfg(feature = "tmux_1_0")]
//pub struct Options<'a> {
////pub server_options: ServerOptions<'a>,
////pub session_options: SessionOptions<'a>,
////pub global_session_options: SessionOptions<'a>,
//pub window_options: WindowOptions,
//pub global_window_options: WindowOptions,
//#[cfg(feature = "tmux_3_1")]
//pub pane_options: PaneOptions,
//}

//pub struct OptionsController {
//}
//pub fn set_server
//}
//

pub mod get_option_ext;
pub mod set_option_ext;

pub use get_option_ext::GetOptionExt;
pub use set_option_ext::SetOptionExt;
