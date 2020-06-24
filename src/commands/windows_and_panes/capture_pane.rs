use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

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
#[derive(Default, Debug)]
pub struct CapturePane<'a> {
    /// [-a] - the alternate screen is used, and the history is not accessible
    #[cfg(feature = "tmux_1_8")]
    pub alternate_screen: Option<bool>,
    /// [-e] - the output includes escape sequences for text and background attributes
    #[cfg(feature = "tmux_1_8")]
    pub escape_sequences: Option<bool>,
    /// [-p] - the output goes to stdout
    #[cfg(feature = "tmux_1_8")]
    pub stdout: Option<bool>,
    /// [-P] - capture only any output that the pane has received that is the beginning of an
    /// as-yet incomplete escape sequence
    #[cfg(feature = "tmux_1_8")]
    pub pane: Option<bool>,
    /// [-q] - quite
    #[cfg(feature = "tmux_1_8")]
    pub quite: Option<bool>,
    /// [-C] - escape non-printable characters as octal \xxx
    #[cfg(feature = "tmux_2_4")]
    pub escape_non_printable: Option<bool>,
    /// [-J] - preserve trailing spaces and joins any wrapped lines
    #[cfg(feature = "tmux_2_4")]
    pub join: Option<bool>,
    /// [-N] - preserves trailing spaces at each line's end
    #[cfg(feature = "tmux_3_1")]
    pub trailing_spaces: Option<bool>,
    /// [-b buffer-name] - buffer-name
    #[cfg(feature = "tmux_1_8")]
    pub buffer_name: Option<&'a str>,
    /// [-E end-line] - specify the ending line number
    #[cfg(feature = "tmux_1_5")]
    pub end_line: Option<&'a str>,
    /// [-S start-line] - specify the starting line number
    #[cfg(feature = "tmux_1_5")]
    pub start_line: Option<&'a str>,
    /// [-t target-pane] - specify target-pane
    #[cfg(feature = "tmux_1_2")]
    pub target_pane: Option<&'a str>,
}

impl<'a> CapturePane<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct CapturePaneBuilder<'a> {
    #[cfg(feature = "tmux_1_8")]
    pub alternate_screen: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub escape_sequences: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub stdout: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub pane: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub quite: Option<bool>,
    #[cfg(feature = "tmux_2_4")]
    pub escape_non_printable: Option<bool>,
    #[cfg(feature = "tmux_2_4")]
    pub join: Option<bool>,
    #[cfg(feature = "tmux_3_1")]
    pub trailing_spaces: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub buffer_name: Option<&'a str>,
    #[cfg(feature = "tmux_1_5")]
    pub end_line: Option<&'a str>,
    #[cfg(feature = "tmux_1_5")]
    pub start_line: Option<&'a str>,
    #[cfg(feature = "tmux_1_2")]
    pub target_pane: Option<&'a str>,
}

impl<'a> CapturePaneBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn alternate_screen(&mut self) -> &mut Self {
        self.alternate_screen = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn escape_sequences(&mut self) -> &mut Self {
        self.escape_sequences = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn stdout(&mut self) -> &mut Self {
        self.stdout = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn pane(&mut self) -> &mut Self {
        self.pane = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn quite(&mut self) -> &mut Self {
        self.quite = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn escape_non_printable(&mut self) -> &mut Self {
        self.escape_non_printable = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn join(&mut self) -> &mut Self {
        self.join = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_1")]
    pub fn trailing_spaces(&mut self) -> &mut Self {
        self.trailing_spaces = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn buffer_name(&mut self, buffer_name: &'a str) -> &mut Self {
        self.buffer_name = Some(buffer_name);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn end_line(&mut self, end_line: &'a str) -> &mut Self {
        self.end_line = Some(end_line);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn start_line(&mut self, start_line: &'a str) -> &mut Self {
        self.start_line = Some(start_line);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn target_pane(&mut self, target_pane: &'a str) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn build(&self) -> CapturePane<'a> {
        CapturePane {
            #[cfg(feature = "tmux_1_8")]
            alternate_screen: self.alternate_screen,
            #[cfg(feature = "tmux_1_8")]
            escape_sequences: self.escape_sequences,
            #[cfg(feature = "tmux_1_8")]
            stdout: self.stdout,
            #[cfg(feature = "tmux_1_8")]
            pane: self.pane,
            #[cfg(feature = "tmux_1_8")]
            quite: self.quite,
            #[cfg(feature = "tmux_2_4")]
            escape_non_printable: self.escape_non_printable,
            #[cfg(feature = "tmux_2_4")]
            join: self.join,
            #[cfg(feature = "tmux_3_1")]
            trailing_spaces: self.trailing_spaces,
            #[cfg(feature = "tmux_1_8")]
            buffer_name: self.buffer_name,
            #[cfg(feature = "tmux_1_5")]
            end_line: self.end_line,
            #[cfg(feature = "tmux_1_5")]
            start_line: self.start_line,
            #[cfg(feature = "tmux_1_2")]
            target_pane: self.target_pane,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const CAPTURE_PANE: &'static str = "capture-pane";

    /// Capture the contents of a pane
    ///
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
    pub fn capture_pane(
        &mut self,
        capture_pane: Option<&CapturePane>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(capture_pane) = capture_pane {
            #[cfg(feature = "tmux_1_8")]
            if capture_pane.alternate_screen.unwrap_or(false) {
                args.push(a_KEY);
            }
            #[cfg(feature = "tmux_1_8")]
            if capture_pane.escape_sequences.unwrap_or(false) {
                args.push(e_KEY);
            }
            #[cfg(feature = "tmux_1_8")]
            if capture_pane.stdout.unwrap_or(false) {
                args.push(p_KEY);
            }
            #[cfg(feature = "tmux_1_8")]
            if capture_pane.pane.unwrap_or(false) {
                args.push(P_KEY);
            }
            #[cfg(feature = "tmux_1_8")]
            if capture_pane.quite.unwrap_or(false) {
                args.push(q_KEY);
            }
            #[cfg(feature = "tmux_2_4")]
            if capture_pane.escape_non_printable.unwrap_or(false) {
                args.push(C_KEY);
            }
            #[cfg(feature = "tmux_2_4")]
            if capture_pane.join.unwrap_or(false) {
                args.push(J_KEY);
            }
            #[cfg(feature = "tmux_3_1")]
            if capture_pane.trailing_spaces.unwrap_or(false) {
                args.push(N_KEY);
            }
            #[cfg(feature = "tmux_1_8")]
            if let Some(buffer_name) = capture_pane.buffer_name {
                args.extend_from_slice(&[b_KEY, &buffer_name])
            }
            #[cfg(feature = "tmux_1_5")]
            if let Some(end_line) = capture_pane.end_line {
                args.extend_from_slice(&[E_KEY, &end_line])
            }
            #[cfg(feature = "tmux_1_5")]
            if let Some(start_line) = capture_pane.start_line {
                args.extend_from_slice(&[S_KEY, &start_line])
            }
            #[cfg(feature = "tmux_1_2")]
            if let Some(target_pane) = capture_pane.target_pane {
                args.extend_from_slice(&[t_KEY, &target_pane])
            }
        }
        let output = self.command(TmuxInterface::CAPTURE_PANE, &args)?;
        Ok(output)
    }
}
