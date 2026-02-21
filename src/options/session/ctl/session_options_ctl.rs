use crate::options::{GetSessionOptionTr, SessionOptions, SetSessionOptionTr, SetSessionOptionsTr};
#[cfg(feature = "tmux_3_4")]
use crate::MessageLine;
use crate::{
    Action, Activity, DestroyUnattached, DetachOnDestroy, Error, Status, StatusJustify, StatusKeys,
    StatusPosition, Switch, TmuxCommand, TmuxOutput,
};
use std::borrow::Cow;
use std::str::FromStr;

// pub struct SessionOptionsCtl;

// impl SetOptionTr for SessionOptionsCtl<'a>;

// trait, subtrai for global local
pub trait SessionOptionsCtl<'a> {
    type Getter: GetSessionOptionTr;
    type Setter: SetSessionOptionTr;
    type GetterAll: GetSessionOptionTr;
    type SetterMultiple: SetSessionOptionsTr<'a>;

    fn target(&self) -> Option<Cow<'a, str>>;

    fn invoker(&self) -> &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>;

    /// # Examples
    ///
    /// ```
    /// use crate::tmux_interface::{LocalSessionOptionsCtl, SessionOptionsCtl};
    ///
    /// let session_options = LocalSessionOptionsCtl::default().get_all().unwrap();
    ///
    /// ```
    fn get_all(&self) -> Result<SessionOptions<'a>, Error> {
        Self::get_all_ext(self.target(), self.invoker())
    }

    fn get_all_ext(
        target: Option<Cow<'a, str>>,
        invoker: impl FnOnce(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
    ) -> Result<SessionOptions<'a>, Error> {
        let cmd = Self::GetterAll::all(target);
        let output = (invoker)(cmd)?.to_string();
        SessionOptions::from_str(&output)
    }

    fn set_all(&self, session_options: SessionOptions<'a>) -> Result<TmuxOutput, Error> {
        Self::set_all_ext(self.target(), self.invoker(), session_options)
    }

    /// # Examples
    ///
    /// ```
    /// use tmux_interface::{LocalSessionOptionsCtl, SessionOptionsCtl, SessionOptions};
    ///
    /// let session_options = SessionOptions::default();
    /// LocalSessionOptionsCtl::default().set_all(session_options).unwrap();
    /// ```
    fn set_all_ext(
        target: Option<Cow<'a, str>>,
        invoker: impl FnOnce(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
        session_options: SessionOptions<'a>,
    ) -> Result<TmuxOutput, Error> {
        let cmds = Self::SetterMultiple::new();

        #[cfg(feature = "tmux_2_6")]
        let cmds = cmds.activity_action(target.clone(), session_options.activity_action);

        #[cfg(feature = "tmux_1_8")]
        let cmds = cmds.assume_paste_time(target.clone(), session_options.assume_paste_time);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.base_index(target.clone(), session_options.base_index);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.bell_action(target.clone(), session_options.bell_action);

        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
        let cmds = cmds.bell_on_alert(target.clone(), session_options.bell_on_alert);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
        let cmds = cmds.buffer_limit(target.clone(), session_options.buffer_limit);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.default_command(target.clone(), session_options.default_command);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.default_shell(target.clone(), session_options.default_shell);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.default_path(target.clone(), session_options.default_path);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
        let cmds = cmds.default_terminal(target.clone(), session_options.default_terminal);

        #[cfg(feature = "tmux_2_9")]
        let cmds = cmds.default_size(target.clone(), session_options.default_size);

        #[cfg(feature = "tmux_1_5")]
        let cmds = cmds.destroy_unattached(target.clone(), session_options.destroy_unattached);

        #[cfg(feature = "tmux_1_4")]
        let cmds = cmds.detach_on_destroy(target.clone(), session_options.detach_on_destroy);

        #[cfg(feature = "tmux_1_2")]
        let cmds = cmds.display_panes_active_colour(
            target.clone(),
            session_options.display_panes_active_colour,
        );

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.display_panes_colour(target.clone(), session_options.display_panes_colour);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.display_panes_time(target.clone(), session_options.display_panes_time);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.display_time(target.clone(), session_options.display_time);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.history_limit(target.clone(), session_options.history_limit);

        #[cfg(feature = "tmux_2_2")]
        let cmds = cmds.key_table(target.clone(), session_options.key_table);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.lock_after_time(target.clone(), session_options.lock_after_time);

        #[cfg(feature = "tmux_1_1")]
        let cmds = cmds.lock_command(target.clone(), session_options.lock_command);

        #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
        let cmds = cmds.lock_server(target.clone(), session_options.lock_server);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_attr(target.clone(), session_options.message_attr);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_bg(target.clone(), session_options.message_bg);

        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_command_attr(target.clone(), session_options.message_command_attr);

        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_command_bg(target.clone(), session_options.message_command_bg);

        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_command_fg(target.clone(), session_options.message_command_fg);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_fg(target.clone(), session_options.message_fg);

        #[cfg(feature = "tmux_1_9")]
        let cmds =
            cmds.message_command_style(target.clone(), session_options.message_command_style);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
        let cmds = cmds.message_limit(target.clone(), session_options.message_limit);

        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.message_style(target.clone(), session_options.message_style);

        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        let cmds = cmds.mouse_resize_pane(target.clone(), session_options.mouse_resize_pane);

        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        let cmds = cmds.mouse_select_pane(target.clone(), session_options.mouse_select_pane);

        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        let cmds = cmds.mouse_select_window(target.clone(), session_options.mouse_select_window);

        #[cfg(feature = "tmux_2_1")]
        let cmds = cmds.mouse(target.clone(), session_options.mouse);

        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
        let cmds = cmds.mouse_utf8(target.clone(), session_options.mouse_utf8);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let cmds =
            cmds.pane_active_border_bg(target.clone(), session_options.pane_active_border_bg);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let cmds =
            cmds.pane_active_border_fg(target.clone(), session_options.pane_active_border_fg);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let cmds = cmds.pane_border_bg(target.clone(), session_options.pane_border_bg);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let cmds = cmds.pane_border_fg(target.clone(), session_options.pane_border_fg);

        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        let cmds =
            cmds.pane_active_border_style(target.clone(), session_options.pane_active_border_style);

        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        let cmds = cmds.pane_border_style(target.clone(), session_options.pane_border_style);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.prefix(target.clone(), session_options.prefix);

        #[cfg(feature = "tmux_1_6")]
        let cmds = cmds.prefix2(target.clone(), session_options.prefix2);

        #[cfg(feature = "tmux_1_7")]
        let cmds = cmds.renumber_windows(target.clone(), session_options.renumber_windows);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.repeat_time(target.clone(), session_options.repeat_time);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
        let cmds = cmds.set_remain_on_exit(target.clone(), session_options.set_remain_on_exit);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.set_titles(target.clone(), session_options.set_titles);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.set_titles_string(target.clone(), session_options.set_titles_string);

        #[cfg(feature = "tmux_2_6")]
        let cmds = cmds.silence_action(target.clone(), session_options.silence_action);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status(target.clone(), session_options.status);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_attr(target.clone(), session_options.status_attr);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_bg(target.clone(), session_options.status_bg);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_fg(target.clone(), session_options.status_fg);

        // #[cfg(feature = "tmux_2_9")]
        // let cmds = cmds.status_format(target.clone(), session_options.status_format);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_interval(target.clone(), session_options.status_interval);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_justify(target.clone(), session_options.status_justify);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_keys(target.clone(), session_options.status_keys);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_left(target.clone(), session_options.status_left);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_left_attr(target.clone(), session_options.status_left_attr);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_left_bg(target.clone(), session_options.status_left_bg);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_left_fg(target.clone(), session_options.status_left_fg);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_left_length(target.clone(), session_options.status_left_length);

        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.status_left_style(target.clone(), session_options.status_left_style);

        #[cfg(feature = "tmux_1_7")]
        let cmds = cmds.status_position(target.clone(), session_options.status_position);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_right(target.clone(), session_options.status_right);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_right_attr(target.clone(), session_options.status_right_attr);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_right_bg(target.clone(), session_options.status_right_bg);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_right_fg(target.clone(), session_options.status_right_fg);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_right_length(target.clone(), session_options.status_right_length);

        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.status_right_style(target.clone(), session_options.status_right_style);

        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.status_style(target.clone(), session_options.status_style);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
        let cmds = cmds.status_utf8(target.clone(), session_options.status_utf8);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        let cmds = cmds.terminal_overrides(target.clone(), session_options.terminal_overrides);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.update_environment(target.clone(), session_options.update_environment);

        #[cfg(all(feature = "tmux_26", not(feature = "tmux_3_0")))]
        let cmds = cmds.user_keys(target.clone(), session_options.user_keys);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.visual_activity(target.clone(), session_options.visual_activity);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.visual_bell(target.clone(), session_options.visual_bell);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        let cmds = cmds.visual_content(target.clone(), session_options.visual_content);

        #[cfg(feature = "tmux_1_4")]
        let cmds = cmds.visual_silence(target.clone(), session_options.visual_silence);

        #[cfg(feature = "tmux_1_6")]
        let cmds = cmds.word_separators(target.clone(), session_options.word_separators);

        // let cmds = cmds.user_options(target.clone(), session_options.user_options);

        let cmd = TmuxCommand::with_cmds(cmds.build());

        invoker(cmd)
    }

    // fn get<T: std::str::FromStr>(&self, cmd: TmuxCommand<'a>) -> Result<Option<T>, Error>;

    // get and parse single line option
    fn get<T: std::str::FromStr>(&self, cmd: TmuxCommand<'a>) -> Result<Option<T>, Error> {
        let output = (self.invoker())(cmd)?.to_string();
        let value = if output.is_empty() {
            None
        } else {
            output.trim().parse::<T>().ok()
        };
        Ok(value)
        // Ok((self.invoker)(cmd)?.to_string().trim().parse::<T>().ok())
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
    fn get_array(&self, get_option_cmd: TmuxCommand<'a>) -> Result<Option<Vec<String>>, Error> {
        let output = (self.invoker())(get_option_cmd)?;
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
    // fn set(&self, cmd: TmuxCommand<'a>) -> Result<TmuxOutput, Error>;
    fn set(&self, cmd: TmuxCommand<'a>) -> Result<TmuxOutput, Error> {
        (self.invoker())(cmd)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// activity-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn get_activity_action(&self) -> Result<Option<Action>, Error> {
        self.get(Self::Getter::activity_action(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// activity-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn set_activity_action(&self, activity_action: Option<Action>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::activity_action(
            self.target(),
            activity_action,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.8:
    /// ```text
    /// assume-paste-time milliseconds
    /// ```
    #[cfg(feature = "tmux_1_8")]
    fn get_assume_paste_time(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::assume_paste_time(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.8:
    /// ```text
    /// assume-paste-time milliseconds
    /// ```
    #[cfg(feature = "tmux_1_8")]
    fn set_assume_paste_time(&self, assume_paste_time: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::assume_paste_time(
            self.target(),
            assume_paste_time,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// base-index index
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_base_index(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::base_index(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// base-index index
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_base_index(&self, base_index: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::base_index(self.target(), base_index))
    }

    /// ### Manual
    ///
    /// ```text
    /// bell-action [any | none | current | other]
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// bell-action [any | none | other]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_bell_action(&self) -> Result<Option<Action>, Error> {
        self.get(Self::Getter::bell_action(self.target()))
    }

    /// ### Manual
    ///
    /// ```text
    /// bell-action [any | none | current | other]
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// bell-action [any | none | other]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_bell_action(&self, bell_action: Option<Action>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::bell_action(self.target(), bell_action))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.6:
    /// ```text
    /// bell-on-alert [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    fn get_bell_on_alert(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::bell_on_alert(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.6:
    /// ```text
    /// bell-on-alert [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    fn set_bell_on_alert(&self, bell_on_alert: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::bell_on_alert(self.target(), bell_on_alert))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.4:
    /// ```text
    /// buffer-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    fn get_buffer_limit(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::buffer_limit(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.4:
    /// ```text
    /// buffer-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    fn set_buffer_limit(&self, buffer_limit: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::buffer_limit(self.target(), buffer_limit))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_default_command(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::default_command(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_default_command(&self, default_command: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_command(
            self.target(),
            default_command,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-shell path
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_default_shell(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::default_shell(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-shell path
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_default_shell(&self, default_shell: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_shell(self.target(), default_shell))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// default-path path
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_default_path(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::default_path(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// default-path path
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_default_path(&self, default_path: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_path(self.target(), default_path))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    fn get_default_terminal(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::default_terminal(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    fn set_default_terminal(&self, default_terminal: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_terminal(
            self.target(),
            default_terminal,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// default-size XxY
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn get_default_size(&self) -> Result<Option<(usize, usize)>, Error> {
        unimplemented!()
        //self.get(Self::Getter::default_size(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// default-size XxY
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn set_default_size(&self, default_size: Option<(usize, usize)>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_size(self.target(), default_size))
    }

    /// ### Manual
    ///
    /// tmux ^3.4:
    /// ```text
    /// destroy-unattached [on | off | keep-last | keep-group]
    /// ```
    ///
    /// tmux ^1.5:
    /// ```text
    /// destroy-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_5")]
    fn get_destroy_unattached(&self) -> Result<Option<DestroyUnattached>, Error> {
        self.get(Self::Getter::destroy_unattached(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^3.4:
    /// ```text
    /// destroy-unattached [on | off | keep-last | keep-group]
    /// ```
    ///
    /// tmux ^1.5:
    /// ```text
    /// destroy-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_5")]
    fn set_destroy_unattached(
        &self,
        destroy_unattached: Option<DestroyUnattached>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::destroy_unattached(
            self.target(),
            destroy_unattached,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// detach-on-destroy [on | off | no-detached]
    /// ```
    ///
    /// tmux ^1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn get_detach_on_destroy(&self) -> Result<Option<DetachOnDestroy>, Error> {
        self.get(Self::Getter::detach_on_destroy(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// detach-on-destroy [on | off | no-detached]
    /// ```
    ///
    /// tmux ^1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn set_detach_on_destroy(
        &self,

        detach_on_destroy: Option<DetachOnDestroy>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::detach_on_destroy(
            self.target(),
            detach_on_destroy,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// display-panes-active-colour colour
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn get_display_panes_active_colour(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::display_panes_active_colour(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// display-panes-active-colour colour
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn set_display_panes_active_colour(
        &self,

        display_panes_active_colour: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::display_panes_active_colour(
            self.target(),
            display_panes_active_colour,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_display_panes_colour(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::display_panes_colour(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_display_panes_colour(
        &self,

        display_panes_colour: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::display_panes_colour(
            self.target(),
            display_panes_colour,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_display_panes_time(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::display_panes_time(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_display_panes_time(
        &self,

        display_panes_time: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::display_panes_time(
            self.target(),
            display_panes_time,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_display_time(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::display_time(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_display_time(&self, display_time: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::display_time(self.target(), display_time))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// history-limit lines
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_history_limit(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::history_limit(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// history-limit lines
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_history_limit(&self, history_limit: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::history_limit(self.target(), history_limit))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// key-table key-table
    /// ```
    #[cfg(feature = "tmux_2_2")]
    fn get_key_table(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::key_table(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// key-table key-table
    /// ```
    #[cfg(feature = "tmux_2_2")]
    fn set_key_table(&self, key_table: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::key_table(self.target(), key_table))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// lock-after-time number
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_lock_after_time(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::lock_after_time(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// lock-after-time number
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_lock_after_time(&self, lock_after_time: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::lock_after_time(
            self.target(),
            lock_after_time,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_lock_command(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::lock_command(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_lock_command(&self, lock_command: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::lock_command(self.target(), lock_command))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-server [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    fn get_lock_server(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::lock_server(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-server [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    fn set_lock_server(&self, lock_server: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::lock_server(self.target(), lock_server))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_message_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_attr(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_message_attr(&self, message_attr: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_attr(self.target(), message_attr))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_message_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_bg(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_message_bg(&self, message_bg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_bg(self.target(), message_bg))
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_message_command_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_command_attr(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn set_message_command_attr(
        &self,
        message_command_attr: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_command_attr(
            self.target(),
            message_command_attr,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_message_command_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_command_bg(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn set_message_command_bg(
        &self,
        message_command_bg: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_command_bg(
            self.target(),
            message_command_bg,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_message_command_fg(mut self) -> Result<Option<String>, Error>
    where
        Self: Sized,
    {
        self.get(Self::Getter::message_command_fg(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn set_message_command_fg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        message_command_fg: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_command_fg(
            self.target(),
            message_command_fg,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_message_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_fg(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_message_fg(&self, message_fg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_fg(self.target(), message_fg))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-command-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn get_message_command_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_command_style(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-command-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn set_message_command_style(
        &self,

        message_command_style: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_command_style(
            self.target(),
            message_command_style,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^3.4:
    /// ```text
    /// message-line [0 | 1 | 2 | 3 | 4]
    /// ```
    #[cfg(feature = "tmux_3_4")]
    fn get_message_line(&self) -> Result<Option<MessageLine>, Error> {
        self.get(Self::Getter::message_line(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^3.4:
    /// ```text
    /// message-line [0 | 1 | 2 | 3 | 4]
    /// ```
    #[cfg(feature = "tmux_3_4")]
    fn set_message_line(&self, message_line: Option<MessageLine>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_line(self.target(), message_line))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn get_message_limit(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::message_limit(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn set_message_limit(&self, message_limit: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_limit(self.target(), message_limit))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn get_message_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_style(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn set_message_style(&self, message_style: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_style(self.target(), message_style))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-resize-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn get_mouse_resize_pane(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::mouse_resize_pane(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-resize-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn set_mouse_resize_pane(
        &self,

        mouse_resize_pane: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mouse_resize_pane(
            self.target(),
            mouse_resize_pane,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn get_mouse_select_pane(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::mouse_select_pane(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn set_mouse_select_pane(&self, default_size: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mouse_select_pane(self.target(), default_size))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-window [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn get_select_window(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::mouse_select_window(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-window [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn set_select_window(&self, select_window: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mouse_select_window(
            self.target(),
            select_window,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// mouse [on | off]
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn get_mouse(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::mouse(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// mouse [on | off]
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn set_mouse(&self, mouse: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mouse(self.target(), mouse))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.2:
    /// ```text
    /// mouse-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    fn get_mouse_utf8(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::mouse_utf8(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.2:
    /// ```text
    /// mouse-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    fn set_mouse_utf8(&self, mouse_utf8: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mouse_utf8(self.target(), mouse_utf8))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn get_pane_active_border_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_active_border_bg(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn set_pane_active_border_bg(
        &self,
        pane_active_border_bg: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_active_border_bg(
            self.target(),
            pane_active_border_bg,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn get_pane_active_border_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_active_border_fg(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn set_pane_active_border_fg(
        &self,
        pane_active_border_fg: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_active_border_fg(
            self.target(),
            pane_active_border_fg,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn get_pane_border_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_border_bg(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn set_pane_border_bg(&self, pane_border_bg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_border_bg(self.target(), pane_border_bg))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn get_pane_border_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_border_fg(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn set_pane_border_fg(&self, pane_border_fg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_border_fg(self.target(), pane_border_fg))
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn get_pane_active_border_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_active_border_style(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn set_pane_active_border_style(
        &self,
        pane_active_border_style: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_active_border_style(
            self.target(),
            pane_active_border_style,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn get_pane_border_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_border_style(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn set_pane_border_style(
        &self,
        pane_border_style: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_border_style(
            self.target(),
            pane_border_style,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// prefix key
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_prefix(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::prefix(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// prefix key
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_prefix(&self, prefix: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::prefix(self.target(), prefix))
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// prefix2 key
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn get_prefix2(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::prefix2(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// prefix2 key
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn set_prefix2(&self, prefix2: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::prefix2(self.target(), prefix2))
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// renumber-windows [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn get_renumber_windows(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::renumber_windows(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// renumber-windows [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn set_renumber_windows(&self, renumber_windows: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::renumber_windows(
            self.target(),
            renumber_windows,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// repeat-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_repeat_time(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::repeat_time(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// repeat-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_repeat_time(&self, repeat_time: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::repeat_time(self.target(), repeat_time))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.4:
    /// ```text
    /// set-remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    fn get_set_remain_on_exit(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::set_remain_on_exit(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.4:
    /// ```text
    /// set-remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    fn set_set_remain_on_exit(
        &self,

        set_remain_on_exit: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::set_remain_on_exit(
            self.target(),
            set_remain_on_exit,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_set_titles(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::set_titles(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_set_titles(&self, set_titles: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::set_titles(self.target(), set_titles))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles-string string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_set_titles_string(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::set_titles_string(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles-string string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_set_titles_string(
        &self,

        set_titles_string: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::set_titles_string(
            self.target(),
            set_titles_string,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// silence-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn get_silence_action(&self) -> Result<Option<Action>, Error> {
        self.get(Self::Getter::silence_action(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// silence-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn set_silence_action(&self, silence_action: Option<Action>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::silence_action(self.target(), silence_action))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// status [off | on | 2 | 3 | 4 | 5]
    /// ```
    /// tmux 1.0:
    /// ```text
    /// status [off | on]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status(&self) -> Result<Option<Status>, Error> {
        self.get(Self::Getter::status(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// status [off | on | 2 | 3 | 4 | 5]
    /// ```
    /// tmux 1.0:
    /// ```text
    /// status [off | on]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status(&self, status: Option<Status>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status(self.target(), status))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_attr(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_attr(&self, status_attr: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_attr(self.target(), status_attr))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_bg(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_bg(&self, status_bg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_bg(self.target(), status_bg))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_fg(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_fg(&self, status_fg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_fg(self.target(), status_fg))
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// status-format[] format
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn get_status_format(&self) -> Result<Option<Vec<String>>, Error> {
        self.get_array(Self::Getter::status_format(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// status-format[] format
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn set_status_format<S, I>(&self, status_format: Option<I>) -> Result<TmuxOutput, Error>
    where
        S: Into<Cow<'a, str>>,
        I: IntoIterator<Item = S>,
    {
        self.set(TmuxCommand::with_cmds(Self::Setter::status_format(
            self.target(),
            status_format,
        )))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-interval interval
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_interval(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::status_interval(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-interval interval
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_interval(&self, status_interval: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_interval(
            self.target(),
            status_interval,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-justify [left | centre | right]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_justify(&self) -> Result<Option<StatusJustify>, Error> {
        self.get(Self::Getter::status_justify(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-justify [left | centre | right]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_justify(
        &self,

        status_justify: Option<StatusJustify>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_justify(self.target(), status_justify))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_keys(&self) -> Result<Option<StatusKeys>, Error> {
        self.get(Self::Getter::status_keys(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_keys(&self, status_keys: Option<StatusKeys>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_keys(self.target(), status_keys))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_left(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_left(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_left(&self, status_left: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left(self.target(), status_left))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_left_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_left_attr(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_left_attr(&self, status_left_attr: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left_attr(
            self.target(),
            status_left_attr,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_left_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_left_bg(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_left_bg(&self, status_left_bg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left_bg(self.target(), status_left_bg))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_left_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_left_fg(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_left_fg(&self, status_left_fg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left_fg(self.target(), status_left_fg))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_left_length(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::status_left_length(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_left_length(
        &self,

        status_left_length: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left_length(
            self.target(),
            status_left_length,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-left-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn get_status_left_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_left_style(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-left-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn set_status_left_style(
        &self,

        status_left_style: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left_style(
            self.target(),
            status_left_style,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// status-position [top | bottom]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn get_status_position(&self) -> Result<Option<StatusPosition>, Error> {
        self.get(Self::Getter::status_position(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// status-position [top | bottom]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn set_status_position(
        &self,

        status_position: Option<StatusPosition>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_position(
            self.target(),
            status_position,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_right(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_right(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_right(&self, status_right: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_right(self.target(), status_right))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_right_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_right_attr(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_right_attr(
        &self,
        status_right_attr: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_right_attr(
            self.target(),
            status_right_attr,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_right_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_right_fg(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_right_fg(&self, status_right_fg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_right_fg(
            self.target(),
            status_right_fg,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_right_length(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::status_right_length(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_right_length(
        &self,

        status_right_length: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_right_length(
            self.target(),
            status_right_length,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-right-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn get_status_right_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_right_style(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-right-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn set_status_right_style(
        &self,

        status_right_style: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_right_style(
            self.target(),
            status_right_style,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn get_status_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_style(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn set_status_style(&self, status_style: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_style(self.target(), status_style))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// status-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn get_status_utf8(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::status_utf8(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// status-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn set_status_utf8(&self, status_utf8: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_utf8(self.target(), status_utf8))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// terminal-overrides string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn get_terminal_overrides(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::terminal_overrides(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// terminal-overrides string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn set_terminal_overrides(
        &self,
        terminal_overrides: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::terminal_overrides(
            self.target(),
            terminal_overrides,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// update-environment[] variable
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_update_environment(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::update_environment(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// update-environment[] variable
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_update_environment<S, I>(
        &self,
        update_environment: Option<I>,
    ) -> Result<TmuxOutput, Error>
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        self.set(TmuxCommand::with_cmds(Self::Setter::update_environment(
            self.target(),
            update_environment,
        )))
    }

    /// ### Manual
    ///
    /// tmux ^2.6 v3.0:
    /// ```text
    /// user-keys
    /// ```
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    fn get_user_keys(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::user_keys(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6 v3.0:
    /// ```text
    /// user-keys[]
    /// ```
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    fn set_user_keys<S: Into<Cow<'a, str>> + Clone>(
        &self,
        user_keys: Option<Vec<String>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(TmuxCommand::with_cmds(Self::Setter::user_keys(
            self.target(),
            user_keys,
        )))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// visual-activity [on | off | both]
    /// ```
    ///
    /// tmux 1.0:
    /// ```text
    /// visual-activity [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_visual_activity(&self) -> Result<Option<Activity>, Error> {
        self.get(Self::Getter::visual_activity(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// visual-activity [on | off | both]
    /// ```
    ///
    /// tmux 1.0:
    /// ```text
    /// visual-activity [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_visual_activity(&self, visual_activity: Option<Activity>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::visual_activity(
            self.target(),
            visual_activity,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// visual-bell [on | off | both]
    /// ```
    ///
    /// tmux 1.0:
    /// ```text
    /// visual-bell [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_visual_bell(&self) -> Result<Option<Activity>, Error> {
        self.get(Self::Getter::visual_bell(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// visual-bell [on | off | both]
    /// ```
    ///
    /// tmux 1.0:
    /// ```text
    /// visual-bell [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_visual_bell(&self, visual_bell: Option<Activity>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::visual_bell(self.target(), visual_bell))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// visual-content [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn get_visual_content(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::visual_content(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// visual-content [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn set_visual_content(&self, visual_content: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::visual_content(self.target(), visual_content))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// visual-silence [on | off | both]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn get_visual_silence(&self) -> Result<Option<Activity>, Error> {
        self.get(Self::Getter::visual_silence(self.target()))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// visual-silence [on | off | both]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn set_visual_silence(&self, visual_silence: Option<Activity>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::visual_silence(self.target(), visual_silence))
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn get_word_separators(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::word_separators(self.target()))
    }
    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn set_word_separators(&self, word_separators: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::word_separators(
            self.target(),
            word_separators,
        ))
    }
}
