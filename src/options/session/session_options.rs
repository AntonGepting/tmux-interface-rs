use super::*;
use crate::options::common::{array_insert, cow_parse, get_parts};
use crate::options::StatusKeys;
use crate::Switch;
use crate::{Action, Activity, DetachOnDestroy, Error, Status, StatusJustify, StatusPosition};
use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;

// TODO: Vec variables solution
// TODO: check types
// 45 Available session options are:
#[derive(PartialEq, Clone, Debug)]
pub struct SessionOptions<'a> {
    //activity-action [any | none | current | other]
    #[cfg(feature = "tmux_2_6")]
    pub activity_action: Option<Action>,
    //assume-paste-time milliseconds
    #[cfg(feature = "tmux_1_8")]
    pub assume_paste_time: Option<usize>,
    //base-index index
    #[cfg(feature = "tmux_1_0")]
    pub base_index: Option<usize>,
    //bell-action [any | none | current | other]
    // tmux 1.0: bell-action [any | none | other]
    #[cfg(feature = "tmux_1_0")]
    pub bell_action: Option<Action>,
    //bell-on-alert [on | off]
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    pub bell_on_alert: Option<Switch>,
    //buffer-limit limit
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    pub buffer_limit: Option<usize>,
    //default-command shell-command
    #[cfg(feature = "tmux_1_0")]
    pub default_command: Option<Cow<'a, str>>,
    //default-shell path
    #[cfg(feature = "tmux_1_0")]
    pub default_shell: Option<Cow<'a, str>>,
    //default-path path
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub default_path: Option<Cow<'a, str>>,
    // default-terminal terminal
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub default_terminal: Option<Cow<'a, str>>,
    //default-size XxY
    #[cfg(feature = "tmux_2_9")]
    pub default_size: Option<(usize, usize)>,
    //destroy-unattached [on | off]
    #[cfg(feature = "tmux_1_4")]
    pub destroy_unattached: Option<Switch>,
    //detach-on-destroy [on | off]
    // tmux ^3.2 detach-on-destroy [on | off | no-detached]
    #[cfg(feature = "tmux_1_4")]
    pub detach_on_destroy: Option<DetachOnDestroy>,
    //display-panes-active-colour colour
    #[cfg(feature = "tmux_1_2")]
    pub display_panes_active_colour: Option<Cow<'a, str>>,
    //display-panes-colour colour
    #[cfg(feature = "tmux_1_0")]
    pub display_panes_colour: Option<Cow<'a, str>>,
    //display-panes-time time
    #[cfg(feature = "tmux_1_0")]
    pub display_panes_time: Option<usize>,
    //display-time time
    #[cfg(feature = "tmux_1_0")]
    pub display_time: Option<usize>,
    //history-limit lines
    #[cfg(feature = "tmux_1_0")]
    pub history_limit: Option<usize>,
    //key-table key-table
    #[cfg(feature = "tmux_2_2")]
    pub key_table: Option<Cow<'a, str>>,
    //lock-after-time number
    #[cfg(feature = "tmux_1_0")]
    pub lock_after_time: Option<usize>,
    //lock-command shell-command
    #[cfg(feature = "tmux_1_1")]
    pub lock_command: Option<Cow<'a, str>>,
    //lock-server [on | off]
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    pub lock_server: Option<Cow<'a, str>>,
    //message-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub message_attr: Option<Cow<'a, str>>,
    //message-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub message_bg: Option<Cow<'a, str>>,
    //message-command-attr attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub message_command_attr: Option<Cow<'a, str>>,
    //message-command-bg colour
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub message_command_bg: Option<Cow<'a, str>>,
    //message-command-fg colour
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub message_command_fg: Option<Cow<'a, str>>,
    //message-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub message_fg: Option<Cow<'a, str>>,
    //message-command-style style
    #[cfg(feature = "tmux_1_9")]
    pub message_command_style: Option<Cow<'a, str>>,
    //message-limit number
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub message_limit: Option<usize>,
    //message-style style
    #[cfg(feature = "tmux_1_9")]
    pub message_style: Option<Cow<'a, str>>,
    //mouse-resize-pane [on | off]
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub mouse_resize_pane: Option<Switch>,
    //mouse-select-pane [on | off]
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub mouse_select_pane: Option<Switch>,
    //mouse-select-window [on | off]
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub mouse_select_window: Option<Switch>,
    //mouse [on | off]
    #[cfg(feature = "tmux_2_1")]
    pub mouse: Option<Switch>,
    // mouse-utf8 [on | off]
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    pub mouse_utf8: Option<Switch>,
    //pane-active-border-bg colour
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub pane_active_border_bg: Option<Cow<'a, str>>,
    //pane-active-border-fg colour
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub pane_active_border_fg: Option<Cow<'a, str>>,
    //pane-border-bg colour
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub pane_border_bg: Option<Cow<'a, str>>,
    //pane-border-fg colour
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub pane_border_fg: Option<Cow<'a, str>>,
    //pane-active-border-style style
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub pane_active_border_style: Option<Cow<'a, str>>,
    //pane-border-style style
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub pane_border_style: Option<Cow<'a, str>>,
    //prefix key
    #[cfg(feature = "tmux_1_0")]
    pub prefix: Option<Cow<'a, str>>,
    //prefix2 key
    #[cfg(feature = "tmux_1_6")]
    pub prefix2: Option<Cow<'a, str>>,
    //renumber-windows [on | off]
    #[cfg(feature = "tmux_1_7")]
    pub renumber_windows: Option<Switch>,
    //repeat-time time
    #[cfg(feature = "tmux_1_0")]
    pub repeat_time: Option<usize>,
    //set-remain-on-exit [on | off]
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    pub set_remain_on_exit: Option<Switch>,
    //set-titles [on | off]
    #[cfg(feature = "tmux_1_0")]
    pub set_titles: Option<Switch>,
    //set-titles-string string
    #[cfg(feature = "tmux_1_0")]
    pub set_titles_string: Option<Cow<'a, str>>,
    //silence-action [any | none | current | other]
    #[cfg(feature = "tmux_2_6")]
    pub silence_action: Option<Action>,
    //status [off | on | 2 | 3 | 4 | 5]
    //tmux 1.0: status [off | on]
    #[cfg(feature = "tmux_1_0")]
    pub status: Option<Status>,
    //status-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_attr: Option<Cow<'a, str>>,
    //status-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_bg: Option<Cow<'a, str>>,
    //status-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_fg: Option<Cow<'a, str>>,
    //status-format[] format
    #[cfg(feature = "tmux_2_9")]
    pub status_format: Option<Vec<Cow<'a, str>>>,
    //status-interval interval
    #[cfg(feature = "tmux_1_0")]
    pub status_interval: Option<usize>,
    //status-justify [left | centre | right]
    #[cfg(feature = "tmux_1_0")]
    pub status_justify: Option<StatusJustify>,
    //status-keys [vi | emacs]
    #[cfg(feature = "tmux_1_0")]
    pub status_keys: Option<StatusKeys>,
    //status-left string
    #[cfg(feature = "tmux_1_0")]
    pub status_left: Option<Cow<'a, str>>,
    //status-left-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_left_attr: Option<Cow<'a, str>>,
    //status-left-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_left_bg: Option<Cow<'a, str>>,
    //status-left-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_left_fg: Option<Cow<'a, str>>,
    //status-left-length length
    #[cfg(feature = "tmux_1_0")]
    pub status_left_length: Option<usize>,
    //status-left-style style
    #[cfg(feature = "tmux_1_9")]
    pub status_left_style: Option<Cow<'a, str>>,
    //status-position [top | bottom]
    #[cfg(feature = "tmux_1_7")]
    pub status_position: Option<StatusPosition>,
    //status-right string
    #[cfg(feature = "tmux_1_0")]
    pub status_right: Option<Cow<'a, str>>,
    //status-right-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_right_attr: Option<Cow<'a, str>>,
    //status-right-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_right_bg: Option<Cow<'a, str>>,
    //status-right-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_right_fg: Option<Cow<'a, str>>,
    //status-right-length length
    #[cfg(feature = "tmux_1_0")]
    pub status_right_length: Option<usize>,
    //status-right-style style
    #[cfg(feature = "tmux_1_9")]
    pub status_right_style: Option<Cow<'a, str>>,
    //status-style style
    #[cfg(feature = "tmux_1_9")]
    pub status_style: Option<Cow<'a, str>>,
    //status-utf8 [on | off]
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub status_utf8: Option<Switch>,
    //terminal-overrides string
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub terminal_overrides: Option<Cow<'a, str>>,
    //update-environment[] variable
    #[cfg(feature = "tmux_1_0")]
    pub update_environment: Option<Vec<Cow<'a, str>>>,
    //user-keys
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    pub user_keys: Option<Vec<Cow<'a, str>>>,
    //visual-activity [on | off | both]
    //tmux 1.0: visual-activity [on | off]
    #[cfg(feature = "tmux_1_0")]
    pub visual_activity: Option<Activity>,
    //visual-bell [on | off | both]
    //tmux 1.0: visual-bell [on | off]
    #[cfg(feature = "tmux_1_0")]
    pub visual_bell: Option<Activity>,
    //visual-content [on | off]
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub visual_content: Option<Switch>,
    //visual-silence [on | off | both]
    #[cfg(feature = "tmux_1_4")]
    pub visual_silence: Option<Activity>,
    //word-separators string
    #[cfg(feature = "tmux_1_6")]
    pub word_separators: Option<Cow<'a, str>>,

    pub user_options: HashMap<String, Option<Cow<'a, str>>>,
}

