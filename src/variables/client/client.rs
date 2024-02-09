use crate::Error;
use crate::FormatsOutput;
use std::str::FromStr;

// XXX: 1.9 processed
#[derive(Default, PartialEq, Clone, Debug)]
pub struct Client {
    /// client_activity - Integer time client last had activity
    #[cfg(feature = "tmux_1_6")]
    pub activity: Option<u128>,
    /// client_cell_height - Height of each client cell in pixels
    #[cfg(feature = "tmux_3_1")]
    pub cell_height: Option<usize>,
    /// client_cell_width - Width of each client cell in pixels
    #[cfg(feature = "tmux_3_1")]
    pub cell_width: Option<usize>,
    /// client_activity_string - String time client last had activity
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    pub activity_string: Option<String>,
    /// client_created - Integer time client created
    #[cfg(feature = "tmux_1_6")]
    pub created: Option<u128>,
    /// client_created_string - String time client created
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    pub created_string: Option<String>,
    /// client_control_mode - 1 if client is in control mode
    #[cfg(feature = "tmux_2_1")]
    pub control_mode: Option<bool>,
    /// client_discarded - Bytes discarded when client behind
    #[cfg(feature = "tmux_2_1")]
    pub discarded: Option<String>,
    /// client_cwd - Working directory of client
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub cwd: Option<String>,
    /// client_height - Height of client
    #[cfg(feature = "tmux_1_6")]
    pub height: Option<usize>,
    /// client_key_table - Current key table
    #[cfg(feature = "tmux_2_2")]
    pub key_table: Option<String>,
    /// client_last_session - Name of the client's last session
    #[cfg(feature = "tmux_1_8")]
    pub last_session: Option<String>,
    /// client_name - Name of client
    #[cfg(feature = "tmux_2_4")]
    pub name: Option<String>,
    /// client_pid - PID of client process
    #[cfg(feature = "tmux_2_1")]
    pub pid: Option<usize>,
    /// client_prefix - 1 if prefix key has been pressed
    #[cfg(feature = "tmux_1_8")]
    pub prefix: Option<bool>,
    /// client_readonly - 1 if client is readonly
    #[cfg(feature = "tmux_1_6")]
    pub readonly: Option<bool>,
    /// client_session - Name of the client's session
    #[cfg(feature = "tmux_1_8")]
    pub session: Option<String>,
    /// client_termname - Terminal name of client
    #[cfg(feature = "tmux_1_6")]
    pub termname: Option<String>,
    /// client_termtype - Terminal type of client
    #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_1")))]
    pub termtype: Option<String>,
    /// client_tty - Pseudo terminal of client
    #[cfg(feature = "tmux_1_6")]
    pub tty: Option<String>,
    /// client_utf8 - 1 if client supports UTF-8
    #[cfg(feature = "tmux_1_6")]
    pub utf8: Option<bool>,
    /// client_width - Width of client
    #[cfg(feature = "tmux_1_6")]
    pub width: Option<usize>,
    /// client_written - Bytes written to client
    #[cfg(feature = "tmux_2_4")]
    pub written: Option<usize>,
}

impl FromStr for Client {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let mut client = Client::new();
        let mut format = FormatsOutput::new();
        format.separator(':');

        #[cfg(feature = "tmux_1_6")]
        format.client_activity(&mut client.activity);
        #[cfg(feature = "tmux_3_1")]
        format.client_cell_height(&mut client.cell_height);
        #[cfg(feature = "tmux_3_1")]
        format.client_cell_width(&mut client.cell_width);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        format.client_activity_string(&mut client.activity_string);
        #[cfg(feature = "tmux_1_6")]
        format.client_created(&mut client.created);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        format.client_created_string(&mut client.created_string);
        #[cfg(feature = "tmux_2_1")]
        format.client_control_mode(&mut client.control_mode);
        #[cfg(feature = "tmux_2_1")]
        format.client_discarded(&mut client.discarded);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        format.client_cwd(&mut client.cwd);
        #[cfg(feature = "tmux_1_6")]
        format.client_height(&mut client.height);
        #[cfg(feature = "tmux_2_2")]
        format.client_key_table(&mut client.key_table);
        #[cfg(feature = "tmux_1_8")]
        format.client_last_session(&mut client.last_session);
        #[cfg(feature = "tmux_2_4")]
        format.client_name(&mut client.name);
        #[cfg(feature = "tmux_2_1")]
        format.client_pid(&mut client.pid);
        #[cfg(feature = "tmux_1_8")]
        format.client_prefix(&mut client.prefix);
        #[cfg(feature = "tmux_1_6")]
        format.client_readonly(&mut client.readonly);
        #[cfg(feature = "tmux_1_8")]
        format.client_session(&mut client.session);
        #[cfg(feature = "tmux_1_6")]
        format.client_termname(&mut client.termname);
        #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_1")))]
        format.client_termtype(&mut client.termtype);
        #[cfg(feature = "tmux_1_6")]
        format.client_tty(&mut client.tty);
        #[cfg(feature = "tmux_1_6")]
        format.client_utf8(&mut client.utf8);
        #[cfg(feature = "tmux_1_6")]
        format.client_width(&mut client.width);
        #[cfg(feature = "tmux_2_4")]
        format.client_written(&mut client.written);

        FormatsOutput::from_string_ext(s, &mut format);
        Ok(client)
    }
}

impl Client {
    pub fn new() -> Self {
        Default::default()
    }

    // XXX: wrapper with format generating and result parsing using callback
}
