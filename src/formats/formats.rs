use crate::formats::variable::Variable;
use std::fmt;

#[derive(Debug)]
pub struct Formats {
    // XXX: string or char, join(), split() ?
    pub separator: char,
    pub variables: Vec<Variable>,
}

impl Default for Formats {
    fn default() -> Self {
        Formats {
            separator: '\'',
            variables: Vec::new(),
        }
    }
}

impl fmt::Display for Formats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = self
            .variables
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(&self.separator.to_string());
        write!(f, "{}", output)
    }
}

impl Formats {
    pub fn new() -> Self {
        Default::default()
    }

    /// set separator character
    pub fn separator(&mut self, c: char) -> &mut Self {
        self.separator = c;
        self
    }

    /// append with variable
    pub fn push(&mut self, variable: Variable) {
        self.variables.push(variable)
    }

    // TODO: check vec same size, return type?
    // XXX: mb from_string for default format too?
    // pub fn custom_string(String) pub fn custom_usize(String)

    // tmux variables

    /// `alternate_on` - if pane is in alternate screen
    #[cfg(feature = "tmux_1_8")]
    pub fn alternate_on(&mut self) -> &mut Self {
        self.push(Variable::AlternateOn);
        self
    }

    /// `alternate_saved_x` - Saved cursor X in alternate screen
    #[cfg(feature = "tmux_1_8")]
    pub fn alternate_saved_x(&mut self) -> &mut Self {
        self.push(Variable::AlternateSavedX);
        self
    }

    /// `alternate_saved_y` - Saved cursor Y in alternate screen
    #[cfg(feature = "tmux_1_8")]
    pub fn alternate_saved_y(&mut self) -> &mut Self {
        self.push(Variable::AlternateSavedY);
        self
    }

    // Buffer

    /// `buffer_created` - Time buffer created
    #[cfg(feature = "tmux_2_6")]
    pub fn buffer_created(&mut self) -> &mut Self {
        self.push(Variable::BufferCreated);
        self
    }

    /// `buffer_name` - Name of buffer
    #[cfg(feature = "tmux_2_3")]
    pub fn buffer_name(&mut self) -> &mut Self {
        self.push(Variable::BufferName);
        self
    }

    /// `buffer_sample` - First 50 characters from the specified buffer
    #[cfg(feature = "tmux_1_7")]
    pub fn buffer_sample(&mut self) -> &mut Self {
        self.push(Variable::BufferSample);
        self
    }

    /// `buffer_size` - Size of the specified buffer in bytes
    #[cfg(feature = "tmux_1_7")]
    pub fn buffer_size(&mut self) -> &mut Self {
        self.push(Variable::BufferSize);
        self
    }

    // Client
    /// `client_activity` - Integer time client last had activity
    #[cfg(feature = "tmux_1_6")]
    pub fn client_activity(&mut self) -> &mut Self {
        self.push(Variable::ClientActivity);
        self
    }

    /// `client_cell_height` - Height of each client cell in pixels
    #[cfg(feature = "tmux_3_1")]
    pub fn client_cell_height(&mut self) -> &mut Self {
        self.push(Variable::ClientCellHeight);
        self
    }

    /// `client_cell_width` - Width of each client cell in pixels
    #[cfg(feature = "tmux_3_1")]
    pub fn client_cell_width(&mut self) -> &mut Self {
        self.push(Variable::ClientCellWidth);
        self
    }

    /// `client_activity_string` - Option<String> time client last had activity
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    pub fn client_activity_string(&mut self) -> &mut Self {
        self.push(Variable::ClientActivityString);
        self
    }

    /// `client_created` - Integer time client created
    #[cfg(feature = "tmux_1_6")]
    pub fn client_created(&mut self) -> &mut Self {
        self.push(Variable::ClientCreated);
        self
    }

    /// `client_created_string` - Option<String> time client created
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    pub fn client_created_string(&mut self) -> &mut Self {
        self.push(Variable::ClientCreatedString);
        self
    }

    /// `client_control_mode` - 1 if client is in control mode
    #[cfg(feature = "tmux_2_1")]
    pub fn client_control_mode(&mut self) -> &mut Self {
        self.push(Variable::ClientControlMode);
        self
    }

    /// `client_discarded` - Bytes discarded when client behind
    #[cfg(feature = "tmux_2_1")]
    pub fn client_discarded(&mut self) -> &mut Self {
        self.push(Variable::ClientDiscarded);
        self
    }

    /// `client_cwd` - Working directory of client
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn client_cwd(&mut self) -> &mut Self {
        self.push(Variable::ClientCwd);
        self
    }

    /// `client_height` - Height of client
    #[cfg(feature = "tmux_1_6")]
    pub fn client_height(&mut self) -> &mut Self {
        self.push(Variable::ClientHeight);
        self
    }

    /// `client_key_table` - Current key table
    #[cfg(feature = "tmux_2_2")]
    pub fn client_key_table(&mut self) -> &mut Self {
        self.push(Variable::ClientKeyTable);
        self
    }

    /// `client_last_session` - Name of the client's last session
    #[cfg(feature = "tmux_1_8")]
    pub fn client_last_session(&mut self) -> &mut Self {
        self.push(Variable::ClientLastSession);
        self
    }

