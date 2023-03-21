use crate::options::{GetSessionOption, SessionOptions, SetSessionOption, SetSessionOptions};
use crate::{
    Action, Activity, DetachOnDestroy, Error, Status, StatusJustify, StatusKeys, StatusPosition,
    Switch, TmuxCommand, TmuxCommands, TmuxOutput,
};
use std::borrow::Cow;

// pub struct SessionOptionsCtl;

// impl SetOptionExt for SessionOptionsCtl<'a>;

// trait, subtrai for global local
pub trait SessionOptionsCtl<'a> {
    type Getter: GetSessionOption;
    type Setter: SetSessionOption;
    type SetterMultiple: SetSessionOptions<'a>;

    fn target(&self) -> Option<Cow<'a, str>>;

    fn get_all(&self) -> Result<SessionOptions<'a>, Error>;

    /// # Examples
    ///
    /// ```
    /// use tmux_interface::{LocalSessionOptionsCtl, SessionOptionsCtl, SessionOptions};
    ///
    /// let session_options = SessionOptions::default();
    /// LocalSessionOptionsCtl::default().set_all(session_options).unwrap();
    /// ```
    fn set_all_ext(
        &self,
        invoke: &dyn Fn(&TmuxCommands<'a>) -> Result<TmuxOutput, Error>,
        session_options: SessionOptions<'a>,
    ) -> Result<TmuxOutput, Error> {
        let cmds = Self::SetterMultiple::new();

        #[cfg(feature = "tmux_2_6")]
        let cmds = cmds.activity_action(self.target(), session_options.activity_action);

        #[cfg(feature = "tmux_1_8")]
        let cmds = cmds.assume_paste_time(self.target(), session_options.assume_paste_time);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.base_index(self.target(), session_options.base_index);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.bell_action(self.target(), session_options.bell_action);

        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
        let cmds = cmds.bell_on_alert(self.target(), session_options.bell_on_alert);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
        let cmds = cmds.buffer_limit(self.target(), session_options.buffer_limit);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.default_command(self.target(), session_options.default_command);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.default_shell(self.target(), session_options.default_shell);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.default_path(self.target(), session_options.default_path);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
        let cmds = cmds.default_terminal(self.target(), session_options.default_terminal);

        #[cfg(feature = "tmux_2_9")]
        let cmds = cmds.default_size(self.target(), session_options.default_size);

        #[cfg(feature = "tmux_1_4")]
        let cmds = cmds.destroy_unattached(self.target(), session_options.destroy_unattached);

        #[cfg(feature = "tmux_1_4")]
        let cmds = cmds.detach_on_destroy(self.target(), session_options.detach_on_destroy);

        #[cfg(feature = "tmux_1_2")]
        let cmds = cmds.display_panes_active_colour(
            self.target(),
            session_options.display_panes_active_colour,
        );

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.display_panes_colour(self.target(), session_options.display_panes_colour);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.display_panes_time(self.target(), session_options.display_panes_time);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.display_time(self.target(), session_options.display_time);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.history_limit(self.target(), session_options.history_limit);

        #[cfg(feature = "tmux_2_2")]
        let cmds = cmds.key_table(self.target(), session_options.key_table);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.lock_after_time(self.target(), session_options.lock_after_time);

        #[cfg(feature = "tmux_1_1")]
        let cmds = cmds.lock_command(self.target(), session_options.lock_command);

        #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
        let cmds = cmds.lock_server(self.target(), session_options.lock_server);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_attr(self.target(), session_options.message_attr);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_bg(self.target(), session_options.message_bg);

        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_command_attr(self.target(), session_options.message_command_attr);

        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_command_bg(self.target(), session_options.message_command_bg);

        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_command_bg(self.target(), session_options.message_command_bg);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.message_fg(self.target(), session_options.message_fg);

        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.message_command_style(self.target(), session_options.message_command_style);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
        let cmds = cmds.message_limit(self.target(), session_options.message_limit);

        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.message_style(self.target(), session_options.message_style);

        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        let cmds = cmds.mouse_resize_pane(self.target(), session_options.mouse_resize_pane);

        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        let cmds = cmds.mouse_select_pane(self.target(), session_options.mouse_select_pane);

        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
        let cmds = cmds.mouse_select_window(self.target(), session_options.mouse_select_window);

        #[cfg(feature = "tmux_2_1")]
        let cmds = cmds.mouse(self.target(), session_options.mouse);

        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
        let cmds = cmds.mouse_utf8(self.target(), session_options.mouse_utf8);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let cmds = cmds.pane_active_border_bg(self.target(), session_options.pane_active_border_bg);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let cmds = cmds.pane_active_border_fg(self.target(), session_options.pane_active_border_fg);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let cmds = cmds.pane_border_bg(self.target(), session_options.pane_border_bg);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
        let cmds = cmds.pane_border_fg(self.target(), session_options.pane_border_fg);

        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        let cmds =
            cmds.pane_active_border_style(self.target(), session_options.pane_active_border_style);

        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        let cmds = cmds.pane_border_style(self.target(), session_options.pane_border_style);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.prefix(self.target(), session_options.prefix);

        #[cfg(feature = "tmux_1_6")]
        let cmds = cmds.prefix2(self.target(), session_options.prefix2);

        #[cfg(feature = "tmux_1_7")]
        let cmds = cmds.renumber_windows(self.target(), session_options.renumber_windows);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.repeat_time(self.target(), session_options.repeat_time);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
        let cmds = cmds.set_remain_on_exit(self.target(), session_options.set_remain_on_exit);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.set_titles(self.target(), session_options.set_titles);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.set_titles_string(self.target(), session_options.set_titles_string);

        #[cfg(feature = "tmux_2_6")]
        let cmds = cmds.silence_action(self.target(), session_options.silence_action);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status(self.target(), session_options.status);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_attr(self.target(), session_options.status_attr);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_bg(self.target(), session_options.status_bg);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_fg(self.target(), session_options.status_fg);

        #[cfg(feature = "tmux_2_9")]
        let cmds = cmds.status_format(self.target(), session_options.status_format);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_interval(self.target(), session_options.status_interval);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_justify(self.target(), session_options.status_justify);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_keys(self.target(), session_options.status_keys);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_left(self.target(), session_options.status_left);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_left_attr(self.target(), session_options.status_left_attr);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_left_bg(self.target(), session_options.status_left_bg);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_left_fg(self.target(), session_options.status_left_fg);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_left_length(self.target(), session_options.status_left_length);

        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.status_left_style(self.target(), session_options.status_left_style);

        #[cfg(feature = "tmux_1_7")]
        let cmds = cmds.status_position(self.target(), session_options.status_position);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_right(self.target(), session_options.status_right);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_right_attr(self.target(), session_options.status_right_attr);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_right_bg(self.target(), session_options.status_right_bg);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.status_right_fg(self.target(), session_options.status_right_fg);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.status_right_length(self.target(), session_options.status_right_length);

        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.status_right_style(self.target(), session_options.status_right_style);

        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.status_style(self.target(), session_options.status_style);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
        let cmds = cmds.status_utf8(self.target(), session_options.status_utf8);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        let cmds = cmds.terminal_overrides(self.target(), session_options.terminal_overrides);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.update_environment(self.target(), session_options.update_environment);

        #[cfg(all(feature = "tmux_26", not(feature = "tmux_3_0")))]
        let cmds = cmds.user_keys(self.target(), session_options.user_keys);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.visual_activity(self.target(), session_options.visual_activity);

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.visual_bell(self.target(), session_options.visual_bell);

        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        let cmds = cmds.visual_content(self.target(), session_options.visual_content);

        #[cfg(feature = "tmux_1_4")]
        let cmds = cmds.visual_silence(self.target(), session_options.visual_silence);

        #[cfg(feature = "tmux_1_6")]
        let cmds = cmds.word_separators(self.target(), session_options.word_separators);

        // let cmds = cmds.user_options(self.target(), session_options.user_options);

        // let cmd = TmuxCommand::with_cmds(cmds.build());

        invoke(&cmds.build())
    }

    fn get<T: std::str::FromStr>(&self, cmd: TmuxCommand<'a>) -> Result<Option<T>, Error>;

    fn set(&self, cmd: TmuxCommand<'a>) -> Result<TmuxOutput, Error>;

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// activity-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn get_activity_action<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Action>, Error> {
        self.get(Self::Getter::activity_action(target))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// activity-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn set_activity_action<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        activity_action: Option<Action>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::activity_action(target, activity_action))
    }

    /// ### Manual
    ///
    /// tmux ^1.8:
    /// ```text
    /// assume-paste-time milliseconds
    /// ```
    #[cfg(feature = "tmux_1_8")]
    fn get_assume_paste_time<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::assume_paste_time(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.8:
    /// ```text
    /// assume-paste-time milliseconds
    /// ```
    #[cfg(feature = "tmux_1_8")]
    fn set_assume_paste_time<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        assume_paste_time: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::assume_paste_time(target, assume_paste_time))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// base-index index
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_base_index<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::base_index(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// base-index index
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_base_index<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        base_index: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::base_index(target, base_index))
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
    fn get_bell_action<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Action>, Error> {
        self.get(Self::Getter::bell_action(target))
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
    fn set_bell_action<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        bell_action: Option<Action>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::bell_action(target, bell_action))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.6:
    /// ```text
    /// bell-on-alert [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    fn get_bell_on_alert<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::bell_on_alert(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.6:
    /// ```text
    /// bell-on-alert [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    fn set_bell_on_alert<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        bell_on_alert: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::bell_on_alert(target, bell_on_alert))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.4:
    /// ```text
    /// buffer-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    fn get_buffer_limit<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::buffer_limit(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.4:
    /// ```text
    /// buffer-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    fn set_buffer_limit<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        buffer_limit: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::buffer_limit(target, buffer_limit))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_default_command<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::default_command(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_default_command<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        default_command: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_command(target, default_command))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-shell path
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_default_shell<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::default_shell(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-shell path
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_default_shell<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        default_shell: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_shell(target, default_shell))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// default-path path
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_default_path<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::default_path(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// default-path path
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_default_path<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        default_path: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_path(target, default_path))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    fn get_default_terminal<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::default_terminal(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    fn set_default_terminal<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        default_terminal: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_terminal(target, default_terminal))
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// default-size XxY
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn get_default_size<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<(usize, usize)>, Error> {
        self.get(Self::Getter::default_size(target))
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// default-size XxY
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn set_default_size<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        default_size: Option<(usize, usize)>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_size(target, default_size))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// destroy-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn get_destroy_unattached<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::destroy_unattached(target))
    }
    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// destroy-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn set_destroy_unattached<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        destroy_unattached: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::destroy_unattached(target, destroy_unattached))
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
    fn get_detach_on_destroy<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<DetachOnDestroy>, Error> {
        self.get(Self::Getter::detach_on_destroy(target))
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
    fn set_detach_on_destroy<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        detach_on_destroy: Option<DetachOnDestroy>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::detach_on_destroy(target, detach_on_destroy))
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// display-panes-active-colour colour
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn get_display_panes_active_colour<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::display_panes_active_colour(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// display-panes-active-colour colour
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn set_display_panes_active_colour<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        display_panes_active_colour: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::display_panes_active_colour(
            target,
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
    fn get_display_panes_colour<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::display_panes_colour(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_display_panes_colour<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        display_panes_colour: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::display_panes_colour(
            target,
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
    fn get_display_panes_time<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::display_panes_time(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_display_panes_time<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        display_panes_time: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::display_panes_time(target, display_panes_time))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_display_time<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::display_time(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_display_time<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        display_time: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::display_time(target, display_time))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// history-limit lines
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_history_limit<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::history_limit(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// history-limit lines
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_history_limit<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        history_limit: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::history_limit(target, history_limit))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// key-table key-table
    /// ```
    #[cfg(feature = "tmux_2_2")]
    fn get_key_table<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::key_table(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// key-table key-table
    /// ```
    #[cfg(feature = "tmux_2_2")]
    fn set_key_table<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        key_table: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::key_table(target, key_table))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// lock-after-time number
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_lock_after_time<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::lock_after_time(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// lock-after-time number
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_lock_after_time<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        lock_after_time: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::lock_after_time(target, lock_after_time))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_lock_command<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::lock_command(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_lock_command<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        lock_command: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::lock_command(target, lock_command))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-server [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    fn get_lock_server<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::lock_server(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-server [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    fn set_lock_server<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        lock_server: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::lock_server(target, lock_server))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_message_attr<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_attr(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_message_attr<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        message_attr: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_attr(target, message_attr))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_message_bg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_bg(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_message_bg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        message_bg: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_bg(target, message_bg))
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_message_command_attr<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_command_attr(target))
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
            target,
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
    fn get_message_command_bg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_command_bg(target))
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
        self.set(Self::Setter::message_command_bg(target, message_command_bg))
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_default_size<S: Into<Cow<'a, str>>>(&self, target: Option<S>) -> Result<Option, Error> {
        self.get(Self::Getter::default_size(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn set_default_size<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        default_size: Option,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_size(target, default_size))
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_fg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::message_command_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_message_fg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_fg(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_message_fg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        message_fg: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_fg(target, message_fg))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-command-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn get_message_command_style<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_command_style(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-command-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn set_message_command_style<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        message_command_style: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_command_style(
            target,
            message_command_style,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn get_mesage_limit<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::mesage_limit(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn set_mesage_limit<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        message_limit: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mesage_limit(target, message_limit))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn get_message_style<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_style(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn set_message_style<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        message_style: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_style(target, message_style))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-resize-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn get_mouse_resize_pane<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::mouse_resize_pane(target))
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
        self.set(Self::Setter::mouse_resize_pane(target, mouse_resize_pane))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn get_mouse_select_pane<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::mouse_select_pane(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn set_mouse_select_pane<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        default_size: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mouse_select_pane(target, default_size))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-window [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn get_select_window<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::select_window(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-window [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn set_select_window<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        select_window: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::select_window(target, select_window))
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// mouse [on | off]
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn get_mouse<S: Into<Cow<'a, str>>>(&self, target: Option<S>) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::mouse(target))
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// mouse [on | off]
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn set_mouse<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        mouse: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mouse(target, mouse))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.2:
    /// ```text
    /// mouse-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    fn get_mouse_utf8<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::mouse_utf8(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.2:
    /// ```text
    /// mouse-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    fn set_mouse_utf8<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        mouse_utf8: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mouse_utf8(target, mouse_utf8))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn get_pane_active_border_bg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_active_border_bg(target))
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
            target,
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
    fn get_pane_active_border_fg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_active_border_fg(target))
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
            target,
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
    fn get_pane_border_bg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_border_bg(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn set_pane_border_bg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        pane_border_bg: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_border_bg(target, pane_border_bg))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn get_pane_border_fg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_border_fg(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn set_pane_border_fg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        pane_border_fg: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_border_fg(target, pane_border_fg))
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn get_pane_active_border_style<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_active_border_style(target))
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
            target,
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
    fn get_pane_border_style<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_border_style(target))
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
        self.set(Self::Setter::pane_border_style(target, pane_border_style))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// prefix key
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_prefix<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::prefix(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// prefix key
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_prefix<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        prefix: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::prefix(target, prefix))
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// prefix2 key
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn get_prefix2<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::prefix2(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// prefix2 key
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn set_prefix2<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        prefix2: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::prefix2(target, prefix2))
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// renumber-windows [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn get_renumber_windows<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::renumber_windows(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// renumber-windows [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn set_renumber_windows<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        renumber_windows: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::renumber_windows(target, renumber_windows))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// repeat-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_repeat_time<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::repeat_time(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// repeat-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_repeat_time<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        repeat_time: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::repeat_time(target, repeat_time))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.4:
    /// ```text
    /// set-remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    fn get_set_remain_on_exit<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::set_remain_on_exit(target))
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
        self.set(Self::Setter::set_remain_on_exit(target, set_remain_on_exit))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_set_titles<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::set_titles(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_set_titles<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        set_titles: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::set_titles(target, set_titles))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles-string string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_set_titles_string<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::set_titles_string(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles-string string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_set_titles_string<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        set_titles_string: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::set_titles_string(target, set_titles_string))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// silence-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn get_silence_action<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Action>, Error> {
        self.get(Self::Getter::silence_action(target))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// silence-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn set_silence_action<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        silence_action: Option<Action>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::silence_action(target, silence_action))
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
    fn get_status<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Status>, Error> {
        self.get(Self::Getter::status(target))
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
    fn set_status<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status: Option<Status>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status(target, status))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_attr<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_attr(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_attr<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_attr: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_attr(target, status_attr))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_bg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_bg(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_bg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_bg: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_bg(target, status_bg))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_fg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_fg(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_fg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_fg: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_fg(target, status_fg))
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// status-format[] format
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn get_status_format<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_format(target))
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// status-format[] format
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn set_status_format<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_format: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_format(target, status_format))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-interval interval
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_interval<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::status_interval(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-interval interval
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_interval<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_interval: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_interval(target, status_interval))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-justify [left | centre | right]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_justify<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<StatusJustify>, Error> {
        self.get(Self::Getter::status_justify(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-justify [left | centre | right]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_justify<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_justify: Option<StatusJustify>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_justify(target, status_justify))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_keys<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<StatusKeys>, Error> {
        self.get(Self::Getter::status_keys(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_keys<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_keys: Option<StatusKeys>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_keys(target, status_keys))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_left<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_left(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_left<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_left: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left(target, status_left))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_left_attr<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_left_attr(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_left_attr<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_left_attr: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left_attr(target, status_left_attr))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_left_bg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_left_bg(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_left_bg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_left_bg: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left_bg(target, status_left_bg))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_left_fg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_left_fg(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_left_fg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_left_fg: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left_fg(target, status_left_fg))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_left_length<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::status_left_length(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_left_length<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_left_length: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left_length(target, status_left_length))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-left-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn get_status_left_style<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_left_style(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-left-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn set_status_left_style<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_left_style: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left_style(target, status_left_style))
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// status-position [top | bottom]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn get_status_position<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<StatusPosition>, Error> {
        self.get(Self::Getter::status_position(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// status-position [top | bottom]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn set_status_position<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_position: Option<StatusPosition>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_position(target, status_position))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_right<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_right(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_right<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_right: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_right(target, status_right))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_right_attr<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_right_attr(target))
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
        self.set(Self::Setter::status_right_attr(target, status_right_attr))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_default_size<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::default_size(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_default_size<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        default_size: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_size(target, default_size))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_right_fg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_right_fg(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_right_fg<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_right_fg: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_right_fg(target, status_right_fg))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_right_length<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::status_right_length(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_right_length<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_right_length: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_right_length(
            target,
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
    fn get_status_right_style<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_right_style(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-right-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn set_status_right_style<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_right_style: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_right_style(target, status_right_style))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn get_status_style<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_style(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn set_status_style<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_style: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_style(target, status_style))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// status-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn get_status_utf8<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::status_utf8(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// status-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn set_status_utf8<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        status_utf8: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_utf8(target, status_utf8))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// terminal-overrides string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn get_terminal_overrides<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::terminal_overrides(target))
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
        self.set(Self::Setter::terminal_overrides(target, terminal_overrides))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// update-environment[] variable
    /// ```
    // #[cfg(feature = "tmux_1_0")]
    // fn get_update_environment<S: Into<Cow<'a, str>>>(&self, target: Option<S>) -> Result<Option<String>, Error> {
    // self.get(Self::Getter::update_environment(target, ))
    // }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// update-environment[] variable
    /// ```
    // #[cfg(feature = "tmux_1_0")]
    // fn set_update_environment<S: Into<Cow<'a, str>>>(&self, target: Option<S>, update_environment: Option<Vec<String>>) -> Result<TmuxOutput, Error> {
    // self.set(Self::Setter::update_environment(target, update_environment))
    // }

    /// ### Manual
    ///
    /// tmux ^2.6 v3.0:
    /// ```text
    /// user-keys
    /// ```
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    fn get_user_keys<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::user_keys(target))
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
        target: Option<S>,
        user_keys: Option<Vec<String>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(TmuxCommand::with_cmds(Self::Setter::user_keys(
            target, user_keys,
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
    fn get_visual_activity<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Activity>, Error> {
        self.get(Self::Getter::visual_activity(target))
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
    fn set_visual_activity<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        visual_activity: Option<Activity>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::visual_activity(target, visual_activity))
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
    fn get_visual_bell<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Activity>, Error> {
        self.get(Self::Getter::visual_bell(target))
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
    fn set_visual_bell<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        visual_bell: Option<Activity>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::visual_bell(target, visual_bell))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// visual-content [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn get_visual_content<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::visual_content(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// visual-content [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn set_visual_content<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        visual_content: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::visual_content(target, visual_content))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// visual-silence [on | off | both]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn get_visual_silence<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Activity>, Error> {
        self.get(Self::Getter::visual_silence(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// visual-silence [on | off | both]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn set_visual_silence<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        visual_silence: Option<Activity>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::visual_silence(target, visual_silence))
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn get_word_separators<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(Self::Getter::word_separators(target))
    }
    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn set_word_separators<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        word_separators: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::word_separators(target, word_separators))
    }
}
