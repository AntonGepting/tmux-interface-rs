use crate::options::{
    GetLocalSessionOptionValue, SessionOptions, SessionOptionsCtl, SetLocalSessionOption,
    SetLocalSessionOptions, SetSessionOptions,
};
use crate::{Error, ShowOptions, Tmux, TmuxCommand, TmuxOutput};
use std::str::FromStr;

// XXX: rename SessionOptionCtl?
// trait top level options, then server session window pane
pub struct LocalSessionOptionsCtl<'a> {
    // TODO: comment/doc
    //
    // function used for executing the given option get/set command
    //
    // ```
    // let tmux = Tmux::new();
    // ```
    pub invoker: fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
}

impl<'a> Default for LocalSessionOptionsCtl<'a> {
    fn default() -> Self {
        Self {
            invoker: |cmd| Tmux::with_command(cmd).output(),
        }
    }
}

impl<'a> LocalSessionOptionsCtl<'a> {
    pub fn new(invoker: fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
        Self { invoker }
    }

    pub fn get_global_all(&self) -> Result<SessionOptions<'a>, Error> {
        let cmd = ShowOptions::new().global().build();
        let output = (self.invoker)(cmd)?.to_string();
        dbg!(&output);
        SessionOptions::from_str(&output)
    }

    pub fn get_local_all(&self) -> Result<SessionOptions<'a>, Error> {
        let cmd = ShowOptions::new().build();
        let output = (self.invoker)(cmd)?.to_string();
        dbg!(&output);
        SessionOptions::from_str(&output)
    }

    // get and parse single line option
    pub fn get<T: std::str::FromStr>(&self, cmd: TmuxCommand<'a>) -> Result<Option<T>, Error> {
        Ok((self.invoker)(cmd)?.to_string().trim().parse::<T>().ok())
    }

    pub fn set(&self, cmd: TmuxCommand<'a>) -> Result<TmuxOutput, Error> {
        (self.invoker)(cmd)
    }

    // FIXME: full array support
    // Tmux binary
    //
    // 1. multiple binary call
    // tmux set -s command-alias[0] value0
    // tmux set -s command-alias[1] value1
    // tmux set -s command-alias[2] value2
    //
    // 2. single binary call
    // tmux set -s command-alias[0] value0 ; set -s command-alias[1] ; set -s command-alias[2]
    //
    // Control Mode
    //
    // 1. multiple control mode commands
    // set -s command-alias[0] value0
    // set -s command-alias[1] value1
    // set -s command-alias[2] value2
    //
    // 2. single control mode command
    // set -s command-alias[0] value0 ; set -s command-alias[1] ; set -s command-alias[2]
    //
    pub fn get_array(&self, get_option_cmd: TmuxCommand<'a>) -> Result<Option<Vec<String>>, Error> {
        let output = (self.invoker)(get_option_cmd)?;
        let v: Vec<String> = output
            .to_string()
            .lines()
            .map(|s| s.trim().into())
            .collect();
        let result = match v.is_empty() {
            true => None,
            false => Some(v),
        };
        Ok(result)
    }
}

