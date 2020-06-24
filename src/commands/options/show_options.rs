use crate::error::Error;
use crate::tmux_interface::*;

/// Structure for showing options
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux show-options [-AgHpqsvw] [-t target-pane] [option]
/// (alias: show)
/// ```
///
/// tmux ^1.8:
/// ```text
/// tmux show-options [-gqsvw] [-t target-session | target-window] [option]
/// (alias: show)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux show-options [-gsw] [-t target-session | target-window] [option]
/// (alias: show)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux show-options [-gsw] [-t target-session | target-window]
/// (alias: show)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux show-options [-t target-session]
/// (alias: show)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux show-options [-t target-session] option value
/// (alias: show)
/// ```
#[derive(Default, Debug)]
pub struct ShowOptions<'a> {
    /// [-A] - includes options inherited from a parent set of options
    #[cfg(feature = "tmux_3_0")]
    pub include_inherited: Option<bool>,
    /// [-g] - global session or window options are listed
    #[cfg(feature = "tmux_1_2")]
    pub global: Option<bool>,
    /// [-H] - includes hooks (omitted by default)
    #[cfg(feature = "tmux_3_0")]
    pub hooks: Option<bool>,
    /// [-p] - show window options
    #[cfg(feature = "tmux_3_0")]
    pub pane: Option<bool>,
    /// [-q] - no error will be returned if `option` is unset
    #[cfg(feature = "tmux_1_8")]
    pub quiet: Option<bool>,
    /// [-s] - show the server options
    #[cfg(feature = "tmux_1_2")]
    pub server: Option<bool>,
    /// [-v] - shows only the option value
    #[cfg(feature = "tmux_1_8")]
    pub value: Option<bool>,
    /// [-w] - show the window options
    #[cfg(feature = "tmux_1_2")]
    pub window: Option<bool>,
    /// [-t target-pane] - target session or window name
    //#[cfg(feature = "tmux_X_X")]
    pub target: Option<&'a str>,
    /// [option] - specify option name
    #[cfg(feature = "tmux_1_7")]
    pub option: Option<&'a str>,
}

impl<'a> ShowOptions<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct ShowOptionsBuilder<'a> {
    #[cfg(feature = "tmux_3_0")]
    pub include_inherited: Option<bool>,
    #[cfg(feature = "tmux_1_2")]
    pub global: Option<bool>,
    #[cfg(feature = "tmux_3_0")]
    pub hooks: Option<bool>,
    #[cfg(feature = "tmux_3_0")]
    pub pane: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub quiet: Option<bool>,
    #[cfg(feature = "tmux_1_2")]
    pub server: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub value: Option<bool>,
    #[cfg(feature = "tmux_1_2")]
    pub window: Option<bool>,
    //#[cfg(feature = "tmux_X_X")]
    pub target: Option<&'a str>,
    #[cfg(feature = "tmux_1_7")]
    pub option: Option<&'a str>,
}

impl<'a> ShowOptionsBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn include_inherited(&mut self) -> &mut Self {
        self.include_inherited = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn global(&mut self) -> &mut Self {
        self.global = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn hooks(&mut self) -> &mut Self {
        self.hooks = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn pane(&mut self) -> &mut Self {
        self.pane = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn quiet(&mut self) -> &mut Self {
        self.quiet = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn server(&mut self) -> &mut Self {
        self.server = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn value(&mut self) -> &mut Self {
        self.value = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn window(&mut self) -> &mut Self {
        self.window = Some(true);
        self
    }

    //#[cfg(feature = "tmux_X_X")]
    pub fn target(&mut self, target: &'a str) -> &mut Self {
        self.target = Some(target);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn option(&mut self, option: &'a str) -> &mut Self {
        self.option = Some(option);
        self
    }

    pub fn build(&self) -> ShowOptions<'a> {
        ShowOptions {
            #[cfg(feature = "tmux_3_0")]
            include_inherited: self.include_inherited,
            #[cfg(feature = "tmux_1_2")]
            global: self.global,
            #[cfg(feature = "tmux_3_0")]
            hooks: self.hooks,
            #[cfg(feature = "tmux_3_0")]
            pane: self.pane,
            #[cfg(feature = "tmux_1_8")]
            quiet: self.quiet,
            #[cfg(feature = "tmux_1_2")]
            server: self.server,
            #[cfg(feature = "tmux_1_8")]
            value: self.value,
            #[cfg(feature = "tmux_1_2")]
            window: self.window,
            //#[cfg(feature = "tmux_X_X")]
            target: self.target,
            #[cfg(feature = "tmux_1_7")]
            option: self.option,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const SHOW_OPTIONS: &'static str = "show-options";

    // XXX: better result type?
    /// # Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// tmux show-options [-AgHpqsvw] [-t target-pane] [option]
    /// (alias: show)
    /// ```
    ///
    /// tmux ^1.8:
    /// ```text
    /// tmux show-options [-gqsvw] [-t target-session | target-window] [option]
    /// (alias: show)
    /// ```
    ///
    /// tmux ^1.7:
    /// ```text
    /// tmux show-options [-gsw] [-t target-session | target-window] [option]
    /// (alias: show)
    /// ```
    ///
    /// tmux ^1.2:
    /// ```text
    /// tmux show-options [-gsw] [-t target-session | target-window]
    /// (alias: show)
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux show-options [-t target-session]
    /// (alias: show)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux show-options [-t target-session] option value
    /// (alias: show)
    /// ```
    pub fn show_options(
        &mut self,
        show_options: Option<&ShowOptions>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(show_options) = show_options {
            #[cfg(feature = "tmux_3_0")]
            if show_options.include_inherited.unwrap_or(false) {
                args.push(A_KEY);
            }
            #[cfg(feature = "tmux_1_2")]
            if show_options.global.unwrap_or(false) {
                args.push(g_KEY);
            }
            #[cfg(feature = "tmux_3_0")]
            if show_options.hooks.unwrap_or(false) {
                args.push(H_KEY);
            }
            #[cfg(feature = "tmux_3_0")]
            if show_options.pane.unwrap_or(false) {
                args.push(p_KEY);
            }
            #[cfg(feature = "tmux_1_8")]
            if show_options.quiet.unwrap_or(false) {
                args.push(q_KEY);
            }
            #[cfg(feature = "tmux_1_2")]
            if show_options.server.unwrap_or(false) {
                args.push(s_KEY);
            }
            #[cfg(feature = "tmux_1_8")]
            if show_options.value.unwrap_or(false) {
                args.push(v_KEY);
            }
            #[cfg(feature = "tmux_1_2")]
            if show_options.window.unwrap_or(false) {
                args.push(w_KEY);
            }
            //#[cfg(feature = "tmux_X_X")]
            if let Some(target) = show_options.target {
                s = target.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            #[cfg(feature = "tmux_1_7")]
            if let Some(option) = show_options.option {
                args.push(&option)
            }
        }
        let output = self.command(TmuxInterface::SHOW_OPTIONS, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }
}
