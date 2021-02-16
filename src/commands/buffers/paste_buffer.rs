use crate::error::Error;
use crate::tmux_interface::*;
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
pub struct PasteBuffer<'a> {
    /// [-d] - delete the paste buffer
    #[cfg(feature = "tmux_0_8")]
    pub delete: Option<bool>,
    /// [-p] - paste bracket control codes are inserted around the buffer
    #[cfg(feature = "tmux_1_7")]
    pub bracket_codes: Option<bool>,
    /// [-r] - do no replacement (equivalent to a separator of LF)
    #[cfg(feature = "tmux_1_0")]
    pub no_replacement: Option<bool>,
    /// [-b buffer-name] - specify the buffer mode
    #[cfg(feature = "tmux_1_7")]
    pub buffer_name: Option<&'a str>,
    /// [-s separator] - specify a separator
    #[cfg(feature = "tmux_1_3")]
    pub separator: Option<&'a str>,
    /// [-t target-pane] - specify the target pane
    #[cfg(feature = "tmux_1_7")]
    pub target_pane: Option<&'a str>,
    /// [-t target-window] - specify the target window
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    pub target_window: Option<&'a str>,
}

impl<'a> PasteBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct PasteBufferBuilder<'a> {
    #[cfg(feature = "tmux_0_8")]
    pub delete: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub bracket_codes: Option<bool>,
    #[cfg(feature = "tmux_1_0")]
    pub no_replacement: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub buffer_name: Option<&'a str>,
    #[cfg(feature = "tmux_1_3")]
    pub separator: Option<&'a str>,
    #[cfg(feature = "tmux_1_7")]
    pub target_pane: Option<&'a str>,
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    pub target_window: Option<&'a str>,
}

impl<'a> PasteBufferBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn delete(&mut self) -> &mut Self {
        self.delete = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn bracket_codes(&mut self) -> &mut Self {
        self.bracket_codes = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn no_replacement(&mut self) -> &mut Self {
        self.no_replacement = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn buffer_name(&mut self, buffer_name: &'a str) -> &mut Self {
        self.buffer_name = Some(buffer_name);
        self
    }

    #[cfg(feature = "tmux_1_3")]
    pub fn separator(&mut self, separator: &'a str) -> &mut Self {
        self.separator = Some(separator);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn target_pane(&mut self, target_pane: &'a str) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    pub fn target_window(&mut self, target_window: &'a str) -> &mut Self {
        self.target_window = Some(target_window);
        self
    }

    pub fn build(&self) -> PasteBuffer<'a> {
        PasteBuffer {
            #[cfg(feature = "tmux_0_8")]
            delete: self.delete,
            #[cfg(feature = "tmux_1_7")]
            bracket_codes: self.bracket_codes,
            #[cfg(feature = "tmux_1_0")]
            no_replacement: self.no_replacement,
            #[cfg(feature = "tmux_1_7")]
            buffer_name: self.buffer_name,
            #[cfg(feature = "tmux_1_3")]
            separator: self.separator,
            #[cfg(feature = "tmux_1_7")]
            target_pane: self.target_pane,
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
            target_window: self.target_window,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const PASTE_BUFFER: &'static str = "paste-buffer";
    #[cfg(feature = "use_cmd_alias")]
    const PASTE_BUFFER: &'static str = "pasteb";

    /// Insert the contents of a paste buffer into the specified pane.
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
    pub fn paste_buffer(&mut self, paste_buffer: Option<&PasteBuffer>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(paste_buffer) = paste_buffer {
            #[cfg(feature = "tmux_0_8")]
            if paste_buffer.delete.unwrap_or(false) {
                args.push(d_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if paste_buffer.bracket_codes.unwrap_or(false) {
                args.push(p_KEY);
            }
            #[cfg(feature = "tmux_1_0")]
            if paste_buffer.no_replacement.unwrap_or(false) {
                args.push(r_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if let Some(buffer_name) = paste_buffer.buffer_name {
                args.extend_from_slice(&[b_KEY, &buffer_name])
            }
            #[cfg(feature = "tmux_1_3")]
            if let Some(separator) = paste_buffer.separator {
                args.extend_from_slice(&[s_KEY, &separator])
            }
            #[cfg(feature = "tmux_1_7")]
            if let Some(target_pane) = paste_buffer.target_pane {
                args.extend_from_slice(&[t_KEY, &target_pane])
            }
        }
        let output = self.command(TmuxInterface::PASTE_BUFFER, &args)?;
        Ok(output)
    }
}
