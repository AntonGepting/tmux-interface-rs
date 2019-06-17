use super::tmux_interface::*;
use super::tmux_interface_error::TmuxInterfaceError;


#[derive(Default, Clone, Debug)]
pub struct SendKeys<'a> {
    pub disable_lookup: Option<bool>,           // [-l]
    pub mouse_event: Option<bool>,              // [-M]
    pub copy_mode: Option<bool>,                // [-R]
    pub reset: Option<bool>,                    // [-X]
    pub repeat_count: Option<usize>,            // [-N repeat-count]
    pub target_pane: Option<&'a str>,           // [-t target-pane]
    pub keys: Vec<&'a str>                      // key
}


impl<'a> SendKeys<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}


/// Key bindings
impl<'a> TmuxInterface<'a> {

    const SEND_KEYS: &'static str = "send-keys";

    /// # Manual
    ///
    /// ```text
    /// tmux bind-key [-nr] [-T key-table] key command [arguments]
    /// (alias: bind)
    /// ```
    pub fn bind_key() {
        unimplemented!();
    }


    /// # Manual
    ///
    /// ```text
    /// tmux list-keys [-T key-table]
    /// (alias: lsk)
    /// ```
    pub fn list_keys() {
        unimplemented!();
    }


    // FIXME: repeat-count
    /// # Manual
    ///
    /// ```text
    /// tmux send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
    /// (alias: send)
    /// ```
    pub fn send_keys(&self, send_keys: &SendKeys) -> Result<bool, TmuxInterfaceError> {
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
        send_keys.target_pane.as_ref().and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        //args.extend_from_slice(send_keys.keys.as_slice());
        //args.extend_from_slice(send_keys.keys);
        args.append(&mut send_keys.keys.clone());
        //args.push("C-m");
        let output = self.subcommand(TmuxInterface::SEND_KEYS, &args)?;
        Ok(output.status.success())
    }


    /// # Manual
    ///
    /// ```text
    /// tmux send-prefix [-2] [-t target-pane]
    /// ```
    pub fn send_prefix() {
        unimplemented!();
    }


    /// # Manual
    ///
    /// ```text
    /// tmux unbind-key [-an] [-T key-table] key
    /// (alias: unbind)
    /// ```
    pub fn unbind_key() {
        unimplemented!();
    }


}
