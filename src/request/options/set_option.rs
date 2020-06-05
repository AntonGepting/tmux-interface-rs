use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure for setting a pane/window/session/server option
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux set-option [-aFgopqsuw] [-t target-pane] option value
/// (alias: set)
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux set-option [-aFgoqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux ^1.8:
/// ```text
/// tmux set-option [-agoqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux set-option [-agqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux set-option [-agsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux set-option [-agu] [-t target-session] option value
/// (alias: set)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux set-option [-gu] [-t target-session] option value
/// (alias: set)
/// ```
#[derive(Default, Debug)]
pub struct SetOption<'a> {
    /// [-a] - value is appended to the existing setting, if the option expects a string or a style
    #[cfg(feature = "tmux_1_0")]
    pub append: Option<bool>,
    /// [-F] - expand formats in the option value
    #[cfg(feature = "tmux_2_6")]
    pub format: Option<bool>,
    /// [-g] - the global session or window option is set
    #[cfg(feature = "tmux_0_8")]
    pub global: Option<bool>,
    /// [-o] - prevents setting an option that is already set
    #[cfg(feature = "tmux_1_8")]
    pub not_overwrite: Option<bool>,
    /// [-p] - set a pane option
    #[cfg(feature = "tmux_3_0")]
    pub pane: Option<bool>,
    /// [-q] - suppress errors about unknown or ambiguous options
    #[cfg(feature = "tmux_1_7")]
    pub quiet: Option<bool>,
    /// [-s] - set a server option
    #[cfg(feature = "tmux_1_2")]
    pub server: Option<bool>,
    /// [-u] - unset an option, so a session inherits the option from the global options
    #[cfg(feature = "tmux_0_8")]
    pub unset: Option<bool>,
    /// [-w] - set a window option
    #[cfg(feature = "tmux_1_2")]
    pub window: Option<bool>,
    /// [-t target-pane] - specify the target-pane
    #[cfg(feature = "tmux_3_0")]
    pub target_pane: Option<&'a str>,
    /// [-t target-session | target-window]
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    pub target: Option<&'a str>,
    /// [-t target-session]
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
    pub target_session: Option<&'a str>,
    // option
    //pub option: &'a str,
    // value
    //pub value: &'a str,
}

impl<'a> SetOption<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct SetOptionBuilder<'a> {
    #[cfg(feature = "tmux_1_0")]
    pub append: Option<bool>,
    #[cfg(feature = "tmux_2_6")]
    pub format: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub global: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub not_overwrite: Option<bool>,
    #[cfg(feature = "tmux_3_0")]
    pub pane: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub quiet: Option<bool>,
    #[cfg(feature = "tmux_1_2")]
    pub server: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub unset: Option<bool>,
    #[cfg(feature = "tmux_1_2")]
    pub window: Option<bool>,
    #[cfg(feature = "tmux_3_0")]
    pub target_pane: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    pub target: Option<&'a str>,
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
    pub target_session: Option<&'a str>,
    //pub option: &'a str,
    //pub value: &'a str,
}

