// auto-generated file
//

use crate::commands::constants::*;

#[cfg(feature = "tmux_2_9a")]
use crate::ClientFlags;
use crate::TmuxCommand;
#[cfg(feature = "tmux_3_2")]
use crate::{AllowActions, Subscribe};
use std::borrow::Cow;

pub type Refresh<'a> = RefreshClient<'a>;

/// Refresh current client
///
/// # Manual
///
/// tmux >=3.3:
/// ```text
/// refresh-client [-cDLRSU] [-A pane:state] [-B name:what:format] [-C XxY] [-f flags]
/// [-l [target-pane]] [-t target-client] [adjustment] (alias: refresh)
/// ```
///
/// tmux >=3.2:
/// ```text
/// refresh-client [-cDlLRSU] [-A pane:state] [-B name:what:format] [-C XxY] [-f flags] [-t target-client] [adjustment]
/// (alias: refresh)
/// ```
///
/// tmux >=3.0a:
/// ```text
/// refresh-client [-cDlLRSU] [-C XxY] [-F flags] [-t target-client] [adjustment]
/// (alias: refresh)
/// ```
///
/// tmux >=2.9:
/// ```text
/// refresh-client [-cDlLRSU] [-C width,height] [-F flags] [-t target-client] [adjustment]
/// (alias: refresh)
/// ```
///
/// tmux >=2.4:
/// ```text
/// refresh-client [-C width,height] [-S] [-t target-client]
/// (alias: refresh)
/// ```
///
/// tmux >=1.6:
/// ```text
/// refresh-client [-S] [-t target-client]
/// (alias: refresh)
/// ```
///
/// tmux >=0.8:
/// ```text
/// refresh-client [-t target-client]
/// (alias: refresh)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RefreshClient<'a> {
    /// `[-c]`
    #[cfg(feature = "tmux_2_9")]
    pub tracking_cursor: bool,

    /// `[-D]`
    #[cfg(feature = "tmux_2_9")]
    pub down: bool,

    /// `[-l]`
    #[cfg(all(feature = "tmux_2_9", not(feature = "tmux_3_3")))]
    pub request_clipboard: bool,

    /// `[-L]`
    #[cfg(feature = "tmux_2_9")]
    pub left: bool,

    /// `[-R]`
    #[cfg(feature = "tmux_2_9")]
    pub right: bool,

    /// `[-S]`
    #[cfg(all(feature = "tmux_1_6"))]
    pub status_line: bool,

    /// `[-U]`
    #[cfg(feature = "tmux_2_9")]
    pub up: bool,

    /// `[-A allow-actions]`
    #[cfg(feature = "tmux_3_2")]
    pub allow_actions: Option<AllowActions<'a>>,

    /// `[-B subscribe]`
    #[cfg(feature = "tmux_3_2")]
    pub subscribe: Option<Subscribe<'a>>,

    /// `[-C size]`
    /// `[-C X,Y]` - set the width and height of a control client
    /// `[-C XxY]` - set the width and height of a control client
    /// `[-C @id:XxY]` - set the width and height of a control client
    #[cfg(all(feature = "tmux_2_4"))]
    pub size: Option<(usize, usize)>,

    /// `[-F flags]`
    #[cfg(all(feature = "tmux_3_0a", not(feature = "tmux_3_2")))]
    pub flags: Option<ClientFlags>,

    /// `[-f flags]`
    #[cfg(feature = "tmux_3_2")]
    pub flags: Option<ClientFlags>,

    /// `[-l [target-pane]]` - request the clipboard from the client using the xterm(1) escape sequence
    #[cfg(all(feature = "tmux_3_3"))]
    pub request_clipboard: Option<Option<Cow<'a, str>>>,
    /// `[-l target-pane]`
    #[cfg(feature = "tmux_3_3")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-r osc10-11-responses]`
    #[cfg(feature = "tmux_3_5")]
    pub osc10_11_responses: Option<Cow<'a, str>>,

    /// `[-t target-client]`
    #[cfg(feature = "tmux_0_8")]
    pub target_client: Option<Cow<'a, str>>,

    /// `[adjustment]`
    #[cfg(feature = "tmux_2_9")]
    pub adjustment: Option<usize>,
}

