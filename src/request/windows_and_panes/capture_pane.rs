use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
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
pub struct CapturePane<'a, T: Display> {
    /// [-a] - the alternate screen is used, and the history is not accessible
    pub alternate_screen: Option<bool>,
    /// [-e] - the output includes escape sequences for text and background attributes
    pub escape_sequences: Option<bool>,
    /// [-p] - the output goes to stdout
    pub stdout: Option<bool>,
    /// [-P] - capture only any output that the pane has received that is the beginning of an
    /// as-yet incomplete escape sequence
    pub pane: Option<bool>,
    /// [-q] - quite
    pub quite: Option<bool>,
    /// [-C] - escape non-printable characters as octal \xxx
    pub escape_non_printable: Option<bool>,
    /// [-J] - preserve trailing spaces and joins any wrapped lines
    pub join: Option<bool>,
    #[cfg(feature = "tmux_X_X")]
    /// [-N] - preserves trailing spaces at each line's end
    pub trailing_spaces: Option<bool>,
    /// [-b buffer-name] - buffer-name
    pub buffer_name: Option<&'a str>,
    /// [-E end-line] - specify the ending line number
    pub end_line: Option<&'a str>,
    /// [-S start-line] - specify the starting line number
    pub start_line: Option<&'a str>,
    /// [-t target-pane] - specify target-pane
    pub target_pane: Option<&'a T>,
}

impl<'a, T: Display + Default> CapturePane<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct CapturePaneBuilder<'a, T: Display> {
    pub alternate_screen: Option<bool>,
    pub escape_sequences: Option<bool>,
    pub stdout: Option<bool>,
    pub pane: Option<bool>,
    pub quite: Option<bool>,
    pub escape_non_printable: Option<bool>,
    pub join: Option<bool>,
    #[cfg(feature = "tmux_X_X")]
    pub trailing_spaces: Option<bool>,
    pub buffer_name: Option<&'a str>,
    pub end_line: Option<&'a str>,
    pub start_line: Option<&'a str>,
    pub target_pane: Option<&'a T>,
}

impl<'a, T: Display + Default> CapturePaneBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn alternate_screen(&mut self) -> &mut Self {
        self.alternate_screen = Some(true);
        self
    }

    pub fn escape_sequences(&mut self) -> &mut Self {
        self.escape_sequences = Some(true);
        self
    }

    pub fn stdout(&mut self) -> &mut Self {
        self.stdout = Some(true);
        self
    }

    pub fn pane(&mut self) -> &mut Self {
        self.pane = Some(true);
        self
    }

    pub fn quite(&mut self) -> &mut Self {
        self.quite = Some(true);
        self
    }

    pub fn escape_non_printable(&mut self) -> &mut Self {
        self.escape_non_printable = Some(true);
        self
    }

    pub fn join(&mut self) -> &mut Self {
        self.join = Some(true);
        self
    }

    #[cfg(feature = "tmux_X_X")]
    pub fn trailing_spaces(&mut self) -> &mut Self {
        self.trailing_spaces = Some(true);
        self
    }

    pub fn buffer_name(&mut self, buffer_name: &'a str) -> &mut Self {
        self.buffer_name = Some(buffer_name);
        self
    }

    pub fn end_line(&mut self, end_line: &'a str) -> &mut Self {
        self.end_line = Some(end_line);
        self
    }

    pub fn start_line(&mut self, start_line: &'a str) -> &mut Self {
        self.start_line = Some(start_line);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn build(&self) -> CapturePane<'a, T> {
        CapturePane {
            alternate_screen: self.alternate_screen,
            escape_sequences: self.escape_sequences,
            stdout: self.stdout,
            pane: self.pane,
            quite: self.quite,
            escape_non_printable: self.escape_non_printable,
            join: self.join,
            #[cfg(feature = "tmux_X_X")]
            trailing_spaces: self.trailing_spaces,
            buffer_name: self.buffer_name,
            end_line: self.end_line,
            start_line: self.start_line,
            target_pane: self.target_pane,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const CAPTURE_PANE: &'static str = "capture-pane";

    /// Capture the contents of a pane
    ///
    /// # Manual (tmux X.X)
    ///
    /// ```text
    /// tmux capture-pane [-aepPqCJN] [-b buffer-name] [-E end-line] [-S start-line]
    /// [-t target-pane]
    /// (alias: capturep)
    /// ```
    ///
    /// # Manual (tmux 2.6)
    ///
    /// ```text
    /// tmux capture-pane [-aepPqCJ] [-b buffer-name] [-E end-line] [-S start-line]
    /// [-t target-pane]
    /// (alias: capturep)
    /// ```
    pub fn capture_pane<T: Display>(
        &mut self,
        capture_pane: Option<&CapturePane<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(capture_pane) = capture_pane {
            if capture_pane.alternate_screen.unwrap_or(false) {
                args.push(a_KEY);
            }
            if capture_pane.escape_sequences.unwrap_or(false) {
                args.push(e_KEY);
            }
            if capture_pane.stdout.unwrap_or(false) {
                args.push(p_KEY);
            }
            if capture_pane.pane.unwrap_or(false) {
                args.push(P_KEY);
            }
            if capture_pane.quite.unwrap_or(false) {
                args.push(q_KEY);
            }
            if capture_pane.escape_non_printable.unwrap_or(false) {
                args.push(C_KEY);
            }
            if capture_pane.join.unwrap_or(false) {
                args.push(J_KEY);
            }
            #[cfg(feature = "tmux_X_X")]
            {
                if capture_pane.trailing_spaces.unwrap_or(false) {
                    args.push(N_KEY);
                }
            }
            if let Some(s) = capture_pane.buffer_name {
                args.extend_from_slice(&[b_KEY, &s])
            }
            if let Some(s) = capture_pane.end_line {
                args.extend_from_slice(&[E_KEY, &s])
            }
            if let Some(s) = capture_pane.start_line {
                args.extend_from_slice(&[S_KEY, &s])
            }
            if let Some(target_pane) = capture_pane.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::CAPTURE_PANE, &args)?;
        Ok(output)
    }
}
