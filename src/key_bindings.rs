use super::tmux_interface::*;
use super::tmux_interface_error::TmuxInterfaceError;
use std::process::Output;


/// # Manual
///
/// ```text
/// tmux bind-key [-nr] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
#[derive(Default, Clone, Debug)]
pub struct BindKey<'a> {
    pub root: Option<bool>,                     // [-n]
    pub repeat: Option<bool>,                   // [-r]
    pub key_table: Option<&'a str>,             // [-T key-table]
    pub key: &'a str,                           // key
    pub command: &'a str,                       // command
    pub arguments: Option<&'a str>              // [arguments]
}

impl<'a> BindKey<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}


/// # Manual
///
/// ```text
/// tmux send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
/// (alias: send)
/// ```
#[derive(Default, Clone, Debug)]
pub struct SendKeys<'a> {
    pub disable_lookup: Option<bool>,           // [-l]
    pub mouse_event: Option<bool>,              // [-M]
    pub copy_mode: Option<bool>,                // [-R]
    pub reset: Option<bool>,                    // [-X]
    pub repeat_count: Option<usize>,            // [-N repeat-count]
    pub target_pane: Option<&'a str>,           // [-t target-pane]
    pub key: Vec<&'a str>                       // key
}

impl<'a> SendKeys<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}


/// Key bindings
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
    pub fn bind_key(&self, bind_key: &BindKey) -> Result<Output, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if bind_key.root.unwrap_or(false) { args.push(n_KEY); }
        if bind_key.repeat.unwrap_or(false) { args.push(r_KEY); }
        bind_key.key_table.and_then(|s| Some(args.extend_from_slice(&[T_KEY, &s])));
        args.push(bind_key.key);
        args.push(bind_key.command);
        bind_key.arguments.and_then(|s| Some(args.push(&s)));
        let output = self.subcommand(TmuxInterface::BIND_KEY, &args)?;
        Ok(output)
    }


    /// # Manual
    ///
    /// ```text
    /// tmux list-keys [-T key-table]
    /// (alias: lsk)
    /// ```
    pub fn list_keys(&self, key_table: Option<&str>) -> Result<Output, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        key_table.and_then(|s| Some(args.extend_from_slice(&[T_KEY, &s])));
        let output = self.subcommand(TmuxInterface::LIST_KEYS, &args)?;
        Ok(output)
    }


    // FIXME: repeat-count
    /// # Manual
    ///
    /// ```text
    /// tmux send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
    /// (alias: send)
    /// ```
    pub fn send_keys(&self, send_keys: &SendKeys) -> Result<Output, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if send_keys.disable_lookup.unwrap_or(false) { args.push(l_KEY); }
        if send_keys.mouse_event.unwrap_or(false) { args.push(M_KEY); }
        if send_keys.copy_mode.unwrap_or(false) { args.push(R_KEY); }
        if send_keys.reset.unwrap_or(false) { args.push(X_KEY); }
        //send_keys.repeat_count.and_then(|s| Some(args.extend_from_slice(&[N_KEY, s])));
        let s;
        if let Some(repeat_count) = send_keys.repeat_count {
            s = repeat_count.to_string();
            args.extend_from_slice(&[N_KEY, &s]);
        }
        send_keys.target_pane.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        //args.extend_from_slice(send_keys.keys.as_slice());
        //args.extend_from_slice(send_keys.keys);
        args.append(&mut send_keys.key.clone());
        //args.push("C-m");
        let output = self.subcommand(TmuxInterface::SEND_KEYS, &args)?;
        Ok(output)
    }


    /// # Manual
    ///
    /// ```text
    /// tmux send-prefix [-2] [-t target-pane]
    /// ```
    pub fn send_prefix(&self,
                       secondary: Option<bool>,
                       target_pane: Option<&str>
                       ) -> Result<Output, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if secondary.unwrap_or(false) { args.push(_2_KEY); }
        target_pane.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::SEND_PREFIX, &args)?;
        Ok(output)
    }


    /// # Manual
    ///
    /// ```text
    /// tmux unbind-key [-an] [-T key-table] key
    /// (alias: unbind)
    /// ```
    pub fn unbind_key(&self,
                      all: Option<bool>,
                      root: Option<bool>,
                      key_table: Option<&str>,
                      key: &str
                      ) -> Result<Output, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) { args.push(a_KEY); }
        if root.unwrap_or(false) { args.push(n_KEY); }
        key_table.and_then(|s| Some(args.extend_from_slice(&[T_KEY, &s])));
        args.push(key);
        let output = self.subcommand(TmuxInterface::UNBIND_KEY, &args)?;
        Ok(output)
    }


}
