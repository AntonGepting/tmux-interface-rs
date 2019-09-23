use super::error::Error;
use super::tmux_interface::*;
use std::process::Output;

/// # Manual
///
/// ```text
/// tmux choose-buffer [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
#[derive(Default)]
pub struct ChooseBuffer<'a> {
    pub no_preview: Option<bool>,     // [-N]
    pub zoom: Option<bool>,           // [-Z]
    pub format: Option<&'a str>,      // [-F]
    pub filter: Option<&'a str>,      // [-f filter]
    pub sort_order: Option<&'a str>,  // [-O sort-order]
    pub target_pane: Option<&'a str>, // [-t target-pane]
    pub template: Option<&'a str>,    // [template]
}

impl<'a> ChooseBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// # Manual
///
/// ```text
/// tmux paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
/// (alias: pasteb)
/// ```
#[derive(Default)]
pub struct PasteBuffer<'a> {
    pub delete: Option<bool>,         // [-d]
    pub bracket_codes: Option<bool>,  // [-p]
    pub no_replacement: Option<bool>, // [-r]
    pub buffer_name: Option<&'a str>, // [-b buffer-name]
    pub separator: Option<&'a str>,   // [-s separator]
    pub target_pane: Option<&'a str>, // [-t target-pane]
}

impl<'a> PasteBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Buffers
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

    /// # Manual
    ///
    /// ```text
    /// tmux choose-buffer [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane]
    /// [template]
    /// ```
    pub fn choose_buffer(&self, choose_buffer: &ChooseBuffer) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if choose_buffer.no_preview.unwrap_or(false) {
            args.push(N_KEY);
        }
        if choose_buffer.zoom.unwrap_or(false) {
            args.push(Z_KEY);
        }
        choose_buffer
            .format
            .and_then(|s| Some(args.extend_from_slice(&[F_KEY, &s])));
        choose_buffer
            .filter
            .and_then(|s| Some(args.extend_from_slice(&[f_KEY, &s])));
        choose_buffer
            .sort_order
            .and_then(|s| Some(args.extend_from_slice(&[O_KEY, &s])));
        choose_buffer
            .target_pane
            .and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        choose_buffer.template.and_then(|s| Some(args.push(&s)));
        let output = self.subcommand(TmuxInterface::CHOOSE_BUFFER, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// ```text
    /// tmux clear-history [-t target-pane]
    /// (alias: clearhist)
    /// ```
    pub fn clear_history(&self, target_pane: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        target_pane.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::CLEAR_HISTORY, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// ```text
    /// tmux delete-buffer [-b buffer-name]
    /// (alias: deleteb)
    /// ```
    pub fn delete_buffer(&self, buffer_name: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        buffer_name.and_then(|s| Some(args.extend_from_slice(&[b_KEY, &s])));
        let output = self.subcommand(TmuxInterface::DELETE_BUFFER, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// ```text
    /// tmux list-buffers [-F format]
    /// (alias: lsb)
    /// ```
    pub fn list_buffers(&self, format: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        format.and_then(|s| Some(args.extend_from_slice(&[F_KEY, &s])));
        let output = self.subcommand(TmuxInterface::LIST_BUFFERS, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// ```text
    /// tmux load-buffer [-b buffer-name] path
    /// (alias: loadb)
    /// ```
    pub fn load_buffer(&self, buffer_name: Option<&str>, path: &str) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        buffer_name.and_then(|s| Some(args.extend_from_slice(&[b_KEY, &s])));
        args.push(path);
        let output = self.subcommand(TmuxInterface::LOAD_BUFFER, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// ```text
    /// tmux paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
    /// (alias: pasteb)
    /// ```
    pub fn paste_buffer(&self, paste_buffer: &PasteBuffer) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if paste_buffer.delete.unwrap_or(false) {
            args.push(d_KEY);
        }
        if paste_buffer.bracket_codes.unwrap_or(false) {
            args.push(p_KEY);
        }
        if paste_buffer.no_replacement.unwrap_or(false) {
            args.push(r_KEY);
        }
        paste_buffer
            .buffer_name
            .and_then(|s| Some(args.extend_from_slice(&[b_KEY, &s])));
        paste_buffer
            .separator
            .and_then(|s| Some(args.extend_from_slice(&[s_KEY, &s])));
        paste_buffer
            .target_pane
            .and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::PASTE_BUFFER, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// ```text
    /// tmux save-buffer [-a] [-b buffer-name] path
    /// (alias: saveb)
    /// ```
    pub fn save_buffer(
        &self,
        append: Option<bool>,
        buffer_name: Option<&str>,
        path: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if append.unwrap_or(false) {
            args.push(a_KEY);
        }
        buffer_name.and_then(|s| Some(args.extend_from_slice(&[b_KEY, &s])));
        args.push(path);
        let output = self.subcommand(TmuxInterface::SAVE_BUFFER, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// ```text
    /// tmux set-buffer [-a] [-b buffer-name] [-n new-buffer-name] data
    /// (alias: setb)
    /// ```
    pub fn set_buffer(
        &self,
        append: Option<bool>,
        buffer_name: Option<&str>,
        new_buffer_name: Option<&str>,
        data: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if append.unwrap_or(false) {
            args.push(a_KEY);
        }
        buffer_name.and_then(|s| Some(args.extend_from_slice(&[b_KEY, &s])));
        new_buffer_name.and_then(|s| Some(args.extend_from_slice(&[n_KEY, &s])));
        args.push(data);
        let output = self.subcommand(TmuxInterface::SET_BUFFER, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// ```text
    /// tmux show-buffer [-b buffer-name]
    /// (alias: showb)
    /// ```
    pub fn show_buffer(&self, buffer_name: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        buffer_name.and_then(|s| Some(args.extend_from_slice(&[b_KEY, &s])));
        let output = self.subcommand(TmuxInterface::SHOW_BUFFER, &args)?;
        Ok(output)
    }
}
