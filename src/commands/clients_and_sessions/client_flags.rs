// TODO: parse from_str

#[cfg(feature = "tmux_2_9a")]
use std::fmt;

#[cfg(feature = "tmux_2_9a")]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ClientFlags {
    /// `active-pane` the client has an independent active pane
    pub active_pane: Option<bool>,
    /// `ignore-size` the client does not affect the size of other clients
    pub ignore_size: Option<bool>,
    /// `no-output` the client does not receive pane output in control mode
    pub no_output: Option<bool>,
    /// `pause-after=seconds` output is paused once the pane is seconds behind in control mode
    pub pause_after: Option<usize>,
    /// `read-only` the client is read-only
    pub read_only: Option<bool>,
    /// `wait-exit` wait for an empty line input before exiting in control mode
    pub wait_exit: Option<bool>,
}

#[cfg(feature = "tmux_2_9a")]
impl ClientFlags {
    fn bool_to_flag<S: AsRef<str>>(value: bool, flag_name: S) -> String {
        if value {
            format!("{}", flag_name.as_ref())
        } else {
            format!("!{}", flag_name.as_ref())
        }
    }
}

const CLIENT_FLAG_ACTIVE_PANE: &str = "active-pane";
const CLIENT_FLAG_IGNORE_SIZE: &str = "ignore-size";
const CLIENT_FLAG_NO_OUTPUT: &str = "no-output";
const CLIENT_FLAG_PAUSE_AFTER: &str = "pause-after";
const CLIENT_FLAG_READ_ONLY: &str = "read-only";
const CLIENT_FLAG_WAIT_EXIT: &str = "wait-exit";

#[cfg(feature = "tmux_2_9a")]
impl fmt::Display for ClientFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();
        if let Some(active_pane) = self.active_pane {
            v.push(Self::bool_to_flag(active_pane, CLIENT_FLAG_ACTIVE_PANE));
        }
        if let Some(ignore_size) = self.ignore_size {
            v.push(Self::bool_to_flag(ignore_size, CLIENT_FLAG_IGNORE_SIZE));
        }
        if let Some(no_output) = self.no_output {
            v.push(Self::bool_to_flag(no_output, CLIENT_FLAG_NO_OUTPUT));
        }
        if let Some(pause_after) = self.pause_after {
            v.push(format!(
                "{}={}",
                Self::bool_to_flag(true, CLIENT_FLAG_PAUSE_AFTER),
                pause_after
            ));
        }
        if let Some(read_only) = self.read_only {
            v.push(Self::bool_to_flag(read_only, CLIENT_FLAG_READ_ONLY));
        }
        if let Some(wait_exit) = self.wait_exit {
            v.push(Self::bool_to_flag(wait_exit, CLIENT_FLAG_WAIT_EXIT));
        }
        let s = v.join(",");
        write!(f, "{}", s)
    }
}
