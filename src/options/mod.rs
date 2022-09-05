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
//! * by Methods
//!     * All
//!     * Single
//!     * Multiple Selective
//!
//! * by Object
//!     * [`Server`](super::options::server)
//!     * [`Session`]()
//!     * [`Window`]()
//!     * [`Pane`]()
//!
//! * by Setting
//!     * Set
//!     * Unset
//!     * Toggle (for `on | off | ...` options)
//!
//! Scope
//!
//! * Global Options
//!     * Session
//!         * [`GetGlobalSessionOption`](crate::GetGlobalSessionOption)
//!         * [`SetGlobalSessionOption`](crate::SetGlobalSessionOption)
//!     * Window
//!         * [`GetGlobalWindowOption`](crate::GetGlobalWindowOption)
//!         * [`SetGlobalWindowOption`](crate::SetGlobalWindowOption)
//!
//! * Local Options
//!     * Server
//!         * [`GetServerOption`](crate::GetServerOption)
//!         * [`SetServerOption`](crate::SetServerOption)
//!     * Session
//!         * [`GetLocalSessionOption`](crate::GetLocalSessionOption)
//!         * [`SetLocalSessionOption`](crate::SetLocalSessionOption)
//!     * Window
//!         * [`GetLocalSessionOption`](crate::GetLocalSessionOption)
//!         * [`SetLocalSessionOption`](crate::SetLocalSessionOption)
//!     * Pane
//!         * [`GetPaneOption`](crate::GetPaneOption)
//!         * [`SetPaneOption`](crate::SetPaneOption)
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
//!       <td>Global</td>
//!       <td>Local</td>
//!     </tr>
//!     <tr>
//!       <td>ServerOptions</td>
//!       <td></td>
//!       <td>x</td>
//!     </tr>
//!     <tr>
//!       <td>SessionOptions</td>
//!       <td>x</td>
//!       <td>x</td>
//!     </tr>
//!     <tr>
//!       <td>WindowOptions</td>
//!       <td>x</td>
//!       <td>x</td>
//!     </tr>
//!     <tr>
//!       <td>PaneOptions</td>
//!       <td></td>
//!       <td>x</td>
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

use crate::{SetOption, ShowOptions, TmuxCommand, TmuxCommands};
use std::borrow::Cow;
use std::fmt;

/// common trait for getting options, allowing different implementations for different object options
pub trait GetOptionExt {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new().option(name).value().build()
    }
}

// TODO: optimize set/set_ext are the same
/// common trait for setting options, allowing different implementations for different object options
pub trait SetOptionExt {
    fn set<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        match value {
            Some(data) => Self::set_ext(name, Some(data)),
            None => Self::unset(name),
        }
    }

    fn unset<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        SetOption::new().option(name).unset().build()
    }

    // unset if value = None
    fn set_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        //(self.setter)(name.into(), value.map(|s| s.into()))
        let cmd = match value {
            Some(data) => SetOption::new().option(name).value(data),
            None => SetOption::new().option(name),
        };
        cmd.build()
    }

    fn set_array<'a, S: fmt::Display>(name: S, value: Option<Vec<String>>) -> TmuxCommands<'a> {
        let mut cmds = TmuxCommands::new();
        if let Some(data) = value {
            for (i, item) in data.iter().enumerate() {
                cmds.push(Self::set(format!("{}[{}]", name, i), Some(item.to_owned())));
            }
        } else {
            cmds.push(Self::set(format!("{}", name), Some("")));
        }
        cmds
    }
}
