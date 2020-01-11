use super::error::Error;
use super::tmux_interface::*;
use std::process::Output;

/// Stucture for putting a pane into buffer mode
///
/// # Manual
///
/// ```text
/// tmux choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
#[derive(Default, Debug)]
pub struct ChooseBuffer<'a> {
    /// [-N] - start without the preview
    pub no_preview: Option<bool>,
    /// [-Z] - zoom the pane
    pub zoom: Option<bool>,
    /// [-r] - reverses the sort order
    pub reverse_sort_order: Option<bool>,
    /// [-F] - specify the format for each item in the list
    pub format: Option<&'a str>,
    /// [-f filter] - specify an initial filter
    pub filter: Option<&'a str>,
    /// [-O sort-order] - specifies the initial sort field
    pub sort_order: Option<&'a str>,
    /// [-t target-pane] - specify the target pane
    pub target_pane: Option<&'a str>,
    /// [template] - specify the template
    pub template: Option<&'a str>,
}

impl<'a> ChooseBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

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
    const CHOOSE_BUFFER: &'static str = "choose-buffer";
    const CLEAR_HISTORY: &'static str = "clear-buffer";
    const DELETE_BUFFER: &'static str = "delete-buffer";
    const LIST_BUFFERS: &'static str = "list-buffer";
    const LOAD_BUFFER: &'static str = "load-buffer";
    const PASTE_BUFFER: &'static str = "paste-buffer";
    const SAVE_BUFFER: &'static str = "save-buffer";
    const SET_BUFFER: &'static str = "set-buffer";
    const SHOW_BUFFER: &'static str = "show-buffer";

    /// Put a pane into buffer mode, where a buffer may be chosen interactively from a list.
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order]
    /// [-t target-pane] [template]
    /// ```
    pub fn choose_buffer(&mut self, choose_buffer: Option<&ChooseBuffer>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(choose_buffer) = choose_buffer {
            if choose_buffer.no_preview.unwrap_or(false) {
                args.push(N_KEY);
            }
            if choose_buffer.zoom.unwrap_or(false) {
                args.push(Z_KEY);
            }
            if choose_buffer.reverse_sort_order.unwrap_or(false) {
                args.push(N_KEY);
            }
            if let Some(s) = choose_buffer.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
            if let Some(s) = choose_buffer.filter {
                args.extend_from_slice(&[f_KEY, &s])
            }
            if let Some(s) = choose_buffer.sort_order {
                args.extend_from_slice(&[O_KEY, &s])
            }
            if let Some(s) = choose_buffer.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = choose_buffer.template {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::CHOOSE_BUFFER, &args)?;
        Ok(output)
    }

    /// Remove and free the history for the specified pane.
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux clear-history [-t target-pane]
    /// (alias: clearhist)
    /// ```
    pub fn clear_history(&mut self, target_pane: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_pane {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::CLEAR_HISTORY, &args)?;
        Ok(output)
    }

    /// Delete the buffer named buffer-name, or the most recently added automatically named buffer
    /// if not specified.
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux delete-buffer [-b buffer-name]
    /// (alias: deleteb)
    /// ```
    pub fn delete_buffer(&mut self, buffer_name: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = buffer_name {
            args.extend_from_slice(&[b_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::DELETE_BUFFER, &args)?;
        Ok(output)
    }

    /// List the global buffers.
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux list-buffers [-F format]
    /// (alias: lsb)
    /// ```
    pub fn list_buffers(&mut self, format: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LIST_BUFFERS, &args)?;
        Ok(output)
    }

    /// Load the contents of the specified paste buffer from path.
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux load-buffer [-b buffer-name] path
    /// (alias: loadb)
    /// ```
    pub fn load_buffer(&mut self, buffer_name: Option<&str>, path: &str) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = buffer_name {
            args.extend_from_slice(&[b_KEY, &s])
        }
        args.push(path);
        let output = self.subcommand(TmuxInterface::LOAD_BUFFER, &args)?;
        Ok(output)
    }

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

    /// Save the contents of the specified paste buffer to path.
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux save-buffer [-a] [-b buffer-name] path
    /// (alias: saveb)
    /// ```
    pub fn save_buffer(
        &mut self,
        append: Option<bool>,
        buffer_name: Option<&str>,
        path: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if append.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = buffer_name {
            args.extend_from_slice(&[b_KEY, &s])
        }
        args.push(path);
        let output = self.subcommand(TmuxInterface::SAVE_BUFFER, &args)?;
        Ok(output)
    }

    /// Set the contents of the specified buffer to data.
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux set-buffer [-a] [-b buffer-name] [-n new-buffer-name] data
    /// (alias: setb)
    /// ```
    pub fn set_buffer(
        &mut self,
        append: Option<bool>,
        buffer_name: Option<&str>,
        new_buffer_name: Option<&str>,
        data: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if append.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = buffer_name {
            args.extend_from_slice(&[b_KEY, &s])
        }
        if let Some(s) = new_buffer_name {
            args.extend_from_slice(&[n_KEY, &s])
        }
        args.push(data);
        let output = self.subcommand(TmuxInterface::SET_BUFFER, &args)?;
        Ok(output)
    }

    /// Display the contents of the specified buffer.
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux show-buffer [-b buffer-name]
    /// (alias: showb)
    /// ```
    pub fn show_buffer(&mut self, buffer_name: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = buffer_name {
            args.extend_from_slice(&[b_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::SHOW_BUFFER, &args)?;
        Ok(output)
    }
}
