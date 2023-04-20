use crate::{SetClipboard, Switch};

// total num: 16
/// `backspace
#[cfg(feature = "tmux_3_1")]
pub const BACKSPACE: &str = "backspace";
/// `buffer-limit`
#[cfg(feature = "tmux_1_5")]
pub const BUFFER_LIMIT: &str = "buffer-limit";
/// `command-alias`
#[cfg(feature = "tmux_2_4")]
pub const COMMAND_ALIAS: &str = "command-alias";
/// `default-terminal`
#[cfg(feature = "tmux_2_1")]
pub const DEFAULT_TERMINAL: &str = "default-terminal";
/// `copy-command`
#[cfg(feature = "tmux_3_2")]
pub const COPY_COMMAND: &str = "copy-command";
/// `escape-time`
#[cfg(feature = "tmux_1_2")]
pub const ESCAPE_TIME: &str = "escape-time";
/// `editor`
#[cfg(feature = "tmux_3_2")]
pub const EDITOR: &str = "editor";
/// `exit-empty`
#[cfg(feature = "tmux_2_7")]
pub const EXIT_EMPTY: &str = "exit-empty";
/// `exit-unattached`
#[cfg(feature = "tmux_1_4")]
pub const EXIT_UNATTACHED: &str = "exit-unattached";
/// `extended-keys`
#[cfg(feature = "tmux_3_2")]
pub const EXTENDED_KEYS: &str = "extended-keys";
/// `focus-events`
#[cfg(feature = "tmux_1_9")]
pub const FOCUS_EVENTS: &str = "focus-events";
/// `history-file`
#[cfg(feature = "tmux_2_1")]
pub const HISTORY_FILE: &str = "history-file";
/// `message-limit`
#[cfg(feature = "tmux_2_0")]
pub const MESSAGE_LIMIT: &str = "message-limit";
/// `prompt-history-limit`
#[cfg(feature = "tmux_3_3")]
pub const PROMPT_HISTORY_LIMIT: &str = "prompt-history-limit";
/// `set-clipboard`
#[cfg(feature = "tmux_1_5")]
pub const SET_CLIPBOARD: &str = "set-clipboard";
/// `terminal-features`
#[cfg(feature = "tmux_3_2")]
pub const TERMINAL_FEATURES: &str = "terminal-features";
/// `terminal-overrides`
#[cfg(feature = "tmux_2_0")]
pub const TERMINAL_OVERRIDES: &str = "terminal-overrides";
/// `user-keys`
#[cfg(feature = "tmux_3_0")]
pub const USER_KEYS: &str = "user-keys";
/// `quiet`
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
pub const QUIET: &str = "quiet";
/// `detach-on-destroy [on | off]
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
pub const DETACH_ON_DESTROY: &str = "detach-on-destroy";

pub const USER_OPTION_MARKER: &str = "@";
pub const SEPARATOR: &str = " ";

// XXX: Option<T> or T for defaults?

/// tmux ^3.1:
/// ```text
/// backspace C-?
/// ```
// FIXME
// octal `\177` = `127` = delete?
#[cfg(feature = "tmux_3_1")]
pub const BACKSPACE_DEFAULT: &str = "C-?";

/// tmux ^1.5:
/// ```text
/// buffer-limit 50
/// ```
#[cfg(feature = "tmux_1_5")]
pub const BUFFER_LIMIT_DEFAULT: usize = 50;

/// tmux ^2.4 v2.6:
/// ```text
/// command-alias[0] split-pane=split-window
/// command-alias[1] splitp=split-window
/// command-alias[2] "server-info=show-messages -JT"
/// command-alias[3] "info=show-messages -JT"
/// ```
///
/// tmux ^2.6:
/// ```text
/// command-alias[0] split-pane=split-window
/// command-alias[1] splitp=split-window
/// command-alias[2] "server-info=show-messages -JT"
/// command-alias[3] "info=show-messages -JT"
/// command-alias[4] "choose-window=choose-tree -w"
/// command-alias[5] "choose-session=choose-tree -s"
/// ```
#[cfg(all(feature = "tmux_2_4", not(feature = "tmux_2_6")))]
pub const COMMAND_ALIAS_DEFAULT: [&str; 4] = [
    "split-pane=split-window",
    "splitp=split-window",
    "\"server-info=show-messages -JT\"",
    "\"info=show-messages -JT\"",
];
#[cfg(feature = "tmux_2_6")]
pub const COMMAND_ALIAS_DEFAULT: [&str; 6] = [
    "split-pane=split-window",
    "splitp=split-window",
    "\"server-info=show-messages -JT\"",
    "\"info=show-messages -JT\"",
    "\"choose-window=choose-tree -w\"",
    "\"choose-session=choose-tree -s\"",
];

