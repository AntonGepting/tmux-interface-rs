// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type CaptureP<'a> = CapturePane<'a>;

/// Capture the contents of a pane
///
/// # Manual
///
/// tmux >=3.6:
/// ```text
/// capture-pane [-aepPqCJMNT] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux >=3.4:
/// ```text
/// capture-pane [-aAepPqCJNT] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux >=3.1:
/// ```text
/// capture-pane [-aepPqCJN] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux >=2.4:
/// ```text
/// capture-pane [-aepPqCJ] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux >=1.8:
/// ```text
/// capture-pane [-aepPq] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux >=1.5:
/// ```text
/// capture-pane [-b buffer-index] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CapturePane<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_1_8")]
    pub alternate_screen: bool,

    /// `[-A]`
    #[cfg(feature = "tmux_3_4")]
    pub a: bool,

    /// `[-e]`
    #[cfg(feature = "tmux_1_8")]
    pub escape_sequences: bool,

    /// `[-p]`
    #[cfg(feature = "tmux_1_8")]
    pub stdout: bool,

    /// `[-P]`
    #[cfg(feature = "tmux_1_8")]
    pub pane: bool,

    /// `[-q]`
    #[cfg(feature = "tmux_1_8")]
    pub quiet: bool,

    /// `[-C]`
    #[cfg(feature = "tmux_2_4")]
    pub escape_non_printable: bool,

    /// `[-J]`
    #[cfg(feature = "tmux_2_4")]
    pub join: bool,

    /// `[-M]`
    #[cfg(feature = "tmux_3_6")]
    pub screen_for_mode: bool,

    /// `[-N]`
    #[cfg(feature = "tmux_3_1")]
    pub trailing_spaces: bool,

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    pub buffer_index: Option<Cow<'a, str>>,

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub buffer_name: Option<Cow<'a, str>>,

    /// `[-E end-line]`
    #[cfg(feature = "tmux_1_5")]
    pub end_line: Option<Cow<'a, str>>,

    /// `[-S start-line]`
    #[cfg(feature = "tmux_1_5")]
    pub start_line: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub target_pane: Option<Cow<'a, str>>,
}

impl<'a> CapturePane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_1_8")]
    pub fn alternate_screen(mut self) -> Self {
        self.alternate_screen = true;
        self
    }

    /// `[-A]`
    #[cfg(feature = "tmux_3_4")]
    pub fn a(mut self) -> Self {
        self.a = true;
        self
    }

    /// `[-e]`
    #[cfg(feature = "tmux_1_8")]
    pub fn escape_sequences(mut self) -> Self {
        self.escape_sequences = true;
        self
    }

    /// `[-p]`
    #[cfg(feature = "tmux_1_8")]
    pub fn stdout(mut self) -> Self {
        self.stdout = true;
        self
    }

    /// `[-P]`
    #[cfg(feature = "tmux_1_8")]
    pub fn pane(mut self) -> Self {
        self.pane = true;
        self
    }

    /// `[-q]`
    #[cfg(feature = "tmux_1_8")]
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// `[-C]`
    #[cfg(feature = "tmux_2_4")]
    pub fn escape_non_printable(mut self) -> Self {
        self.escape_non_printable = true;
        self
    }

    /// `[-J]`
    #[cfg(feature = "tmux_2_4")]
    pub fn join(mut self) -> Self {
        self.join = true;
        self
    }

    /// `[-M]`
    #[cfg(feature = "tmux_3_6")]
    pub fn screen_for_mode(mut self) -> Self {
        self.screen_for_mode = true;
        self
    }

    /// `[-N]`
    #[cfg(feature = "tmux_3_1")]
    pub fn trailing_spaces(mut self) -> Self {
        self.trailing_spaces = true;
        self
    }

    /// `[-b buffer-index]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    pub fn buffer_index<S: Into<Cow<'a, str>>>(mut self, buffer_index: S) -> Self {
        self.buffer_index = Some(buffer_index.into());
        self
    }

    /// `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    pub fn buffer_name<S: Into<Cow<'a, str>>>(mut self, buffer_name: S) -> Self {
        self.buffer_name = Some(buffer_name.into());
        self
    }

    // XXX: type usize?
    /// `[-E end-line]`
    #[cfg(feature = "tmux_1_5")]
    pub fn end_line<S: Into<Cow<'a, str>>>(mut self, end_line: S) -> Self {
        self.end_line = Some(end_line.into());
        self
    }

    // XXX: type usize?
    /// `[-S start-line]`
    #[cfg(feature = "tmux_1_5")]
    pub fn start_line<S: Into<Cow<'a, str>>>(mut self, start_line: S) -> Self {
        self.start_line = Some(start_line.into());
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(CAPTURE_PANE);

        // `[-a]`
        #[cfg(feature = "tmux_1_8")]
        if self.alternate_screen {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-A]`
        #[cfg(feature = "tmux_3_4")]
        if self.a {
            cmd.push_flag(A_UPPERCASE_KEY);
        }

        // `[-e]`
        #[cfg(feature = "tmux_1_8")]
        if self.escape_sequences {
            cmd.push_flag(E_LOWERCASE_KEY);
        }

        // `[-p]`
        #[cfg(feature = "tmux_1_8")]
        if self.stdout {
            cmd.push_flag(P_LOWERCASE_KEY);
        }

        // `[-P]`
        #[cfg(feature = "tmux_1_8")]
        if self.pane {
            cmd.push_flag(P_UPPERCASE_KEY);
        }

        // `[-q]`
        #[cfg(feature = "tmux_1_8")]
        if self.quiet {
            cmd.push_flag(Q_LOWERCASE_KEY);
        }

        // `[-C]`
        #[cfg(feature = "tmux_2_4")]
        if self.escape_non_printable {
            cmd.push_flag(C_UPPERCASE_KEY);
        }

        // `[-J]`
        #[cfg(feature = "tmux_2_4")]
        if self.join {
            cmd.push_flag(J_UPPERCASE_KEY);
        }

        // `[-M]`
        #[cfg(feature = "tmux_3_6")]
        if self.screen_for_mode {
            cmd.push_flag(M_UPPERCASE_KEY);
        }

        // `[-N]`
        #[cfg(feature = "tmux_3_1")]
        if self.trailing_spaces {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-b buffer-index]`
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
        if let Some(buffer_index) = self.buffer_index {
            cmd.push_option(B_LOWERCASE_KEY, buffer_index);
        }

        // `[-b buffer-name]`
        #[cfg(feature = "tmux_2_0")]
        if let Some(buffer_name) = self.buffer_name {
            cmd.push_option(B_LOWERCASE_KEY, buffer_name);
        }

        // `[-E end-line]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(end_line) = self.end_line {
            cmd.push_option(E_UPPERCASE_KEY, end_line);
        }

        // `[-S start-line]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(start_line) = self.start_line {
            cmd.push_option(S_UPPERCASE_KEY, start_line);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        cmd
    }
}
