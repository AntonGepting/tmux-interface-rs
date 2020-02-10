use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
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
pub struct SendKeys<'a, T: Display> {
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
    pub target_pane: Option<&'a T>,
    // key
    //pub key: Vec<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Clone, Debug)]
pub struct SendKeys<'a, T: Display> {
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
    pub target_pane: Option<&'a T>,
    // key
    //pub key: Vec<&'a str>,
}

impl<'a, T: Display + Default> SendKeys<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Clone, Debug)]
pub struct SendKeysBuilder<'a, T: Display> {
    pub expand_formats: Option<bool>,
    pub hex: Option<bool>,
    pub disable_lookup: Option<bool>,
    pub mouse_event: Option<bool>,
    pub copy_mode: Option<bool>,
    pub reset: Option<bool>,
    pub repeat_count: Option<usize>,
    pub target_pane: Option<&'a T>,
    //pub key: Vec<&'a str>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a, T: Display + Default> SendKeysBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn expand_formats(&mut self) -> &mut Self {
        self.expand_formats = Some(true);
        self
    }

    pub fn hex(&mut self) -> &mut Self {
        self.hex = Some(true);
        self
    }

    pub fn disable_lookup(&mut self) -> &mut Self {
        self.disable_lookup = Some(true);
        self
    }

    pub fn mouse_event(&mut self) -> &mut Self {
        self.mouse_event = Some(true);
        self
    }

    pub fn copy_mode(&mut self) -> &mut Self {
        self.copy_mode = Some(true);
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.reset = Some(true);
        self
    }

    pub fn repeat_count(&mut self, repeat_count: usize) -> &mut Self {
        self.repeat_count = Some(repeat_count);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn build(&self) -> SendKeys<'a, T> {
        SendKeys {
            expand_formats: self.expand_formats,
            hex: self.hex,
            disable_lookup: self.disable_lookup,
            mouse_event: self.mouse_event,
            copy_mode: self.copy_mode,
            reset: self.reset,
            repeat_count: self.repeat_count,
            target_pane: self.target_pane,
            // key: Vec<&'a str>,
        }
    }
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Clone, Debug)]
pub struct SendKeysBuilder<'a, T: Display> {
    pub disable_lookup: Option<bool>,
    pub mouse_event: Option<bool>,
    pub copy_mode: Option<bool>,
    pub reset: Option<bool>,
    pub repeat_count: Option<usize>,
    pub target_pane: Option<&'a T>,
    //pub key: Vec<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
impl<'a, T: Display + Default> SendKeysBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn disable_lookup(&mut self) -> &mut Self {
        self.disable_lookup = Some(true);
        self
    }

    pub fn mouse_event(&mut self) -> &mut Self {
        self.mouse_event = Some(true);
        self
    }

    pub fn copy_mode(&mut self) -> &mut Self {
        self.copy_mode = Some(true);
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.reset = Some(true);
        self
    }

    pub fn repeat_count(&mut self, repeat_count: usize) -> &mut Self {
        self.repeat_count = Some(repeat_count);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn build(&self) -> SendKeys<'a, T> {
        SendKeys {
            disable_lookup: self.disable_lookup,
            mouse_event: self.mouse_event,
            copy_mode: self.copy_mode,
            reset: self.reset,
            repeat_count: self.repeat_count,
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
    pub fn send_keys<T: Display>(
        &mut self,
        send_keys: Option<&SendKeys<T>>,
        key: &Vec<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let n;
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
                n = repeat_count.to_string();
                args.extend_from_slice(&[N_KEY, &n]);
            }
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
    pub fn send_keys<T: Display>(
        &mut self,
        send_keys: Option<&SendKeys<T>>,
        key: &Vec<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let n;
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
                n = repeat_count.to_string();
                args.extend_from_slice(&[N_KEY, &n]);
            }
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
