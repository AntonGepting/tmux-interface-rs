use crate::format::variable::Variable;
use crate::Layout;
use crate::PaneTabs;
#[cfg(feature = "tmux_2_5")]
use crate::SessionStack;
use crate::WindowFlag;

#[derive(Debug)]
pub enum VariableOutput {
    // not recognized part?
    Error,

    /// alternate_on - if pane is in alternate screen
    #[cfg(feature = "tmux_1_8")]
    AlternateOn(Option<usize>),
    /// alternate_saved_x - Saved cursor X in alternate screen
    #[cfg(feature = "tmux_1_8")]
    AlternateSavedX(Option<usize>),
    /// alternate_saved_y - Saved cursor Y in alternate screen
    #[cfg(feature = "tmux_1_8")]
    AlternateSavedY(Option<usize>),

    // Buffer
    /// buffer_created - Time buffer created
    #[cfg(feature = "tmux_2_6")]
    BufferCreated(Option<u128>),
    /// buffer_name - Name of buffer
    #[cfg(feature = "tmux_2_3")]
    BufferName(String),
    /// buffer_sample - First 50 characters from the specified buffer
    #[cfg(feature = "tmux_1_7")]
    BufferSample(String),
    /// buffer_size - Size of the specified buffer in bytes
    #[cfg(feature = "tmux_1_7")]
    BufferSize(Option<usize>),

    // Client
    /// client_activity - Integer time client last had activity
    #[cfg(feature = "tmux_1_6")]
    ClientActivity(Option<u128>),
    /// client_cell_height - Height of each client cell in pixels
    #[cfg(feature = "tmux_3_1")]
    ClientCellHeight(Option<usize>),
    /// client_cell_width - Width of each client cell in pixels
    #[cfg(feature = "tmux_3_1")]
    ClientCellWidth(Option<usize>),
    /// client_activity_string - String time client last had activity
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    ClientActivityString(String),
    /// client_created - Integer time client created
    #[cfg(feature = "tmux_1_6")]
    ClientCreated(Option<u128>),
    /// client_created_string - String time client created
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    ClientCreatedString(String),
    /// client_control_mode - 1 if client is in control mode
    #[cfg(feature = "tmux_2_1")]
    ClientControlMode(Option<bool>),
    /// client_discarded - Bytes discarded when client behind
    #[cfg(feature = "tmux_2_1")]
    ClientDiscarded(String),
    /// client_cwd - Working directory of client
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    ClientCwd(String),
    /// client_height - Height of client
    #[cfg(feature = "tmux_1_6")]
    ClientHeight(Option<usize>),
    /// client_key_table - Current key table
    #[cfg(feature = "tmux_2_2")]
    ClientKeyTable(String),
    /// client_last_session - Name of the client's last session
    #[cfg(feature = "tmux_1_8")]
    ClientLastSession(String),
    /// client_name - Name of client
    #[cfg(feature = "tmux_2_4")]
    ClientName(String),
    /// client_pid - PID of client process
    #[cfg(feature = "tmux_2_1")]
    ClientPid(Option<bool>),
    /// client_prefix - 1 if prefix key has been pressed
    #[cfg(feature = "tmux_1_8")]
    ClientPrefix(Option<bool>),
    /// client_readonly - 1 if client is readonly
    #[cfg(feature = "tmux_1_6")]
    ClientReadonly(Option<bool>),
    /// client_session - Name of the client's session
    #[cfg(feature = "tmux_1_8")]
    ClientSession(String),
    /// client_termname - Terminal name of client
    #[cfg(feature = "tmux_1_6")]
    ClientTermname(String),
    /// client_termtype - Terminal type of client
    #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_1")))]
    ClientTermtype(String),
    /// client_tty - Pseudo terminal of client
    #[cfg(feature = "tmux_1_6")]
    ClientTty(String),
    /// client_utf8 - 1 if client supports UTF-8
    #[cfg(feature = "tmux_1_6")]
    ClientUtf8(Option<bool>),
    /// client_width - Width of client
    #[cfg(feature = "tmux_1_6")]
    ClientWidth(Option<usize>),
    /// client_written - Bytes written to client
    #[cfg(feature = "tmux_2_4")]
    ClientWritten(Option<usize>),

    // Command
    /// command_hooked - Name of command hooked, if any
    #[cfg(feature = "tmux_2_3")]
    CommandHooked(String),
    /// command_name - Name of command in use, if any
    #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_2_4")))]
    CommandName(String),
    /// command - Name of command in use, if any
    #[cfg(feature = "tmux_2_4")]
    Command(String),
    /// command_list_name - Command name if listing commands
    #[cfg(feature = "tmux_2_3")]
    CommandListName(String),
    /// command_list_alias - Command alias if listing commands
    #[cfg(feature = "tmux_2_3")]
    CommandListAlias(String),
    /// command_list_usage - Command usage if listing commands
    #[cfg(feature = "tmux_2_3")]
    CommandListUsage(String),

    // Cursor
    /// cursor_flag - Pane cursor flag
    #[cfg(feature = "tmux_1_8")]
    CursorFlag(String),
    /// cursor_character - Character at cursor in pane
    #[cfg(feature = "tmux_2_9")]
    CursorCharacter(String),
    /// cursor_x - Cursor X position in pane
    #[cfg(feature = "tmux_1_8")]
    CursorX(Option<usize>),
    /// cursor_y - Cursor Y position in pane
    #[cfg(feature = "tmux_1_8")]
    CursorY(Option<usize>),

    /// copy_cursor_line - Line the cursor is on in copy mode
    #[cfg(feature = "tmux_3_1")]
    CopyCursorLine(String),
    /// copy_cursor_word - Word under cursor in copy mode
    #[cfg(feature = "tmux_3_1")]
    CopyCursorWord(String),
    /// copy_cursor_x - Cursor X position in copy mode
    #[cfg(feature = "tmux_3_1")]
    CopyCursorX(String),
    /// copy_cursor_y - Cursor Y position in copy mode
    #[cfg(feature = "tmux_3_1")]
    CopyCursorY(String),

    // history
    /// history_bytes             Number of bytes in window history
    #[cfg(feature = "tmux_1_7")]
    HistotyBytes(Option<usize>),
    /// history_limit             Maximum window history lines
    #[cfg(feature = "tmux_1_7")]
    HistotyLimit(Option<usize>),
    /// history_size              Size of history in bytes
    #[cfg(feature = "tmux_1_7")]
    HistorySize(Option<usize>),

    // hook
    /// hook - Name of running hook, if any
    #[cfg(feature = "tmux_2_4")]
    Hook(String),
    /// hook_pane - ID of pane where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    HookPane(Option<usize>),
    /// hook_session - ID of session where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    HookSession(Option<usize>),
    /// hook_session_name - Name of session where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    HookSessionName(String),
    /// hook_window - ID of window where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    HookWindow(Option<usize>),
    /// hook_window_name - Name of window where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    HookWindowName(String),

    // host
    /// host - Hostname of local host
    #[cfg(feature = "tmux_1_6")]
    Host(String),
    /// host_short - #h Hostname of local host (no domain name)
    #[cfg(feature = "tmux_1_9")]
    HostShort(String),

