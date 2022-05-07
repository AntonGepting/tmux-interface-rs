use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// # Manual
///
/// tmux ^1.6
/// ```text
/// tmux send-prefix [-2] [-t target-pane]
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux send-prefix [-t target-pane]
/// ```
#[derive(Debug, Default, Clone)]
pub struct SendPrefix<'a> {
    /// `[-2]`
    #[cfg(feature = "tmux_1_6")]
    pub secondary: bool,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_0_8")]
    pub target_pane: Option<Cow<'a, str>>,
}

impl<'a> SendPrefix<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-2]`
    #[cfg(feature = "tmux_1_6")]
    pub fn secondary(&mut self) -> &mut Self {
        self.secondary = true;
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(SEND_PREFIX);

        // `[-2]`
        #[cfg(feature = "tmux_1_6")]
        if self.secondary {
            cmd.push_flag(_2_KEY);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_pane) = &self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane.as_ref());
        }

        cmd
    }
}