// XXX: mb no need for Local, Global only custom implementation?
// XXX: mb better name instead of Local (not a network meaning)
impl<'a> SessionOptionsCtl<'a> for LocalSessionOptionsCtl<'a> {
    type Getter = GetLocalSessionOptionValue;
    type Setter = SetLocalSessionOption;

    /// # Examples
    ///
    /// ```
    /// use tmux_interface::{LocalSessionOptionsCtl, SessionOptionsCtl};
    ///
    /// let session_options = LocalSessionOptionsCtl::default().get_all().unwrap();
    /// ```
    fn get_all(&self) -> Result<SessionOptions<'a>, Error> {
        let cmd = ShowOptions::new().build();
        let output = (self.invoker)(cmd)?.to_string();
        dbg!(&output);
        SessionOptions::from_str(&output)
    }

    /// # Examples
    ///
    /// ```
    /// use tmux_interface::{LocalSessionOptionsCtl, SessionOptionsCtl, SessionOptions};
    ///
    /// let session_options = SessionOptions::default();
    /// LocalSessionOptionsCtl::default().set_all(session_options).unwrap();
    /// ```
    fn set_all(self, session_options: SessionOptions<'a>) -> Result<TmuxOutput, Error> {
        let cmds = SetLocalSessionOptions::new();

        #[cfg(feature = "tmux_2_6")]
        let cmds = cmds.activity_action(session_options.activity_action);

        #[cfg(feature = "tmux_1_8")]
        let cmds = cmds.assume_paste_time(session_options.assume_paste_time);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.base_index(session_options.base_index);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.bell_action(session_options.bell_action);

        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
        let cmds = cmds.bell_on_alert(session_options.bell_on_alert);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
        let cmds = cmds.buffer_limit(session_options.buffer_limit);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.default_command(session_options.default_command);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.default_shell(session_options.default_shell);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.default_path(session_options.default_path);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
        let cmds = cmds.default_terminal(session_options.default_terminal);

        #[cfg(feature = "tmux_2_9")]
        let cmds = cmds.default_size(session_options.default_size);

        #[cfg(feature = "tmux_1_4")]
        let cmds = cmds.destroy_unattached(session_options.destroy_unattached);

        #[cfg(feature = "tmux_1_4")]
        let cmds = cmds.detach_on_destroy(session_options.detach_on_destroy);

        #[cfg(feature = "tmux_1_2")]
        let cmds = cmds.display_panes_active_colour(session_options.display_panes_active_colour);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.display_panes_colour(session_options.display_panes_colour);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.display_panes_time(session_options.display_panes_time);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.display_time(session_options.display_time);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.history_limit(session_options.history_limit);

        #[cfg(feature = "tmux_2_2")]
        let cmds = cmds.key_table(session_options.key_table);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.lock_after_time(session_options.lock_after_time);

        #[cfg(feature = "tmux_1_1")]
        let cmds = cmds.lock_command(session_options.lock_command);

        #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
        let cmds = cmds.lock_server(session_options.lock_server);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_attr(session_options.message_attr);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_bg(session_options.message_bg);

        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_command_attr(session_options.message_command_attr);

        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_command_bg(session_options.message_command_bg);

        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_command_bg(session_options.message_command_bg);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_fg(session_options.message_fg);

        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.message_command_style(session_options.message_command_style);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
        let cmds = cmds.message_limit(session_options.message_limit);

        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.message_style(session_options.message_style);

        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        let cmds = cmds.mouse_resize_pane(session_options.mouse_resize_pane);

        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        let cmds = cmds.mouse_select_pane(session_options.mouse_select_pane);

        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        let cmds = cmds.mouse_select_window(session_options.mouse_select_window);

        #[cfg(feature = "tmux_2_1")]
        let cmds = cmds.mouse(session_options.mouse);

        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
        let cmds = cmds.mouse_utf8(session_options.mouse_utf8);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let cmds = cmds.pane_active_border_bg(session_options.pane_active_border_bg);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let cmds = cmds.pane_active_border_fg(session_options.pane_active_border_fg);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let cmds = cmds.pane_border_bg(session_options.pane_border_bg);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let cmds = cmds.pane_border_fg(session_options.pane_border_fg);

        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        let cmds = cmds.pane_active_border_style(session_options.pane_active_border_style);

        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        let cmds = cmds.pane_border_style(session_options.pane_border_style);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.prefix(session_options.prefix);

        #[cfg(feature = "tmux_1_6")]
        let cmds = cmds.prefix2(session_options.prefix2);

        #[cfg(feature = "tmux_1_7")]
        let cmds = cmds.renumber_windows(session_options.renumber_windows);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.repeat_time(session_options.repeat_time);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
        let cmds = cmds.set_remain_on_exit(session_options.set_remain_on_exit);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.set_titles(session_options.set_titles);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.set_titles_string(session_options.set_titles_string);

        #[cfg(feature = "tmux_2_6")]
        let cmds = cmds.silence_action(session_options.silence_action);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status(session_options.status);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_attr(session_options.status_attr);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_bg(session_options.status_bg);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_fg(session_options.status_fg);

        #[cfg(feature = "tmux_2_9")]
        let cmds = cmds.status_format(session_options.status_format);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_interval(session_options.status_interval);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_justify(session_options.status_justify);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_keys(session_options.status_keys);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_left(session_options.status_left);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_left_attr(session_options.status_left_attr);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_left_bg(session_options.status_left_bg);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_left_fg(session_options.status_left_fg);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_left_length(session_options.status_left_length);

        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.status_left_style(session_options.status_left_style);

        #[cfg(feature = "tmux_1_7")]
        let cmds = cmds.status_position(session_options.status_position);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_right(session_options.status_right);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_right_attr(session_options.status_right_attr);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_right_bg(session_options.status_right_bg);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_right_fg(session_options.status_right_fg);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_right_length(session_options.status_right_length);

        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.status_right_style(session_options.status_right_style);

        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.status_style(session_options.status_style);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
        let cmds = cmds.status_utf8(session_options.status_utf8);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        let cmds = cmds.terminal_overrides(session_options.terminal_overrides);

        // #[cfg(feature = "tmux_1_0")]
        // let cmds = cmds.update_environment(session_options.update_environment);

        #[cfg(all(feature = "tmux_26", not(feature = "tmux_3_0")))]
        let cmds = cmds.user_keys(session_options.user_keys);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.visual_activity(session_options.visual_activity);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.visual_bell(session_options.visual_bell);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        let cmds = cmds.visual_content(session_options.visual_content);

        #[cfg(feature = "tmux_1_4")]
        let cmds = cmds.visual_silence(session_options.visual_silence);

        #[cfg(feature = "tmux_1_6")]
        let cmds = cmds.word_separators(session_options.word_separators);

        // let cmds = cmds.user_options(session_options.user_options);

        let cmd = TmuxCommand::with_cmds(cmds.build());

        (self.invoker)(cmd)
    }

    fn get<T: std::str::FromStr>(&self, cmd: TmuxCommand<'a>) -> Result<Option<T>, Error> {
        Ok((self.invoker)(cmd)?.to_string().trim().parse::<T>().ok())
    }

    fn set(&self, cmd: TmuxCommand<'a>) -> Result<TmuxOutput, Error> {
        (self.invoker)(cmd)
    }
}
