use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure for refreshing the current client
///
/// # Manual
///
/// ```text
/// tmux refresh-client [-cDlLRSU] [-C XxY] [-F flags] [-t target-client]
/// [adjustment]
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

/// All functions from man tmux "Clients and Sessions" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS)
impl<'a> TmuxInterface<'a> {
    const REFRESH_CLIENT: &'static str = "refresh-client";
    /// Refresh the current client
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux refresh-client [-cDlLRSU] [-C XxY] [-F flags] [-t target-client]
    /// [adjustment]
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
}