    /// insert_flag - Pane insert flag
    #[cfg(feature = "tmux_1_8")]
    InsertFlag(String),
    /// keypad_cursor_flag - Pane keypad cursor flag
    #[cfg(feature = "tmux_1_8")]
    KeypadCursorFlag(String),
    /// keypad_flag - Pane keypad flag
    #[cfg(feature = "tmux_1_8")]
    KeypadFlag(String),

    /// line - Line number in the list
    #[cfg(feature = "tmux_1_6")]
    Line(Option<usize>),

    /// mouse_all_flag - Pane mouse all flag
    #[cfg(feature = "tmux_3_0")]
    MouseAllFlag(String),
    /// mouse_any_flag - Pane mouse any flag
    #[cfg(feature = "tmux_1_8")]
    MouseAnyFlag(String),
    /// mouse_button_flag - Pane mouse button flag
    #[cfg(feature = "tmux_1_8")]
    MouseButtonFlag(String),
    /// mouse_line - Line under mouse, if any
    #[cfg(feature = "tmux_3_0")]
    MouseLine(String),
    /// sgr_flag - Pane mouse SGR flag
    #[cfg(feature = "tmux_3_0")]
    MouseSgrFlag(String),
    /// mouse_standard_flag - Pane mouse standard flag
    #[cfg(feature = "tmux_1_8")]
    MouseStandardFlag(String),
    /// mouse_utf8_flag - Pane mouse UTF-8 flag
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_2"), feature = "tmux_3_0"))]
    MouseUtf8Flag(String),
    /// mouse_all_flag - Pane mouse all flag
    #[cfg(feature = "tmux_2_4")]
    MouseAllFlag(String),
    /// mouse_word - Word under mouse, if any
    #[cfg(feature = "tmux_3_0")]
    MouseWord(String),
    /// mouse_x - Mouse X position, if any
    #[cfg(feature = "tmux_3_0")]
    MouseX(Option<usize>),
    /// mouse_y - Mouse Y position, if any
    #[cfg(feature = "tmux_3_0")]
    MouseY(Option<usize>),
    /// origin_flag - Pane origin flag
    #[cfg(feature = "tmux_3_0")]
    OriginFlag(String),

    // pane
    /// pane_active - 1 if active pane
    #[cfg(feature = "tmux_1_6")]
    PaneActive(Option<bool>),
    /// pane_at_bottom - 1 if pane is at the bottom of window
    #[cfg(feature = "tmux_2_6")]
    PaneAtBottom(Option<bool>),
    /// pane_at_left - 1 if pane is at the left of window
    #[cfg(feature = "tmux_2_6")]
    PaneAtLeft(Option<bool>),
    /// pane_at_right - 1 if pane is at the right of window
    #[cfg(feature = "tmux_2_6")]
    PaneAtRight(Option<bool>),
    /// pane_at_top - 1 if pane is at the top of window
    #[cfg(feature = "tmux_2_6")]
    PaneAtTop(Option<bool>),
    /// pane_bottom - Bottom of pane
    #[cfg(feature = "tmux_2_0")]
    PaneBottom(Option<usize>),
    /// pane_current_command - Current command if available
    #[cfg(feature = "tmux_1_8")]
    PaneCurrentCommand(String),
    /// pane_current_path - Current path if available
    #[cfg(feature = "tmux_1_7")]
    PaneCurrentPath(String),
    /// pane_dead - 1 if pane is dead
    #[cfg(feature = "tmux_1_6")]
    PaneDead(Option<bool>),
    /// pane_dead_status - Exit status of process in dead pane
    #[cfg(feature = "tmux_2_0")]
    PaneDeadStatus(Option<usize>),
    /// pane_format - 1 if format is for a pane
    #[cfg(feature = "tmux_2_6")]
    PaneFormat(Option<bool>),
    /// pane_height - Height of pane
    #[cfg(feature = "tmux_1_6")]
    PaneHeight(Option<usize>),
    /// pane_id - #D Unique pane ID
    #[cfg(feature = "tmux_1_6")]
    PaneId(Option<usize>),
    /// pane_in_mode - 1 if pane is in a mode
    #[cfg(feature = "tmux_1_8")]
    PaneInMode(Option<bool>),
    /// pane_index - #P Index of pane
    #[cfg(feature = "tmux_1_7")]
    PaneIndex(Option<usize>),
    /// pane_input_off - 1 if input to pane is disabled
    #[cfg(feature = "tmux_2_0")]
    PaneInputOff(Option<bool>),
    /// pane_left - Left of pane
    #[cfg(feature = "tmux_2_0")]
    PaneLeft(Option<usize>),
    /// pane_marked - 1 if this is the marked pane
    #[cfg(feature = "tmux_3_0")]
    PaneMarked(Option<bool>),
    /// pane_marked_set - 1 if a marked pane is set
    #[cfg(feature = "tmux_3_0")]
    PaneMarkedSet(Option<bool>),
    /// pane_mode - Name of pane mode, if any
    #[cfg(feature = "tmux_2_5")]
    PaneMode(Option<usize>),
    /// pane_path - #T Path of pane (can be set by application)
    #[cfg(feature = "tmux_3_1")]
    PanePath(String),
    /// pane_pid - PID of first process in pane
    #[cfg(feature = "tmux_1_6")]
    PanePid(Option<usize>),
    /// pane_pipe - 1 if pane is being piped
    #[cfg(feature = "tmux_2_6")]
    PanePipe(Option<bool>),
    /// pane_right - Right of pane
    #[cfg(feature = "tmux_2_0")]
    PaneRight(Option<usize>),
    /// Last search string in copy mode
    #[cfg(feature = "tmux_2_5")]
    PaneSearchString(Option<usize>),
    /// pane_start_command - Command pane started with
    #[cfg(feature = "tmux_1_6")]
    PaneStartCommand(Option<usize>),
    /// pane_start_path - Path pane started with
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_0")))]
    PaneStartPath(Option<usize>),
    /// pane_synchronized - 1 if pane is synchronized
    #[cfg(feature = "tmux_1_9")]
    PaneSynchronized(Option<bool>),
    /// pane_tabs - Pane tab positions
    #[cfg(feature = "tmux_1_8")]
    PaneTabs(Option<PaneTabs>),
    /// pane_title - #T Title of pane (can be set by application)
    #[cfg(feature = "tmux_1_6")]
    PaneTitle(String),
    /// pane_top - Top of pane
    #[cfg(feature = "tmux_2_0")]
    PaneTop(Option<usize>),
    /// pane_tty - Pseudo terminal of pane
    #[cfg(feature = "tmux_1_6")]
    PaneTty(String),
    /// pane_width - Width of pane
    #[cfg(feature = "tmux_1_6")]
    PaneWidth(Option<usize>),

    /// saved_cursor_x - Saved cursor X in pane
    #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
    SavedCursorX(Option<usize>),
    /// saved_cursor_y - Saved cursor Y in pane
    #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
    SavedCursorY(Option<usize>),

    /// pid - Server PID
    #[cfg(feature = "tmux_2_1")]
    Pid(Option<usize>),
    /// rectangle_toggle - 1 if rectangle selection is activated
    #[cfg(feature = "tmux_2_7")]
    RectangleToggle(Option<bool>),

