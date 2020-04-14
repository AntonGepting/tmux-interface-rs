use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Put a pane into client mode, allowing a client to be selected interactively from a list
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.7:
/// ```text
/// tmux choose-client [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux choose-client [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux choose-client [-F format] [-t target-window] [template]
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux choose-client  [-t target-window] [template]
/// ```
#[derive(Default, Debug)]
pub struct ChooseClient<'a, T: Display> {
    /// [-N] - start without the preview
    pub without_preview: Option<bool>,
    #[cfg(feature = "tmux_X_X")]
    /// [-r] - reverse the sort order
    pub reverse_sort_order: Option<bool>,
    #[cfg(any(feature = "tmux_2_7", feature = "tmux_X_X"))]
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

impl<'a, T: Display + Default> ChooseClient<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct ChooseClientBuilder<'a, T: Display> {
    pub without_preview: Option<bool>,
    #[cfg(feature = "tmux_X_X")]
    pub reverse_sort_order: Option<bool>,
    #[cfg(any(feature = "tmux_2_7", feature = "tmux_X_X"))]
    pub zoom: Option<bool>,
    pub format: Option<&'a str>,
    pub filter: Option<&'a str>,
    pub sort_order: Option<&'a str>,
    pub target_pane: Option<&'a T>,
    pub template: Option<&'a str>,
}

impl<'a, T: Display + Default> ChooseClientBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn without_preview(&mut self) -> &mut Self {
        self.without_preview = Some(true);
        self
    }

    #[cfg(feature = "tmux_X_X")]
    pub fn reverse_sort_order(&mut self) -> &mut Self {
        self.reverse_sort_order = Some(true);
        self
    }

    #[cfg(any(feature = "tmux_2_7", feature = "tmux_X_X"))]
    pub fn zoom(&mut self) -> &mut Self {
        self.zoom = Some(true);
        self
    }

    pub fn format(&mut self, format: &'a str) -> &mut Self {
        self.format = Some(format);
        self
    }

    pub fn filter(&mut self, filter: &'a str) -> &mut Self {
        self.filter = Some(filter);
        self
    }

    pub fn sort_order(&mut self, sort_order: &'a str) -> &mut Self {
        self.sort_order = Some(sort_order);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn template(&mut self, template: &'a str) -> &mut Self {
        self.template = Some(template);
        self
    }

    pub fn build(&self) -> ChooseClient<'a, T> {
        ChooseClient {
            without_preview: self.without_preview,
            #[cfg(feature = "tmux_X_X")]
            reverse_sort_order: self.reverse_sort_order,
            #[cfg(any(feature = "tmux_2_7", feature = "tmux_X_X"))]
            zoom: self.zoom,
            format: self.format,
            filter: self.filter,
            sort_order: self.sort_order,
            target_pane: self.target_pane,
            template: self.template,
        }
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
            #[cfg(feature = "tmux_X_X")]
            {
                if choose_client.reverse_sort_order.unwrap_or(false) {
                    args.push(r_KEY);
                }
            }
            #[cfg(any(feature = "tmux_2_7", feature = "tmux_X_X"))]
            {
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
