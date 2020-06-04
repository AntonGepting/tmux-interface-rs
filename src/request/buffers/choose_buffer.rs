use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Stucture for putting a pane into buffer mode
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.7:
/// ```text
/// tmux choose-buffer [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux choose-buffer [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux choose-buffer [-F format] [-t target-pane] [template]
/// ```
///
/// tmux ^1.3:
/// ```text
/// tmux choose-buffer [-t target-pane] [template]
/// ```
#[derive(Default, Debug)]
pub struct ChooseBuffer<'a> {
    /// [-N] - start without the preview
    #[cfg(feature = "tmux_2_6")]
    pub no_preview: Option<bool>,
    /// [-Z] - zoom the pane
    #[cfg(feature = "tmux_2_7")]
    pub zoom: Option<bool>,
    /// [-r] - reverses the sort order
    #[cfg(feature = "tmux_3_1")]
    pub reverse_sort_order: Option<bool>,
    /// [-F] - specify the format for each item in the list
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<&'a str>,
    /// [-f filter] - specify an initial filter
    #[cfg(feature = "tmux_2_6")]
    pub filter: Option<&'a str>,
    /// [-O sort-order] - specifies the initial sort field
    #[cfg(feature = "tmux_2_6")]
    pub sort_order: Option<&'a str>,
    /// [-t target-pane] - specify the target pane
    #[cfg(feature = "tmux_1_3")]
    pub target_pane: Option<&'a str>,
    /// [template] - specify the template
    #[cfg(feature = "tmux_1_3")]
    pub template: Option<&'a str>,
}

impl<'a> ChooseBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct ChooseBufferBuilder<'a> {
    #[cfg(feature = "tmux_2_6")]
    pub no_preview: Option<bool>,
    #[cfg(feature = "tmux_2_7")]
    pub zoom: Option<bool>,
    #[cfg(feature = "tmux_3_1")]
    pub reverse_sort_order: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<&'a str>,
    #[cfg(feature = "tmux_2_6")]
    pub filter: Option<&'a str>,
    #[cfg(feature = "tmux_2_6")]
    pub sort_order: Option<&'a str>,
    #[cfg(feature = "tmux_1_3")]
    pub target_pane: Option<&'a str>,
    #[cfg(feature = "tmux_1_3")]
    pub template: Option<&'a str>,
}

impl<'a> ChooseBufferBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn no_preview(&mut self) -> &mut Self {
        self.no_preview = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_7")]
    pub fn zoom(&mut self) -> &mut Self {
        self.zoom = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_1")]
    pub fn reverse_sort_order(&mut self) -> &mut Self {
        self.reverse_sort_order = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn format(&mut self, format: &'a str) -> &mut Self {
        self.format = Some(format);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn filter(&mut self, filter: &'a str) -> &mut Self {
        self.filter = Some(filter);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn sort_order(&mut self, sort_order: &'a str) -> &mut Self {
        self.sort_order = Some(sort_order);
        self
    }

    #[cfg(feature = "tmux_1_3")]
    pub fn target_pane(&mut self, target_pane: &'a str) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    #[cfg(feature = "tmux_1_3")]
    pub fn template(&mut self, template: &'a str) -> &mut Self {
        self.template = Some(template);
        self
    }

    pub fn build(&self) -> ChooseBuffer<'a> {
        ChooseBuffer {
            #[cfg(feature = "tmux_2_6")]
            no_preview: self.no_preview,
            #[cfg(feature = "tmux_2_7")]
            zoom: self.zoom,
            #[cfg(feature = "tmux_3_1")]
            reverse_sort_order: self.reverse_sort_order,
            #[cfg(feature = "tmux_1_7")]
            format: self.format,
            #[cfg(feature = "tmux_2_6")]
            filter: self.filter,
            #[cfg(feature = "tmux_2_6")]
            sort_order: self.sort_order,
            #[cfg(feature = "tmux_1_3")]
            target_pane: self.target_pane,
            #[cfg(feature = "tmux_1_3")]
            template: self.template,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const CHOOSE_BUFFER: &'static str = "choose-buffer";

    /// Put a pane into buffer mode, where a buffer may be chosen interactively from a list.
    ///
    /// # Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// tmux choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    ///
    /// tmux ^2.7:
    /// ```text
    /// tmux choose-buffer [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    ///
    /// tmux ^2.6:
    /// ```text
    /// tmux choose-buffer [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    ///
    /// tmux ^1.7:
    /// ```text
    /// tmux choose-buffer [-F format] [-t target-pane] [template]
    /// ```
    ///
    /// tmux ^1.3:
    /// ```text
    /// tmux choose-buffer [-t target-pane] [template]
    /// ```
    pub fn choose_buffer(
        &mut self,
        choose_buffer: Option<&ChooseBuffer>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(choose_buffer) = choose_buffer {
            #[cfg(feature = "tmux_2_6")]
            if choose_buffer.no_preview.unwrap_or(false) {
                args.push(N_KEY);
            }
            #[cfg(feature = "tmux_2_7")]
            if choose_buffer.zoom.unwrap_or(false) {
                args.push(Z_KEY);
            }
            #[cfg(feature = "tmux_3_1")]
            if choose_buffer.reverse_sort_order.unwrap_or(false) {
                args.push(r_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if let Some(format) = choose_buffer.format {
                args.extend_from_slice(&[F_KEY, &format])
            }
            #[cfg(feature = "tmux_2_6")]
            if let Some(filter) = choose_buffer.filter {
                args.extend_from_slice(&[f_KEY, &filter])
            }
            #[cfg(feature = "tmux_2_6")]
            if let Some(sort_order) = choose_buffer.sort_order {
                args.extend_from_slice(&[O_KEY, &sort_order])
            }
            #[cfg(feature = "tmux_1_7")]
            if let Some(target_pane) = choose_buffer.target_pane {
                args.extend_from_slice(&[t_KEY, &target_pane])
            }
            #[cfg(feature = "tmux_1_3")]
            if let Some(template) = choose_buffer.template {
                args.push(&template)
            }
        }
        let output = self.subcommand(TmuxInterface::CHOOSE_BUFFER, &args)?;
        Ok(output)
    }
}