impl<'a> RefreshClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-c]`
    #[cfg(feature = "tmux_2_9")]
    pub fn tracking_cursor(mut self) -> Self {
        self.tracking_cursor = true;
        self
    }

    /// `[-D]`
    #[cfg(feature = "tmux_2_9")]
    pub fn down(mut self) -> Self {
        self.down = true;
        self
    }

    /// `[-l]`
    #[cfg(all(feature = "tmux_2_9", not(feature = "tmux_3_3")))]
    pub fn request_clipboard(mut self) -> Self {
        self.request_clipboard = true;
        self
    }

    /// `[-L]`
    #[cfg(feature = "tmux_2_9")]
    pub fn left(mut self) -> Self {
        self.left = true;
        self
    }

    /// `[-R]`
    #[cfg(feature = "tmux_2_9")]
    pub fn right(mut self) -> Self {
        self.right = true;
        self
    }

    /// `[-S]`
    #[cfg(all(feature = "tmux_1_6"))]
    pub fn status_line(mut self) -> Self {
        self.status_line = true;
        self
    }

    /// `[-U]`
    #[cfg(feature = "tmux_2_9")]
    pub fn up(mut self) -> Self {
        self.up = true;
        self
    }

    /// `[-A allow-actions]`
    #[cfg(feature = "tmux_3_2")]
    pub fn allow_actions(mut self, allow_actions: AllowActions<'a>) -> Self {
        self.allow_actions = Some(allow_actions);
        self
    }

    /// `[-B subscribe]`
    #[cfg(feature = "tmux_3_2")]
    pub fn subscribe(mut self, subscribe: Subscribe<'a>) -> Self {
        self.subscribe = Some(subscribe);
        self
    }

    /// `[-C size]`
    /// `[-C X,Y]` - set the width and height of a control client
    /// `[-C XxY]` - set the width and height of a control client
    /// `[-C @id:XxY]` - set the width and height of a control client
    #[cfg(all(feature = "tmux_2_4"))]
    pub fn size(mut self, size: (usize, usize)) -> Self {
        self.size = Some(size);
        self
    }

    // XXX: refactor vec?
    /// `[-F flags]`
    #[cfg(all(feature = "tmux_3_0a", not(feature = "tmux_3_2")))]
    pub fn flags(mut self, flags: ClientFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    // XXX: refactor vec?
    /// `[-f flags]`
    #[cfg(feature = "tmux_3_2")]
    pub fn flags(mut self, flags: ClientFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    /// `[-l]` - request the clipboard from the client using the xterm(1) escape sequence
    #[cfg(all(feature = "tmux_3_3"))]
    pub fn request_clipboard<S: Into<Cow<'a, str>>>(mut self, target_pane: Option<S>) -> Self {
        self.request_clipboard = Some(target_pane.and_then(|t| Some(t.into())));
        self
    }
    /// `[-l target-pane]`
    #[cfg(feature = "tmux_3_3")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-r osc10-11-responses]`
    #[cfg(feature = "tmux_3_5")]
    pub fn osc10_11_responses<S: Into<Cow<'a, str>>>(mut self, osc10_11_responses: S) -> Self {
        self.osc10_11_responses = Some(osc10_11_responses.into());
        self
    }

    /// `[-t target-client]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[adjustment]`
    #[cfg(feature = "tmux_2_9")]
    pub fn adjustment(mut self, adjustment: usize) -> Self {
        self.adjustment = Some(adjustment);
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(REFRESH_CLIENT);

        // `[-c]`
        #[cfg(feature = "tmux_2_9")]
        if self.tracking_cursor {
            cmd.push_flag(C_LOWERCASE_KEY);
        }

        // `[-D]`
        #[cfg(feature = "tmux_2_9")]
        if self.down {
            cmd.push_flag(D_UPPERCASE_KEY);
        }

        // `[-l]`
        #[cfg(all(feature = "tmux_2_9", not(feature = "tmux_3_3")))]
        if self.request_clipboard {
            cmd.push_flag(L_LOWERCASE_KEY);
        }

        // `[-L]`
        #[cfg(feature = "tmux_2_9")]
        if self.left {
            cmd.push_flag(L_UPPERCASE_KEY);
        }

        // `[-R]`
        #[cfg(feature = "tmux_2_9")]
        if self.right {
            cmd.push_flag(R_UPPERCASE_KEY);
        }

        // `[-S]`
        #[cfg(all(feature = "tmux_1_6"))]
        if self.status_line {
            cmd.push_flag(S_UPPERCASE_KEY);
        }

        // `[-U]`
        #[cfg(feature = "tmux_2_9")]
        if self.up {
            cmd.push_flag(U_UPPERCASE_KEY);
        }

        // TODO: accept target_pane
        // `[-A pane:state]` - allows a control mode client to trigger actions on a pane
        // `[-A allow-actions]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(allow_actions) = self.allow_actions {
            cmd.push_option(
                A_UPPERCASE_KEY,
                format!("{}:{}", allow_actions.pane, allow_actions.state),
            );
        }

        // TODO: refactor, accept target_pane, target_window and masks * ...
        // `[-B subscribe]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(subscribe) = self.subscribe {
            let mut arg = format!("%{}", subscribe.name);
            if let Some(s) = subscribe.what {
                arg = format!("{}:{}", arg, s);
                if let Some(s) = subscribe.format {
                    arg = format!("{}:{}", arg, s);
                }
            }
            cmd.push_option(B_UPPERCASE_KEY, arg);
        }

        // `[-C size]`
        // `[-C X,Y]` - set the width and height of a control client
        // `[-C XxY]` - set the width and height of a control client
        #[cfg(feature = "tmux_2_4")]
        if let Some(size) = self.size {
            #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_0a")))]
            let s = format!("{},{}", size.0, size.1);
            // FIXME: multiple incompatible def's
            #[cfg(all(feature = "tmux_3_0a", not(feature = "tmux_3_3")))]
            let s = format!("{}x{}", size.0, size.1);
            // FIXME Size Enum + Struct wxh or @window:wxh
            #[cfg(feature = "tmux_3_3")]
            let s = format!("{}x{}", size.0, size.1);
            cmd.push_option(C_UPPERCASE_KEY, s);
        }

        // XXX: refactor vec?
        // `[-f flags]` - sets a comma-separated list of client flags
        // `[-F flags]`
        #[cfg(all(feature = "tmux_3_0a", not(feature = "tmux_3_2")))]
        if let Some(flags) = self.flags {
            cmd.push_option(F_UPPERCASE_KEY, flags.to_string());
        }

        // `[-f flags]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(flags) = self.flags {
            cmd.push_option(F_LOWERCASE_KEY, flags.to_string());
        }

        // `[-l target-pane]`
        #[cfg(feature = "tmux_3_3")]
        if let Some(request_clipboard) = self.request_clipboard {
            match request_clipboard {
                Some(target_pane) => cmd.push_option(L_LOWERCASE_KEY, target_pane),
                None => cmd.push_flag(L_LOWERCASE_KEY),
            };
        }

        // `[-r osc10-11-responses]`
        #[cfg(feature = "tmux_3_5")]
        if let Some(osc10_11_responses) = self.osc10_11_responses {
            cmd.push_option(R_LOWERCASE_KEY, osc10_11_responses);
        }

        // `[-t target-client]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_client) = self.target_client {
            cmd.push_option(T_LOWERCASE_KEY, target_client);
        }

        // `[adjustment]`
        #[cfg(feature = "tmux_2_9")]
        if let Some(adjustment) = self.adjustment {
            cmd.push_param(adjustment.to_string());
        }

        cmd
    }
}