    /// scroll_position - Scroll position in copy mode
    #[cfg(feature = "tmux_2_2")]
    ScrollPosition(Option<usize>),
    /// scroll_region_lower - Bottom of scroll region in pane
    #[cfg(feature = "tmux_1_8")]
    ScrollRegionLower(Option<usize>),
    /// scroll_region_upper - Top of scroll region in pane
    #[cfg(feature = "tmux_1_8")]
    ScrollRegionUpper(Option<usize>),

    /// selection_active - 1 if selection started and changes with the curso
    #[cfg(feature = "tmux_3_1")]
    SelectionActive(Option<bool>),
    /// selection_end_x - X position of the end of the selection
    #[cfg(feature = "tmux_3_1")]
    SelectionEndX(Option<usize>),
    /// selection_end_y - Y position of the end of the selection
    #[cfg(feature = "tmux_3_1")]
    SelectionEndY(Option<usize>),
    /// selection_present - 1 if selection started in copy mode
    #[cfg(feature = "tmux_2_6")]
    SelectionPresent(Option<bool>),
    /// selection_start_x - X position of the start of the selection
    #[cfg(feature = "tmux_3_1")]
    SelectionStartX(Option<usize>),
    /// selection_start_y - Y position of the start of the selection
    #[cfg(feature = "tmux_3_1")]
    SelectionStartY(Option<usize>),

    // Session
    /// session_activity - Time of session last activity
    #[cfg(feature = "tmux_2_1")]
    SessionActivity(Option<u128>),
    /// session_activity_string - String time of session last activity
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    SessionActivityString(String),
    /// session_alerts - List of window indexes with alerts
    #[cfg(feature = "tmux_2_1")]
    SessionAlerts(String),
    /// session_attached - Number of clients session is attached to
    #[cfg(feature = "tmux_1_6")]
    SessionAttached(Option<usize>),
    /// session_attached_list - List of clients session is attached to
    #[cfg(feature = "tmux_3_1")]
    SessionAttachedList(Option<usize>),
    /// session_created - Time session created
    #[cfg(feature = "tmux_1_6")]
    SessionCreated(Option<u128>),
    /// session_created_string - String time session created
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    SessionCreatedString(String),
    /// session_format - 1 if format is for a session (not assuming the current)
    #[cfg(feature = "tmux_2_6")]
    SessionFormat(Option<bool>),
    /// session_group - Name of session group
    #[cfg(feature = "tmux_1_6")]
    SessionGroup(String),
    /// session_group_attached - Number of clients sessions in group are attached >
    #[cfg(feature = "tmux_3_1")]
    SessionGroupAttached(Option<usize>),
    /// session_group_attached_list - List of clients sessions in group are attached to
    #[cfg(feature = "tmux_3_1")]
    SessionGroupAttachedList(String),
    /// session_group_list - List of sessions in group
    #[cfg(feature = "tmux_2_7")]
    SessionGroupList(String),
    /// session_group_many_attached - 1 if multiple clients attached to sessions in gro
    #[cfg(feature = "tmux_3_1")]
    SessionGroupManyAttached(Option<bool>),
    /// session_size - Size of session group
    #[cfg(feature = "tmux_2_7")]
    SessionGroupSize(String),
    /// session_grouped - 1 if session in a group
    #[cfg(feature = "tmux_1_6")]
    SessionGrouped(Option<bool>),
    /// session_height - Height of session
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    SessionHeight(Option<usize>),
    /// session_width - Width of session
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    SessionWidth(Option<usize>),
    /// session_id - Unique session ID
    #[cfg(feature = "tmux_1_8")]
    SessionId(Option<usize>),
    /// session_last_attached - Time session last attached
    #[cfg(feature = "tmux_2_1")]
    SessionLastAttached(Option<u128>),
    /// session_last_attached_string - String time session last attached
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    SessionLastAttachedString(String),
    /// session_many_attached - 1 if multiple clients attached
    #[cfg(feature = "tmux_2_0")]
    SessionManyAttached(Option<bool>),
    /// session_name - #S Name of session
    #[cfg(feature = "tmux_1_6")]
    SessionName(String),
    /// session_stack - Window indexes in most recent order
    #[cfg(feature = "tmux_2_5")]
    SessionStack(Option<SessionStack>),
    /// session_windows - Number of windows in session
    #[cfg(feature = "tmux_1_6")]
    SessionWindows(Option<usize>),

    /// socket_path - Server socket path
    #[cfg(feature = "tmux_2_2")]
    SocketPath(String),
    /// start_time - Server start time
    #[cfg(feature = "tmux_2_2")]
    StartTime(Option<u128>),

    /// version - Server version
    #[cfg(feature = "tmux_2_4")]
    Version(String),