impl<'a> SessionOptions<'a> {
    //pub fn get() -> TmuxCommand<'a> {
    //ShowOptions::new().build()
    ////GetServerOptions::new();
    //}

    //pub fn get_ext(invoke: &dyn Fn(&TmuxCommand<'a>) -> String) -> Result<Self, Error> {
    //let cmd = ShowOptions::new().server().build();
    //let output = invoke(&cmd);
    //SessionOptions::from_str(&output)
    //}

    //pub fn set(self) -> TmuxCommands<'a> {
    //let cmds = SetLocalSessionOptions::new();

    //#[cfg(feature = "tmux_2_6")]
    //let cmds = cmds.activity_action(self.activity_action);
    //#[cfg(feature = "tmux_1_8")]
    //let cmds = cmds.assume_paste_time(self.assume_paste_time);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.base_index(self.base_index);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.bell_action(self.bell_action);
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    //let cmds = cmds.bell_on_alert(self.bell_on_alert);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    //let cmds = cmds.buffer_limit(self.buffer_limit);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.default_command(self.default_command);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.default_shell(self.default_shell);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let cmds = cmds.default_path(self.default_path);
    //#[cfg(feature = "tmux_2_9")]
    //let cmds = cmds.default_size(self.default_size);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    //let cmds = cmds.default_terminal(self.default_terminal);
    //#[cfg(feature = "tmux_1_4")]
    //let cmds = cmds.destroy_unattached(self.destroy_unattached);
    //#[cfg(feature = "tmux_1_4")]
    //let cmds = cmds.detach_on_destroy(self.detach_on_destroy);
    //#[cfg(feature = "tmux_1_2")]
    //let cmds = cmds.display_panes_active_colour(self.display_panes_active_colour);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.display_panes_colour(self.display_panes_colour);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.display_panes_time(self.display_panes_time);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.display_time(self.display_time);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.history_limit(self.history_limit);
    //#[cfg(feature = "tmux_2_2")]
    //let cmds = cmds.key_table(self.key_table);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.lock_after_time(self.lock_after_time);
    //#[cfg(feature = "tmux_1_1")]
    //let cmds = cmds.lock_command(self.lock_command);
    //#[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    //let cmds = cmds.lock_server(self.lock_server);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let cmds = cmds.message_attr(self.message_attr);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let cmds = cmds.message_bg(self.message_bg);
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let cmds = cmds.message_command_attr(self.message_command_attr);
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let cmds = cmds.message_command_bg(self.message_command_bg);
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let cmds = cmds.message_command_fg(self.message_command_fg);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let cmds = cmds.message_fg(self.message_fg);
    //#[cfg(feature = "tmux_1_9")]
    //let cmds = cmds.message_command_style(self.message_command_style);
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    //let cmds = cmds.message_limit(self.message_limit);
    //#[cfg(feature = "tmux_1_9")]
    //let cmds = cmds.message_style(self.message_style);
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let cmds = cmds.mouse_resize_pane(self.mouse_resize_pane);
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let cmds = cmds.mouse_select_pane(self.mouse_select_pane);
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let cmds = cmds.mouse_select_window(self.mouse_select_window);
    //#[cfg(feature = "tmux_2_1")]
    //let cmds = cmds.mouse(self.mouse);
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    //let cmds = cmds.mouse_utf8(self.mouse_utf8);
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let cmds = cmds.pane_active_border_bg(self.pane_active_border_bg);
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let cmds = cmds.pane_active_border_fg(self.pane_active_border_fg);
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let cmds = cmds.pane_border_bg(self.pane_border_bg);
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let cmds = cmds.pane_border_fg(self.pane_border_fg);
    //#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    //let cmds = cmds.pane_active_border_style(self.pane_active_border_style);
    //#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    //let cmds = cmds.pane_border_style(self.pane_border_style);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.prefix(self.prefix);
    //#[cfg(feature = "tmux_1_6")]
    //let cmds = cmds.prefix2(self.prefix2);
    //#[cfg(feature = "tmux_1_7")]
    //let cmds = cmds.renumber_windows(self.renumber_windows);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.repeat_time(self.repeat_time);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    //let cmds = cmds.set_remain_on_exit(self.set_remain_on_exit);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.set_titles(self.set_titles);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.set_titles_string(self.set_titles_string);
    //#[cfg(feature = "tmux_2_6")]
    //let cmds = cmds.silence_action(self.silence_action);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.status(self.status);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let cmds = cmds.status_attr(self.status_attr);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let cmds = cmds.status_bg(self.status_bg);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let cmds = cmds.status_fg(self.status_fg);
    //#[cfg(feature = "tmux_2_9")]
    //let cmds = cmds.status_format(self.status_format);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.status_interval(self.status_interval);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.status_justify(self.status_justify);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.status_keys(self.status_keys);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.status_left(self.status_left);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let cmds = cmds.status_left_attr(self.status_left_attr);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let cmds = cmds.status_left_bg(self.status_left_bg);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let cmds = cmds.status_left_fg(self.status_left_fg);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.status_left_length(self.status_left_length);
    //#[cfg(feature = "tmux_1_9")]
    //let cmds = cmds.status_left_style(self.status_left_style);
    //#[cfg(feature = "tmux_1_7")]
    //let cmds = cmds.status_position(self.status_position);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.status_right(self.status_right);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let cmds = cmds.status_right_attr(self.status_right_attr);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let cmds = cmds.status_right_bg(self.status_right_bg);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let cmds = cmds.status_right_fg(self.status_right_fg);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.status_right_length(self.status_right_length);
    //#[cfg(feature = "tmux_1_9")]
    //let cmds = cmds.status_right_style(self.status_right_style);
    //#[cfg(feature = "tmux_1_9")]
    //let cmds = cmds.status_style(self.status_style);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    //let cmds = cmds.status_utf8(self.status_utf8);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    //let cmds = cmds.terminal_overrides(self.terminal_overrides);
    //// FIXME
    ////#[cfg(feature = "tmux_1_0")]
    ////let cmds = cmds.update_environment(self.update_environment);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.visual_activity(self.visual_activity);
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.visual_bell(self.visual_bell);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    //let cmds = cmds.visual_content(self.visual_content);
    //#[cfg(feature = "tmux_1_4")]
    //let cmds = cmds.visual_silence(self.visual_silence);
    //#[cfg(feature = "tmux_1_6")]
    //let cmds = cmds.word_separators(self.word_separators);
    // `@USER_OPTION`