    /// `client_name` - Name of client
    #[cfg(feature = "tmux_2_4")]
    pub fn client_name(&mut self) -> &mut Self {
        self.push(Variable::ClientName);
        self
    }

    /// `client_pid` - PID of client process
    #[cfg(feature = "tmux_2_1")]
    pub fn client_pid(&mut self) -> &mut Self {
        self.push(Variable::ClientPid);
        self
    }

    /// `client_prefix` - 1 if prefix key has been pressed
    #[cfg(feature = "tmux_1_8")]
    pub fn client_prefix(&mut self) -> &mut Self {
        self.push(Variable::ClientPrefix);
        self
    }

    /// `client_readonly` - 1 if client is readonly
    #[cfg(feature = "tmux_1_6")]
    pub fn client_readonly(&mut self) -> &mut Self {
        self.push(Variable::ClientReadonly);
        self
    }

    /// `client_session` - Name of the client's session
    #[cfg(feature = "tmux_1_8")]
    pub fn client_session(&mut self) -> &mut Self {
        self.push(Variable::ClientSession);
        self
    }

    /// `client_termname` - Terminal name of client
    #[cfg(feature = "tmux_1_6")]
    pub fn client_termname(&mut self) -> &mut Self {
        self.push(Variable::ClientTermname);
        self
    }

    /// `client_termtype` - Terminal type of client
    #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_1")))]
    pub fn client_termtype(&mut self) -> &mut Self {
        self.push(Variable::ClientTermtype);
        self
    }

    /// `client_tty` - Pseudo terminal of client
    #[cfg(feature = "tmux_1_6")]
    pub fn client_tty(&mut self) -> &mut Self {
        self.push(Variable::ClientTty);
        self
    }

    /// `client_utf8` - 1 if client supports UTF-8
    #[cfg(feature = "tmux_1_6")]
    pub fn client_utf8(&mut self) -> &mut Self {
        self.push(Variable::ClientUtf8);
        self
    }

    /// `client_width` - Width of client
    #[cfg(feature = "tmux_1_6")]
    pub fn client_width(&mut self) -> &mut Self {
        self.push(Variable::ClientWidth);
        self
    }

    /// `client_written` - Bytes written to client
    #[cfg(feature = "tmux_2_4")]
    pub fn client_written(&mut self) -> &mut Self {
        self.push(Variable::ClientWritten);
        self
    }

    // Command

    /// `command_hooked` - Name of command hooked, if any
    #[cfg(feature = "tmux_2_3")]
    pub fn command_hooked(&mut self) -> &mut Self {
        self.push(Variable::CommandHooked);
        self
    }

    /// `command_name` - Name of command in use, if any
    #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_2_4")))]
    pub fn command_name(&mut self) -> &mut Self {
        self.push(Variable::CommandName);
        self
    }

    /// `command` - Name of command in use, if any
    #[cfg(feature = "tmux_2_4")]
    pub fn command(&mut self) -> &mut Self {
        self.push(Variable::Command);
        self
    }

    /// `command_list_name` - Command name if listing commands
    #[cfg(feature = "tmux_2_3")]
    pub fn command_list_name(&mut self) -> &mut Self {
        self.push(Variable::CommandListName);
        self
    }

    /// `command_list_alias` - Command alias if listing commands
    #[cfg(feature = "tmux_2_3")]
    pub fn command_list_alias(&mut self) -> &mut Self {
        self.push(Variable::CommandListAlias);
        self
    }

    /// `command_list_usage` - Command usage if listing commands
    #[cfg(feature = "tmux_2_3")]
    pub fn command_list_usage(&mut self) -> &mut Self {
        self.push(Variable::CommandListUsage);
        self
    }

    // Cursor

    /// `cursor_flag` - Pane cursor flag
    #[cfg(feature = "tmux_1_8")]
    pub fn cursor_flag(&mut self) -> &mut Self {
        self.push(Variable::CursorFlag);
        self
    }

    /// `cursor_character` - Character at cursor in pane
    #[cfg(feature = "tmux_2_9")]
    pub fn cursor_character(&mut self) -> &mut Self {
        self.push(Variable::CursorCharacter);
        self
    }

    /// `cursor_x` - Cursor X position in pane
    #[cfg(feature = "tmux_1_8")]
    pub fn cursor_x(&mut self) -> &mut Self {
        self.push(Variable::CursorX);
        self
    }

    /// `cursor_y` - Cursor Y position in pane
    #[cfg(feature = "tmux_1_8")]
    pub fn cursor_y(&mut self) -> &mut Self {
        self.push(Variable::CursorY);
        self
    }

    /// `copy_cursor_line` - Line the cursor is on in copy mode
    #[cfg(feature = "tmux_3_1")]
    pub fn copy_cursor_line(&mut self) -> &mut Self {
        self.push(Variable::CopyCursorLine);
        self
    }

    /// `copy_cursor_word` - Word under cursor in copy mode
    #[cfg(feature = "tmux_3_1")]
    pub fn copy_cursor_word(&mut self) -> &mut Self {
        self.push(Variable::CopyCursorWord);
        self
    }

