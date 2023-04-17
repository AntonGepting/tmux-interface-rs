use super::*;
use crate::options::common::{array_insert, cow_parse, get_parts, option_to_string};
use crate::options::StatusKeys;
use crate::Switch;
use crate::{Action, Activity, DetachOnDestroy, Error, Status, StatusJustify, StatusPosition};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

// TODO: Vec variables solution
// TODO: check types
// 45 Available session options are:
#[derive(PartialEq, Default, Clone, Debug)]
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
    pub lock_server: Option<Switch>,
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
    // NOTE: Tmux struct wrong decision here, because of dircet and control mode
    //Self::get_all_ext(None)
    //}

    // do not create anything, just get option from tmux
    //pub fn get_from_new_server(bitflags: u128) -> Result<Self, Error> {
    //let mut cmds = TmuxBinCommands::default();

    //let start_server = StartServer::new();
    //cmds.push(start_server);

    //Self::get_ext(Some(&mut cmds), bitflags)
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

impl<'a> fmt::Display for SessionOptions<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();

        #[cfg(feature = "tmux_2_6")]
        option_to_string(&mut v, ACTIVITY_ACTION, &self.activity_action);
        #[cfg(feature = "tmux_1_8")]
        option_to_string(&mut v, ASSUME_PASTE_TIME, &self.assume_paste_time);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, BASE_INDEX, &self.base_index);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, BELL_ACTION, &self.bell_action);
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
        option_to_string(&mut v, BELL_ON_ALERT, &self.bell_on_alert);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
        option_to_string(&mut v, BUFFER_LIMIT, &self.buffer_limit);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, DEFAULT_COMMAND, &self.default_command);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, DEFAULT_SHELL, &self.default_shell);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, DEFAULT_PATH, &self.default_path);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
        option_to_string(&mut v, DEFAULT_TERMINAL, &self.default_terminal);
        // #[cfg(feature = "tmux_2_9")]
        // option_to_string(&mut v, DEFAULT_SIZE, &self.default_size);
        #[cfg(feature = "tmux_1_4")]
        option_to_string(&mut v, DESTROY_UNATTACHED, &self.destroy_unattached);
        #[cfg(feature = "tmux_1_4")]
        option_to_string(&mut v, DETACH_ON_DESTROY, &self.detach_on_destroy);
        #[cfg(feature = "tmux_1_2")]
        option_to_string(
            &mut v,
            DISPLAY_PANES_ACTIVE_COLOUR,
            &self.display_panes_active_colour,
        );
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, DISPLAY_PANES_COLOUR, &self.display_panes_colour);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, DISPLAY_PANES_TIME, &self.display_panes_time);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, DISPLAY_TIME, &self.display_time);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, HISTORY_LIMIT, &self.history_limit);
        #[cfg(feature = "tmux_2_2")]
        option_to_string(&mut v, KEY_TABLE, &self.key_table);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, LOCK_AFTER_TIME, &self.lock_after_time);
        #[cfg(feature = "tmux_1_1")]
        option_to_string(&mut v, LOCK_COMMAND, &self.lock_command);
        #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
        option_to_string(&mut v, LOCK_SERVER, &self.lock_server);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, MESSAGE_ATTR, &self.message_attr);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, MESSAGE_BG, &self.message_bg);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, MESSAGE_COMMAND_ATTR, &self.message_command_attr);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, MESSAGE_COMMAND_BG, &self.message_command_bg);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, MESSAGE_COMMAND_FG, &self.message_command_fg);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, MESSAGE_FG, &self.message_fg);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(&mut v, MESSAGE_COMMAND_STYLE, &self.message_command_style);
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
        option_to_string(&mut v, MESSAGE_LIMIT, &self.message_limit);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(&mut v, MESSAGE_STYLE, &self.message_style);
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        option_to_string(&mut v, MOUSE_RESIZE_PANE, &self.mouse_resize_pane);
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        option_to_string(&mut v, MOUSE_SELECT_PANE, &self.mouse_select_pane);
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        option_to_string(&mut v, MOUSE_SELECT_WINDOW, &self.mouse_select_window);
        #[cfg(feature = "tmux_2_1")]
        option_to_string(&mut v, MOUSE, &self.mouse);
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
        option_to_string(&mut v, MOUSE_UTF8, &self.mouse_utf8);
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, PANE_ACTIVE_BORDER_BG, &self.pane_active_border_bg);
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, PANE_ACTIVE_BORDER_FG, &self.pane_active_border_fg);
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, PANE_BORDER_BG, &self.pane_border_bg);
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, PANE_BORDER_FG, &self.pane_border_fg);
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        option_to_string(
            &mut v,
            PANE_ACTIVE_BORDER_STYLE,
            &self.pane_active_border_style,
        );
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        option_to_string(&mut v, PANE_BORDER_STYLE, &self.pane_border_style);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, PREFIX, &self.prefix);
        #[cfg(feature = "tmux_1_6")]
        option_to_string(&mut v, PREFIX2, &self.prefix2);
        #[cfg(feature = "tmux_1_7")]
        option_to_string(&mut v, RENUMBER_WINDOWS, &self.renumber_windows);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, REPEAT_TIME, &self.repeat_time);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
        option_to_string(&mut v, SET_REMAIN_ON_EXIT, &self.set_remain_on_exit);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, SET_TITLES, &self.set_titles);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, SET_TITLES_STRING, &self.set_titles_string);
        #[cfg(feature = "tmux_2_6")]
        option_to_string(&mut v, SILENCE_ACTION, &self.silence_action);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, STATUS, &self.status);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, STATUS_ATTR, &self.status_attr);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, STATUS_BG, &self.status_bg);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, STATUS_FG, &self.status_fg);
        // #[cfg(feature = "tmux_2_9")]
        // option_to_string(&mut v, STATUS_FORMAT, &self.status_format);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, STATUS_INTERVAL, &self.status_interval);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, STATUS_JUSTIFY, &self.status_justify);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, STATUS_KEYS, &self.status_keys);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, STATUS_LEFT, &self.status_left);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, STATUS_LEFT_ATTR, &self.status_left_attr);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, STATUS_LEFT_BG, &self.status_left_bg);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, STATUS_LEFT_FG, &self.status_left_fg);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, STATUS_LEFT_LENGTH, &self.status_left_length);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(&mut v, STATUS_LEFT_STYLE, &self.status_left_style);
        #[cfg(feature = "tmux_1_7")]
        option_to_string(&mut v, STATUS_POSITION, &self.status_position);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, STATUS_RIGHT, &self.status_right);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, STATUS_RIGHT_ATTR, &self.status_right_attr);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, STATUS_RIGHT_BG, &self.status_right_bg);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, STATUS_RIGHT_FG, &self.status_right_fg);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, STATUS_RIGHT_LENGTH, &self.status_right_length);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(&mut v, STATUS_RIGHT_STYLE, &self.status_right_style);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(&mut v, STATUS_STYLE, &self.status_style);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
        option_to_string(&mut v, STATUS_UTF8, &self.status_utf8);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        option_to_string(&mut v, TERMINAL_OVERRIDES, &self.terminal_overrides);
        // #[cfg(feature = "tmux_1_0")]
        // option_to_string(&mut v, UPDATE_ENVIRONMENT, &self.update_environment);
        // #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
        // option_to_string(&mut v, USER_KEYS, &self.user_keys);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, VISUAL_ACTIVITY, &self.visual_activity);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, VISUAL_BELL, &self.visual_bell);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        option_to_string(&mut v, VISUAL_CONTENT, &self.visual_content);
        #[cfg(feature = "tmux_1_4")]
        option_to_string(&mut v, VISUAL_SILENCE, &self.visual_silence);
        #[cfg(feature = "tmux_1_6")]
        option_to_string(&mut v, WORD_SEPARATORS, &self.word_separators);

        // option_to_string(&mut v, , &self.user_options);

        // option_to_string(&mut v, USER_OPTIONS, &self.user_options);
        let s = v.join("\n");
        write!(f, "{}", s)
    }
}

