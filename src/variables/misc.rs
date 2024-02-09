#[derive(Default, PartialEq, Clone, Debug)]
pub struct Server {
    /// pid - Server PID
    #[cfg(feature = "tmux_2_1")]
    pub pid: Option<usize>,
    /// socket_path - Server socket path
    #[cfg(feature = "tmux_2_2")]
    pub socket_path: Option<String>,
    /// start_time - Server start time
    #[cfg(feature = "tmux_2_2")]
    pub start_time: Option<u128>,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct History {
    /// history_bytes             Number of bytes in window history
    #[cfg(feature = "tmux_1_7")]
    pub bytes: Option<usize>,
    /// history_limit             Maximum window history lines
    #[cfg(feature = "tmux_1_7")]
    pub limit: Option<usize>,
    /// history_size              Size of history in bytes
    #[cfg(feature = "tmux_1_7")]
    pub size: Option<usize>,
}

#[cfg(feature = "tmux_2_4")]
#[derive(Default, PartialEq, Clone, Debug)]
pub struct Hook {
    ///hook - Name of running hook, if any
    #[cfg(feature = "tmux_2_4")]
    pub hook: Option<String>,
    ///hook_pane - ID of pane where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    pub pane: Option<usize>,
    ///hook_session - ID of session where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    pub session: Option<usize>,
    ///hook_session_name - Name of session where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    pub session_name: Option<String>,
    ///hook_window - ID of window where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    pub window: Option<usize>,
    ///hook_window_name - Name of window where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    pub window_name: Option<String>,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Misc {
    /// alternate_on - if pane is in alternate screen
    #[cfg(feature = "tmux_1_8")]
    pub alternate_on: Option<usize>,
    /// alternate_saved_x - Saved cursor X in alternate screen
    #[cfg(feature = "tmux_1_8")]
    pub alternate_saved_x: Option<usize>,
    /// alternate_saved_y - Saved cursor Y in alternate screen
    #[cfg(feature = "tmux_1_8")]
    pub alternate_saved_y: Option<usize>,

    /// command_hooked - Name of command hooked, if any
    #[cfg(feature = "tmux_2_3")]
    pub command_hooked: Option<String>,
    /// command_name - Name of command in use, if any
    #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_2_4")))]
    pub command_name: Option<String>,
    /// command - Name of command in use, if any
    #[cfg(feature = "tmux_2_4")]
    pub command: Option<String>,
    /// command_list_name - Command name if listing commands
    #[cfg(feature = "tmux_2_3")]
    pub command_list_name: Option<String>,
    /// command_list_alias - Command alias if listing commands
    #[cfg(feature = "tmux_2_3")]
    pub command_list_alias: Option<String>,
    /// command_list_usage - Command usage if listing commands
    #[cfg(feature = "tmux_2_3")]
    pub command_list_usage: Option<String>,

    /// host - Hostname of local host
    #[cfg(feature = "tmux_1_6")]
    pub host: Option<String>,

    /// host_short - #h Hostname of local host (no domain name)
    #[cfg(feature = "tmux_1_9")]
    pub host_short: Option<String>,

    /// insert_flag - Pane insert flag
    #[cfg(feature = "tmux_1_8")]
    pub insert_flag: Option<String>,
    /// keypad_cursor_flag - Pane keypad cursor flag
    #[cfg(feature = "tmux_1_8")]
    pub keypad_cursor_flag: Option<String>,
    /// keypad_flag - Pane keypad flag
    #[cfg(feature = "tmux_1_8")]
    pub keypad_flag: Option<String>,

    /// line - Line number in the list
    #[cfg(feature = "tmux_1_6")]
    pub line: Option<usize>,

    /// origin_flag - Pane origin flag
    #[cfg(feature = "tmux_3_0")]
    pub origin_flag: Option<String>,

    /// saved_cursor_x - Saved cursor X in pane
    #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
    pub saved_cursor_x: Option<usize>,
    /// saved_cursor_y - Saved cursor Y in pane
    #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
    pub saved_cursor_y: Option<usize>,
    /// scroll_region_lower - Bottom of scroll region in pane
    #[cfg(feature = "tmux_1_8")]
    pub scroll_region_lower: Option<usize>,
    /// scroll_region_upper - Top of scroll region in pane
    #[cfg(feature = "tmux_1_8")]
    pub scroll_region_upper: Option<usize>,
    /// scroll_position - Scroll position in copy mode
    #[cfg(feature = "tmux_2_2")]
    pub scroll_position: Option<usize>,

