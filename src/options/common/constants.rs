pub const VI: &str = "vi";
pub const EMACS: &str = "emacs";

pub const ON: &str = "on";
pub const OFF: &str = "off";

pub const FAILED: &str = "failed";

pub const ALWAYS: &str = "always";

pub const KEEP_LAST: &str = "keep-last";
pub const KEEP_GROUP: &str = "keep-group";

#[cfg(feature = "tmux_2_6")]
pub const EXTERNAL: &str = "external";

pub const ANY: &str = "any";
pub const NONE: &str = "none";
pub const CURRENT: &str = "current";
pub const OTHER: &str = "other";

#[cfg(feature = "tmux_2_6")]
pub const BOTH: &str = "both";

#[cfg(feature = "tmux_3_2")]
pub const NO_DETACHED: &str = "no-detached";

#[cfg(feature = "tmux_3_4")]
pub const NUMBER_0: &str = "0";
#[cfg(feature = "tmux_3_4")]
pub const NUMBER_1: &str = "1";
#[cfg(feature = "tmux_2_9")]
pub const NUMBER_2: &str = "2";
#[cfg(feature = "tmux_2_9")]
pub const NUMBER_3: &str = "3";
#[cfg(feature = "tmux_2_9")]
pub const NUMBER_4: &str = "4";
#[cfg(feature = "tmux_2_9")]
pub const NUMBER_5: &str = "5";

#[cfg(feature = "tmux_1_0")]
pub const LEFT: &str = "left";
#[cfg(feature = "tmux_1_0")]
pub const CENTRE: &str = "centre";
#[cfg(feature = "tmux_1_0")]
pub const RIGHT: &str = "right";

#[cfg(feature = "tmux_1_7")]
pub const TOP: &str = "top";
#[cfg(feature = "tmux_1_7")]
pub const BOTTOM: &str = "bottom";

#[cfg(feature = "tmux_1_0")]
pub const H12: &str = "12";
#[cfg(feature = "tmux_1_0")]
pub const H24: &str = "24";

#[cfg(feature = "tmux_2_9")]
pub const LARGEST: &str = "largest";
#[cfg(feature = "tmux_2_9")]
pub const SMALLEST: &str = "smallest";
#[cfg(feature = "tmux_2_9")]
pub const MANUAL: &str = "manual";
#[cfg(feature = "tmux_3_1")]
pub const LATEST: &str = "latest";