impl<'a> SessionOptions<'a> {
    pub fn new() -> Self {
        let options = SessionOptions::default();
        #[cfg(feature = "tmux_2_6")]
        let options = options.activity_action(Some(ACTIVITY_ACTION_DEFAULT));
        #[cfg(feature = "tmux_1_8")]
        let options = options.assume_paste_time(Some(ASSUME_PASTE_TIME_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.base_index(Some(BASE_INDEX_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.bell_action(Some(BELL_ACTION_DEFAULT));
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
        let options = options.bell_on_alert(Some(BELL_ON_ALERT_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
        let options = options.buffer_limit(Some(BUFFER_LIMIT_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.default_command(Some(DEFAULT_COMMAND_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.default_shell(Some(DEFAULT_SHELL_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.default_path(Some(DEFAULT_PATH_DEFAULT));
        #[cfg(feature = "tmux_2_9")]
        let options = options.default_size(Some(DEFAULT_SIZE_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
        let options = options.default_terminal(Some(DEFAULT_TERMINAL_DEFAULT));
        #[cfg(feature = "tmux_1_4")]
        let options = options.destroy_unattached(Some(DESTROY_UNATTACHED_DEFAULT));
        #[cfg(feature = "tmux_1_4")]
        let options = options.detach_on_destroy(Some(DETACH_ON_DESTROY_DEFAULT));
        #[cfg(feature = "tmux_1_2")]
        let options =
            options.display_panes_active_colour(Some(DISPLAY_PANES_ACTIVE_COLOUR_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.display_panes_colour(Some(DISPLAY_PANES_COLOUR_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.display_panes_time(Some(DISPLAY_PANES_TIME_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.display_time(Some(DISPLAY_TIME_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.history_limit(Some(HISTORY_LIMIT_DEFAULT));
        #[cfg(feature = "tmux_2_2")]
        let options = options.key_table(Some(KEY_TABLE_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.lock_after_time(Some(LOCK_AFTER_TIME_DEFAULT));
        #[cfg(feature = "tmux_1_1")]
        let options = options.lock_command(Some(LOCK_COMMAND_DEFAULT));
        #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
        let options = options.lock_server(Some(LOCK_SERVER_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.message_attr(Some(MESSAGE_ATTR_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.message_bg(Some(MESSAGE_BG_DEFAULT));
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.message_command_attr(Some(MESSAGE_COMMAND_ATTR_DEFAULT));
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.message_command_bg(Some(MESSAGE_COMMAND_BG_DEFAULT));
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.message_command_fg(Some(MESSAGE_COMMAND_FG_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.message_fg(Some(MESSAGE_FG_DEFAULT));
        #[cfg(feature = "tmux_1_9")]
        let options = options.message_command_style(Some(MESSAGE_COMMAND_STYLE_DEFAULT));
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
        let options = options.message_limit(Some(MESSAGE_LIMIT_DEFAULT));
        #[cfg(feature = "tmux_1_9")]
        let options = options.message_style(Some(MESSAGE_STYLE_DEFAULT));
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        let options = options.mouse_resize_pane(Some(MOUSE_RESIZE_PANE_DEFAULT));
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        let options = options.mouse_select_pane(Some(MOUSE_SELECT_PANE_DEFAULT));
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        let options = options.mouse_select_window(Some(MOUSE_SELECT_WINDOW_DEFAULT));
        #[cfg(feature = "tmux_2_1")]
        let options = options.mouse(Some(MOUSE_DEFAULT));
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
        let options = options.mouse_utf8(Some(MOUSE_UTF8_DEFAULT));
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let options = options.pane_active_border_bg(Some(PANE_ACTIVE_BORDER_BG_DEFAULT));
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let options = options.pane_active_border_fg(Some(PANE_ACTIVE_BORDER_FG_DEFAULT));
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let options = options.pane_border_bg(Some(PANE_BORDER_BG_DEFAULT));
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let options = options.pane_border_fg(Some(PANE_BORDER_FG_DEFAULT));
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        let options = options.pane_active_border_style(Some(PANE_ACTIVE_BORDER_STYLE_DEFAULT));
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        let options = options.pane_border_style(Some(PANE_BORDER_STYLE_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.prefix(Some(PREFIX_DEFAULT));
        #[cfg(feature = "tmux_1_6")]
        let options = options.prefix2(Some(PREFIX2_DEFAULT)); // KEYC_NONE
        #[cfg(feature = "tmux_1_7")]
        let options = options.renumber_windows(Some(RENUMBER_WINDOWS_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.repeat_time(Some(REPEAT_TIME_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
        let options = options.set_remain_on_exit(Some(SET_REMAIN_ON_EXIT_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.set_titles(Some(SET_TITLES_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.set_titles_string(Some(SET_TITLES_STRING_DEFAULT));
        #[cfg(feature = "tmux_2_6")]
        let options = options.silence_action(Some(SILENCE_ACTION_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.status(Some(STATUS_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_attr(Some(STATUS_ATTR_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_bg(Some(STATUS_BG_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_fg(Some(STATUS_FG_DEFAULT));
        #[cfg(feature = "tmux_2_9")]
        let options = options.status_format(Some(STATUS_FORMAT_DEFAULT)); // options_table_status_format_default
        #[cfg(feature = "tmux_1_0")]
        let options = options.status_interval(Some(STATUS_INTERVAL_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.status_justify(Some(STATUS_JUSTIFY_DEFAULT)); // 0
        #[cfg(feature = "tmux_1_0")]
        let options = options.status_keys(Some(STATUS_KEYS_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.status_left(Some(STATUS_LEFT_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_left_attr(Some(STATUS_LEFT_ATTR_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_left_bg(Some(STATUS_LEFT_BG_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_left_fg(Some(STATUS_LEFT_FG_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.status_left_length(Some(STATUS_LEFT_LENGTH_DEFAULT));
        #[cfg(feature = "tmux_1_9")]
        let options = options.status_left_style(Some(STATUS_LEFT_STYLE_DEFAULT));
        #[cfg(feature = "tmux_1_7")]
        let options = options.status_position(Some(STATUS_POSITION_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.status_right(Some(STATUS_RIGHT_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_right_attr(Some(STATUS_RIGHT_ATTR_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_right_bg(Some(STATUS_RIGHT_BG_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.status_right_fg(Some(STATUS_RIGHT_FG_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.status_right_length(Some(STATUS_RIGHT_LENGTH_DEFAULT));
        #[cfg(feature = "tmux_1_9")]
        let options = options.status_right_style(Some(STATUS_RIGHT_STYLE_DEFAULT));
        #[cfg(feature = "tmux_1_9")]
        let options = options.status_style(Some(STATUS_STYLE_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
        let options = options.status_utf8(Some(STATUS_UTF8_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        let options = options.terminal_overrides(Some(TERMINAL_OVERRIDES_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.update_environment(Some(UPDATE_ENVIRONMENT_DEFAULT));
        #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
        let options = options.user_keys(Some(USER_KEYS_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.visual_activity(Some(VISUAL_ACTIVITY_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.visual_bell(Some(VISUAL_BELL_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        let options = options.visual_content(Some(VISUAL_CONTENT_DEFAULT));
        #[cfg(feature = "tmux_1_4")]
        let options = options.visual_silence(Some(VISUAL_SILENCE_DEFAULT));
        #[cfg(feature = "tmux_1_6")]
        let options = options.word_separators(Some(WORD_SEPARATORS_DEFAULT));

        options
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
    pub fn lock_server(mut self, lock_server: Option<Switch>) -> Self {
        self.lock_server = lock_server;
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
    pub fn message_limit(mut self, message_limit: Option<usize>) -> Self {
        self.message_limit = message_limit;
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn message_style<S: Into<Cow<'a, str>>>(mut self, message_style: Option<S>) -> Self {
        self.message_style = message_style.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub fn mouse_resize_pane(mut self, mouse_resize_pane: Option<Switch>) -> Self {
        self.mouse_resize_pane = mouse_resize_pane;
        self
    }

    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub fn mouse_select_pane(mut self, mouse_select_pane: Option<Switch>) -> Self {
        self.mouse_select_pane = mouse_select_pane;
        self
    }

    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub fn mouse_select_window(mut self, mouse_select_window: Option<Switch>) -> Self {
        self.mouse_select_window = mouse_select_window;
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
    pub fn pane_active_border_style<S: Into<Cow<'a, str>>>(
        mut self,
        pane_active_border_style: Option<S>,
    ) -> Self {
        self.pane_active_border_style = pane_active_border_style.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub fn pane_border_style<S: Into<Cow<'a, str>>>(
        mut self,
        pane_border_style: Option<S>,
    ) -> Self {
        self.pane_border_style = pane_border_style.map(|s| s.into());
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
    pub fn status_format<S, I>(mut self, status_format: Option<I>) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        self.status_format = status_format.map(|v| v.into_iter().map(|s| s.into()).collect());
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
    pub fn visual_content(mut self, visual_content: Option<Switch>) -> Self {
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
        let mut session_options = SessionOptions::default();

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
                    DEFAULT_TERMINAL => session_options.default_terminal = cow_parse(value),
                    // #[cfg(feature = "tmux_2_9")]
                    // DEFAULT_SIZE => {
                    // session_options.default_size = value.and_then(|s| s.parse().ok())
                    // }
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
                        session_options.pane_active_border_style = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
                    PANE_BORDER_STYLE => session_options.pane_border_style = cow_parse(value),
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
                    STATUS_FORMAT => array_insert(
                        &mut session_options.status_format,
                        i,
                        value.and_then(|s| s.parse().ok()),
                    ),
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
                    TERMINAL_OVERRIDES => session_options.terminal_overrides = cow_parse(value),
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
