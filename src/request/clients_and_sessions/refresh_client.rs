use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

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
#[derive(Default, Debug)]
pub struct RefreshClient<'a> {
    /// [-c] - return to tracking the cursor automatically
    #[cfg(feature = "tmux_2_9a")]
    pub tracking_cursor: Option<bool>,
    /// [-D] - move the visible part of a window down by `adjustment` rows
    #[cfg(feature = "tmux_2_9a")]
    pub down: Option<bool>,
    /// [-l] - request the clipboard from the client using the xterm(1) escape sequence
    #[cfg(feature = "tmux_2_9a")]
    pub request_clipboard: Option<bool>,
    /// [-L] - move the visible part of a window left by `adjustment` columns
    #[cfg(feature = "tmux_2_9a")]
    pub left: Option<bool>,
    /// [-R] - move the visible part of a window right by `adjustment` columns
    #[cfg(feature = "tmux_2_9a")]
    pub right: Option<bool>,
    /// [-S] - only update the client's status line
    #[cfg(feature = "tmux_1_6")]
    pub status_line: Option<bool>,
    /// [-U] - move the visible part of a window up by `adjustment` rows
    #[cfg(feature = "tmux_2_9a")]
    pub up: Option<bool>,
    /// [-C XxY] - set the width and height of a control client
    #[cfg(feature = "tmux_2_4")]
    pub size: Option<(usize, usize)>,
    /// [-F flags] - set a comma-separated list of flags
    #[cfg(feature = "tmux_2_9a")]
    pub flags: Option<&'a str>,
    /// [-t target-client] - specify the client
    #[cfg(feature = "tmux_0_8")]
    pub target_client: Option<&'a str>,
    /// [adjustment] - moves the visible part up/down left/right by adjustment rows/columns
    #[cfg(feature = "tmux_2_9a")]
    pub adjustment: Option<usize>,
}

impl<'a> RefreshClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct RefreshClientBuilder<'a> {
    #[cfg(feature = "tmux_2_9a")]
    pub tracking_cursor: Option<bool>,
    #[cfg(feature = "tmux_2_9a")]
    pub down: Option<bool>,
    #[cfg(feature = "tmux_2_9a")]
    pub request_clipboard: Option<bool>,
    #[cfg(feature = "tmux_2_9a")]
    pub left: Option<bool>,
    #[cfg(feature = "tmux_2_9a")]
    pub right: Option<bool>,
    #[cfg(feature = "tmux_1_6")]
    pub status_line: Option<bool>,
    #[cfg(feature = "tmux_2_9a")]
    pub up: Option<bool>,
    #[cfg(feature = "tmux_2_4")]
    pub size: Option<(usize, usize)>,
    #[cfg(feature = "tmux_2_9a")]
    pub flags: Option<&'a str>,
    #[cfg(feature = "tmux_0_8")]
    pub target_client: Option<&'a str>,
    #[cfg(feature = "tmux_2_9a")]
    pub adjustment: Option<usize>,
}

