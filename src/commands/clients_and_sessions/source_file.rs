use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Source<'a> = SourceFile<'a>;

/// Execute commands from path
///
/// # Manual
///
/// tmux ^3.4:
/// ```text
/// source-file [-Fnqv] [-t target-pane] path ...
/// (alias: source)
/// ```
///
/// tmux ^3.2:
/// ```text
/// source-file [-Fnqv] path ...
/// (alias: source)
/// ```
///
/// tmux ^3.0:
/// ```text
/// source-file [-nqv] path
/// (alias: source)
/// ```
///
/// tmux ^2.3:
/// ```text
/// source-file path
/// (alias: source)
///
/// ```
/// tmux ^0.8:
/// ```text
/// source-file path
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
    pub quiet: bool,

    /// `[-v]`
    #[cfg(feature = "tmux_3_0")]
    pub verbose: bool,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_3_4")]
    pub target_pane: Option<Cow<'a, str>>,

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
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// `[-v]`
    #[cfg(feature = "tmux_3_0")]
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_3_4")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
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

        cmd.name(SOURCE_FILE);

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
        if self.quiet {
            cmd.push_flag(Q_LOWERCASE_KEY);
        }

        // `[-v]`
        #[cfg(feature = "tmux_3_0")]
        if self.verbose {
            cmd.push_flag(V_LOWERCASE_KEY);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_3_4")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `path`
        #[cfg(feature = "tmux_0_8")]
        if let Some(path) = self.path {
            cmd.push_param(path);
        }

        cmd
    }
}
