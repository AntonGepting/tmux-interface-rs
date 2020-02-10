use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Structure for setting a pane/window/session/server option
///
/// # Manual
///
/// tmux X.X:
/// ```text
/// tmux set-option [-aFgopqsuw] [-t target-pane] option value
/// (alias: set)
/// ```
///
/// tmux 2.6:
/// ```text
/// tmux set-option [-aFgoqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct SetOption<'a, T: Display> {
    /// [-a] - value is appended to the existing setting, if the option expects a string or a style
    pub append: Option<bool>,
    /// [-F] - expand formats in the option value
    pub format: Option<bool>,
    /// [-g] - the global session or window option is set
    pub global: Option<bool>,
    /// [-o] - prevents setting an option that is already set
    pub not_overwrite: Option<bool>,
    /// [-p] - set a pane option
    pub pane: Option<bool>,
    /// [-q] - suppress errors about unknown or ambiguous options
    pub quiet: Option<bool>,
    /// [-s] - set a server option
    pub server: Option<bool>,
    /// [-u] - unset an option, so a session inherits the option from the global options
    pub unset: Option<bool>,
    /// [-w] - set a window option
    pub window: Option<bool>,
    /// [-t target-pane] - specify the target-pane
    pub target: Option<&'a T>,
    // option
    //pub option: &'a str,
    // value
    //pub value: &'a str,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct SetOption<'a, T: Display> {
    /// [-a] - value is appended to the existing setting, if the option expects a string or a style
    pub append: Option<bool>,
    /// [-F] - expand formats in the option value
    pub format: Option<bool>,
    /// [-g] - the global session or window option is set
    pub global: Option<bool>,
    /// [-o] - prevents setting an option that is already set
    pub not_overwrite: Option<bool>,
    /// [-q] - suppress errors about unknown or ambiguous options
    pub quiet: Option<bool>,
    /// [-s] - set a server option
    pub server: Option<bool>,
    /// [-u] - unset an option, so a session inherits the option from the global options
    pub unset: Option<bool>,
    /// [-w] - set a window option
    pub window: Option<bool>,
    /// [-t target-pane] - specify the target-pane
    pub target: Option<&'a T>,
    // option
    //pub option: &'a str,
    // value
    //pub value: &'a str,
}

impl<'a, T: Display + Default> SetOption<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct SetOptionBuilder<'a, T: Display> {
    pub append: Option<bool>,
    pub format: Option<bool>,
    pub global: Option<bool>,
    pub not_overwrite: Option<bool>,
    pub pane: Option<bool>,
    pub quiet: Option<bool>,
    pub server: Option<bool>,
    pub unset: Option<bool>,
    pub window: Option<bool>,
    pub target: Option<&'a T>,
    //pub option: &'a str,
    //pub value: &'a str,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a, T: Display + Default> SetOptionBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn append(&mut self) -> &mut Self {
        self.append = Some(true);
        self
    }

    pub fn format(&mut self) -> &mut Self {
        self.format = Some(true);
        self
    }

    pub fn global(&mut self) -> &mut Self {
        self.global = Some(true);
        self
    }

    pub fn not_overwrite(&mut self) -> &mut Self {
        self.not_overwrite = Some(true);
        self
    }

    pub fn pane(&mut self) -> &mut Self {
        self.pane = Some(true);
        self
    }

    pub fn quiet(&mut self) -> &mut Self {
        self.quiet = Some(true);
        self
    }

    pub fn server(&mut self) -> &mut Self {
        self.server = Some(true);
        self
    }

    pub fn unset(&mut self) -> &mut Self {
        self.unset = Some(true);
        self
    }

    pub fn window(&mut self) -> &mut Self {
        self.window = Some(true);
        self
    }

    pub fn target(&mut self, target: &'a T) -> &mut Self {
        self.target = Some(target);
        self
    }

    pub fn build(&self) -> SetOption<'a, T> {
        SetOption {
            append: self.append,
            format: self.format,
            global: self.global,
            not_overwrite: self.not_overwrite,
            pane: self.pane,
            quiet: self.quiet,
            server: self.server,
            unset: self.unset,
            window: self.window,
            target: self.target,
        }
    }
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct SetOptionBuilder<'a, T: Display> {
    pub append: Option<bool>,
    pub format: Option<bool>,
    pub global: Option<bool>,
    pub not_overwrite: Option<bool>,
    pub quiet: Option<bool>,
    pub server: Option<bool>,
    pub unset: Option<bool>,
    pub window: Option<bool>,
    pub target: Option<&'a T>,
    //pub option: &'a str,
    //pub value: &'a str,
}

