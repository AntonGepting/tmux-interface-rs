use crate::commands::constants::*;
use crate::PaneSize;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

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
#[derive(Debug, Clone)]
pub struct SplitWindow<'a>(pub TmuxCommand<'a>);

impl<'a> Default for SplitWindow<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SPLIT_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> SplitWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-b]` - cause the new pane to be created to the left of or above target-pane
    #[cfg(feature = "tmux_2_4")]
    pub fn before(&mut self) -> &mut Self {
        self.0.push_flag(B_LOWERCASE_KEY);
        self
    }

    /// `[-d]` - do not make the new window the current window
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(&mut self) -> &mut Self {
        self.0.push_flag(D_LOWERCASE_KEY);
        self
    }

    /// `[-f]` - creates a new pane spanning the full window size (h, v)
    #[cfg(feature = "tmux_2_4")]
    pub fn full(&mut self) -> &mut Self {
        self.0.push_flag(F_LOWERCASE_KEY);
        self
    }

    /// `[-h]` - horizontal split
    #[cfg(feature = "tmux_1_0")]
    pub fn horizontal(&mut self) -> &mut Self {
        self.0.push_flag(H_LOWERCASE_KEY);
        self
    }

    /// `[-I]` - create an empty pane and forward any output from stdin to it
    #[cfg(feature = "tmux_3_0")]
    pub fn stdin_forward(&mut self) -> &mut Self {
        self.0.push_flag(I_UPPERCASE_KEY);
        self
    }

    /// `[-v]` - vertical split
    #[cfg(feature = "tmux_1_0")]
    pub fn vertical(&mut self) -> &mut Self {
        self.0.push_flag(V_LOWERCASE_KEY);
        self
    }

    /// `[-P]` - print information about the new window after it has been created
    #[cfg(feature = "tmux_1_5")]
    pub fn print(&mut self) -> &mut Self {
        self.0.push_flag(P_UPPERCASE_KEY);
        self
    }

    /// `[-c start_directory]` - start-directory
    #[cfg(feature = "tmux_1_7")]
    pub fn start_directory<S: Into<Cow<'a, str>>>(&mut self, start_directory: S) -> &mut Self {
        self.0.push_option(C_LOWERCASE_KEY, start_directory);
        self
    }

    /// `[-e environment]` - environment
    #[cfg(feature = "tmux_3_1")]
    pub fn environment<S: Into<Cow<'a, str>>>(&mut self, environment: S) -> &mut Self {
        self.0.push_option(E_LOWERCASE_KEY, environment);
        self
    }

    /// `[-l size]` - specify the size of the new pane in lines
    /// `[-l size | -p percentage]` - specify the size of the new pane in lines
    #[cfg(feature = "tmux_0_8")]
    pub fn size(&mut self, size: &PaneSize) -> &mut Self {
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_1")))]
        match size {
            PaneSize::Size(size) => self.0.push_option(L_LOWERCASE_KEY, size.to_string()),
            PaneSize::Percentage(size) => self.0.push_option(L_LOWERCASE_KEY, format!("{}%", size)),
        };
        #[cfg(feature = "tmux_3_1")]
        match size {
            PaneSize::Size(size) => self.0.push_option(L_LOWERCASE_KEY, size.to_string()),
            PaneSize::Percentage(size) => self.0.push_option(P_LOWERCASE_KEY, size.to_string()),
        };
        self
    }

    /// `[-t target-pane]` -
    #[cfg(feature = "tmux_1_2")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[shell-command]` - shell-command
    #[cfg(feature = "tmux_1_2")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(&mut self, shell_command: S) -> &mut Self {
        self.0.push_param(shell_command);
        self
    }

    /// `[-t target-window]` -
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    /// `[-F format]` - format
    #[cfg(feature = "tmux_1_7")]
    pub fn format<S: Into<Cow<'a, str>>>(&mut self, format: S) -> &mut Self {
        self.0.push_option(F_UPPERCASE_KEY, format);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for SplitWindow<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SPLIT_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for SplitWindow<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SPLIT_WINDOW)),
            ..Default::default()
        })
    }
}