/// tmux ^3.2:
/// ```text
/// copy-command
/// ```
#[cfg(feature = "tmux_3_2")]
pub const COPY_COMMAND_DEFAULT: &str = "";

// tmux.h
/// tmux ^2.1:
/// ```text
/// default-terminal screen
/// ```
#[cfg(feature = "tmux_2_1")]
pub const DEFAULT_TERMINAL_DEFAULT: &str = "screen";

/// tmux ^1.2:
/// ```text
/// escape-time 500
/// ```
#[cfg(feature = "tmux_1_2")]
pub const ESCAPE_TIME_DEFAULT: usize = 500;

// compat.h
/// tmux ^3.2:
/// ```text
/// editor /usr/bin/vi
/// ```
#[cfg(feature = "tmux_3_2")]
pub const EDITOR_DEFAULT: &str = "/usr/bin/vi";

/// tmux ^2.7:
/// ```text
/// exit-empty on
/// ```
#[cfg(feature = "tmux_2_7")]
pub const EXIT_EMPTY_DEFAULT: Switch = Switch::On;

/// tmux ^1.4:
/// ```text
/// exit-unattached off
/// ```
#[cfg(feature = "tmux_1_4")]
pub const EXIT_UNATTACHED_DEFAULT: Switch = Switch::Off;

/// tmux ^3.2:
/// ```text
/// extended-keys off
/// ```
#[cfg(feature = "tmux_3_2")]
pub const EXTENDED_KEYS_DEFAULT: Switch = Switch::Off;

/// tmux ^1.9:
/// ```text
/// focus-events off
/// ```
#[cfg(feature = "tmux_1_9")]
pub const FOCUS_EVENTS_DEFAULT: Switch = Switch::Off;

/// tmux ^2.1:
/// `""`
/// ```text
/// history-file ""
/// ```
#[cfg(feature = "tmux_2_1")]
pub const HISTORY_FILE_DEFAULT: &str = "";

/// tmux ^3.2:
/// ```text
/// message-limit 1000
/// ```
///
/// tmux ^2.0 v3.2:
/// ```text
/// message-limit 100
/// ```
#[cfg(all(feature = "tmux_2_0", not(feature = "tmux_3_2")))]
pub const MESSAGE_LIMIT_DEFAULT: usize = 100;
#[cfg(feature = "tmux_3_2")]
pub const MESSAGE_LIMIT_DEFAULT: usize = 1000;

/// tmux ^3.3:
/// ```text
/// prompt-history-limit 100
/// ```
#[cfg(feature = "tmux_3_3")]
pub const PROMPT_HISTORY_LIMIT_DEFAULT: usize = 100;

/// tmux ^2.6:
/// ```text
/// set-clipboard external
/// ```
///
/// tmux ^2.1 v2.6:
/// ```text
/// set-clipboard on
/// ```
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
pub const SET_CLIPBOARD_DEFAULT: SetClipboard = SetClipboard::On;
#[cfg(feature = "tmux_2_6")]
pub const SET_CLIPBOARD_DEFAULT: SetClipboard = SetClipboard::External;

/// tmux ^3.2:
/// ```text
/// terminal-features[0] "xterm*:clipboard:ccolour:cstyle:focus:title"
/// terminal-features[1] "screen*:title"
/// ```
#[cfg(feature = "tmux_3_2")]
pub const TERMINAL_FEATURES_DEFAULT: [&str; 2] = [
    "xterm*:clipboard:ccolour:cstyle:focus:title",
    "screen*:title",
];

/// tmux ^3.2:
/// ```text
/// terminal-overrides[0] ""
/// ```
///
/// tmux ^2.0 v3.2:
/// ```text
/// terminal-overrides[0] "xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q"
/// terminal-overrides[1] screen*:XT
/// ```
#[cfg(all(feature = "tmux_2_0", not(feature = "tmux_3_2")))]
pub const TERMINAL_OVERRIDES_DEFAULT: [&str; 2] = [
    "xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q",
    "screen*:XT"
];
#[cfg(feature = "tmux_3_2")]
pub const TERMINAL_OVERRIDES_DEFAULT: [&str; 1] = [""];

/// tmux ^3.0:
/// ```text
/// user-keys[] ""
/// ```
// `""`
#[cfg(feature = "tmux_3_0")]
pub const USER_KEYS_DEFAULT: [&str; 1] = [""];

/// tmux ^1.2 v2.0:
/// ```text
/// quiet off
/// ```
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
pub const QUIET_DEFAULT: Switch = Switch::Off;

/// tmux ^1.3 v1.4:
/// ```text
/// detach-on-destroy on
/// ```
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
pub const DETACH_ON_DESTROY_DEFAULT: Switch = Switch::On;