    /// wrap_flag - Pane wrap flag
    #[cfg(feature = "tmux_1_8")]
    pub wrap_flag: Option<bool>,

    /// version - Server version
    #[cfg(feature = "tmux_2_4")]
    pub version: Option<String>,

    /// rectangle_toggle - 1 if rectangle selection is activated
    #[cfg(feature = "tmux_2_7")]
    pub rectangle_toggle: Option<bool>,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Cursor {
    /// cursor_flag - Pane cursor flag
    #[cfg(feature = "tmux_1_8")]
    pub flag: Option<String>,
    /// cursor_character - Character at cursor in pane
    #[cfg(feature = "tmux_2_9")]
    pub character: Option<String>,
    /// cursor_x - Cursor X position in pane
    pub x: Option<usize>,
    #[cfg(feature = "tmux_1_8")]
    /// cursor_y - Cursor Y position in pane
    #[cfg(feature = "tmux_1_8")]
    pub y: Option<usize>,

    /// copy_cursor_line - Line the cursor is on in copy mode
    #[cfg(feature = "tmux_3_1")]
    pub copy_cursor_line: Option<String>,
    /// copy_cursor_word - Word under cursor in copy mode
    #[cfg(feature = "tmux_3_1")]
    pub copy_cursor_word: Option<String>,
    /// copy_cursor_x - Cursor X position in copy mode
    #[cfg(feature = "tmux_3_1")]
    pub copy_cursor_x: Option<String>,
    /// copy_cursor_y - Cursor Y position in copy mode
    #[cfg(feature = "tmux_3_1")]
    pub copy_cursor_y: Option<String>,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Mouse {
    /// TODO: ? mouse_all_flag - Pane mouse all flag
    #[cfg(feature = "tmux_3_0")]
    pub all_flag: Option<String>,
    /// TODO: ? mouse_all_flag - Pane mouse all flag
    //#[cfg(feature = "tmux_2_4")]
    //pub all_flag: Option<String>,
    /// mouse_any_flag - Pane mouse any flag
    #[cfg(feature = "tmux_1_8")]
    pub any_flag: Option<String>,
    /// mouse_button_flag - Pane mouse button flag
    #[cfg(feature = "tmux_1_8")]
    pub button_flag: Option<String>,
    /// mouse_line - Line under mouse, if any
    #[cfg(feature = "tmux_3_0")]
    pub line: Option<String>,
    /// sgr_flag - Pane mouse SGR flag
    #[cfg(feature = "tmux_3_0")]
    pub mouse_sgr_flag: Option<String>,
    /// mouse_standard_flag - Pane mouse standard flag
    #[cfg(feature = "tmux_1_8")]
    pub standard_flag: Option<String>,
    /// mouse_utf8_flag - Pane mouse UTF-8 flag
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_2"), feature = "tmux_3_0"))]
    pub utf8_flag: Option<String>,
    /// mouse_word - Word under mouse, if any
    #[cfg(feature = "tmux_3_0")]
    pub word: Option<String>,
    /// mouse_x - Mouse X position, if any
    #[cfg(feature = "tmux_3_0")]
    pub x: Option<usize>,
    /// mouse_y - Mouse Y position, if any
    #[cfg(feature = "tmux_3_0")]
    pub y: Option<usize>,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Selection {
    /// selection_active - 1 if selection started and changes with the curso
    #[cfg(feature = "tmux_3_1")]
    pub active: Option<bool>,
    /// selection_end_x - X position of the end of the selection
    #[cfg(feature = "tmux_3_1")]
    pub end_x: Option<usize>,
    /// selection_end_y - Y position of the end of the selection
    #[cfg(feature = "tmux_3_1")]
    pub end_y: Option<usize>,
    /// selection_present - 1 if selection started in copy mode
    #[cfg(feature = "tmux_2_6")]
    pub present: Option<bool>,
    /// selection_start_x - X position of the start of the selection
    #[cfg(feature = "tmux_3_1")]
    pub start_x: Option<usize>,
    /// selection_start_y - Y position of the start of the selection
    #[cfg(feature = "tmux_3_1")]
    pub start_y: Option<usize>,
}
