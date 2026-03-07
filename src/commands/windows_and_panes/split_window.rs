// auto-generated file
//

use crate::commands::constants::*;
use crate::PaneSize;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type SplitW<'a> = SplitWindow<'a>;

/// Create a new pane by splitting target-pane
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// split-window [-bdfhIvPZ] [-c start-directory] [-e environment] [-l size] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux >=3.1:
/// ```text
/// split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux >=3.0:
/// ```text
/// split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size | -p percentage]
/// [-t target-pane] [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux >=2.4:
/// ```text
/// split-window [-bdfhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux >=2.0:
/// ```text
/// split-window [-bdhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux >=1.7:
/// ```text
/// split-window [-dhvP] [-c start-directory] [-l size | -p percentage] [-t target-pane]
/// [shell-command] [-F format]
/// (alias: splitw)
/// ```
///
/// tmux >=1.5:
/// ```text
/// split-window [-dhvP] [-l size | -p percentage] [-t target-pane] [shell-command]
/// (alias: splitw)
/// ```
///
/// tmux >=0.8:
/// ```text
/// split-window [-d] [-l size | -p percentage] [-t target-window] [command]
/// (alias: splitw)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SplitWindow<'a> {
    /// `[-b]`
    #[cfg(feature = "tmux_2_0")]
    pub before: bool,

    /// `[-d]`
    #[cfg(feature = "tmux_0_8")]
    pub detached: bool,

    /// `[-f]`
    #[cfg(feature = "tmux_2_4")]
    pub full_size: bool,

    /// `[-h]`
    #[cfg(feature = "tmux_1_5")]
    pub horizontal: bool,

    /// `[-I]`
    #[cfg(feature = "tmux_3_0")]
    pub stdin_forward: bool,

    /// `[-v]`
    #[cfg(feature = "tmux_1_5")]
    pub vertical: bool,

    /// `[-P]`
    #[cfg(feature = "tmux_1_5")]
    pub print: bool,

    /// `[-Z]`
    #[cfg(feature = "tmux_3_2")]
    pub zoom: bool,

    /// `[-c start-directory]`
    #[cfg(feature = "tmux_1_7")]
    pub start_directory: Option<Cow<'a, str>>,

    /// `[-e environment]`
    #[cfg(feature = "tmux_3_1")]
    pub environment: Option<Cow<'a, str>>,

    /// `[-l size]`
    /// `[-l size | -p percentage]`
    #[cfg(feature = "tmux_0_8")]
    pub size: Option<&'a PaneSize>,

    /// `[-p precentage]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_1")))]
    pub precentage: Option<Cow<'a, str>>,

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub target_window: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-F format]`
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<Cow<'a, str>>,

    /// `[shell-command]`
    #[cfg(feature = "tmux_0_8")]
    pub shell_command: Option<Cow<'a, str>>,
}

impl<'a> SplitWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-b]`
    #[cfg(feature = "tmux_2_0")]
    pub fn before(mut self) -> Self {
        self.before = true;
        self
    }

    /// `[-d]`
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(mut self) -> Self {
        self.detached = true;
        self
    }

    /// `[-f]`
    #[cfg(feature = "tmux_2_4")]
    pub fn full_size(mut self) -> Self {
        self.full_size = true;
        self
    }

    /// `[-h]`
    #[cfg(feature = "tmux_1_5")]
    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }

    /// `[-I]`
    #[cfg(feature = "tmux_3_0")]
    pub fn stdin_forward(mut self) -> Self {
        self.stdin_forward = true;
        self
    }

    /// `[-v]`
    #[cfg(feature = "tmux_1_5")]
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    /// `[-P]`
    #[cfg(feature = "tmux_1_5")]
    pub fn print(mut self) -> Self {
        self.print = true;
        self
    }

    /// `[-Z]`
    #[cfg(feature = "tmux_3_2")]
    pub fn zoom(mut self) -> Self {
        self.zoom = true;
        self
    }

    /// `[-c start-directory]`
    #[cfg(feature = "tmux_1_7")]
    pub fn start_directory<S: Into<Cow<'a, str>>>(mut self, start_directory: S) -> Self {
        self.start_directory = Some(start_directory.into());
        self
    }

    /// `[-e environment]`
    #[cfg(feature = "tmux_3_1")]
    pub fn environment<S: Into<Cow<'a, str>>>(mut self, environment: S) -> Self {
        self.environment = Some(environment.into());
        self
    }

    /// `[-l size]`
    /// `[-l size | -p percentage]`
    #[cfg(feature = "tmux_0_8")]
    pub fn size(mut self, size: &'a PaneSize) -> Self {
        self.size = Some(size);
        self
    }

    /// `[-p precentage]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_1")))]
    pub fn precentage<S: Into<Cow<'a, str>>>(mut self, precentage: S) -> Self {
        self.precentage = Some(precentage.into());
        self
    }

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_1_7")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[shell-command]`
    #[cfg(feature = "tmux_0_8")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(mut self, shell_command: S) -> Self {
        self.shell_command = Some(shell_command.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SPLIT_WINDOW);

        // `[-b]`
        #[cfg(feature = "tmux_2_0")]
        if self.before {
            cmd.push_flag(B_LOWERCASE_KEY);
        }

        // `[-d]`
        #[cfg(feature = "tmux_0_8")]
        if self.detached {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-f]`
        #[cfg(feature = "tmux_2_4")]
        if self.full_size {
            cmd.push_flag(F_LOWERCASE_KEY);
        }

        // `[-h]`
        #[cfg(feature = "tmux_1_5")]
        if self.horizontal {
            cmd.push_flag(H_LOWERCASE_KEY);
        }

        // `[-I]`
        #[cfg(feature = "tmux_3_0")]
        if self.stdin_forward {
            cmd.push_flag(I_UPPERCASE_KEY);
        }

        // `[-v]`
        #[cfg(feature = "tmux_1_5")]
        if self.vertical {
            cmd.push_flag(V_LOWERCASE_KEY);
        }

        // `[-P]`
        #[cfg(feature = "tmux_1_5")]
        if self.print {
            cmd.push_flag(P_UPPERCASE_KEY);
        }

        // `[-Z]`
        #[cfg(feature = "tmux_3_2")]
        if self.zoom {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-c start-directory]`
        #[cfg(feature = "tmux_1_7")]
        if let Some(start_directory) = self.start_directory {
            cmd.push_option(C_LOWERCASE_KEY, start_directory);
        }

        // `[-e environment]`
        #[cfg(feature = "tmux_3_1")]
        if let Some(environment) = self.environment {
            cmd.push_option(E_LOWERCASE_KEY, environment);
        }

        // `[-l size]`
        // `[-l size | -p percentage]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(size) = &self.size {
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_1")))]
            match size {
                PaneSize::Size(size) => {
                    cmd.push_option(L_LOWERCASE_KEY, Cow::Owned(size.to_string()))
                }
                PaneSize::Percentage(size) => {
                    cmd.push_option(L_LOWERCASE_KEY, Cow::Owned(format!("{}%", size)))
                }
            };
            #[cfg(feature = "tmux_3_1")]
            match size {
                PaneSize::Size(size) => cmd.push_option(L_LOWERCASE_KEY, size.to_string()),
                PaneSize::Percentage(size) => cmd.push_option(P_LOWERCASE_KEY, size.to_string()),
            };
        }

        // `[-p precentage]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_1")))]
        if let Some(precentage) = self.precentage {
            cmd.push_option(P_LOWERCASE_KEY, precentage);
        }

        // `[-t target-window]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[-F format]`
        #[cfg(feature = "tmux_1_7")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[shell-command]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(shell_command) = self.shell_command {
            cmd.push_param(shell_command);
        }

        cmd
    }
}
