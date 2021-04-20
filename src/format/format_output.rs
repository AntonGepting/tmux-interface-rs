use crate::format::variable_output::VariableOutput;
use crate::Layout;
use crate::PaneTabs;
use crate::SessionStack;

#[derive(Debug)]
pub struct FormatOutput<'a> {
    pub separator: char,
    pub variables: Vec<VariableOutput<'a>>,
}

impl<'a> Default for FormatOutput<'a> {
    fn default() -> Self {
        FormatOutput {
            separator: '\'',
            variables: Vec::new(),
        }
    }
}

impl<'a> FormatOutput<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// set separator character
    pub fn separator(&mut self, c: char) -> &mut Self {
        self.separator = c;
        self
    }

    /// append with variable
    pub fn push(&mut self, variable: VariableOutput<'a>) {
        self.variables.push(variable)
    }

    // TODO: check vec same size, return type?
    // XXX: mb from_string for default format too?
    pub fn from_string_ext(s: &str, format: &'a mut FormatOutput<'a>) {
        let v: Vec<&str> = s.split(format.separator).collect();
        for (i, variable) in v.iter().enumerate() {
            VariableOutput::from_string_ext(variable, &mut format.variables[i]);
        }
    }

    // pub fn custom_string(String) pub fn custom_usize(String)

    // tmux variables

    /// `alternate_on` - if pane is in alternate screen
    #[cfg(feature = "tmux_1_8")]
    pub fn alternate_on(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::AlternateOn(v));
        self
    }

    /// `alternate_saved_x` - Saved cursor X in alternate screen
    #[cfg(feature = "tmux_1_8")]
    pub fn alternate_saved_x(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::AlternateSavedX(v));
        self
    }

    /// `alternate_saved_y` - Saved cursor Y in alternate screen
    #[cfg(feature = "tmux_1_8")]
    pub fn alternate_saved_y(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::AlternateSavedY(v));
        self
    }

    // Buffer

    /// `buffer_created` - Time buffer created
    #[cfg(feature = "tmux_2_6")]
    pub fn buffer_created(&mut self, v: &'a mut Option<u128>) -> &mut Self {
        self.push(VariableOutput::BufferCreated(v));
        self
    }

    /// `buffer_name` - Name of buffer
    #[cfg(feature = "tmux_2_3")]
    pub fn buffer_name(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::BufferName(v));
        self
    }

    /// `buffer_sample` - First 50 characters from the specified buffer
    #[cfg(feature = "tmux_1_7")]
    pub fn buffer_sample(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::BufferSample(v));
        self
    }

    /// `buffer_size` - Size of the specified buffer in bytes
    #[cfg(feature = "tmux_1_7")]
    pub fn buffer_size(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::BufferSize(v));
        self
    }

    // Client
    /// `client_activity` - Integer time client last had activity
    #[cfg(feature = "tmux_1_6")]
    pub fn client_activity(&mut self, v: &'a mut Option<u128>) -> &mut Self {
        self.push(VariableOutput::ClientActivity(v));
        self
    }

    /// `client_cell_height` - Height of each client cell in pixels
    #[cfg(feature = "tmux_3_1")]
    pub fn client_cell_height(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::ClientCellHeight(v));
        self
    }

    /// `client_cell_width` - Width of each client cell in pixels
    #[cfg(feature = "tmux_3_1")]
    pub fn client_cell_width(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::ClientCellWidth(v));
        self
    }

    /// `client_activity_string` - Option<String> time client last had activity
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    pub fn client_activity_string(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::ClientActivityString(v));
        self
    }

    /// `client_created` - Integer time client created
    #[cfg(feature = "tmux_1_6")]
    pub fn client_created(&mut self, v: &'a mut Option<u128>) -> &mut Self {
        self.push(VariableOutput::ClientCreated(v));
        self
    }

    /// `client_created_string` - Option<String> time client created
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    pub fn client_created_string(&mut self, v: &'a mut Option<u128>) -> &mut Self {
        self.push(VariableOutput::ClientCreatedString(v));
        self
    }

    /// `client_control_mode` - 1 if client is in control mode
    #[cfg(feature = "tmux_2_1")]
    pub fn client_control_mode(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::ClientControlMode(v));
        self
    }

    /// `client_discarded` - Bytes discarded when client behind
    #[cfg(feature = "tmux_2_1")]
    pub fn client_discarded(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::ClientDiscarded(v));
        self
    }

    /// `client_cwd` - Working directory of client
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn client_cwd(&mut self, v: &'a mut Option<u128>) -> &mut Self {
        self.push(VariableOutput::ClientCwd(v));
        self
    }

    /// `client_height` - Height of client
    #[cfg(feature = "tmux_1_6")]
    pub fn client_height(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::ClientHeight(v));
        self
    }

    /// `client_key_table` - Current key table
    #[cfg(feature = "tmux_2_2")]
    pub fn client_key_table(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::ClientKeyTable(v));
        self
    }

    /// `client_last_session` - Name of the client's last session
    #[cfg(feature = "tmux_1_8")]
    pub fn client_last_session(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::ClientLastSession(v));
        self
    }

    /// `client_name` - Name of client
    #[cfg(feature = "tmux_2_4")]
    pub fn client_name(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::ClientName(v));
        self
    }

    /// `client_pid` - PID of client process
    #[cfg(feature = "tmux_2_1")]
    pub fn client_pid(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::ClientPid(v));
        self
    }

    /// `client_prefix` - 1 if prefix key has been pressed
    #[cfg(feature = "tmux_1_8")]
    pub fn client_prefix(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::ClientPrefix(v));
        self
    }

    /// `client_readonly` - 1 if client is readonly
    #[cfg(feature = "tmux_1_6")]
    pub fn client_readonly(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::ClientReadonly(v));
        self
    }

    /// `client_session` - Name of the client's session
    #[cfg(feature = "tmux_1_8")]
    pub fn client_session(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::ClientSession(v));
        self
    }

    /// `client_termname` - Terminal name of client
    #[cfg(feature = "tmux_1_6")]
    pub fn client_termname(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::ClientTermname(v));
        self
    }

    /// `client_termtype` - Terminal type of client
    #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_1")))]
    pub fn client_termtype(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::ClientTermtype(v));
        self
    }

    /// `client_tty` - Pseudo terminal of client
    #[cfg(feature = "tmux_1_6")]
    pub fn client_tty(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::ClientTty(v));
        self
    }

    /// `client_utf8` - 1 if client supports UTF-8
    #[cfg(feature = "tmux_1_6")]
    pub fn client_utf8(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::ClientUtf8(v));
        self
    }

    /// `client_width` - Width of client
    #[cfg(feature = "tmux_1_6")]
    pub fn client_width(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::ClientWidth(v));
        self
    }

    /// `client_written` - Bytes written to client
    #[cfg(feature = "tmux_2_4")]
    pub fn client_written(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::ClientWritten(v));
        self
    }

    // Command

    /// `command_hooked` - Name of command hooked, if any
    #[cfg(feature = "tmux_2_3")]
    pub fn command_hooked(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::CommandHooked(v));
        self
    }

    /// `command_name` - Name of command in use, if any
    #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_2_4")))]
    pub fn command_name(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::CommandName(v));
        self
    }

    /// `command` - Name of command in use, if any
    #[cfg(feature = "tmux_2_4")]
    pub fn command(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::Command(v));
        self
    }

    /// `command_list_name` - Command name if listing commands
    #[cfg(feature = "tmux_2_3")]
    pub fn command_list_name(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::CommandListName(v));
        self
    }

    /// `command_list_alias` - Command alias if listing commands
    #[cfg(feature = "tmux_2_3")]
    pub fn command_list_alias(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::CommandListAlias(v));
        self
    }

    /// `command_list_usage` - Command usage if listing commands
    #[cfg(feature = "tmux_2_3")]
    pub fn command_list_usage(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::CommandListUsage(v));
        self
    }

    // Cursor

    /// `cursor_flag` - Pane cursor flag
    #[cfg(feature = "tmux_1_8")]
    pub fn cursor_flag(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::CursorFlag(v));
        self
    }

    /// `cursor_character` - Character at cursor in pane
    #[cfg(feature = "tmux_2_9")]
    pub fn cursor_character(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::CursorCharacter(v));
        self
    }

    /// `cursor_x` - Cursor X position in pane
    #[cfg(feature = "tmux_1_8")]
    pub fn cursor_x(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::CursorX(v));
        self
    }

    /// `cursor_y` - Cursor Y position in pane
    #[cfg(feature = "tmux_1_8")]
    pub fn cursor_y(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::CursorY(v));
        self
    }

    /// `copy_cursor_line` - Line the cursor is on in copy mode
    #[cfg(feature = "tmux_3_1")]
    pub fn copy_cursor_line(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::CopyCursorLine(v));
        self
    }

    /// `copy_cursor_word` - Word under cursor in copy mode
    #[cfg(feature = "tmux_3_1")]
    pub fn copy_cursor_word(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::CopyCursorWord(v));
        self
    }

    /// `copy_cursor_x` - Cursor X position in copy mode
    #[cfg(feature = "tmux_3_1")]
    pub fn copy_cursor_x(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::CopyCursorX(v));
        self
    }

    /// `copy_cursor_y` - Cursor Y position in copy mode
    #[cfg(feature = "tmux_3_1")]
    pub fn copy_cursor_y(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::CopyCursorY(v));
        self
    }

    // history

    /// `history_bytes`             Number of bytes in window history
    #[cfg(feature = "tmux_1_7")]
    pub fn histoty_bytes(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::HistotyBytes(v));
        self
    }

    /// `history_limit`             Maximum window history lines
    #[cfg(feature = "tmux_1_7")]
    pub fn history_limit(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::HistotyLimit(v));
        self
    }

    /// `history_size`              Size of history in bytes
    #[cfg(feature = "tmux_1_7")]
    pub fn history_size(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::HistorySize(v));
        self
    }

    // hook

    /// `hook` - Name of running hook, if any
    #[cfg(feature = "tmux_2_4")]
    pub fn hook(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::Hook(v));
        self
    }

    /// `hook_pane` - ID of pane where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    pub fn hook_pane(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::HookPane(v));
        self
    }

    /// `hook_session` - ID of session where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    pub fn hook_session(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::HookSession(v));
        self
    }

    /// `hook_session_name` - Name of session where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    pub fn hook_session_name(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::HookSessionName(v));
        self
    }

    /// `hook_window` - ID of window where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    pub fn hook_window(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::HookWindow(v));
        self
    }

    /// `hook_window_name` - Name of window where hook was run, if any
    #[cfg(feature = "tmux_2_4")]
    pub fn hook_window_name(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::HookWindowName(v));
        self
    }

    // host

    /// `host` - Hostname of local host
    #[cfg(feature = "tmux_1_6")]
    pub fn host(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::Host(v));
        self
    }

    /// `host_short` - #h Hostname of local host (no domain name)
    #[cfg(feature = "tmux_1_9")]
    pub fn host_short(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::HostShort(v));
        self
    }

    /// `insert_flag` - Pane insert flag
    #[cfg(feature = "tmux_1_8")]
    pub fn insert_flag(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::InsertFlag(v));
        self
    }

    /// `keypad_cursor_flag` - Pane keypad cursor flag
    #[cfg(feature = "tmux_1_8")]
    pub fn keypad_cursor_flag(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::KeypadCursorFlag(v));
        self
    }

    /// `keypad_flag` - Pane keypad flag
    #[cfg(feature = "tmux_1_8")]
    pub fn keypad_flag(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::KeypadFlag(v));
        self
    }

    /// `line` - Line number in the list
    #[cfg(feature = "tmux_1_6")]
    pub fn line(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::Line(v));
        self
    }

    /// `mouse_all_flag` - Pane mouse all flag
    #[cfg(feature = "tmux_3_0")]
    pub fn mouse_all_flag(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::MouseAllFlag(v));
        self
    }

    /// `mouse_any_flag` - Pane mouse any flag
    #[cfg(feature = "tmux_1_8")]
    pub fn mouse_any_flag(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::MouseAnyFlag(v));
        self
    }

    /// `mouse_button_flag` - Pane mouse button flag
    #[cfg(feature = "tmux_1_8")]
    pub fn mouse_button_flag(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::MouseButtonFlag(v));
        self
    }

    /// `mouse_line` - Line under mouse, if any
    #[cfg(feature = "tmux_3_0")]
    pub fn mouse_line(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::MouseLine(v));
        self
    }

    /// `sgr_flag` - Pane mouse SGR flag
    #[cfg(feature = "tmux_3_0")]
    pub fn sgr_line(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::MouseSgrFlag(v));
        self
    }

    /// `mouse_standard_flag` - Pane mouse standard flag
    #[cfg(feature = "tmux_1_8")]
    pub fn mouse_standard_flag(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::MouseStandardFlag(v));
        self
    }

    /// `mouse_utf8_flag` - Pane mouse UTF-8 flag
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_2"), feature = "tmux_3_0"))]
    pub fn mouse_utf8_flag(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::MouseUtf8Flag(v));
        self
    }

    /// `mouse_all_flag` - Pane mouse all flag
    #[cfg(feature = "tmux_2_4")]
    pub fn mouse_all_flag(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::MouseAllFlag(v));
        self
    }

    /// `mouse_word` - Word under mouse, if any
    #[cfg(feature = "tmux_3_0")]
    pub fn mouse_word(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::MouseWord(v));
        self
    }

    /// `mouse_x` - Mouse X position, if any
    #[cfg(feature = "tmux_3_0")]
    pub fn mouse_x(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::MouseX(v));
        self
    }

    /// `mouse_y` - Mouse Y position, if any
    #[cfg(feature = "tmux_3_0")]
    pub fn mouse_y(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::MouseY(v));
        self
    }

    /// `origin_flag` - Pane origin flag
    #[cfg(feature = "tmux_3_0")]
    pub fn origin_flag(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::OriginFlag(v));
        self
    }

    // pane

    /// `pane_active` - 1 if active pane
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_active(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::PaneActive(v));
        self
    }

    /// `pane_at_bottom` - 1 if pane is at the bottom of window
    #[cfg(feature = "tmux_2_6")]
    pub fn pane_at_bottom(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::PaneAtBottom(v));
        self
    }

    /// `pane_at_left` - 1 if pane is at the left of window
    #[cfg(feature = "tmux_2_6")]
    pub fn pane_at_left(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::PaneAtLeft(v));
        self
    }

    /// `pane_at_right` - 1 if pane is at the right of window
    #[cfg(feature = "tmux_2_6")]
    pub fn pane_at_right(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::PaneAtRight(v));
        self
    }

    /// `pane_at_top` - 1 if pane is at the top of window
    #[cfg(feature = "tmux_2_6")]
    pub fn pane_at_top(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::PaneAtTop(v));
        self
    }

    /// `pane_bottom` - Bottom of pane
    #[cfg(feature = "tmux_2_0")]
    pub fn pane_bottom(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::PaneBottom(v));
        self
    }

    /// `pane_current_command` - Current command if available
    #[cfg(feature = "tmux_1_8")]
    pub fn pane_current_command(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::PaneCurrentCommand(v));
        self
    }

    /// `pane_current_path` - Current path if available
    #[cfg(feature = "tmux_1_7")]
    pub fn pane_current_path(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::PaneCurrentPath(v));
        self
    }

    /// `pane_dead` - 1 if pane is dead
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_dead(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::PaneDead(v));
        self
    }

    /// `pane_dead_status` - Exit status of process in dead pane
    #[cfg(feature = "tmux_2_0")]
    pub fn pane_dead_status(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::PaneDeadStatus(v));
        self
    }

    /// `pane_format` - 1 if format is for a pane
    #[cfg(feature = "tmux_2_6")]
    pub fn pane_format(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::PaneFormat(v));
        self
    }

    /// `pane_height` - Height of pane
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_height(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::PaneHeight(v));
        self
    }

    /// `pane_id` - #D Unique pane ID
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_id(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::PaneId(v));
        self
    }

    /// `pane_in_mode` - 1 if pane is in a mode
    #[cfg(feature = "tmux_1_8")]
    pub fn pane_in_mode(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::PaneInMode(v));
        self
    }

    /// `pane_index` - #P Index of pane
    #[cfg(feature = "tmux_1_7")]
    pub fn pane_index(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::PaneIndex(v));
        self
    }

    /// `pane_input_off` - 1 if input to pane is disabled
    #[cfg(feature = "tmux_2_0")]
    pub fn pane_input_off(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::PaneInputOff(v));
        self
    }

    /// `pane_left` - Left of pane
    #[cfg(feature = "tmux_2_0")]
    pub fn pane_left(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::PaneLeft(v));
        self
    }

    /// `pane_marked` - 1 if this is the marked pane
    #[cfg(feature = "tmux_3_0")]
    pub fn pane_marked(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::PaneMarked(v));
        self
    }

    /// `pane_marked_set` - 1 if a marked pane is set
    #[cfg(feature = "tmux_3_0")]
    pub fn pane_marked_set(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::PaneMarkedSet(v));
        self
    }

    /// `pane_mode` - Name of pane mode, if any
    #[cfg(feature = "tmux_2_5")]
    pub fn pane_mode(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::PaneMode(v));
        self
    }

    /// `pane_path` - #T Path of pane (can be set by application)
    #[cfg(feature = "tmux_3_1")]
    pub fn pane_path(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::PanePath(v));
        self
    }

    /// `pane_pid` - PID of first process in pane
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_pid(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::PanePid(v));
        self
    }

    /// `pane_pipe` - 1 if pane is being piped
    #[cfg(feature = "tmux_2_6")]
    pub fn pane_pipe(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::PanePipe(v));
        self
    }

    /// `pane_right` - Right of pane
    #[cfg(feature = "tmux_2_0")]
    pub fn pane_right(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::PaneRight(v));
        self
    }

    /// `pane_search_string` - Last search Option<String> in copy mode
    #[cfg(feature = "tmux_2_5")]
    pub fn pane_search_string(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::PaneSearchString(v));
        self
    }

    /// `pane_start_command` - Command pane started with
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_start_command(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::PaneStartCommand(v));
        self
    }

    /// `pane_start_path` - Path pane started with
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_0")))]
    pub fn pane_start_path(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::PaneStartPath(v));
        self
    }

    /// `pane_synchronized` - 1 if pane is synchronized
    #[cfg(feature = "tmux_1_9")]
    pub fn pane_synchronized(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::PaneSynchronized(v));
        self
    }

    /// `pane_tabs` - Pane tab positions
    #[cfg(feature = "tmux_1_8")]
    pub fn pane_tabs(&mut self, v: &'a mut Option<PaneTabs>) -> &mut Self {
        self.push(VariableOutput::PaneTabs(v));
        self
    }

    /// `pane_title` - #T Title of pane (can be set by application)
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_title(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::PaneTitle(v));
        self
    }

    /// `pane_top` - Top of pane
    #[cfg(feature = "tmux_2_0")]
    pub fn pane_top(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::PaneTop(v));
        self
    }

    /// `pane_tty` - Pseudo terminal of pane
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_tty(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::PaneTty(v));
        self
    }

    /// `pane_width` - Width of pane
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_width(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::PaneWidth(v));
        self
    }

    /// `saved_cursor_x` - Saved cursor X in pane
    #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
    pub fn saved_cursor_x(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::SavedCursorX(v));
        self
    }

    /// `saved_cursor_y` - Saved cursor Y in pane
    #[cfg(any(feature = "tmux_1_8", not(feature = "tmux_2_1")))]
    pub fn saved_cursor_y(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::SavedCursorY(v));
        self
    }

    /// `pid` - Server PID
    #[cfg(feature = "tmux_2_1")]
    pub fn pid(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::Pid(v));
        self
    }

    /// `rectangle_toggle` - 1 if rectangle selection is activated
    #[cfg(feature = "tmux_2_7")]
    pub fn rectangle_toggle(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::RectangleToggle(v));
        self
    }

    /// `scroll_position` - Scroll position in copy mode
    #[cfg(feature = "tmux_2_2")]
    pub fn scroll_position(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::ScrollPosition(v));
        self
    }

    /// `scroll_region_lower` - Bottom of scroll region in pane
    #[cfg(feature = "tmux_1_8")]
    pub fn scroll_region_lower(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::ScrollRegionLower(v));
        self
    }

    /// `scroll_region_upper` - Top of scroll region in pane
    #[cfg(feature = "tmux_1_8")]
    pub fn scroll_region_upper(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::ScrollRegionUpper(v));
        self
    }

    /// `selection_active` - 1 if selection started and changes with the curso
    #[cfg(feature = "tmux_3_1")]
    pub fn selection_active(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::SelectionActive(v));
        self
    }

    /// `selection_end_x` - X position of the end of the selection
    #[cfg(feature = "tmux_3_1")]
    pub fn selection_end_x(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::SelectionEndX(v));
        self
    }

    /// `selection_end_y` - Y position of the end of the selection
    #[cfg(feature = "tmux_3_1")]
    pub fn selection_end_y(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::SelectionEndY(v));
        self
    }

    /// `selection_present` - 1 if selection started in copy mode
    #[cfg(feature = "tmux_2_6")]
    pub fn selection_present(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::SelectionPresent(v));
        self
    }

    /// `selection_start_x` - X position of the start of the selection
    #[cfg(feature = "tmux_3_1")]
    pub fn selection_start_x(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::SelectionStartX(v));
        self
    }

    /// `selection_start_y` - Y position of the start of the selection
    #[cfg(feature = "tmux_3_1")]
    pub fn selection_start_y(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::SelectionStartY(v));
        self
    }

    // Session
    /// `session_activity` - Time of session last activity
    #[cfg(feature = "tmux_2_1")]
    pub fn session_activity(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::SessionActivity(v));
        self
    }

    /// `session_activity_string` - Option<String> time of session last activity
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub fn session_activity_string(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::SessionActivityString(v));
        self
    }

    /// `session_alerts` - List of window indexes with alerts
    #[cfg(feature = "tmux_2_1")]
    pub fn session_alerts(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::SessionAlerts(v));
        self
    }

    /// `session_attached` - Number of clients session is attached to
    #[cfg(feature = "tmux_1_6")]
    pub fn session_attached(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::SessionAttached(v));
        self
    }

    /// `session_attached_list` - List of clients session is attached to
    #[cfg(feature = "tmux_3_1")]
    pub fn session_atached_list(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::SessionAttachedList(v));
        self
    }

    /// `session_created` - Time session created
    #[cfg(feature = "tmux_1_6")]
    pub fn session_created(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::SessionCreated(v));
        self
    }

    /// `session_created_string` - Option<String> time session created
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    pub fn session_created_string(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::SessionCreatedString(v));
        self
    }

    /// `session_format` - 1 if format is for a session (not assuming the current)
    #[cfg(feature = "tmux_2_6")]
    pub fn session_format(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::SessionFormat(v));
        self
    }

    /// `session_group` - Name of session group
    #[cfg(feature = "tmux_1_6")]
    pub fn session_group(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::SessionGroup(v));
        self
    }

    /// `session_group_attached` - Number of clients sessions in group are attached >
    #[cfg(feature = "tmux_3_1")]
    pub fn session_group_attached(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::SessionGroupAttached(v));
        self
    }

    /// `session_group_attached_list` - List of clients sessions in group are attached to
    #[cfg(feature = "tmux_3_1")]
    pub fn session_group_attached_list(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::SessionGroupAttachedList(v));
        self
    }

    /// `session_group_list` - List of sessions in group
    #[cfg(feature = "tmux_2_7")]
    pub fn session_group_list(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::SessionGroupList(v));
        self
    }

    /// `session_group_many_attached` - 1 if multiple clients attached to sessions in gro
    #[cfg(feature = "tmux_3_1")]
    pub fn session_group_many_attached(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::SessionGroupManyAttached(v));
        self
    }

    /// `session_group_size` - Size of session group
    #[cfg(feature = "tmux_2_7")]
    pub fn session_group_size(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::SessionGroupSize(v));
        self
    }

    /// `session_grouped` - 1 if session in a group
    #[cfg(feature = "tmux_1_6")]
    pub fn session_grouped(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::SessionGrouped(v));
        self
    }

    /// `session_height` - Height of session
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    pub fn session_height(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::SessionHeight(v));
        self
    }

    /// `session_width` - Width of session
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    pub fn session_width(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::SessionWidth(v));
        self
    }

    /// `session_id` - Unique session ID
    #[cfg(feature = "tmux_1_8")]
    pub fn session_id(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::SessionId(v));
        self
    }

    /// `session_last_attached` - Time session last attached
    #[cfg(feature = "tmux_2_1")]
    pub fn session_last_attached(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::SessionLastAttached(v));
        self
    }

    /// `session_last_attached_string` - Option<String> time session last attached
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub fn session_last_attached_string(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::SessionLastAttachedString(v));
        self
    }

    /// `session_many_attached` - 1 if multiple clients attached
    #[cfg(feature = "tmux_2_0")]
    pub fn session_many_attached(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::SessionManyAttached(v));
        self
    }

    /// `session_name` - #S Name of session
    #[cfg(feature = "tmux_1_6")]
    pub fn session_name(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::SessionName(v));
        self
    }

    /// `session_stack` - Window indexes in most recent order
    #[cfg(feature = "tmux_2_5")]
    pub fn session_stack(&mut self, v: &'a mut Option<SessionStack>) -> &mut Self {
        self.push(VariableOutput::SessionStack(v));
        self
    }

    /// `session_windows` - Number of windows in session
    #[cfg(feature = "tmux_1_6")]
    pub fn session_windows(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::SessionWindows(v));
        self
    }

    /// `socket_path` - Server socket path
    #[cfg(feature = "tmux_2_2")]
    pub fn socket_path(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::SocketPath(v));
        self
    }

    /// `start_time` - Server start time
    #[cfg(feature = "tmux_2_2")]
    pub fn start_time(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::StartTime(v));
        self
    }

    /// `version` - Server version
    #[cfg(feature = "tmux_2_4")]
    pub fn version(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::Version(v));
        self
    }

    // Window
    //
    /// `window_active` - 1 if window active
    #[cfg(feature = "tmux_1_6")]
    pub fn window_active(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowActive(v));
        self
    }

    /// `window_active_clients` - Number of clients viewing this window
    #[cfg(feature = "tmux_3_1")]
    pub fn window_active_clients(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowActiveClients(v));
        self
    }

    /// `window_active_clients_list` - List of clients viewing this window
    #[cfg(feature = "tmux_3_1")]
    pub fn window_active_clients_list(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowActiveClientsList(v));
        self
    }

    /// `window_active_sessions` - Number of sessions on which this window is active
    #[cfg(feature = "tmux_3_1")]
    pub fn window_active_sessions(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowActiveSessions(v));
        self
    }

    /// `window_active_sessions_list` - List of sessions on which this window is active
    #[cfg(feature = "tmux_3_1")]
    pub fn window_active_sessions_list(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowActiveSessionsList(v));
        self
    }

    /// `window_activity` - Time of window last activity
    #[cfg(feature = "tmux_2_1")]
    pub fn window_activity(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::WindowActivity(v));
        self
    }

    /// `session_activity_string` - Option<String> time of session last activity
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub fn session_activity_string(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::SessionActivityString(v));
        self
    }

    /// `window_activity_flag` - 1 if window has activity
    #[cfg(any(
        all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
        feature = "tmux_2_3"
    ))]
    pub fn window_activity_flag(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowActivityFlag(v));
        self
    }

    /// `window_bell_flag` - 1 if window has bell
    #[cfg(feature = "tmux_1_9")]
    pub fn window_bell_flag(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowBellFlag(v));
        self
    }

    /// `window_content_flag` - 1 if window has content alert
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub fn window_content_flag(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowContentFlag(v));
        self
    }

    /// `window_bigger` - 1 if window is larger than client
    #[cfg(feature = "tmux_2_9")]
    pub fn window_bigger(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowBigger(v));
        self
    }

    /// `window_cell_height` - Height of each cell in pixels
    #[cfg(feature = "tmux_3_1")]
    pub fn window_cell_height(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowCellHeight(v));
        self
    }

    /// `window_cell_width` - Width of each cell in pixels
    #[cfg(feature = "tmux_3_1")]
    pub fn window_cell_width(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowCellWidth(v));
        self
    }

    /// `window_end_flag` - 1 if window has the highest index
    #[cfg(feature = "tmux_2_9")]
    pub fn window_end_flag(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowEndFlag(v));
        self
    }

    /// `window_find_matches` - Matched data from the find-window command if available
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub fn window_find_matches(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowFindMatches(v));
        self
    }

    /// `window_flags` - #F Window flags
    #[cfg(feature = "tmux_1_6")]
    pub fn window_flags(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::Version(v));
        self
    }

    /// `window_format` - 1 if format is for a window
    #[cfg(feature = "tmux_2_6")]
    pub fn window_format(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowFormat(v));
        self
    }

    /// `window_height` - Height of window
    #[cfg(feature = "tmux_1_6")]
    pub fn window_height(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::WindowHeight(v));
        self
    }

    /// `window_id` - Unique window ID
    #[cfg(feature = "tmux_1_7")]
    pub fn window_id(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::WindowId(v));
        self
    }

    /// `window_index` - #I Index of window
    #[cfg(feature = "tmux_1_6")]
    pub fn window_index(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::WindowIndex(v));
        self
    }

    /// `window_last_flag` - 1 if window is the last used
    #[cfg(feature = "tmux_2_0")]
    pub fn window_last_flag(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowLastFlag(v));
        self
    }

    /// `window_layout` - Window layout description, ignoring zoomed window panes
    #[cfg(feature = "tmux_1_6")]
    pub fn window_layout(&mut self, v: &'a mut Option<Layout>) -> &mut Self {
        self.push(VariableOutput::WindowLayout(v));
        self
    }

    /// `window_linked` - 1 if window is linked across sessions
    #[cfg(feature = "tmux_2_1")]
    pub fn window_linked(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowLinked(v));
        self
    }

    /// `window_linked_sessions` - Number of sessions this window is linked to
    #[cfg(feature = "tmux_3_1")]
    pub fn window_linked_sessions(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowLinkedSessions(v));
        self
    }

    /// `window_linked_sessions_list` - List of sessions this window is linked to
    #[cfg(feature = "tmux_3_1")]
    pub fn window_linked_sessions_list(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowLinkedSessionsList(v));
        self
    }

    /// `window_marked_flag` - 1 if window contains the marked pane
    #[cfg(feature = "tmux_3_1")]
    pub fn window_marked_flag(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowMarkedFlag(v));
        self
    }

    /// `window_name` - #W Name of window
    #[cfg(feature = "tmux_1_6")]
    pub fn window_name(&mut self, v: &'a mut Option<String>) -> &mut Self {
        self.push(VariableOutput::WindowName(v));
        self
    }

    /// `window_offset_x` - X offset into window if larger than client
    #[cfg(feature = "tmux_2_9")]
    pub fn window_offset_x(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowOffsetX(v));
        self
    }

    /// `window_offset_y` - Y offset into window if larger than client
    #[cfg(feature = "tmux_2_9")]
    pub fn window_offset_y(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowOffsetY(v));
        self
    }

    /// `window_panes` - Number of panes in window
    #[cfg(feature = "tmux_1_7")]
    pub fn window_panes(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::WindowPanes(v));
        self
    }

    /// `window_silence_flag` - 1 if window has silence alert
    #[cfg(feature = "tmux_1_9")]
    pub fn window_silence_flag(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowSilenceFlag(v));
        self
    }

    /// `window_stack_index` - Index in session most recent stack
    #[cfg(feature = "tmux_2_5")]
    pub fn window_stack_index(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::WindowStackIndex(v));
        self
    }

    /// `window_start_flag` - 1 if window has the lowest index
    #[cfg(feature = "tmux_2_9")]
    pub fn window_start_flag(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowStartFlag(v));
        self
    }

    /// `window_visible_layout` - Window layout description, respecting zoomed window panes
    #[cfg(feature = "tmux_2_2")]
    pub fn window_visible_layout(&mut self, v: &'a mut Option<Layout>) -> &mut Self {
        self.push(VariableOutput::WindowVisibleLayout(v));
        self
    }

    /// `window_width` - Width of window
    #[cfg(feature = "tmux_1_6")]
    pub fn window_width(&mut self, v: &'a mut Option<usize>) -> &mut Self {
        self.push(VariableOutput::WindowWidth(v));
        self
    }

    /// `window_zoomed_flag` - 1 if window is zoomed
    #[cfg(feature = "tmux_2_0")]
    pub fn window_zoomed_flag(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WindowZoomedFlag(v));
        self
    }

    /// `wrap_flag` - Pane wrap flag
    #[cfg(feature = "tmux_1_8")]
    pub fn wrap_flag(&mut self, v: &'a mut Option<bool>) -> &mut Self {
        self.push(VariableOutput::WrapFlag(v));
        self
    }
}