    /// `copy_cursor_x` - Cursor X position in copy mode
    #[cfg(feature = "tmux_3_1")]
    pub fn copy_cursor_x(&mut self) -> &mut Self {
        self.push(Variable::CopyCursorX);
        self
    }

    /// `copy_cursor_y` - Cursor Y position in copy mode
    #[cfg(feature = "tmux_3_1")]
    pub fn copy_cursor_y(&mut self) -> &mut Self {
        self.push(Variable::CopyCursorY);
        self
    }

    /// `current_file` - Current configuration file
    #[cfg(feature = "tmux_3_2")]
    pub fn current_file(&mut self) -> &mut Self {
        self.push(Variable::CurrentFile);
        self
    }

    // history

    /// `history_bytes`             Number of bytes in window history
    #[cfg(feature = "tmux_1_7")]
    pub fn histoty_bytes(&mut self) -> &mut Self {
        self.push(Variable::HistotyBytes);
        self
    }

    /// `history_limit`             Maximum window history lines
    #[cfg(feature = "tmux_1_7")]
    pub fn history_limit(&mut self) -> &mut Self {
        self.push(Variable::HistotyLimit);
        self
    }

    /// `history_size`              Size of history in bytes
    #[cfg(feature = "tmux_1_7")]
    pub fn history_size(&mut self) -> &mut Self {
        self.push(Variable::HistorySize);
        self
    }

    // hook

    /// `hook` - Name of running hook, if any
    #[cfg(feature = "tmux_2_4")]
    pub fn hook(&mut self) -> &mut Self {
        self.push(Variable::Hook);
        self
    }