    // Window
    //
    /// window_active - 1 if window active
    #[cfg(feature = "tmux_1_6")]
    WindowActive(Option<bool>),
    /// window_active_clients - Number of clients viewing this window
    #[cfg(feature = "tmux_3_1")]
    WindowActiveClients(Option<usize>),
    /// window_active_clients_list - List of clients viewing this window
    #[cfg(feature = "tmux_3_1")]
    WindowActiveClientsList(String),
    /// window_active_sessions - Number of sessions on which this window is active
    #[cfg(feature = "tmux_3_1")]
    WindowActiveSessions(Option<usize>),
    /// window_active_sessions_list - List of sessions on which this window is active
    #[cfg(feature = "tmux_3_1")]
    WindowActiveSessionsList(String),
    /// window_activity - Time of window last activity
    #[cfg(feature = "tmux_2_1")]
    WindowActivity(Option<usize>),
    /// session_activity_string - String time of session last activity
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    SessionActivityString(String),
    /// window_activity_flag - 1 if window has activity
    #[cfg(any(
        all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
        feature = "tmux_2_3"
    ))]
    WindowActivityFlag(Option<bool>),
    /// window_bell_flag - 1 if window has bell
    #[cfg(feature = "tmux_1_9")]
    WindowBellFlag(Option<bool>),
    /// window_content_flag - 1 if window has content alert
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    WindowContentflag(Option<bool>),
    /// window_bigger - 1 if window is larger than client
    #[cfg(feature = "tmux_2_9")]
    WindowBigger(Option<bool>),
    /// window_cell_height - Height of each cell in pixels
    #[cfg(feature = "tmux_3_1")]
    WindowCellHeight(Option<usize>),
    /// window_cell_width - Width of each cell in pixels
    #[cfg(feature = "tmux_3_1")]
    WindowCellWidth(Option<usize>),
    /// window_end_flag - 1 if window has the highest index
    #[cfg(feature = "tmux_2_9")]
    WindowEndFlag(Option<bool>),
    /// window_find_matches - Matched data from the find-window command if available
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    WindowFindMatches(String),
    /// window_flags - #F Window flags
    #[cfg(feature = "tmux_1_6")]
    WindowFlags(Option<WindowFlag>),
    /// window_format - 1 if format is for a window
    #[cfg(feature = "tmux_2_6")]
    WindowFormat(Option<bool>),
    /// window_height - Height of window
    #[cfg(feature = "tmux_1_6")]
    WindowHeight(Option<usize>),
    /// window_id - Unique window ID
    #[cfg(feature = "tmux_1_7")]
    WindowId(Option<usize>),
    /// window_index - #I Index of window
    #[cfg(feature = "tmux_1_6")]
    WindowIndex(Option<usize>),
    /// window_last_flag - 1 if window is the last used
    #[cfg(feature = "tmux_2_0")]
    WindowLastFlag(Option<bool>),
    /// window_layout - Window layout description, ignoring zoomed window panes
    #[cfg(feature = "tmux_1_6")]
    WindowLayout(Option<Layout>),
    /// window_linked - 1 if window is linked across sessions
    #[cfg(feature = "tmux_2_1")]
    WindowLinked(Option<bool>),
    /// window_linked_sessions - Number of sessions this window is linked to
    #[cfg(feature = "tmux_3_1")]
    WindowLinkedSessions(Option<usize>),
    /// window_linked_sessions_list - List of sessions this window is linked to
    #[cfg(feature = "tmux_3_1")]
    WindowLinkedSessionsList(String),
    /// window_marked_flag - 1 if window contains the marked pane
    #[cfg(feature = "tmux_3_1")]
    WindowMarkedFlag(Option<bool>),
    /// window_name - #W Name of window
    #[cfg(feature = "tmux_1_6")]
    WindowName(String),
    /// window_offset_x - X offset into window if larger than client
    #[cfg(feature = "tmux_2_9")]
    WindowOffsetX(Option<usize>),
    /// window_offset_y - Y offset into window if larger than client
    #[cfg(feature = "tmux_2_9")]
    WindowOffsetY(Option<usize>),
    /// window_panes - Number of panes in window
    #[cfg(feature = "tmux_1_7")]
    WindowPanes(Option<usize>),
    /// window_silence_flag - 1 if window has silence alert
    #[cfg(feature = "tmux_1_9")]
    WindowSilenceFlag(Option<bool>),
    /// window_stack_index - Index in session most recent stack
    #[cfg(feature = "tmux_2_5")]
    WindowStackIndex(Option<usize>),
    /// window_start_flag - 1 if window has the lowest index
    #[cfg(feature = "tmux_2_9")]
    WindowStartFlag(Option<bool>),
    /// window_visible_layout - Window layout description, respecting zoomed window panes
    #[cfg(feature = "tmux_2_2")]
    WindowVisibleLayout(Option<Layout>),
    /// window_width - Width of window
    #[cfg(feature = "tmux_1_6")]
    WindowWidth(Option<usize>),
    /// window_zoomed_flag - 1 if window is zoomed
    #[cfg(feature = "tmux_2_0")]
    WindowZoomedFlag(Option<bool>),

    /// wrap_flag - Pane wrap flag
    #[cfg(feature = "tmux_1_8")]
    WrapFlag(Option<bool>),
}

impl VariableOutput {
    // convert str to Option<bool>
    pub fn parse_bool(s: &str) -> Option<bool> {
        s.parse::<usize>().map(|i| i == 1).ok()
    }

