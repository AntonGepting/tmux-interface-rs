use std::fmt;

// XXX: ? + - etc refactor in structure in future? split in enum and struct add fields
// XXX: options allowed too
#[derive(Debug)]
pub enum Variable {
    //Custom(String)
    /// `alternate_on` - if pane is in alternate screen
    #[cfg(feature = "tmux_1_8")]
    AlternateOn,
    /// `alternate_saved_x` - Saved cursor X in alternate screen
    #[cfg(feature = "tmux_1_8")]
    AlternateSavedX,
    /// `alternate_saved_y` - Saved cursor Y in alternate screen
    #[cfg(feature = "tmux_1_8")]
    AlternateSavedY,

    // Buffer
    /// `buffer_created` - Time buffer created
    #[cfg(feature = "tmux_2_6")]
    BufferCreated,
    /// `buffer_name` - Name of buffer
    #[cfg(feature = "tmux_2_3")]
    BufferName,
    /// `buffer_sample` - First 50 characters from the specified buffer
    #[cfg(feature = "tmux_1_7")]
    BufferSample,
    /// `buffer_size` - Size of the specified buffer in bytes
    #[cfg(feature = "tmux_1_7")]
    BufferSize,

    // Client
    /// `client_activity` - Integer time client last had activity
    #[cfg(feature = "tmux_1_6")]
    ClientActivity,
    /// `client_cell_height` - Height of each client cell in pixels
    #[cfg(feature = "tmux_3_1")]
    ClientCellHeight,
    /// `client_cell_width` - Width of each client cell in pixels
    #[cfg(feature = "tmux_3_1")]
    ClientCellWidth,
    /// `client_activity_string` - Option<String> time client last had activity
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    ClientActivityString,
    /// `client_created` - Integer time client created
    #[cfg(feature = "tmux_1_6")]
    ClientCreated,
    /// `client_created_string` - Option<String> time client created
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    ClientCreatedString,
    /// `client_control_mode` - 1 if client is in control mode
    #[cfg(feature = "tmux_2_1")]
    ClientControlMode,
    /// `client_discarded` - Bytes discarded when client behind
    #[cfg(feature = "tmux_2_1")]
    ClientDiscarded,
    /// `client_cwd` - Working directory of client
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    ClientCwd,
    /// `client_height` - Height of client
    #[cfg(feature = "tmux_1_6")]
    ClientHeight,
    /// `client_key_table` - Current key table
    #[cfg(feature = "tmux_2_2")]
    ClientKeyTable,
    /// `client_last_session` - Name of the client's last session
    #[cfg(feature = "tmux_1_8")]
    ClientLastSession,
    /// `client_name` - Name of client
    #[cfg(feature = "tmux_2_4")]
    ClientName,
    /// `client_pid` - PID of client process
    #[cfg(feature = "tmux_2_1")]
    ClientPid,
    /// `client_prefix` - 1 if prefix key has been pressed
    #[cfg(feature = "tmux_1_8")]
    ClientPrefix,
    /// `client_readonly` - 1 if client is readonly
    #[cfg(feature = "tmux_1_6")]
    ClientReadonly,
    /// `client_session` - Name of the client's session
    #[cfg(feature = "tmux_1_8")]
    ClientSession,
    /// `client_termname` - Terminal name of client
    #[cfg(feature = "tmux_1_6")]
    ClientTermname,
    /// `client_termtype` - Terminal type of client
    #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_1")))]
    ClientTermtype,
    /// `client_tty` - Pseudo terminal of client
    #[cfg(feature = "tmux_1_6")]
    ClientTty,
    /// `client_utf8` - 1 if client supports UTF-8
    #[cfg(feature = "tmux_1_6")]
    ClientUtf8,
    /// `client_width` - Width of client
    #[cfg(feature = "tmux_1_6")]
    ClientWidth,
    /// `client_written` - Bytes written to client
    #[cfg(feature = "tmux_2_4")]
    ClientWritten,

    // Command
    /// `command_hooked` - Name of command hooked, if any
    #[cfg(feature = "tmux_2_3")]
    CommandHooked,
    /// `command_name` - Name of command in use, if any
    #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_2_4")))]
    CommandName,
    /// `command` - Name of command in use, if any
    #[cfg(feature = "tmux_2_4")]
    Command,
    /// `command_list_name` - Command name if listing commands
    #[cfg(feature = "tmux_2_3")]
    CommandListName,
    /// `command_list_alias` - Command alias if listing commands
    #[cfg(feature = "tmux_2_3")]
    CommandListAlias,
    /// `command_list_usage` - Command usage if listing commands
    #[cfg(feature = "tmux_2_3")]
    CommandListUsage,

    // Cursor
    /// `cursor_flag` - Pane cursor flag
    #[cfg(feature = "tmux_1_8")]
    CursorFlag,
    /// `cursor_character` - Character at cursor in pane
    #[cfg(feature = "tmux_2_9")]
    CursorCharacter,
    /// `cursor_x` - Cursor X position in pane
    #[cfg(feature = "tmux_1_8")]
    CursorX,
    /// `cursor_y` - Cursor Y position in pane
    #[cfg(feature = "tmux_1_8")]
    CursorY,

    /// `copy_cursor_line` - Line the cursor is on in copy mode
    #[cfg(feature = "tmux_3_1")]
    CopyCursorLine,
    /// `copy_cursor_word` - Word under cursor in copy mode
    #[cfg(feature = "tmux_3_1")]
    CopyCursorWord,
    /// `copy_cursor_x` - Cursor X position in copy mode
    #[cfg(feature = "tmux_3_1")]
    CopyCursorX,
    /// `copy_cursor_y` - Cursor Y position in copy mode
    #[cfg(feature = "tmux_3_1")]
    CopyCursorY,
    /// `current_file` - Current configuration file
    #[cfg(feature = "tmux_3_2")]
    CurrentFile,

    // history
    /// `history_bytes`             Number of bytes in window history
    #[cfg(feature = "tmux_1_7")]
    HistotyBytes,
    /// `history_limit`             Maximum window history lines
    #[cfg(feature = "tmux_1_7")]
    HistotyLimit,
    /// `history_size`              Size of history in bytes
    #[cfg(feature = "tmux_1_7")]
    HistorySize,

