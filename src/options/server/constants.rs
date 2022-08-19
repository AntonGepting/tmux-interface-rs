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
