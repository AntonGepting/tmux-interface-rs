use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Display a visible indicator of each pane shown by `target-client`
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux display-panes [-bN] [-d duration] [-t target-client] [template]
/// (alias: displayp)
/// ```
///
/// tmux ^2.9:
/// ```text
/// tmux display-panes [-b] [-d duration] [-t target-client] [template]
/// (alias: displayp)
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux display-panes [-d duration] [-t target-client] [template]
/// (alias: displayp)
/// ```
///
/// tmux ^2.3:
/// ```text
/// tmux display-panes [-t target-client] [template]
/// (alias: displayp)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux display-panes [-t target-client]
/// (alias: displayp)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct DisplayPanes<'a> {
    // `[-b]`
    #[cfg(feature = "tmux_2_9")]
    pub not_block: bool,

    // `[-N]`
    #[cfg(feature = "tmux_3_2")]
    pub ignore_keys: bool,

    // XXX: type?
    // `[-d duration]`
    #[cfg(feature = "tmux_2_6")]
    pub duration: Option<Cow<'a, str>>,

    // `[-t target-client]`
    #[cfg(feature = "tmux_1_0")]
    pub target_client: Option<Cow<'a, str>>,

    // `[template]`
    #[cfg(feature = "tmux_2_3")]
    pub template: Option<Cow<'a, str>>,
}

impl<'a> DisplayPanes<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-b]`
    #[cfg(feature = "tmux_2_9")]
    pub fn not_block(mut self) -> Self {
        self.not_block = true;
        self
    }

    /// `[-N]`
    #[cfg(feature = "tmux_3_2")]
    pub fn ignore_keys(mut self) -> Self {
        self.ignore_keys = true;
        self
    }

    /// `[-d duration]`
    #[cfg(feature = "tmux_2_6")]
    pub fn duration<S: Into<Cow<'a, str>>>(mut self, duration: S) -> Self {
        self.duration = Some(duration.into());
        self
    }

    /// `[-d target-client]`
    #[cfg(feature = "tmux_1_0")]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[template]`
    #[cfg(feature = "tmux_2_3")]
    pub fn template<S: Into<Cow<'a, str>>>(mut self, template: S) -> Self {
        self.template = Some(template.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(DISPLAY_PANES);

        // `[-b]`
        #[cfg(feature = "tmux_2_9")]
        if self.not_block {
            cmd.push_flag(B_LOWERCASE_KEY);
        }

        // `[-N]`
        #[cfg(feature = "tmux_3_2")]
        if self.ignore_keys {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-d duration]`
        #[cfg(feature = "tmux_2_6")]
        if let Some(duration) = self.duration {
            cmd.push_option(D_LOWERCASE_KEY, duration);
        }

        // `[-d duration]`
        #[cfg(feature = "tmux_1_0")]
        if let Some(target_client) = self.target_client {
            cmd.push_option(T_LOWERCASE_KEY, target_client);
        }

        // `[template]`
        #[cfg(feature = "tmux_2_3")]
        if let Some(template) = self.template {
            cmd.push_param(template);
        }

        cmd
    }
}
