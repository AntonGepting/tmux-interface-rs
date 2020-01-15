use crate::error::Error;
use crate::tmux_interface::*;

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
pub struct NewWindow<'a> {
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
    pub target_window: Option<&'a str>,
    /// [shell-command] - shell-command
    pub shell_command: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct NewWindow<'a> {
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
    pub target_window: Option<&'a str>,
    /// [shell-command] - shell-command
    pub shell_command: Option<&'a str>,
}

impl<'a> NewWindow<'a> {
    pub fn new() -> NewWindow<'a> {
        Default::default()
    }
}

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
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
    pub fn new_window(&mut self, new_window: Option<&NewWindow>) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
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
            if let Some(s) = new_window.window_name {
                args.extend_from_slice(&[n_KEY, &s])
            }
            if let Some(s) = new_window.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
            if let Some(s) = new_window.target_window {
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
    pub fn new_window(&mut self, new_window: Option<&NewWindow>) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
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
            if let Some(s) = new_window.window_name {
                args.extend_from_slice(&[n_KEY, &s])
            }
            if let Some(s) = new_window.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
            if let Some(s) = new_window.target_window {
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
