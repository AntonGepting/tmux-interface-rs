/// `%begin`
#[cfg(feature = "tmux_1_8")]
pub const OUTPUT_BLOCK_BEGIN: &str = "%begin";
/// `%end`
#[cfg(feature = "tmux_1_8")]
pub const OUTPUT_BLOCK_END: &str = "%end";
/// `%error`
#[cfg(feature = "tmux_1_8")]
pub const OUTPUT_BLOCK_ERROR: &str = "%error";

/// In control mode, tmux outputs notifications.  A notification will
/// never occur inside an output block. (tmux man)
///
/// `%client-detached client`
#[cfg(feature = "tmux_3_2")]
pub const NOTIFICATION_CLIENT_DETACHED: &str = "%client-detached";
/// `%client-session-changed client session-id name`
#[cfg(feature = "tmux_2_4")]
pub const NOTIFICATION_CLIENT_SESSION_CHANGED: &str = "%client-session-changed";
/// `%continue pane-id`
#[cfg(feature = "tmux_X_X")]
pub const NOTIFICATION_CONTINUE: &str = "%continue";
/// `%exit [reason]`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_EXIT: &str = "%exit";
/// `%extended-output pane-id age ... : value`
#[cfg(feature = "tmux_X_X")]
pub const NOTIFICATION_EXTENDED_OUTPUT: &str = "%extended-output";
/// tmux ^2.2 `%layout-change window-id window-layout window-visible-layout`
/// tmux ^1.8 `%layout-change window-id window-layout`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_LAYOUT_CHANGE: &str = "%layout-change";
/// `%output pane-id value`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_OUTPUT: &str = "%output";
/// `%pane-mode-changed pane-id`
#[cfg(feature = "tmux_2_5")]
pub const NOTIFICATION_PANE_MODE_CHANGED: &str = "%pane-mode-changed";
/// `%pause pane-id`
#[cfg(feature = "tmux_X_X")]
pub const NOTIFICATION_PAUSE: &str = "%pause";
/// `%session-changed session-id name`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_SESSION_CHANGED: &str = "%session-changed";
/// `%session-renamed name`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_SESSION_RENAMED: &str = "%session-renamed";
/// `%session-window-changed session-id window-id`
#[cfg(feature = "tmux_2_5")]
pub const NOTIFICATION_SESSION_WINDOW_CHANGED: &str = "%session-window-changed";
/// `%sessions-changed`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_SESSIONS_CHANGED: &str = "%sessions-changed";
/// `%subscription-changed name session-id window-id window-index`
#[cfg(feature = "tmux_X_X")]
pub const NOTIFICATION_SUBSCRIPTION_CHANGED: &str = "%subscription-changed";
/// `%unlinked-window-add window-id`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_UNLINKED_WINDOW_ADD: &str = "%unlinked-window-add";
/// `%window-add window-id`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_WINDOW_ADD: &str = "%window-add";
/// `%window-close window-id`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_WINDOW_CLOSE: &str = "%window-close";
/// `%window-pane-changed window-id pane-id`
#[cfg(feature = "tmux_2_5")]
pub const NOTIFICATION_WINDOW_PANE_CHANGED: &str = "%window-pane-changed";
/// `%window-renamed window-id name`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_WINDOW_RENAMED: &str = "%window-renamed";

/// separator in notifications, and output block (`%begin<' '>1234<' '>0`)
pub const CONTROL_MODE_SEPARATOR: char = ' ';
/// additional separator used in extended-output notification
pub const CONTROL_MODE_EXTENDED_OUTPUT_SEPARATOR: &str = " : ";
