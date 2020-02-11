use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;

/// Structure for creating new window, using `tmux new-window` command
///
/// # Manual
///
/// tmux X.X:
/// ```text
/// tmux new-window [-adkP] [-c start-directory] [-e environment] [-F format]
/// [-n window-name] [-t target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux 2.6:
/// ```text
/// tmux new-window [-adkP] [-c start-directory] [-F format]
/// [-n window-name] [-t target-window] [shell-command]
/// (alias: neww)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct NewWindow<'a, T: Display> {
    /// [-a] - new window is inserted at the next index up from the specified target-window
    pub add: Option<bool>,
    /// [-d] - the session does not make the new window the current window
    pub detached: Option<bool>,
    /// [-k] - destroy if already exists
    pub kill: Option<bool>,
    /// [-P] - print information about the new window after it has been created
    pub print: Option<bool>,
    /// [-c start-directory] - start-directory
    pub cwd: Option<&'a str>,
    /// [-e environment] - environment
    pub environment: Option<&'a str>,
    /// [-F format] - format
    pub format: Option<&'a str>,
    /// [-n window-name] - window-name
    pub window_name: Option<&'a str>,
    /// [-t target-window] - target-window
    pub target_window: Option<&'a T>,
    /// [shell-command] - shell-command
    pub shell_command: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct NewWindow<'a, T: Display> {
    /// [-a] - new window is inserted at the next index up from the specified target-window
    pub add: Option<bool>,
    /// [-d] - the session does not make the new window the current window
    pub detached: Option<bool>,
    /// [-k] - destroy if already exists
    pub kill: Option<bool>,
    /// [-P] - print information about the new window after it has been created
    pub print: Option<bool>,
    /// [-c start-directory] - start-directory
    pub cwd: Option<&'a str>,
    /// [-F format] - format
    pub format: Option<&'a str>,
    /// [-n window-name] - window-name
    pub window_name: Option<&'a str>,
    /// [-t target-window] - target-window
    pub target_window: Option<&'a T>,
    /// [shell-command] - shell-command
    pub shell_command: Option<&'a str>,
}

impl<'a, T: Default + Display> NewWindow<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct NewWindowBuilder<'a, T: Display> {
    pub add: Option<bool>,
    pub detached: Option<bool>,
    pub kill: Option<bool>,
    pub print: Option<bool>,
    pub cwd: Option<&'a str>,
    pub environment: Option<&'a str>,
    pub format: Option<&'a str>,
    pub window_name: Option<&'a str>,
    pub target_window: Option<&'a T>,
    pub shell_command: Option<&'a str>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a, T: Display + Default> NewWindowBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self) -> &mut Self {
        self.add = Some(true);
        self
    }

    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    pub fn kill(&mut self) -> &mut Self {
        self.kill = Some(true);
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

    pub fn format(&mut self, format: &'a str) -> &mut Self {
        self.format = Some(format);
        self
    }

    pub fn window_name(&mut self, window_name: &'a str) -> &mut Self {
        self.window_name = Some(window_name);
        self
    }

    pub fn target_window(&mut self, target_window: &'a T) -> &mut Self {
        self.target_window = Some(target_window);
        self
    }

    pub fn shell_command(&mut self, shell_command: &'a str) -> &mut Self {
        self.shell_command = Some(shell_command);
        self
    }

    pub fn build(&self) -> NewWindow<'a, T> {
        NewWindow {
            add: self.add,
            detached: self.detached,
            kill: self.kill,
            print: self.print,
            cwd: self.cwd,
            environment: self.environment,
            format: self.format,
            window_name: self.window_name,
            target_window: self.target_window,
            shell_command: self.shell_command,
        }
    }
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct NewWindowBuilder<'a, T: Display> {
    pub add: Option<bool>,
    pub detached: Option<bool>,
    pub kill: Option<bool>,
    pub print: Option<bool>,
    pub cwd: Option<&'a str>,
    pub format: Option<&'a str>,
    pub window_name: Option<&'a str>,
    pub target_window: Option<&'a T>,
    pub shell_command: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