    // hook
    /// `hook` - Name of running hook, if any
    #[cfg(feature = "tmux_2_4")]
    Hook,
    /// `hook_pane` - ID of pane where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    HookPane,
    /// `hook_session` - ID of session where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    HookSession,
    /// `hook_session_name` - Name of session where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    HookSessionName,
    /// `hook_window` - ID of window where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    HookWindow,
    /// `hook_window_name` - Name of window where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    HookWindowName,

    // host
    /// `host` - Hostname of local host
    #[cfg(feature = "tmux_1_6")]
    Host,
    /// `host_short` - #h Hostname of local host (no domain name)
    #[cfg(feature = "tmux_1_9")]
    HostShort,

    /// `insert_flag` - Pane insert flag
    #[cfg(feature = "tmux_1_8")]
    InsertFlag,
    /// `keypad_cursor_flag` - Pane keypad cursor flag
    #[cfg(feature = "tmux_1_8")]
    KeypadCursorFlag,
    /// `keypad_flag` - Pane keypad flag
    #[cfg(feature = "tmux_1_8")]
    KeypadFlag,

    /// `line` - Line number in the list
    #[cfg(feature = "tmux_1_6")]
    Line,

    // `mouse_all_flag` - Pane mouse all flag
    //#[cfg(feature = "tmux_3_0")]
    //MouseAllFlag,
    /// `mouse_any_flag` - Pane mouse any flag
    #[cfg(feature = "tmux_1_8")]
    MouseAnyFlag,
    /// `mouse_button_flag` - Pane mouse button flag
    #[cfg(feature = "tmux_1_8")]
    MouseButtonFlag,
    /// `mouse_line` - Line under mouse, if any
    #[cfg(feature = "tmux_3_0")]
    MouseLine,
    /// `sgr_flag` - Pane mouse SGR flag
    #[cfg(feature = "tmux_3_0")]
    MouseSgrFlag,
    /// `mouse_standard_flag` - Pane mouse standard flag
    #[cfg(feature = "tmux_1_8")]
    MouseStandardFlag,
    /// `mouse_utf8_flag` - Pane mouse UTF-8 flag
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_2"), feature = "tmux_3_0"))]
    MouseUtf8Flag,
    /// `mouse_all_flag` - Pane mouse all flag
    #[cfg(feature = "tmux_2_4")]
    MouseAllFlag,
    /// `mouse_word` - Word under mouse, if any
    #[cfg(feature = "tmux_3_0")]
    MouseWord,
    /// `mouse_x` - Mouse X position, if any
    #[cfg(feature = "tmux_3_0")]
    MouseX,
    /// `mouse_y` - Mouse Y position, if any
    #[cfg(feature = "tmux_3_0")]
    MouseY,
    /// `origin_flag` - Pane origin flag
    #[cfg(feature = "tmux_3_0")]
    OriginFlag,

    // pane
    /// `pane_active` - 1 if active pane
    #[cfg(feature = "tmux_1_6")]
    PaneActive,
    /// `pane_at_bottom` - 1 if pane is at the bottom of window
    #[cfg(feature = "tmux_2_6")]
    PaneAtBottom,
    /// `pane_at_left` - 1 if pane is at the left of window
    #[cfg(feature = "tmux_2_6")]
    PaneAtLeft,
    /// `pane_at_right` - 1 if pane is at the right of window
    #[cfg(feature = "tmux_2_6")]
    PaneAtRight,
    /// `pane_at_top` - 1 if pane is at the top of window
    #[cfg(feature = "tmux_2_6")]
    PaneAtTop,
    /// `pane_bottom` - Bottom of pane
    #[cfg(feature = "tmux_2_0")]
    PaneBottom,
    /// `pane_current_command` - Current command if available
    #[cfg(feature = "tmux_1_8")]
    PaneCurrentCommand,
    /// `pane_current_path` - Current path if available
    #[cfg(feature = "tmux_1_7")]
    PaneCurrentPath,
    /// `pane_dead` - 1 if pane is dead
    #[cfg(feature = "tmux_1_6")]
    PaneDead,
    /// `pane_dead_status` - Exit status of process in dead pane
    #[cfg(feature = "tmux_2_0")]
    PaneDeadStatus,
    /// `pane_format` - 1 if format is for a pane
    #[cfg(feature = "tmux_2_6")]
    PaneFormat,
    /// `pane_height` - Height of pane
    #[cfg(feature = "tmux_1_6")]
    PaneHeight,
    /// `pane_id` - #D Unique pane ID
    #[cfg(feature = "tmux_1_6")]
    PaneId,
    /// `pane_in_mode` - 1 if pane is in a mode
    #[cfg(feature = "tmux_1_8")]
    PaneInMode,
    /// `pane_index` - #P Index of pane
    #[cfg(feature = "tmux_1_7")]
    PaneIndex,
    /// `pane_input_off` - 1 if input to pane is disabled
    #[cfg(feature = "tmux_2_0")]
    PaneInputOff,
    /// `pane_left` - Left of pane
    #[cfg(feature = "tmux_2_0")]
    PaneLeft,
    /// `pane_marked` - 1 if this is the marked pane
    #[cfg(feature = "tmux_3_0")]
    PaneMarked,
    /// `pane_marked_set` - 1 if a marked pane is set
    #[cfg(feature = "tmux_3_0")]
    PaneMarkedSet,
    /// `pane_mode` - Name of pane mode, if any
    #[cfg(feature = "tmux_2_5")]
    PaneMode,
    /// `pane_path` - #T Path of pane (can be set by application)
    #[cfg(feature = "tmux_3_1")]
    PanePath,
    /// `pane_pid` - PID of first process in pane
    #[cfg(feature = "tmux_1_6")]
    PanePid,
    /// `pane_pipe` - 1 if pane is being piped
    #[cfg(feature = "tmux_2_6")]
    PanePipe,
    /// `pane_right` - Right of pane
    #[cfg(feature = "tmux_2_0")]
    PaneRight,
    /// `Last` search Option<String> in copy mode
    #[cfg(feature = "tmux_2_5")]
    PaneSearchString,
    /// `pane_start_command` - Command pane started with
    #[cfg(feature = "tmux_1_6")]
    PaneStartCommand,
    /// `pane_start_path` - Path pane started with
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_0")))]
    PaneStartPath,
    /// `pane_synchronized` - 1 if pane is synchronized
    #[cfg(feature = "tmux_1_9")]
    PaneSynchronized,
    /// `pane_tabs` - Pane tab positions
    #[cfg(feature = "tmux_1_8")]
    PaneTabs,
    /// `pane_title` - #T Title of pane (can be set by application)
    #[cfg(feature = "tmux_1_6")]
    PaneTitle,
    /// `pane_top` - Top of pane
    #[cfg(feature = "tmux_2_0")]
    PaneTop,
    /// `pane_tty` - Pseudo terminal of pane
    #[cfg(feature = "tmux_1_6")]
    PaneTty,
    /// `pane_width` - Width of pane
    #[cfg(feature = "tmux_1_6")]
    PaneWidth,

