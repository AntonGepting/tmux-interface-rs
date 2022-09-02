use super::{Action, Activity, DetachOnDestroy, Status, StatusJustify, StatusPosition};
use crate::options::StatusKeys;
use crate::{
    SetLocalSessionOptions, SetSessionOption, SetSessionOptions, ShowOptions, Switch, TmuxCommand,
    TmuxCommands,
};
use std::borrow::Cow;

// TODO: Vec variables solution
// TODO: check types
// 45 Available session options are:
//#[derive(Default, PartialEq, Clone, Debug)]
//pub struct SessionOptions {
////activity-action [any | none | current | other]
//#[cfg(feature = "tmux_2_6")]
//pub activity_action: Option<Activity>,
////assume-paste-time milliseconds
//#[cfg(feature = "tmux_1_8")]
//pub assume_paste_time: Option<usize>,
////base-index index
//#[cfg(feature = "tmux_1_0")]
//pub base_index: Option<usize>,
////bell-action [any | none | current | other]
//// tmux 1.0: bell-action [any | none | other]
//#[cfg(feature = "tmux_1_0")]
//pub bell_action: Option<Action>,
////bell-on-alert [on | off]
//#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
//pub bell_on_alert: Option<Switch>,
////buffer-limit limit
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
//pub buffer_limit: Option<usize>,
////default-command shell-command
//#[cfg(feature = "tmux_1_0")]
//pub default_command: Option<String>,
////default-shell path
//#[cfg(feature = "tmux_1_0")]
//pub default_shell: Option<String>,
////default-path path
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub default_path: Option<String>,
//// default-terminal terminal
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
//pub default_terminal: Option<String>,
////default-size XxY
//#[cfg(feature = "tmux_2_9")]
//pub default_size: Option<(usize, usize)>,
////destroy-unattached [on | off]
//#[cfg(feature = "tmux_1_4")]
//pub destroy_unattached: Option<Switch>,
////detach-on-destroy [on | off]
//// tmux ^3.2 detach-on-destroy [on | off | no-detached]
//#[cfg(feature = "tmux_1_4")]
//pub detach_on_destroy: Option<DetachOnDestroy>,
////display-panes-active-colour colour
//#[cfg(feature = "tmux_1_2")]
//pub display_panes_active_colour: Option<String>,
////display-panes-colour colour
//#[cfg(feature = "tmux_1_0")]
//pub display_panes_colour: Option<String>,
////display-panes-time time
//#[cfg(feature = "tmux_1_0")]
//pub display_panes_time: Option<usize>,
////display-time time
//#[cfg(feature = "tmux_1_0")]
//pub display_time: Option<usize>,
////history-limit lines
//#[cfg(feature = "tmux_1_0")]
//pub history_limit: Option<usize>,
////key-table key-table
//#[cfg(feature = "tmux_2_2")]
//pub key_table: Option<String>,
////lock-after-time number
//#[cfg(feature = "tmux_1_0")]
//pub lock_after_time: Option<usize>,
////lock-command shell-command
//#[cfg(feature = "tmux_1_1")]
//pub lock_command: Option<String>,
////lock-server [on | off]
//#[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
//pub lock_server: Option<String>,
////message-attr attributes
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub message_attr: Option<String>,
////message-bg colour
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub message_bg: Option<String>,
////message-command-attr attributes
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//pub message_command_attr: Option<String>,
////message-command-bg colour
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//pub message_command_bg: Option<String>,
////message-command-fg colour
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//pub message_command_fg: Option<String>,
////message-fg colour
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub message_fg: Option<String>,
////message-command-style style
//#[cfg(feature = "tmux_1_9")]
//pub message_command_style: Option<String>,
////message-limit number
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
//pub message_limit: Option<usize>,
////message-style style
//#[cfg(feature = "tmux_1_9")]
//pub message_style: Option<String>,
////mouse-resize-pane [on | off]
//#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
//pub mouse_resize_pane: Option<Switch>,
////mouse-select-pane [on | off]
//#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
//pub mouse_select_pane: Option<Switch>,
////mouse-select-window [on | off]
//#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
//pub mouse_select_window: Option<Switch>,
////mouse [on | off]
//#[cfg(feature = "tmux_2_1")]
//pub mouse: Option<Switch>,
//// mouse-utf8 [on | off]
//#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
//pub mouse_utf8: Option<Switch>,
////pane-active-border-bg colour
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
//pub pane_active_border_bg: Option<String>,
////pane-active-border-fg colour
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
//pub pane_active_border_fg: Option<String>,
////pane-border-bg colour
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
//pub pane_border_bg: Option<String>,
////pane-border-fg colour
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
//pub pane_border_fg: Option<String>,
////pane-active-border-style style
//#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
//pub pane_active_border_style: Option<String>,
////pane-border-style style
//#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
//pub pane_border_style: Option<String>,
////prefix key
//#[cfg(feature = "tmux_1_0")]
//pub prefix: Option<String>,
////prefix2 key
//#[cfg(feature = "tmux_1_6")]
//pub prefix2: Option<String>,
////renumber-windows [on | off]
//#[cfg(feature = "tmux_1_7")]
//pub renumber_windows: Option<Switch>,
////repeat-time time
//#[cfg(feature = "tmux_1_0")]
//pub repeat_time: Option<usize>,
////set-remain-on-exit [on | off]
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
//pub set_remain_on_exit: Option<Switch>,
////set-titles [on | off]
//#[cfg(feature = "tmux_1_0")]
//pub set_titles: Option<Switch>,
////set-titles-string string
//#[cfg(feature = "tmux_1_0")]
//pub set_titles_string: Option<String>,
////silence-action [any | none | current | other]
//#[cfg(feature = "tmux_2_6")]
//pub silence_action: Option<Action>,
////status [off | on | 2 | 3 | 4 | 5]
////tmux 1.0: status [off | on]
//#[cfg(feature = "tmux_1_0")]
//pub status: Option<Status>,
////status-attr attributes
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub status_attr: Option<String>,
////status-bg colour
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub status_bg: Option<String>,
////status-fg colour
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub status_fg: Option<String>,
////status-format[] format
//#[cfg(feature = "tmux_2_9")]
//pub status_format: Option<Vec<String>>,
////status-interval interval
//#[cfg(feature = "tmux_1_0")]
//pub status_interval: Option<usize>,
////status-justify [left | centre | right]
//#[cfg(feature = "tmux_1_0")]
//pub status_justify: Option<StatusJustify>,
////status-keys [vi | emacs]
//#[cfg(feature = "tmux_1_0")]
//pub status_keys: Option<StatusKeys>,
////status-left string
//#[cfg(feature = "tmux_1_0")]
//pub status_left: Option<String>,
////status-left-attr attributes
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub status_left_attr: Option<String>,
////status-left-bg colour
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub status_left_bg: Option<String>,
////status-left-fg colour
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub status_left_fg: Option<String>,
////status-left-length length
//#[cfg(feature = "tmux_1_0")]
//pub status_left_length: Option<usize>,
////status-left-style style
//#[cfg(feature = "tmux_1_9")]
//pub status_left_style: Option<String>,
////status-position [top | bottom]
//#[cfg(feature = "tmux_1_7")]
//pub status_position: Option<StatusPosition>,
////status-right string
//#[cfg(feature = "tmux_1_0")]
//pub status_right: Option<String>,
////status-right-attr attributes
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub status_right_attr: Option<String>,
////status-right-bg colour
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub status_right_bg: Option<String>,
////status-right-fg colour
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub status_right_fg: Option<String>,
////status-right-length length
//#[cfg(feature = "tmux_1_0")]
//pub status_right_length: Option<usize>,
////status-right-style style
//#[cfg(feature = "tmux_1_9")]
//pub status_right_style: Option<String>,
////status-style style
//#[cfg(feature = "tmux_1_9")]
//pub status_style: Option<String>,
////status-utf8 [on | off]
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
//pub status_utf8: Option<Switch>,
////terminal-overrides string
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
//pub terminal_overrides: Option<String>,
////update-environment[] variable
//#[cfg(feature = "tmux_1_0")]
//pub update_environment: Option<Vec<String>>,
////user-keys
//#[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
//pub user_keys: Option<Vec<String>>,
////visual-activity [on | off | both]
////tmux 1.0: visual-activity [on | off]
//#[cfg(feature = "tmux_1_0")]
//pub visual_activity: Option<Activity>,
////visual-bell [on | off | both]
////tmux 1.0: visual-bell [on | off]
//#[cfg(feature = "tmux_1_0")]
//pub visual_bell: Option<Activity>,
////visual-content [on | off]
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
//pub visual_content: Option<Switch>,
////visual-silence [on | off | both]
//#[cfg(feature = "tmux_1_4")]
//pub visual_silence: Option<Activity>,
////word-separators string
//#[cfg(feature = "tmux_1_6")]
//pub word_separators: Option<String>,
////pub user_options: Option<HashMap<String, String>>
//}

