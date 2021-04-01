use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux capture-pane [-aepPqCJN] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux ^2.4:
/// ```text
/// tmux capture-pane [-aepPqCJ] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux ^1.8:
/// ```text
/// tmux capture-pane [-aepPq] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux capture-pane [-b buffer-index] [-E end-line] [-S start-line] [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux capture-pane [-b buffer-index] [-t target-pane]
/// (alias: capturep)
/// ```
#[derive(Debug, Clone)]
pub struct CapturePane<'a>(pub TmuxCommand<'a>);

impl<'a> Default for CapturePane<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(CAPTURE_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> CapturePane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]` - the alternate screen is used, and the history is not accessible
    #[cfg(feature = "tmux_1_8")]
    pub fn alternate_screen(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    /// `[-e]` - the output includes escape sequences for text and background attributes
    #[cfg(feature = "tmux_1_8")]
    pub fn escape_sequences(&mut self) -> &mut Self {
        self.0.push_flag(E_LOWERCASE_KEY);
        self
    }

    /// `[-p]` - the output goes to stdout
    #[cfg(feature = "tmux_1_8")]
    pub fn stdout(&mut self) -> &mut Self {
        self.0.push_flag(P_LOWERCASE_KEY);
        self
    }

    /// `[-P]` - capture only any output that the pane has received that is the beginning of an
    /// as-yet incomplete escape sequence
    #[cfg(feature = "tmux_1_8")]
    pub fn pane(&mut self) -> &mut Self {
        self.0.push_flag(P_UPPERCASE_KEY);
        self
    }

    /// `[-q]` - quite
    #[cfg(feature = "tmux_1_8")]
    pub fn quite(&mut self) -> &mut Self {
        self.0.push_flag(Q_LOWERCASE_KEY);
        self
    }

    /// `[-C]` - escape non-printable characters as octal \xxx
    #[cfg(feature = "tmux_2_4")]
    pub fn escape_non_printable(&mut self) -> &mut Self {
        self.0.push_flag(C_UPPERCASE_KEY);
        self
    }

    /// `[-J]` - preserve trailing spaces and joins any wrapped lines
    #[cfg(feature = "tmux_2_4")]
    pub fn join(&mut self) -> &mut Self {
        self.0.push_flag(J_UPPERCASE_KEY);
        self
    }

    /// `[-N]` - preserves trailing spaces at each line's end
    #[cfg(feature = "tmux_3_1")]
    pub fn trailing_spaces(&mut self) -> &mut Self {
        self.0.push_flag(N_UPPERCASE_KEY);
        self
    }

    /// `[-b buffer-name]` - buffer-name
    #[cfg(feature = "tmux_1_8")]
    pub fn buffer_name<S: Into<Cow<'a, str>>>(&mut self, buffer_name: S) -> &mut Self {
        self.0.push_option(B_LOWERCASE_KEY, buffer_name);
        self
    }

    /// `[-E end-line]` - specify the ending line number
    #[cfg(feature = "tmux_1_5")]
    pub fn end_line<S: Into<Cow<'a, str>>>(&mut self, end_line: S) -> &mut Self {
        self.0.push_option(E_UPPERCASE_KEY, end_line);
        self
    }

    /// `[-S start-line]` - specify the starting line number
    #[cfg(feature = "tmux_1_5")]
    pub fn start_line<S: Into<Cow<'a, str>>>(&mut self, start_line: S) -> &mut Self {
        self.0.push_option(S_UPPERCASE_KEY, start_line);
        self
    }

    /// `[-t target-pane]` - specify target-pane
    #[cfg(feature = "tmux_1_2")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for CapturePane<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(CAPTURE_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for CapturePane<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(CAPTURE_PANE)),
            ..Default::default()
        })
    }
}
