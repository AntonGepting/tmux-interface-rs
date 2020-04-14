use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure binding key `key` to command
///
/// # Manual
///
/// tmux 3.1:
/// ```text
/// tmux bind-key [-nr] [-N note] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^2.4:
/// ```text
/// tmux bind-key [-nr] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^2.3:
/// ```text
/// tmux bind-key [-cnr] [-R repeat-count] [-t mode-table] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux bind-key [-cnr] [-t mode-table] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^2.0:
/// ```text
/// tmux bind-key [-cnr] [-t mode-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux bind-key [-cnr] [-t key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux bind-key [-r] key command [arguments]
/// (alias: bind)
/// ```
#[derive(Default, Clone, Debug)]
pub struct BindKey<'a> {
    /// [-n] - an alias for -T root
    pub root: Option<bool>,
    /// [-r] - this key may repeat
    pub repeat: Option<bool>,
    #[cfg(feature = "tmux_X_X")]
    /// [-N note] - attaches note to the key
    pub note: Option<&'a str>,
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

#[derive(Default, Clone, Debug)]
pub struct BindKeyBuilder<'a> {
    pub root: Option<bool>,
    pub repeat: Option<bool>,
    #[cfg(feature = "tmux_X_X")]
    pub note: Option<&'a str>,
    pub key_table: Option<&'a str>,
    //pub key: &'a str,
    //pub command: &'a str,
    pub arguments: Option<&'a str>,
}

impl<'a> BindKeyBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn root(&mut self) -> &mut Self {
        self.root = Some(true);
        self
    }

    pub fn repeat(&mut self) -> &mut Self {
        self.repeat = Some(true);
        self
    }

    #[cfg(feature = "tmux_X_X")]
    pub fn note(&mut self, note: &'a str) -> &mut Self {
        self.note = Some(note);
        self
    }

    pub fn key_table(&mut self, key_table: &'a str) -> &mut Self {
        self.key_table = Some(key_table);
        self
    }

    pub fn arguments(&mut self, arguments: &'a str) -> &mut Self {
        self.arguments = Some(arguments);
        self
    }

    pub fn build(&self) -> BindKey<'a> {
        BindKey {
            root: self.root,
            repeat: self.repeat,
            #[cfg(feature = "tmux_X_X")]
            note: self.note,
            key_table: self.key_table,
            arguments: self.arguments,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const BIND_KEY: &'static str = "bind-key";

    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux bind-key [-nr] [-N note] [-T key-table] key command [arguments]
    /// (alias: bind)
    /// ```
    ///
    /// tmux 2.6:
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
            #[cfg(feature = "tmux_X_X")]
            {
                if let Some(s) = bind_key.note {
                    args.extend_from_slice(&[N_KEY, &s])
                }
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
}
