use crate::commands::constants::*;
#[cfg(feature = "tmux_2_9a")]
use crate::ClientFlags;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;
#[cfg(feature = "tmux_3_2")]
use std::fmt;

#[derive(Debug, Clone)]
#[cfg(feature = "tmux_3_2")]
pub enum State {
    On,
    Off,
    Continue,
    Pause,
}

#[cfg(feature = "tmux_3_2")]
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::On => "on",
            Self::Off => "off",
            Self::Continue => "continue",
            Self::Pause => "pause",
        };
        write!(f, "{}", s)
    }
}

/// Structure for refreshing the current client
///
/// # Manual
///
/// tmux 3.2:
/// ```text
/// tmux refresh-client [-cDlLRSU] [-A pane:state] [-B name:what:format] [-C XxY] [-f flags] [-t target-client] [adjustment]
/// (alias: refresh)
/// ```
///
/// tmux 3.0:
/// ```text
/// tmux refresh-client [-cDlLRSU] [-C XxY] [-F flags] [-t target-client] [adjustment]
/// (alias: refresh)
/// ```
///
/// tmux 2.9a:
/// ```text
/// tmux refresh-client [-cDlLRSU] [-C width,height] [-F flags] [-t target-client] [adjustment]
/// (alias: refresh)
/// ```
///
/// tmux 2.4:
/// ```text
/// tmux refresh-client [-C width,height] [-S] [-t target-client]
/// (alias: refresh)
/// ```
///
/// tmux 1.6:
/// ```text
/// tmux refresh-client [-S] [-t target-client]
/// (alias: refresh)
/// ```
///
/// tmux 0.8:
/// ```text
/// tmux refresh-client [-t target-client]
/// (alias: refresh)
/// ```
#[derive(Debug, Clone)]
pub struct RefreshClient<'a>(pub TmuxCommand<'a>);

impl<'a> Default for RefreshClient<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(REFRESH_CLIENT)),
            ..Default::default()
        })
    }
}

impl<'a> RefreshClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-c]` - return to tracking the cursor automatically
    #[cfg(feature = "tmux_2_9a")]
    pub fn tracking_cursor(&mut self) -> &mut Self {
        self.0.push_flag(C_LOWERCASE_KEY);
        self
    }

    /// `[-D]` - move the visible part of a window down by `adjustment` rows
    #[cfg(feature = "tmux_2_9a")]
    pub fn down(&mut self) -> &mut Self {
        self.0.push_flag(D_UPPERCASE_KEY);
        self
    }

    /// `[-l]` - request the clipboard from the client using the xterm(1) escape sequence
    #[cfg(feature = "tmux_2_9a")]
    pub fn request_clipboard(&mut self) -> &mut Self {
        self.0.push_flag(L_LOWERCASE_KEY);
        self
    }

    /// `[-L]` - move the visible part of a window left by `adjustment` columns
    #[cfg(feature = "tmux_2_9a")]
    pub fn left(&mut self) -> &mut Self {
        self.0.push_flag(L_UPPERCASE_KEY);
        self
    }

    /// `[-R]` - move the visible part of a window right by `adjustment` columns
    #[cfg(feature = "tmux_2_9a")]
    pub fn right(&mut self) -> &mut Self {
        self.0.push_flag(R_UPPERCASE_KEY);
        self
    }

    /// `[-S]` - only update the client's status line
    #[cfg(feature = "tmux_1_6")]
    pub fn status_line(&mut self) -> &mut Self {
        self.0.push_flag(S_UPPERCASE_KEY);
        self
    }

    /// `[-U]` - move the visible part of a window up by `adjustment` rows
    #[cfg(feature = "tmux_2_9a")]
    pub fn up(&mut self) -> &mut Self {
        self.0.push_flag(U_UPPERCASE_KEY);
        self
    }

    // TODO: accept target_pane
    /// `[-A pane:state]` - allows a control mode client to trigger actions on a pane
    #[cfg(feature = "tmux_3_2")]
    pub fn allow_actions(&mut self, pane: usize, state: State) -> &mut Self {
        self.0
            .push_option(A_UPPERCASE_KEY, format!("%{}:{}", pane, state));
        self
    }

    // TODO: refactor, accept target_pane, target_window and masks * ...
    /// [-B name:what:format]
    #[cfg(feature = "tmux_3_2")]
    pub fn subscribe(
        &mut self,
        name: usize,
        what: Option<String>,
        format: Option<String>,
    ) -> &mut Self {
        let mut arg = format!("%{}", name);
        if let Some(s) = what {
            arg = format!("{}:{}", arg, s);
            if let Some(s) = format {
                arg = format!("{}:{}", arg, s);
            }
        }
        self.0.push_option(B_UPPERCASE_KEY, arg);
        self
    }

    /// `[-C X,Y]` - set the width and height of a control client
    /// `[-C XxY]` - set the width and height of a control client
    #[cfg(feature = "tmux_2_4")]
    pub fn size(&mut self, size: (usize, usize)) -> &mut Self {
        #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_0")))]
        let s = format!("{},{}", size.0, size.1);
        // FIXME: multiple incompatible def's
        #[cfg(feature = "tmux_3_0")]
        let s = format!("{}x{}", size.0, size.1);

        self.0.push_option(C_UPPERCASE_KEY, s);
        self
    }

    // XXX: refactor vec?
    /// `[-F flags]` - set a comma-separated list of flags
    #[cfg(all(feature = "tmux_2_9a", not(feature = "tmux_3_2")))]
    pub fn flags(&mut self, flags: ClientFlags) -> &mut Self {
        self.0.push_option(F_UPPERCASE_KEY, flags.to_string());
        self
    }

    // XXX: refactor vec?
    /// `[-f flags]` - sets a comma-separated list of client flags
    #[cfg(feature = "tmux_3_2")]
    pub fn flags(&mut self, flags: ClientFlags) -> &mut Self {
        self.0.push_option(F_LOWERCASE_KEY, flags.to_string());
        self
    }

    /// `[-t target-client]` - specify the client
    #[cfg(feature = "tmux_0_8")]
    pub fn target_client(&mut self, target_client: &'a str) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_client);
        self
    }

    /// `[adjustment]` - moves the visible part up/down left/right by adjustment rows/columns
    #[cfg(feature = "tmux_2_9a")]
    pub fn adjustment(&mut self, adjustment: usize) -> &mut Self {
        self.0.push_param(adjustment.to_string());
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for RefreshClient<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(REFRESH_CLIENT)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for RefreshClient<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(REFRESH_CLIENT)),
            ..Default::default()
        })
    }
}
