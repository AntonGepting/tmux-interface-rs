#[cfg(feature = "tmux_2_5")]
use crate::SessionStack;
#[cfg(feature = "tmux_1_6")]
use crate::{Layout, PaneTabs, WindowFlags};

// XXX: ? + - etc refactor in structure in future? split in enum and struct add fields
// XXX: options allowed too
#[derive(Debug)]
pub enum VariableOutput<'a> {
    //Custom(String)
    /// `alternate_on` - if pane is in alternate screen
    #[cfg(feature = "tmux_1_8")]
    AlternateOn(&'a mut Option<usize>),
    /// `alternate_saved_x` - Saved cursor X in alternate screen
    #[cfg(feature = "tmux_1_8")]
    AlternateSavedX(&'a mut Option<usize>),
    /// `alternate_saved_y` - Saved cursor Y in alternate screen
    #[cfg(feature = "tmux_1_8")]
    AlternateSavedY(&'a mut Option<usize>),

    // Buffer
    /// `buffer_created` - Time buffer created
    #[cfg(feature = "tmux_2_6")]
    BufferCreated(&'a mut Option<u128>),
    /// `buffer_name` - Name of buffer
    #[cfg(feature = "tmux_2_3")]
    BufferName(&'a mut Option<String>),
    /// `buffer_sample` - First 50 characters from the specified buffer
    #[cfg(feature = "tmux_1_7")]
    BufferSample(&'a mut Option<String>),
    /// `buffer_size` - Size of the specified buffer in bytes
    #[cfg(feature = "tmux_1_7")]
    BufferSize(&'a mut Option<usize>),

    // Client
    /// `client_activity` - Integer time client last had activity
    #[cfg(feature = "tmux_1_6")]
    ClientActivity(&'a mut Option<u128>),
    /// `client_cell_height` - Height of each client cell in pixels
    #[cfg(feature = "tmux_3_1")]
    ClientCellHeight(&'a mut Option<usize>),
    /// `client_cell_width` - Width of each client cell in pixels
    #[cfg(feature = "tmux_3_1")]
    ClientCellWidth(&'a mut Option<usize>),
    /// `client_activity_string` - Option<String> time client last had activity
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    ClientActivityString(&'a mut Option<String>),
    /// `client_created` - Integer time client created
    #[cfg(feature = "tmux_1_6")]
    ClientCreated(&'a mut Option<u128>),
    /// `client_created_string` - Option<String> time client created
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    ClientCreatedString(&'a mut Option<String>),
    /// `client_control_mode` - 1 if client is in control mode
    #[cfg(feature = "tmux_2_1")]
    ClientControlMode(&'a mut Option<bool>),
    /// `client_discarded` - Bytes discarded when client behind
    #[cfg(feature = "tmux_2_1")]
    ClientDiscarded(&'a mut Option<String>),
    /// `client_cwd` - Working directory of client
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    ClientCwd(&'a mut Option<String>),
    /// `client_height` - Height of client
    #[cfg(feature = "tmux_1_6")]
    ClientHeight(&'a mut Option<usize>),
    /// `client_key_table` - Current key table
    #[cfg(feature = "tmux_2_2")]
    ClientKeyTable(&'a mut Option<String>),
    /// `client_last_session` - Name of the client's last session
    #[cfg(feature = "tmux_1_8")]
    ClientLastSession(&'a mut Option<String>),
    /// `client_name` - Name of client
    #[cfg(feature = "tmux_2_4")]
    ClientName(&'a mut Option<String>),
    /// `client_pid` - PID of client process
    #[cfg(feature = "tmux_2_1")]
    ClientPid(&'a mut Option<usize>),
    /// `client_prefix` - 1 if prefix key has been pressed
    #[cfg(feature = "tmux_1_8")]
    ClientPrefix(&'a mut Option<bool>),
    /// `client_readonly` - 1 if client is readonly
    #[cfg(feature = "tmux_1_6")]
    ClientReadonly(&'a mut Option<bool>),
    /// `client_session` - Name of the client's session
    #[cfg(feature = "tmux_1_8")]
    ClientSession(&'a mut Option<String>),
    /// `client_termname` - Terminal name of client
    #[cfg(feature = "tmux_1_6")]
    ClientTermname(&'a mut Option<String>),
    /// `client_termtype` - Terminal type of client
    #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_1")))]
    ClientTermtype(&'a mut Option<String>),
    /// `client_tty` - Pseudo terminal of client
    #[cfg(feature = "tmux_1_6")]
    ClientTty(&'a mut Option<String>),
    /// `client_utf8` - 1 if client supports UTF-8
    #[cfg(feature = "tmux_1_6")]
    ClientUtf8(&'a mut Option<bool>),
    /// `client_width` - Width of client
    #[cfg(feature = "tmux_1_6")]
    ClientWidth(&'a mut Option<usize>),
    /// `client_written` - Bytes written to client
    #[cfg(feature = "tmux_2_4")]
    ClientWritten(&'a mut Option<usize>),

    // Command
    /// `command_hooked` - Name of command hooked, if any
    #[cfg(feature = "tmux_2_3")]
    CommandHooked(&'a mut Option<String>),
    /// `command_name` - Name of command in use, if any
    #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_2_4")))]
    CommandName(&'a mut Option<String>),
    /// `command` - Name of command in use, if any
    #[cfg(feature = "tmux_2_4")]
    Command(&'a mut Option<String>),
    /// `command_list_name` - Command name if listing commands
    #[cfg(feature = "tmux_2_3")]
    CommandListName(&'a mut Option<String>),
    /// `command_list_alias` - Command alias if listing commands
    #[cfg(feature = "tmux_2_3")]
    CommandListAlias(&'a mut Option<String>),
    /// `command_list_usage` - Command usage if listing commands
    #[cfg(feature = "tmux_2_3")]
    CommandListUsage(&'a mut Option<String>),

    // Cursor
    /// `cursor_flag` - Pane cursor flag
    #[cfg(feature = "tmux_1_8")]
    CursorFlag(&'a mut Option<String>),
    /// `cursor_character` - Character at cursor in pane
    #[cfg(feature = "tmux_2_9")]
    CursorCharacter(&'a mut Option<String>),
    /// `cursor_x` - Cursor X position in pane
    #[cfg(feature = "tmux_1_8")]
    CursorX(&'a mut Option<usize>),
    /// `cursor_y` - Cursor Y position in pane
    #[cfg(feature = "tmux_1_8")]
    CursorY(&'a mut Option<usize>),

    /// `copy_cursor_line` - Line the cursor is on in copy mode
    #[cfg(feature = "tmux_3_1")]
    CopyCursorLine(&'a mut Option<String>),
    /// `copy_cursor_word` - Word under cursor in copy mode
    #[cfg(feature = "tmux_3_1")]
    CopyCursorWord(&'a mut Option<String>),
    /// `copy_cursor_x` - Cursor X position in copy mode
    #[cfg(feature = "tmux_3_1")]
    CopyCursorX(&'a mut Option<usize>),
    /// `copy_cursor_y` - Cursor Y position in copy mode
    #[cfg(feature = "tmux_3_1")]
    CopyCursorY(&'a mut Option<usize>),
    /// `current_file` - Current configuration file
    #[cfg(feature = "tmux_3_2")]
    CurrentFile(&'a mut Option<String>),

    // history
    /// `history_bytes`             Number of bytes in window history
    #[cfg(feature = "tmux_1_7")]
    HistotyBytes(&'a mut Option<usize>),
    /// `history_limit`             Maximum window history lines
    #[cfg(feature = "tmux_1_7")]
    HistotyLimit(&'a mut Option<usize>),
    /// `history_size`              Size of history in bytes
    #[cfg(feature = "tmux_1_7")]
    HistorySize(&'a mut Option<usize>),

    // hook
    /// `hook` - Name of running hook, if any
    #[cfg(feature = "tmux_2_4")]
    Hook(&'a mut Option<String>),
    /// `hook_pane` - ID of pane where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    HookPane(&'a mut Option<usize>),
    /// `hook_session` - ID of session where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    HookSession(&'a mut Option<usize>),
    /// `hook_session_name` - Name of session where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    HookSessionName(&'a mut Option<String>),
    /// `hook_window` - ID of window where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    HookWindow(&'a mut Option<usize>),
    /// `hook_window_name` - Name of window where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    HookWindowName(&'a mut Option<String>),

    // host
    /// `host` - Hostname of local host
    #[cfg(feature = "tmux_1_6")]
    Host(&'a mut Option<String>),
    /// `host_short` - #h Hostname of local host (no domain name)
    #[cfg(feature = "tmux_1_9")]
    HostShort(&'a mut Option<String>),

    /// `insert_flag` - Pane insert flag
    #[cfg(feature = "tmux_1_8")]
    InsertFlag(&'a mut Option<String>),
    /// `keypad_cursor_flag` - Pane keypad cursor flag
    #[cfg(feature = "tmux_1_8")]
    KeypadCursorFlag(&'a mut Option<String>),
    /// `keypad_flag` - Pane keypad flag
    #[cfg(feature = "tmux_1_8")]
    KeypadFlag(&'a mut Option<String>),

    /// `line` - Line number in the list
    #[cfg(feature = "tmux_1_6")]
    Line(&'a mut Option<usize>),

    // `mouse_all_flag` - Pane mouse all flag
    // #[cfg(feature = "tmux_3_0")]
    // MouseAllFlag(&'a mut Option<String>),
    /// `mouse_any_flag` - Pane mouse any flag
    #[cfg(feature = "tmux_1_8")]
    MouseAnyFlag(&'a mut Option<String>),
    /// `mouse_button_flag` - Pane mouse button flag
    #[cfg(feature = "tmux_1_8")]
    MouseButtonFlag(&'a mut Option<String>),
    /// `mouse_line` - Line under mouse, if any
    #[cfg(feature = "tmux_3_0")]
    MouseLine(&'a mut Option<String>),
    /// `sgr_flag` - Pane mouse SGR flag
    #[cfg(feature = "tmux_3_0")]
    MouseSgrFlag(&'a mut Option<String>),
    /// `mouse_standard_flag` - Pane mouse standard flag
    #[cfg(feature = "tmux_1_8")]
    MouseStandardFlag(&'a mut Option<String>),
    /// `mouse_status_line` - Status line on which mouse event took place
    #[cfg(feature = "tmux_3_4")]
    MouseStatusLine(&'a mut Option<String>),
    /// `mouse_status_range` - Range type or argument of mouse event on status line
    #[cfg(feature = "tmux_3_4")]
    MouseStatusRange(&'a mut Option<String>),
    /// `mouse_utf8_flag` - Pane mouse UTF-8 flag
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_2"), feature = "tmux_3_0"))]
    MouseUtf8Flag(&'a mut Option<String>),
    /// `mouse_all_flag` - Pane mouse all flag
    #[cfg(feature = "tmux_2_4")]
    MouseAllFlag(&'a mut Option<String>),
    /// `mouse_word` - Word under mouse, if any
    #[cfg(feature = "tmux_3_0")]
    MouseWord(&'a mut Option<String>),
    /// `mouse_x` - Mouse X position, if any
    #[cfg(feature = "tmux_3_0")]
    MouseX(&'a mut Option<usize>),
    /// `mouse_y` - Mouse Y position, if any
    #[cfg(feature = "tmux_3_0")]
    MouseY(&'a mut Option<usize>),
    /// `origin_flag` - Pane origin flag
    #[cfg(feature = "tmux_3_0")]
    OriginFlag(&'a mut Option<String>),

    // pane
    /// `pane_active` - 1 if active pane
    #[cfg(feature = "tmux_1_6")]
    PaneActive(&'a mut Option<bool>),
    /// `pane_at_bottom` - 1 if pane is at the bottom of window
    #[cfg(feature = "tmux_2_6")]
    PaneAtBottom(&'a mut Option<bool>),
    /// `pane_at_left` - 1 if pane is at the left of window
    #[cfg(feature = "tmux_2_6")]
    PaneAtLeft(&'a mut Option<bool>),
    /// `pane_at_right` - 1 if pane is at the right of window
    #[cfg(feature = "tmux_2_6")]
    PaneAtRight(&'a mut Option<bool>),
    /// `pane_at_top` - 1 if pane is at the top of window
    #[cfg(feature = "tmux_2_6")]
    PaneAtTop(&'a mut Option<bool>),
    /// `pane_bottom` - Bottom of pane
    #[cfg(feature = "tmux_2_0")]
    PaneBottom(&'a mut Option<usize>),
    /// `pane_current_command` - Current command if available
    #[cfg(feature = "tmux_1_8")]
    PaneCurrentCommand(&'a mut Option<String>),
    /// `pane_current_path` - Current path if available
    #[cfg(feature = "tmux_1_7")]
    PaneCurrentPath(&'a mut Option<String>),
    /// `pane_dead` - 1 if pane is dead
    #[cfg(feature = "tmux_1_6")]
    PaneDead(&'a mut Option<bool>),
    /// `pane_dead_status` - Exit status of process in dead pane
    #[cfg(feature = "tmux_2_0")]
    PaneDeadStatus(&'a mut Option<usize>),
    /// `pane_format` - 1 if format is for a pane
    #[cfg(feature = "tmux_2_6")]
    PaneFormat(&'a mut Option<bool>),
    /// `pane_height` - Height of pane
    #[cfg(feature = "tmux_1_6")]
    PaneHeight(&'a mut Option<usize>),
    /// `pane_id` - #D Unique pane ID
    #[cfg(feature = "tmux_1_6")]
    PaneId(&'a mut Option<usize>),
    /// `pane_in_mode` - 1 if pane is in a mode
    #[cfg(feature = "tmux_1_8")]
    PaneInMode(&'a mut Option<bool>),
    /// `pane_index` - #P Index of pane
    #[cfg(feature = "tmux_1_7")]
    PaneIndex(&'a mut Option<usize>),
    /// `pane_input_off` - 1 if input to pane is disabled
    #[cfg(feature = "tmux_2_0")]
    PaneInputOff(&'a mut Option<bool>),
    /// `pane_left` - Left of pane
    #[cfg(feature = "tmux_2_0")]
    PaneLeft(&'a mut Option<usize>),
    /// `pane_marked` - 1 if this is the marked pane
    #[cfg(feature = "tmux_3_0")]
    PaneMarked(&'a mut Option<bool>),
    /// `pane_marked_set` - 1 if a marked pane is set
    #[cfg(feature = "tmux_3_0")]
    PaneMarkedSet(&'a mut Option<bool>),
    /// `pane_mode` - Name of pane mode, if any
    #[cfg(feature = "tmux_2_5")]
    PaneMode(&'a mut Option<usize>),
    /// `pane_path` - #T Path of pane (can be set by application)
    #[cfg(feature = "tmux_3_1")]
    PanePath(&'a mut Option<String>),
    /// `pane_pid` - PID of first process in pane
    #[cfg(feature = "tmux_1_6")]
    PanePid(&'a mut Option<usize>),
    /// `pane_pipe` - 1 if pane is being piped
    #[cfg(feature = "tmux_2_6")]
    PanePipe(&'a mut Option<bool>),
    /// `pane_right` - Right of pane
    #[cfg(feature = "tmux_2_0")]
    PaneRight(&'a mut Option<usize>),
    /// `Last` search `Option<String>` in copy mode
    #[cfg(feature = "tmux_2_5")]
    PaneSearchString(&'a mut Option<usize>),
    /// `pane_start_command` - Command pane started with
    #[cfg(feature = "tmux_1_6")]
    PaneStartCommand(&'a mut Option<usize>),
    /// `pane_start_path` - Path pane started with
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_0")))]
    PaneStartPath(&'a mut Option<usize>),
    /// `pane_synchronized` - 1 if pane is synchronized
    #[cfg(feature = "tmux_1_9")]
    PaneSynchronized(&'a mut Option<bool>),
    /// `pane_tabs` - Pane tab positions
    #[cfg(feature = "tmux_1_8")]
    PaneTabs(&'a mut Option<PaneTabs>),
    /// `pane_title` - #T Title of pane (can be set by application)
    #[cfg(feature = "tmux_1_6")]
    PaneTitle(&'a mut Option<String>),
    /// `pane_top` - Top of pane
    #[cfg(feature = "tmux_2_0")]
    PaneTop(&'a mut Option<usize>),
    /// `pane_tty` - Pseudo terminal of pane
    #[cfg(feature = "tmux_1_6")]
    PaneTty(&'a mut Option<String>),
    /// `pane_unseen_changes` - 1 if there were changes in pane while in mode
    #[cfg(feature = "tmux_3_4")]
    PaneUnseenChanges(&'a mut Option<bool>),
    /// `pane_width` - Width of pane
    #[cfg(feature = "tmux_1_6")]
    PaneWidth(&'a mut Option<usize>),

    /// `saved_cursor_x` - Saved cursor X in pane
    #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
    SavedCursorX(&'a mut Option<usize>),
    /// `saved_cursor_y` - Saved cursor Y in pane
    #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
    SavedCursorY(&'a mut Option<usize>),

    /// `pid` - Server PID
    #[cfg(feature = "tmux_2_1")]
    Pid(&'a mut Option<usize>),
    /// `rectangle_toggle` - 1 if rectangle selection is activated
    #[cfg(feature = "tmux_2_7")]
    RectangleToggle(&'a mut Option<bool>),

    /// `scroll_position` - Scroll position in copy mode
    #[cfg(feature = "tmux_2_2")]
    ScrollPosition(&'a mut Option<usize>),
    /// `scroll_region_lower` - Bottom of scroll region in pane
    #[cfg(feature = "tmux_1_8")]
    ScrollRegionLower(&'a mut Option<usize>),
    /// `scroll_region_upper` - Top of scroll region in pane
    #[cfg(feature = "tmux_1_8")]
    ScrollRegionUpper(&'a mut Option<usize>),

    /// `selection_active` - 1 if selection started and changes with the curso
    #[cfg(feature = "tmux_3_1")]
    SelectionActive(&'a mut Option<bool>),
    /// `selection_end_x` - X position of the end of the selection
    #[cfg(feature = "tmux_3_1")]
    SelectionEndX(&'a mut Option<usize>),
    /// `selection_end_y` - Y position of the end of the selection
    #[cfg(feature = "tmux_3_1")]
    SelectionEndY(&'a mut Option<usize>),
    /// `selection_present` - 1 if selection started in copy mode
    #[cfg(feature = "tmux_2_6")]
    SelectionPresent(&'a mut Option<bool>),
    /// `selection_start_x` - X position of the start of the selection
    #[cfg(feature = "tmux_3_1")]
    SelectionStartX(&'a mut Option<usize>),
    /// `selection_start_y` - Y position of the start of the selection
    #[cfg(feature = "tmux_3_1")]
    SelectionStartY(&'a mut Option<usize>),

    // Session
    /// `session_activity` - Time of session last activity
    #[cfg(feature = "tmux_2_1")]
    SessionActivity(&'a mut Option<usize>),
    /// `session_activity_string` - Option<String> time of session last activity
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    SessionActivityString(&'a mut Option<String>),
    /// `session_alerts` - List of window indexes with alerts
    #[cfg(feature = "tmux_2_1")]
    SessionAlerts(&'a mut Option<String>),
    /// `session_attached` - Number of clients session is attached to
    #[cfg(feature = "tmux_1_6")]
    SessionAttached(&'a mut Option<usize>),
    /// `session_attached_list` - List of clients session is attached to
    #[cfg(feature = "tmux_3_1")]
    SessionAttachedList(&'a mut Option<usize>),
    /// `session_created` - Time session created
    #[cfg(feature = "tmux_1_6")]
    SessionCreated(&'a mut Option<usize>),
    /// `session_created_string` - Option<String> time session created
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    SessionCreatedString(&'a mut Option<String>),
    /// `session_format` - 1 if format is for a session (not assuming the current)
    #[cfg(feature = "tmux_2_6")]
    SessionFormat(&'a mut Option<bool>),
    /// `session_group` - Name of session group
    #[cfg(feature = "tmux_1_6")]
    SessionGroup(&'a mut Option<String>),
    /// `session_group_attached` - Number of clients sessions in group are attached >
    #[cfg(feature = "tmux_3_1")]
    SessionGroupAttached(&'a mut Option<usize>),
    /// `session_group_attached_list` - List of clients sessions in group are attached to
    #[cfg(feature = "tmux_3_1")]
    SessionGroupAttachedList(&'a mut Option<String>),
    /// `session_group_list` - List of sessions in group
    #[cfg(feature = "tmux_2_7")]
    SessionGroupList(&'a mut Option<String>),
    /// `session_group_many_attached` - 1 if multiple clients attached to sessions in gro
    #[cfg(feature = "tmux_3_1")]
    SessionGroupManyAttached(&'a mut Option<bool>),
    /// `session_size` - Size of session group
    #[cfg(feature = "tmux_2_7")]
    SessionGroupSize(&'a mut Option<String>),
    /// `session_grouped` - 1 if session in a group
    #[cfg(feature = "tmux_1_6")]
    SessionGrouped(&'a mut Option<bool>),
    /// `session_height` - Height of session
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    SessionHeight(&'a mut Option<usize>),
    /// `session_width` - Width of session
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    SessionWidth(&'a mut Option<usize>),
    /// `session_id` - Unique session ID
    #[cfg(feature = "tmux_1_8")]
    SessionId(&'a mut Option<usize>),
    /// `session_last_attached` - Time session last attached
    #[cfg(feature = "tmux_2_1")]
    SessionLastAttached(&'a mut Option<usize>),
    /// `session_last_attached_string` - Option<String> time session last attached
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    SessionLastAttachedString(&'a mut Option<String>),
    /// `session_many_attached` - 1 if multiple clients attached
    #[cfg(feature = "tmux_2_0")]
    SessionManyAttached(&'a mut Option<bool>),
    /// `session_name` - #S Name of session
    #[cfg(feature = "tmux_1_6")]
    SessionName(&'a mut Option<String>),
    /// `session_stack` - Window indexes in most recent order
    #[cfg(feature = "tmux_2_5")]
    SessionStack(&'a mut Option<SessionStack>),
    /// `session_windows` - Number of windows in session
    #[cfg(feature = "tmux_1_6")]
    SessionWindows(&'a mut Option<usize>),

    /// `socket_path` - Server socket path
    #[cfg(feature = "tmux_2_2")]
    SocketPath(&'a mut Option<String>),
    /// `start_time` - Server start time
    #[cfg(feature = "tmux_2_2")]
    StartTime(&'a mut Option<usize>),

    /// `version` - Server version
    #[cfg(feature = "tmux_2_4")]
    Version(&'a mut Option<String>),

    // Window
    //
    /// `window_active` - 1 if window active
    #[cfg(feature = "tmux_1_6")]
    WindowActive(&'a mut Option<bool>),
    /// `window_active_clients` - Number of clients viewing this window
    #[cfg(feature = "tmux_3_1")]
    WindowActiveClients(&'a mut Option<usize>),
    /// `window_active_clients_list` - List of clients viewing this window
    #[cfg(feature = "tmux_3_1")]
    WindowActiveClientsList(&'a mut Option<String>),
    /// `window_active_sessions` - Number of sessions on which this window is active
    #[cfg(feature = "tmux_3_1")]
    WindowActiveSessions(&'a mut Option<usize>),
    /// `window_active_sessions_list` - List of sessions on which this window is active
    #[cfg(feature = "tmux_3_1")]
    WindowActiveSessionsList(&'a mut Option<String>),
    /// `window_activity` - Time of window last activity
    #[cfg(feature = "tmux_2_1")]
    WindowActivity(&'a mut Option<usize>),
    /// `window_activity_string` - String time of window last activity
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    WindowActivityString(&'a mut Option<String>),
    /// `window_activity_flag` - 1 if window has activity
    #[cfg(any(
        all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
        feature = "tmux_2_3"
    ))]
    WindowActivityFlag(&'a mut Option<bool>),
    /// `window_bell_flag` - 1 if window has bell
    #[cfg(feature = "tmux_1_9")]
    WindowBellFlag(&'a mut Option<bool>),
    /// `window_content_flag` - 1 if window has content alert
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    WindowContentFlag(&'a mut Option<bool>),
    /// `window_bigger` - 1 if window is larger than client
    #[cfg(feature = "tmux_2_9")]
    WindowBigger(&'a mut Option<bool>),
    /// `window_cell_height` - Height of each cell in pixels
    #[cfg(feature = "tmux_3_1")]
    WindowCellHeight(&'a mut Option<usize>),
    /// `window_cell_width` - Width of each cell in pixels
    #[cfg(feature = "tmux_3_1")]
    WindowCellWidth(&'a mut Option<usize>),
    /// `window_end_flag` - 1 if window has the highest index
    #[cfg(feature = "tmux_2_9")]
    WindowEndFlag(&'a mut Option<bool>),
    /// `window_find_matches` - Matched data from the find-window command if available
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    WindowFindMatches(&'a mut Option<String>),
    /// `window_flags` - #F Window flags
    #[cfg(feature = "tmux_1_6")]
    WindowFlags(&'a mut Option<WindowFlags>),
    // TODO: WindowRawFlags
    /// `window_raw_flags` - Window flags with nothing escaped
    #[cfg(feature = "tmux_3_2")]
    WindowRawFlags(&'a mut Option<WindowFlags>),
    /// `window_format` - 1 if format is for a window
    #[cfg(feature = "tmux_2_6")]
    WindowFormat(&'a mut Option<bool>),
    /// `window_height` - Height of window
    #[cfg(feature = "tmux_1_6")]
    WindowHeight(&'a mut Option<usize>),
    /// `window_id` - Unique window ID
    #[cfg(feature = "tmux_1_7")]
    WindowId(&'a mut Option<usize>),
    /// `window_index` - #I Index of window
    #[cfg(feature = "tmux_1_6")]
    WindowIndex(&'a mut Option<usize>),
    /// `window_last_flag` - 1 if window is the last used
    #[cfg(feature = "tmux_2_0")]
    WindowLastFlag(&'a mut Option<bool>),
    /// `window_layout` - Window layout description, ignoring zoomed window panes
    #[cfg(feature = "tmux_1_6")]
    WindowLayout(&'a mut Option<Layout>),
    /// `window_linked` - 1 if window is linked across sessions
    #[cfg(feature = "tmux_2_1")]
    WindowLinked(&'a mut Option<bool>),
    /// `window_linked_sessions` - Number of sessions this window is linked to
    #[cfg(feature = "tmux_3_1")]
    WindowLinkedSessions(&'a mut Option<usize>),
    /// `window_linked_sessions_list` - List of sessions this window is linked to
    #[cfg(feature = "tmux_3_1")]
    WindowLinkedSessionsList(&'a mut Option<String>),
    /// `window_marked_flag` - 1 if window contains the marked pane
    #[cfg(feature = "tmux_3_1")]
    WindowMarkedFlag(&'a mut Option<bool>),
    /// `window_name` - #W Name of window
    #[cfg(feature = "tmux_1_6")]
    WindowName(&'a mut Option<String>),
    /// `window_offset_x` - X offset into window if larger than client
    #[cfg(feature = "tmux_2_9")]
    WindowOffsetX(&'a mut Option<usize>),
    /// `window_offset_y` - Y offset into window if larger than client
    #[cfg(feature = "tmux_2_9")]
    WindowOffsetY(&'a mut Option<usize>),
    /// `window_panes` - Number of panes in window
    #[cfg(feature = "tmux_1_7")]
    WindowPanes(&'a mut Option<usize>),
    /// `window_silence_flag` - 1 if window has silence alert
    #[cfg(feature = "tmux_1_9")]
    WindowSilenceFlag(&'a mut Option<bool>),
    /// `window_stack_index` - Index in session most recent stack
    #[cfg(feature = "tmux_2_5")]
    WindowStackIndex(&'a mut Option<usize>),
    /// `window_start_flag` - 1 if window has the lowest index
    #[cfg(feature = "tmux_2_9")]
    WindowStartFlag(&'a mut Option<bool>),
    /// `window_visible_layout` - Window layout description, respecting zoomed window panes
    #[cfg(feature = "tmux_2_2")]
    WindowVisibleLayout(&'a mut Option<Layout>),
    /// `window_width` - Width of window
    #[cfg(feature = "tmux_1_6")]
    WindowWidth(&'a mut Option<usize>),
    /// `window_zoomed_flag` - 1 if window is zoomed
    #[cfg(feature = "tmux_2_0")]
    WindowZoomedFlag(&'a mut Option<bool>),

    /// `wrap_flag` - Pane wrap flag
    #[cfg(feature = "tmux_1_8")]
    WrapFlag(&'a mut Option<bool>),
}

impl<'a> VariableOutput<'a> {
    fn parse_option_bool(s: &str) -> Option<bool> {
        s.parse::<usize>().map(|i| i == 1).ok()
    }

    fn parse_option_string(s: &str) -> Option<String> {
        if !s.is_empty() {
            Some(s.to_string())
        } else {
            None
        }
    }

    fn parse_option_usize(s: &str) -> Option<usize> {
        s.parse::<usize>().ok()
    }

    //fn parse_option_duration(s: &str) -> Option<Duration> {
    //s.parse().ok().map(Duration::from_millis)
    //}

    pub fn from_string_ext(s: &str, variable: &mut VariableOutput<'a>) {
        match variable {
            // alternate_on - if pane is in alternate screen
            #[cfg(feature = "tmux_1_8")]
            Self::AlternateOn(v) => **v = s.parse::<usize>().ok(),

            // alternate_saved_x - Saved cursor X in alternate screen
            #[cfg(feature = "tmux_1_8")]
            Self::AlternateSavedX(v) => **v = s.parse::<usize>().ok(),
            // alternate_saved_y - Saved cursor Y in alternate screen
            #[cfg(feature = "tmux_1_8")]
            Self::AlternateSavedY(v) => **v = s.parse::<usize>().ok(),

            // Buffer
            // buffer_created - Time buffer created
            #[cfg(feature = "tmux_2_6")]
            Self::BufferCreated(v) => **v = s.parse::<u128>().ok(),
            // buffer_name - Name of buffer
            #[cfg(feature = "tmux_2_3")]
            Self::BufferName(v) => **v = Self::parse_option_string(s),
            // buffer_sample - First 50 characters from the specified buffer
            #[cfg(feature = "tmux_1_7")]
            Self::BufferSample(v) => **v = Self::parse_option_string(s),
            // buffer_size - Size of the specified buffer in bytes
            #[cfg(feature = "tmux_1_7")]
            Self::BufferSize(v) => **v = s.parse::<usize>().ok(),

            // Client
            // client_activity - Integer time client last had activity
            #[cfg(feature = "tmux_1_6")]
            Self::ClientActivity(v) => **v = s.parse::<u128>().ok(),
            // client_cell_height - Height of each client cell in pixels
            #[cfg(feature = "tmux_3_1")]
            Self::ClientCellHeight(v) => **v = Self::parse_option_usize(s),
            // client_cell_width - Width of each client cell in pixels
            #[cfg(feature = "tmux_3_1")]
            Self::ClientCellWidth(v) => **v = Self::parse_option_usize(s),
            // client_activity_string - Option<String> time client last had activity
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
            Self::ClientActivityString(v) => **v = Self::parse_option_string(s),
            // client_created - Integer time client created
            #[cfg(feature = "tmux_1_6")]
            Self::ClientCreated(v) => **v = s.parse::<u128>().ok(),
            // client_created_string - Option<String> time client created
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
            Self::ClientCreatedString(v) => **v = Self::parse_option_string(s),
            // client_control_mode - 1 if client is in control mode
            #[cfg(feature = "tmux_2_1")]
            Self::ClientControlMode(v) => **v = Self::parse_option_bool(s),
            // client_discarded - Bytes discarded when client behind
            #[cfg(feature = "tmux_2_1")]
            Self::ClientDiscarded(v) => **v = Self::parse_option_string(s),
            // client_cwd - Working directory of client
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            Self::ClientCwd(v) => **v = Self::parse_option_string(s),
            // client_height - Height of client
            #[cfg(feature = "tmux_1_6")]
            Self::ClientHeight(v) => **v = s.parse::<usize>().ok(),
            // client_key_table - Current key table
            #[cfg(feature = "tmux_2_2")]
            Self::ClientKeyTable(v) => **v = Self::parse_option_string(s),
            // client_last_session - Name of the client's last session
            #[cfg(feature = "tmux_1_8")]
            Self::ClientLastSession(v) => **v = Self::parse_option_string(s),
            // client_name - Name of client
            #[cfg(feature = "tmux_2_4")]
            Self::ClientName(v) => **v = Self::parse_option_string(s),
            // client_pid - PID of client process
            #[cfg(feature = "tmux_2_1")]
            Self::ClientPid(v) => **v = s.parse::<usize>().ok(),
            // client_prefix - 1 if prefix key has been pressed
            #[cfg(feature = "tmux_1_8")]
            Self::ClientPrefix(v) => **v = Self::parse_option_bool(s),
            // client_readonly - 1 if client is readonly
            #[cfg(feature = "tmux_1_6")]
            Self::ClientReadonly(v) => **v = Self::parse_option_bool(s),
            // client_session - Name of the client's session
            #[cfg(feature = "tmux_1_8")]
            Self::ClientSession(v) => **v = Self::parse_option_string(s),
            // client_termname - Terminal name of client
            #[cfg(feature = "tmux_1_6")]
            Self::ClientTermname(v) => **v = Self::parse_option_string(s),
            // client_termtype - Terminal type of client
            #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_1")))]
            Self::ClientTermtype(v) => **v = Self::parse_option_string(s),
            // client_tty - Pseudo terminal of client
            #[cfg(feature = "tmux_1_6")]
            Self::ClientTty(v) => **v = Self::parse_option_string(s),
            // client_utf8 - 1 if client supports UTF-8
            #[cfg(feature = "tmux_1_6")]
            Self::ClientUtf8(v) => **v = Self::parse_option_bool(s),
            // client_width - Width of client
            #[cfg(feature = "tmux_1_6")]
            Self::ClientWidth(v) => **v = s.parse::<usize>().ok(),
            // client_written - Bytes written to client
            #[cfg(feature = "tmux_2_4")]
            Self::ClientWritten(v) => **v = s.parse::<usize>().ok(),

            // Command
            // command_hooked - Name of command hooked, if any
            #[cfg(feature = "tmux_2_3")]
            Self::CommandHooked(v) => **v = Self::parse_option_string(s),
            // command_name - Name of command in use, if any
            #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_2_4")))]
            Self::CommandName(v) => **v = Self::parse_option_string(s),
            // command - Name of command in use, if any
            #[cfg(feature = "tmux_2_4")]
            Self::Command(v) => **v = Self::parse_option_string(s),
            // command_list_name - Command name if listing commands
            #[cfg(feature = "tmux_2_3")]
            Self::CommandListName(v) => **v = Self::parse_option_string(s),
            // command_list_alias - Command alias if listing commands
            #[cfg(feature = "tmux_2_3")]
            Self::CommandListAlias(v) => **v = Self::parse_option_string(s),
            // command_list_usage - Command usage if listing commands
            #[cfg(feature = "tmux_2_3")]
            Self::CommandListUsage(v) => **v = Self::parse_option_string(s),

            // Cursor
            // cursor_flag - Pane cursor flag
            #[cfg(feature = "tmux_1_8")]
            Self::CursorFlag(v) => **v = Self::parse_option_string(s),
            // cursor_character - Character at cursor in pane
            #[cfg(feature = "tmux_2_9")]
            Self::CursorCharacter(v) => **v = Self::parse_option_string(s),
            // cursor_x - Cursor X position in pane
            #[cfg(feature = "tmux_1_8")]
            Self::CursorX(v) => **v = s.parse::<usize>().ok(),
            // cursor_y - Cursor Y position in pane
            #[cfg(feature = "tmux_1_8")]
            Self::CursorY(v) => **v = s.parse::<usize>().ok(),

            // copy_cursor_line - Line the cursor is on in copy mode
            #[cfg(feature = "tmux_3_1")]
            Self::CopyCursorLine(v) => **v = Self::parse_option_string(s),
            // copy_cursor_word - Word under cursor in copy mode
            #[cfg(feature = "tmux_3_1")]
            Self::CopyCursorWord(v) => **v = Self::parse_option_string(s),
            // copy_cursor_x - Cursor X position in copy mode
            #[cfg(feature = "tmux_3_1")]
            Self::CopyCursorX(v) => **v = s.parse::<usize>().ok(),
            // copy_cursor_y - Cursor Y position in copy mode
            #[cfg(feature = "tmux_3_1")]
            Self::CopyCursorY(v) => **v = s.parse::<usize>().ok(),
            // current_file - Current configuration file
            #[cfg(feature = "tmux_3_2")]
            Self::CurrentFile(v) => **v = Self::parse_option_string(s),

            // history
            // history_bytes             Number of bytes in window history
            #[cfg(feature = "tmux_1_7")]
            Self::HistotyBytes(v) => **v = s.parse::<usize>().ok(),
            // history_limit             Maximum window history lines
            #[cfg(feature = "tmux_1_7")]
            Self::HistotyLimit(v) => **v = s.parse::<usize>().ok(),
            // history_size              Size of history in bytes
            #[cfg(feature = "tmux_1_7")]
            Self::HistorySize(v) => **v = s.parse::<usize>().ok(),

            // hook
            // hook - Name of running hook, if any
            #[cfg(feature = "tmux_2_4")]
            Self::Hook(v) => **v = Self::parse_option_string(s),
            // hook_pane - ID of pane where hook was run, if any
            #[cfg(feature = "tmux_2_4")]
            Self::HookPane(v) => **v = s.parse::<usize>().ok(),
            // hook_session - ID of session where hook was run, if any
            #[cfg(feature = "tmux_2_4")]
            Self::HookSession(v) => **v = s.parse::<usize>().ok(),
            // hook_session_name - Name of session where hook was run, if any
            #[cfg(feature = "tmux_2_4")]
            Self::HookSessionName(v) => **v = Self::parse_option_string(s),
            // hook_window - ID of window where hook was run, if any
            #[cfg(feature = "tmux_2_4")]
            Self::HookWindow(v) => **v = s.parse::<usize>().ok(),
            // hook_window_name - Name of window where hook was run, if any
            #[cfg(feature = "tmux_2_4")]
            Self::HookWindowName(v) => **v = Self::parse_option_string(s),

            // host
            // host - Hostname of local host
            #[cfg(feature = "tmux_1_6")]
            Self::Host(v) => **v = Self::parse_option_string(s),
            // host_short - #h Hostname of local host (no domain name)
            #[cfg(feature = "tmux_1_9")]
            Self::HostShort(v) => **v = Self::parse_option_string(s),

            // insert_flag - Pane insert flag
            #[cfg(feature = "tmux_1_8")]
            Self::InsertFlag(v) => **v = Self::parse_option_string(s),
            // keypad_cursor_flag - Pane keypad cursor flag
            #[cfg(feature = "tmux_1_8")]
            Self::KeypadCursorFlag(v) => **v = Self::parse_option_string(s),
            // keypad_flag - Pane keypad flag
            #[cfg(feature = "tmux_1_8")]
            Self::KeypadFlag(v) => **v = Self::parse_option_string(s),

            // line - Line number in the list
            #[cfg(feature = "tmux_1_6")]
            Self::Line(v) => **v = s.parse::<usize>().ok(),

            // mouse_all_flag - Pane mouse all flag
            // #[cfg(feature = "tmux_3_0")]
            // Self::MouseAllFlag(v) => **v = Self::parse_option_string(s),
            // mouse_any_flag - Pane mouse any flag
            #[cfg(feature = "tmux_1_8")]
            Self::MouseAnyFlag(v) => **v = Self::parse_option_string(s),
            // mouse_button_flag - Pane mouse button flag
            #[cfg(feature = "tmux_1_8")]
            Self::MouseButtonFlag(v) => **v = Self::parse_option_string(s),
            // mouse_line - Line under mouse, if any
            #[cfg(feature = "tmux_3_0")]
            Self::MouseLine(v) => **v = Self::parse_option_string(s),
            // sgr_flag - Pane mouse SGR flag
            #[cfg(feature = "tmux_3_0")]
            Self::MouseSgrFlag(v) => **v = Self::parse_option_string(s),
            // mouse_standard_flag - Pane mouse standard flag
            #[cfg(feature = "tmux_1_8")]
            Self::MouseStandardFlag(v) => **v = Self::parse_option_string(s),
            // mouse_status_line - Status line on which mouse event took place
            #[cfg(feature = "tmux_3_4")]
            Self::MouseStatusLine(v) => **v = Self::parse_option_string(s),
            // mouse_status_range - Range type or argument of mouse event on status line
            #[cfg(feature = "tmux_3_4")]
            Self::MouseStatusRange(v) => **v = Self::parse_option_string(s),
            // mouse_utf8_flag - Pane mouse UTF-8 flag
            #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_2"), feature = "tmux_3_0"))]
            Self::MouseUtf8Flag(v) => **v = Self::parse_option_string(s),
            // mouse_all_flag - Pane mouse all flag
            #[cfg(feature = "tmux_2_4")]
            Self::MouseAllFlag(v) => **v = Self::parse_option_string(s),
            // mouse_word - Word under mouse, if any
            #[cfg(feature = "tmux_3_0")]
            Self::MouseWord(v) => **v = Self::parse_option_string(s),
            // mouse_x - Mouse X position, if any
            #[cfg(feature = "tmux_3_0")]
            Self::MouseX(v) => **v = Self::parse_option_usize(s),
            // mouse_y - Mouse Y position, if any
            #[cfg(feature = "tmux_3_0")]
            Self::MouseY(v) => **v = Self::parse_option_usize(s),
            // origin_flag - Pane origin flag
            #[cfg(feature = "tmux_3_0")]
            Self::OriginFlag(v) => **v = Self::parse_option_string(s),

            // pane
            // pane_active - 1 if active pane
            #[cfg(feature = "tmux_1_6")]
            Self::PaneActive(v) => **v = Self::parse_option_bool(s),
            // pane_at_bottom - 1 if pane is at the bottom of window
            #[cfg(feature = "tmux_2_6")]
            Self::PaneAtBottom(v) => **v = Self::parse_option_bool(s),
            // pane_at_left - 1 if pane is at the left of window
            #[cfg(feature = "tmux_2_6")]
            Self::PaneAtLeft(v) => **v = Self::parse_option_bool(s),
            // 1 if pane is at the right of window
            #[cfg(feature = "tmux_2_6")]
            Self::PaneAtRight(v) => **v = Self::parse_option_bool(s),
            // 1 if pane is at the top of window
            #[cfg(feature = "tmux_2_6")]
            Self::PaneAtTop(v) => **v = Self::parse_option_bool(s),
            // pane_bottom - Bottom of pane
            #[cfg(feature = "tmux_2_0")]
            Self::PaneBottom(v) => **v = s.parse::<usize>().ok(),
            // pane_current_command - Current command if available
            #[cfg(feature = "tmux_1_8")]
            Self::PaneCurrentCommand(v) => **v = Self::parse_option_string(s),
            // pane_current_path - Current path if available
            #[cfg(feature = "tmux_1_7")]
            Self::PaneCurrentPath(v) => **v = Self::parse_option_string(s),
            // pane_dead - 1 if pane is dead
            #[cfg(feature = "tmux_1_6")]
            Self::PaneDead(v) => **v = Self::parse_option_bool(s),
            // pane_dead_status - Exit status of process in dead pane
            #[cfg(feature = "tmux_2_0")]
            Self::PaneDeadStatus(v) => **v = s.parse::<usize>().ok(),
            // 1 if format is for a pane
            #[cfg(feature = "tmux_2_6")]
            Self::PaneFormat(v) => **v = Self::parse_option_bool(s),
            // pane_height - Height of pane
            #[cfg(feature = "tmux_1_6")]
            Self::PaneHeight(v) => **v = s.parse::<usize>().ok(),
            // pane_id - #D Unique pane ID
            #[cfg(feature = "tmux_1_6")]
            Self::PaneId(v) => **v = s[1..].parse::<usize>().ok(),
            // pane_in_mode - 1 if pane is in a mode
            #[cfg(feature = "tmux_1_8")]
            Self::PaneInMode(v) => **v = Self::parse_option_bool(s),
            // pane_index - #P Index of pane
            #[cfg(feature = "tmux_1_7")]
            Self::PaneIndex(v) => **v = s.parse::<usize>().ok(),
            // pane_input_off - 1 if input to pane is disabled
            #[cfg(feature = "tmux_2_0")]
            Self::PaneInputOff(v) => **v = Self::parse_option_bool(s),
            // pane_left - Left of pane
            #[cfg(feature = "tmux_2_0")]
            Self::PaneLeft(v) => **v = s.parse::<usize>().ok(),
            // pane_marked - 1 if this is the marked pane
            #[cfg(feature = "tmux_3_0")]
            Self::PaneMarked(v) => **v = Self::parse_option_bool(s),
            // pane_marked_set - 1 if a marked pane is set
            #[cfg(feature = "tmux_3_0")]
            Self::PaneMarkedSet(v) => **v = Self::parse_option_bool(s),
            // pane_mode - Name of pane mode, if any
            #[cfg(feature = "tmux_2_5")]
            Self::PaneMode(v) => **v = s.parse::<usize>().ok(),
            // pane_path - #T Path of pane (can be set by application)
            #[cfg(feature = "tmux_3_1")]
            Self::PanePath(v) => **v = Self::parse_option_string(s),
            // pane_pid - PID of first process in pane
            #[cfg(feature = "tmux_1_6")]
            Self::PanePid(v) => **v = s.parse::<usize>().ok(),
            // pane_pipe - 1 if pane is being piped
            #[cfg(feature = "tmux_2_6")]
            Self::PanePipe(v) => **v = Self::parse_option_bool(s),
            // pane_right - Right of pane
            #[cfg(feature = "tmux_2_0")]
            Self::PaneRight(v) => **v = s.parse::<usize>().ok(),
            // Last search Option<String> in copy mode
            #[cfg(feature = "tmux_2_5")]
            Self::PaneSearchString(v) => **v = s.parse::<usize>().ok(),
            // pane_start_command - Command pane started with
            #[cfg(feature = "tmux_1_6")]
            Self::PaneStartCommand(v) => **v = s.parse::<usize>().ok(),
            // pane_start_path - Path pane started with
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_0")))]
            Self::PaneStartPath(v) => **v = Self::parse_option_usize(s),
            // pane_synchronized - 1 if pane is synchronized
            #[cfg(feature = "tmux_1_9")]
            Self::PaneSynchronized(v) => **v = Self::parse_option_bool(s),
            // pane_tabs - Pane tab positions
            #[cfg(feature = "tmux_1_8")]
            Self::PaneTabs(v) => **v = s.parse::<PaneTabs>().ok(),
            // pane_title - #T Title of pane (can be set by application)
            #[cfg(feature = "tmux_1_6")]
            Self::PaneTitle(v) => **v = Self::parse_option_string(s),
            // pane_top - Top of pane
            #[cfg(feature = "tmux_2_0")]
            Self::PaneTop(v) => **v = s.parse::<usize>().ok(),
            // pane_tty - Pseudo terminal of pane
            #[cfg(feature = "tmux_1_6")]
            Self::PaneTty(v) => **v = Self::parse_option_string(s),
            // `pane_unseen_changes` - 1 if there were changes in pane while in mode
            #[cfg(feature = "tmux_3_4")]
            Self::PaneUnseenChanges(v) => **v = Self::parse_option_bool(s),
            // pane_width - Width of pane
            #[cfg(feature = "tmux_1_6")]
            Self::PaneWidth(v) => **v = s.parse::<usize>().ok(),

            // saved_cursor_x - Saved cursor X in pane
            #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
            Self::SavedCursorX(v) => **v = s.parse::<usize>().ok(),
            // saved_cursor_y - Saved cursor Y in pane
            #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
            Self::SavedCursorY(v) => **v = s.parse::<usize>().ok(),

            // pid - Server PID
            #[cfg(feature = "tmux_2_1")]
            Self::Pid(v) => **v = s.parse::<usize>().ok(),
            // rectangle_toggle - 1 if rectangle selection is activated
            #[cfg(feature = "tmux_2_7")]
            Self::RectangleToggle(v) => **v = Self::parse_option_bool(s),

            // scroll_position - Scroll position in copy mode
            #[cfg(feature = "tmux_2_2")]
            Self::ScrollPosition(v) => **v = s.parse::<usize>().ok(),
            // scroll_region_lower - Bottom of scroll region in pane
            #[cfg(feature = "tmux_1_8")]
            Self::ScrollRegionLower(v) => **v = s.parse::<usize>().ok(),
            // scroll_region_upper - Top of scroll region in pane
            #[cfg(feature = "tmux_1_8")]
            Self::ScrollRegionUpper(v) => **v = s.parse::<usize>().ok(),

            // selection_active - 1 if selection started and changes with the curso
            #[cfg(feature = "tmux_3_1")]
            Self::SelectionActive(v) => **v = Self::parse_option_bool(s),
            // selection_end_x - X position of the end of the selection
            #[cfg(feature = "tmux_3_1")]
            Self::SelectionEndX(v) => **v = Self::parse_option_usize(s),
            // selection_end_y - Y position of the end of the selection
            #[cfg(feature = "tmux_3_1")]
            Self::SelectionEndY(v) => **v = Self::parse_option_usize(s),
            // selection_present - 1 if selection started in copy mode
            #[cfg(feature = "tmux_2_6")]
            Self::SelectionPresent(v) => **v = Self::parse_option_bool(s),
            // selection_start_x - X position of the start of the selection
            #[cfg(feature = "tmux_3_1")]
            Self::SelectionStartX(v) => **v = Self::parse_option_usize(s),
            // selection_start_y - Y position of the start of the selection
            #[cfg(feature = "tmux_3_1")]
            Self::SelectionStartY(v) => **v = Self::parse_option_usize(s),

            // Session
            // session_activity - Time of session last activity
            #[cfg(feature = "tmux_2_1")]
            Self::SessionActivity(v) => **v = Self::parse_option_usize(s),
            // session_activity_string - Option<String> time of session last activity
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
            Self::SessionActivityString(v) => **v = Self::parse_option_string(s),
            // session_alerts - List of window indexes with alerts
            #[cfg(feature = "tmux_2_1")]
            Self::SessionAlerts(v) => **v = Self::parse_option_string(s),
            // session_attached - Number of clients session is attached to
            #[cfg(feature = "tmux_1_6")]
            Self::SessionAttached(v) => **v = s.parse::<usize>().ok(),
            // session_attached_list - List of clients session is attached to
            #[cfg(feature = "tmux_3_1")]
            Self::SessionAttachedList(v) => **v = Self::parse_option_usize(s),
            // session_created - Time session created
            #[cfg(feature = "tmux_1_6")]
            Self::SessionCreated(v) => **v = Self::parse_option_usize(s),
            // Option<String> time session created
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
            Self::SessionCreatedString(v) => **v = Self::parse_option_string(s),
            // session_format - 1 if format is for a session (not assuming the current)
            #[cfg(feature = "tmux_2_6")]
            Self::SessionFormat(v) => **v = Self::parse_option_bool(s),
            // session_group - Name of session group
            #[cfg(feature = "tmux_1_6")]
            Self::SessionGroup(v) => **v = Self::parse_option_string(s),
            // session_group_attached - Number of clients sessions in group are attached >
            #[cfg(feature = "tmux_3_1")]
            Self::SessionGroupAttached(v) => **v = Self::parse_option_usize(s),
            // session_group_attached_list - List of clients sessions in group are attached to
            #[cfg(feature = "tmux_3_1")]
            Self::SessionGroupAttachedList(v) => **v = Self::parse_option_string(s),
            // session_group_list - List of sessions in group
            #[cfg(feature = "tmux_2_7")]
            Self::SessionGroupList(v) => **v = Self::parse_option_string(s),
            // session_group_many_attached - 1 if multiple clients attached to sessions in gro
            #[cfg(feature = "tmux_3_1")]
            Self::SessionGroupManyAttached(v) => **v = Self::parse_option_bool(s),
            // session_size - Size of session group
            #[cfg(feature = "tmux_2_7")]
            Self::SessionGroupSize(v) => **v = Self::parse_option_string(s),
            // session_grouped - 1 if session in a group
            #[cfg(feature = "tmux_1_6")]
            Self::SessionGrouped(v) => **v = Self::parse_option_bool(s),
            // session_height - Height of session
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
            Self::SessionHeight(v) => **v = s.parse::<usize>().ok(),
            // session_width - Width of session
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
            Self::SessionWidth(v) => **v = s.parse::<usize>().ok(),
            // session_id - Unique session ID
            #[cfg(feature = "tmux_1_8")]
            Self::SessionId(v) => **v = s[1..].parse::<usize>().ok(), // skip $ char
            // session_last_attached - Time session last attached
            #[cfg(feature = "tmux_2_1")]
            Self::SessionLastAttached(v) => **v = Self::parse_option_usize(s),
            // session_last_attached_string - Option<String> time session last attached
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
            Self::SessionLastAttachedString(v) => **v = Self::parse_option_string(s),
            // session_many_attached - 1 if multiple clients attached
            #[cfg(feature = "tmux_2_0")]
            Self::SessionManyAttached(v) => **v = Self::parse_option_bool(s),
            // session_name - #S Name of session
            #[cfg(feature = "tmux_1_6")]
            Self::SessionName(v) => **v = Self::parse_option_string(s),
            // session_stack - Window indexes in most recent order
            #[cfg(feature = "tmux_2_5")]
            Self::SessionStack(v) => **v = s.parse::<SessionStack>().ok(),
            // session_windows - Number of windows in session
            #[cfg(feature = "tmux_1_6")]
            Self::SessionWindows(v) => **v = s.parse::<usize>().ok(),

            // socket_path - Server socket path
            #[cfg(feature = "tmux_2_2")]
            Self::SocketPath(v) => **v = Self::parse_option_string(s),
            // start_time - Server start time
            #[cfg(feature = "tmux_2_2")]
            Self::StartTime(v) => **v = Self::parse_option_usize(s),

            // version - Server version
            #[cfg(feature = "tmux_2_4")]
            Self::Version(v) => **v = Self::parse_option_string(s),

            // Window
            //
            // window_active - 1 if window active
            #[cfg(feature = "tmux_1_6")]
            Self::WindowActive(v) => **v = Self::parse_option_bool(s),
            // window_active_clients - Number of clients viewing this window
            #[cfg(feature = "tmux_3_1")]
            Self::WindowActiveClients(v) => **v = Self::parse_option_usize(s),
            // window_active_clients_list - List of clients viewing this window
            #[cfg(feature = "tmux_3_1")]
            Self::WindowActiveClientsList(v) => **v = Self::parse_option_string(s),
            // window_active_sessions - Number of sessions on which this window is active
            #[cfg(feature = "tmux_3_1")]
            Self::WindowActiveSessions(v) => **v = Self::parse_option_usize(s),
            // window_active_sessions_list - List of sessions on which this window is active
            #[cfg(feature = "tmux_3_1")]
            Self::WindowActiveSessionsList(v) => **v = Self::parse_option_string(s),
            // window_activity - Time of window last activity
            #[cfg(feature = "tmux_2_1")]
            Self::WindowActivity(v) => **v = s.parse::<usize>().ok(),
            // window_activity_string - String time of window last activity
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
            Self::WindowActivityString(v) => **v = Self::parse_option_string(s),
            // window_activity_flag - 1 if window has activity
            #[cfg(any(
                all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
                feature = "tmux_2_3"
            ))]
            Self::WindowActivityFlag(v) => **v = Self::parse_option_bool(s),
            // window_bell_flag - 1 if window has bell
            #[cfg(feature = "tmux_1_9")]
            Self::WindowBellFlag(v) => **v = Self::parse_option_bool(s),
            // window_content_flag - 1 if window has content alert
            #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
            Self::WindowContentFlag(v) => **v = Self::parse_option_bool(s),
            // window_bigger - 1 if window is larger than client
            #[cfg(feature = "tmux_2_9")]
            Self::WindowBigger(v) => **v = Self::parse_option_bool(s),
            // window_cell_height - Height of each cell in pixels
            #[cfg(feature = "tmux_3_1")]
            Self::WindowCellHeight(v) => **v = Self::parse_option_usize(s),
            // window_cell_width - Width of each cell in pixels
            #[cfg(feature = "tmux_3_1")]
            Self::WindowCellWidth(v) => **v = Self::parse_option_usize(s),
            // window_end_flag - 1 if window has the highest index
            #[cfg(feature = "tmux_2_9")]
            Self::WindowEndFlag(v) => **v = Self::parse_option_bool(s),
            // window_find_matches - Matched data from the find-window command if available
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
            Self::WindowFindMatches(v) => **v = Self::parse_option_string(s),
            // window_flags - #F Window flags
            #[cfg(feature = "tmux_1_6")]
            Self::WindowFlags(v) => **v = s.parse::<WindowFlags>().ok(),
            // window_raw_flags - Window flags with nothing escaped
            #[cfg(feature = "tmux_3_2")]
            Self::WindowRawFlags(v) => **v = s.parse::<WindowFlags>().ok(),
            // window_format - 1 if format is for a window
            #[cfg(feature = "tmux_2_6")]
            Self::WindowFormat(v) => **v = Self::parse_option_bool(s),
            // window_height - Height of window
            #[cfg(feature = "tmux_1_6")]
            Self::WindowHeight(v) => **v = s.parse::<usize>().ok(),
            // window_id - Unique window ID
            #[cfg(feature = "tmux_1_7")]
            Self::WindowId(v) => **v = s[1..].parse::<usize>().ok(),
            // window_index - #I Index of window
            #[cfg(feature = "tmux_1_6")]
            Self::WindowIndex(v) => **v = s.parse::<usize>().ok(),
            // window_last_flag - 1 if window is the last used
            #[cfg(feature = "tmux_2_0")]
            Self::WindowLastFlag(v) => **v = Self::parse_option_bool(s),
            // window_layout - Window layout description, ignoring zoomed window panes
            #[cfg(feature = "tmux_1_6")]
            Self::WindowLayout(v) => **v = s.parse::<Layout>().ok(),
            // window_linked - 1 if window is linked across sessions
            #[cfg(feature = "tmux_2_1")]
            Self::WindowLinked(v) => **v = Self::parse_option_bool(s),
            // window_linked_sessions - Number of sessions this window is linked to
            #[cfg(feature = "tmux_3_1")]
            Self::WindowLinkedSessions(v) => **v = Self::parse_option_usize(s),
            // window_linked_sessions_list - List of sessions this window is linked to
            #[cfg(feature = "tmux_3_1")]
            Self::WindowLinkedSessionsList(v) => **v = Self::parse_option_string(s),
            // window_marked_flag - 1 if window contains the marked pane
            #[cfg(feature = "tmux_3_1")]
            Self::WindowMarkedFlag(v) => **v = Self::parse_option_bool(s),
            // window_name - #W Name of window
            #[cfg(feature = "tmux_1_6")]
            Self::WindowName(v) => **v = Self::parse_option_string(s),
            // window_offset_x - X offset into window if larger than client
            #[cfg(feature = "tmux_2_9")]
            Self::WindowOffsetX(v) => **v = Self::parse_option_usize(s),
            // window_offset_y - Y offset into window if larger than client
            #[cfg(feature = "tmux_2_9")]
            Self::WindowOffsetY(v) => **v = Self::parse_option_usize(s),
            // window_panes - Number of panes in window
            #[cfg(feature = "tmux_1_7")]
            Self::WindowPanes(v) => **v = s.parse::<usize>().ok(),
            // window_silence_flag - 1 if window has silence alert
            #[cfg(feature = "tmux_1_9")]
            Self::WindowSilenceFlag(v) => **v = Self::parse_option_bool(s),
            // window_stack_index - Index in session most recent stack
            #[cfg(feature = "tmux_2_5")]
            Self::WindowStackIndex(v) => **v = s.parse::<usize>().ok(),
            // window_start_flag - 1 if window has the lowest index
            #[cfg(feature = "tmux_2_9")]
            Self::WindowStartFlag(v) => **v = Self::parse_option_bool(s),
            // window_visible_layout - Window layout description, respecting zoomed window panes
            #[cfg(feature = "tmux_2_2")]
            Self::WindowVisibleLayout(v) => **v = s.parse::<Layout>().ok(),
            // window_width - Width of window
            #[cfg(feature = "tmux_1_6")]
            Self::WindowWidth(v) => **v = s.parse::<usize>().ok(),
            // window_zoomed_flag - 1 if window is zoomed
            #[cfg(feature = "tmux_2_0")]
            Self::WindowZoomedFlag(v) => **v = Self::parse_option_bool(s),

            // wrap_flag - Pane wrap flag
            #[cfg(feature = "tmux_1_8")]
            Self::WrapFlag(v) => **v = Self::parse_option_bool(s),
            // other
            //_ => Self::Error,
        }
    }
}
