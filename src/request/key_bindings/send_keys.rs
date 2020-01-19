use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure
///
/// # Manual
///
/// tmux X.X:
/// ```text
/// tmux send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux 2.6:
/// ```text
/// tmux send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Clone, Debug)]
pub struct SendKeys<'a> {
    /// [-F] - expand formats in arguments where appropriate
    pub expand_formats: Option<bool>,
    /// [-H] - expect each key to be a hexadecimal number for an ASCII character
    pub hex: Option<bool>,
    /// [-l] - disable key name lookup and processes the keys as literal UTF-8 characters
    pub disable_lookup: Option<bool>,
    /// [-M] - pass through a mouse event
    pub mouse_event: Option<bool>,
    /// [-R] - cause the terminal state to be reset
    pub copy_mode: Option<bool>,
    /// [-X] - send a command into copy mode
    pub reset: Option<bool>,
    /// [-N repeat-count] - specify a repeat count
    pub repeat_count: Option<usize>,
    /// [-t target-pane] - specify the target pane
    pub target_pane: Option<&'a str>,
    // key
    //pub key: Vec<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Clone, Debug)]
pub struct SendKeys<'a> {
    /// [-l] - disable key name lookup and processes the keys as literal UTF-8 characters
    pub disable_lookup: Option<bool>,
    /// [-M] - pass through a mouse event
    pub mouse_event: Option<bool>,
    /// [-R] - cause the terminal state to be reset
    pub copy_mode: Option<bool>,
    /// [-X] - send a command into copy mode
    pub reset: Option<bool>,
    /// [-N repeat-count] - specify a repeat count
    pub repeat_count: Option<usize>,
    /// [-t target-pane] - specify the target pane
    pub target_pane: Option<&'a str>,
    // key
    //pub key: Vec<&'a str>,
}

impl<'a> SendKeys<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a> TmuxInterface<'a> {
    const SEND_KEYS: &'static str = "send-keys";

    // FIXME: repeat-count
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
    /// (alias: send)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
    /// (alias: send)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn send_keys(
        &mut self,
        send_keys: Option<&SendKeys>,
        key: &Vec<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(send_keys) = send_keys {
            if send_keys.expand_formats.unwrap_or(false) {
                args.push(F_KEY);
            }
            if send_keys.hex.unwrap_or(false) {
                args.push(H_KEY);
            }
            if send_keys.disable_lookup.unwrap_or(false) {
                args.push(l_KEY);
            }
            if send_keys.mouse_event.unwrap_or(false) {
                args.push(M_KEY);
            }
            if send_keys.copy_mode.unwrap_or(false) {
                args.push(R_KEY);
            }
            if send_keys.reset.unwrap_or(false) {
                args.push(X_KEY);
            }
            //send_keys.repeat_count.map(|s| Some(args.extend_from_slice(&[N_KEY, s])));
            if let Some(repeat_count) = send_keys.repeat_count {
                s = repeat_count.to_string();
                args.extend_from_slice(&[N_KEY, &s]);
            }
            if let Some(s) = send_keys.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        //args.extend_from_slice(send_keys.keys.as_slice());
        //args.extend_from_slice(send_keys.keys);
        args.append(&mut key.clone());
        //args.push("C-m");
        let output = self.subcommand(TmuxInterface::SEND_KEYS, &args)?;
        Ok(output)
    }

    // FIXME: repeat-count
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
    /// (alias: send)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
    /// (alias: send)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn send_keys(
        &mut self,
        send_keys: Option<&SendKeys>,
        key: &Vec<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(send_keys) = send_keys {
            if send_keys.disable_lookup.unwrap_or(false) {
                args.push(l_KEY);
            }
            if send_keys.mouse_event.unwrap_or(false) {
                args.push(M_KEY);
            }
            if send_keys.copy_mode.unwrap_or(false) {
                args.push(R_KEY);
            }
            if send_keys.reset.unwrap_or(false) {
                args.push(X_KEY);
            }
            //send_keys.repeat_count.map(|s| Some(args.extend_from_slice(&[N_KEY, s])));
            if let Some(repeat_count) = send_keys.repeat_count {
                s = repeat_count.to_string();
                args.extend_from_slice(&[N_KEY, &s]);
            }
            if let Some(s) = send_keys.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        //args.extend_from_slice(send_keys.keys.as_slice());
        //args.extend_from_slice(send_keys.keys);
        args.append(&mut key.clone());
        //args.push("C-m");
        let output = self.subcommand(TmuxInterface::SEND_KEYS, &args)?;
        Ok(output)
    }
}
