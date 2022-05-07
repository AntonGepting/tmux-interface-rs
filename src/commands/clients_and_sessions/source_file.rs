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
#[derive(Debug, Default, Clone)]
pub struct SourceFile<'a> {
    /// `[-F]` - value is expanded as a format
    #[cfg(feature = "tmux_3_2")]
    pub expand: Option(bool),

    /// `[-n]`
    #[cfg(feature = "tmux_3_0")]
    pub not_execute: Option(bool),

    /// `[-q]`
    #[cfg(feature = "tmux_3_0")]
    pub quite: Option(bool),

    /// `[-v]`
    #[cfg(feature = "tmux_3_0")]
    pub verbose: Option(bool),

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
    pub fn expand(&mut self) -> &mut Self {
        self.expand = Some(true);
        self
    }

    /// `[-n]`
    #[cfg(feature = "tmux_3_0")]
    pub fn not_execute(&mut self) -> &mut Self {
        self.not_execute = Some(true);
        self
    }

    /// `[-q]`
    #[cfg(feature = "tmux_3_0")]
    pub fn quite(&mut self) -> &mut Self {
        self.quite = Some(true);
        self
    }

    /// `[-v]`
    #[cfg(feature = "tmux_3_0")]
    pub fn verbose(&mut self) -> &mut Self {
        self.verbose = Some(true);
        self
    }

    /// `path`
    #[cfg(feature = "tmux_0_8")]
    pub fn path<S: Into<Cow<'a, str>>>(&mut self, path: S) -> &mut Self {
        self.path = Some(path.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(SOURCE_FILE);

        // `[-F]` - value is expanded as a format
        #[cfg(feature = "tmux_3_2")]
        if self.expand.is_some() {
            cmd.push_flag(F_UPPERCASE_KEY);
        }

        // `[-n]`
        #[cfg(feature = "tmux_3_0")]
        if self.not_execute.is_some() {
            cmd.push_flag(N_LOWERCASE_KEY);
        }

        // `[-q]`
        #[cfg(feature = "tmux_3_0")]
        if self.quite.is_some() {
            cmd.push_flag(Q_LOWERCASE_KEY);
        }

        // `[-v]`
        #[cfg(feature = "tmux_3_0")]
        if self.verbose.is_some() {
            cmd.push_flag(V_LOWERCASE_KEY);
        }

        // `path`
        #[cfg(feature = "tmux_0_8")]
        if let Some(path) = &self.path {
            cmd.push_param(path.as_ref());
        }

        cmd
    }
}
