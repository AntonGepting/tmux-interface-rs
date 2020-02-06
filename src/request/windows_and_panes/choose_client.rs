use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Put a pane into client mode, allowing a client to be selected interactively from a list
///
/// # Manual
///
/// tmux X.X:
/// ```text
/// tmux choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane]
/// [template]
/// ```
///
/// tmux 2.6:
/// ```text
/// tmux choose-client [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane]
/// [template]
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct ChooseClient<'a, T: Display> {
    /// [-N] - start without the preview
    pub without_preview: Option<bool>,
    /// [-r] - reverse the sort order
    pub reverse_sort_order: Option<bool>,
    /// [-Z] - zoom the pane
    pub zoom: Option<bool>,
    /// [-F format] - format
    pub format: Option<&'a str>,
    /// [-f filter] - specify an initial filter
    pub filter: Option<&'a str>,
    /// [-O sort-order] - specify the initial sort field
    pub sort_order: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a T>,
    /// [template] - template
    pub template: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct ChooseClient<'a, T: Display> {
    /// [-N] - start without the preview
    pub without_preview: Option<bool>,
    /// [-F format] - format
    pub format: Option<&'a str>,
    /// [-f filter] - specify an initial filter
    pub filter: Option<&'a str>,
    /// [-O sort-order] - specify the initial sort field
    pub sort_order: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a T>,
    /// [template] - template
    pub template: Option<&'a str>,
}

impl<'a, T: Display + Default> ChooseClient<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a> TmuxInterface<'a> {
    const CHOOSE_CLIENT: &'static str = "choose-client";

    /// Put a pane into client mode, allowing a client to be selected interactively from a list
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane]
    /// [template]
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux choose-client [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane]
    /// [template]
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn choose_client<T: Display>(
        &mut self,
        choose_client: Option<&ChooseClient<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(choose_client) = choose_client {
            if choose_client.without_preview.unwrap_or(false) {
                args.push(N_KEY);
            }
            if choose_client.reverse_sort_order.unwrap_or(false) {
                args.push(r_KEY);
            }
            if choose_client.zoom.unwrap_or(false) {
                args.push(Z_KEY);
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
            if let Some(target_pane) = choose_client.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = choose_client.template {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::CHOOSE_CLIENT, &args)?;
        Ok(output)
    }

    /// Put a pane into client mode, allowing a client to be selected interactively from a list
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane]
    /// [template]
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux choose-client [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane]
    /// [template]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn choose_client<T: Display>(
        &mut self,
        choose_client: Option<&ChooseClient<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(choose_client) = choose_client {
            if choose_client.without_preview.unwrap_or(false) {
                args.push(N_KEY);
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
            if let Some(target_pane) = choose_client.target_pane {
                s = target_pane.to_string();
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