    pub fn from_string_ext(s: &str, variable: &Variable) -> Self {
        match variable {
            // alternate_on - if pane is in alternate screen
            #[cfg(feature = "tmux_1_8")]
            Variable::AlternateOn => Self::AlternateOn(s.parse::<usize>().ok()),

            // alternate_saved_x - Saved cursor X in alternate screen
            #[cfg(feature = "tmux_1_8")]
            Variable::AlternateSavedX => Self::AlternateSavedX(s.parse::<usize>().ok()),
            // alternate_saved_y - Saved cursor Y in alternate screen
            #[cfg(feature = "tmux_1_8")]
            Variable::AlternateSavedY => Self::AlternateSavedY(s.parse::<usize>().ok()),

            // Buffer
            // buffer_created - Time buffer created
            #[cfg(feature = "tmux_2_6")]
            Variable::BufferCreated => Self::BufferCreated(s.parse::<u128>().ok()),
            // buffer_name - Name of buffer
            #[cfg(feature = "tmux_2_3")]
            Variable::BufferName => Self::BufferName(s.to_string()),
            // buffer_sample - First 50 characters from the specified buffer
            #[cfg(feature = "tmux_1_7")]
            Variable::BufferSample => Self::BufferSample(s.to_string()),
            // buffer_size - Size of the specified buffer in bytes
            #[cfg(feature = "tmux_1_7")]
            Variable::BufferSize => Self::BufferSize(s.parse::<usize>().ok()),

            // Client
            // client_activity - Integer time client last had activity
            #[cfg(feature = "tmux_1_6")]
            Variable::ClientActivity => Self::ClientActivity(s.parse::<u128>().ok()),
            // client_cell_height - Height of each client cell in pixels
            #[cfg(feature = "tmux_3_1")]
            Variable::ClientCellHeight => Self::ClientCellHeight(Self::parse_usize(s)),
            // client_cell_width - Width of each client cell in pixels
            #[cfg(feature = "tmux_3_1")]
            Variable::ClientCellWidth => Self::ClientCellWidth(Self::parse_usize(s)),
            // client_activity_string - String time client last had activity
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
            Variable::ClientActivityString => Self::ClientActivityString(s.to_string()),
            // client_created - Integer time client created
            #[cfg(feature = "tmux_1_6")]
            Variable::ClientCreated => Self::ClientCreated(s.parse::<u128>().ok()),
            // client_created_string - String time client created
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
            Variable::ClientCreatedString => Self::ClientCreatedString(s.to_string()),
            // client_control_mode - 1 if client is in control mode
            #[cfg(feature = "tmux_2_1")]
            Variable::ClientControlMode => Self::ClientControlMode(Self::parse_bool(s)),
            // client_discarded - Bytes discarded when client behind
            #[cfg(feature = "tmux_2_1")]
            Variable::ClientDiscarded => Self::ClientDiscarded(s.to_string()),
            // client_cwd - Working directory of client
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            Variable::ClientCwd => Self::ClientCwd(s.to_string()),
            // client_height - Height of client
            #[cfg(feature = "tmux_1_6")]
            Variable::ClientHeight => Self::ClientHeight(s.parse::<usize>().ok()),
            // client_key_table - Current key table
            #[cfg(feature = "tmux_2_2")]
            Variable::ClientKeyTable => Self::ClientKeyTable(s.to_string()),
            // client_last_session - Name of the client's last session
            #[cfg(feature = "tmux_1_8")]
            Variable::ClientLastSession => Self::ClientLastSession(s.to_string()),
            // client_name - Name of client
            #[cfg(feature = "tmux_2_4")]
            Variable::ClientName => Self::ClientName(s.to_string()),
            // client_pid - PID of client process
            #[cfg(feature = "tmux_2_1")]
            Variable::ClientPid => Self::ClientPid(Self::parse_bool(s)),
            // client_prefix - 1 if prefix key has been pressed
            #[cfg(feature = "tmux_1_8")]
            Variable::ClientPrefix => Self::ClientPrefix(Self::parse_bool(s)),
            // client_readonly - 1 if client is readonly
            #[cfg(feature = "tmux_1_6")]
            Variable::ClientReadonly => Self::ClientReadonly(Self::parse_bool(s)),
            // client_session - Name of the client's session
            #[cfg(feature = "tmux_1_8")]
            Variable::ClientSession => Self::ClientSession(s.to_string()),
            // client_termname - Terminal name of client
            #[cfg(feature = "tmux_1_6")]
            Variable::ClientTermname => Self::ClientTermname(s.to_string()),
            // client_termtype - Terminal type of client
            #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_1")))]
            Variable::ClientTermtype => Self::ClientTermtype(s.to_string()),
            // client_tty - Pseudo terminal of client
            #[cfg(feature = "tmux_1_6")]
            Variable::ClientTty => Self::ClientTty(s.to_string()),
            // client_utf8 - 1 if client supports UTF-8
            #[cfg(feature = "tmux_1_6")]
            Variable::ClientUtf8 => Self::ClientUtf8(Self::parse_bool(s)),
            // client_width - Width of client
            #[cfg(feature = "tmux_1_6")]
            Variable::ClientWidth => Self::ClientWidth(s.parse::<usize>().ok()),
            // client_written - Bytes written to client
            #[cfg(feature = "tmux_2_4")]
            Variable::ClientWritten => Self::ClientWritten(s.parse::<usize>().ok()),

            // Command
            // command_hooked - Name of command hooked, if any
            #[cfg(feature = "tmux_2_3")]
            Variable::CommandHooked => Self::CommandHooked(s.to_string()),
            // command_name - Name of command in use, if any
            #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_2_4")))]
            Variable::CommandName => Self::CommandName(s.to_string()),
            // command - Name of command in use, if any
            #[cfg(feature = "tmux_2_4")]
            Variable::Command => Self::Command(s.to_string()),
            // command_list_name - Command name if listing commands
            #[cfg(feature = "tmux_2_3")]
            Variable::CommandListName => Self::CommandListName(s.to_string()),
            // command_list_alias - Command alias if listing commands
            #[cfg(feature = "tmux_2_3")]
            Variable::CommandListAlias => Self::CommandListAlias(s.to_string()),
            // command_list_usage - Command usage if listing commands
            #[cfg(feature = "tmux_2_3")]
            Variable::CommandListUsage => Self::CommandListUsage(s.to_string()),

            // Cursor
            // cursor_flag - Pane cursor flag
            #[cfg(feature = "tmux_1_8")]
            Variable::CursorFlag => Self::CursorFlag(s.to_string()),
            // cursor_character - Character at cursor in pane
            #[cfg(feature = "tmux_2_9")]
            Variable::CursorCharacter => Self::CursorCharacter(s.to_string()),
            // cursor_x - Cursor X position in pane
            #[cfg(feature = "tmux_1_8")]
            Variable::CursorX => Self::CursorX(s.parse::<usize>().ok()),
            // cursor_y - Cursor Y position in pane
            #[cfg(feature = "tmux_1_8")]
            Variable::CursorY => Self::CursorY(s.parse::<usize>().ok()),

            // copy_cursor_line - Line the cursor is on in copy mode
            #[cfg(feature = "tmux_3_1")]
            Variable::CursorCopyCursorLine => Self::CursorCopyCursorLine(s.to_string()),
            // copy_cursor_word - Word under cursor in copy mode
            #[cfg(feature = "tmux_3_1")]
            Variable::CursorCopyCursorWord => Self::CursorCopyCursorWord(s.to_string()),
            // copy_cursor_x - Cursor X position in copy mode
            #[cfg(feature = "tmux_3_1")]
            Variable::CursorCopyCursorX => Self::CursorCopyCursorX(s.to_string()),
            // copy_cursor_y - Cursor Y position in copy mode
            #[cfg(feature = "tmux_3_1")]
            Variable::CursorCopyCursorY => Self::CursorCopyCursorY(s.to_string()),

            // history
            // history_bytes             Number of bytes in window history
            #[cfg(feature = "tmux_1_7")]
            Variable::HistotyBytes => Self::HistotyBytes(s.parse::<usize>().ok()),
            // history_limit             Maximum window history lines
            #[cfg(feature = "tmux_1_7")]
            Variable::HistotyLimit => Self::HistotyLimit(s.parse::<usize>().ok()),
            // history_size              Size of history in bytes
            #[cfg(feature = "tmux_1_7")]
            Variable::HistorySize => Self::HistorySize(s.parse::<usize>().ok()),

            // hook
            // hook - Name of running hook, if any
            #[cfg(feature = "tmux_2_4")]
            Variable::Hook => Self::Hook(s.to_string()),
            // hook_pane - ID of pane where hook was run, if any
            #[cfg(feature = "tmux_2_4")]
            Variable::HookPane => Self::HookPane(s.parse::<usize>().ok()),
            // hook_session - ID of session where hook was run, if any
            #[cfg(feature = "tmux_2_4")]
            Variable::HookSession => Self::HookSession(s.parse::<usize>().ok()),
            // hook_session_name - Name of session where hook was run, if any
            #[cfg(feature = "tmux_2_4")]
            Variable::HookSessionName => Self::HookSessionName(s.to_string()),
            // hook_window - ID of window where hook was run, if any
            #[cfg(feature = "tmux_2_4")]
            Variable::HookWindow => Self::HookWindow(s.parse::<usize>().ok()),
            // hook_window_name - Name of window where hook was run, if any
            #[cfg(feature = "tmux_2_4")]
            Variable::HookWindowName => Self::HookWindowName(s.to_string()),

            // host
            // host - Hostname of local host
            #[cfg(feature = "tmux_1_6")]
            Variable::Host => Self::Host(s.to_string()),
            // host_short - #h Hostname of local host (no domain name)
            #[cfg(feature = "tmux_1_9")]
            Variable::HostShort => Self::HostShort(s.to_string()),

            // insert_flag - Pane insert flag
            #[cfg(feature = "tmux_1_8")]
            Variable::InsertFlag => Self::InsertFlag(s.to_string()),
            // keypad_cursor_flag - Pane keypad cursor flag
            #[cfg(feature = "tmux_1_8")]
            Variable::KeypadCursorFlag => Self::KeypadCursorFlag(s.to_string()),
            // keypad_flag - Pane keypad flag
            #[cfg(feature = "tmux_1_8")]
            Variable::KeypadFlag => Self::KeypadFlag(s.to_string()),

            // line - Line number in the list
            #[cfg(feature = "tmux_1_6")]
            Variable::Line => Self::Line(s.parse::<usize>().ok()),

            // mouse_all_flag - Pane mouse all flag
            #[cfg(feature = "tmux_3_0")]
            Variable::MouseAllFlag => Self::MouseAllFlag(s.to_string()),
            // mouse_any_flag - Pane mouse any flag
            #[cfg(feature = "tmux_1_8")]
            Variable::MouseAnyFlag => Self::MouseAnyFlag(s.to_string()),
            // mouse_button_flag - Pane mouse button flag
            #[cfg(feature = "tmux_1_8")]
            Variable::MouseButtonFlag => Self::MouseButtonFlag(s.to_string()),
            // mouse_line - Line under mouse, if any
            #[cfg(feature = "tmux_3_0")]
            Variable::MouseLine => Self::MouseLine(s.to_string()),
            // sgr_flag - Pane mouse SGR flag
            #[cfg(feature = "tmux_3_0")]
            Variable::MouseSgrFlag => Self::MouseSgrFlag(s.to_string()),
            // mouse_standard_flag - Pane mouse standard flag
            #[cfg(feature = "tmux_1_8")]
            Variable::MouseStandardFlag => Self::MouseStandardFlag(s.to_string()),
            // mouse_utf8_flag - Pane mouse UTF-8 flag
            #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_2"), feature = "tmux_3_0"))]
            Variable::MouseUtf8Flag => Self::MouseUtf8Flag(s.to_string()),
            // mouse_all_flag - Pane mouse all flag
            #[cfg(feature = "tmux_2_4")]
            Variable::MouseAllFlag => Self::MouseAllFlag(s.to_string()),
            // mouse_word - Word under mouse, if any
            #[cfg(feature = "tmux_3_0")]
            Variable::MouseWord => Self::MouseWord(s.to_string()),
            // mouse_x - Mouse X position, if any
            #[cfg(feature = "tmux_3_0")]
            Variable::MouseX => Self::MouseX(Self::parse_usize(s)),
            // mouse_y - Mouse Y position, if any
            #[cfg(feature = "tmux_3_0")]
            Variable::MouseY => Self::MouseY(Self::parse_usize(s)),
            // origin_flag - Pane origin flag
            #[cfg(feature = "tmux_3_0")]
            Variable::OriginFlag => Self::OriginFlag(s.to_string()),

            // pane
            // pane_active - 1 if active pane
            #[cfg(feature = "tmux_1_6")]
            Variable::PaneActive => Self::PaneActive(Self::parse_bool(s)),
            // pane_at_bottom - 1 if pane is at the bottom of window
            #[cfg(feature = "tmux_2_6")]
            Variable::PaneAtBottom => Self::PaneAtBottom(Self::parse_bool(s)),
            // pane_at_left - 1 if pane is at the left of window
            #[cfg(feature = "tmux_2_6")]
            Variable::PaneAtLeft => Self::PaneAtLeft(Self::parse_bool(s)),
            // 1 if pane is at the right of window
            #[cfg(feature = "tmux_2_6")]
            Variable::PaneAtRight => Self::PaneAtRight(Self::parse_bool(s)),
            // 1 if pane is at the top of window
            #[cfg(feature = "tmux_2_6")]
            Variable::PaneAtTop => Self::PaneAtTop(Self::parse_bool(s)),
            // pane_bottom - Bottom of pane
            #[cfg(feature = "tmux_2_0")]
            Variable::PaneBottom => Self::PaneBottom(s.parse::<usize>().ok()),
            // pane_current_command - Current command if available
            #[cfg(feature = "tmux_1_8")]
            Variable::PaneCurrentCommand => Self::PaneCurrentCommand(s.to_string()),
            // pane_current_path - Current path if available
            #[cfg(feature = "tmux_1_7")]
            Variable::PaneCurrentPath => Self::PaneCurrentPath(s.to_string()),
            // pane_dead - 1 if pane is dead
            #[cfg(feature = "tmux_1_6")]
            Variable::PaneDead => Self::PaneDead(Self::parse_bool(s)),
            // pane_dead_status - Exit status of process in dead pane
            #[cfg(feature = "tmux_2_0")]
            Variable::PaneDeadStatus => Self::PaneDeadStatus(s.parse::<usize>().ok()),
            // 1 if format is for a pane
            #[cfg(feature = "tmux_2_6")]
            Variable::PaneFormat => Self::PaneFormat(Self::parse_bool(s)),
            // pane_height - Height of pane
            #[cfg(feature = "tmux_1_6")]
            Variable::PaneHeight => Self::PaneHeight(s.parse::<usize>().ok()),
            // pane_id - #D Unique pane ID
            #[cfg(feature = "tmux_1_6")]
            Variable::PaneId => Self::PaneId(s.parse::<usize>().ok()),
            // pane_in_mode - 1 if pane is in a mode
            #[cfg(feature = "tmux_1_8")]
            Variable::PaneInMode => Self::PaneInMode(Self::parse_bool(s)),
            // pane_index - #P Index of pane
            #[cfg(feature = "tmux_1_7")]
            Variable::PaneIndex => Self::PaneIndex(s.parse::<usize>().ok()),
            // pane_input_off - 1 if input to pane is disabled
            #[cfg(feature = "tmux_2_0")]
            Variable::PaneInputOff => Self::PaneInputOff(Self::parse_bool(s)),
            // pane_left - Left of pane
            #[cfg(feature = "tmux_2_0")]
            Variable::PaneLeft => Self::PaneLeft(s.parse::<usize>().ok()),
            // pane_marked - 1 if this is the marked pane
            #[cfg(feature = "tmux_3_0")]
            Variable::PaneMarked => Self::PaneMarked(Self::parse_bool()),
            // pane_marked_set - 1 if a marked pane is set
            #[cfg(feature = "tmux_3_0")]
            Variable::PaneMarkedSet => Self::PaneMarkedSet(Self::parse_bool()),
            // pane_mode - Name of pane mode, if any
            #[cfg(feature = "tmux_2_5")]
            Variable::PaneMode => Self::PaneMode(s.parse::<usize>().ok()),
            // pane_path - #T Path of pane (can be set by application)
            #[cfg(feature = "tmux_3_1")]
            Variable::PanePath => Self::PanePath(s.to_string()),
            // pane_pid - PID of first process in pane
            #[cfg(feature = "tmux_1_6")]
            Variable::PanePid => Self::PanePid(s.parse::<usize>().ok()),
            // pane_pipe - 1 if pane is being piped
            #[cfg(feature = "tmux_2_6")]
            Variable::PanePipe => Self::PanePipe(Self::parse_bool(s)),
            // pane_right - Right of pane
            #[cfg(feature = "tmux_2_0")]
            Variable::PaneRight => Self::PaneRight(s.parse::<usize>().ok()),
            // Last search string in copy mode
            #[cfg(feature = "tmux_2_5")]
            Variable::PaneSearchString => Self::PaneSearchString(s.parse::<usize>().ok()),
            // pane_start_command - Command pane started with
            #[cfg(feature = "tmux_1_6")]
            Variable::PaneStartCommand => Self::PaneStartCommand(s.parse::<usize>().ok()),
            // pane_start_path - Path pane started with
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_0")))]
            Variable::PaneStartPath => Self::PaneStartPath(Self::parse_usize(s)),
            // pane_synchronized - 1 if pane is synchronized
            #[cfg(feature = "tmux_1_9")]
            Variable::PaneSynchronized => Self::PaneSynchronized(Self::parse_bool(s)),
            // pane_tabs - Pane tab positions
            #[cfg(feature = "tmux_1_8")]
            Variable::PaneTabs => Self::PaneTabs(s.parse::<PaneTabs>().ok()),
            // pane_title - #T Title of pane (can be set by application)
            #[cfg(feature = "tmux_1_6")]
            Variable::PaneTitle => Self::PaneTitle(s.to_string()),
            // pane_top - Top of pane
            #[cfg(feature = "tmux_2_0")]
            Variable::PaneTop => Self::PaneTop(s.parse::<usize>().ok()),
            // pane_tty - Pseudo terminal of pane
            #[cfg(feature = "tmux_1_6")]
            Variable::PaneTty => Self::PaneTty(s.to_string()),
            // pane_width - Width of pane
            #[cfg(feature = "tmux_1_6")]
            Variable::PaneWidth => Self::PaneWidth(s.parse::<usize>().ok()),

            // saved_cursor_x - Saved cursor X in pane
            #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
            Variable::SavedCursorX => Self::SavedCursorX(s.parse::<usize>().ok()),
            // saved_cursor_y - Saved cursor Y in pane
            #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
            Variable::SavedCursorY => Self::SavedCursorY(s.parse::<usize>().ok()),

            // pid - Server PID
            #[cfg(feature = "tmux_2_1")]
            Variable::Pid => Self::Pid(s.parse::<usize>().ok()),
            // rectangle_toggle - 1 if rectangle selection is activated
            #[cfg(feature = "tmux_2_7")]
            Variable::RectangleToggle => Self::RectangleToggle(Self::parse_bool(s)),

            // scroll_position - Scroll position in copy mode
            #[cfg(feature = "tmux_2_2")]
            Variable::ScrollPosition => Self::ScrollPosition(s.parse::<usize>().ok()),
            // scroll_region_lower - Bottom of scroll region in pane
            #[cfg(feature = "tmux_1_8")]
            Variable::ScrollRegionLower => Self::ScrollRegionLower(s.parse::<usize>().ok()),
            // scroll_region_upper - Top of scroll region in pane
            #[cfg(feature = "tmux_1_8")]
            Variable::ScrollRegionUpper => Self::ScrollRegionUpper(s.parse::<usize>().ok()),

            // selection_active - 1 if selection started and changes with the curso
            #[cfg(feature = "tmux_3_1")]
            Variable::SelectionActive => Self::SelectionActive(Self::parse_bool()),
            // selection_end_x - X position of the end of the selection
            #[cfg(feature = "tmux_3_1")]
            Variable::SelectionEndX => Self::SelectionEndX(Self::parse_usize(s)),
            // selection_end_y - Y position of the end of the selection
            #[cfg(feature = "tmux_3_1")]
            Variable::SelectionEndY => Self::SelectionEndY(Self::parse_usize(s)),
            // selection_present - 1 if selection started in copy mode
            #[cfg(feature = "tmux_2_6")]
            Variable::SelectionPresent => Self::SelectionPresent(Self::parse_bool(s)),
            // selection_start_x - X position of the start of the selection
            #[cfg(feature = "tmux_3_1")]
            Variable::SelectionStartX => Self::SelectionStartX(Self::parse_usize(s)),
            // selection_start_y - Y position of the start of the selection
            #[cfg(feature = "tmux_3_1")]
            Variable::SelectionStartY => Self::SelectionStartY(Self::parse_usize(s)),

            // Session
            // session_activity - Time of session last activity
            #[cfg(feature = "tmux_2_1")]
            Variable::SessionActivity => Self::SessionActivity(s.parse::<u128>().ok()),
            // session_activity_string - String time of session last activity
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
            Variable::SessionActivityString => Self::SessionActivityString(s.to_string()),
            // session_alerts - List of window indexes with alerts
            #[cfg(feature = "tmux_2_1")]
            Variable::SessionAlerts => Self::SessionAlerts(s.to_string()),
            // session_attached - Number of clients session is attached to
            #[cfg(feature = "tmux_1_6")]
            Variable::SessionAttached => Self::SessionAttached(s.parse::<usize>().ok()),
            // session_attached_list - List of clients session is attached to
            #[cfg(feature = "tmux_3_1")]
            Variable::SessionAttachedList => Self::SessionAttachedList(Self::parse_usize(s)),
            // session_created - Time session created
            #[cfg(feature = "tmux_1_6")]
            Variable::SessionCreated => Self::SessionCreated(s.parse::<u128>().ok()),
            // String time session created
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
            Variable::SessionCreatedString => Self::SessionCreatedString(s.to_string()),
            // session_format - 1 if format is for a session (not assuming the current)
            #[cfg(feature = "tmux_2_6")]
            Variable::SessionFormat => Self::SessionFormat(Self::parse_bool(s)),
            // session_group - Name of session group
            #[cfg(feature = "tmux_1_6")]
            Variable::SessionGroup => Self::SessionGroup(s.to_string()),
            // session_group_attached - Number of clients sessions in group are attached >
            #[cfg(feature = "tmux_3_1")]
            Variable::SessionGroupAttached => Self::SessionGroupAttached(Self::parse_usize(s)),
            // session_group_attached_list - List of clients sessions in group are attached to
            #[cfg(feature = "tmux_3_1")]
            Variable::SessionGroupAttachedList => Self::SessionGroupAttachedList(s.to_string()),
            // session_group_list - List of sessions in group
            #[cfg(feature = "tmux_2_7")]
            Variable::SessionGroupList => Self::SessionGroupList(s.to_string()),
            // session_group_many_attached - 1 if multiple clients attached to sessions in gro
            #[cfg(feature = "tmux_3_1")]
            Variable::SessionGroupManyAttached => {
                Self::SessionGroupManyAttached(Self::parse_bool())
            }
            // session_size - Size of session group
            #[cfg(feature = "tmux_2_7")]
            Variable::SessionGroupSize => Self::SessionGroupSize(s.to_string()),
            // session_grouped - 1 if session in a group
            #[cfg(feature = "tmux_1_6")]
            Variable::SessionGrouped => Self::SessionGrouped(Self::parse_bool(s)),
            // session_height - Height of session
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
            Variable::SessionHeight => Self::SessionHeight(s.parse::<usize>().ok()),
            // session_width - Width of session
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
            Variable::SessionWidth => Self::SessionWidth(s.parse::<usize>().ok()),
            // session_id - Unique session ID
            #[cfg(feature = "tmux_1_8")]
            Variable::SessionId => Self::SessionId(s.parse::<usize>().ok()),
            // session_last_attached - Time session last attached
            #[cfg(feature = "tmux_2_1")]
            Variable::SessionLastAttached => Self::SessionLastAttached(s.parse::<u128>().ok()),
            // session_last_attached_string - String time session last attached
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
            Variable::SessionLastAttachedString => Self::SessionLastAttachedString(s.to_string()),
            // session_many_attached - 1 if multiple clients attached
            #[cfg(feature = "tmux_2_0")]
            Variable::SessionManyAttached => Self::SessionManyAttached(Self::parse_bool(s)),
            // session_name - #S Name of session
            #[cfg(feature = "tmux_1_6")]
            Variable::SessionName => Self::SessionName(s.to_string()),
            // session_stack - Window indexes in most recent order
            #[cfg(feature = "tmux_2_5")]
            Variable::SessionStack => Self::SessionStack(s.parse::<SessionStack>().ok()),
            // session_windows - Number of windows in session
            #[cfg(feature = "tmux_1_6")]
            Variable::SessionWindows => Self::SessionWindows(s.parse::<usize>().ok()),

            // socket_path - Server socket path
            #[cfg(feature = "tmux_2_2")]
            Variable::SocketPath => Self::SocketPath(s.to_string()),
            // start_time - Server start time
            #[cfg(feature = "tmux_2_2")]
            Variable::StartTime => Self::StartTime(s.parse::<u128>().ok()),

            // version - Server version
            #[cfg(feature = "tmux_2_4")]
            Variable::Version => Self::Version(s.to_string()),

            // Window
            //
            // window_active - 1 if window active
            #[cfg(feature = "tmux_1_6")]
            Variable::WindowActive => Self::WindowActive(Self::parse_bool(s)),
            // window_active_clients - Number of clients viewing this window
            #[cfg(feature = "tmux_3_1")]
            Variable::WindowActiveClients => Self::WindowActiveClients(Self::parse_usize(s)),
            // window_active_clients_list - List of clients viewing this window
            #[cfg(feature = "tmux_3_1")]
            Variable::WindowActiveClientsList => Self::WindowActiveClientsList(s.to_string()),
            // window_active_sessions - Number of sessions on which this window is active
            #[cfg(feature = "tmux_3_1")]
            Variable::WindowActiveSessions => Self::WindowActiveSessions(Self::parse_usize(s)),
            // window_active_sessions_list - List of sessions on which this window is active
            #[cfg(feature = "tmux_3_1")]
            Variable::WindowActiveSessionsList => Self::WindowActiveSessionsList(s.to_string()),
            // window_activity - Time of window last activity
            #[cfg(feature = "tmux_2_1")]
            Variable::WindowActivity => Self::WindowActivity(s.parse::<usize>().ok()),
            // session_activity_string - String time of session last activity
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
            Variable::SessionActivityString => Self::SessionActivityString(s.to_string()),
            // window_activity_flag - 1 if window has activity
            #[cfg(any(
                all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
                feature = "tmux_2_3"
            ))]
            Variable::WindowActivityFlag => Self::WindowActivityFlag(Self::parse_bool(s)),
            // window_bell_flag - 1 if window has bell
            #[cfg(feature = "tmux_1_9")]
            Variable::WindowBellFlag => Self::WindowBellFlag(Self::parse_bool(s)),
            // window_content_flag - 1 if window has content alert
            #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
            Variable::WindowContentflag => Self::WindowContentflag(Self::parse_bool()),
            // window_bigger - 1 if window is larger than client
            #[cfg(feature = "tmux_2_9")]
            Variable::WindowBigger => Self::WindowBigger(Self::parse_bool()),
            // window_cell_height - Height of each cell in pixels
            #[cfg(feature = "tmux_3_1")]
            Variable::WindowCellHeight => Self::WindowCellHeight(Self::parse_usize(s)),
            // window_cell_width - Width of each cell in pixels
            #[cfg(feature = "tmux_3_1")]
            Variable::WindowCellWidth => Self::WindowCellWidth(Self::parse_usize(s)),
            // window_end_flag - 1 if window has the highest index
            #[cfg(feature = "tmux_2_9")]
            Variable::WindowEndFlag => Self::WindowEndFlag(Self::parse_bool()),
            // window_find_matches - Matched data from the find-window command if available
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
            Variable::WindowFindMatches => Self::WindowFindMatches(s.to_string()),
            // window_flags - #F Window flags
            #[cfg(feature = "tmux_1_6")]
            Variable::WindowFlags => Self::WindowFlags(s.parse::<WindowFlag>().ok()),
            // window_format - 1 if format is for a window
            #[cfg(feature = "tmux_2_6")]
            Variable::WindowFormat => Self::WindowFormat(Self::parse_bool(s)),
            // window_height - Height of window
            #[cfg(feature = "tmux_1_6")]
            Variable::WindowHeight => Self::WindowHeight(s.parse::<usize>().ok()),
            // window_id - Unique window ID
            #[cfg(feature = "tmux_1_7")]
            Variable::WindowId => Self::WindowId(s.parse::<usize>().ok()),
            // window_index - #I Index of window
            #[cfg(feature = "tmux_1_6")]
            Variable::WindowIndex => Self::WindowIndex(s.parse::<usize>().ok()),
            // window_last_flag - 1 if window is the last used
            #[cfg(feature = "tmux_2_0")]
            Variable::WindowLastFlag => Self::WindowLastFlag(Self::parse_bool(s)),
            // window_layout - Window layout description, ignoring zoomed window panes
            #[cfg(feature = "tmux_1_6")]
            Variable::WindowLayout => Self::WindowLayout(s.parse::<Layout>().ok()),
            // window_linked - 1 if window is linked across sessions
            #[cfg(feature = "tmux_2_1")]
            Variable::WindowLinked => Self::WindowLinked(Self::parse_bool(s)),
            // window_linked_sessions - Number of sessions this window is linked to
            #[cfg(feature = "tmux_3_1")]
            Variable::WindowLinkedSessions => Self::WindowLinkedSessions(Self::parse_usize(s)),
            // window_linked_sessions_list - List of sessions this window is linked to
            #[cfg(feature = "tmux_3_1")]
            Variable::WindowLinkedSessionsList => Self::WindowLinkedSessionsList(s.to_string()),
            // window_marked_flag - 1 if window contains the marked pane
            #[cfg(feature = "tmux_3_1")]
            Variable::WindowMarkedFlag => Self::WindowMarkedFlag(Self::parse_bool()),
            // window_name - #W Name of window
            #[cfg(feature = "tmux_1_6")]
            Variable::WindowName => Self::WindowName(s.to_string()),
            // window_offset_x - X offset into window if larger than client
            #[cfg(feature = "tmux_2_9")]
            Variable::WindowOffsetX => Self::WindowOffsetX(Self::parse_usize(s)),
            // window_offset_y - Y offset into window if larger than client
            #[cfg(feature = "tmux_2_9")]
            Variable::WindowOffsetY => Self::WindowOffsetY(Self::parse_usize(s)),
            // window_panes - Number of panes in window
            #[cfg(feature = "tmux_1_7")]
            Variable::WindowPanes => Self::WindowPanes(s.parse::<usize>().ok()),
            // window_silence_flag - 1 if window has silence alert
            #[cfg(feature = "tmux_1_9")]
            Variable::WindowSilenceFlag => Self::WindowSilenceFlag(Self::parse_bool(s)),
            // window_stack_index - Index in session most recent stack
            #[cfg(feature = "tmux_2_5")]
            Variable::WindowStackIndex => Self::WindowStackIndex(s.parse::<usize>().ok()),
            // window_start_flag - 1 if window has the lowest index
            #[cfg(feature = "tmux_2_9")]
            Variable::WindowStartFlag => Self::WindowStartFlag(Self::parse_bool(s)),
            // window_visible_layout - Window layout description, respecting zoomed window panes
            #[cfg(feature = "tmux_2_2")]
            Variable::WindowVisibleLayout => Self::WindowVisibleLayout(s.parse::<Layout>().ok()),
            // window_width - Width of window
            #[cfg(feature = "tmux_1_6")]
            Variable::WindowWidth => Self::WindowWidth(s.parse::<usize>().ok()),
            // window_zoomed_flag - 1 if window is zoomed
            #[cfg(feature = "tmux_2_0")]
            Variable::WindowZoomedFlag => Self::WindowZoomedFlag(Self::parse_bool(s)),

            // wrap_flag - Pane wrap flag
            #[cfg(feature = "tmux_1_8")]
            Variable::WrapFlag => Self::WrapFlag(Self::parse_bool(s)),
            // other
            //_ => Self::Error,
        }
    }
}
