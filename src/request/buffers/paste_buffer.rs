use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure for inserting the contents of a paste buffer into the specified pane
///
/// # Manual
///
/// ```text
/// tmux paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
/// (alias: pasteb)
/// ```
#[derive(Default, Debug)]
pub struct PasteBuffer<'a> {
    /// [-d] - delete the paste buffer
    pub delete: Option<bool>,
    /// [-p] - paste bracket control codes are inserted around the buffer
    pub bracket_codes: Option<bool>,
    /// [-r] - do no replacement (equivalent to a separator of LF)
    pub no_replacement: Option<bool>,
    /// [-b buffer-name] - specify the buffer mode
    pub buffer_name: Option<&'a str>,
    /// [-s separator] - specify a separator
    pub separator: Option<&'a str>,
    /// [-t target-pane] - specify the target pane
    pub target_pane: Option<&'a str>,
}

impl<'a> PasteBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// All functions from man tmux "Buffers" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#BUFFERS)
impl<'a> TmuxInterface<'a> {
    const PASTE_BUFFER: &'static str = "paste-buffer";

    /// Insert the contents of a paste buffer into the specified pane.
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
    /// (alias: pasteb)
    /// ```
    pub fn paste_buffer(&mut self, paste_buffer: Option<&PasteBuffer>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(paste_buffer) = paste_buffer {
            if paste_buffer.delete.unwrap_or(false) {
                args.push(d_KEY);
            }
            if paste_buffer.bracket_codes.unwrap_or(false) {
                args.push(p_KEY);
            }
            if paste_buffer.no_replacement.unwrap_or(false) {
                args.push(r_KEY);
            }
            if let Some(s) = paste_buffer.buffer_name {
                args.extend_from_slice(&[b_KEY, &s])
            }
            if let Some(s) = paste_buffer.separator {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = paste_buffer.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::PASTE_BUFFER, &args)?;
        Ok(output)
    }
}
