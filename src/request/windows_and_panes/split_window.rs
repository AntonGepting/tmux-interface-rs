use crate::error::Error;
use crate::tmux_interface::*;
use crate::PaneSize;
use std::fmt::Display;

/// Create a new pane by splitting target-pane
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux ^3.0:
/// ```text
/// tmux split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size | -p percentage]
/// [-t target-pane] [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux ^2.4:
/// ```text
/// tmux split-window [-bdfhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux ^2.0:
/// ```text
/// tmux split-window [-bdhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux split-window [-dhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux split-window [-dhvP] [-l size | -p percentage] [-t target-pane] [shell-command]
/// (alias: splitw)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux split-window [-dhv] [-l size | -p percentage] [-t target-pane] [shell-command]
/// (alias: splitw)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux split-window [-dhv] [-l size | -p percentage] [-t target-window] [command]
/// (alias: splitw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux split-window [-d] [-l size | -p percentage] [-t target-window] [command]
/// (alias: splitw)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct SplitWindow<'a, T: Display> {
    /// [-b] - cause the new pane to be created to the left of or above target-pane
    pub before: Option<bool>,
    /// [-d] - do not make the new window the current window
    pub detached: Option<bool>,
    /// [-f] - creates a new pane spanning the full window size (h, v)
    pub full: Option<bool>,
    /// [-h] - horizontal split
    pub horizontal: Option<bool>,
    /// [-I] - create an empty pane and forward any output from stdin to it
    pub stdin_forward: Option<bool>,
    /// [-v] - vertical split
    pub vertical: Option<bool>,
    /// [-P] - print information about the new window after it has been created
    pub print: Option<bool>,
    /// [-c start_directory] - start-directory
    pub cwd: Option<&'a str>,
    /// [-e environment] - environment
    pub environment: Option<&'a str>,
    /// [-l size] - specify the size of the new pane in lines
    pub size: Option<&'a PaneSize>,
    /// [-t target-pane] -
    pub target_pane: Option<&'a T>,
    /// [shell-command] - shell-command
    pub shell_command: Option<&'a str>,
    /// [-F format] - format
    pub format: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct SplitWindow<'a, T: Display> {
    /// [-b] - cause the new pane to be created to the left of or above target-pane
    pub before: Option<bool>,
    /// [-d] - do not make the new window the current window
    pub detached: Option<bool>,
    /// [-f] - creates a new pane spanning the full window size (h, v)
    pub full: Option<bool>,
    /// [-h] - horizontal split
    pub horizontal: Option<bool>,
    /// [-v] - vertical split
    pub vertical: Option<bool>,
    /// [-P] - print information about the new window after it has been created
    pub print: Option<bool>,
    /// [-c start_directory] - start-directory
    pub cwd: Option<&'a str>,
    /// [-l size | -p percentage] - specify the size of the new pane in lines
    pub size: Option<&'a PaneSize>,
    /// [-t target-pane] -
    pub target_pane: Option<&'a T>,
    /// [shell-command] - shell-command
    pub shell_command: Option<&'a str>,
    /// [-F format] - format
    pub format: Option<&'a str>,
}

impl<'a, T: Display + Default> SplitWindow<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct SplitWindowBuilder<'a, T: Display> {
    pub before: Option<bool>,
    pub detached: Option<bool>,
    pub full: Option<bool>,
    pub horizontal: Option<bool>,
    pub stdin_forward: Option<bool>,
    pub vertical: Option<bool>,
    pub print: Option<bool>,
    pub cwd: Option<&'a str>,
    pub environment: Option<&'a str>,
    pub size: Option<&'a PaneSize>,
    pub target_pane: Option<&'a T>,
    pub shell_command: Option<&'a str>,
    pub format: Option<&'a str>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a, T: Display + Default> SplitWindowBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn before(&mut self) -> &mut Self {
        self.before = Some(true);
        self
    }

    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    pub fn full(&mut self) -> &mut Self {
        self.full = Some(true);
        self
    }

    pub fn horizontal(&mut self) -> &mut Self {
        self.horizontal = Some(true);
        self
    }

    pub fn stdin_forward(&mut self) -> &mut Self {
        self.stdin_forward = Some(true);
        self
    }

    pub fn vertical(&mut self) -> &mut Self {
        self.vertical = Some(true);
        self
    }

    pub fn print(&mut self) -> &mut Self {
        self.print = Some(true);
        self
    }

    pub fn cwd(&mut self, cwd: &'a str) -> &mut Self {
        self.cwd = Some(cwd);
        self
    }

    pub fn environment(&mut self, environment: &'a str) -> &mut Self {
        self.environment = Some(environment);
        self
    }

    pub fn size(&mut self, size: &'a PaneSize) -> &mut Self {
        self.size = Some(size);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn shell_command(&mut self, shell_command: &'a str) -> &mut Self {
        self.shell_command = Some(shell_command);
        self
    }

    pub fn format(&mut self, format: &'a str) -> &mut Self {
        self.format = Some(format);
        self
    }

    pub fn build(&self) -> SplitWindow<'a, T> {
        SplitWindow {
            before: self.before,
            detached: self.detached,
            full: self.full,
            horizontal: self.horizontal,
            stdin_forward: self.stdin_forward,
            vertical: self.vertical,
            print: self.print,
            cwd: self.cwd,
            environment: self.environment,
            size: self.size,
            target_pane: self.target_pane,
            shell_command: self.shell_command,
            format: self.format,
        }
    }
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct SplitWindowBuilder<'a, T: Display> {
    pub before: Option<bool>,
    pub detached: Option<bool>,
    pub full: Option<bool>,
    pub horizontal: Option<bool>,
    pub vertical: Option<bool>,
    pub print: Option<bool>,
    pub cwd: Option<&'a str>,
    pub size: Option<&'a PaneSize>,
    pub target_pane: Option<&'a T>,
    pub shell_command: Option<&'a str>,
    pub format: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
