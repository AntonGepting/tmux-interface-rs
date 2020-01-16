use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Stucture for putting a pane into buffer mode
///
/// # Manual
///
/// tmux X.X:
/// ```text
/// tmux choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux 2.6:
/// ```text
/// tmux choose-buffer [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
#[cfg(not(feature = "tmux_2_6"))]
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

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct ChooseBuffer<'a> {
    /// [-N] - start without the preview
    pub no_preview: Option<bool>,
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

impl<'a> TmuxInterface<'a> {
    const CHOOSE_BUFFER: &'static str = "choose-buffer";

    /// Put a pane into buffer mode, where a buffer may be chosen interactively from a list.
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux choose-buffer [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
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
                args.push(r_KEY);
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

    /// Put a pane into buffer mode, where a buffer may be chosen interactively from a list.
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux choose-buffer [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn choose_buffer(&mut self, choose_buffer: Option<&ChooseBuffer>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(choose_buffer) = choose_buffer {
            if choose_buffer.no_preview.unwrap_or(false) {
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
}