impl<'a> SessionOptions<'a> {
    pub fn get() -> TmuxCommand<'a> {
        ShowOptions::new().build()
        //GetServerOptions::new();
    }

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
    //#[cfg(feature = "tmux_1_0")]
    //let cmds = cmds.update_environment(self.update_environment);
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
    //// `@USER_OPTION`

    //cmds.build()

    //// NOTE: Tmux struct wrong decision here, because of dircet and control mode
    ////Self::get_all_ext(None)
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

#[derive(Default, Debug)]
pub struct SessionOptions<'a> {
    #[cfg(feature = "tmux_2_6")]
    pub activity_action: Option<Action>,
    #[cfg(feature = "tmux_1_8")]
    pub assume_paste_time: Option<usize>,
    #[cfg(feature = "tmux_1_0")]
    pub base_index: Option<usize>,
    #[cfg(feature = "tmux_1_0")]
    pub bell_action: Option<Action>,
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    pub bell_on_alert: Option<Switch>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    pub buffer_limit: Option<usize>,
    #[cfg(feature = "tmux_1_0")]
    pub default_command: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub default_shell: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub default_path: Option<&'a str>,
    #[cfg(feature = "tmux_2_9")]
    pub default_size: Option<(usize, usize)>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub default_terminal: Option<&'a str>,
    #[cfg(feature = "tmux_1_4")]
    pub destroy_unattached: Option<Switch>,
    #[cfg(feature = "tmux_1_4")]
    pub detach_on_destroy: Option<DetachOnDestroy>,
    #[cfg(feature = "tmux_1_2")]
    pub display_panes_active_colour: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub display_panes_colour: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub display_panes_time: Option<usize>,
    #[cfg(feature = "tmux_1_0")]
    pub display_time: Option<usize>,
    #[cfg(feature = "tmux_1_0")]
    pub history_limit: Option<usize>,
    #[cfg(feature = "tmux_2_2")]
    pub key_table: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub lock_after_time: Option<usize>,
    #[cfg(feature = "tmux_1_1")]
    pub lock_command: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    pub lock_server: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub message_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub message_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub message_command_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub message_command_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub message_command_fg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub message_fg: Option<&'a str>,
    #[cfg(feature = "tmux_1_9")]
    pub message_command_style: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub message_limit: Option<usize>,
    #[cfg(feature = "tmux_1_9")]
    pub message_style: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub mouse_resize_pane: Option<Switch>,
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub mouse_select_pane: Option<Switch>,
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub mouse_select_window: Option<Switch>,
    #[cfg(feature = "tmux_2_1")]
    pub mouse: Option<Switch>,
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    pub mouse_utf8: Option<Switch>,
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub pane_active_border_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub pane_active_border_fg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub pane_border_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub pane_border_fg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub pane_active_border_style: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub pane_border_style: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub prefix: Option<&'a str>,
    #[cfg(feature = "tmux_1_6")]
    pub prefix2: Option<&'a str>,
    #[cfg(feature = "tmux_1_7")]
    pub renumber_windows: Option<Switch>,
    #[cfg(feature = "tmux_1_0")]
    pub repeat_time: Option<usize>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    pub set_remain_on_exit: Option<Switch>,
    #[cfg(feature = "tmux_1_0")]
    pub set_titles: Option<Switch>,
    #[cfg(feature = "tmux_1_0")]
    pub set_titles_string: Option<&'a str>,
    #[cfg(feature = "tmux_2_6")]
    pub silence_action: Option<Action>,
    #[cfg(feature = "tmux_1_0")]
    pub status: Option<Status>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_fg: Option<&'a str>,
    #[cfg(feature = "tmux_2_9")]
    pub status_format: Option<Vec<&'a str>>,
    #[cfg(feature = "tmux_1_0")]
    pub status_interval: Option<usize>,
    #[cfg(feature = "tmux_1_0")]
    pub status_justify: Option<StatusJustify>,
    #[cfg(feature = "tmux_1_0")]
    pub status_keys: Option<StatusKeys>,
    #[cfg(feature = "tmux_1_0")]
    pub status_left: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_left_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_left_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_left_fg: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub status_left_length: Option<usize>,
    #[cfg(feature = "tmux_1_9")]
    pub status_left_style: Option<&'a str>,
    #[cfg(feature = "tmux_1_7")]
    pub status_position: Option<StatusPosition>,
    #[cfg(feature = "tmux_1_0")]
    pub status_right: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_right_attr: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_right_bg: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub status_right_fg: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub status_right_length: Option<usize>,
    #[cfg(feature = "tmux_1_9")]
    pub status_right_style: Option<&'a str>,
    #[cfg(feature = "tmux_1_9")]
    pub status_style: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub status_utf8: Option<Switch>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub terminal_overrides: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub update_environment: Option<Vec<&'a str>>,
    #[cfg(feature = "tmux_1_0")]
    pub visual_activity: Option<Activity>,
    #[cfg(feature = "tmux_1_0")]
    pub visual_bell: Option<Activity>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub visual_content: Option<Switch>,
    #[cfg(feature = "tmux_1_4")]
    pub visual_silence: Option<Activity>,
    #[cfg(feature = "tmux_1_6")]
    pub word_separators: Option<Cow<'a, str>>,
}

