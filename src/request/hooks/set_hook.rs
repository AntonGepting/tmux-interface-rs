use crate::error::Error;
use crate::tmux_interface::*;
use crate::TargetSession;
use std::process::Output;

/// Structure for setting or unsetting hook `hook-name` to command.
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux set-hook [-agRu] [-t target-session] hook-name command
/// ```
///
/// tmux ^2.8:
/// ```text
/// tmux set-hook [-gRu] [-t target-session] hook-name command
/// ```
///
/// tmux ^2.4:
/// ```text
/// tmux set-hook [-gu] [-t target-session] hook-name command
/// ```
///
/// tmux ^2.2:
/// ```text
/// tmux set-hook [-g] [-t target-session] hook-name command
/// ```
#[derive(Default, Clone, Debug)]
pub struct SetHook<'a> {
    /// [-a] - append to a hook
    #[cfg(feature = "tmux_3_0")]
    pub append: Option<bool>,
    /// [-g] - add hook-name to the global list of hooks
    #[cfg(feature = "tmux_2_2")]
    pub global: Option<bool>,
    /// [-R] - run hook-name immediately
    #[cfg(feature = "tmux_2_8")]
    pub run: Option<bool>,
    /// [-u] - unset
    #[cfg(feature = "tmux_2_4")]
    pub unset: Option<bool>,
    /// [-t target-session] - target-session
    #[cfg(feature = "tmux_2_2")]
    pub target_session: Option<&'a TargetSession<'a>>,
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

#[derive(Default, Clone, Debug)]
pub struct SetHookBuilder<'a> {
    #[cfg(feature = "tmux_3_0")]
    pub append: Option<bool>,
    #[cfg(feature = "tmux_2_2")]
    pub global: Option<bool>,
    #[cfg(feature = "tmux_2_8")]
    pub run: Option<bool>,
    #[cfg(feature = "tmux_2_4")]
    pub unset: Option<bool>,
    #[cfg(feature = "tmux_2_2")]
    pub target_session: Option<&'a TargetSession<'a>>,
    //pub hook_name: &'a str,
    //pub command: &'a str,
}

impl<'a> SetHookBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn append(&mut self) -> &mut Self {
        self.append = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_2")]
    pub fn global(&mut self) -> &mut Self {
        self.global = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_8")]
    pub fn run(&mut self) -> &mut Self {
        self.run = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn unset(&mut self) -> &mut Self {
        self.unset = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_2")]
    pub fn target_session(&mut self, target_session: &'a TargetSession<'a>) -> &mut Self {
        self.target_session = Some(target_session);
        self
    }

    pub fn build(&self) -> SetHook<'a> {
        SetHook {
            #[cfg(feature = "tmux_3_0")]
            append: self.append,
            #[cfg(feature = "tmux_2_2")]
            global: self.global,
            #[cfg(feature = "tmux_2_8")]
            run: self.run,
            #[cfg(feature = "tmux_2_4")]
            unset: self.unset,
            #[cfg(feature = "tmux_2_2")]
            target_session: self.target_session,
            //hook_name: &'a str,
            //command: &'a str,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const SET_HOOK: &'static str = "set-hook";

    /// # Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// tmux set-hook [-agRu] [-t target-session] hook-name command
    /// ```
    ///
    /// tmux ^2.8:
    /// ```text
    /// tmux set-hook [-gRu] [-t target-session] hook-name command
    /// ```
    ///
    /// tmux ^2.4:
    /// ```text
    /// tmux set-hook [-gu] [-t target-session] hook-name command
    /// ```
    ///
    /// tmux ^2.2:
    /// ```text
    /// tmux set-hook [-g] [-t target-session] hook-name command
    /// ```
    pub fn set_hook(
        &mut self,
        set_hook: Option<&SetHook>,
        hook_name: &str,
        command: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(set_hook) = set_hook {
            #[cfg(feature = "tmux_3_0")]
            if set_hook.append.unwrap_or(false) {
                args.push(a_KEY);
            }
            #[cfg(feature = "tmux_2_2")]
            if set_hook.global.unwrap_or(false) {
                args.push(g_KEY);
            }
            #[cfg(feature = "tmux_2_8")]
            if set_hook.run.unwrap_or(false) {
                args.push(R_KEY);
            }
            #[cfg(feature = "tmux_2_4")]
            if set_hook.unset.unwrap_or(false) {
                args.push(u_KEY);
            }
            #[cfg(feature = "tmux_2_2")]
            if let Some(target_session) = set_hook.target_session {
                s = target_session.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        args.push(hook_name);
        args.push(command);
        let output = self.subcommand(TmuxInterface::SET_HOOK, &args)?;
        Ok(output)
    }
}
