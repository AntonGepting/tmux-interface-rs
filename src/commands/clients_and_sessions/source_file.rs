use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Execute commands from path
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux source-file [-Fnqv] path ...
/// (alias: source)
/// ```
///
/// tmux ^3.0:
/// ```text
/// tmux source-file [-nqv] path
/// (alias: source)
/// ```
///
/// tmux ^2.3:
/// ```text
/// tmux source-file path
/// (alias: source)
///
/// ```
/// tmux ^0.8:
/// ```text
/// tmux source-file path
/// (alias: source)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SourceFile<'a> {
    /// `[-F]` - value is expanded as a format
    #[cfg(feature = "tmux_3_2")]
    pub expand: bool,

    /// `[-n]`
    #[cfg(feature = "tmux_3_0")]
    pub not_execute: bool,

    /// `[-q]`
    #[cfg(feature = "tmux_3_0")]
    pub quite: bool,

    /// `[-v]`
    #[cfg(feature = "tmux_3_0")]
    pub verbose: bool,

    /// `path`
    #[cfg(feature = "tmux_0_8")]
    pub path: Option<Cow<'a, str>>,
}

impl<'a> SourceFile<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-F]` - value is expanded as a format
    #[cfg(feature = "tmux_3_2")]
    pub fn expand(mut self) -> Self {
        self.expand = true;
        self
    }

    /// `[-n]`
    #[cfg(feature = "tmux_3_0")]
    pub fn not_execute(mut self) -> Self {
        self.not_execute = true;
        self
    }

    /// `[-q]`
    #[cfg(feature = "tmux_3_0")]
    pub fn quite(mut self) -> Self {
        self.quite = true;
        self
    }

    /// `[-v]`
    #[cfg(feature = "tmux_3_0")]
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// `path`
    #[cfg(feature = "tmux_0_8")]
    pub fn path<S: Into<Cow<'a, str>>>(mut self, path: S) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(SOURCE_FILE);

        // `[-F]` - value is expanded as a format
        #[cfg(feature = "tmux_3_2")]
        if self.expand {
            cmd.push_flag(F_UPPERCASE_KEY);
        }

        // `[-n]`
        #[cfg(feature = "tmux_3_0")]
        if self.not_execute {
            cmd.push_flag(N_LOWERCASE_KEY);
        }

        // `[-q]`
        #[cfg(feature = "tmux_3_0")]
        if self.quite {
            cmd.push_flag(Q_LOWERCASE_KEY);
        }

        // `[-v]`
        #[cfg(feature = "tmux_3_0")]
        if self.verbose {
            cmd.push_flag(V_LOWERCASE_KEY);
        }

        // `path`
        #[cfg(feature = "tmux_0_8")]
        if let Some(path) = self.path {
            cmd.push_param(path);
        }

        cmd
    }
}
