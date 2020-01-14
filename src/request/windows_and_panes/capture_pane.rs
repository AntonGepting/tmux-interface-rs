use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// # Manual
///
/// tmux X.X:
/// ```text
/// tmux capture-pane [-aepPqCJN] [-b buffer-name] [-E end-line] [-S start-line]
/// [-t target-pane]
/// (alias: capturep)
/// ```
///
/// tmux 2.6:
/// ```text
/// tmux capture-pane [-aepPqCJ] [-b buffer-name] [-E end-line] [-S start-line]
/// [-t target-pane]
/// (alias: capturep)
/// ```
#[derive(Default, Debug)]
pub struct CapturePane<'a> {
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
    /// [-N] - preserves trailing spaces at each line's end
    #[cfg(not(feature = "tmux_2_6"))]
    pub trailing_spaces: Option<bool>,
    /// [-b buffer-name] - buffer-name
    pub buffer_name: Option<&'a str>,
    /// [-E end-line] - specify the ending line number
    pub end_line: Option<&'a str>,
    /// [-S start-line] - specify the starting line number
    pub start_line: Option<&'a str>,
    /// [-t target-pane] - specify target-pane
    pub target_pane: Option<&'a str>,
}

impl<'a> CapturePane<'a> {
    pub fn new() -> CapturePane<'a> {
        Default::default()
    }
}

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
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
    pub fn capture_pane(&mut self, capture_pane: Option<&CapturePane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
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

            if cfg!(not(feature = "tmux_2_6")) {
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
            if let Some(s) = capture_pane.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::CAPTURE_PANE, &args)?;
        Ok(output)
    }
}
