use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Put a pane into client mode, allowing a client to be selected interactively from a list
///
/// # Manual
///
/// tmux X.X
/// ```text
/// tmux choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane]
/// [template]
/// ```
///
/// tmux 2.6
/// ```text
/// tmux choose-client [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane]
/// [template]
/// ```
#[derive(Default, Debug)]
pub struct ChooseClient<'a> {
    /// [-N] - start without the preview
    pub without_preview: Option<bool>,
    #[cfg(not(feature = "tmux_2_6"))]
    /// [-r] - reverse the sort order
    pub reverse_sort_order: Option<bool>,
    #[cfg(not(feature = "tmux_2_6"))]
    /// [-Z] - zoom the pane
    pub zoom: Option<bool>,
    /// [-F format] - format
    pub format: Option<&'a str>,
    /// [-f filter] - specify an initial filter
    pub filter: Option<&'a str>,
    /// [-O sort-order] - specify the initial sort field
    pub sort_order: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
    /// [template] - template
    pub template: Option<&'a str>,
}

impl<'a> ChooseClient<'a> {
    pub fn new() -> ChooseClient<'a> {
        Default::default()
    }
}

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const CHOOSE_CLIENT: &'static str = "choose-client";

    /// Put a pane into client mode, allowing a client to be selected interactively from a list
    ///
    /// # Manual
    ///
    /// tmux X.X
    /// ```text
    /// tmux choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane]
    /// [template]
    /// ```
    ///
    /// tmux 2.6
    /// ```text
    /// tmux choose-client [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane]
    /// [template]
    /// ```
    pub fn choose_client(&mut self, choose_client: Option<&ChooseClient>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(choose_client) = choose_client {
            if choose_client.without_preview.unwrap_or(false) {
                args.push(N_KEY);
            }
            if cfg!(not(feature = "tmux_2_6")) {
                if choose_client.reverse_sort_order.unwrap_or(false) {
                    args.push(r_KEY);
                }
                if choose_client.zoom.unwrap_or(false) {
                    args.push(Z_KEY);
                }
            }
            if let Some(s) = choose_client.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
            if let Some(s) = choose_client.filter {
                args.extend_from_slice(&[f_KEY, &s])
            }
            if let Some(s) = choose_client.sort_order {
                args.extend_from_slice(&[O_KEY, &s])
            }
            if let Some(s) = choose_client.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = choose_client.template {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::CHOOSE_CLIENT, &args)?;
        Ok(output)
    }
}
