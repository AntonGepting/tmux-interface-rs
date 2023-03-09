use crate::options::{
    GetGlobalSessionOptionValue, SessionOptions, SessionOptionsCtl, SetGlobalSessionOption,
    SetGlobalSessionOptions, SetSessionOptions,
};
use crate::{Error, ShowOptions, Tmux, TmuxCommand, TmuxOutput};
use std::str::FromStr;

// XXX: rename SessionOptionCtl?
// trait top level options, then server session window pane
pub struct GlobalSessionOptionsCtl<'a> {
    // TODO: comment/doc
    //
    // function used for executing the given option get/set command
    //
    // ```
    // let tmux = Tmux::new();
    // ```
    pub invoker: fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
}

impl<'a> Default for GlobalSessionOptionsCtl<'a> {
    fn default() -> Self {
        Self {
            invoker: |cmd| Tmux::with_command(cmd).output(),
        }
    }
}

impl<'a> GlobalSessionOptionsCtl<'a> {
    pub fn new(invoker: fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
        Self { invoker }
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

impl<'a> SessionOptionsCtl<'a> for GlobalSessionOptionsCtl<'a> {
    type Getter = GetGlobalSessionOptionValue;
    type Setter = SetGlobalSessionOption;

    /// # Examples
    ///
    /// ```
    /// use crate::tmux_interface::{GlobalSessionOptionsCtl, SessionOptionsCtl};
    ///
    /// let session_options = GlobalSessionOptionsCtl::default().get_all().unwrap();
    /// ```
    fn get_all(&self) -> Result<SessionOptions<'a>, Error> {
        let cmd = ShowOptions::new().global().build();
        let output = (self.invoker)(cmd)?.to_string();
        dbg!(&output);
        SessionOptions::from_str(&output)
    }

    fn set_all(self, server_options: SessionOptions<'a>) -> Result<TmuxOutput, Error> {
        //let cmd = ShowOptions::new().build();

        let cmds = SetGlobalSessionOptions::new();

        #[cfg(feature = "tmux_2_6")]
        let cmds = cmds.activity_action(server_options.activity_action);

        #[cfg(feature = "tmux_1_8")]
        let cmds = cmds.assume_paste_time(server_options.assume_paste_time);

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
