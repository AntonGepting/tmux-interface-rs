use crate::{
    Error, GetPaneOption, GetPaneOptionTrait, PaneOptions, RemainOnExit, SetPaneOption,
    SetPaneOptionTrait, SetPaneOptions, SetPaneOptionsTrait, ShowOptions, Switch, Tmux,
    TmuxCommand, TmuxOutput,
};
use std::borrow::Cow;
use std::str::FromStr;

//oneline
//multiline

// Output:
// * random (got buffer, need recognize fields)
//      * Form
//          * long (`option_name value`)
//          * short (`value`), single option get/parse (otherwise there is no chance to assign)
//
// * expected (got buffer, need to extract some fields)
//      * Form
//          * long (`option_name value`)
//          * short (`value`), single option get/parse (otherwise there is no chance to assign)
//

// get
// * get single one
// * get all for object
//
// set
// * set single one
//      * set value
//      * toggle (on/off {default}/off) if no value specified

// FIXME: proper Error in return
//
//
// PaneOption::backspace() -> Result<Self::Backspace(String), Error>
//  GetPaneOption::backspace()
//  ParsePaneOption::from_str()
//
// PaneOption::set().backspace() -> Result<(), Error> {
//  SetPaneOption::backspace()
//  Output
// }

// XXX: rename PaneOptionCtl?
// trait top level options, then server session window pane
pub struct PaneOptionsCtl<'a> {
    // TODO: comment/doc
    //
    // function used for executing the given option get/set command
    //
    // ```
    // let tmux = Tmux::new();
    // ```
    pub invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
    pub target: Option<Cow<'a, str>>,
}

impl<'a> Default for PaneOptionsCtl<'a> {
    fn default() -> Self {
        Self {
            invoker: &|cmd| Tmux::with_command(cmd).output(),
            target: None,
        }
    }
}

impl<'a> PaneOptionsCtl<'a> {
    pub fn new(
        target: Option<Cow<'a, str>>,
        invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
    ) -> Self {
        PaneOptionsCtl { invoker, target }
    }

    pub fn target(&self) -> Option<Cow<'a, str>> {
        self.target.to_owned()
    }

    pub fn invoker(&self) -> &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error> {
        self.invoker
    }

    pub fn get_all(&self) -> Result<PaneOptions<'a>, Error> {
        Self::get_all_ext(self.target(), self.invoker())
    }

    pub fn get_all_ext(
        target: Option<Cow<'a, str>>,
        invoke: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
    ) -> Result<PaneOptions<'a>, Error> {
        let cmd = ShowOptions::new().pane();
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        let cmd = cmd.build();
        let output = invoke(cmd)?.to_string();
        PaneOptions::from_str(&output)
    }

    pub fn set_all(&self, pane_options: PaneOptions<'a>) -> Result<TmuxOutput, Error> {
        Self::set_all_ext(self.target(), self.invoker(), pane_options)
    }

    pub fn set_all_ext(
        target: Option<Cow<'a, str>>,
        invoke: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
        pane_options: PaneOptions<'a>,
    ) -> Result<TmuxOutput, Error> {
        let cmds = SetPaneOptions::new();

        #[cfg(feature = "tmux_3_0")]
        let cmds = cmds.allow_rename(target.clone(), pane_options.allow_rename);
        #[cfg(feature = "tmux_3_0")]
        let cmds = cmds.alternate_screen(target.clone(), pane_options.alternate_screen);
        #[cfg(feature = "tmux_3_0")]
        let cmds = cmds.remain_on_exit(target.clone(), pane_options.remain_on_exit);
        #[cfg(feature = "tmux_3_0")]
        let cmds = cmds.window_active_style(target.clone(), pane_options.window_active_style);
        #[cfg(feature = "tmux_3_0")]
        let cmds = cmds.window_style(target.clone(), pane_options.window_style);
        #[cfg(feature = "tmux_3_2")]
        let cmds = cmds.synchronize_panes(target.clone(), pane_options.synchronize_panes);
        // #[cfg(feature = "tmux_3_0")]
        // let pane_options = pane_options.user_options();
        // `@USER_OPTION`

        let cmd = TmuxCommand::with_cmds(cmds.build());

        invoke(cmd)
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

    /// tmux ^3.0:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn get_allow_rename(&self) -> Result<Option<Switch>, Error> {
        self.get(GetPaneOption::allow_rename(self.target.clone()))
    }

    /// tmux ^3.0:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn set_allow_rename<S: Into<Cow<'a, str>>>(
        &self,
        allow_rename: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetPaneOption::allow_rename(
            self.target.clone(),
            allow_rename,
        ))
    }

    /// tmux ^3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn get_alternate_screen(&self) -> Result<Option<Switch>, Error> {
        self.get(GetPaneOption::alternate_screen(self.target.clone()))
    }

    /// tmux ^3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn set_alternate_screen<S: Into<Cow<'a, str>>>(
        &self,
        alternate_screen: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetPaneOption::alternate_screen(
            self.target.clone(),
            alternate_screen,
        ))
    }

    /// tmux ^3.2:
    /// ```text
    /// remain-on-exit [on | off | failed]
    /// ```
    ///
    /// tmux ^3.0:
    /// ```text
    /// remain-on-exit [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn get_remain_on_exit(&self) -> Result<Option<RemainOnExit>, Error> {
        self.get(GetPaneOption::remain_on_exit(self.target.clone()))
    }

    /// tmux ^3.2:
    /// ```text
    /// remain-on-exit [on | off | failed]
    /// ```
    ///
    /// tmux ^3.0:
    /// ```text
    /// remain-on-exit [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn set_remain_on_exit<S: Into<Cow<'a, str>>>(
        &self,
        remain_on_exit: Option<RemainOnExit>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetPaneOption::remain_on_exit(
            self.target.clone(),
            remain_on_exit,
        ))
    }

    /// tmux ^3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn get_window_active_style(&self) -> Result<Option<String>, Error> {
        self.get(GetPaneOption::window_active_style(self.target.clone()))
    }

    /// tmux ^3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn set_window_active_style<S: Into<Cow<'a, str>>>(
        &self,
        window_active_style: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetPaneOption::window_active_style(
            self.target.clone(),
            window_active_style,
        ))
    }

    /// tmux ^3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn get_window_style(&self) -> Result<Option<String>, Error> {
        self.get(GetPaneOption::window_style(self.target.clone()))
    }

    /// tmux ^3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn set_window_style<S: Into<Cow<'a, str>>>(
        &self,
        window_style: Option<Cow<'a, str>>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetPaneOption::window_style(
            self.target.clone(),
            window_style,
        ))
    }

    /// tmux ^3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn get_synchronize_panes(&self) -> Result<Option<Switch>, Error> {
        self.get(GetPaneOption::synchronize_panes(self.target.clone()))
    }

    /// tmux ^3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn set_synchronize_panes(
        &self,
        synchronize_panes: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetPaneOption::synchronize_panes(
            self.target.clone(),
            synchronize_panes,
        ))
    }
}
