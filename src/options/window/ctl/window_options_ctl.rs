use crate::{
    ClockModeStyle, Error, GetWindowOption, SetWindowOptionExt, SetWindowOptions, StatusKeys,
    Switch, TmuxCommand, TmuxOutput, WindowOptions,
};
use std::str::FromStr;

#[cfg(feature = "tmux_2_3")]
use crate::PaneBorderStatus;

#[cfg(feature = "tmux_2_9")]
use crate::WindowSize;

use std::borrow::Cow;

// trait, subtrai for global local
pub trait WindowOptionsCtl<'a> {
    type Getter: GetWindowOption;
    type Setter: SetWindowOptionExt;
    type GetterAll: GetWindowOption;
    type SetterMultiple: SetWindowOptions<'a>;

    fn target(&self) -> Option<Cow<'a, str>>;

    fn invoker(&self) -> &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>;

    fn get_all(&self) -> Result<WindowOptions<'a>, Error> {
        Self::get_all_ext(self.target(), self.invoker())
    }

    fn get_all_ext(
        target: Option<Cow<'a, str>>,
        invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
    ) -> Result<WindowOptions<'a>, Error> {
        let cmd = Self::GetterAll::all(target);
        let output = (invoker)(cmd)?.to_string();
        WindowOptions::from_str(&output)
    }

    fn set_all(&self, window_options: WindowOptions<'a>) -> Result<TmuxOutput, Error> {
        Self::set_all_ext(self.target(), self.invoker(), window_options)
    }

    // XXX: split in build command custom run command
    fn set_all_ext(
        target: Option<Cow<'a, str>>,
        invoke: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
        window_options: WindowOptions<'a>,
    ) -> Result<TmuxOutput, Error> {
        let cmds = Self::SetterMultiple::new();

        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.aggressive_resize(target.clone(), window_options.aggressive_resize);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
        let cmds = cmds.allow_rename(target.clone(), window_options.allow_rename);
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
        let cmds = cmds.alternate_screen(target.clone(), window_options.alternate_screen);
        #[cfg(feature = "tmux_1_0")] // 0.8
        let cmds = cmds.automatic_rename(target.clone(), window_options.automatic_rename);
        #[cfg(feature = "tmux_1_9")]
        let cmds =
            cmds.automatic_rename_format(target.clone(), window_options.automatic_rename_format);
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
        let cmds = cmds.c0_change_interval(target.clone(), window_options.c0_change_interval);
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
        let cmds = cmds.c0_change_trigger(target.clone(), window_options.c0_change_trigger);
        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.clock_mode_colour(target.clone(), window_options.clock_mode_colour);
        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.clock_mode_style(target.clone(), window_options.clock_mode_style);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
        let cmds = cmds.force_height(target.clone(), window_options.force_height);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
        let cmds = cmds.force_width(target.clone(), window_options.force_width);
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
        let cmds = cmds.layout_history_limit(target.clone(), window_options.layout_history_limit);
        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.main_pane_height(target.clone(), window_options.main_pane_height);
        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.main_pane_width(target.clone(), window_options.main_pane_width);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.mode_attr(target.clone(), window_options.mode_attr);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.mode_bg(target.clone(), window_options.mode_bg);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.mode_fg(target.clone(), window_options.mode_fg);
        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.mode_keys(target.clone(), window_options.mode_keys);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
        let cmds = cmds.mode_mouse(target.clone(), window_options.mode_mouse);
        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.mode_style(target.clone(), window_options.mode_style);
        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.monitor_activity(target.clone(), window_options.monitor_activity);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        let cmds = cmds.monitor_content(target.clone(), window_options.monitor_content);
        #[cfg(feature = "tmux_2_6")]
        let cmds = cmds.monitor_bell(target.clone(), window_options.monitor_bell);
        #[cfg(feature = "tmux_1_4")]
        let cmds = cmds.monitor_silence(target.clone(), window_options.monitor_silence);
        #[cfg(feature = "tmux_1_4")]
        let cmds = cmds.other_pane_height(target.clone(), window_options.other_pane_height);
        #[cfg(feature = "tmux_1_4")]
        let cmds = cmds.other_pane_width(target.clone(), window_options.other_pane_width);
        #[cfg(feature = "tmux_2_0")]
        let cmds =
            cmds.pane_active_border_style(target.clone(), window_options.pane_active_border_style);
        #[cfg(feature = "tmux_1_6")]
        let cmds = cmds.pane_base_index(target.clone(), window_options.pane_base_index);
        #[cfg(feature = "tmux_2_3")]
        let cmds = cmds.pane_border_format(target.clone(), window_options.pane_border_format);
        #[cfg(feature = "tmux_2_3")]
        let cmds = cmds.pane_border_status(target.clone(), window_options.pane_border_status);
        #[cfg(feature = "tmux_2_0")]
        let cmds = cmds.pane_border_style(target.clone(), window_options.pane_border_style);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
        let cmds = cmds.remain_on_exit(target.clone(), window_options.remain_on_exit);
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
        let cmds = cmds.synchronize_panes(target.clone(), window_options.synchronize_panes);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
        let cmds = cmds.utf8(target.clone(), window_options.utf8);
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
        let cmds = cmds.window_active_style(target.clone(), window_options.window_active_style);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds =
            cmds.window_status_bell_attr(target.clone(), window_options.window_status_bell_attr);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds = cmds.window_status_bell_bg(target.clone(), window_options.window_status_bell_bg);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds = cmds.window_status_bell_fg(target.clone(), window_options.window_status_bell_fg);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds = cmds
            .window_status_content_attr(self.target(), window_options.window_status_content_attr);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds =
            cmds.window_status_content_bg(target.clone(), window_options.window_status_content_bg);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds =
            cmds.window_status_content_fg(target.clone(), window_options.window_status_content_fg);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds = cmds
            .window_status_activity_attr(self.target(), window_options.window_status_activity_attr);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds = cmds
            .window_status_activity_bg(target.clone(), window_options.window_status_activity_bg);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let cmds = cmds
            .window_status_activity_fg(target.clone(), window_options.window_status_activity_fg);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.window_status_attr(target.clone(), window_options.window_status_attr);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.window_status_bg(target.clone(), window_options.window_status_bg);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds.window_status_fg(target.clone(), window_options.window_status_fg);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds = cmds
            .window_status_current_attr(self.target(), window_options.window_status_current_attr);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds =
            cmds.window_status_current_bg(target.clone(), window_options.window_status_current_bg);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let cmds =
            cmds.window_status_current_fg(target.clone(), window_options.window_status_current_fg);
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        let cmds =
            cmds.window_status_alert_attr(target.clone(), window_options.window_status_alert_attr);
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        let cmds =
            cmds.window_status_alert_bg(target.clone(), window_options.window_status_alert_bg);
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        let cmds =
            cmds.window_status_alert_fg(target.clone(), window_options.window_status_alert_fg);
        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.window_status_activity_style(
            target.clone(),
            window_options.window_status_activity_style,
        );
        #[cfg(feature = "tmux_1_9")]
        let cmds =
            cmds.window_status_bell_style(target.clone(), window_options.window_status_bell_style);
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        let cmds = cmds
            .window_status_content_style(self.target(), window_options.window_status_content_style);
        #[cfg(feature = "tmux_1_2")]
        let cmds = cmds.window_status_current_format(
            target.clone(),
            window_options.window_status_current_format,
        );
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        let cmds =
            cmds.window_status_last_attr(target.clone(), window_options.window_status_last_attr);
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        let cmds = cmds.window_status_last_bg(target.clone(), window_options.window_status_last_bg);
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        let cmds = cmds.window_status_last_fg(target.clone(), window_options.window_status_last_fg);
        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.window_status_current_style(
            target.clone(),
            window_options.window_status_current_style,
        );
        #[cfg(feature = "tmux_1_2")]
        let cmds = cmds.window_status_format(target.clone(), window_options.window_status_format);
        #[cfg(feature = "tmux_1_9")]
        let cmds =
            cmds.window_status_last_style(target.clone(), window_options.window_status_last_style);
        #[cfg(feature = "tmux_1_7")]
        let cmds =
            cmds.window_status_separator(target.clone(), window_options.window_status_separator);
        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.window_status_style(target.clone(), window_options.window_status_style);
        #[cfg(feature = "tmux_2_9")]
        let cmds = cmds.window_size(target.clone(), window_options.window_size);
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
        let cmds = cmds.word_separators(target.clone(), window_options.word_separators);
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
        let cmds = cmds.window_style(target.clone(), window_options.window_style);
        #[cfg(feature = "tmux_1_7")]
        let cmds = cmds.wrap_search(target.clone(), window_options.wrap_search);
        #[cfg(feature = "tmux_1_0")]
        let cmds = cmds.xterm_keys(target.clone(), window_options.xterm_keys);

        let cmd = TmuxCommand::with_cmds(cmds.build());

        invoke(cmd)
    }

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

    // fn set(&self, cmd: TmuxCommand<'a>) -> Result<TmuxOutput, Error>;
    fn set(&self, cmd: TmuxCommand<'a>) -> Result<TmuxOutput, Error> {
        (self.invoker())(cmd)
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // aggressive-resize [on | off]
    // ```
    #[cfg(feature = "tmux_1_0")]
    fn get_aggressive_resize(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::aggressive_resize(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // aggressive-resize [on | off]
    // ```
    #[cfg(feature = "tmux_1_0")]
    fn set_aggressive_resize(
        &self,
        aggressive_resize: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::aggressive_resize(
            self.target(),
            aggressive_resize,
        ))
    }

    // # Manual
    //
    // tmux ^1.6 v3.0:
    // ```text
    // allow-rename [on | off]
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    fn get_allow_rename(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::allow_rename(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6 v3.0:
    // ```text
    // allow-rename [on | off]
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    fn set_allow_rename(&self, allow_rename: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::allow_rename(self.target(), allow_rename))
    }

    // # Manual
    //
    // tmux ^1.2 v3.0:
    // ```text
    // alternate-screen [on | off]
    // ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    fn get_alternate_screen(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::alternate_screen(self.target()))
    }

    // # Manual
    //
    // tmux ^1.2 v3.0:
    // ```text
    // alternate-screen [on | off]
    // ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    fn set_alternate_screen(&self, alternate_screen: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::alternate_screen(
            self.target(),
            alternate_screen,
        ))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // automatic-rename [on | off]
    // ```
    #[cfg(feature = "tmux_1_0")] // 0.8
    fn get_automatic_rename(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::automatic_rename(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // automatic-rename [on | off]
    // ```
    #[cfg(feature = "tmux_1_0")] // 0.8
    fn set_automatic_rename(&self, automatic_rename: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::automatic_rename(
            self.target(),
            automatic_rename,
        ))
    }

    // # Manual
    //
    // tmux ^1.9:
    // ```text
    // automatic-rename-format format
    // ```
    #[cfg(feature = "tmux_1_9")]
    fn get_automatic_rename_format(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::automatic_rename_format(self.target()))
    }

    // # Manual
    //
    // tmux ^1.9:
    // ```text
    // automatic-rename-format format
    // ```
    #[cfg(feature = "tmux_1_9")]
    fn set_automatic_rename_format(
        &self,
        automatic_rename_format: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::automatic_rename_format(
            self.target(),
            automatic_rename_format,
        ))
    }

    // # Manual
    //
    // tmux ^1.7 v2.1:
    // ```text
    // c0-change-interval interval
    // ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    fn get_c0_change_interval(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::c0_change_interval(self.target()))
    }

    // # Manual
    //
    // tmux ^1.7 v2.1:
    // ```text
    // c0-change-interval interval
    // ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    fn set_c0_change_interval(
        &self,
        c0_change_interval: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::c0_change_interval(
            self.target(),
            c0_change_interval,
        ))
    }

    // # Manual
    //
    // tmux ^1.7 v2.1:
    // ```text
    // c0-change-trigger trigger
    // ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    fn get_c0_change_trigger(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::automatic_rename(self.target()))
    }

    // # Manual
    //
    // tmux ^1.7 v2.1:
    // ```text
    // c0-change-trigger trigger
    // ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    fn set_c0_change_trigger(&self, c0_change_trigger: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::c0_change_trigger(
            self.target(),
            c0_change_trigger,
        ))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // clock-mode-colour colour
    // ```
    #[cfg(feature = "tmux_1_0")]
    fn get_clock_mode_colour(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::clock_mode_colour(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // clock-mode-colour colour
    // ```
    #[cfg(feature = "tmux_1_0")]
    fn set_clock_mode_colour(
        &self,
        clock_mode_colour: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::clock_mode_colour(
            self.target(),
            clock_mode_colour,
        ))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // clock-mode-style [12 | 24]
    // ```
    #[cfg(feature = "tmux_1_0")]
    fn get_clock_mode_style(&self) -> Result<Option<ClockModeStyle>, Error> {
        self.get(Self::Getter::clock_mode_style(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // clock-mode-style [12 | 24]
    // ```
    #[cfg(feature = "tmux_1_0")]
    fn set_clock_mode_style(
        &self,
        clock_mode_style: Option<ClockModeStyle>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::clock_mode_style(
            self.target(),
            clock_mode_style,
        ))
    }

    // # Manual
    //
    // tmux ^1.0 v2.9:
    // ```text
    // force-height height
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    fn get_force_height(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::force_height(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0 v2.9:
    // ```text
    // force-height height
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    fn set_force_height(&self, force_height: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::force_height(self.target(), force_height))
    }

    // # Manual
    //
    // tmux ^1.0 v2.9:
    // ```text
    // force-width width
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    fn get_force_width(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::force_width(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0 v2.9:
    // ```text
    // force-width width
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    fn set_force_width(&self, force_width: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::force_width(self.target(), force_width))
    }

    // # Manual
    //
    // tmux ^1.7 v1.8:
    // ```text
    // layout-history-limit limit
    // ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    fn get_layout_history_limit(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::layout_history_limit(self.target()))
    }

    // # Manual
    //
    // tmux ^1.7 v1.8:
    // ```text
    // layout-history-limit limit
    // ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    fn set_layout_history_limit(
        &self,
        layout_history_limit: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::layout_history_limit(
            self.target(),
            layout_history_limit,
        ))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // main-pane-height height
    // ```
    #[cfg(feature = "tmux_1_0")]
    fn get_main_pane_height(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::main_pane_height(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // main-pane-height height
    // ```
    #[cfg(feature = "tmux_1_0")]
    fn set_main_pane_height(&self, main_pane_height: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::main_pane_height(
            self.target(),
            main_pane_height,
        ))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // main-pane-width width
    // ```
    #[cfg(feature = "tmux_1_0")]
    fn get_main_pane_width(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::main_pane_width(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // main-pane-width width
    // ```
    #[cfg(feature = "tmux_1_0")]
    fn set_main_pane_width(&self, main_pane_width: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::main_pane_width(
            self.target(),
            main_pane_width,
        ))
    }

    // # Manual
    //
    // tmux ^1.0 v1.9:
    // ```text
    // mode-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_mode_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::mode_attr(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0 v1.9:
    // ```text
    // mode-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_mode_attr(&self, mode_attr: Option<Cow<'a, str>>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mode_attr(self.target(), mode_attr))
    }

    // # Manual
    //
    // tmux ^1.0 v1.9:
    // ```text
    // mode-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_mode_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::mode_bg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0 v1.9:
    // ```text
    // mode-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_mode_bg(&self, mode_bg: Option<Cow<'a, str>>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mode_bg(self.target(), mode_bg))
    }

    // # Manual
    //
    // tmux ^1.0 v1.9:
    // ```text
    // mode-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_mode_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::mode_fg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0 v1.9:
    // ```text
    // mode-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_mode_fg(&self, mode_fg: Option<Cow<'a, str>>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mode_fg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // mode-keys [vi | emacs]
    // ```
    #[cfg(feature = "tmux_1_0")]
    fn get_mode_keys(&self) -> Result<Option<StatusKeys>, Error> {
        self.get(Self::Getter::mode_keys(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // mode-keys [vi | emacs]
    // ```
    #[cfg(feature = "tmux_1_0")]
    fn set_mode_keys(&self, mode_keys: Option<StatusKeys>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mode_keys(self.target(), mode_keys))
    }

    // # Manual
    //
    // tmux ^1.0 v2.1:
    // ```text
    // mode-mouse [on | off]
    // ```
    //
    // tmux ^1.6:
    // ```text
    // mode-mouse [on | off | copy-mode]
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    fn get_mode_mouse(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::mode_mouse(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0 v2.1:
    // ```text
    // mode-mouse [on | off]
    // ```
    //
    // tmux ^1.6:
    // ```text
    // mode-mouse [on | off | copy-mode]
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    fn set_mode_mouse(&self, mode_mouse: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mode_mouse(self.target()))
    }

    // # Manual
    //
    // tmux ^1.9:
    // ```text
    // mode-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    fn get_mode_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::mode_style(self.target()))
    }

    // # Manual
    //
    // tmux ^1.9:
    // ```text
    // mode-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    fn set_mode_style(&self, mode_style: Option<Cow<'a, str>>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mode_style(self.target(), mode_style))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // monitor-activity [on | off]
    // ```
    #[cfg(feature = "tmux_1_0")]
    fn get_monitor_activity(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::monitor_activity(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // monitor-activity [on | off]
    // ```
    #[cfg(feature = "tmux_1_0")]
    fn set_monitor_activity(&self, monitor_activity: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::monitor_activity(
            self.target(),
            monitor_activity,
        ))
    }

    // # Manual
    //
    // tmux ^1.0 v2.0:
    // ```text
    // monitor-content match-string
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn get_monitor_content(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::monitor_content(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0 v2.0:
    // ```text
    // monitor-content match-string
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn set_monitor_content(
        &self,
        monitor_content: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::monitor_content(self.target()))
    }

    // # Manual
    //
    // tmux ^2.6:
    // ```text
    // monitor-bell [on | off]
    // ```
    #[cfg(feature = "tmux_2_6")]
    fn get_monitor_bell(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::monitor_bell(self.target()))
    }

    // # Manual
    //
    // tmux ^2.6:
    // ```text
    // monitor-bell [on | off]
    // ```
    #[cfg(feature = "tmux_2_6")]
    fn set_monitor_bell(&self, monitor_bell: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::monitor_bell(self.target(), monitor_bell))
    }

    // # Manual
    //
    // tmux ^1.4:
    // ```text
    // monitor-silence [interval]
    // ```
    #[cfg(feature = "tmux_1_4")]
    fn get_monitor_silence(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::monitor_silence(self.target()))
    }

    // # Manual
    //
    // tmux ^1.4:
    // ```text
    // monitor-silence [interval]
    // ```
    #[cfg(feature = "tmux_1_4")]
    fn set_monitor_silence(&self, monitor_silence: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::monitor_silence(
            self.target(),
            monitor_silence,
        ))
    }

    // # Manual
    //
    // tmux ^1.4:
    // ```text
    // other-pane-height height
    // ```
    #[cfg(feature = "tmux_1_4")]
    fn get_other_pane_height(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::other_pane_height(self.target()))
    }

    // # Manual
    //
    // tmux ^1.4:
    // ```text
    // other-pane-height height
    // ```
    #[cfg(feature = "tmux_1_4")]
    fn set_other_pane_height(&self, other_pane_height: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::other_pane_height(
            self.target(),
            other_pane_height,
        ))
    }

    // # Manual
    //
    // tmux ^1.4:
    // ```text
    // other-pane-width width
    // ```
    #[cfg(feature = "tmux_1_4")]
    fn get_other_pane_width(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::other_pane_width(self.target()))
    }

    // # Manual
    //
    // tmux ^1.4:
    // ```text
    // other-pane-width width
    // ```
    #[cfg(feature = "tmux_1_4")]
    fn set_other_pane_width(&self, other_pane_width: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::other_pane_width(
            self.target(),
            other_pane_width,
        ))
    }

    // # Manual
    //
    // tmux ^2.0:
    // ```text
    // pane-active-border-style style
    // ```
    #[cfg(feature = "tmux_2_0")]
    fn get_pane_active_border_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_active_border_style(self.target()))
    }

    // # Manual
    //
    // tmux ^2.0:
    // ```text
    // pane-active-border-style style
    // ```
    #[cfg(feature = "tmux_2_0")]
    fn set_pane_active_border_style(
        &self,
        pane_active_border_style: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_active_border_style(
            self.target(),
            pane_active_border_style,
        ))
    }

    // # Manual
    //
    // tmux ^1.6:
    // ```text
    // pane-base-index index
    // ```
    #[cfg(feature = "tmux_1_6")]
    fn get_pane_base_index(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::pane_base_index(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6:
    // ```text
    // pane-base-index index
    // ```
    #[cfg(feature = "tmux_1_6")]
    fn set_pane_base_index(&self, pane_base_index: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_base_index(
            self.target(),
            pane_base_index,
        ))
    }

    // # Manual
    //
    // tmux ^2.3:
    // ```text
    // pane-border-format format
    // ```
    #[cfg(feature = "tmux_2_3")]
    fn get_pane_border_format(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_border_format(self.target()))
    }

    // # Manual
    //
    // tmux ^2.3:
    // ```text
    // pane-border-format format
    // ```
    #[cfg(feature = "tmux_2_3")]
    fn set_pane_border_format(
        &self,
        pane_border_format: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_border_format(
            self.target(),
            pane_border_format,
        ))
    }

    // # Manual
    //
    // tmux ^2.3:
    // ```text
    // pane-border-status [off | top | bottom]
    // ```
    #[cfg(feature = "tmux_2_3")]
    fn get_pane_border_status(&self) -> Result<Option<PaneBorderStatus>, Error> {
        self.get(Self::Getter::pane_border_status(self.target()))
    }

    // # Manual
    //
    // tmux ^2.3:
    // ```text
    // pane-border-status [off | top | bottom]
    // ```
    #[cfg(feature = "tmux_2_3")]
    fn set_pane_border_status(
        &self,
        pane_border_status: Option<PaneBorderStatus>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_border_status(
            self.target(),
            pane_border_status,
        ))
    }

    // # Manual
    //
    // tmux ^2.0:
    // ```text
    // pane-border-style style
    // ```
    #[cfg(feature = "tmux_2_0")]
    fn get_pane_border_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_border_style(self.target()))
    }

    // # Manual
    //
    // tmux ^2.0:
    // ```text
    // pane-border-style style
    // ```
    #[cfg(feature = "tmux_2_0")]
    fn set_pane_border_style(
        &self,
        pane_border_style: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_border_style(
            self.target(),
            pane_border_style,
        ))
    }

    // # Manual
    //
    // tmux ^1.0 v3.0:
    // ```text
    // remain-on-exit [on | off]
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    fn get_remain_on_exit(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::remain_on_exit(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0 v3.0:
    // ```text
    // remain-on-exit [on | off]
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    fn set_remain_on_exit(&self, remain_on_exit: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::remain_on_exit(self.target(), remain_on_exit))
    }

    // # Manual
    //
    // tmux ^1.2 v3.2:
    // ```text
    // synchronize-panes [on | off]
    // ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    fn get_synchronize_panes(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::synchronize_panes(self.target()))
    }

    // # Manual
    //
    // tmux ^1.2 v3.2:
    // ```text
    // synchronize-panes [on | off]
    // ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    fn set_synchronize_panes(
        &self,
        synchronize_panes: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::synchronize_panes(
            self.target(),
            synchronize_panes,
        ))
    }

    // # Manual
    //
    // tmux ^1.0 v2.2:
    // ```text
    // utf8 [on | off]
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn get_utf8(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::utf8(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0 v2.2:
    // ```text
    // utf8 [on | off]
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn set_utf8(&self, utf8: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::utf8(self.target(), utf8))
    }

    // # Manual
    //
    // tmux ^2.1 v3.0:
    // ```text
    // window-active-style style
    // ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    fn get_window_active_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_active_style(self.target()))
    }

    // # Manual
    //
    // tmux ^2.1 v3.0:
    // ```text
    // window-active-style style
    // ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    fn set_window_active_style(
        &self,
        window_active_style: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_active_style(
            self.target(),
            window_active_style,
        ))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-bell-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_window_status_bell_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_bell_attr(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-bell-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn set_window_status_bell_attr(
        &self,
        window_status_bell_attr: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_bell_attr(
            self.target(),
            window_status_bell_attr,
        ))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-bell-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_window_status_bell_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_bell_bg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-bell-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn set_window_status_bell_bg(
        &self,
        window_status_bell_bg: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_bell_bg(
            self.target(),
            window_status_bell_bg,
        ))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-bell-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_window_status_bell_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_bell_fg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-bell-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn set_window_status_bell_fg(
        &self,
        window_status_bell_fg: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_bell_fg(
            self.target(),
            window_status_bell_fg,
        ))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-content-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_window_status_content_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_content_attr(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-content-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn set_window_status_content_attr(
        &self,
        window_status_content_attr: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_content_attr(
            self.target(),
            window_status_content_attr,
        ))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-content-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_window_status_content_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_content_bg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-content-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn set_window_status_content_bg(
        &self,
        window_status_content_bg: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_content_bg(
            self.target(),
            window_status_content_bg,
        ))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-content-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_window_status_content_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_content_fg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-content-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn set_window_status_content_fg(
        &self,
        window_status_content_fg: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_content_fg(
            self.target(),
            window_status_content_fg,
        ))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-activity-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_window_status_activity_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_activity_attr(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-activity-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn set_window_status_activity_attr(
        &self,
        window_status_activity_attr: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_activity_attr(
            self.target(),
            window_status_activity_attr,
        ))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-activity-bg attributes
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_window_status_activity_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_activity_bg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-activity-bg attributes
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn set_window_status_activity_bg(
        &self,
        window_status_activity_bg: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_activity_bg(
            self.target(),
            window_status_activity_bg,
        ))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-activity-fg attributes
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_window_status_activity_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_activity_fg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-activity-fg attributes
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn set_window_status_activity_fg(
        &self,
        window_status_activity_fg: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_activity_fg(
            self.target(),
            window_status_activity_fg,
        ))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_window_status_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_attr(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_window_status_attr(
        &self,
        window_status_attr: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_attr(
            self.target(),
            window_status_attr,
        ))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_window_status_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_bg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_window_status_bg(
        &self,
        window_status_bg: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_bg(
            self.target(),
            window_status_bg,
        ))
    }
    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_window_status_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_fg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_window_status_fg(
        &self,
        window_status_fg: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_fg(
            self.target(),
            window_status_fg,
        ))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-current-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_window_status_current_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_current_attr(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-current-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_window_status_current_attr(
        &self,
        window_status_current_attr: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_current_attr(
            self.target(),
            window_status_current_attr,
        ))
    }
    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-current-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_window_status_current_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_current_bg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-current-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_window_status_current_bg(
        &self,
        window_status_current_bg: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_current_bg(
            self.target(),
            window_status_current_bg,
        ))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-current-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_window_status_current_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_current_fg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-current-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_window_status_current_fg(
        &self,
        window_status_current_fg: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_current_fg(
            self.target(),
            window_status_current_fg,
        ))
    }

    // # Manual
    //
    // tmux ^1.3 v1.6:
    // ```text
    // window-status-alert-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn get_window_status_alert_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_alert_attr(self.target()))
    }

    // # Manual
    //
    // tmux ^1.3 v1.6:
    // ```text
    // window-status-alert-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn set_window_status_alert_attr(
        &self,
        window_status_alert_attr: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_alert_attr(
            self.target(),
            window_status_alert_attr,
        ))
    }

    // # Manual
    //
    // tmux ^1.3 v1.6:
    // ```text
    // window-status-alert-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn get_window_status_alert_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_alert_bg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.3 v1.6:
    // ```text
    // window-status-alert-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn set_window_status_alert_bg(
        &self,
        window_status_alert_bg: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_alert_bg(
            self.target(),
            window_status_alert_bg,
        ))
    }

    // # Manual
    //
    // tmux ^1.3 v1.6:
    // ```text
    // window-status-alert-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn get_window_status_alert_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_alert_fg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.3 v1.6:
    // ```text
    // window-status-alert-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn set_window_status_alert_fg(
        &self,
        window_status_alert_fg: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_alert_fg(
            self.target(),
            window_status_alert_fg,
        ))
    }

    // # Manual
    //
    // tmux ^1.9:
    // ```text
    // window-status-activity-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    fn get_window_status_activity_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_activity_style(self.target()))
    }

    // # Manual
    //
    // tmux ^1.9:
    // ```text
    // window-status-activity-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    fn set_window_status_activity_style(
        &self,
        window_status_activity_style: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_activity_style(
            self.target(),
            window_status_activity_style,
        ))
    }

    // # Manual
    //
    // tmux ^1.9:
    // ```text
    // window-status-bell-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    fn get_window_status_bell_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_bell_style(self.target()))
    }

    // # Manual
    //
    // tmux ^1.9:
    // ```text
    // window-status-bell-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    fn set_window_status_bell_style(
        &self,
        window_status_bell_style: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_bell_style(
            self.target(),
            window_status_bell_style,
        ))
    }

    // # Manual
    //
    // tmux ^1.9 v2.0:
    // ```text
    // window-status-content-style style
    // ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn get_window_status_content_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_content_style(self.target()))
    }

    // # Manual
    //
    // tmux ^1.9 v2.0:
    // ```text
    // window-status-content-style style
    // ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn set_window_status_content_style(
        &self,
        window_status_content_style: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_content_style(
            self.target(),
            window_status_content_style,
        ))
    }

    // # Manual
    //
    // tmux ^1.2:
    // ```text
    // window-status-current-format string
    // ```
    #[cfg(feature = "tmux_1_2")]
    fn get_window_status_current_format(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_current_format(self.target()))
    }

    // # Manual
    //
    // tmux ^1.2:
    // ```text
    // window-status-current-format string
    // ```
    #[cfg(feature = "tmux_1_2")]
    fn set_window_status_current_format(
        &self,
        window_status_current_format: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_current_format(
            self.target(),
            window_status_current_format,
        ))
    }

    // # Manual
    //
    // tmux ^1.8 v1.9:
    // ```text
    // window-status-last-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn get_window_status_last_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_last_attr(self.target()))
    }

    // # Manual
    //
    // tmux ^1.8 v1.9:
    // ```text
    // window-status-last-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn set_window_status_last_attr(
        &self,
        window_status_last_attr: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_last_attr(
            self.target(),
            window_status_last_attr,
        ))
    }

    // # Manual
    //
    // tmux ^1.8 v1.9:
    // ```text
    // window-status-last-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn get_window_status_last_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_last_bg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.8 v1.9:
    // ```text
    // window-status-last-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn set_window_status_last_bg(
        &self,
        window_status_last_bg: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_last_bg(
            self.target(),
            window_status_last_bg,
        ))
    }

    // # Manual
    //
    // tmux ^1.8 v1.9:
    // ```text
    // window-status-last-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn get_window_status_last_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_last_fg(self.target()))
    }

    // # Manual
    //
    // tmux ^1.8 v1.9:
    // ```text
    // window-status-last-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn set_window_status_last_fg(
        &self,
        window_status_last_fg: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_last_fg(
            self.target(),
            window_status_last_fg,
        ))
    }

    // # Manual
    //
    // tmux ^1.9:
    // ```text
    // window-status-current-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    fn get_window_status_current_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_current_style(self.target()))
    }

    // # Manual
    //
    // tmux ^1.9:
    // ```text
    // window-status-current-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    fn set_window_status_current_style(
        &self,
        window_status_current_style: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_current_style(
            self.target(),
            window_status_current_style,
        ))
    }

    // # Manual
    //
    // tmux ^1.2:
    // ```text
    // window-status-format string
    // ```
    #[cfg(feature = "tmux_1_2")]
    fn get_window_status_format(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_format(self.target()))
    }

    // # Manual
    //
    // tmux ^1.2:
    // ```text
    // window-status-format string
    // ```
    #[cfg(feature = "tmux_1_2")]
    fn set_window_status_format(
        &self,
        window_status_format: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_format(
            self.target(),
            window_status_format,
        ))
    }

    // # Manual
    //
    // tmux ^1.9:
    // ```text
    // window-status-last-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    fn get_window_status_last_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_last_style(self.target()))
    }

    // # Manual
    //
    // tmux ^1.9:
    // ```text
    // window-status-last-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    fn set_window_status_last_style(
        &self,
        window_status_last_style: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_last_style(
            self.target(),
            window_status_last_style,
        ))
    }

    // # Manual
    //
    // tmux ^1.7:
    // ```text
    // window-status-separator string
    // ```
    #[cfg(feature = "tmux_1_7")]
    fn get_window_status_separator(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_separator(self.target()))
    }

    // # Manual
    //
    // tmux ^1.7:
    // ```text
    // window-status-separator string
    // ```
    #[cfg(feature = "tmux_1_7")]
    fn set_window_status_separator(
        &self,
        window_status_separator: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_separator(
            self.target(),
            window_status_separator,
        ))
    }

    // # Manual
    //
    // tmux ^1.9:
    // ```text
    // window-status-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    fn get_window_status_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_status_style(self.target()))
    }

    // # Manual
    //
    // tmux ^1.9:
    // ```text
    // window-status-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    fn set_window_status_style(
        &self,
        window_status_style: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_status_style(
            self.target(),
            window_status_style,
        ))
    }

    // # Manual
    //
    // tmux ^2.9:
    // ```text
    // window-size largest | smallest | manual | latest
    // ```
    #[cfg(feature = "tmux_2_9")]
    fn get_window_size(&self) -> Result<Option<WindowSize>, Error> {
        self.get(Self::Getter::window_size(self.target()))
    }

    // # Manual
    //
    // tmux ^2.9:
    // ```text
    // window-size largest | smallest | manual | latest
    // ```
    #[cfg(feature = "tmux_2_9")]
    fn set_window_size(&self, window_size: Option<WindowSize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_size(self.target(), window_size))
    }

    // # Manual
    //
    // tmux ^1.2 v1.6:
    // ```text
    // word-separators string
    // ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    fn get_word_separators(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::word_separators(self.target()))
    }

    // # Manual
    //
    // tmux ^1.2 v1.6:
    // ```text
    // word-separators string
    // ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    fn set_word_separators(
        &self,
        word_separators: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::word_separators(
            self.target(),
            word_separators,
        ))
    }

    // # Manual
    //
    // tmux ^2.1 v3.0:
    // ```text
    // window-style style
    // ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    fn get_window_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::window_style(self.target()))
    }

    // # Manual
    //
    // tmux ^2.1 v3.0:
    // ```text
    // window-style style
    // ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    fn set_window_style(&self, window_style: Option<Cow<'a, str>>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::window_style(self.target(), window_style))
    }

    // # Manual
    //
    // tmux ^1.7:
    // ```text
    // wrap-search [on | off]
    // ```
    #[cfg(feature = "tmux_1_7")]
    fn get_wrap_search(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::wrap_search(self.target()))
    }

    // # Manual
    //
    // tmux ^1.7:
    // ```text
    // wrap-search [on | off]
    // ```
    #[cfg(feature = "tmux_1_7")]
    fn set_wrap_search(&self, wrap_search: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::wrap_search(self.target(), wrap_search))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // xterm-keys [on | off]
    // ```
    #[cfg(feature = "tmux_1_0")]
    fn get_xterm_keys(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::xterm_keys(self.target()))
    }

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // xterm-keys [on | off]
    // ```
    #[cfg(feature = "tmux_1_0")]
    fn set_xterm_keys(&self, xterm_keys: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::xterm_keys(self.target(), xterm_keys))
    }

    // # Manual
    //
    // ```text
    // @user-option value
    // ```
    // fn get_user_options(&self) -> Result<Option<HashMap<String, Option<Cow<'a, str>>>>, Error> {
    //     self.get(Self::Getter::user_options(self.target(), ))
    // }

    // # Manual
    //
    // ```text
    // @user-option value
    // ```
    // fn set_user_options(&self, user_options: Option<HashMap<String<>>>) -> Result<TmuxOutput, Error> {
    //     self.set(Self::Setter::user_options(self.target(), user_options))
    // }
}