impl<'a, T: Display + Default> SplitWindowBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn before(&mut self) -> &mut Self {
        self.before = Some(true);
        self
    }

    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    pub fn full(&mut self) -> &mut Self {
        self.full = Some(true);
        self
    }

    pub fn horizontal(&mut self) -> &mut Self {
        self.horizontal = Some(true);
        self
    }

    pub fn vertical(&mut self) -> &mut Self {
        self.vertical = Some(true);
        self
    }

    pub fn print(&mut self) -> &mut Self {
        self.print = Some(true);
        self
    }

    pub fn cwd(&mut self, cwd: &'a str) -> &mut Self {
        self.cwd = Some(cwd);
        self
    }

    pub fn size(&mut self, size: &'a PaneSize) -> &mut Self {
        self.size = Some(size);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn shell_command(&mut self, shell_command: &'a str) -> &mut Self {
        self.shell_command = Some(shell_command);
        self
    }

    pub fn format(&mut self, format: &'a str) -> &mut Self {
        self.format = Some(format);
        self
    }

    pub fn build(&self) -> SplitWindow<'a, T> {
        SplitWindow {
            before: self.before,
            detached: self.detached,
            full: self.full,
            horizontal: self.horizontal,
            vertical: self.vertical,
            print: self.print,
            cwd: self.cwd,
            size: self.size,
            target_pane: self.target_pane,
            shell_command: self.shell_command,
            format: self.format,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const SPLIT_WINDOW: &'static str = "split-window";

    /// Create a new pane by splitting `target-pane`
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size] [-t target-pane]
    /// [shell-command] [-F format]
    /// (alias: splitw)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux split-window [-bdfhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
    /// [shell-command] [-F format]
    /// (alias: splitw)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn split_window<T: Display>(
        &mut self,
        split_window: Option<&SplitWindow<T>>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let d;
        if let Some(split_window) = split_window {
            if split_window.before.unwrap_or(false) {
                args.push(b_KEY);
            }
            if split_window.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if split_window.full.unwrap_or(false) {
                args.push(f_KEY);
            }
            if split_window.horizontal.unwrap_or(false) {
                args.push(h_KEY);
            }
            if split_window.stdin_forward.unwrap_or(false) {
                args.push(I_KEY);
            }
            if split_window.vertical.unwrap_or(false) {
                args.push(v_KEY);
            }
            if split_window.print.unwrap_or(false) {
                args.push(P_KEY);
            }
            if let Some(s) = split_window.cwd {
                args.extend_from_slice(&[c_KEY, &s]);
            }
            if let Some(s) = split_window.environment {
                args.extend_from_slice(&[e_KEY, &s]);
            }
            if let Some(size) = &split_window.size {
                match size {
                    PaneSize::Size(size) => d = size.to_string(),
                    PaneSize::Percentage(size) => d = format!("{}%", size),
                };
                args.extend_from_slice(&[l_KEY, &d]);
            }
            if let Some(target_pane) = split_window.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = split_window.shell_command {
                args.push(&s)
            }
            if let Some(s) = split_window.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::SPLIT_WINDOW, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }

    /// Create a new pane by splitting `target-pane`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size]
    /// [-t target-pane] [shell-command] [-F format]
    /// (alias: splitw)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn split_window<T: Display>(
        &mut self,
        split_window: Option<&SplitWindow<T>>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let d;
        if let Some(split_window) = split_window {
            if split_window.before.unwrap_or(false) {
                args.push(b_KEY);
            }
            if split_window.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if split_window.full.unwrap_or(false) {
                args.push(f_KEY);
            }
            if split_window.horizontal.unwrap_or(false) {
                args.push(h_KEY);
            }
            if split_window.vertical.unwrap_or(false) {
                args.push(v_KEY);
            }
            if split_window.print.unwrap_or(false) {
                args.push(P_KEY);
            }
            if let Some(s) = split_window.cwd {
                args.extend_from_slice(&[c_KEY, &s]);
            }
            if let Some(size) = &split_window.size {
                match size {
                    PaneSize::Size(size) => {
                        d = size.to_string();
                        args.extend_from_slice(&[l_KEY, &d]);
                    }
                    PaneSize::Percentage(size) => {
                        d = size.to_string();
                        args.extend_from_slice(&[p_KEY, &d]);
                    }
                };
            }
            if let Some(target_pane) = split_window.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = split_window.shell_command {
                args.push(&s)
            }
            if let Some(s) = split_window.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::SPLIT_WINDOW, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }
}
