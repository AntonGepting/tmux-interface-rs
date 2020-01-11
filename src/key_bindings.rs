use super::error::Error;
use super::tmux_interface::*;
use std::process::Output;

/// Structure binding key `key` to command
///
/// # Manual
///
/// ```text
/// tmux bind-key [-nr] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
#[derive(Default, Clone, Debug)]
pub struct BindKey<'a> {
    /// [-n] - an alias for -T root
    pub root: Option<bool>,
    /// [-r] - this key may repeat
    pub repeat: Option<bool>,
    /// [-T key-table] - key-table
    pub key_table: Option<&'a str>,
    // key -
    //pub key: &'a str,
    // command
    //pub command: &'a str,
    /// [arguments] - arguments
    pub arguments: Option<&'a str>,
}

impl<'a> BindKey<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Structure
///
/// # Manual
///
/// ```text
/// tmux send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
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

impl<'a> SendKeys<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// All functions from man tmux "Key Bindings" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#KEY_BINDINGS)
impl<'a> TmuxInterface<'a> {
    const BIND_KEY: &'static str = "bind-key";
    const LIST_KEYS: &'static str = "list-keys";
    const SEND_KEYS: &'static str = "send-keys";
    const SEND_PREFIX: &'static str = "send-prefix";
    const UNBIND_KEY: &'static str = "unbind-key";

    /// # Manual
    ///
    /// ```text
    /// tmux bind-key [-nr] [-T key-table] key command [arguments]
    /// (alias: bind)
    /// ```
    pub fn bind_key(
        &mut self,
        bind_key: Option<&BindKey>,
        key: &str,
        command: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(bind_key) = bind_key {
            if bind_key.root.unwrap_or(false) {
                args.push(n_KEY);
            }
            if bind_key.repeat.unwrap_or(false) {
                args.push(r_KEY);
            }
            if let Some(s) = bind_key.key_table {
                args.extend_from_slice(&[T_KEY, &s])
            }
        }
        args.push(key);
        args.push(command);
        if let Some(bind_key) = bind_key {
            if let Some(s) = bind_key.arguments {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::BIND_KEY, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// ```text
    /// tmux list-keys [-T key-table]
    /// (alias: lsk)
    /// ```
    pub fn list_keys(&mut self, key_table: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = key_table {
            args.extend_from_slice(&[T_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LIST_KEYS, &args)?;
        Ok(output)
    }

    // FIXME: repeat-count
    /// # Manual
    ///
    /// ```text
    /// tmux send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
    /// (alias: send)
    /// ```
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

    /// # Manual
    ///
    /// ```text
    /// tmux send-prefix [-2] [-t target-pane]
    /// ```
    pub fn send_prefix(
        &mut self,
        secondary: Option<bool>,
        target_pane: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if secondary.unwrap_or(false) {
            args.push(_2_KEY);
        }
        if let Some(s) = target_pane {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::SEND_PREFIX, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// ```text
    /// tmux unbind-key [-an] [-T key-table] key
    /// (alias: unbind)
    /// ```
    pub fn unbind_key(
        &mut self,
        all: Option<bool>,
        root: Option<bool>,
        key_table: Option<&str>,
        key: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if root.unwrap_or(false) {
            args.push(n_KEY);
        }
        if let Some(s) = key_table {
            args.extend_from_slice(&[T_KEY, &s])
        }
        args.push(key);
        let output = self.subcommand(TmuxInterface::UNBIND_KEY, &args)?;
        Ok(output)
    }
}
