use crate::commands::constants::*;
#[cfg(feature = "tmux_2_9a")]
use crate::ClientFlags;
use crate::TmuxCommand;
use std::borrow::Cow;
#[cfg(feature = "tmux_3_2")]
use std::fmt;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RefreshClient<'a> {
    /// `[-c]` - return to tracking the cursor automatically
    #[cfg(feature = "tmux_2_9a")]
    pub tracking_cursor: bool,

    /// `[-D]` - move the visible part of a window down by `adjustment` rows
    #[cfg(feature = "tmux_2_9a")]
    pub down: bool,

    /// `[-l]` - request the clipboard from the client using the xterm(1) escape sequence
    #[cfg(feature = "tmux_2_9a")]
    pub request_clipboard: bool,

    /// `[-L]` - move the visible part of a window left by `adjustment` columns
    #[cfg(feature = "tmux_2_9a")]
    pub left: bool,

    /// `[-R]` - move the visible part of a window right by `adjustment` columns
    #[cfg(feature = "tmux_2_9a")]
    pub right: bool,

    /// `[-S]` - only update the client's status line
    #[cfg(feature = "tmux_1_6")]
    pub status_line: bool,

    /// `[-U]` - move the visible part of a window up by `adjustment` rows
    #[cfg(feature = "tmux_2_9a")]
    pub up: bool,

    // TODO: accept target_pane
    /// `[-A pane:state]` - allows a control mode client to trigger actions on a pane
    #[cfg(feature = "tmux_3_2")]
    pub allow_actions: Option<usize, State>,

    // TODO: refactor, accept target_pane, target_window and masks * ...
    /// [-B name:what:format]
    #[cfg(feature = "tmux_3_2")]
    pub subscribe: Option<(usize, usize)>,

    /// `[-C X,Y]` - set the width and height of a control client
    /// `[-C XxY]` - set the width and height of a control client
    #[cfg(feature = "tmux_2_4")]
    pub size: Option<(usize, usize)>,

    // XXX: refactor vec?
    /// `[-F flags]` - set a comma-separated list of flags
    #[cfg(all(feature = "tmux_2_9a", not(feature = "tmux_3_2")))]
    pub flags: Option<ClientFlags>,

    // XXX: refactor vec?
    /// `[-f flags]` - sets a comma-separated list of client flags
    #[cfg(feature = "tmux_3_2")]
    pub flags: Option<ClientFlags>,

    /// `[-t target-client]` - specify the client
    #[cfg(feature = "tmux_0_8")]
    pub target_client: Option<Cow<'a, str>>,

    /// `[adjustment]` - moves the visible part up/down left/right by adjustment rows/columns
    #[cfg(feature = "tmux_2_9a")]
    pub adjustment: Option<usize>,
}