    /// `hook_pane` - ID of pane where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    pub fn hook_pane(&mut self) -> &mut Self {
        self.push(Variable::HookPane);
        self
    }

    /// `hook_session` - ID of session where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    pub fn hook_session(&mut self) -> &mut Self {
        self.push(Variable::HookSession);
        self
    }

    /// `hook_session_name` - Name of session where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    pub fn hook_session_name(&mut self) -> &mut Self {
        self.push(Variable::HookSessionName);
        self
    }

    /// `hook_window` - ID of window where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    pub fn hook_window(&mut self) -> &mut Self {
        self.push(Variable::HookWindow);
        self
    }

    /// `hook_window_name` - Name of window where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    pub fn hook_window_name(&mut self) -> &mut Self {
        self.push(Variable::HookWindowName);
        self
    }

    // host

    /// `host` - Hostname of local host
    #[cfg(feature = "tmux_1_6")]
    pub fn host(&mut self) -> &mut Self {
        self.push(Variable::Host);
        self
    }

    /// `host_short` - #h Hostname of local host (no domain name)
    #[cfg(feature = "tmux_1_9")]
    pub fn host_short(&mut self) -> &mut Self {
        self.push(Variable::HostShort);
        self
    }

    /// `insert_flag` - Pane insert flag
    #[cfg(feature = "tmux_1_8")]
    pub fn insert_flag(&mut self) -> &mut Self {
        self.push(Variable::InsertFlag);
        self
    }

    /// `keypad_cursor_flag` - Pane keypad cursor flag
    #[cfg(feature = "tmux_1_8")]
    pub fn keypad_cursor_flag(&mut self) -> &mut Self {
        self.push(Variable::KeypadCursorFlag);
        self
    }

    /// `keypad_flag` - Pane keypad flag
    #[cfg(feature = "tmux_1_8")]
    pub fn keypad_flag(&mut self) -> &mut Self {
        self.push(Variable::KeypadFlag);
        self
    }

    /// `line` - Line number in the list
    #[cfg(feature = "tmux_1_6")]
    pub fn line(&mut self) -> &mut Self {
        self.push(Variable::Line);
        self
    }

    // `mouse_all_flag` - Pane mouse all flag
    //#[cfg(feature = "tmux_3_0")]
    //pub fn mouse_all_flag(&mut self) -> &mut Self {
    //self.push(Variable::MouseAllFlag);
    //self
    //}

    /// `mouse_any_flag` - Pane mouse any flag
    #[cfg(feature = "tmux_1_8")]
    pub fn mouse_any_flag(&mut self) -> &mut Self {
        self.push(Variable::MouseAnyFlag);
        self
    }

    /// `mouse_button_flag` - Pane mouse button flag
    #[cfg(feature = "tmux_1_8")]
    pub fn mouse_button_flag(&mut self) -> &mut Self {
        self.push(Variable::MouseButtonFlag);
        self
    }

    /// `mouse_line` - Line under mouse, if any
    #[cfg(feature = "tmux_3_0")]
    pub fn mouse_line(&mut self) -> &mut Self {
        self.push(Variable::MouseLine);
        self
    }

    /// `sgr_flag` - Pane mouse SGR flag
    #[cfg(feature = "tmux_3_0")]
    pub fn sgr_line(&mut self) -> &mut Self {
        self.push(Variable::MouseSgrFlag);
        self
    }

    /// `mouse_standard_flag` - Pane mouse standard flag
    #[cfg(feature = "tmux_1_8")]
    pub fn mouse_standard_flag(&mut self) -> &mut Self {
        self.push(Variable::MouseStandardFlag);
        self
    }

    /// `mouse_utf8_flag` - Pane mouse UTF-8 flag
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_2"), feature = "tmux_3_0"))]
    pub fn mouse_utf8_flag(&mut self) -> &mut Self {
        self.push(Variable::MouseUtf8Flag);
        self
    }

    /// `mouse_all_flag` - Pane mouse all flag
    #[cfg(feature = "tmux_2_4")]
    pub fn mouse_all_flag(&mut self) -> &mut Self {
        self.push(Variable::MouseAllFlag);
        self
    }

    /// `mouse_word` - Word under mouse, if any
    #[cfg(feature = "tmux_3_0")]
    pub fn mouse_word(&mut self) -> &mut Self {
        self.push(Variable::MouseWord);
        self
    }

    /// `mouse_x` - Mouse X position, if any
    #[cfg(feature = "tmux_3_0")]
    pub fn mouse_x(&mut self) -> &mut Self {
        self.push(Variable::MouseX);
        self
    }

    /// `mouse_y` - Mouse Y position, if any
    #[cfg(feature = "tmux_3_0")]
    pub fn mouse_y(&mut self) -> &mut Self {
        self.push(Variable::MouseY);
        self
    }

    /// `origin_flag` - Pane origin flag
    #[cfg(feature = "tmux_3_0")]
    pub fn origin_flag(&mut self) -> &mut Self {
        self.push(Variable::OriginFlag);
        self
    }

    // pane

    /// `pane_active` - 1 if active pane
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_active(&mut self) -> &mut Self {
        self.push(Variable::PaneActive);
        self
    }

    /// `pane_at_bottom` - 1 if pane is at the bottom of window
    #[cfg(feature = "tmux_2_6")]
    pub fn pane_at_bottom(&mut self) -> &mut Self {
        self.push(Variable::PaneAtBottom);
        self
    }

    /// `pane_at_left` - 1 if pane is at the left of window
    #[cfg(feature = "tmux_2_6")]
    pub fn pane_at_left(&mut self) -> &mut Self {
        self.push(Variable::PaneAtLeft);
        self
    }

    /// `pane_at_right` - 1 if pane is at the right of window
    #[cfg(feature = "tmux_2_6")]
    pub fn pane_at_right(&mut self) -> &mut Self {
        self.push(Variable::PaneAtRight);
        self
    }

    /// `pane_at_top` - 1 if pane is at the top of window
    #[cfg(feature = "tmux_2_6")]
    pub fn pane_at_top(&mut self) -> &mut Self {
        self.push(Variable::PaneAtTop);
        self
    }

    /// `pane_bottom` - Bottom of pane
    #[cfg(feature = "tmux_2_0")]
    pub fn pane_bottom(&mut self) -> &mut Self {
        self.push(Variable::PaneBottom);
        self
    }

    /// `pane_current_command` - Current command if available
    #[cfg(feature = "tmux_1_8")]
    pub fn pane_current_command(&mut self) -> &mut Self {
        self.push(Variable::PaneCurrentCommand);
        self
    }

    /// `pane_current_path` - Current path if available
    #[cfg(feature = "tmux_1_7")]
    pub fn pane_current_path(&mut self) -> &mut Self {
        self.push(Variable::PaneCurrentPath);
        self
    }

    /// `pane_dead` - 1 if pane is dead
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_dead(&mut self) -> &mut Self {
        self.push(Variable::PaneDead);
        self
    }

    /// `pane_dead_status` - Exit status of process in dead pane
    #[cfg(feature = "tmux_2_0")]
    pub fn pane_dead_status(&mut self) -> &mut Self {
        self.push(Variable::PaneDeadStatus);
        self
    }

    /// `pane_format` - 1 if format is for a pane
    #[cfg(feature = "tmux_2_6")]
    pub fn pane_format(&mut self) -> &mut Self {
        self.push(Variable::PaneFormat);
        self
    }

    /// `pane_height` - Height of pane
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_height(&mut self) -> &mut Self {
        self.push(Variable::PaneHeight);
        self
    }

    /// `pane_id` - #D Unique pane ID
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_id(&mut self) -> &mut Self {
        self.push(Variable::PaneId);
        self
    }

    /// `pane_in_mode` - 1 if pane is in a mode
    #[cfg(feature = "tmux_1_8")]
    pub fn pane_in_mode(&mut self) -> &mut Self {
        self.push(Variable::PaneInMode);
        self
    }

    /// `pane_index` - #P Index of pane
    #[cfg(feature = "tmux_1_7")]
    pub fn pane_index(&mut self) -> &mut Self {
        self.push(Variable::PaneIndex);
        self
    }

    /// `pane_input_off` - 1 if input to pane is disabled
    #[cfg(feature = "tmux_2_0")]
    pub fn pane_input_off(&mut self) -> &mut Self {
        self.push(Variable::PaneInputOff);
        self
    }

    /// `pane_left` - Left of pane
    #[cfg(feature = "tmux_2_0")]
    pub fn pane_left(&mut self) -> &mut Self {
        self.push(Variable::PaneLeft);
        self
    }

    /// `pane_marked` - 1 if this is the marked pane
    #[cfg(feature = "tmux_3_0")]
    pub fn pane_marked(&mut self) -> &mut Self {
        self.push(Variable::PaneMarked);
        self
    }

    /// `pane_marked_set` - 1 if a marked pane is set
    #[cfg(feature = "tmux_3_0")]
    pub fn pane_marked_set(&mut self) -> &mut Self {
        self.push(Variable::PaneMarkedSet);
        self
    }

    /// `pane_mode` - Name of pane mode, if any
    #[cfg(feature = "tmux_2_5")]
    pub fn pane_mode(&mut self) -> &mut Self {
        self.push(Variable::PaneMode);
        self
    }

    /// `pane_path` - #T Path of pane (can be set by application)
    #[cfg(feature = "tmux_3_1")]
    pub fn pane_path(&mut self) -> &mut Self {
        self.push(Variable::PanePath);
        self
    }

    /// `pane_pid` - PID of first process in pane
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_pid(&mut self) -> &mut Self {
        self.push(Variable::PanePid);
        self
    }

    /// `pane_pipe` - 1 if pane is being piped
    #[cfg(feature = "tmux_2_6")]
    pub fn pane_pipe(&mut self) -> &mut Self {
        self.push(Variable::PanePipe);
        self
    }

    /// `pane_right` - Right of pane
    #[cfg(feature = "tmux_2_0")]
    pub fn pane_right(&mut self) -> &mut Self {
        self.push(Variable::PaneRight);
        self
    }

    /// `pane_search_string` - Last search Option<String> in copy mode
    #[cfg(feature = "tmux_2_5")]
    pub fn pane_search_string(&mut self) -> &mut Self {
        self.push(Variable::PaneSearchString);
        self
    }

    /// `pane_start_command` - Command pane started with
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_start_command(&mut self) -> &mut Self {
        self.push(Variable::PaneStartCommand);
        self
    }

    /// `pane_start_path` - Path pane started with
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_0")))]
    pub fn pane_start_path(&mut self) -> &mut Self {
        self.push(Variable::PaneStartPath);
        self
    }

    /// `pane_synchronized` - 1 if pane is synchronized
    #[cfg(feature = "tmux_1_9")]
    pub fn pane_synchronized(&mut self) -> &mut Self {
        self.push(Variable::PaneSynchronized);
        self
    }

    /// `pane_tabs` - Pane tab positions
    #[cfg(feature = "tmux_1_8")]
    pub fn pane_tabs(&mut self) -> &mut Self {
        self.push(Variable::PaneTabs);
        self
    }

    /// `pane_title` - #T Title of pane (can be set by application)
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_title(&mut self) -> &mut Self {
        self.push(Variable::PaneTitle);
        self
    }

    /// `pane_top` - Top of pane
    #[cfg(feature = "tmux_2_0")]
    pub fn pane_top(&mut self) -> &mut Self {
        self.push(Variable::PaneTop);
        self
    }

    /// `pane_tty` - Pseudo terminal of pane
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_tty(&mut self) -> &mut Self {
        self.push(Variable::PaneTty);
        self
    }

    /// `pane_width` - Width of pane
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_width(&mut self) -> &mut Self {
        self.push(Variable::PaneWidth);
        self
    }

    /// `saved_cursor_x` - Saved cursor X in pane
    #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
    pub fn saved_cursor_x(&mut self) -> &mut Self {
        self.push(Variable::SavedCursorX);
        self
    }

    /// `saved_cursor_y` - Saved cursor Y in pane
    #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
    pub fn saved_cursor_y(&mut self) -> &mut Self {
        self.push(Variable::SavedCursorY);
        self
    }

    /// `pid` - Server PID
    #[cfg(feature = "tmux_2_1")]
    pub fn pid(&mut self) -> &mut Self {
        self.push(Variable::Pid);
        self
    }

    /// `rectangle_toggle` - 1 if rectangle selection is activated
    #[cfg(feature = "tmux_2_7")]
    pub fn rectangle_toggle(&mut self) -> &mut Self {
        self.push(Variable::RectangleToggle);
        self
    }

    /// `scroll_position` - Scroll position in copy mode
    #[cfg(feature = "tmux_2_2")]
    pub fn scroll_position(&mut self) -> &mut Self {
        self.push(Variable::ScrollPosition);
        self
    }

    /// `scroll_region_lower` - Bottom of scroll region in pane
    #[cfg(feature = "tmux_1_8")]
    pub fn scroll_region_lower(&mut self) -> &mut Self {
        self.push(Variable::ScrollRegionLower);
        self
    }

    /// `scroll_region_upper` - Top of scroll region in pane
    #[cfg(feature = "tmux_1_8")]
    pub fn scroll_region_upper(&mut self) -> &mut Self {
        self.push(Variable::ScrollRegionUpper);
        self
    }

    /// `selection_active` - 1 if selection started and changes with the curso
    #[cfg(feature = "tmux_3_1")]
    pub fn selection_active(&mut self) -> &mut Self {
        self.push(Variable::SelectionActive);
        self
    }

    /// `selection_end_x` - X position of the end of the selection
    #[cfg(feature = "tmux_3_1")]
    pub fn selection_end_x(&mut self) -> &mut Self {
        self.push(Variable::SelectionEndX);
        self
    }

    /// `selection_end_y` - Y position of the end of the selection
    #[cfg(feature = "tmux_3_1")]
    pub fn selection_end_y(&mut self) -> &mut Self {
        self.push(Variable::SelectionEndY);
        self
    }

    /// `selection_present` - 1 if selection started in copy mode
    #[cfg(feature = "tmux_2_6")]
    pub fn selection_present(&mut self) -> &mut Self {
        self.push(Variable::SelectionPresent);
        self
    }

    /// `selection_start_x` - X position of the start of the selection
    #[cfg(feature = "tmux_3_1")]
    pub fn selection_start_x(&mut self) -> &mut Self {
        self.push(Variable::SelectionStartX);
        self
    }

    /// `selection_start_y` - Y position of the start of the selection
    #[cfg(feature = "tmux_3_1")]
    pub fn selection_start_y(&mut self) -> &mut Self {
        self.push(Variable::SelectionStartY);
        self
    }

    // Session
    /// `session_activity` - Time of session last activity
    #[cfg(feature = "tmux_2_1")]
    pub fn session_activity(&mut self) -> &mut Self {
        self.push(Variable::SessionActivity);
        self
    }

    /// `session_activity_string` - Option<String> time of session last activity
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub fn session_activity_string(&mut self) -> &mut Self {
        self.push(Variable::SessionActivityString);
        self
    }

    /// `session_alerts` - List of window indexes with alerts
    #[cfg(feature = "tmux_2_1")]
    pub fn session_alerts(&mut self) -> &mut Self {
        self.push(Variable::SessionAlerts);
        self
    }

    /// `session_attached` - Number of clients session is attached to
    #[cfg(feature = "tmux_1_6")]
    pub fn session_attached(&mut self) -> &mut Self {
        self.push(Variable::SessionAttached);
        self
    }

    /// `session_attached_list` - List of clients session is attached to
    #[cfg(feature = "tmux_3_1")]
    pub fn session_attached_list(&mut self) -> &mut Self {
        self.push(Variable::SessionAttachedList);
        self
    }

    /// `session_created` - Time session created
    #[cfg(feature = "tmux_1_6")]
    pub fn session_created(&mut self) -> &mut Self {
        self.push(Variable::SessionCreated);
        self
    }

    /// `session_created_string` - Option<String> time session created
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    pub fn session_created_string(&mut self) -> &mut Self {
        self.push(Variable::SessionCreatedString);
        self
    }

    /// `session_format` - 1 if format is for a session (not assuming the current)
    #[cfg(feature = "tmux_2_6")]
    pub fn session_format(&mut self) -> &mut Self {
        self.push(Variable::SessionFormat);
        self
    }

    /// `session_group` - Name of session group
    #[cfg(feature = "tmux_1_6")]
    pub fn session_group(&mut self) -> &mut Self {
        self.push(Variable::SessionGroup);
        self
    }

    /// `session_group_attached` - Number of clients sessions in group are attached >
    #[cfg(feature = "tmux_3_1")]
    pub fn session_group_attached(&mut self) -> &mut Self {
        self.push(Variable::SessionGroupAttached);
        self
    }

    /// `session_group_attached_list` - List of clients sessions in group are attached to
    #[cfg(feature = "tmux_3_1")]
    pub fn session_group_attached_list(&mut self) -> &mut Self {
        self.push(Variable::SessionGroupAttachedList);
        self
    }

    /// `session_group_list` - List of sessions in group
    #[cfg(feature = "tmux_2_7")]
    pub fn session_group_list(&mut self) -> &mut Self {
        self.push(Variable::SessionGroupList);
        self
    }

    /// `session_group_many_attached` - 1 if multiple clients attached to sessions in gro
    #[cfg(feature = "tmux_3_1")]
    pub fn session_group_many_attached(&mut self) -> &mut Self {
        self.push(Variable::SessionGroupManyAttached);
        self
    }

    /// `session_group_size` - Size of session group
    #[cfg(feature = "tmux_2_7")]
    pub fn session_group_size(&mut self) -> &mut Self {
        self.push(Variable::SessionGroupSize);
        self
    }

    /// `session_grouped` - 1 if session in a group
    #[cfg(feature = "tmux_1_6")]
    pub fn session_grouped(&mut self) -> &mut Self {
        self.push(Variable::SessionGrouped);
        self
    }

    /// `session_height` - Height of session
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    pub fn session_height(&mut self) -> &mut Self {
        self.push(Variable::SessionHeight);
        self
    }

    /// `session_width` - Width of session
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    pub fn session_width(&mut self) -> &mut Self {
        self.push(Variable::SessionWidth);
        self
    }

    /// `session_id` - Unique session ID
    #[cfg(feature = "tmux_1_8")]
    pub fn session_id(&mut self) -> &mut Self {
        self.push(Variable::SessionId);
        self
    }

    /// `session_last_attached` - Time session last attached
    #[cfg(feature = "tmux_2_1")]
    pub fn session_last_attached(&mut self) -> &mut Self {
        self.push(Variable::SessionLastAttached);
        self
    }

    /// `session_last_attached_string` - Option<String> time session last attached
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub fn session_last_attached_string(&mut self) -> &mut Self {
        self.push(Variable::SessionLastAttachedString);
        self
    }

    /// `session_many_attached` - 1 if multiple clients attached
    #[cfg(feature = "tmux_2_0")]
    pub fn session_many_attached(&mut self) -> &mut Self {
        self.push(Variable::SessionManyAttached);
        self
    }

    /// `session_name` - #S Name of session
    #[cfg(feature = "tmux_1_6")]
    pub fn session_name(&mut self) -> &mut Self {
        self.push(Variable::SessionName);
        self
    }

    /// `session_stack` - Window indexes in most recent order
    #[cfg(feature = "tmux_2_5")]
    pub fn session_stack(&mut self) -> &mut Self {
        self.push(Variable::SessionStack);
        self
    }

    /// `session_windows` - Number of windows in session
    #[cfg(feature = "tmux_1_6")]
    pub fn session_windows(&mut self) -> &mut Self {
        self.push(Variable::SessionWindows);
        self
    }

    /// `socket_path` - Server socket path
    #[cfg(feature = "tmux_2_2")]
    pub fn socket_path(&mut self) -> &mut Self {
        self.push(Variable::SocketPath);
        self
    }

    /// `start_time` - Server start time
    #[cfg(feature = "tmux_2_2")]
    pub fn start_time(&mut self) -> &mut Self {
        self.push(Variable::StartTime);
        self
    }

    /// `version` - Server version
    #[cfg(feature = "tmux_2_4")]
    pub fn version(&mut self) -> &mut Self {
        self.push(Variable::Version);
        self
    }

    // Window
    //
    /// `window_active` - 1 if window active
    #[cfg(feature = "tmux_1_6")]
    pub fn window_active(&mut self) -> &mut Self {
        self.push(Variable::WindowActive);
        self
    }

    /// `window_active_clients` - Number of clients viewing this window
    #[cfg(feature = "tmux_3_1")]
    pub fn window_active_clients(&mut self) -> &mut Self {
        self.push(Variable::WindowActiveClients);
        self
    }

    /// `window_active_clients_list` - List of clients viewing this window
    #[cfg(feature = "tmux_3_1")]
    pub fn window_active_clients_list(&mut self) -> &mut Self {
        self.push(Variable::WindowActiveClientsList);
        self
    }

    /// `window_active_sessions` - Number of sessions on which this window is active
    #[cfg(feature = "tmux_3_1")]
    pub fn window_active_sessions(&mut self) -> &mut Self {
        self.push(Variable::WindowActiveSessions);
        self
    }

    /// `window_active_sessions_list` - List of sessions on which this window is active
    #[cfg(feature = "tmux_3_1")]
    pub fn window_active_sessions_list(&mut self) -> &mut Self {
        self.push(Variable::WindowActiveSessionsList);
        self
    }

    /// `window_activity` - Time of window last activity
    #[cfg(feature = "tmux_2_1")]
    pub fn window_activity(&mut self) -> &mut Self {
        self.push(Variable::WindowActivity);
        self
    }

    /// `window_activity_string` - String time of window last activity
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub fn window_activity_string(&mut self) -> &mut Self {
        self.push(Variable::SessionActivityString);
        self
    }

    /// `window_activity_flag` - 1 if window has activity
    #[cfg(any(
        all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
        feature = "tmux_2_3"
    ))]
    pub fn window_activity_flag(&mut self) -> &mut Self {
        self.push(Variable::WindowActivityFlag);
        self
    }

    /// `window_bell_flag` - 1 if window has bell
    #[cfg(feature = "tmux_1_9")]
    pub fn window_bell_flag(&mut self) -> &mut Self {
        self.push(Variable::WindowBellFlag);
        self
    }

    /// `window_content_flag` - 1 if window has content alert
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub fn window_content_flag(&mut self) -> &mut Self {
        self.push(Variable::WindowContentFlag);
        self
    }

    /// `window_bigger` - 1 if window is larger than client
    #[cfg(feature = "tmux_2_9")]
    pub fn window_bigger(&mut self) -> &mut Self {
        self.push(Variable::WindowBigger);
        self
    }

    /// `window_cell_height` - Height of each cell in pixels
    #[cfg(feature = "tmux_3_1")]
    pub fn window_cell_height(&mut self) -> &mut Self {
        self.push(Variable::WindowCellHeight);
        self
    }

    /// `window_cell_width` - Width of each cell in pixels
    #[cfg(feature = "tmux_3_1")]
    pub fn window_cell_width(&mut self) -> &mut Self {
        self.push(Variable::WindowCellWidth);
        self
    }

    /// `window_end_flag` - 1 if window has the highest index
    #[cfg(feature = "tmux_2_9")]
    pub fn window_end_flag(&mut self) -> &mut Self {
        self.push(Variable::WindowEndFlag);
        self
    }

    /// `window_find_matches` - Matched data from the find-window command if available
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub fn window_find_matches(&mut self) -> &mut Self {
        self.push(Variable::WindowFindMatches);
        self
    }

    /// `window_flags` - #F Window flags
    #[cfg(feature = "tmux_1_6")]
    pub fn window_flags(&mut self) -> &mut Self {
        self.push(Variable::WindowFlags);
        self
    }

    // TODO: WindowRawFlags
    /// `window_raw_flags` - Window flags with nothing escaped
    #[cfg(feature = "tmux_3_2")]
    pub fn window_raw_flags(&mut self) -> &mut Self {
        self.push(Variable::WindowRawFlags);
        self
    }

    /// `window_format` - 1 if format is for a window
    #[cfg(feature = "tmux_2_6")]
    pub fn window_format(&mut self) -> &mut Self {
        self.push(Variable::WindowFormat);
        self
    }

    /// `window_height` - Height of window
    #[cfg(feature = "tmux_1_6")]
    pub fn window_height(&mut self) -> &mut Self {
        self.push(Variable::WindowHeight);
        self
    }

    /// `window_id` - Unique window ID
    #[cfg(feature = "tmux_1_7")]
    pub fn window_id(&mut self) -> &mut Self {
        self.push(Variable::WindowId);
        self
    }

    /// `window_index` - #I Index of window
    #[cfg(feature = "tmux_1_6")]
    pub fn window_index(&mut self) -> &mut Self {
        self.push(Variable::WindowIndex);
        self
    }

    /// `window_last_flag` - 1 if window is the last used
    #[cfg(feature = "tmux_2_0")]
    pub fn window_last_flag(&mut self) -> &mut Self {
        self.push(Variable::WindowLastFlag);
        self
    }

    /// `window_layout` - Window layout description, ignoring zoomed window panes
    #[cfg(feature = "tmux_1_6")]
    pub fn window_layout(&mut self) -> &mut Self {
        self.push(Variable::WindowLayout);
        self
    }

    /// `window_linked` - 1 if window is linked across sessions
    #[cfg(feature = "tmux_2_1")]
    pub fn window_linked(&mut self) -> &mut Self {
        self.push(Variable::WindowLinked);
        self
    }

    /// `window_linked_sessions` - Number of sessions this window is linked to
    #[cfg(feature = "tmux_3_1")]
    pub fn window_linked_sessions(&mut self) -> &mut Self {
        self.push(Variable::WindowLinkedSessions);
        self
    }

    /// `window_linked_sessions_list` - List of sessions this window is linked to
    #[cfg(feature = "tmux_3_1")]
    pub fn window_linked_sessions_list(&mut self) -> &mut Self {
        self.push(Variable::WindowLinkedSessionsList);
        self
    }

    /// `window_marked_flag` - 1 if window contains the marked pane
    #[cfg(feature = "tmux_3_1")]
    pub fn window_marked_flag(&mut self) -> &mut Self {
        self.push(Variable::WindowMarkedFlag);
        self
    }

    /// `window_name` - #W Name of window
    #[cfg(feature = "tmux_1_6")]
    pub fn window_name(&mut self) -> &mut Self {
        self.push(Variable::WindowName);
        self
    }

    /// `window_offset_x` - X offset into window if larger than client
    #[cfg(feature = "tmux_2_9")]
    pub fn window_offset_x(&mut self) -> &mut Self {
        self.push(Variable::WindowOffsetX);
        self
    }

    /// `window_offset_y` - Y offset into window if larger than client
    #[cfg(feature = "tmux_2_9")]
    pub fn window_offset_y(&mut self) -> &mut Self {
        self.push(Variable::WindowOffsetY);
        self
    }

    /// `window_panes` - Number of panes in window
    #[cfg(feature = "tmux_1_7")]
    pub fn window_panes(&mut self) -> &mut Self {
        self.push(Variable::WindowPanes);
        self
    }

    /// `window_silence_flag` - 1 if window has silence alert
    #[cfg(feature = "tmux_1_9")]
    pub fn window_silence_flag(&mut self) -> &mut Self {
        self.push(Variable::WindowSilenceFlag);
        self
    }

    /// `window_stack_index` - Index in session most recent stack
    #[cfg(feature = "tmux_2_5")]
    pub fn window_stack_index(&mut self) -> &mut Self {
        self.push(Variable::WindowStackIndex);
        self
    }

    /// `window_start_flag` - 1 if window has the lowest index
    #[cfg(feature = "tmux_2_9")]
    pub fn window_start_flag(&mut self) -> &mut Self {
        self.push(Variable::WindowStartFlag);
        self
    }

    /// `window_visible_layout` - Window layout description, respecting zoomed window panes
    #[cfg(feature = "tmux_2_2")]
    pub fn window_visible_layout(&mut self) -> &mut Self {
        self.push(Variable::WindowVisibleLayout);
        self
    }

    /// `window_width` - Width of window
    #[cfg(feature = "tmux_1_6")]
    pub fn window_width(&mut self) -> &mut Self {
        self.push(Variable::WindowWidth);
        self
    }

    /// `window_zoomed_flag` - 1 if window is zoomed
    #[cfg(feature = "tmux_2_0")]
    pub fn window_zoomed_flag(&mut self) -> &mut Self {
        self.push(Variable::WindowZoomedFlag);
        self
    }

    /// `wrap_flag` - Pane wrap flag
    #[cfg(feature = "tmux_1_8")]
    pub fn wrap_flag(&mut self) -> &mut Self {
        self.push(Variable::WrapFlag);
        self
    }
}
