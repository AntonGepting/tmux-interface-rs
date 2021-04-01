use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Structure for refreshing the current client
///
/// # Manual
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

    /// `[-F flags]` - set a comma-separated list of flags
    #[cfg(feature = "tmux_2_9a")]
    pub fn flags(&mut self, flags: &'a str) -> &mut Self {
        self.0.push_option(F_UPPERCASE_KEY, flags);
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