impl<'a, T: Display + Default> NewWindowBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self) -> &mut Self {
        self.add = Some(true);
        self
    }

    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    pub fn kill(&mut self) -> &mut Self {
        self.kill = Some(true);
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

    pub fn format(&mut self, format: &'a str) -> &mut Self {
        self.format = Some(format);
        self
    }

    pub fn window_name(&mut self, window_name: &'a str) -> &mut Self {
        self.window_name = Some(window_name);
        self
    }

    pub fn target_window(&mut self, target_window: &'a T) -> &mut Self {
        self.target_window = Some(target_window);
        self
    }

    pub fn shell_command(&mut self, shell_command: &'a str) -> &mut Self {
        self.shell_command = Some(shell_command);
        self
    }

    pub fn build(&self) -> NewWindow<'a, T> {
        NewWindow {
            add: self.add,
            detached: self.detached,
            kill: self.kill,
            print: self.print,
            cwd: self.cwd,
            format: self.format,
            window_name: self.window_name,
            target_window: self.target_window,
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
    /// tmux X.X:
    /// ```text
    /// tmux new-window [-adkP] [-c start-directory] [-e environment] [-F format]
    /// [-n window-name] [-t target-window] [shell-command]
    /// (alias: neww)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux new-window [-adkP] [-c start-directory] [-F format]
    /// [-n window-name] [-t target-window] [shell-command]
    /// (alias: neww)
    /// ```
    // TODO: target_window exitst error
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn new_window<T: Display>(
        &mut self,
        new_window: Option<&NewWindow<T>>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(new_window) = new_window {
            if new_window.add.unwrap_or(false) {
                args.push(a_KEY);
            }
            if new_window.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if new_window.kill.unwrap_or(false) {
                args.push(k_KEY);
            }
            if new_window.print.unwrap_or(false) {
                args.push(P_KEY);
            }
            if let Some(s) = new_window.cwd {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(s) = new_window.environment {
                args.extend_from_slice(&[e_KEY, &s])
            }
            if let Some(s) = new_window.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
            if let Some(s) = new_window.window_name {
                args.extend_from_slice(&[n_KEY, &s])
            }
            if let Some(target_window) = new_window.target_window {
                s = target_window.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = new_window.shell_command {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::NEW_WINDOW, &args)?;
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
            Ok(stdout.to_string())
        } else {
            let stdout = String::from_utf8_lossy(&output.stderr.as_slice());
            Err(Error::new(&stdout))
        }
    }

    /// Create a new window
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux new-window [-adkP] [-c start-directory] [-e environment] [-F format]
    /// [-n window-name] [-t target-window] [shell-command]
    /// (alias: neww)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux new-window [-adkP] [-c start-directory] [-F format]
    /// [-n window-name] [-t target-window] [shell-command]
    /// (alias: neww)
    /// ```
    // TODO: target_window exitst error
    #[cfg(feature = "tmux_2_6")]
    pub fn new_window<T: Display>(
        &mut self,
        new_window: Option<&NewWindow<T>>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(new_window) = new_window {
            if new_window.add.unwrap_or(false) {
                args.push(a_KEY);
            }
            if new_window.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if new_window.kill.unwrap_or(false) {
                args.push(k_KEY);
            }
            if new_window.print.unwrap_or(false) {
                args.push(P_KEY);
            }
            if let Some(s) = new_window.cwd {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(s) = new_window.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
            if let Some(s) = new_window.window_name {
                args.extend_from_slice(&[n_KEY, &s])
            }
            if let Some(target_window) = new_window.target_window {
                s = target_window.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = new_window.shell_command {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::NEW_WINDOW, &args)?;
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
            Ok(stdout.to_string())
        } else {
            let stdout = String::from_utf8_lossy(&output.stderr.as_slice());
            Err(Error::new(&stdout))
        }
    }
}