    /// `saved_cursor_x` - Saved cursor X in pane
    #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
    SavedCursorX,
    /// `saved_cursor_y` - Saved cursor Y in pane
    #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
    SavedCursorY,

    /// `pid` - Server PID
    #[cfg(feature = "tmux_2_1")]
    Pid,
    /// `rectangle_toggle` - 1 if rectangle selection is activated
    #[cfg(feature = "tmux_2_7")]
    RectangleToggle,

    /// `scroll_position` - Scroll position in copy mode
    #[cfg(feature = "tmux_2_2")]
    ScrollPosition,
    /// `scroll_region_lower` - Bottom of scroll region in pane
    #[cfg(feature = "tmux_1_8")]
    ScrollRegionLower,
    /// `scroll_region_upper` - Top of scroll region in pane
    #[cfg(feature = "tmux_1_8")]
    ScrollRegionUpper,

    /// `selection_active` - 1 if selection started and changes with the curso
    #[cfg(feature = "tmux_3_1")]
    SelectionActive,
    /// `selection_end_x` - X position of the end of the selection
    #[cfg(feature = "tmux_3_1")]
    SelectionEndX,
    /// `selection_end_y` - Y position of the end of the selection
    #[cfg(feature = "tmux_3_1")]
    SelectionEndY,
    /// `selection_present` - 1 if selection started in copy mode
    #[cfg(feature = "tmux_2_6")]
    SelectionPresent,
    /// `selection_start_x` - X position of the start of the selection
    #[cfg(feature = "tmux_3_1")]
    SelectionStartX,
    /// `selection_start_y` - Y position of the start of the selection
    #[cfg(feature = "tmux_3_1")]
    SelectionStartY,

    // Session
    /// `session_activity` - Time of session last activity
    #[cfg(feature = "tmux_2_1")]
    SessionActivity,
    /// `session_activity_string` - Option<String> time of session last activity
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    SessionActivityString,
    /// `session_alerts` - List of window indexes with alerts
    #[cfg(feature = "tmux_2_1")]
    SessionAlerts,
    /// `session_attached` - Number of clients session is attached to
    #[cfg(feature = "tmux_1_6")]
    SessionAttached,
    /// `session_attached_list` - List of clients session is attached to
    #[cfg(feature = "tmux_3_1")]
    SessionAttachedList,
    /// `session_created` - Time session created
    #[cfg(feature = "tmux_1_6")]
    SessionCreated,
    /// `session_created_string` - Option<String> time session created
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    SessionCreatedString,
    /// `session_format` - 1 if format is for a session (not assuming the current)
    #[cfg(feature = "tmux_2_6")]
    SessionFormat,
    /// `session_group` - Name of session group
    #[cfg(feature = "tmux_1_6")]
    SessionGroup,
    /// `session_group_attached` - Number of clients sessions in group are attached >
    #[cfg(feature = "tmux_3_1")]
    SessionGroupAttached,
    /// `session_group_attached_list` - List of clients sessions in group are attached to
    #[cfg(feature = "tmux_3_1")]
    SessionGroupAttachedList,
    /// `session_group_list` - List of sessions in group
    #[cfg(feature = "tmux_2_7")]
    SessionGroupList,
    /// `session_group_many_attached` - 1 if multiple clients attached to sessions in gro
    #[cfg(feature = "tmux_3_1")]
    SessionGroupManyAttached,
    /// `session_size` - Size of session group
    #[cfg(feature = "tmux_2_7")]
    SessionGroupSize,
    /// `session_grouped` - 1 if session in a group
    #[cfg(feature = "tmux_1_6")]
    SessionGrouped,
    /// `session_height` - Height of session
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    SessionHeight,
    /// `session_width` - Width of session
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    SessionWidth,
    /// `session_id` - Unique session ID
    #[cfg(feature = "tmux_1_8")]
    SessionId,
    /// `session_last_attached` - Time session last attached
    #[cfg(feature = "tmux_2_1")]
    SessionLastAttached,
    /// `session_last_attached_string` - Option<String> time session last attached
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    SessionLastAttachedString,
    /// `session_many_attached` - 1 if multiple clients attached
    #[cfg(feature = "tmux_2_0")]
    SessionManyAttached,
    /// `session_name` - #S Name of session
    #[cfg(feature = "tmux_1_6")]
    SessionName,
    /// `session_stack` - Window indexes in most recent order
    #[cfg(feature = "tmux_2_5")]
    SessionStack,
    /// `session_windows` - Number of windows in session
    #[cfg(feature = "tmux_1_6")]
    SessionWindows,

    /// `socket_path` - Server socket path
    #[cfg(feature = "tmux_2_2")]
    SocketPath,
    /// `start_time` - Server start time
    #[cfg(feature = "tmux_2_2")]
    StartTime,

    /// `version` - Server version
    #[cfg(feature = "tmux_2_4")]
    Version,

