use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Structure for inserting the contents of a paste buffer into the specified pane
///
/// # Manual
///
/// tmux ^1.7:
/// ```text
/// tmux paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
/// (alias: pasteb)
/// ```
///
/// tmux ^1.3:
/// ```text
/// tmux paste-buffer [-dr] [-b buffer-index] [-s separator] [-t target-window]
/// (alias: pasteb)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux paste-buffer [-dr] [-b buffer-index] [-t target-window]
/// (alias: pasteb)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux paste-buffer [-d] [-b buffer-index] [-t target-window]
/// (alias: pasteb)
/// ```
#[derive(Default, Debug)]
pub struct PasteBuffer<'a, T: Display> {
    /// [-d] - delete the paste buffer
    pub delete: Option<bool>,
    /// [-p] - paste bracket control codes are inserted around the buffer
    #[cfg(feature = "tmux_1_0")]
    pub bracket_codes: Option<bool>,
    /// [-r] - do no replacement (equivalent to a separator of LF)
    pub no_replacement: Option<bool>,
    /// [-b buffer-name] - specify the buffer mode
    pub buffer_name: Option<&'a str>,
    /// [-s separator] - specify a separator
    pub separator: Option<&'a str>,
    /// [-t target-pane] - specify the target pane
    pub target_pane: Option<&'a T>,
}

impl<'a, T: Display + Default> PasteBuffer<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct PasteBufferBuilder<'a, T: Display> {
    pub delete: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub bracket_codes: Option<bool>,
    pub no_replacement: Option<bool>,
    pub buffer_name: Option<&'a str>,
    pub separator: Option<&'a str>,
    pub target_pane: Option<&'a T>,
}

impl<'a, T: Display + Default> PasteBufferBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn delete(&mut self) -> &mut Self {
        self.delete = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn bracket_codes(&mut self) -> &mut Self {
        self.bracket_codes = Some(true);
        self
    }

    pub fn no_replacement(&mut self) -> &mut Self {
        self.no_replacement = Some(true);
        self
    }

    pub fn buffer_name(&mut self, buffer_name: &'a str) -> &mut Self {
        self.buffer_name = Some(buffer_name);
        self
    }

    pub fn separator(&mut self, separator: &'a str) -> &mut Self {
        self.separator = Some(separator);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn build(&self) -> PasteBuffer<'a, T> {
        PasteBuffer {
            delete: self.delete,
            #[cfg(feature = "tmux_1_7")]
            bracket_codes: self.bracket_codes,
            no_replacement: self.no_replacement,
            buffer_name: self.buffer_name,
            separator: self.separator,
            target_pane: self.target_pane,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const PASTE_BUFFER: &'static str = "paste-buffer";

    /// Insert the contents of a paste buffer into the specified pane.
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
    /// (alias: pasteb)
    /// ```
    ///
    /// tmux ^1.7:
    /// ```text
    /// tmux paste-buffer [-dp] [-b buffer-name] [-s separator] [-t target-pane]
    /// (alias: pasteb)
    /// ```
    pub fn paste_buffer<T: Display>(
        &mut self,
        paste_buffer: Option<&PasteBuffer<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(paste_buffer) = paste_buffer {
            if paste_buffer.delete.unwrap_or(false) {
                args.push(d_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            {
                if paste_buffer.bracket_codes.unwrap_or(false) {
                    args.push(p_KEY);
                }
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
            if let Some(target_pane) = paste_buffer.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::PASTE_BUFFER, &args)?;
        Ok(output)
    }
}