#[cfg(feature = "tmux_2_6")]
impl<'a, T: Display + Default> SetOptionBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn append(&mut self) -> &mut Self {
        self.append = Some(true);
        self
    }

    pub fn format(&mut self) -> &mut Self {
        self.format = Some(true);
        self
    }

    pub fn global(&mut self) -> &mut Self {
        self.global = Some(true);
        self
    }

    pub fn not_overwrite(&mut self) -> &mut Self {
        self.not_overwrite = Some(true);
        self
    }

    pub fn quiet(&mut self) -> &mut Self {
        self.quiet = Some(true);
        self
    }

    pub fn server(&mut self) -> &mut Self {
        self.server = Some(true);
        self
    }

    pub fn unset(&mut self) -> &mut Self {
        self.unset = Some(true);
        self
    }

    pub fn window(&mut self) -> &mut Self {
        self.window = Some(true);
        self
    }

    pub fn target(&mut self, target: &'a T) -> &mut Self {
        self.target = Some(target);
        self
    }

    pub fn build(&self) -> SetOption<'a, T> {
        SetOption {
            append: self.append,
            format: self.format,
            global: self.global,
            not_overwrite: self.not_overwrite,
            quiet: self.quiet,
            server: self.server,
            unset: self.unset,
            window: self.window,
            target: self.target,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const SET_OPTION: &'static str = "set-option";

    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux set-option [-aFgopqsuw] [-t target-pane] option value
    /// (alias: set)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux set-option [-aFgoqsuw] [-t target-session | target-window] option value
    /// (alias: set)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn set_option<T: Display>(
        &mut self,
        set_option: Option<&SetOption<T>>,
        option: &str,
        value: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(set_option) = set_option {
            if set_option.append.unwrap_or(false) {
                args.push(a_KEY);
            }
            if set_option.format.unwrap_or(false) {
                args.push(F_KEY);
            }
            if set_option.global.unwrap_or(false) {
                args.push(g_KEY);
            }
            if set_option.not_overwrite.unwrap_or(false) {
                args.push(o_KEY);
            }
            if set_option.pane.unwrap_or(false) {
                args.push(p_KEY);
            }
            if set_option.quiet.unwrap_or(false) {
                args.push(q_KEY);
            }
            if set_option.server.unwrap_or(false) {
                args.push(s_KEY);
            }
            if set_option.unset.unwrap_or(false) {
                args.push(u_KEY);
            }
            if set_option.window.unwrap_or(false) {
                args.push(w_KEY);
            }
            if let Some(target) = set_option.target {
                s = target.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        args.push(option);
        args.push(value);
        let output = self.subcommand(TmuxInterface::SET_OPTION, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux set-option [-aFgopqsuw] [-t target-pane] option value
    /// (alias: set)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux set-option [-aFgoqsuw] [-t target-session | target-window] option value
    /// (alias: set)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn set_option<T: Display>(
        &mut self,
        set_option: Option<&SetOption<T>>,
        option: &str,
        value: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(set_option) = set_option {
            if set_option.append.unwrap_or(false) {
                args.push(a_KEY);
            }
            if set_option.format.unwrap_or(false) {
                args.push(F_KEY);
            }
            if set_option.global.unwrap_or(false) {
                args.push(g_KEY);
            }
            if set_option.not_overwrite.unwrap_or(false) {
                args.push(o_KEY);
            }
            if set_option.quiet.unwrap_or(false) {
                args.push(q_KEY);
            }
            if set_option.server.unwrap_or(false) {
                args.push(s_KEY);
            }
            if set_option.unset.unwrap_or(false) {
                args.push(u_KEY);
            }
            if set_option.window.unwrap_or(false) {
                args.push(w_KEY);
            }
            if let Some(target) = set_option.target {
                s = target.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        args.push(option);
        args.push(value);
        let output = self.subcommand(TmuxInterface::SET_OPTION, &args)?;
        Ok(output)
    }
}
