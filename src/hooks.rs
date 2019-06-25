use super::tmux_interface::*;
use super::tmux_interface_error::TmuxInterfaceError;
use std::process::Output;


/// # Manual
///
/// ```text
/// tmux set-hook [-agRu] [-t target-session] hook-name command
/// ```
#[derive(Default, Clone, Debug)]
pub struct SetHook<'a> {
    pub append: Option<bool>,                   // [-a]
    pub global: Option<bool>,                   // [-g]
    pub run: Option<bool>,                      // [-R]
    pub unset: Option<bool>,                    // [-u]
    pub target_session: Option<&'a str>,        // [-t target-session]
    pub hook_name: &'a str,                     // hook-name
    pub command: &'a str                        // command
}

impl<'a> SetHook<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}


/// Hooks
impl<'a> TmuxInterface<'a> {


    const SET_HOOK: &'static str = "set-hook";
    const SHOW_HOOK: &'static str = "show-hook";


    /// # Manual
    ///
    /// ```text
    /// tmux set-hook [-agRu] [-t target-session] hook-name command
    /// ```
    pub fn set_hook(&self, set_hook: &SetHook) -> Result<Output, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if set_hook.append.unwrap_or(false) { args.push(a_KEY); }
        if set_hook.global.unwrap_or(false) { args.push(g_KEY); }
        if set_hook.run.unwrap_or(false) { args.push(R_KEY); }
        if set_hook.unset.unwrap_or(false) { args.push(u_KEY); }
        set_hook.target_session.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        args.push(set_hook.hook_name);
        args.push(set_hook.command);
        let output = self.subcommand(TmuxInterface::SET_HOOK, &args)?;
        Ok(output)
    }


    /// # Manual
    ///
    /// ```text
    /// tmux show-hooks [-g] [-t target-session]
    /// ```
    pub fn show_hooks(&self, global: Option<bool>, target_session: Option<&str>) -> Result<Output, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if global.unwrap_or(false) { args.push(g_KEY); }
        target_session.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::SHOW_HOOK, &args)?;
        Ok(output)
    }


}
