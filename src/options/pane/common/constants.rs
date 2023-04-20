use crate::{RemainOnExit, Switch};

/// tmux ^3.0:
/// ```text
/// allow-rename
/// ```
#[cfg(feature = "tmux_3_0")]
pub const ALLOW_RENAME: &str = "allow-rename";

/// tmux ^3.0:
/// ```text
/// alternate-screen
/// ```
#[cfg(feature = "tmux_3_0")]
pub const ALTERNATE_SCREEN: &str = "alternate-screen";

/// tmux ^3.0:
/// ```text
/// remain-on-exit
/// ```
#[cfg(feature = "tmux_3_0")]
pub const REMAIN_ON_EXIT: &str = "remain-on-exit";

/// tmux ^3.0:
/// ```text
/// window-active-style
/// ```
#[cfg(feature = "tmux_3_0")]
pub const WINDOW_ACTIVE_STYLE: &str = "window-active-style";

/// tmux ^3.0:
/// ```text
/// window-style
/// ```
#[cfg(feature = "tmux_3_0")]
pub const WINDOW_STYLE: &str = "window-style";

/// tmux ^3.2:
/// ```text
/// synchronize-panes
/// ```
#[cfg(feature = "tmux_3_2")]
pub const SYNCHRONIZE_PANES: &str = "synchronize-panes";

// defaults from tmux sources `options-table.c`, checked

/// tmux ^3.0:
/// ```text
/// allow-rename off
/// ```
#[cfg(feature = "tmux_3_0")]
pub const ALLOW_RENAME_DEFAULT: Switch = Switch::Off;

/// tmux ^3.0:
/// ```text
/// alternate-screen on
/// ```
#[cfg(feature = "tmux_3_0")]
pub const ALTERNATE_SCREEN_DEFAULT: Switch = Switch::On;

/// tmux ^3.0:
/// ```text
/// remain-on-exit off
/// ```
#[cfg(feature = "tmux_3_0")]
pub const REMAIN_ON_EXIT_DEFAULT: RemainOnExit = RemainOnExit::Off;

/// tmux ^3.0:
/// ```text
/// window-active-style default
/// ```
#[cfg(feature = "tmux_3_0")]
pub const WINDOW_ACTIVE_STYLE_DEFAULT: &str = "default";

/// tmux ^3.0:
/// ```text
/// window-style default
/// ```
#[cfg(feature = "tmux_3_0")]
pub const WINDOW_STYLE_DEFAULT: &str = "default";

/// tmux ^3.2:
/// ```text
/// synchronize-panes off
/// ```
#[cfg(feature = "tmux_3_2")]
pub const SYNCHRONIZE_PANES_DEFAULT: Switch = Switch::Off;
