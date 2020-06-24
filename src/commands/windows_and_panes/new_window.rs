use crate::error::Error;
use crate::tmux_interface::*;

/// Structure for creating new window, using `tmux new-window` command
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux new-window [-adkP] [-c start-directory] [-e environment] [-F format] [-n window-name] [-t
/// target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux new-window [-adkP] [-c start-directory] [-F format] [-n window-name] [-t target-window]
/// [shell-command]
/// (alias: neww)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux new-window [-adkP] [-n window-name] [-t target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux ^1.3:
/// ```text
/// tmux new-window [-adk] [-n window-name] [-t target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux new-window [-dk] [-n window-name] [-t target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux new-window [-dk] [-n window-name] [-t target-window] [command]
/// (alias: neww)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux new-window [-d] [-n window-name] [-t target-window] [command]
/// (alias: neww)
/// ```
#[derive(Default, Debug)]
pub struct NewWindow<'a> {
    /// [-a] - new window is inserted at the next index up from the specified target-window
    #[cfg(feature = "tmux_1_3")]
    pub add: Option<bool>,
    /// [-d] - the session does not make the new window the current window
    #[cfg(feature = "tmux_0_8")]
    pub detached: Option<bool>,
    /// [-k] - destroy if already exists
    #[cfg(feature = "tmux_1_0")]
    pub kill: Option<bool>,
    /// [-P] - print information about the new window after it has been created
    #[cfg(feature = "tmux_1_5")]
    pub print: Option<bool>,
    /// [-c start-directory] - start-directory
    #[cfg(feature = "tmux_1_7")]
    pub cwd: Option<&'a str>,
    /// [-e environment] - environment
    #[cfg(feature = "tmux_3_0")]
    pub environment: Option<&'a str>,
    /// [-F format] - format
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<&'a str>,
    /// [-n window-name] - window-name
    #[cfg(feature = "tmux_0_8")]
    pub window_name: Option<&'a str>,
    /// [-t target-window] - target-window
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<&'a str>,
    /// [shell-command] - shell-command
    #[cfg(feature = "tmux_1_2")]
    pub shell_command: Option<&'a str>,
}

impl<'a> NewWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct NewWindowBuilder<'a> {
    #[cfg(feature = "tmux_1_3")]
    pub add: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub detached: Option<bool>,
    #[cfg(feature = "tmux_1_0")]
    pub kill: Option<bool>,
    #[cfg(feature = "tmux_1_5")]
    pub print: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub cwd: Option<&'a str>,
    #[cfg(feature = "tmux_3_0")]
    pub environment: Option<&'a str>,
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<&'a str>,
    #[cfg(feature = "tmux_0_8")]
    pub window_name: Option<&'a str>,
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<&'a str>,
    #[cfg(feature = "tmux_1_2")]
    pub shell_command: Option<&'a str>,
}

impl<'a> NewWindowBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_1_3")]
    pub fn add(&mut self) -> &mut Self {
        self.add = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn kill(&mut self) -> &mut Self {
        self.kill = Some(true);
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

    #[cfg(feature = "tmux_3_0")]
    pub fn environment(&mut self, environment: &'a str) -> &mut Self {
        self.environment = Some(environment);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn format(&mut self, format: &'a str) -> &mut Self {
        self.format = Some(format);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn window_name(&mut self, window_name: &'a str) -> &mut Self {
        self.window_name = Some(window_name);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn target_window(&mut self, target_window: &'a str) -> &mut Self {
        self.target_window = Some(target_window);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn shell_command(&mut self, shell_command: &'a str) -> &mut Self {
        self.shell_command = Some(shell_command);
        self
    }

    pub fn build(&self) -> NewWindow<'a> {
        NewWindow {
            #[cfg(feature = "tmux_1_3")]
            add: self.add,
            #[cfg(feature = "tmux_0_8")]
            detached: self.detached,
            #[cfg(feature = "tmux_1_0")]
            kill: self.kill,
            #[cfg(feature = "tmux_1_5")]
            print: self.print,
            #[cfg(feature = "tmux_1_7")]
            cwd: self.cwd,
            #[cfg(feature = "tmux_3_0")]
            environment: self.environment,
            #[cfg(feature = "tmux_1_7")]
            format: self.format,
            #[cfg(feature = "tmux_0_8")]
            window_name: self.window_name,
            #[cfg(feature = "tmux_0_8")]
            target_window: self.target_window,
            #[cfg(feature = "tmux_1_2")]
            shell_command: self.shell_command,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const NEW_WINDOW: &'static str = "new-window";

    /// Create a new window
    ///
    /// # Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// tmux new-window [-adkP] [-c start-directory] [-e environment] [-F format] [-n window-name] [-t
    /// target-window] [shell-command]
    /// (alias: neww)
    /// ```
    ///
    /// tmux ^1.7:
    /// ```text
    /// tmux new-window [-adkP] [-c start-directory] [-F format] [-n window-name] [-t target-window]
    /// [shell-command]
    /// (alias: neww)
    /// ```
    ///
    /// tmux ^1.5:
    /// ```text
    /// tmux new-window [-adkP] [-n window-name] [-t target-window] [shell-command]
    /// (alias: neww)
    /// ```
    ///
    /// tmux ^1.3:
    /// ```text
    /// tmux new-window [-adk] [-n window-name] [-t target-window] [shell-command]
    /// (alias: neww)
    /// ```
    ///
    /// tmux ^1.2:
    /// ```text
    /// tmux new-window [-dk] [-n window-name] [-t target-window] [shell-command]
    /// (alias: neww)
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux new-window [-dk] [-n window-name] [-t target-window] [command]
    /// (alias: neww)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux new-window [-d] [-n window-name] [-t target-window] [command]
    /// (alias: neww)
    /// ```
    // TODO: target_window exitst error
    pub fn new_window(
        &mut self,
        new_window: Option<&NewWindow>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(new_window) = new_window {
            #[cfg(feature = "tmux_1_3")]
            if new_window.add.unwrap_or(false) {
                args.push(a_KEY);
            }
            #[cfg(feature = "tmux_0_8")]
            if new_window.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            #[cfg(feature = "tmux_1_0")]
            if new_window.kill.unwrap_or(false) {
                args.push(k_KEY);
            }
            #[cfg(feature = "tmux_1_5")]
            if new_window.print.unwrap_or(false) {
                args.push(P_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if let Some(s) = new_window.cwd {
                args.extend_from_slice(&[c_KEY, &s])
            }
            #[cfg(feature = "tmux_3_0")]
            if let Some(s) = new_window.environment {
                args.extend_from_slice(&[e_KEY, &s])
            }
            #[cfg(feature = "tmux_1_7")]
            if let Some(s) = new_window.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
            #[cfg(feature = "tmux_0_8")]
            if let Some(s) = new_window.window_name {
                args.extend_from_slice(&[n_KEY, &s])
            }
            #[cfg(feature = "tmux_0_8")]
            if let Some(target_window) = new_window.target_window {
                args.extend_from_slice(&[t_KEY, &target_window])
            }
            #[cfg(feature = "tmux_1_2")]
            if let Some(shell_command) = new_window.shell_command {
                args.push(&shell_command)
            }
        }
        let output = self.command(TmuxInterface::NEW_WINDOW, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        if output.status.success() {
            Ok(stdout.to_string())
        } else {
            Err(Error::Tmux(stdout.to_string()))
        }
    }
}