impl<'a> SessionOptions<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn activity_action(mut self, activity_action: Action) -> Self {
        self.activity_action = Some(activity_action);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn assume_paste_time(mut self, assume_paste_time: usize) -> Self {
        self.assume_paste_time = Some(assume_paste_time);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn base_index(mut self, base_index: usize) -> Self {
        self.base_index = Some(base_index);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn bell_action(mut self, bell_action: Action) -> Self {
        self.bell_action = Some(bell_action);
        self
    }

    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    pub fn bell_on_alert(mut self, bell_on_alert: Switch) -> Self {
        self.bell_on_alert = Some(bell_on_alert);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn default_command(mut self, default_command: &'a str) -> Self {
        self.default_command = Some(default_command);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn default_shell(mut self, default_shell: &'a str) -> Self {
        self.default_shell = Some(default_shell);
        self
    }

    #[cfg(feature = "tmux_2_9")]
    pub fn default_size(mut self, default_size: (usize, usize)) -> Self {
        self.default_size = Some(default_size);
        self
    }

    //#[cfg(feature = "tmux_2_9")]
    //pub fn default_terminal(mut self, default_terminal: (usize, usize)) ->  Self {
    //self.default_size = Some(default_size);
    //self
    //}

    #[cfg(feature = "tmux_1_4")]
    pub fn destroy_unattached(mut self, destroy_unattached: Switch) -> Self {
        self.destroy_unattached = Some(destroy_unattached);
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn detach_on_destroy(mut self, detach_on_destroy: DetachOnDestroy) -> Self {
        self.detach_on_destroy = Some(detach_on_destroy);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn display_panes_active_colour(mut self, display_panes_active_colour: &'a str) -> Self {
        self.display_panes_active_colour = Some(display_panes_active_colour);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn display_panes_colour(mut self, display_panes_colour: &'a str) -> Self {
        self.display_panes_colour = Some(display_panes_colour);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn display_panes_time(mut self, display_panes_time: usize) -> Self {
        self.display_panes_time = Some(display_panes_time);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn display_time(mut self, display_time: usize) -> Self {
        self.display_time = Some(display_time);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn history_limit(mut self, history_limit: usize) -> Self {
        self.history_limit = Some(history_limit);
        self
    }

    #[cfg(feature = "tmux_2_2")]
    pub fn key_table(mut self, key_table: &'a str) -> Self {
        self.key_table = Some(key_table);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn lock_after_time(mut self, lock_after_time: usize) -> Self {
        self.lock_after_time = Some(lock_after_time);
        self
    }

    #[cfg(feature = "tmux_1_1")]
    pub fn lock_command(mut self, lock_command: &'a str) -> Self {
        self.lock_command = Some(lock_command);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn message_command_style(mut self, message_command_style: &'a str) -> Self {
        self.message_command_style = Some(message_command_style);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn message_style(mut self, message_style: &'a str) -> Self {
        self.message_style = Some(message_style);
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn mouse(mut self, mouse: Switch) -> Self {
        self.mouse = Some(mouse);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn prefix(mut self, prefix: &'a str) -> Self {
        self.prefix = Some(prefix);
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn prefix2(mut self, prefix2: &'a str) -> Self {
        self.prefix2 = Some(prefix2);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn renumber_windows(mut self, renumber_windows: Switch) -> Self {
        self.renumber_windows = Some(renumber_windows);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn repeat_time(mut self, repeat_time: usize) -> Self {
        self.repeat_time = Some(repeat_time);
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    pub fn set_remain_on_exit(mut self, set_remain_on_exit: Switch) -> Self {
        self.set_remain_on_exit = Some(set_remain_on_exit);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn set_titles(mut self, set_titles: Switch) -> Self {
        self.set_titles = Some(set_titles);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn set_titles_string(mut self, set_titles_string: &'a str) -> Self {
        self.set_titles_string = Some(set_titles_string);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn silence_action(mut self, silence_action: Action) -> Self {
        self.silence_action = Some(silence_action);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }

    #[cfg(feature = "tmux_2_9")]
    pub fn status_format(mut self, status_format: Vec<&'a str>) -> Self {
        self.status_format = Some(status_format);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_interval(mut self, status_interval: usize) -> Self {
        self.status_interval = Some(status_interval);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_justify(mut self, status_justify: StatusJustify) -> Self {
        self.status_justify = Some(status_justify);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_keys(mut self, status_keys: StatusKeys) -> Self {
        self.status_keys = Some(status_keys);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_left(mut self, status_left: &'a str) -> Self {
        self.status_left = Some(status_left);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_left_length(mut self, status_left_length: usize) -> Self {
        self.status_left_length = Some(status_left_length);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn status_left_style(mut self, status_left_style: &'a str) -> Self {
        self.status_left_style = Some(status_left_style);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn status_position(mut self, status_position: StatusPosition) -> Self {
        self.status_position = Some(status_position);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn status_right(mut self, status_right: &'a str) -> Self {
        self.status_right = Some(status_right);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn status_right_length(mut self, status_right_length: usize) -> Self {
        self.status_right_length = Some(status_right_length);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn status_right_style(mut self, status_right_style: &'a str) -> Self {
        self.status_right_style = Some(status_right_style);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn status_style(mut self, status_style: &'a str) -> Self {
        self.status_style = Some(status_style);
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub fn status_utf8(mut self, status_utf8: Switch) -> Self {
        self.status_utf8 = Some(status_utf8);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn update_environment(mut self, update_environment: Vec<&'a str>) -> Self {
        self.update_environment = Some(update_environment);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn visual_activity(mut self, visual_activity: Activity) -> Self {
        self.visual_activity = Some(visual_activity);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn visual_bell(mut self, visual_bell: Activity) -> Self {
        self.visual_bell = Some(visual_bell);
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn visual_silence(mut self, visual_silence: Activity) -> Self {
        self.visual_silence = Some(visual_silence);
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn word_separators<S: Into<Cow<'a, str>>>(mut self, word_separators: S) -> Self {
        self.word_separators = Some(word_separators.into());
        self
    }
}
