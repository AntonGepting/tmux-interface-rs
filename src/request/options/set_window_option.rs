use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
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
pub struct SetWindowOption<'a, T: Display> {
    /// [-a] -
    pub append: Option<bool>,
    /// [-F] -
    pub format: Option<bool>,
    /// [-g] -
    pub global: Option<bool>,
    /// [-o] -
    pub not_overwrite: Option<bool>,
    /// [-q] -
    pub quiet: Option<bool>,
    /// [-u] -
    pub unset: Option<bool>,
    /// [-t target-window] -
    pub target_window: Option<&'a T>,
    //pub option: &'a str,                // option
    //pub value: &'a str,                 // value
}

#[derive(Default, Debug)]
pub struct SetWindowOptionBuilder<'a, T: Display> {
    pub append: Option<bool>,
    pub format: Option<bool>,
    pub global: Option<bool>,
    pub not_overwrite: Option<bool>,
    pub quiet: Option<bool>,
    pub unset: Option<bool>,
    pub target_window: Option<&'a T>,
    //pub option: &'a str,                // option
    //pub value: &'a str,                 // value
}

impl<'a, T: Display + Default> SetWindowOptionBuilder<'a, T> {
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

    pub fn unset(&mut self) -> &mut Self {
        self.unset = Some(true);
        self
    }

    pub fn target_window(&mut self, target_window: &'a T) -> &mut Self {
        self.target_window = Some(target_window);
        self
    }

    pub fn build(&self) -> SetWindowOption<'a, T> {
        SetWindowOption {
            append: self.append,
            format: self.format,
            global: self.global,
            not_overwrite: self.not_overwrite,
            quiet: self.quiet,
            unset: self.unset,
            target_window: self.target_window,
        }
    }
}

impl<'a, T: Display + Default> SetWindowOption<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a> TmuxInterface<'a> {
    const SET_WINDOW_OPTION: &'static str = "set-window-option";

    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// (removed)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux set-window-option [-aFgoqu] [-t target-window] option value
    /// (alias: setw)
    /// ```
    pub fn set_window_option<T: Display>(
        &mut self,
        set_window_option: Option<&SetWindowOption<T>>,
        option: &str,
        value: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(set_window_option) = set_window_option {
            if set_window_option.append.unwrap_or(false) {
                args.push(a_KEY);
            }
            if set_window_option.format.unwrap_or(false) {
                args.push(F_KEY);
            }
            if set_window_option.global.unwrap_or(false) {
                args.push(g_KEY);
            }
            if set_window_option.not_overwrite.unwrap_or(false) {
                args.push(o_KEY);
            }
            if set_window_option.quiet.unwrap_or(false) {
                args.push(q_KEY);
            }
            if set_window_option.unset.unwrap_or(false) {
                args.push(u_KEY);
            }
            if let Some(target_window) = set_window_option.target_window {
                s = target_window.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        args.push(option);
        args.push(value);
        let output = self.subcommand(TmuxInterface::SET_WINDOW_OPTION, &args)?;
        Ok(output)
    }
}