    // Window
    //
    /// `window_active` - 1 if window active
    #[cfg(feature = "tmux_1_6")]
    WindowActive,
    /// `window_active_clients` - Number of clients viewing this window
    #[cfg(feature = "tmux_3_1")]
    WindowActiveClients,
    /// `window_active_clients_list` - List of clients viewing this window
    #[cfg(feature = "tmux_3_1")]
    WindowActiveClientsList,
    /// `window_active_sessions` - Number of sessions on which this window is active
    #[cfg(feature = "tmux_3_1")]
    WindowActiveSessions,
    /// `window_active_sessions_list` - List of sessions on which this window is active
    #[cfg(feature = "tmux_3_1")]
    WindowActiveSessionsList,
    /// `window_activity` - Time of window last activity
    #[cfg(feature = "tmux_2_1")]
    WindowActivity,
    /// `window_activity_string` - String time of window last activity
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    WindowActivityString,
    /// `window_activity_flag` - 1 if window has activity
    #[cfg(any(
        all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
        feature = "tmux_2_3"
    ))]
    WindowActivityFlag,
    /// `window_bell_flag` - 1 if window has bell
    #[cfg(feature = "tmux_1_9")]
    WindowBellFlag,
    /// `window_content_flag` - 1 if window has content alert
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    WindowContentFlag,
    /// `window_bigger` - 1 if window is larger than client
    #[cfg(feature = "tmux_2_9")]
    WindowBigger,
    /// `window_cell_height` - Height of each cell in pixels
    #[cfg(feature = "tmux_3_1")]
    WindowCellHeight,
    /// `window_cell_width` - Width of each cell in pixels
    #[cfg(feature = "tmux_3_1")]
    WindowCellWidth,
    /// `window_end_flag` - 1 if window has the highest index
    #[cfg(feature = "tmux_2_9")]
    WindowEndFlag,
    /// `window_find_matches` - Matched data from the find-window command if available
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    WindowFindMatches,
    /// `window_flags` - #F Window flags
    #[cfg(feature = "tmux_1_6")]
    WindowFlags,
    // TODO: WindowRawFlags
    /// `window_raw_flags` - Window flags with nothing escaped
    #[cfg(feature = "tmux_3_2")]
    WindowRawFlags,
    /// `window_format` - 1 if format is for a window
    #[cfg(feature = "tmux_2_6")]
    WindowFormat,
    /// `window_height` - Height of window
    #[cfg(feature = "tmux_1_6")]
    WindowHeight,
    /// `window_id` - Unique window ID
    #[cfg(feature = "tmux_1_7")]
    WindowId,
    /// `window_index` - #I Index of window
    #[cfg(feature = "tmux_1_6")]
    WindowIndex,
    /// `window_last_flag` - 1 if window is the last used
    #[cfg(feature = "tmux_2_0")]
    WindowLastFlag,
    /// `window_layout` - Window layout description, ignoring zoomed window panes
    #[cfg(feature = "tmux_1_6")]
    WindowLayout,
    /// `window_linked` - 1 if window is linked across sessions
    #[cfg(feature = "tmux_2_1")]
    WindowLinked,
    /// `window_linked_sessions` - Number of sessions this window is linked to
    #[cfg(feature = "tmux_3_1")]
    WindowLinkedSessions,
    /// `window_linked_sessions_list` - List of sessions this window is linked to
    #[cfg(feature = "tmux_3_1")]
    WindowLinkedSessionsList,
    /// `window_marked_flag` - 1 if window contains the marked pane
    #[cfg(feature = "tmux_3_1")]
    WindowMarkedFlag,
    /// `window_name` - #W Name of window
    #[cfg(feature = "tmux_1_6")]
    WindowName,
    /// `window_offset_x` - X offset into window if larger than client
    #[cfg(feature = "tmux_2_9")]
    WindowOffsetX,
    /// `window_offset_y` - Y offset into window if larger than client
    #[cfg(feature = "tmux_2_9")]
    WindowOffsetY,
    /// `window_panes` - Number of panes in window
    #[cfg(feature = "tmux_1_7")]
    WindowPanes,
    /// `window_silence_flag` - 1 if window has silence alert
    #[cfg(feature = "tmux_1_9")]
    WindowSilenceFlag,
    /// `window_stack_index` - Index in session most recent stack
    #[cfg(feature = "tmux_2_5")]
    WindowStackIndex,
    /// `window_start_flag` - 1 if window has the lowest index
    #[cfg(feature = "tmux_2_9")]
    WindowStartFlag,
    /// `window_visible_layout` - Window layout description, respecting zoomed window panes
    #[cfg(feature = "tmux_2_2")]
    WindowVisibleLayout,
    /// `window_width` - Width of window
    #[cfg(feature = "tmux_1_6")]
    WindowWidth,
    /// `window_zoomed_flag` - 1 if window is zoomed
    #[cfg(feature = "tmux_2_0")]
    WindowZoomedFlag,

    /// `wrap_flag` - Pane wrap flag
    #[cfg(feature = "tmux_1_8")]
    WrapFlag,
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            // alternate_on - if pane is in alternate screen
            #[cfg(feature = "tmux_1_8")]
            Self::AlternateOn => "alternate_on",
            // alternate_saved_x - Saved cursor X in alternate screen
            #[cfg(feature = "tmux_1_8")]
            Self::AlternateSavedX => "alternate_saved_x",
            // alternate_saved_y - Saved cursor Y in alternate screen
            #[cfg(feature = "tmux_1_8")]
            Self::AlternateSavedY => "alternate_saved_y",

            // Buffer
            // buffer_created - Time buffer created
            #[cfg(feature = "tmux_2_6")]
            Self::BufferCreated => "buffer_created",
            // buffer_name - Name of buffer
            #[cfg(feature = "tmux_2_3")]
            Self::BufferName => "buffer_name",
            // buffer_sample - First 50 characters from the specified buffer
            #[cfg(feature = "tmux_1_7")]
            Self::BufferSample => "buffer_sample",
            // buffer_size - Size of the specified buffer in bytes
            #[cfg(feature = "tmux_1_7")]
            Self::BufferSize => "buffer_size",

            // Client
            // client_activity - Integer time client last had activity
            #[cfg(feature = "tmux_1_6")]
            Self::ClientActivity => "client_activity",
            // client_cell_height - Height of each client cell in pixels
            #[cfg(feature = "tmux_3_1")]
            Self::ClientCellHeight => "client_cell_height",
            // client_cell_width - Width of each client cell in pixels
            #[cfg(feature = "tmux_3_1")]
            Self::ClientCellWidth => "client_cell_width",
            // client_activity_string - Option<String> time client last had activity
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
            Self::ClientActivityString => "client_activity_string",
            // client_created - Integer time client created
            #[cfg(feature = "tmux_1_6")]
            Self::ClientCreated => "client_created",
            // client_created_string - Option<String> time client created
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
            Self::ClientCreatedString => "client_created_string",
            // client_control_mode - 1 if client is in control mode
            #[cfg(feature = "tmux_2_1")]
            Self::ClientControlMode => "client_control_mode",
            // client_discarded - Bytes discarded when client behind
            #[cfg(feature = "tmux_2_1")]
            Self::ClientDiscarded => "client_discarded",
            // client_cwd - Working directory of client
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            Self::ClientCwd => "client_cwd",
            // client_height - Height of client
            #[cfg(feature = "tmux_1_6")]
            Self::ClientHeight => "client_height",
            // client_key_table - Current key table
            #[cfg(feature = "tmux_2_2")]
            Self::ClientKeyTable => "client_key_table",
            // client_last_session - Name of the client's last session
            #[cfg(feature = "tmux_1_8")]
            Self::ClientLastSession => "client_last_session",
            // client_name - Name of client
            #[cfg(feature = "tmux_2_4")]
            Self::ClientName => "client_name",
            // client_pid - PID of client process
            #[cfg(feature = "tmux_2_1")]
            Self::ClientPid => "client_pid",
            // client_prefix - 1 if prefix key has been pressed
            #[cfg(feature = "tmux_1_8")]
            Self::ClientPrefix => "client_prefix",
            // client_readonly - 1 if client is readonly
            #[cfg(feature = "tmux_1_6")]
            Self::ClientReadonly => "client_readonly",
            // client_session - Name of the client's session
            #[cfg(feature = "tmux_1_8")]
            Self::ClientSession => "client_session",
            // client_termname - Terminal name of client
            #[cfg(feature = "tmux_1_6")]
            Self::ClientTermname => "client_termname",
            // client_termtype - Terminal type of client
            #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_1")))]
            Self::ClientTermtype => "client_termtype",
            // client_tty - Pseudo terminal of client
            #[cfg(feature = "tmux_1_6")]
            Self::ClientTty => "client_tty",
            // client_utf8 - 1 if client supports UTF-8
            #[cfg(feature = "tmux_1_6")]
            Self::ClientUtf8 => "client_utf8",
            // client_width - Width of client
            #[cfg(feature = "tmux_1_6")]
            Self::ClientWidth => "client_width",
            // client_written - Bytes written to client
            #[cfg(feature = "tmux_2_4")]
            Self::ClientWritten => "client_written",

            // Command
            // command_hooked - Name of command hooked, if any
            #[cfg(feature = "tmux_2_3")]
            Self::CommandHooked => "command_hooked",
            // command_name - Name of command in use, if any
            #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_2_4")))]
            Self::CommandName => "command_name",
            // command - Name of command in use, if any
            #[cfg(feature = "tmux_2_4")]
            Self::Command => "command",
            // command_list_name - Command name if listing commands
            #[cfg(feature = "tmux_2_3")]
            Self::CommandListName => "command_list_name",
            // command_list_alias - Command alias if listing commands
            #[cfg(feature = "tmux_2_3")]
            Self::CommandListAlias => "command_list_alias",
            // command_list_usage - Command usage if listing commands
            #[cfg(feature = "tmux_2_3")]
            Self::CommandListUsage => "command_list_usage",

            // Cursor
            // cursor_flag - Pane cursor flag
            #[cfg(feature = "tmux_1_8")]
            Self::CursorFlag => "cursor_flag",
            // cursor_character - Character at cursor in pane
            #[cfg(feature = "tmux_2_9")]
            Self::CursorCharacter => "cursor_character",
            // cursor_x - Cursor X position in pane
            #[cfg(feature = "tmux_1_8")]
            Self::CursorX => "cursor_x",
            // cursor_y - Cursor Y position in pane
            #[cfg(feature = "tmux_1_8")]
            Self::CursorY => "cursor_y",

            // copy_cursor_line - Line the cursor is on in copy mode
            #[cfg(feature = "tmux_3_1")]
            Self::CopyCursorLine => "copy_cursor_line",
            // copy_cursor_word - Word under cursor in copy mode
            #[cfg(feature = "tmux_3_1")]
            Self::CopyCursorWord => "copy_cursor_word",
            // copy_cursor_x - Cursor X position in copy mode
            #[cfg(feature = "tmux_3_1")]
            Self::CopyCursorX => "copy_cursor_x",
            // copy_cursor_y - Cursor Y position in copy mode
            #[cfg(feature = "tmux_3_1")]
            Self::CopyCursorY => "copy_cursor_y",
            // current_file - Current configuration file
            #[cfg(feature = "tmux_3_2")]
            Self::CurrentFile => "current_file",

            // history
            // history_bytes             Number of bytes in window history
            #[cfg(feature = "tmux_1_7")]
            Self::HistotyBytes => "history_bytes",
            // history_limit             Maximum window history lines
            #[cfg(feature = "tmux_1_7")]
            Self::HistotyLimit => "history_limit",
            // history_size              Size of history in bytes
            #[cfg(feature = "tmux_1_7")]
            Self::HistorySize => "history_size",

            // hook
            // hook - Name of running hook, if any
            #[cfg(feature = "tmux_2_4")]
            Self::Hook => "hook",
            // hook_pane - ID of pane where hook was run, if any
            #[cfg(feature = "tmux_2_4")]
            Self::HookPane => "hook_pane",
            // hook_session - ID of session where hook was run, if any
            #[cfg(feature = "tmux_2_4")]
            Self::HookSession => "hook_session",
            // hook_session_name - Name of session where hook was run, if any
            #[cfg(feature = "tmux_2_4")]
            Self::HookSessionName => "hook_session_name",
            // hook_window - ID of window where hook was run, if any
            #[cfg(feature = "tmux_2_4")]
            Self::HookWindow => "hook_window",
            // hook_window_name - Name of window where hook was run, if any
            #[cfg(feature = "tmux_2_4")]
            Self::HookWindowName => "hook_window_name",

            // host
            // host - Hostname of local host
            #[cfg(feature = "tmux_1_6")]
            Self::Host => "host",
            // host_short - #h Hostname of local host (no domain name)
            #[cfg(feature = "tmux_1_9")]
            Self::HostShort => "host_short",

            // insert_flag - Pane insert flag
            #[cfg(feature = "tmux_1_8")]
            Self::InsertFlag => "insert_flag",
            // keypad_cursor_flag - Pane keypad cursor flag
            #[cfg(feature = "tmux_1_8")]
            Self::KeypadCursorFlag => "keypad_cursor_flag",
            // keypad_flag - Pane keypad flag
            #[cfg(feature = "tmux_1_8")]
            Self::KeypadFlag => "keypad_flag",

            // line - Line number in the list
            #[cfg(feature = "tmux_1_6")]
            Self::Line => "line",

            // mouse_all_flag - Pane mouse all flag
            #[cfg(feature = "tmux_3_0")]
            Self::MouseAllFlag => "mouse_all_flag",
            // mouse_any_flag - Pane mouse any flag
            #[cfg(feature = "tmux_1_8")]
            Self::MouseAnyFlag => "mouse_any_flag",
            // mouse_button_flag - Pane mouse button flag
            #[cfg(feature = "tmux_1_8")]
            Self::MouseButtonFlag => "mouse_button_flag",
            // mouse_line - Line under mouse, if any
            #[cfg(feature = "tmux_3_0")]
            Self::MouseLine => "mouse_line",
            // sgr_flag - Pane mouse SGR flag
            #[cfg(feature = "tmux_3_0")]
            Self::MouseSgrFlag => "sgr_flag",
            // mouse_standard_flag - Pane mouse standard flag
            #[cfg(feature = "tmux_1_8")]
            Self::MouseStandardFlag => "mouse_standard_flag",
            // mouse_utf8_flag - Pane mouse UTF-8 flag
            #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_2"), feature = "tmux_3_0"))]
            Self::MouseUtf8Flag => "mouse_utf8_flag",
            // mouse_all_flag - Pane mouse all flag
            #[cfg(feature = "tmux_2_4")]
            Self::MouseAllFlag => "mouse_all_flag",
            // mouse_word - Word under mouse, if any
            #[cfg(feature = "tmux_3_0")]
            Self::MouseWord => "mouse_word",
            // mouse_x - Mouse X position, if any
            #[cfg(feature = "tmux_3_0")]
            Self::MouseX => "mouse_x",
            // mouse_y - Mouse Y position, if any
            #[cfg(feature = "tmux_3_0")]
            Self::MouseY => "mouse_y",
            // origin_flag - Pane origin flag
            #[cfg(feature = "tmux_3_0")]
            Self::OriginFlag => "origin_flag",

            // pane
            // pane_active - 1 if active pane
            #[cfg(feature = "tmux_1_6")]
            Self::PaneActive => "pane_active",
            // pane_at_bottom - 1 if pane is at the bottom of window
            #[cfg(feature = "tmux_2_6")]
            Self::PaneAtBottom => "pane_at_bottom",
            // pane_at_left - 1 if pane is at the left of window
            #[cfg(feature = "tmux_2_6")]
            Self::PaneAtLeft => "pane_at_left",
            // pane_at_richt - 1 if pane is at the right of window
            #[cfg(feature = "tmux_2_6")]
            Self::PaneAtRight => "pane_at_right",
            // pane_at_top - 1 if pane is at the top of window
            #[cfg(feature = "tmux_2_6")]
            Self::PaneAtTop => "pane_at_top",
            // pane_bottom - Bottom of pane
            #[cfg(feature = "tmux_2_0")]
            Self::PaneBottom => "pane_bottom",
            // pane_current_command - Current command if available
            #[cfg(feature = "tmux_1_8")]
            Self::PaneCurrentCommand => "pane_current_command",
            // pane_current_path - Current path if available
            #[cfg(feature = "tmux_1_7")]
            Self::PaneCurrentPath => "pane_current_path",
            // pane_dead - 1 if pane is dead
            #[cfg(feature = "tmux_1_6")]
            Self::PaneDead => "pane_dead",
            // pane_dead_status - Exit status of process in dead pane
            #[cfg(feature = "tmux_2_0")]
            Self::PaneDeadStatus => "pane_dead_status",
            // pane_format - 1 if format is for a pane
            #[cfg(feature = "tmux_2_6")]
            Self::PaneFormat => "pane_format",
            // pane_height - Height of pane
            #[cfg(feature = "tmux_1_6")]
            Self::PaneHeight => "pane_height",
            // pane_id - #D Unique pane ID
            #[cfg(feature = "tmux_1_6")]
            Self::PaneId => "pane_id",
            // pane_in_mode - 1 if pane is in a mode
            #[cfg(feature = "tmux_1_8")]
            Self::PaneInMode => "pane_in_mode",
            // pane_index - #P Index of pane
            #[cfg(feature = "tmux_1_7")]
            Self::PaneIndex => "pane_index",
            // pane_input_off - 1 if input to pane is disabled
            #[cfg(feature = "tmux_2_0")]
            Self::PaneInputOff => "pane_input_off",
            // pane_left - Left of pane
            #[cfg(feature = "tmux_2_0")]
            Self::PaneLeft => "pane_left",
            // pane_marked - 1 if this is the marked pane
            #[cfg(feature = "tmux_3_0")]
            Self::PaneMarked => "pane_marked",
            // pane_marked_set - 1 if a marked pane is set
            #[cfg(feature = "tmux_3_0")]
            Self::PaneMarkedSet => "pane_marked_set",
            // pane_mode - Name of pane mode, if any
            #[cfg(feature = "tmux_2_5")]
            Self::PaneMode => "pane_mode",
            // pane_path - #T Path of pane (can be set by application)
            #[cfg(feature = "tmux_3_1")]
            Self::PanePath => "pane_path",
            // pane_pid - PID of first process in pane
            #[cfg(feature = "tmux_1_6")]
            Self::PanePid => "pane_pid",
            // pane_pipe - 1 if pane is being piped
            #[cfg(feature = "tmux_2_6")]
            Self::PanePipe => "pane_pipe",
            // pane_right - Right of pane
            #[cfg(feature = "tmux_2_0")]
            Self::PaneRight => "pane_right",
            // pane_search_string - Last search Option<String> in copy mode
            #[cfg(feature = "tmux_2_5")]
            Self::PaneSearchString => "pane_search_string",
            // pane_start_command - Command pane started with
            #[cfg(feature = "tmux_1_6")]
            Self::PaneStartCommand => "pane_start_command",
            // pane_start_path - Path pane started with
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_0")))]
            Self::PaneStartPath => "pane_start_path",
            // pane_synchronized - 1 if pane is synchronized
            #[cfg(feature = "tmux_1_9")]
            Self::PaneSynchronized => "pane_synchronized",
            // pane_tabs - Pane tab positions
            #[cfg(feature = "tmux_1_8")]
            Self::PaneTabs => "pane_tabspane_tabs",
            // pane_title - #T Title of pane (can be set by application)
            #[cfg(feature = "tmux_1_6")]
            Self::PaneTitle => "pane_titlepane_title",
            // pane_top - Top of pane
            #[cfg(feature = "tmux_2_0")]
            Self::PaneTop => "pane_top",
            // pane_tty - Pseudo terminal of pane
            #[cfg(feature = "tmux_1_6")]
            Self::PaneTty => "pane_tty",
            // pane_width - Width of pane
            #[cfg(feature = "tmux_1_6")]
            Self::PaneWidth => "pane_width",

            // saved_cursor_x - Saved cursor X in pane
            #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
            Self::SavedCursorX => "saved_cursor_x",
            // saved_cursor_y - Saved cursor Y in pane
            #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
            Self::SavedCursorY => "saved_cursor_y",

            // pid - Server PID
            #[cfg(feature = "tmux_2_1")]
            Self::Pid => "pid",
            // rectangle_toggle - 1 if rectangle selection is activated
            #[cfg(feature = "tmux_2_7")]
            Self::RectangleToggle => "rectangle_toggle",

            // scroll_position - Scroll position in copy mode
            #[cfg(feature = "tmux_2_2")]
            Self::ScrollPosition => "scroll_position",
            // scroll_region_lower - Bottom of scroll region in pane
            #[cfg(feature = "tmux_1_8")]
            Self::ScrollRegionLower => "scroll_region_lower",
            // scroll_region_upper - Top of scroll region in pane
            #[cfg(feature = "tmux_1_8")]
            Self::ScrollRegionUpper => "scroll_region_upper",

            // selection_active - 1 if selection started and changes with the curso
            #[cfg(feature = "tmux_3_1")]
            Self::SelectionActive => "selection_active",
            // selection_end_x - X position of the end of the selection
            #[cfg(feature = "tmux_3_1")]
            Self::SelectionEndX => "selection_end_x",
            // selection_end_y - Y position of the end of the selection
            #[cfg(feature = "tmux_3_1")]
            Self::SelectionEndY => "selection_end_y",
            // selection_present - 1 if selection started in copy mode
            #[cfg(feature = "tmux_2_6")]
            Self::SelectionPresent => "selection_present",
            // selection_start_x - X position of the start of the selection
            #[cfg(feature = "tmux_3_1")]
            Self::SelectionStartX => "selection_start_x",
            // selection_start_y - Y position of the start of the selection
            #[cfg(feature = "tmux_3_1")]
            Self::SelectionStartY => "selection_start_y",

            // Session
            // session_activity - Time of session last activity
            #[cfg(feature = "tmux_2_1")]
            Self::SessionActivity => "session_activity",
            // session_activity_string - Option<String> time of session last activity
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
            Self::SessionActivityString => "session_activity_string",
            // session_alerts - List of window indexes with alerts
            #[cfg(feature = "tmux_2_1")]
            Self::SessionAlerts => "session_alerts",
            // session_attached - Number of clients session is attached to
            #[cfg(feature = "tmux_1_6")]
            Self::SessionAttached => "session_attached",
            // session_attached_list - List of clients session is attached to
            #[cfg(feature = "tmux_3_1")]
            Self::SessionAttachedList => "session_attached_list",
            // session_created - Time session created
            #[cfg(feature = "tmux_1_6")]
            Self::SessionCreated => "session_created",
            // session_created_string - Option<String> time session created
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
            Self::SessionCreatedString => "session_created_string",
            // session_format - 1 if format is for a session (not assuming the current)
            #[cfg(feature = "tmux_2_6")]
            Self::SessionFormat => "session_format",
            // session_group - Name of session group
            #[cfg(feature = "tmux_1_6")]
            Self::SessionGroup => "session_group",
            // session_group_attached - Number of clients sessions in group are attached >
            #[cfg(feature = "tmux_3_1")]
            Self::SessionGroupAttached => "session_group_attached",
            // session_group_attached_list - List of clients sessions in group are attached to
            #[cfg(feature = "tmux_3_1")]
            Self::SessionGroupAttachedList => "session_group_attached_list",
            // session_group_list - List of sessions in group
            #[cfg(feature = "tmux_2_7")]
            Self::SessionGroupList => "session_group_list",
            // session_group_many_attached - 1 if multiple clients attached to sessions in gro
            #[cfg(feature = "tmux_3_1")]
            Self::SessionGroupManyAttached => "session_group_many_attached",
            // session_size - Size of session group
            #[cfg(feature = "tmux_2_7")]
            Self::SessionGroupSize => "session_size",
            // session_grouped - 1 if session in a group
            #[cfg(feature = "tmux_1_6")]
            Self::SessionGrouped => "session_grouped",
            // session_height - Height of session
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
            Self::SessionHeight => "session_height",
            // session_width - Width of session
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
            Self::SessionWidth => "session_width",
            // session_id - Unique session ID
            #[cfg(feature = "tmux_1_8")]
            Self::SessionId => "session_id",
            // session_last_attached - Time session last attached
            #[cfg(feature = "tmux_2_1")]
            Self::SessionLastAttached => "session_last_attached",
            // session_last_attached_string - Option<String> time session last attached
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
            Self::SessionLastAttachedString => "session_last_attached_string",
            // session_many_attached - 1 if multiple clients attached
            #[cfg(feature = "tmux_2_0")]
            Self::SessionManyAttached => "session_many_attached",
            // session_name - #S Name of session
            #[cfg(feature = "tmux_1_6")]
            Self::SessionName => "session_name",
            // session_stack - Window indexes in most recent order
            #[cfg(feature = "tmux_2_5")]
            Self::SessionStack => "session_stack",
            // session_windows - Number of windows in session
            #[cfg(feature = "tmux_1_6")]
            Self::SessionWindows => "session_windows",

            // socket_path - Server socket path
            #[cfg(feature = "tmux_2_2")]
            Self::SocketPath => "socket_path",
            // start_time - Server start time
            #[cfg(feature = "tmux_2_2")]
            Self::StartTime => "start_time",

            // version - Server version
            #[cfg(feature = "tmux_2_4")]
            Self::Version => "version",

            // Window
            //
            // window_active - 1 if window active
            #[cfg(feature = "tmux_1_6")]
            Self::WindowActive => "window_active",
            // window_active_clients - Number of clients viewing this window
            #[cfg(feature = "tmux_3_1")]
            Self::WindowActiveClients => "window_active_clients",
            // window_active_clients_list - List of clients viewing this window
            #[cfg(feature = "tmux_3_1")]
            Self::WindowActiveClientsList => "window_active_clients_list",
            // window_active_sessions - Number of sessions on which this window is active
            #[cfg(feature = "tmux_3_1")]
            Self::WindowActiveSessions => "window_active_sessions",
            // window_active_sessions_list - List of sessions on which this window is active
            #[cfg(feature = "tmux_3_1")]
            Self::WindowActiveSessionsList => "window_active_sessions_list",
            // window_activity - Time of window last activity
            #[cfg(feature = "tmux_2_1")]
            Self::WindowActivity => "window_activity",
            // window_activity_string - String time of window last activity
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
            Self::WindowActivityString => "window_activity_string",
            // window_activity_flag - 1 if window has activity
            #[cfg(any(
                all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
                feature = "tmux_2_3"
            ))]
            Self::WindowActivityFlag => "window_activity_flag",
            // window_bell_flag - 1 if window has bell
            #[cfg(feature = "tmux_1_9")]
            Self::WindowBellFlag => "window_bell_flag",
            // window_content_flag - 1 if window has content alert
            #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
            Self::WindowContentFlag => "window_content_flag",
            // window_bigger - 1 if window is larger than client
            #[cfg(feature = "tmux_2_9")]
            Self::WindowBigger => "window_bigger",
            // window_cell_height - Height of each cell in pixels
            #[cfg(feature = "tmux_3_1")]
            Self::WindowCellHeight => "window_cell_height",
            // window_cell_width - Width of each cell in pixels
            #[cfg(feature = "tmux_3_1")]
            Self::WindowCellWidth => "window_cell_width",
            // window_end_flag - 1 if window has the highest index
            #[cfg(feature = "tmux_2_9")]
            Self::WindowEndFlag => "window_end_flag",
            // window_find_matches - Matched data from the find-window command if available
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
            Self::WindowFindMatches => "window_find_matches",
            // window_flags - #F Window flags
            #[cfg(feature = "tmux_1_6")]
            Self::WindowFlags => "window_flags",
            // window_raw_flags - Window flags with nothing escaped
            #[cfg(feature = "tmux_3_2")]
            Self::WindowRawFlags => "window_raw_flags",
            // window_format - 1 if format is for a window
            #[cfg(feature = "tmux_2_6")]
            Self::WindowFormat => "window_format",
            // window_height - Height of window
            #[cfg(feature = "tmux_1_6")]
            Self::WindowHeight => "window_height",
            // window_id - Unique window ID
            #[cfg(feature = "tmux_1_7")]
            Self::WindowId => "window_id",
            // window_index - #I Index of window
            #[cfg(feature = "tmux_1_6")]
            Self::WindowIndex => "window_index",
            // window_last_flag - 1 if window is the last used
            #[cfg(feature = "tmux_2_0")]
            Self::WindowLastFlag => "window_last_flag",
            // window_layout - Window layout description, ignoring zoomed window panes
            #[cfg(feature = "tmux_1_6")]
            Self::WindowLayout => "window_layout",
            // window_linked - 1 if window is linked across sessions
            #[cfg(feature = "tmux_2_1")]
            Self::WindowLinked => "window_linked",
            // window_linked_sessions - Number of sessions this window is linked to
            #[cfg(feature = "tmux_3_1")]
            Self::WindowLinkedSessions => "window_linked_sessions",
            // window_linked_sessions_list - List of sessions this window is linked to
            #[cfg(feature = "tmux_3_1")]
            Self::WindowLinkedSessionsList => "window_linked_sessions_list",
            // window_marked_flag - 1 if window contains the marked pane
            #[cfg(feature = "tmux_3_1")]
            Self::WindowMarkedFlag => "window_marked_flag",
            // window_name - #W Name of window
            #[cfg(feature = "tmux_1_6")]
            Self::WindowName => "window_name",
            // window_offset_x - X offset into window if larger than client
            #[cfg(feature = "tmux_2_9")]
            Self::WindowOffsetX => "window_offset_x",
            // window_offset_y - Y offset into window if larger than client
            #[cfg(feature = "tmux_2_9")]
            Self::WindowOffsetY => "window_offset_y",
            // window_panes - Number of panes in window
            #[cfg(feature = "tmux_1_7")]
            Self::WindowPanes => "window_panes",
            // window_silence_flag - 1 if window has silence alert
            #[cfg(feature = "tmux_1_9")]
            Self::WindowSilenceFlag => "window_silence_flag",
            // window_stack_index - Index in session most recent stack
            #[cfg(feature = "tmux_2_5")]
            Self::WindowStackIndex => "window_stack_index",
            // window_start_flag - 1 if window has the lowest index
            #[cfg(feature = "tmux_2_9")]
            Self::WindowStartFlag => "window_start_flag",
            // window_visible_layout - Window layout description, respecting zoomed window panes
            #[cfg(feature = "tmux_2_2")]
            Self::WindowVisibleLayout => "window_visible_layout",
            // window_width - Width of window
            #[cfg(feature = "tmux_1_6")]
            Self::WindowWidth => "window_width",
            // window_zoomed_flag - 1 if window is zoomed
            #[cfg(feature = "tmux_2_0")]
            Self::WindowZoomedFlag => "window_zoomed_flag",

            // wrap_flag - Pane wrap flag
            #[cfg(feature = "tmux_1_8")]
            Self::WrapFlag => "wrap_flag",
        };

        write!(f, "#{{{}}}", output)
    }
}