    //cmds.build()

    // NOTE: Tmux struct wrong decision here, because of dircet and control mode
    //Self::get_all_ext(None)
    //}

    // XXX: bitmask is overkill now, mb later use for multiple select
    // NOTE: not allows selective get by bitmask
    //#[cfg(feature = "tmux_1_7")]
    //pub fn get(bitflags: u128) -> Result<Self, Error> {
    //Self::get_ext(None, bitflags)
    //}

    // extended
    //
    // XXX: bitmask is overkill now, mb later use for multiple select
    // NOTE: not allows selective get by bitmask
    //#[cfg(feature = "tmux_1_7")]
    //pub fn get_ext(cb: Option<&dyn Fn() -> String>, bitflags: u128) -> Result<Self, Error> {
    //let selected_option = SESSION_OPTIONS
    //.iter()
    //.filter(|t| bitflags == t.3)
    //.map(|t| t.0.to_string())
    //.collect::<Vec<String>>()
    //.join(" ");

    //let s = match cb {
    //Some(cb) => cb(),
    //None => Tmux::new()
    //.command(ShowOptions::new().option(selected_option))
    //.output()?
    //.to_string(),
    //};

    //s.parse()
    //}

    // do not create anything, just get option from tmux
    //pub fn get_from_new_server(bitflags: u128) -> Result<Self, Error> {
    //let mut cmds = TmuxBinCommands::default();

    //let start_server = StartServer::new();
    //cmds.push(start_server);

    //Self::get_ext(Some(&mut cmds), bitflags)
    //}

    //#[cfg(feature = "tmux_1_7")]
    //pub fn get_global(bitflags: u128) -> Result<Self, Error> {
    //let selected_option = SESSION_OPTIONS
    //.iter()
    //.filter(|t| bitflags == t.3)
    //.map(|t| t.0.to_string())
    //.collect::<Vec<String>>()
    //.join(" ");
    //let s = ShowOptions::new()
    //.global()
    //.option(&selected_option)
    //.build()
    ////.into_tmux_bin_command()
    ////.output()?
    //.to_string();
    //s.parse()
    //}

    // allows selective set by bitmask
    //pub fn set(&self, bitflags: u128) -> Result<(), Error> {
    //for selected_option in SESSION_OPTIONS.iter().filter(|t| bitflags & t.3 == t.3) {
    //if let Some(selected_value) = selected_option.2(&self) {
    //SetOption::new()
    //.option(selected_option.0)
    //.option(&selected_value);
    //}
    //}
    //Ok(())
    //}

    //pub fn set_global(&self, bitflags: u128) -> Result<(), Error> {
    //for selected_option in SESSION_OPTIONS.iter().filter(|t| bitflags & t.3 == t.3) {
    //if let Some(selected_value) = selected_option.2(&self) {
    //SetOption::new()
    //.global()
    //.option(selected_option.0)
    //.value(&selected_value);
    //}
    //}
    //Ok(())
    //}

    // XXX: single set get methods
}

// command_alias[0] = "alias1" => command_alias["alias1"]
// command_alias[1] = "alias2" => command_alias["alias2"]
// ...
// command_alias[n] = "aliasN" => command_alias["aliasN"]
// TODO: optimization, merge server, session, window, pane?
//impl FromStr for SessionOptions {
//type Err = Error;

//fn from_str(_options: &str) -> Result<Self, Self::Err> {
//let session_options: SessionOptions = Default::default();
////let mut v: Vec<&str>;
////let mut arr: Vec<&str>;
////for option in options.lines() {
////v = option.trim().splitn(2, ' ').collect();
////arr = v[0].split(|c| c == '[' || c == ']').collect();
////for session_var in SESSION_OPTIONS.iter() {
////if session_var.0 == arr[0] {
////session_var.1(
////&mut session_options,
////arr.get(1).and_then(|i| i.parse::<usize>().ok()),
////v.get(1).unwrap_or(&""),
////)
////}
////}
////}
//Ok(session_options)
//}
//}

//impl fmt::Display for SessionOptions {
//fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
//// pane option
////for var in SESSION_OPTIONS.iter() {
////// if is set some - extract
////if let Some(ref v) = var.2(self) {
////writeln!(f, "{} {}", var.0, v)?;
////}
////}
//Ok(())
//}
//}

