// XXX: 1.9 pocessed
#[derive(Default, PartialEq, Clone, Debug)]
pub struct Client {
    /// client_activity           Integer time client last had activity
    #[cfg(feature = "tmux_1_6")]
    pub activity: Option<u128>,
    ///client_activity_string    String time client last had activity
    #[cfg(feature = "tmux_1_6")]
    pub activity_string: Option<String>,
    ///client_created            Integer time client created
    #[cfg(feature = "tmux_1_6")]
    pub created: Option<u128>,
    ///client_created_string     String time client created
    #[cfg(feature = "tmux_1_6")]
    pub created_string: Option<String>,
    ///client_cwd                Working directory of client
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub cwd: Option<String>,
    ///client_height             Height of client
    #[cfg(feature = "tmux_1_6")]
    pub height: Option<usize>,
///client_last_session       Name of the client's last session
    #[cfg(feature = "tmux_1_8")]
    pub last_session: Option<String>,
///client_prefix             1 if prefix key has been pressed
    #[cfg(feature = "tmux_1_8")]
    pub prefix: Option<bool>
    ///client_readonly           1 if client is readonly
    #[cfg(feature = "tmux_1_6")]
    pub readonly: Option<bool>,
    ///client_session            Name of the client's session
    #[cfg(feature = "tmux_1_8")]
    pub session: Option<String>,
    ///client_termname           Terminal name of client
    #[cfg(feature = "tmux_1_6")]
    pub termname: Option<String>,
    ///client_tty                Pseudo terminal of client
    #[cfg(feature = "tmux_1_6")]
    pub tty: Option<String>,
    ///client_utf8               1 if client supports utf8
    #[cfg(feature = "tmux_1_6")]
    pub utf8: Option<bool>,
    ///client_width              Width of client
    #[cfg(feature = "tmux_1_6")]
    pub width: Option<usize>,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Buffer {
    ///buffer_sample             First 50 characters from the specified buffer
    #[cfg(feature = "tmux_1_7")]
    pub sample: Option<String>,
    ///buffer_size               Size of the specified buffer in bytes
    #[cfg(feature = "tmux_1_7")]
    pub size: Option<usize>,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct History {
    ///history_bytes             Number of bytes in window history
    #[cfg(feature = "tmux_1_7")]
    pub bytes: Option<usize>,
    ///history_limit             Maximum window history lines
    #[cfg(feature = "tmux_1_7")]
    pub limit: Option<usize>,
    ///history_size              Size of history in bytes
    #[cfg(feature = "tmux_1_7")]
    pub size: Option<usize>,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Misc {
    ///alternate_on              If pane is in alternate screen
    #[cfg(feature = "tmux_1_8")]
    pub alternate_on: Option<usize>,
    ///alternate_saved_x         Saved cursor X in alternate screen
    #[cfg(feature = "tmux_1_8")]
    pub alternate_saved_x: Option<usize>,
    ///alternate_saved_y         Saved cursor Y in alternate screen
    #[cfg(feature = "tmux_1_8")]
    pub alternate_saved_y: Option<usize>,

    /// Hostname of local host
    #[cfg(feature = "tmux_1_6")]
    pub host: Option<String>,

    /// #h       Hostname of local host (no domain name)
    #[cfg(feature = "tmux_1_9")]
    pub host_short: Option<String>,

///insert_flag               Pane insert flag
    #[cfg(feature = "tmux_1_8")]
    pub insert_flag: Option<String>,
///keypad_cursor_flag        Pane keypad cursor flag
    #[cfg(feature = "tmux_1_8")]
    pub keypad_cursor_flag: Option<String>,
///keypad_flag               Pane keypad flag
    #[cfg(feature = "tmux_1_8")]
    pub keypad_flag: Option<String>,

    /// line                      Line number in the list
    #[cfg(feature = "tmux_1_6")]
    pub line: Option<usize>,

    ///saved_cursor_x            Saved cursor X in pane
    #[cfg(feature = "tmux_1_8")]
    pub saved_cursor_x: Option<usize>,
    ///saved_cursor_y            Saved cursor Y in pane
    #[cfg(feature = "tmux_1_8")]
    pub saved_cursor_y: Option<usize>,
    ///scroll_region_lower       Bottom of scroll region in pane
    #[cfg(feature = "tmux_1_8")]
    pub scroll_region_lower: Option<usize>,
    ///scroll_region_upper       Top of scroll region in pane
    #[cfg(feature = "tmux_1_8")]
    pub scroll_region_upper: Option<usize>,

    ///wrap_flag                 Pane wrap flag
    #[cfg(feature = "tmux_1_8")]
    pub wrap_flag: Option<bool>,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Cursor {
    ///cursor_flag               Pane cursor flag
    #[cfg(feature = "tmux_1_8")]
    pub flag: Option<String>,
    ///cursor_x                  Cursor X position in pane
    pub x: Option<usize>,
    #[cfg(feature = "tmux_1_8")]
    ///cursor_y                  Cursor Y position in pane
    #[cfg(feature = "tmux_1_8")]
    pub y: Option<usize>,
}


#[derive(Default, PartialEq, Clone, Debug)]
pub struct Mouse {
    ///mouse_any_flag            Pane mouse any flag
    #[cfg(feature = "tmux_1_8")]
    pub any_flag: Option<String>,
    ///mouse_button_flag         Pane mouse button flag
    #[cfg(feature = "tmux_1_8")]
    pub button_flag: Option<String>,
    ///mouse_standard_flag       Pane mouse standard flag
    #[cfg(feature = "tmux_1_8")]
    pub standard_flag: Option<String>,
    ///mouse_utf8_flag           Pane mouse UTF-8 flag
    #[cfg(feature = "tmux_1_8")]
    pub utf8_flag: Option<String>,
}

