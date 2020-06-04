use crate::error::Error;
use crate::tmux_interface::*;
use crate::PaneSize;

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
#[derive(Default, Debug)]
pub struct SplitWindow<'a> {
    /// [-b] - cause the new pane to be created to the left of or above target-pane
    #[cfg(feature = "tmux_2_4")]
    pub before: Option<bool>,
    /// [-d] - do not make the new window the current window
    #[cfg(feature = "tmux_0_8")]
    pub detached: Option<bool>,
    /// [-f] - creates a new pane spanning the full window size (h, v)
    #[cfg(feature = "tmux_2_4")]
    pub full: Option<bool>,
    /// [-h] - horizontal split
    #[cfg(feature = "tmux_1_0")]
    pub horizontal: Option<bool>,
    /// [-I] - create an empty pane and forward any output from stdin to it
    #[cfg(feature = "tmux_3_0")]
    pub stdin_forward: Option<bool>,
    /// [-v] - vertical split
    #[cfg(feature = "tmux_1_0")]
    pub vertical: Option<bool>,
    /// [-P] - print information about the new window after it has been created
    #[cfg(feature = "tmux_1_5")]
    pub print: Option<bool>,
    /// [-c start_directory] - start-directory
    #[cfg(feature = "tmux_1_7")]
    pub cwd: Option<&'a str>,
    /// [-e environment] - environment
    #[cfg(feature = "tmux_3_1")]
    pub environment: Option<&'a str>,
    /// [-l size] - specify the size of the new pane in lines
    /// [-l size | -p percentage] - specify the size of the new pane in lines
    #[cfg(feature = "tmux_0_8")]
    pub size: Option<&'a PaneSize>,
    /// [-t target-pane] -
    #[cfg(feature = "tmux_1_2")]
    pub target_pane: Option<&'a str>,
    /// [-t target-window] -
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
    pub target_window: Option<&'a str>,
    /// [shell-command] - shell-command
    #[cfg(feature = "tmux_1_2")]
    pub shell_command: Option<&'a str>,
    /// [-F format] - format
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<&'a str>,
}

impl<'a> SplitWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct SplitWindowBuilder<'a> {
    #[cfg(feature = "tmux_2_4")]
    pub before: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub detached: Option<bool>,
    #[cfg(feature = "tmux_2_4")]
    pub full: Option<bool>,
    #[cfg(feature = "tmux_1_0")]
    pub horizontal: Option<bool>,
    #[cfg(feature = "tmux_3_0")]
    pub stdin_forward: Option<bool>,
    #[cfg(feature = "tmux_1_0")]
    pub vertical: Option<bool>,
    #[cfg(feature = "tmux_1_5")]
    pub print: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub cwd: Option<&'a str>,
    #[cfg(feature = "tmux_3_1")]
    pub environment: Option<&'a str>,
    #[cfg(feature = "tmux_0_8")]
    pub size: Option<&'a PaneSize>,
    #[cfg(feature = "tmux_1_2")]
    pub target_pane: Option<&'a str>,
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
    pub target_window: Option<&'a str>,
    #[cfg(feature = "tmux_1_2")]
    pub shell_command: Option<&'a str>,
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<&'a str>,
}

impl<'a> SplitWindowBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn before(&mut self) -> &mut Self {
        self.before = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn full(&mut self) -> &mut Self {
        self.full = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn horizontal(&mut self) -> &mut Self {
        self.horizontal = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn stdin_forward(&mut self) -> &mut Self {
        self.stdin_forward = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn vertical(&mut self) -> &mut Self {
        self.vertical = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn print(&mut self) -> &mut Self {
        self.print = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn cwd(&mut self, cwd: &'a str) -> &mut Self {
        self.cwd = Some(cwd);
        self
    }

    #[cfg(feature = "tmux_3_1")]
    pub fn environment(&mut self, environment: &'a str) -> &mut Self {
        self.environment = Some(environment);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn size(&mut self, size: &'a PaneSize) -> &mut Self {
        self.size = Some(size);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn target_pane(&mut self, target_pane: &'a str) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn shell_command(&mut self, shell_command: &'a str) -> &mut Self {
        self.shell_command = Some(shell_command);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn format(&mut self, format: &'a str) -> &mut Self {
        self.format = Some(format);
        self
    }

    pub fn build(&self) -> SplitWindow<'a> {
        SplitWindow {
            #[cfg(feature = "tmux_2_4")]
            before: self.before,
            #[cfg(feature = "tmux_0_8")]
            detached: self.detached,
            #[cfg(feature = "tmux_2_4")]
            full: self.full,
            #[cfg(feature = "tmux_1_0")]
            horizontal: self.horizontal,
            #[cfg(feature = "tmux_3_0")]
            stdin_forward: self.stdin_forward,
            #[cfg(feature = "tmux_1_0")]
            vertical: self.vertical,
            #[cfg(feature = "tmux_1_5")]
            print: self.print,
            #[cfg(feature = "tmux_1_7")]
            cwd: self.cwd,
            #[cfg(feature = "tmux_3_1")]
            environment: self.environment,
            #[cfg(feature = "tmux_0_8")]
            size: self.size,
            #[cfg(feature = "tmux_1_2")]
            target_pane: self.target_pane,
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
            target_window: self.target_window,
            #[cfg(feature = "tmux_1_2")]
            shell_command: self.shell_command,
            #[cfg(feature = "tmux_1_7")]
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
    pub fn split_window(
        &mut self,
        split_window: Option<&SplitWindow>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        let d;

        if let Some(split_window) = split_window {
            #[cfg(feature = "tmux_2_4")]
            if split_window.before.unwrap_or(false) {
                args.push(b_KEY);
            }
            #[cfg(feature = "tmux_0_8")]
            if split_window.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            #[cfg(feature = "tmux_2_4")]
            if split_window.full.unwrap_or(false) {
                args.push(f_KEY);
            }
            #[cfg(feature = "tmux_1_0")]
            if split_window.horizontal.unwrap_or(false) {
                args.push(h_KEY);
            }
            #[cfg(feature = "tmux_3_0")]
            if split_window.stdin_forward.unwrap_or(false) {
                args.push(I_KEY);
            }
            #[cfg(feature = "tmux_1_0")]
            if split_window.vertical.unwrap_or(false) {
                args.push(v_KEY);
            }
            #[cfg(feature = "tmux_1_5")]
            if split_window.print.unwrap_or(false) {
                args.push(P_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if let Some(cwd) = split_window.cwd {
                args.extend_from_slice(&[c_KEY, &cwd]);
            }
            #[cfg(feature = "tmux_3_1")]
            if let Some(environment) = split_window.environment {
                args.extend_from_slice(&[e_KEY, &environment]);
            }
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_1")))]
            if let Some(size) = &split_window.size {
                match size {
                    PaneSize::Size(size) => d = size.to_string(),
                    PaneSize::Percentage(size) => d = format!("{}%", size),
                };
                args.extend_from_slice(&[l_KEY, &d]);
            }
            #[cfg(feature = "tmux_3_1")]
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
            #[cfg(feature = "tmux_1_2")]
            if let Some(target_pane) = split_window.target_pane {
                args.extend_from_slice(&[t_KEY, &target_pane])
            }
            #[cfg(feature = "tmux_1_2")]
            if let Some(shell_command) = split_window.shell_command {
                args.push(&shell_command)
            }
            #[cfg(feature = "tmux_1_7")]
            if let Some(format) = split_window.format {
                args.extend_from_slice(&[F_KEY, &format])
            }
        }
        let output = self.subcommand(TmuxInterface::SPLIT_WINDOW, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }
}
