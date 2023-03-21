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

use crate::{
    Error, GetPaneOptionTrait, GetPaneOptionValue, PaneOptions, SetClipboard, SetPaneOption,
    SetPaneOptionTrait, SetPaneOptions, SetPaneOptionsTrait, ShowOptions, Switch, Tmux,
    TmuxCommand, TmuxCommands, TmuxOutput,
};

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
    pub invoker: fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
    pub target: Option<Cow<'a, str>>,
}

impl<'a> Default for PaneOptionsCtl<'a> {
    fn default() -> Self {
        Self {
            invoker: |cmd| Tmux::with_command(cmd).output(),
        }
    }
}

impl<'a> PaneOptionsCtl<'a> {
    pub fn new(invoker: fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
        PaneOptionsCtl { invoker }
    }

    fn target(&self) -> Option<Cow<'a, str>> {
        self.target.to_owned()
    }

    //pub fn get(&self) -> Result<Self, Error> {
    //let mut cmd = ShowOptions::new().server().build();
    //let output = self.invoker(&mut cmd);
    //dbg!(&output);
    //PaneOptions::from_str(&output)
    //}

    pub fn get_all(&self) -> Result<PaneOptions<'a>, Error> {
        let cmd = ShowOptions::new().pane().build();
        let output = (self.invoker)(cmd)?.to_string();
        dbg!(&output);
        PaneOptions::from_str(&output)
    }

    pub fn get_all_ext(
        invoke: &dyn Fn(&mut TmuxCommand<'a>) -> String,
    ) -> Result<PaneOptions<'a>, Error> {
        let mut cmd = ShowOptions::new().pane().build();
        let output = invoke(&mut cmd);
        dbg!(&output);
        PaneOptions::from_str(&output)
    }

    pub fn set_all_ext(
        self,
        invoke: &dyn Fn(&TmuxCommands<'a>) -> Result<String, Error>,
        pane_options: PaneOptions<'a>,
    ) -> Result<String, Error> {
        let cmds = SetPaneOptions::new();

        // `@USER_OPTION`

        let cmds = cmds.build();

        invoke(&cmds)
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

impl<'a> PaneOptionsCtl<'a> {}
