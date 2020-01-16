use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure for setting or unsetting hook `hook-name` to command.
///
/// # Manual
///
/// tmux X.X:
/// ```text
/// tmux set-hook [-agRu] [-t target-session] hook-name command
/// ```
///
/// tmux 2.6:
/// ```text
/// tmux set-hook [-gu] [-t target-session] hook-name command
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Clone, Debug)]
pub struct SetHook<'a> {
    /// [-a] - append to a hook
    pub append: Option<bool>,
    /// [-g] - add hook-name to the global list of hooks
    pub global: Option<bool>,
    /// [-R] - run hook-name immediately
    pub run: Option<bool>,
    /// [-u] - unset
    pub unset: Option<bool>,
    /// [-t target-session] - target-session
    pub target_session: Option<&'a str>,
    // hook-name
    //pub hook_name: &'a str,
    // command
    //pub command: &'a str,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Clone, Debug)]
pub struct SetHook<'a> {
    /// [-g] - add hook-name to the global list of hooks
    pub global: Option<bool>,
    /// [-u] - unset
    pub unset: Option<bool>,
    /// [-t target-session] - target-session
    pub target_session: Option<&'a str>,
    // hook-name
    //pub hook_name: &'a str,
    // command
    //pub command: &'a str,
}
impl<'a> SetHook<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a> TmuxInterface<'a> {
    const SET_HOOK: &'static str = "set-hook";

    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux set-hook [-agRu] [-t target-session] hook-name command
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux set-hook [-gu] [-t target-session] hook-name command
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn set_hook(
        &mut self,
        set_hook: Option<&SetHook>,
        hook_name: &str,
        command: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(set_hook) = set_hook {
            if set_hook.append.unwrap_or(false) {
                args.push(a_KEY);
            }
            if set_hook.global.unwrap_or(false) {
                args.push(g_KEY);
            }
            if set_hook.run.unwrap_or(false) {
                args.push(R_KEY);
            }
            if set_hook.unset.unwrap_or(false) {
                args.push(u_KEY);
            }
            if let Some(s) = set_hook.target_session {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        args.push(hook_name);
        args.push(command);
        let output = self.subcommand(TmuxInterface::SET_HOOK, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux set-hook [-agRu] [-t target-session] hook-name command
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux set-hook [-gu] [-t target-session] hook-name command
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn set_hook(
        &mut self,
        set_hook: Option<&SetHook>,
        hook_name: &str,
        command: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(set_hook) = set_hook {
            if set_hook.global.unwrap_or(false) {
                args.push(g_KEY);
            }
            if set_hook.unset.unwrap_or(false) {
                args.push(u_KEY);
            }
            if let Some(s) = set_hook.target_session {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        args.push(hook_name);
        args.push(command);
        let output = self.subcommand(TmuxInterface::SET_HOOK, &args)?;
        Ok(output)
    }
}