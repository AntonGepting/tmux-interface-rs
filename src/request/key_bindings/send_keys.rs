use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Structure
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^3.0:
/// ```text
/// tmux send-keys [-HlMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^2.4:
/// ```text
/// tmux send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux send-keys [-lMR] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux send-keys [-lR] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^1.6:
/// ```text
/// tmux send-keys [-R] [-t target-pane] key ...
/// (alias: send)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux send-keys [-t target-window] key ...
/// (alias: send)
/// ```
#[derive(Default, Clone, Debug)]
pub struct SendKeys<'a, T: Display> {
    /// [-F] - expand formats in arguments where appropriate
    #[cfg(feature = "tmux_3_1")]
    pub expand_formats: Option<bool>,
    /// [-H] - expect each key to be a hexadecimal number for an ASCII character
    #[cfg(feature = "tmux_3_0")]
    pub hex: Option<bool>,
    /// [-l] - disable key name lookup and processes the keys as literal UTF-8 characters
    #[cfg(feature = "tmux_1_7")]
    pub disable_lookup: Option<bool>,
    /// [-M] - pass through a mouse event
    #[cfg(feature = "tmux_2_1")]
    pub mouse_event: Option<bool>,
    /// [-R] - cause the terminal state to be reset
    #[cfg(feature = "tmux_1_6")]
    pub copy_mode: Option<bool>,
    /// [-X] - send a command into copy mode
    #[cfg(feature = "tmux_2_4")]
    pub reset: Option<bool>,
    /// [-N repeat-count] - specify a repeat count
    #[cfg(feature = "tmux_2_4")]
    pub repeat_count: Option<usize>,
    /// [-t target-pane] - specify the target pane
    #[cfg(feature = "tmux_1_6")]
    pub target_pane: Option<&'a T>,
    // key
    //pub key: Vec<&'a str>,
}

impl<'a, T: Display + Default> SendKeys<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Clone, Debug)]
pub struct SendKeysBuilder<'a, T: Display> {
    #[cfg(feature = "tmux_3_1")]
    pub expand_formats: Option<bool>,
    #[cfg(feature = "tmux_3_0")]
    pub hex: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub disable_lookup: Option<bool>,
    #[cfg(feature = "tmux_2_1")]
    pub mouse_event: Option<bool>,
    #[cfg(feature = "tmux_1_6")]
    pub copy_mode: Option<bool>,
    #[cfg(feature = "tmux_2_4")]
    pub reset: Option<bool>,
    #[cfg(feature = "tmux_2_4")]
    pub repeat_count: Option<usize>,
    #[cfg(feature = "tmux_1_6")]
    pub target_pane: Option<&'a T>,
    //pub key: Vec<&'a str>,
}

impl<'a, T: Display + Default> SendKeysBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_3_1")]
    pub fn expand_formats(&mut self) -> &mut Self {
        self.expand_formats = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn hex(&mut self) -> &mut Self {
        self.hex = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn disable_lookup(&mut self) -> &mut Self {
        self.disable_lookup = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn mouse_event(&mut self) -> &mut Self {
        self.mouse_event = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn copy_mode(&mut self) -> &mut Self {
        self.copy_mode = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn reset(&mut self) -> &mut Self {
        self.reset = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn repeat_count(&mut self, repeat_count: usize) -> &mut Self {
        self.repeat_count = Some(repeat_count);
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn build(&self) -> SendKeys<'a, T> {
        SendKeys {
            #[cfg(feature = "tmux_3_1")]
            expand_formats: self.expand_formats,
            #[cfg(feature = "tmux_3_0")]
            hex: self.hex,
            #[cfg(feature = "tmux_1_7")]
            disable_lookup: self.disable_lookup,
            #[cfg(feature = "tmux_2_1")]
            mouse_event: self.mouse_event,
            #[cfg(feature = "tmux_1_6")]
            copy_mode: self.copy_mode,
            #[cfg(feature = "tmux_2_4")]
            reset: self.reset,
            #[cfg(feature = "tmux_2_4")]
            repeat_count: self.repeat_count,
            #[cfg(feature = "tmux_1_6")]
            target_pane: self.target_pane,
            // key: Vec<&'a str>,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const SEND_KEYS: &'static str = "send-keys";

    // FIXME: repeat-count
    /// # Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// tmux send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
    /// (alias: send)
    /// ```
    ///
    /// tmux ^3.0:
    /// ```text
    /// tmux send-keys [-HlMRX] [-N repeat-count] [-t target-pane] key ...
    /// (alias: send)
    /// ```
    ///
    /// tmux ^2.4:
    /// ```text
    /// tmux send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
    /// (alias: send)
    /// ```
    ///
    /// tmux ^2.1:
    /// ```text
    /// tmux send-keys [-lMR] [-t target-pane] key ...
    /// (alias: send)
    /// ```
    ///
    /// tmux ^1.7:
    /// ```text
    /// tmux send-keys [-lR] [-t target-pane] key ...
    /// (alias: send)
    /// ```
    ///
    /// tmux ^1.6:
    /// ```text
    /// tmux send-keys [-R] [-t target-pane] key ...
    /// (alias: send)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux send-keys [-t target-window] key ...
    /// (alias: send)
    /// ```
    pub fn send_keys<T: Display>(
        &mut self,
        send_keys: Option<&SendKeys<T>>,
        key: &Vec<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let n;
        if let Some(send_keys) = send_keys {
            #[cfg(feature = "tmux_3_1")]
            if send_keys.expand_formats.unwrap_or(false) {
                args.push(F_KEY);
            }
            #[cfg(feature = "tmux_3_0")]
            if send_keys.hex.unwrap_or(false) {
                args.push(H_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if send_keys.disable_lookup.unwrap_or(false) {
                args.push(l_KEY);
            }
            #[cfg(feature = "tmux_2_1")]
            if send_keys.mouse_event.unwrap_or(false) {
                args.push(M_KEY);
            }
            #[cfg(feature = "tmux_1_6")]
            if send_keys.copy_mode.unwrap_or(false) {
                args.push(R_KEY);
            }
            #[cfg(feature = "tmux_2_4")]
            if send_keys.reset.unwrap_or(false) {
                args.push(X_KEY);
            }
            //send_keys.repeat_count.map(|s| Some(args.extend_from_slice(&[N_KEY, s])));
            #[cfg(feature = "tmux_2_4")]
            if let Some(repeat_count) = send_keys.repeat_count {
                n = repeat_count.to_string();
                args.extend_from_slice(&[N_KEY, &n]);
            }
            #[cfg(feature = "tmux_1_6")]
            if let Some(target_pane) = send_keys.target_pane {
                s = target_pane.to_string();
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
