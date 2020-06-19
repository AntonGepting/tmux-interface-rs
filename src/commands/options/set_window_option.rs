use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// # Manual
///
/// tmux ^3.0:
/// ```text
/// (removed)
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux set-window-option [-aFgoqu] [-t target-window] option value
/// (alias: setw)
/// ```
///
/// tmux ^1.9:
/// ```text
/// tmux set-window-option [-agoqu] [-t target-window] option value
/// (alias: setw)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux set-window-option [-agqu] [-t target-window] option value
/// (alias: setw)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux set-window-option [-agu] [-t target-window] option value
/// (alias: setw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux set-window-option [-gu] [-t target-window] option value
/// (alias: setw)
/// ```
#[derive(Default, Debug)]
pub struct SetWindowOption<'a> {
    /// [-a] -
    #[cfg(feature = "tmux_1_0")]
    pub append: Option<bool>,
    /// [-F] -
    #[cfg(feature = "tmux_2_6")]
    pub format: Option<bool>,
    /// [-g] -
    #[cfg(feature = "tmux_0_8")]
    pub global: Option<bool>,
    /// [-o] -
    #[cfg(feature = "tmux_1_9")]
    pub not_overwrite: Option<bool>,
    /// [-q] -
    #[cfg(feature = "tmux_1_7")]
    pub quiet: Option<bool>,
    /// [-u] -
    #[cfg(feature = "tmux_0_8")]
    pub unset: Option<bool>,
    /// [-t target-window] -
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<&'a str>,
    //pub option: &'a str,                // option
    //pub value: &'a str,                 // value
}

#[derive(Default, Debug)]
pub struct SetWindowOptionBuilder<'a> {
    #[cfg(feature = "tmux_1_0")]
    pub append: Option<bool>,
    #[cfg(feature = "tmux_2_6")]
    pub format: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub global: Option<bool>,
    #[cfg(feature = "tmux_1_9")]
    pub not_overwrite: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub quiet: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub unset: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<&'a str>,
    //pub option: &'a str,                // option
    //pub value: &'a str,                 // value
}

impl<'a> SetWindowOptionBuilder<'a> {
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

    #[cfg(feature = "tmux_1_9")]
    pub fn not_overwrite(&mut self) -> &mut Self {
        self.not_overwrite = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn quiet(&mut self) -> &mut Self {
        self.quiet = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn unset(&mut self) -> &mut Self {
        self.unset = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn target_window(&mut self, target_window: &'a str) -> &mut Self {
        self.target_window = Some(target_window);
        self
    }

    pub fn build(&self) -> SetWindowOption<'a> {
        SetWindowOption {
            #[cfg(feature = "tmux_1_0")]
            append: self.append,
            #[cfg(feature = "tmux_2_6")]
            format: self.format,
            #[cfg(feature = "tmux_0_8")]
            global: self.global,
            #[cfg(feature = "tmux_1_9")]
            not_overwrite: self.not_overwrite,
            #[cfg(feature = "tmux_1_7")]
            quiet: self.quiet,
            #[cfg(feature = "tmux_0_8")]
            unset: self.unset,
            #[cfg(feature = "tmux_0_8")]
            target_window: self.target_window,
        }
    }
}

impl<'a> SetWindowOption<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a> TmuxInterface<'a> {
    const SET_WINDOW_OPTION: &'static str = "set-window-option";

    /// # Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// (removed)
    /// ```
    ///
    /// tmux ^2.6:
    /// ```text
    /// tmux set-window-option [-aFgoqu] [-t target-window] option value
    /// (alias: setw)
    /// ```
    ///
    /// tmux ^1.9:
    /// ```text
    /// tmux set-window-option [-agoqu] [-t target-window] option value
    /// (alias: setw)
    /// ```
    ///
    /// tmux ^1.7:
    /// ```text
    /// tmux set-window-option [-agqu] [-t target-window] option value
    /// (alias: setw)
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux set-window-option [-agu] [-t target-window] option value
    /// (alias: setw)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux set-window-option [-gu] [-t target-window] option value
    /// (alias: setw)
    /// ```
    pub fn set_window_option(
        &mut self,
        set_window_option: Option<&SetWindowOption>,
        option: &str,
        value: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(set_window_option) = set_window_option {
            #[cfg(feature = "tmux_1_0")]
            if set_window_option.append.unwrap_or(false) {
                args.push(a_KEY);
            }
            #[cfg(feature = "tmux_2_6")]
            if set_window_option.format.unwrap_or(false) {
                args.push(F_KEY);
            }
            #[cfg(feature = "tmux_0_8")]
            if set_window_option.global.unwrap_or(false) {
                args.push(g_KEY);
            }
            #[cfg(feature = "tmux_1_9")]
            if set_window_option.not_overwrite.unwrap_or(false) {
                args.push(o_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if set_window_option.quiet.unwrap_or(false) {
                args.push(q_KEY);
            }
            #[cfg(feature = "tmux_0_8")]
            if set_window_option.unset.unwrap_or(false) {
                args.push(u_KEY);
            }
            #[cfg(feature = "tmux_0_8")]
            if let Some(target_window) = set_window_option.target_window {
                args.extend_from_slice(&[t_KEY, &target_window])
            }
        }
        args.push(option);
        args.push(value);
        let output = self.subcommand(TmuxInterface::SET_WINDOW_OPTION, &args)?;
        Ok(output)
    }
}