impl<'a> RefreshClientBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_9a")]
    pub fn tracking_cursor(&mut self) -> &mut Self {
        self.tracking_cursor = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_9a")]
    pub fn down(&mut self) -> &mut Self {
        self.down = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_9a")]
    pub fn request_clipboard(&mut self) -> &mut Self {
        self.request_clipboard = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_9a")]
    pub fn left(&mut self) -> &mut Self {
        self.left = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_9a")]
    pub fn right(&mut self) -> &mut Self {
        self.right = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn status_line(&mut self) -> &mut Self {
        self.status_line = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_9a")]
    pub fn up(&mut self) -> &mut Self {
        self.up = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn size(&mut self, size: (usize, usize)) -> &mut Self {
        self.size = Some(size);
        self
    }

    #[cfg(feature = "tmux_2_9a")]
    pub fn flags(&mut self, flags: &'a str) -> &mut Self {
        self.flags = Some(flags);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn target_client(&mut self, target_client: &'a str) -> &mut Self {
        self.target_client = Some(target_client);
        self
    }

    #[cfg(feature = "tmux_2_9a")]
    pub fn adjustment(&mut self, adjustment: usize) -> &mut Self {
        self.adjustment = Some(adjustment);
        self
    }

    pub fn build(&self) -> RefreshClient<'a> {
        RefreshClient {
            #[cfg(feature = "tmux_2_9a")]
            tracking_cursor: self.tracking_cursor,
            #[cfg(feature = "tmux_2_9a")]
            down: self.down,
            #[cfg(feature = "tmux_2_9a")]
            request_clipboard: self.request_clipboard,
            #[cfg(feature = "tmux_2_9a")]
            left: self.left,
            #[cfg(feature = "tmux_2_9a")]
            right: self.right,
            #[cfg(feature = "tmux_1_6")]
            status_line: self.status_line,
            #[cfg(feature = "tmux_2_9a")]
            up: self.up,
            #[cfg(feature = "tmux_2_4")]
            size: self.size,
            #[cfg(feature = "tmux_2_9a")]
            flags: self.flags,
            #[cfg(feature = "tmux_0_8")]
            target_client: self.target_client,
            #[cfg(feature = "tmux_2_9a")]
            adjustment: self.adjustment,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const REFRESH_CLIENT: &'static str = "refresh-client";
    /// Refresh the current client
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
    pub fn refresh_client(
        &mut self,
        refresh_client: Option<&RefreshClient>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        #[cfg(feature = "tmux_2_9a")]
        let n: String;
        if let Some(refresh_client) = refresh_client {
            #[cfg(feature = "tmux_2_9a")]
            {
                if refresh_client.tracking_cursor.unwrap_or(false) {
                    args.push(c_KEY);
                }
            }
            #[cfg(feature = "tmux_2_9a")]
            {
                if refresh_client.down.unwrap_or(false) {
                    args.push(D_KEY);
                }
            }
            #[cfg(feature = "tmux_2_9a")]
            {
                if refresh_client.request_clipboard.unwrap_or(false) {
                    args.push(l_KEY);
                }
            }
            #[cfg(feature = "tmux_2_9a")]
            {
                if refresh_client.left.unwrap_or(false) {
                    args.push(L_KEY);
                }
            }
            #[cfg(feature = "tmux_2_9a")]
            {
                if refresh_client.right.unwrap_or(false) {
                    args.push(R_KEY);
                }
            }
            #[cfg(feature = "tmux_1_6")]
            {
                if refresh_client.status_line.unwrap_or(false) {
                    args.push(S_KEY);
                }
            }
            #[cfg(feature = "tmux_2_9a")]
            {
                if refresh_client.up.unwrap_or(false) {
                    args.push(U_KEY);
                }
            }
            #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_0")))]
            {
                if let Some(size) = refresh_client.size {
                    s = format!("{},{}", size.0, size.1);
                    args.extend_from_slice(&[C_KEY, &s]);
                }
            }
            // FIXME: multiple incompatible def's
            #[cfg(feature = "tmux_3_0")]
            {
                if let Some(size) = refresh_client.size {
                    s = format!("{}x{}", size.0, size.1);
                    args.extend_from_slice(&[C_KEY, &s]);
                }
            }
            #[cfg(feature = "tmux_2_9a")]
            {
                if let Some(s) = refresh_client.flags {
                    args.extend_from_slice(&[F_KEY, &s])
                }
            }
            #[cfg(feature = "tmux_0_8")]
            {
                if let Some(s) = refresh_client.target_client {
                    args.extend_from_slice(&[t_KEY, &s])
                }
            }
            #[cfg(feature = "tmux_2_9a")]
            {
                if let Some(adjustment) = refresh_client.adjustment {
                    n = adjustment.to_string();
                    args.push(&n);
                }
            }
        }
        let output = self.subcommand(TmuxInterface::REFRESH_CLIENT, &args)?;
        Ok(output)
    }
}