/// Default
///
/// ```text
/// tmux show-options -g
/// ```
///
/// ```text
/// activity-action other
/// assume-paste-time 1
/// base-index 1
/// bell-action none
/// default-command
/// default-shell /bin/bash
/// default-size 80x24
/// destroy-unattached off
/// detach-on-destroy on
/// display-panes-active-colour red
/// display-panes-colour blue
/// display-panes-time 1000
/// display-time 750
/// history-limit 2000
/// key-table root
/// lock-after-time 0
/// lock-command "lock -np"
/// message-command-style fg=blue,bg=black
/// message-style fg=colour232,bg=colour166,bright
/// mouse on
/// prefix C-a
/// prefix2 Invalid#1fff00000000
/// renumber-windows off
/// repeat-time 500
/// set-titles on
/// set-titles-string "#W"
/// silence-action other
/// status on
/// status-bg #282c34
/// status-fg colour247
/// status-format[0] "#[align=left range=left #{status-left-style}]#[push-default]#{T;=/#{status-left-length}:status-left}#[pop-default]#[norange default]#[list=on align=#{status-justify}]#[list=left-marker]<#[list=right-marker]>#[list=on]#{W:#[range=window|#{window_index} #{window-status-style}#{?#{&&:#{window_last_flag},#{!=:#{window-status-last-style},default}}, #{window-status-last-style},}#{?#{&&:#{window_bell_flag},#{!=:#{window-status-bell-style},default}}, #{window-status-bell-style},#{?#{&&:#{||:#{window_activity_flag},#{window_silence_flag}},#{!=:#{window-status-activity-style},default}}, #{window-status-activity-style},}}]#[push-default]#{T:window-status-format}#[pop-default]#[norange default]#{?window_end_flag,,#{window-status-separator}},#[range=window|#{window_index} list=focus #{?#{!=:#{window-status-current-style},default},#{window-status-current-style},#{window-status-style}}#{?#{&&:#{window_last_flag},#{!=:#{window-status-last-style},default}}, #{window-status-last-style},}#{?#{&&:#{window_bell_flag},#{!=:#{window-status-bell-style},default}}, #{window-status-bell-style},#{?#{&&:#{||:#{window_activity_flag},#{window_silence_flag}},#{!=:#{window-status-activity-style},default}}, #{window-status-activity-style},}}]#[push-default]#{T:window-status-current-format}#[pop-default]#[norange list=on default]#{?window_end_flag,,#{window-status-separator}}}#[nolist align=right range=right #{status-right-style}]#[push-default]#{T;=/#{status-right-length}:status-right}#[pop-default]#[norange default]"
/// status-format[1] "#[align=centre]#{P:#{?pane_active,#[reverse],}#{pane_index}[#{pane_width}x#{pane_height}]#[default] }"
/// status-interval 2
/// status-justify left
/// status-keys emacs
/// status-left " #(whoami)@#H:#[fg=#dcdfe4, bg=#282c34] #S   "
/// status-left-length 50
/// status-left-style default
/// status-position bottom
/// status-right "#[fg=colour247, bg=#282c34] %a, %e. %b %H:%M "
/// status-right-length 50
/// status-right-style default
/// status-style fg=colour247,bg=#282c34
/// update-environment[0] DISPLAY
/// update-environment[1] KRB5CCNAME
/// update-environment[2] SSH_ASKPASS
/// update-environment[3] SSH_AUTH_SOCK
/// update-environment[4] SSH_AGENT_PID
/// update-environment[5] SSH_CONNECTION
/// update-environment[6] WINDOWID
/// update-environment[7] XAUTHORITY
/// visual-activity off
/// visual-bell off
/// visual-silence off
/// word-separators " "
/// ```
impl<'a> Default for SessionOptions<'a> {
    fn default() -> Self {
        let options = SessionOptions::new();
        #[cfg(feature = "tmux_2_6")]
        let options = options.activity_action(Some(Action::Other));
        #[cfg(feature = "tmux_1_8")]
        let options = options.assume_paste_time(Some(1));
        #[cfg(feature = "tmux_1_0")]
        let options = options.base_index(Some(0));
        #[cfg(feature = "tmux_1_0")]
        let options = options.bell_action(Some(Action::Any));
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
        let options = options.bell_on_alert(None);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
        let options = options.buffer_limit(None);
        #[cfg(feature = "tmux_1_0")]
        let options = options.default_command(Some(""));
        #[cfg(feature = "tmux_1_0")]
        let options = options.default_shell(Some("/bin/sh")); // _PATH_BSHELL
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.default_path(None);
        #[cfg(feature = "tmux_2_9")]
        let options = options.default_size(Some((80, 24)));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
        let options = options.default_terminal(None);
        #[cfg(feature = "tmux_1_4")]
        let options = options.destroy_unattached(Some(Switch::Off));
        #[cfg(feature = "tmux_1_4")]
        let options = options.detach_on_destroy(Some(DetachOnDestroy::Off));
        #[cfg(feature = "tmux_1_2")]
        let options = options.display_panes_active_colour(Some("1"));
        #[cfg(feature = "tmux_1_0")]
        let options = options.display_panes_colour(Some("4"));
        #[cfg(feature = "tmux_1_0")]
        let options = options.display_panes_time(Some(1000));
        #[cfg(feature = "tmux_1_0")]
        let options = options.display_time(Some(750));
        #[cfg(feature = "tmux_1_0")]
        let options = options.history_limit(Some(2000));
        #[cfg(feature = "tmux_2_2")]
        let options = options.key_table(Some("root"));
        #[cfg(feature = "tmux_1_0")]
        let options = options.lock_after_time(Some(0));
        #[cfg(feature = "tmux_1_1")]
        let options = options.lock_command(Some("lock -np"));
        #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
        let options = options.lock_server(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.message_attr(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.message_bg(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.message_command_attr(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.message_command_bg(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.message_command_fg(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.message_fg(Some());
        #[cfg(feature = "tmux_1_9")]
        let options = options.message_command_style(Some("bg=black,fg=yellow"));
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
        let options = options.message_limit(Some());
        #[cfg(feature = "tmux_1_9")]
        let options = options.message_style(Some("bg=yellow,fg=black"));
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        let options = options.mouse_resize_pane(Some());
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        let options = options.mouse_select_pane(Some());
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        let options = options.mouse_select_window(Some());
        #[cfg(feature = "tmux_2_1")]
        let options = options.mouse(Some(Switch::Off));
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
        let options = options.mouse_utf8(Some());
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let options = options.pane_active_border_bg(Some());
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let options = options.pane_active_border_fg(Some());
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let options = options.pane_border_bg(Some());
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let options = options.pane_border_fg(Some());
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        let options = options.pane_active_border_style(Some());
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        let options = options.pane_border_style(Some());
        #[cfg(feature = "tmux_1_0")]
        let options = options.prefix(Some(""));
        #[cfg(feature = "tmux_1_6")]
        let options = options.prefix2(Some("")); // KEYC_NONE
        #[cfg(feature = "tmux_1_7")]
        let options = options.renumber_windows(Some(Switch::Off));
        #[cfg(feature = "tmux_1_0")]
        let options = options.repeat_time(Some(500));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
        let options = options.set_remain_on_exit(Some());
        #[cfg(feature = "tmux_1_0")]
        let options = options.set_titles(Some(Switch::Off));
        #[cfg(feature = "tmux_1_0")]
        let options = options.set_titles_string(Some("#S:#I:#W - \"#T\" #{session_alerts}"));
        #[cfg(feature = "tmux_2_6")]
        let options = options.silence_action(Some(Action::Other));
        #[cfg(feature = "tmux_1_0")]
        let options = options.status(Some(Status::Off));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_attr(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_bg(Some(8));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_fg(Some());
        #[cfg(feature = "tmux_2_9")]
        let options = options.status_format(Some()); // options_table_status_format_default
        #[cfg(feature = "tmux_1_0")]
        let options = options.status_interval(Some(15));
        #[cfg(feature = "tmux_1_0")]
        let options = options.status_justify(Some(StatusJustify::Left)); // 0
        #[cfg(feature = "tmux_1_0")]
        let options = options.status_keys(Some(StatusKeys::Emacs));
        #[cfg(feature = "tmux_1_0")]
        let options = options.status_left(Some("[#{session_name}] "));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_left_attr(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_left_bg(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_left_fg(Some());
        #[cfg(feature = "tmux_1_0")]
        let options = options.status_left_length(Some(10));
        #[cfg(feature = "tmux_1_9")]
        let options = options.status_left_style(Some("default"));
        #[cfg(feature = "tmux_1_7")]
        let options = options.status_position(Some(StatusPosition::Bottom));
        #[cfg(feature = "tmux_1_0")]
        let options = options.status_right(Some("#{?window_bigger, [#{window_offset_x}#,#{window_offset_y}] ,} \"#{=21:pane_title}\" %H:%M %d-%b-%y"));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_right_attr(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_right_bg(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_right_fg(Some());
        #[cfg(feature = "tmux_1_0")]
        let options = options.status_right_length(Some(40));
        #[cfg(feature = "tmux_1_9")]
        let options = options.status_right_style(Some("default"));
        #[cfg(feature = "tmux_1_9")]
        let options = options.status_style(Some("bg=green,fg=black"));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
        let options = options.status_utf8(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        let options = options.terminal_overrides(Some());
        #[cfg(feature = "tmux_1_0")]
        let options = options.update_environment(Some(vec![
            "DISPLAY",
            "KRB5CCNAME",
            "SSH_ASKPASS",
            "SSH_AUTH_SOCK",
            "SSH_AGENT_PID",
            "SSH_CONNECTION",
            "WINDOWID",
            "XAUTHORITY",
        ]));
        #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
        let options = options.user_keys(Some(vec![""]));
        #[cfg(feature = "tmux_1_0")]
        let options = options.visual_activity(Some(Activity::Off));
        #[cfg(feature = "tmux_1_0")]
        let options = options.visual_bell(Some(Activity::Off));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        let options = options.visual_content(Some());
        #[cfg(feature = "tmux_1_4")]
        let options = options.visual_silence(Some(Activity::Off));
        #[cfg(feature = "tmux_1_6")]
        let options = options.word_separators(Some("!\"#$%&'()*+,-./:;<=>?@[\\]^`{|}~"));

        options
    }
}

impl<'a> SessionOptions<'a> {
    pub fn new() -> Self {
        SessionOptions {
            #[cfg(feature = "tmux_2_6")]
            activity_action: None,
            #[cfg(feature = "tmux_1_8")]
            assume_paste_time: None,
            #[cfg(feature = "tmux_1_0")]
            base_index: None,
            #[cfg(feature = "tmux_1_0")]
            bell_action: None,
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
            bell_on_alert: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
            buffer_limit: None,
            #[cfg(feature = "tmux_1_0")]
            default_command: None,
            #[cfg(feature = "tmux_1_0")]
            default_shell: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            default_path: None,
            #[cfg(feature = "tmux_2_9")]
            default_size: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
            default_terminal: None,
            #[cfg(feature = "tmux_1_4")]
            destroy_unattached: None,
            #[cfg(feature = "tmux_1_4")]
            detach_on_destroy: None,
            #[cfg(feature = "tmux_1_2")]
            display_panes_active_colour: None,
            #[cfg(feature = "tmux_1_0")]
            display_panes_colour: None,
            #[cfg(feature = "tmux_1_0")]
            display_panes_time: None,
            #[cfg(feature = "tmux_1_0")]
            display_time: None,
            #[cfg(feature = "tmux_1_0")]
            history_limit: None,
            #[cfg(feature = "tmux_2_2")]
            key_table: None,
            #[cfg(feature = "tmux_1_0")]
            lock_after_time: None,
            #[cfg(feature = "tmux_1_1")]
            lock_command: None,
            #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
            lock_server: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            message_attr: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            message_bg: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            message_command_attr: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            message_command_bg: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            message_command_fg: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            message_fg: None,
            #[cfg(feature = "tmux_1_9")]
            message_command_style: None,
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
            message_limit: None,
            #[cfg(feature = "tmux_1_9")]
            message_style: None,
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
            mouse_resize_pane: None,
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
            mouse_select_pane: None,
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
            mouse_select_window: None,
            #[cfg(feature = "tmux_2_1")]
            mouse: None,
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
            mouse_utf8: None,
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
            pane_active_border_bg: None,
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
            pane_active_border_fg: None,
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
            pane_border_bg: None,
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
            pane_border_fg: None,
            #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
            pane_active_border_style: None,
            #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
            pane_border_style: None,
            #[cfg(feature = "tmux_1_0")]
            prefix: None,
            #[cfg(feature = "tmux_1_6")]
            prefix2: None,
            #[cfg(feature = "tmux_1_7")]
            renumber_windows: None,
            #[cfg(feature = "tmux_1_0")]
            repeat_time: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
            set_remain_on_exit: None,
            #[cfg(feature = "tmux_1_0")]
            set_titles: None,
            #[cfg(feature = "tmux_1_0")]
            set_titles_string: None,
            #[cfg(feature = "tmux_2_6")]
            silence_action: None,
            #[cfg(feature = "tmux_1_0")]
            status: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_attr: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_bg: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_fg: None,
            #[cfg(feature = "tmux_2_9")]
            tatus_format: None,
            #[cfg(feature = "tmux_1_0")]
            status_interval: None,
            #[cfg(feature = "tmux_1_0")]
            status_justify: None,
            #[cfg(feature = "tmux_1_0")]
            status_keys: None,
            #[cfg(feature = "tmux_1_0")]
            status_left: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_left_attr: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_left_bg: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_left_fg: None,
            #[cfg(feature = "tmux_1_0")]
            status_left_length: None,
            #[cfg(feature = "tmux_1_9")]
            status_left_style: None,
            #[cfg(feature = "tmux_1_7")]
            status_position: None,
            #[cfg(feature = "tmux_1_0")]
            status_right: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_right_attr: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_right_bg: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            status_right_fg: None,
            #[cfg(feature = "tmux_1_0")]
            status_right_length: None,
            #[cfg(feature = "tmux_1_9")]
            status_right_style: None,
            #[cfg(feature = "tmux_1_9")]
            status_style: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
            status_utf8: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
            terminal_overrides: None,
            #[cfg(feature = "tmux_1_0")]
            update_environment: None,
            #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
            user_keys: None,
            #[cfg(feature = "tmux_1_0")]
            visual_activity: None,
            #[cfg(feature = "tmux_1_0")]
            visual_bell: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
            visual_content: None,
            #[cfg(feature = "tmux_1_4")]
            visual_silence: None,
            #[cfg(feature = "tmux_1_6")]
            word_separators: None,
            user_options: HashMap::new(),
        }
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn activity_action(mut self, activity_action: Option<Action>) -> Self {
        self.activity_action = activity_action;
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn assume_paste_time(mut self, assume_paste_time: Option<usize>) -> Self {
        self.assume_paste_time = assume_paste_time;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn base_index(mut self, base_index: Option<usize>) -> Self {
        self.base_index = base_index;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn bell_action(mut self, bell_action: Option<Action>) -> Self {
        self.bell_action = bell_action;
        self
    }

    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    pub fn bell_on_alert(mut self, bell_on_alert: Option<Switch>) -> Self {
        self.bell_on_alert = bell_on_alert;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    pub fn buffer_limit(mut self, buffer_limit: Option<usize>) -> Self {
        self.buffer_limit = buffer_limit;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn default_command<S: Into<Cow<'a, str>>>(mut self, default_command: Option<S>) -> Self {
        self.default_command = default_command.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn default_shell<S: Into<Cow<'a, str>>>(mut self, default_shell: Option<S>) -> Self {
        self.default_shell = default_shell.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_2_9")]
    pub fn default_size(mut self, default_size: Option<(usize, usize)>) -> Self {
        self.default_size = default_size;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub fn default_terminal<S: Into<Cow<'a, str>>>(mut self, default_terminal: Option<S>) -> Self {
        self.default_terminal = default_terminal.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn destroy_unattached(mut self, destroy_unattached: Option<Switch>) -> Self {
        self.destroy_unattached = destroy_unattached;
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn detach_on_destroy(mut self, detach_on_destroy: Option<DetachOnDestroy>) -> Self {
        self.detach_on_destroy = detach_on_destroy;
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn display_panes_active_colour<S: Into<Cow<'a, str>>>(
        mut self,
        display_panes_active_colour: Option<S>,
    ) -> Self {
        self.display_panes_active_colour = display_panes_active_colour.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn display_panes_colour<S: Into<Cow<'a, str>>>(
        mut self,
        display_panes_colour: Option<S>,
    ) -> Self {
        self.display_panes_colour = display_panes_colour.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn display_panes_time(mut self, display_panes_time: Option<usize>) -> Self {
        self.display_panes_time = display_panes_time;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn display_time(mut self, display_time: Option<usize>) -> Self {
        self.display_time = display_time;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn history_limit(mut self, history_limit: Option<usize>) -> Self {
        self.history_limit = history_limit;
        self
    }

    #[cfg(feature = "tmux_2_2")]
    pub fn key_table<S: Into<Cow<'a, str>>>(mut self, key_table: Option<S>) -> Self {
        self.key_table = key_table.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn lock_after_time(mut self, lock_after_time: Option<usize>) -> Self {
        self.lock_after_time = lock_after_time;
        self
    }

    #[cfg(feature = "tmux_1_1")]
    pub fn lock_command<S: Into<Cow<'a, str>>>(mut self, lock_command: Option<S>) -> Self {
        self.lock_command = lock_command.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    pub fn lock_server<S: Into<Cow<'a, str>>>(mut self, lock_server: Option<S>) -> Self {
        self.lock_server = lock_server.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn message_attr<S: Into<Cow<'a, str>>>(mut self, message_attr: Option<S>) -> Self {
        self.message_attr = message_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn message_bg<S: Into<Cow<'a, str>>>(mut self, message_bg: Option<S>) -> Self {
        self.message_bg = message_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn message_command_attr<S: Into<Cow<'a, str>>>(
        mut self,
        message_command_attr: Option<S>,
    ) -> Self {
        self.message_command_attr = message_command_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn message_command_bg<S: Into<Cow<'a, str>>>(
        mut self,
        message_command_bg: Option<S>,
    ) -> Self {
        self.message_command_bg = message_command_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn message_command_fg<S: Into<Cow<'a, str>>>(
        mut self,
        message_command_fg: Option<S>,
    ) -> Self {
        self.message_command_fg = message_command_fg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn message_fg<S: Into<Cow<'a, str>>>(mut self, message_fg: Option<S>) -> Self {
        self.message_fg = message_fg.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn message_command_style<S: Into<Cow<'a, str>>>(
        mut self,
        message_command_style: Option<S>,
    ) -> Self {
        self.message_command_style = message_command_style.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub fn message_limit<S: Into<Cow<'a, str>>>(mut self, message_limit: Option<S>) -> Self {
        self.message_limit = message_style.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn message_style<S: Into<Cow<'a, str>>>(mut self, message_style: Option<S>) -> Self {
        self.message_style = message_style.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub fn mouse_select_pane<S: Into<Cow<'a, str>>>(
        mut self,
        mouse_select_pane: Option<S>,
    ) -> Self {
        self.mouse_select_pane = mouse_select_pane.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub fn mouse_select_window<S: Into<Cow<'a, str>>>(
        mut self,
        mouse_select_window: Option<S>,
    ) -> Self {
        self.mouse_select_window = mouse_select_window.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn mouse(mut self, mouse: Option<Switch>) -> Self {
        self.mouse = mouse;
        self
    }

    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    pub fn mouse_utf8(mut self, mouse_utf8: Option<Switch>) -> Self {
        self.mouse_utf8 = mouse_utf8;
        self
    }

    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub fn pane_active_border_bg(mut self, pane_active_border_bg: Option<Switch>) -> Self {
        self.pane_active_border_bg = pane_active_border_bg;
        self
    }

    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub fn pane_active_border_fg(mut self, pane_active_border_fg: Option<Switch>) -> Self {
        self.pane_active_border_fg = pane_active_border_fg;
        self
    }

    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub fn pane_border_bg(mut self, pane_border_bg: Option<Switch>) -> Self {
        self.pane_border_bg = pane_border_bg;
        self
    }

    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub fn pane_border_fg(mut self, pane_border_fg: Option<Switch>) -> Self {
        self.pane_border_fg = pane_border_fg;
        self
    }

    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub fn pane_active_border_style(mut self, pane_active_border_style: Option<Switch>) -> Self {
        self.pane_active_border_style = pane_active_border_style;
        self
    }

    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub fn pane_border_style(mut self, pane_border_style: Option<Switch>) -> Self {
        self.pane_border_style = pane_border_style;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn prefix<S: Into<Cow<'a, str>>>(mut self, prefix: Option<S>) -> Self {
        self.prefix = prefix.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn prefix2<S: Into<Cow<'a, str>>>(mut self, prefix2: Option<S>) -> Self {
        self.prefix2 = prefix2.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn renumber_windows(mut self, renumber_windows: Option<Switch>) -> Self {
        self.renumber_windows = renumber_windows;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn repeat_time(mut self, repeat_time: Option<usize>) -> Self {
        self.repeat_time = repeat_time;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    pub fn set_remain_on_exit(mut self, set_remain_on_exit: Option<Switch>) -> Self {
        self.set_remain_on_exit = set_remain_on_exit;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn set_titles(mut self, set_titles: Option<Switch>) -> Self {
        self.set_titles = set_titles;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn set_titles_string<S: Into<Cow<'a, str>>>(
        mut self,
        set_titles_string: Option<S>,
    ) -> Self {
        self.set_titles_string = set_titles_string.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn silence_action(mut self, silence_action: Option<Action>) -> Self {
        self.silence_action = silence_action;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status(mut self, status: Option<Status>) -> Self {
        self.status = status;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_attr<S: Into<Cow<'a, str>>>(mut self, status_attr: Option<S>) -> Self {
        self.status_attr = status_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_bg<S: Into<Cow<'a, str>>>(mut self, status_bg: Option<S>) -> Self {
        self.status_bg = status_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_fg<S: Into<Cow<'a, str>>>(mut self, status_fg: Option<S>) -> Self {
        self.status_fg = status_fg.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_2_9")]
    pub fn status_format<S: Into<Cow<'a, str>>>(mut self, status_format: Option<Vec<S>>) -> Self {
        self.status_format = status_format;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_interval(mut self, status_interval: Option<usize>) -> Self {
        self.status_interval = status_interval;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_justify(mut self, status_justify: Option<StatusJustify>) -> Self {
        self.status_justify = status_justify;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_keys(mut self, status_keys: Option<StatusKeys>) -> Self {
        self.status_keys = status_keys;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_left<S: Into<Cow<'a, str>>>(mut self, status_left: Option<S>) -> Self {
        self.status_left = status_left.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_left_attr<S: Into<Cow<'a, str>>>(mut self, status_left_attr: Option<S>) -> Self {
        self.status_left_attr = status_left_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_left_bg<S: Into<Cow<'a, str>>>(mut self, status_left_bg: Option<S>) -> Self {
        self.status_left_bg = status_left_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_left_fg<S: Into<Cow<'a, str>>>(mut self, status_left_fg: Option<S>) -> Self {
        self.status_left_fg = status_left_fg.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_left_length(mut self, status_left_length: Option<usize>) -> Self {
        self.status_left_length = status_left_length;
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn status_left_style<S: Into<Cow<'a, str>>>(
        mut self,
        status_left_style: Option<S>,
    ) -> Self {
        self.status_left_style = status_left_style.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn status_position(mut self, status_position: Option<StatusPosition>) -> Self {
        self.status_position = status_position;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_right<S: Into<Cow<'a, str>>>(mut self, status_right: Option<S>) -> Self {
        self.status_right = status_right.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_right_attr<S: Into<Cow<'a, str>>>(
        mut self,
        status_right_attr: Option<S>,
    ) -> Self {
        self.status_right_attr = status_right_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_right_bg<S: Into<Cow<'a, str>>>(mut self, status_right_bg: Option<S>) -> Self {
        self.status_right_bg = status_right_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_right_fg<S: Into<Cow<'a, str>>>(mut self, status_right_fg: Option<S>) -> Self {
        self.status_right_fg = status_right_fg.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn status_right_length(mut self, status_right_length: Option<usize>) -> Self {
        self.status_right_length = status_right_length.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn status_right_style<S: Into<Cow<'a, str>>>(
        mut self,
        status_right_style: Option<S>,
    ) -> Self {
        self.status_right_style = status_right_style.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn status_style<S: Into<Cow<'a, str>>>(mut self, status_style: Option<S>) -> Self {
        self.status_style = status_style.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub fn status_utf8(mut self, status_utf8: Option<Switch>) -> Self {
        self.status_utf8 = status_utf8;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub fn terminal_overrides<S: Into<Cow<'a, str>>>(
        mut self,
        terminal_overrides: Option<S>,
    ) -> Self {
        self.terminal_overrides = terminal_overrides.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn update_environment<I, S>(mut self, update_environment: Option<I>) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        self.update_environment =
            update_environment.map(|v| v.into_iter().map(|s| s.into()).collect());
        self
    }

    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    pub fn user_keys<I, S>(mut self, user_keys: Option<I>) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        self.user_keys = user_keys.map(|v| v.into_iter().map(|s| s.into()).collect());
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn visual_activity(mut self, visual_activity: Option<Activity>) -> Self {
        self.visual_activity = visual_activity;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn visual_bell(mut self, visual_bell: Option<Activity>) -> Self {
        self.visual_bell = visual_bell;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub fn visual_content(mut self, visual_content: Option<Activity>) -> Self {
        self.visual_content = visual_content;
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn visual_silence(mut self, visual_silence: Option<Activity>) -> Self {
        self.visual_silence = visual_silence;
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn word_separators<S: Into<Cow<'a, str>>>(mut self, word_separators: Option<S>) -> Self {
        self.word_separators = word_separators.map(|s| s.into());
        self
    }
}

impl<'a> FromStr for SessionOptions<'a> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut session_options = SessionOptions::new();

        for line in s.lines() {
            if let Some((name, i, value)) = get_parts(line) {
                match name {
                    #[cfg(feature = "tmux_2_6")]
                    ACTIVITY_ACTION => {
                        session_options.activity_action = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_8")]
                    ASSUME_PASTE_TIME => {
                        session_options.assume_paste_time = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    BASE_INDEX => session_options.base_index = value.and_then(|s| s.parse().ok()),
                    #[cfg(feature = "tmux_1_0")]
                    BELL_ACTION => session_options.bell_action = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
                    BELL_ON_ALERT => {
                        session_options.bell_on_alert = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
                    BUFFER_LIMIT => {
                        session_options.buffer_limit = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    DEFAULT_COMMAND => session_options.default_command = cow_parse(value),
                    #[cfg(feature = "tmux_1_0")]
                    DEFAULT_SHELL => session_options.default_shell = cow_parse(value),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    DEFAULT_PATH => {
                        session_options.default_path = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
                    DEFAULT_TERMINAL => {
                        session_options.default_terminal = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_2_9")]
                    DEFAULT_SIZE => {
                        session_options.default_size = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_4")]
                    DESTROY_UNATTACHED => {
                        session_options.destroy_unattached = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_4")]
                    DETACH_ON_DESTROY => {
                        session_options.detach_on_destroy = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_2")]
                    DISPLAY_PANES_ACTIVE_COLOUR => {
                        session_options.display_panes_active_colour = cow_parse(value)
                    }
                    #[cfg(feature = "tmux_1_0")]
                    DISPLAY_PANES_COLOUR => session_options.display_panes_colour = cow_parse(value),
                    #[cfg(feature = "tmux_1_0")]
                    DISPLAY_PANES_TIME => {
                        session_options.display_panes_time = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    DISPLAY_TIME => {
                        session_options.display_time = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    HISTORY_LIMIT => {
                        session_options.history_limit = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_2_2")]
                    KEY_TABLE => session_options.key_table = cow_parse(value),
                    #[cfg(feature = "tmux_1_0")]
                    LOCK_AFTER_TIME => {
                        session_options.lock_after_time = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_1")]
                    LOCK_COMMAND => session_options.lock_command = cow_parse(value),
                    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
                    LOCK_SERVER => session_options.lock_server = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    MESSAGE_ATTR => {
                        session_options.message_attr = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    MESSAGE_BG => session_options.message_bg = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    MESSAGE_COMMAND_ATTR => {
                        session_options.message_command_attr = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    MESSAGE_COMMAND_BG => {
                        session_options.message_command_bg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    MESSAGE_COMMAND_FG => {
                        session_options.message_command_fg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    MESSAGE_FG => session_options.message_fg = value.and_then(|s| s.parse().ok()),
                    #[cfg(feature = "tmux_1_9")]
                    MESSAGE_COMMAND_STYLE => {
                        session_options.message_command_style = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
                    MESSAGE_LIMIT => {
                        session_options.message_limit = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_9")]
                    MESSAGE_STYLE => session_options.message_style = cow_parse(value),
                    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
                    MOUSE_RESIZE_PANE => {
                        session_options.mouse_resize_pane = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
                    MOUSE_SELECT_PANE => {
                        session_options.mouse_select_pane = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
                    MOUSE_SELECT_WINDOW => {
                        session_options.mouse_select_window = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_2_1")]
                    MOUSE => session_options.mouse = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
                    MOUSE_UTF8 => session_options.mouse_utf8 = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
                    PANE_ACTIVE_BORDER_BG => {
                        session_options.pane_active_border_bg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
                    PANE_ACTIVE_BORDER_FG => {
                        session_options.pane_active_border_fg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
                    PANE_BORDER_BG => {
                        session_options.pane_border_bg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
                    PANE_BORDER_FG => {
                        session_options.pane_active_border_fg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
                    PANE_ACTIVE_BORDER_STYLE => {
                        session_options.pane_active_border_style =
                            value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
                    PANE_BORDER_STYLE => {
                        session_options.pane_border_style = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    PREFIX => session_options.prefix = cow_parse(value),
                    #[cfg(feature = "tmux_1_6")]
                    PREFIX2 => session_options.prefix2 = cow_parse(value),
                    #[cfg(feature = "tmux_1_7")]
                    RENUMBER_WINDOWS => {
                        session_options.renumber_windows = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    REPEAT_TIME => session_options.repeat_time = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
                    SET_REMAIN_ON_EXIT => {
                        session_options.set_remain_on_exit = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    SET_TITLES => session_options.set_titles = value.and_then(|s| s.parse().ok()),
                    #[cfg(feature = "tmux_1_0")]
                    SET_TITLES_STRING => session_options.set_titles_string = cow_parse(value),
                    #[cfg(feature = "tmux_2_6")]
                    SILENCE_ACTION => {
                        session_options.silence_action = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    STATUS => session_options.status = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    STATUS_ATTR => session_options.status_attr = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    STATUS_BG => session_options.status_bg = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    STATUS_FG => session_options.status_fg = value.and_then(|s| s.parse().ok()),
                    #[cfg(feature = "tmux_2_9")]
                    STATUS_FORMAT => {
                        session_options.status_format = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    STATUS_INTERVAL => {
                        session_options.status_interval = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    STATUS_JUSTIFY => {
                        session_options.status_justify = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    STATUS_KEYS => session_options.status_keys = value.and_then(|s| s.parse().ok()),
                    #[cfg(feature = "tmux_1_0")]
                    STATUS_LEFT => session_options.status_left = cow_parse(value),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    STATUS_LEFT_ATTR => {
                        session_options.status_left_attr = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    STATUS_LEFT_BG => {
                        session_options.status_left_bg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    STATUS_LEFT_FG => {
                        session_options.status_left_fg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    STATUS_LEFT_LENGTH => {
                        session_options.status_left_length = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_9")]
                    STATUS_LEFT_STYLE => session_options.status_left_style = cow_parse(value),
                    #[cfg(feature = "tmux_1_7")]
                    STATUS_POSITION => {
                        session_options.status_position = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    STATUS_RIGHT => session_options.status_right = cow_parse(value),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    STATUS_RIGHT_ATTR => {
                        session_options.status_right_attr = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    STATUS_RIGHT_BG => {
                        session_options.status_right_bg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    STATUS_RIGHT_FG => {
                        session_options.status_right_fg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    STATUS_RIGHT_LENGTH => {
                        session_options.status_right_length = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_9")]
                    STATUS_RIGHT_STYLE => session_options.status_right_style = cow_parse(value),
                    #[cfg(feature = "tmux_1_9")]
                    STATUS_STYLE => session_options.status_style = cow_parse(value),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
                    STATUS_UTF8 => session_options.status_utf8 = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
                    TERMINAL_OVERRIDES => {
                        session_options.terminal_overrides = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    UPDATE_ENVIRONMENT => array_insert(
                        &mut session_options.update_environment,
                        i,
                        value.and_then(|s| s.parse().ok()),
                    ),
                    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
                    USER_KEYS => array_insert(
                        &mut session_options.user_keys,
                        i,
                        value.and_then(|s| s.parse().ok()),
                    ),
                    #[cfg(feature = "tmux_1_0")]
                    VISUAL_ACTIVITY => {
                        session_options.visual_activity = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    VISUAL_BELL => session_options.visual_bell = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
                    VISUAL_CONTENT => {
                        session_options.visual_content = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_4")]
                    VISUAL_SILENCE => {
                        session_options.visual_silence = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_6")]
                    WORD_SEPARATORS => session_options.word_separators = cow_parse(value),
                    _ => {
                        // if user option (@user_option value)
                        if let Some(name) = name.strip_prefix('@') {
                            session_options
                                .user_options
                                .insert(name.to_string(), cow_parse(value));
                        }
                    }
                }
            }
        }

        Ok(session_options)
    }
}
