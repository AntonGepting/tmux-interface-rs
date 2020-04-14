use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure for refreshing the current client
///
/// # Manual
///
/// tmux 3.0a:
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
    pub tracking_cursor: Option<bool>,
    /// [-D] - move the visible part of a window down by `adjustment` rows
    pub down: Option<bool>,
    /// [-l] - request the clipboard from the client using the xterm(1) escape sequence
    pub request_clipboard: Option<bool>,
    /// [-L] - move the visible part of a window left by `adjustment` columns
    pub left: Option<bool>,
    /// [-R] - move the visible part of a window right by `adjustment` columns
    pub right: Option<bool>,
    /// [-S] - only update the client's status line
    pub status_line: Option<bool>,
    /// [-U] - move the visible part of a window up by `adjustment` rows
    pub up: Option<bool>,
    /// [-C XxY] - set the width and height of a control client
    pub size: Option<(usize, usize)>,
    /// [-F flags] - set a comma-separated list of flags
    pub flags: Option<&'a str>,
    /// [-t target-client] - specify the client
    pub target_client: Option<&'a str>,
    /// [adjustment] - moves the visible part up/down left/right by adjustment rows/columns
    pub adjustment: Option<usize>,
}

impl<'a> RefreshClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct RefreshClientBuilder<'a> {
    pub tracking_cursor: Option<bool>,
    pub down: Option<bool>,
    pub request_clipboard: Option<bool>,
    pub left: Option<bool>,
    pub right: Option<bool>,
    pub status_line: Option<bool>,
    pub up: Option<bool>,
    pub size: Option<(usize, usize)>,
    pub flags: Option<&'a str>,
    pub target_client: Option<&'a str>,
    pub adjustment: Option<usize>,
}

impl<'a> RefreshClientBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn tracking_cursor(&mut self) -> &mut Self {
        self.tracking_cursor = Some(true);
        self
    }

    pub fn down(&mut self) -> &mut Self {
        self.down = Some(true);
        self
    }

    pub fn request_clipboard(&mut self) -> &mut Self {
        self.request_clipboard = Some(true);
        self
    }

    pub fn left(&mut self) -> &mut Self {
        self.left = Some(true);
        self
    }

    pub fn right(&mut self) -> &mut Self {
        self.right = Some(true);
        self
    }

    pub fn status_line(&mut self) -> &mut Self {
        self.status_line = Some(true);
        self
    }

    pub fn up(&mut self) -> &mut Self {
        self.up = Some(true);
        self
    }

    pub fn size(&mut self, size: (usize, usize)) -> &mut Self {
        self.size = Some(size);
        self
    }

    pub fn flags(&mut self, flags: &'a str) -> &mut Self {
        self.flags = Some(flags);
        self
    }

    pub fn target_client(&mut self, target_client: &'a str) -> &mut Self {
        self.target_client = Some(target_client);
        self
    }

    pub fn adjustment(&mut self, adjustment: usize) -> &mut Self {
        self.adjustment = Some(adjustment);
        self
    }

    pub fn build(&self) -> RefreshClient<'a> {
        RefreshClient {
            tracking_cursor: self.tracking_cursor,
            down: self.down,
            request_clipboard: self.request_clipboard,
            left: self.left,
            right: self.right,
            status_line: self.status_line,
            up: self.up,
            size: self.size,
            flags: self.flags,
            target_client: self.target_client,
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
    /// tmux X.X:
    /// ```text
    /// tmux refresh-client [-cDlLRSU] [-C XxY] [-F flags] [-t target-client]
    /// [adjustment]
    /// (alias: refresh)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux refresh-client [-C width,height] [-S] [-t target-client]
    /// (alias: refresh)
    /// ```
    pub fn refresh_client(
        &mut self,
        refresh_client: Option<&RefreshClient>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let n;
        if let Some(refresh_client) = refresh_client {
            if refresh_client.tracking_cursor.unwrap_or(false) {
                args.push(c_KEY);
            }
            if refresh_client.down.unwrap_or(false) {
                args.push(D_KEY);
            }
            if refresh_client.request_clipboard.unwrap_or(false) {
                args.push(l_KEY);
            }
            if refresh_client.left.unwrap_or(false) {
                args.push(L_KEY);
            }
            if refresh_client.right.unwrap_or(false) {
                args.push(R_KEY);
            }
            if refresh_client.status_line.unwrap_or(false) {
                args.push(S_KEY);
            }
            if refresh_client.up.unwrap_or(false) {
                args.push(U_KEY);
            }
            if let Some(size) = refresh_client.size {
                s = format!("{}x{}", size.0, size.1);
                args.extend_from_slice(&[C_KEY, &s]);
            }
            if let Some(s) = refresh_client.flags {
                args.extend_from_slice(&[F_KEY, &s])
            }
            if let Some(s) = refresh_client.target_client {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(adjustment) = refresh_client.adjustment {
                n = adjustment.to_string();
                args.push(&n);
            }
        }
        let output = self.subcommand(TmuxInterface::REFRESH_CLIENT, &args)?;
        Ok(output)
    }

    //pub fn refresh_client(
    //&mut self,
    //size: Option<(usize, usize)>,
    //status_line: Option<bool>,
    //target_client: Option<&'a str>,
    //) -> Result<Output, Error> {
    //let mut args: Vec<&str> = Vec::new();
    //let s;
    //if let Some(size) = size {
    //s = format!("{},{}", size.0, size.1);
    //args.extend_from_slice(&[C_KEY, &s]);
    //}
    //if status_line.unwrap_or(false) {
    //args.push(S_KEY);
    //}
    //if let Some(s) = target_client {
    //args.extend_from_slice(&[t_KEY, &s])
    //}
    //let output = self.subcommand(TmuxInterface::REFRESH_CLIENT, &args)?;
    //Ok(output)
    //}
}
