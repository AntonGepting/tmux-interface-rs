// doc note parts/modules commands builders and parsers
//
//!
//! Command builders and output parsers
//!
//! # Table Of Contents
//! * 1. Quick Start
//! * 2. Developer Information
//!     * 2.1. Tmux Manual
//!     * 2.2. Structure
//!     * 2.3. Implementation
//!     * 2.4. Modules Hierarchy
//!
//! # 1. Quick Start
//!
//! ## Examples
//!
//!
//! # 2. Developer Information
//!
//! ## 2.1. Tmux Manual
//!
//! Get command
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
//! Set command
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
//! # 2.2. Structure
//!
//! Briefly, abstractions for working procedures with Tmux options can be done using following principles:
//!
//! * by [Scope](#scope) (`-g` flag)
//!     * Global (`-g` flag)
//!     * Local (`` flag omitted)
//!
//! * by access type (`[option]` parameter)
//!     * All (`[option]` parameter omitted)
//!     * Value only (`-v` flag)
//!     * Single (`[option]` parameter)
//!     * Multiple Selective (multiple `show-option`, `set-option` commands)
//!
//! * by object type (`-s`, ` `, `-w`, `-p` flags)
//!     * [`Server`](self::server) (`-s` flag)
//!     * [`Session`](self::session) (` ` flag omitted)
//!     * [`Window`](self::window) (`-w` flag)
//!     * [`Pane`](self::pane) (`-p` flag)
//!
//! * by modifiying methods (`[value]` parameter and `-u` flag)
//!     * Set (`[value]` parameter is set)
//!     * Unset (`-u` flag)
//!     * Toggle (for `on | off | ...` options) (`[value] parameter omitted`)
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
//! # 2.3. XXX
//!
//! ```
//! let server_options = GetServerOption::
//! ```
//!
//! # 2.3. Control
//!
//! * [`ServerOptionsCtl`]
//! * [`SessionOptionsCtl`]
//!     * [`GlobalSessionOptionsCtl`]
//!     * [`LocalSessionOptionsCtl`]
//! * [`WindowOptionsCtl`]
//!     * [`GlobalWindowOptionsCtl`]
//!     * [`LocalWindowOptionsCtl`]
//! * [`PaneOptionsCtl`]
//!
//! # 2.3. Implementation
//!
//! implementations derived using previous prinnciples
//!
//! * Common
//!     * [`UserOptions`](self::common::user_option)
//!         * [`GetUserOption`] getter method for user option (`@user-option-name value`)
//!         * [`GetUserOptions`] getter method for user options (`@user-option-name value`)
//!         * [`SetUserOption`] setter method for user option (`@user-option-name value`)
//!         * [`SetUserOptions`] setter method for user options (`@user-option-name value`)
//!     * [`GetOptionExt`] common getter method for all options
//!     * [`SetOptionExt`] common setter method for all options
//! * [`Server`](self::server)
//!     * [`GetServerOption`]
//!     * [`GetServerOptionValue`]
//!     * [`SetServerOption`]
//!     * [`Ctl`](self::server::ctl)
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
//!     * [`Ctl`](self::session::ctl)
//!         * [`SessionOptionsCtl`](self::session::ctl::session_options_ctl)
//!         * [`GlobalSessionOptionsCtl`](self::session::ctl::global_session_options_ctl)
//!         * [`LocalSessionOptionsCtl`](self::session::ctl::local_session_options_ctl)
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
//!     * [`Ctl`](self::window::ctl)
//!         * [`WindowOptionsCtl`](self::window::ctl::window_options_ctl)
//!         * [`LocalWindowOptionsCtl`](self::window::ctl::local_window_options_ctl)
//!         * [`GlobalWindowOptionsCtl`](self::window::ctl::global_window_options_ctl)
//! * [`Pane`](self::pane)
//!     * [`GetPaneOption`]
//!     * [`GetPaneOptionValue`]
//!     * [`SetPaneOption`]
//!     * [`Ctl`](self::pane::ctl)
//!
//!
//! ## 2.4. Modules Hierarchy
//!
//! Options submodules, traits and structures schematic hierarchy
//! ```text
//! Get/Set options control traits
//!                           Global
//!                           +-------------------------+     +------------------------+
//!                           | GlobalSessionOptionsCtl |     | GlobalWindowOptionsCtl |
//!                           +-------------------------+     +------------------------+
//!                             * .get_<OPTION>()               ...
//!                             * .set_<OPTION>()
//!                             * ...
//!                             * .get_all()
//!                             * .set_all()
//!                           Local
//!                           +------------------------+      +-----------------------+
//!                           | LocalSessionOptionsCtl |      | LocalWindowOptionsCtl |
//!                           +------------------------+      +-----------------------+
//!                            ...                             ...
//!
//!  +------------------+     +-------------------+           +------------------+          +----------------+
//!  | ServerOptionsCtl |     | SessionOptionsCtl |           | WindowOptionsCtl |          | PaneOptionsCtl |
//!  +------------------+     +-------------------+           +------------------+          +----------------+
//!   ...                      ...                             ...                           ...
//!
//! Get/Set options command builder traits (by target, by scope, by access)
//!
//!                           Global
//!                            single option
//!                           +------------------------+      +-----------------------+
//!                           | GetGlobalSessionOption |      | GetGlobalWindowOption |
//!                           +------------------------+      +-----------------------+
//!                            ...                             ...
//!                            single value only
//!                           +-----------------------------+ +----------------------------+
//!                           | GetGlobalSessionOptionValue | | GetGlobalWindowOptionValue |
//!                           +-----------------------------+ +----------------------------+
//!                            ...                             ...
//!                            multiple options
//!                           +-------------------------+     +------------------------+
//!                           | GetGlobalSessionOptions |     | GetGlobalWindowOptions |
//!                           +-------------------------+     +------------------------+
//!                            ...                             ...
//!                            single option
//!                           +------------------------+      +-----------------------+
//!                           | SetGlobalSessionOption |      | SetGlobalWindowOption |
//!                           +------------------------+      +-----------------------+
//!                            ...                             ...
//!                            multiple options
//!                           +-------------------------+     +------------------------+
//!                           | SetGlobalSessionOptions |     | SetGlobalWindowOptions |
//!                           +-------------------------+     +------------------------+
//!                            ...                             ...
//!  Local
//!   single option
//!  +----------------+       +-----------------------+       +----------------------+       +---------------+
//!  | GetServerOpton |       | GetLocalSessionOption |       | GetLocalWindowOption |       | GetPaneOption |
//!  +----------------+       +-----------------------+       +----------------------+       +---------------+
//!   ...                      ...                             ...                            ...
//!   single value only
//!  +---------------------+  +----------------------------+  +---------------------------+  +--------------------+
//!  | GetServerOptonValue |  | GetLocalSessionOptionValue |  | GetLocalWindowOptionValue |  | GetPaneOptionValue |
//!  +---------------------+  +----------------------------+  +---------------------------+  +--------------------+
//!   ...                      ...                             ...                            ...
//!   multiple options
//!  +-----------------+      +------------------------+      +-----------------------+      +----------------+
//!  | GetServerOptons |      | GetLocalSessionOptions |      | GetLocalWindowOptions |      | GetPaneOptions |
//!  +-----------------+      +------------------------+      +-----------------------+      +----------------+
//!   ...                      ...                             ...                            ...
//!   single option
//!  +----------------+       +-----------------------+       +----------------------+       +---------------+
//!  | SetServerOpton |       | SetLocalSessionOption |       | SetLocalWindowOption |       | SetPaneOption |
//!  +----------------+       +-----------------------+       +----------------------+       +---------------+
//!   ...                      ...                             ...                            ...
//!   multiple options
//!  +-----------------+      +------------------------+      +-----------------------+      +----------------+
//!  | SetServerOptons |      | SetLocalSessionOptions |      | SetLocalWindowOptions |      | SetPaneOptions |
//!  +-----------------+      +------------------------+      +-----------------------+      +----------------+
//!   ...                      ...                             ...                            ...
//! Options Structures
//!  +---------------+        +----------------+              +---------------+              +-------------+
//!  | ServerOptions |        | SessionOptions |              | WindowOptions |              | PaneOptions |
//!  +---------------+        +----------------+              +---------------+              +-------------+
//!   ...                      ...                             ...                            ...
//! Get/Set user options command builder traits (custom get/set implementations for user options `@name value`)
//!  +---------------+            +---------------+
//!  | GetUserOption |            | SetUserOption |
//!  +---------------+            +---------------+
//!   ...                          ...
//!  +----------------+           +----------------+
//!  | GetUserOptions |           | SetUserOptions |
//!  +----------------+           +----------------+
//!   ...                          ...
//! Get/Set options command builder traits (custom get/set implementations for server/session/window/pane)
//!  +--------------+             +--------------+
//!  | GetOptionExt |             | SetOptionExt |
//!  +--------------+             +--------------+
//!   ...                          ...
//!
//! Get/Set options commands builder structures (named methods for server/session/window/pane and other)
//!  +------------+               +-----------+
//!  | ShowOption |               | SetOption |
//!  +------------+               +-----------+
//!   ...                          ...
//!
//! Tmux interface command structures (command, flags, options, parameters)
//!  +-------------+
//!  | TmuxCommand |
//!  +-------------+
//!   ...
//!  +--------------+
//!  | TmuxCommands |
//!  +--------------+
//!   ...
//! Tmux command (resulting command as `std::process::Command` or `String`)
//!  +--------------------------+ +---------------------------------+
//!  | `show [-FLAGS] <OPTION>` | | `set [-FLAGS] <OPTION> <VALUE>` |
//!  +--------------------------+ +---------------------------------+
//! ```
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
pub mod get_option_ext;
pub mod set_option_ext;

pub use common::*;

pub use get_option_ext::GetOptionExt;
pub use set_option_ext::SetOptionExt;

#[cfg(feature = "tmux_3_1")]
pub mod pane;
#[cfg(feature = "tmux_1_2")]
pub mod server;
#[cfg(feature = "tmux_1_0")]
pub mod session;
#[cfg(feature = "tmux_1_2")]
pub mod window;

#[cfg(feature = "tmux_3_1")]
pub use pane::*;
#[cfg(feature = "tmux_1_2")]
pub use server::*;
#[cfg(feature = "tmux_1_0")]
pub use session::*;
#[cfg(feature = "tmux_1_2")]
pub use window::*;

#[cfg(test)]
#[path = "."]
mod options_tests {
    pub mod get_option_ext_tests;

    pub mod set_option_ext_tests;
}