impl<'a> RefreshClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-c]` - return to tracking the cursor automatically
    #[cfg(feature = "tmux_2_9a")]
    pub fn tracking_cursor(&mut self) -> &mut Self {
        self.tracking_cursor = true;
        self
    }

    /// `[-D]` - move the visible part of a window down by `adjustment` rows
    #[cfg(feature = "tmux_2_9a")]
    pub fn down(&mut self) -> &mut Self {
        self.down = true;
        self
    }

    /// `[-l]` - request the clipboard from the client using the xterm(1) escape sequence
    #[cfg(feature = "tmux_2_9a")]
    pub fn request_clipboard(&mut self) -> &mut Self {
        self.request_clipboard = true;
        self
    }

    /// `[-L]` - move the visible part of a window left by `adjustment` columns
    #[cfg(feature = "tmux_2_9a")]
    pub fn left(&mut self) -> &mut Self {
        self.left = true;
        self
    }

    /// `[-R]` - move the visible part of a window right by `adjustment` columns
    #[cfg(feature = "tmux_2_9a")]
    pub fn right(&mut self) -> &mut Self {
        self.right = true;
        self
    }

    /// `[-S]` - only update the client's status line
    #[cfg(feature = "tmux_1_6")]
    pub fn status_line(&mut self) -> &mut Self {
        self.status_line = true;
        self
    }

    /// `[-U]` - move the visible part of a window up by `adjustment` rows
    #[cfg(feature = "tmux_2_9a")]
    pub fn up(&mut self) -> &mut Self {
        self.up = true;
        self
    }

    // TODO: accept target_pane
    /// `[-A pane:state]` - allows a control mode client to trigger actions on a pane
    #[cfg(feature = "tmux_3_2")]
    pub fn allow_actions(&mut self, pane: usize, state: State) -> &mut Self {
        self.allow_actions = Some((pane, state));
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
        self.subscribe = Some((name, what, format));
        self
    }

    /// `[-C X,Y]` - set the width and height of a control client
    /// `[-C XxY]` - set the width and height of a control client
    #[cfg(feature = "tmux_2_4")]
    pub fn size(&mut self, size: (usize, usize)) -> &mut Self {
        self.size = Some(size);
        self
    }

    // XXX: refactor vec?
    /// `[-F flags]` - set a comma-separated list of flags
    #[cfg(all(feature = "tmux_2_9a", not(feature = "tmux_3_2")))]
    pub fn flags(&mut self, flags: ClientFlags) -> &mut Self {
        self.flags = Some(flags);
        self
    }

    // XXX: refactor vec?
    /// `[-f flags]` - sets a comma-separated list of client flags
    #[cfg(feature = "tmux_3_2")]
    pub fn flags(&mut self, flags: ClientFlags) -> &mut Self {
        self.flags = Some(flags);
        self
    }

    /// `[-t target-client]` - specify the client
    #[cfg(feature = "tmux_0_8")]
    pub fn target_client(&mut self, target_client: &'a str) -> &mut Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[adjustment]` - moves the visible part up/down left/right by adjustment rows/columns
    #[cfg(feature = "tmux_2_9a")]
    pub fn adjustment(&mut self, adjustment: usize) -> &mut Self {
        self.adjustment = Some(adjustment);
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(REFRESH_CLIENT);

        // `[-c]` - return to tracking the cursor automatically
        #[cfg(feature = "tmux_2_9a")]
        if self.tracking_cursor {
            cmd.push_flag(C_LOWERCASE_KEY);
        }

        // `[-D]` - move the visible part of a window down by `adjustment` rows
        #[cfg(feature = "tmux_2_9a")]
        if self.down {
            cmd.push_flag(D_UPPERCASE_KEY);
        }

        // `[-l]` - request the clipboard from the client using the xterm(1) escape sequence
        #[cfg(feature = "tmux_2_9a")]
        if self.request_clipboard {
            cmd.push_flag(L_LOWERCASE_KEY);
        }

        // `[-L]` - move the visible part of a window left by `adjustment` columns
        #[cfg(feature = "tmux_2_9a")]
        if self.left {
            cmd.push_flag(L_UPPERCASE_KEY);
        }

        // `[-R]` - move the visible part of a window right by `adjustment` columns
        #[cfg(feature = "tmux_2_9a")]
        if self.right {
            cmd.push_flag(R_UPPERCASE_KEY);
        }

        // `[-S]` - only update the client's status line
        #[cfg(feature = "tmux_1_6")]
        if self.status_line {
            cmd.push_flag(S_UPPERCASE_KEY);
        }

        // `[-U]` - move the visible part of a window up by `adjustment` rows
        #[cfg(feature = "tmux_2_9a")]
        if self.up {
            cmd.push_flag(U_UPPERCASE_KEY);
        }

        // TODO: accept target_pane
        // `[-A pane:state]` - allows a control mode client to trigger actions on a pane
        #[cfg(feature = "tmux_3_2")]
        if let Some(allow_actions) = self.allow_actions {
            cmd.push_option(A_UPPERCASE_KEY, format!("%{}:{}", pane, state));
        }

        // TODO: refactor, accept target_pane, target_window and masks * ...
        // [-B name:what:format]
        #[cfg(feature = "tmux_3_2")]
        if let Some(subscribe) = self.subscribe {
            let mut arg = format!("%{}", name);
            if let Some(s) = what {
                arg = format!("{}:{}", arg, s);
                if let Some(s) = format {
                    arg = format!("{}:{}", arg, s);
                }
            }
            cmd.push_option(B_UPPERCASE_KEY, arg);
        }

        // `[-C X,Y]` - set the width and height of a control client
        // `[-C XxY]` - set the width and height of a control client
        #[cfg(feature = "tmux_2_4")]
        if let Some(size) = self.size {
            #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_0")))]
            let s = format!("{},{}", size.0, size.1);
            // FIXME: multiple incompatible def's
            #[cfg(feature = "tmux_3_0")]
            let s = format!("{}x{}", size.0, size.1);

            cmd.push_option(C_UPPERCASE_KEY, s);
        }

        // XXX: refactor vec?
        // `[-F flags]` - set a comma-separated list of flags
        #[cfg(all(feature = "tmux_2_9a", not(feature = "tmux_3_2")))]
        if let Some(flags) = &self.flags {
            cmd.push_option(F_UPPERCASE_KEY, flags.to_string());
        }

        // XXX: refactor vec?
        // `[-f flags]` - sets a comma-separated list of client flags
        #[cfg(feature = "tmux_3_2")]
        if let Some(flags) = self.flags {
            cmd.push_option(F_LOWERCASE_KEY, flags.to_string());
        }

        // `[-t target-client]` - specify the client
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_client) = &self.target_client {
            cmd.push_option(T_LOWERCASE_KEY, target_client.as_ref());
        }

        // `[adjustment]` - moves the visible part up/down left/right by adjustment rows/columns
        #[cfg(feature = "tmux_2_9a")]
        if let Some(adjustment) = self.adjustment {
            cmd.push_param(adjustment.to_string());
        }

        cmd
    }
}