impl<'a> SetOptionBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn append(&mut self) -> &mut Self {
        self.append = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn format(&mut self) -> &mut Self {
        self.format = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn global(&mut self) -> &mut Self {
        self.global = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn not_overwrite(&mut self) -> &mut Self {
        self.not_overwrite = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn pane(&mut self) -> &mut Self {
        self.pane = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn quiet(&mut self) -> &mut Self {
        self.quiet = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn server(&mut self) -> &mut Self {
        self.server = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn unset(&mut self) -> &mut Self {
        self.unset = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn window(&mut self) -> &mut Self {
        self.window = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn target_pane(&mut self, target_pane: &'a str) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    pub fn target(&mut self, target: &'a str) -> &mut Self {
        self.target = Some(target);
        self
    }
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
    pub fn target_session(&mut self, target_session: &'a str) -> &mut Self {
        self.target_session = Some(target_session);
        self
    }

    pub fn build(&self) -> SetOption<'a> {
        SetOption {
            #[cfg(feature = "tmux_1_0")]
            append: self.append,
            #[cfg(feature = "tmux_2_6")]
            format: self.format,
            #[cfg(feature = "tmux_0_8")]
            global: self.global,
            #[cfg(feature = "tmux_1_8")]
            not_overwrite: self.not_overwrite,
            #[cfg(feature = "tmux_3_0")]
            pane: self.pane,
            #[cfg(feature = "tmux_1_7")]
            quiet: self.quiet,
            #[cfg(feature = "tmux_1_2")]
            server: self.server,
            #[cfg(feature = "tmux_0_8")]
            unset: self.unset,
            #[cfg(feature = "tmux_1_2")]
            window: self.window,
            #[cfg(feature = "tmux_3_0")]
            target_pane: self.target,
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
            target: self.target,
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
            target_session: self.target,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const SET_OPTION: &'static str = "set-option";

    /// # Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// tmux set-option [-aFgopqsuw] [-t target-pane] option value
    /// (alias: set)
    /// ```
    ///
    /// tmux ^2.6:
    /// ```text
    /// tmux set-option [-aFgoqsuw] [-t target-session | target-window] option value
    /// (alias: set)
    /// ```
    ///
    /// tmux ^1.8:
    /// ```text
    /// tmux set-option [-agoqsuw] [-t target-session | target-window] option value
    /// (alias: set)
    /// ```
    ///
    /// tmux ^1.7:
    /// ```text
    /// tmux set-option [-agqsuw] [-t target-session | target-window] option value
    /// (alias: set)
    /// ```
    ///
    /// tmux ^1.2:
    /// ```text
    /// tmux set-option [-agsuw] [-t target-session | target-window] option value
    /// (alias: set)
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux set-option [-agu] [-t target-session] option value
    /// (alias: set)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux set-option [-gu] [-t target-session] option value
    /// (alias: set)
    /// ```
    pub fn set_option(
        &mut self,
        set_option: Option<&SetOption>,
        option: &str,
        value: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(set_option) = set_option {
            #[cfg(feature = "tmux_1_0")]
            if set_option.append.unwrap_or(false) {
                args.push(a_KEY);
            }
            #[cfg(feature = "tmux_2_6")]
            if set_option.format.unwrap_or(false) {
                args.push(F_KEY);
            }
            #[cfg(feature = "tmux_0_8")]
            if set_option.global.unwrap_or(false) {
                args.push(g_KEY);
            }
            #[cfg(feature = "tmux_1_8")]
            if set_option.not_overwrite.unwrap_or(false) {
                args.push(o_KEY);
            }
            #[cfg(feature = "tmux_3_0")]
            if set_option.pane.unwrap_or(false) {
                args.push(p_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if set_option.quiet.unwrap_or(false) {
                args.push(q_KEY);
            }
            #[cfg(feature = "tmux_1_2")]
            if set_option.server.unwrap_or(false) {
                args.push(s_KEY);
            }
            #[cfg(feature = "tmux_0_8")]
            if set_option.unset.unwrap_or(false) {
                args.push(u_KEY);
            }
            #[cfg(feature = "tmux_1_2")]
            if set_option.window.unwrap_or(false) {
                args.push(w_KEY);
            }
            #[cfg(feature = "tmux_3_0")]
            if let Some(target_pane) = set_option.target_pane {
                args.extend_from_slice(&[t_KEY, &target])
            }
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
            if let Some(target) = set_option.target {
                args.extend_from_slice(&[t_KEY, &target])
            }
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
            if let Some(target_session) = set_option.target_session {
                args.extend_from_slice(&[t_KEY, &target])
            }
        }
        args.push(option);
        args.push(value);
        let output = self.subcommand(TmuxInterface::SET_OPTION, &args)?;
        Ok(output)
    }
}
