use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
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
pub struct ChooseBuffer<'a, T: Display> {
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
    pub target_pane: Option<&'a T>,
    /// [template] - specify the template
    pub template: Option<&'a str>,
}

impl<'a, T: Display + Default> ChooseBuffer<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct ChooseBufferBuilder<'a, T: Display> {
    pub no_preview: Option<bool>,
    pub zoom: Option<bool>,
    pub reverse_sort_order: Option<bool>,
    pub format: Option<&'a str>,
    pub filter: Option<&'a str>,
    pub sort_order: Option<&'a str>,
    pub target_pane: Option<&'a T>,
    pub template: Option<&'a str>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a, T: Display + Default> ChooseBufferBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn no_preview(&mut self) -> &mut Self {
        self.no_preview = Some(true);
        self
    }

    pub fn zoom(&mut self) -> &mut Self {
        self.zoom = Some(true);
        self
    }

    pub fn reverse_sort_order(&mut self) -> &mut Self {
        self.reverse_sort_order = Some(true);
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

    pub fn build(&self) -> ChooseBuffer<'a, T> {
        ChooseBuffer {
            no_preview: self.no_preview,
            zoom: self.zoom,
            reverse_sort_order: self.reverse_sort_order,
            format: self.format,
            filter: self.filter,
            sort_order: self.sort_order,
            target_pane: self.target_pane,
            template: self.template,
        }
    }
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct ChooseBuffer<'a, T: Display> {
    /// [-N] - start without the preview
    pub no_preview: Option<bool>,
    /// [-F] - specify the format for each item in the list
    pub format: Option<&'a str>,
    /// [-f filter] - specify an initial filter
    pub filter: Option<&'a str>,
    /// [-O sort-order] - specifies the initial sort field
    pub sort_order: Option<&'a str>,
    /// [-t target-pane] - specify the target pane
    pub target_pane: Option<&'a T>,
    /// [template] - specify the template
    pub template: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct ChooseBufferBuilder<'a, T: Display> {
    pub no_preview: Option<bool>,
    pub format: Option<&'a str>,
    pub filter: Option<&'a str>,
    pub sort_order: Option<&'a str>,
    pub target_pane: Option<&'a T>,
    pub template: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
impl<'a, T: Display + Default> ChooseBufferBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn no_preview(&mut self) -> &mut Self {
        self.no_preview = Some(true);
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

    pub fn build(&self) -> ChooseBuffer<'a, T> {
        ChooseBuffer {
            no_preview: self.no_preview,
            format: self.format,
            filter: self.filter,
            sort_order: self.sort_order,
            target_pane: self.target_pane,
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
    pub fn choose_buffer<T: Display>(
        &mut self,
        choose_buffer: Option<&ChooseBuffer<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
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
            if let Some(target_pane) = choose_buffer.target_pane {
                s = target_pane.to_string();
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
    pub fn choose_buffer<T: Display>(
        &mut self,
        choose_buffer: Option<&ChooseBuffer<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
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
            if let Some(target_pane) = choose_buffer.target_pane {
                s = target_pane.to_string();
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
