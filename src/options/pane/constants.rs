// XXX: conditionals?
// NOTE: total num: 5 (usize)
pub const ALLOW_RENAME: usize = 1 << 0;
pub const ALTERNATE_SCREEN: usize = 1 << 1;
pub const REMAIN_ON_EXIT: usize = 1 << 2;
pub const WINDOW_ACTIVE_STYLE: usize = 1 << 3;
pub const WINDOW_STYLE: usize = 1 << 4;
#[cfg(feature = "tmux_3_2")]
pub const SYNCHRONIZE_PANES: usize = 1 << 5;

pub const PANE_OPTIONS_NONE: usize = 0;
////pub const PANE_OPTIONS_DEFAULT: usize = ;
pub const PANE_OPTIONS_ALL: usize =
    ALLOW_RENAME | ALTERNATE_SCREEN | REMAIN_ON_EXIT | WINDOW_ACTIVE_STYLE | WINDOW_STYLE;

#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_2")))]
pub const PANE_OPTIONS_NUM: usize = 5;
#[cfg(feature = "tmux_3_2")]
pub const PANE_OPTIONS_NUM: usize = 6;


