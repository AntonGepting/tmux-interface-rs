//! **tmux_interface** as a Rust library provides functionality to communicate with TMUX via CLI.
//!
//! # Description
//!
//! Main purpose of the tmux_interface is to implement simple sending and recieving data mechanisms
//! for some Rust application, using intercommunication with TMUX only via standard streams (stdin,
//! stdout, stderr).
//!
//! This goal can be reached by splitting it into two separate tasks:
//!
//! 1. Providing wrapper functions for tmux subcommands (sending data). Wrapper functions are
//! structured like in tmux manual in few next categories:
//!
//!     - Clients and Sessions ([`clients_and_sessions`](crate::clients_and_sessions))
//!     - Windows and Panes ([`windows_and_panes`](crate::windows_and_panes))
//!     - Key Bindings ([`key_bindings`](crate::key_bindings))
//!     - Options ([`options`](crate::options))
//!     - Hooks ([`hooks`](crate::hooks))
//!     - Global and Session Environment ([`global_and_session_environment`](crate::global_and_session_environment))
//!     - Status Line ([`status_line`](crate::status_line))
//!     - Buffers ([`buffers`](crate::buffers))
//!     - Miscellaneous ([`miscellaneous`](crate::miscellaneous))
//!
//! Main structure is [`TmuxInterface`](crate::tmux_interface::TmuxInterface) wich has all these wrapper functions implementations.
//!
//! 2. Parsing functions for tmux output as rust structures (recieving data). Parsing function are
//! structured by objects they operate with:
//!
//!     - [`Sessions`](crate::Sessions)
//!     - [`Session`](crate::Session)
//!     - [`Windows`](crate::Windows)
//!     - [`Window`](crate::Window)
//!     - [`Panes`](crate::Panes)
//!     - [`Pane`](crate::Pane)
//!     - ...
//!     - [`TmuxOption`](crate::TmuxOption)
//!
//! # Library Functions
//!
//! 1. Function names and their grouping are inherited from tmux manual
//! 2. Function arguments and their optionality inherited from tmux manual
//! 3. Functions can have max. 4 arguments, otherwise a structure will be used
//!
//! # Examples
//!
//! ```
//! use crate::tmux_interface::TmuxInterface;
//! use crate::tmux_interface::NewSession;
//!
//!
//! fn main() {
//!     let tmux = TmuxInterface::new();
//!
//!     let new_session = NewSession {
//!         detached: Some(true),
//!         session_name: Some("test_session_name1"),
//!         ..Default::default()
//!     };
//!     tmux.new_session(&new_session).unwrap();
//!     tmux.kill_session(None, None, Some("test_session_name1")).unwrap();
//!
//!     // or alternatively
//!     let mut new_session = NewSession::new();
//!     new_session.detached = Some(true);
//!     new_session.session_name = Some("test_session_name2");
//!     tmux.new_session(&new_session).unwrap();
//!     tmux.kill_session(None, None, Some("test_session_name2")).unwrap();
//! }
//! ```
//!
//!
//! # Examples
//!
//! ```
//! use crate::tmux_interface::Sessions;
//!
//!
//! fn main() {
//!     let sessions = Sessions::get().unwrap();
//! }
//! ```
//!
//!
//! # Parsing objects and supported tmux variables
//!
//! ```text
//! Object      Variable name       Alias Replaced with
//! ===============================================================================================
//! Pane        alternate_on              1 if pane is in alternate screen
//! ?           alternate_saved_x         Saved cursor X in alternate screen
//! ?           alternate_saved_y         Saved cursor Y in alternate screen
//! Buffer      buffer_created            Time buffer created
//! Buffer      buffer_name               Name of buffer
//! Buffer      buffer_sample             Sample of start of buffer
//! Buffer      buffer_size               Size of the specified buffer in bytes
//! Client      client_activity           Time client last had activity
//! Client      client_control_mode       1 if client is in control mode
//! Client      client_created            Time client created
//! Client      client_discarded          Bytes discarded when client behind
//! Client      client_height             Height of client
//! Client      client_key_table          Current key table
//! Client      client_last_session       Name of the client's last session
//! Client      client_name               Name of client
//! Client      client_pid                PID of client process
//! Client      client_prefix             1 if prefix key has been pressed
//! Client      client_readonly           1 if client is readonly
//! Client      client_session            Name of the client's session
//! Client      client_termname           Terminal name of client
//! Client      client_termtype           Terminal type of client
//! Client      client_tty                Pseudo terminal of client
//! Client      client_utf8               1 if client supports utf8
//! Client      client_width              Width of client
//! Client      client_written            Bytes written to client
//! Command     command                   Name of command in use, if any
//! Command     command_list_alias        Command alias if listing commands
//! Command     command_list_name         Command name if listing commands
//! Command     command_list_usage        Command usage if listing commands
//! Cursor      cursor_character          Character at cursor in pane
//! Cursor      cursor_flag               Pane cursor flag
//! Cursor      cursor_x                  Cursor X position in pane
//! Cursor      cursor_y                  Cursor Y position in pane
//! History     history_bytes             Number of bytes in window history
//! History     history_limit             Maximum window history lines
//! History     history_size              Size of history in lines
//! Hook        hook                      Name of running hook, if any
//! Hook        hook_pane                 ID of pane where hook was run, if any
//! Hook        hook_session              ID of session where hook was run, if any
//! Hook        hook_session_name         Name of session where hook was run, if any
//! Hook        hook_window               ID of window where hook was run, if any
//! Hook        hook_window_name          Name of window where hook was run, if any
//! ?           host                   #H Hostname of local host
//! ?           host_short             #h Hostname of local host (no domain name)
//! Pane        insert_flag               Pane insert flag
//! Pane        keypad_cursor_flag        Pane keypad cursor flag
//! Pane        keypad_flag               Pane keypad flag
//! ?           line                      Line number in the list
//! Mouse       mouse_all_flag            Pane mouse all flag
//! Mouse       mouse_any_flag            Pane mouse any flag
//! Mouse       mouse_button_flag         Pane mouse button flag
//! Mouse       mouse_line                Line under mouse, if any
//! Mouse       mouse_sgr_flag            Pane mouse SGR flag
//! Mouse       mouse_standard_flag       Pane mouse standard flag
//! Mouse       mouse_utf8_flag           Pane mouse UTF-8 flag
//! Mouse       mouse_word                Word under mouse, if any
//! Mouse       mouse_x                   Mouse X position, if any
//! Mouse       mouse_y                   Mouse Y position, if any
//! Pane        origin_flag               Pane origin flag
//! +Pane       pane_active               1 if active pane
//! +Pane       pane_at_bottom            1 if pane is at the bottom of window
//! +Pane       pane_at_left              1 if pane is at the left of window
//! +Pane       pane_at_right             1 if pane is at the right of window
//! +Pane       pane_at_top               1 if pane is at the top of window
//! +Pane       pane_bottom               Bottom of pane
//! +Pane       pane_current_command      Current command if available
//! +Pane       pane_current_path         Current path if available
//! +Pane       pane_dead                 1 if pane is dead
//! +Pane       pane_dead_status          Exit status of process in dead pane
//! +Pane       pane_format               1 if format is for a pane (not assuming the current)
//! +Pane       pane_height               Height of pane
//! +Pane       pane_id                #D Unique pane ID
//! +Pane       pane_in_mode              1 if pane is in a mode
//! +Pane       pane_index             #P Index of pane
//! +Pane       pane_input_off            1 if input to pane is disabled
//! +Pane       pane_left                 Left of pane
//! +Pane       pane_marked               1 if this is the marked pane
//! +Pane       pane_marked_set           1 if a marked pane is set
//! +Pane       pane_mode                 Name of pane mode, if any
//! +Pane       pane_pid                  PID of first process in pane
//! +Pane       pane_pipe                 1 if pane is being piped
//! +Pane       pane_right                Right of pane
//! +Pane       pane_search_string        Last search string in copy mode
//! +Pane       pane_start_command        Command pane started with
//! +Pane       pane_synchronized         1 if pane is synchronized
//! +Pane       pane_tabs                 Pane tab positions
//! +Pane       pane_title             #T Title of pane
//! +Pane       pane_top                  Top of pane
//! +Pane       pane_tty                  Pseudo terminal of pane
//! +Pane       pane_width                Width of pane
//! Server      pid                       Server PID
//! ?           rectangle_toggle          1 if rectangle selection is activated
//! ?           scroll_position           Scroll position in copy mode
//! ?           scroll_region_lower       Bottom of scroll region in pane
//! ?           scroll_region_upper       Top of scroll region in pane
//! ?           selection_present         1 if selection started in copy mode
//! +Session    session_activity          Time of session last activity
//! +Session    session_alerts            List of window indexes with alerts
//! +Session    session_attached          Number of clients session is attached to
//! +Session    session_created           Time session created
//! +Session    session_format            1 if format is for a session (not assuming the current)
//! +Session    session_group             Name of session group
//! +Session    session_group_list        List of sessions in group
//! +Session    session_group_size        Size of session group
//! +Session    session_grouped           1 if session in a group
//! +Session    session_id                Unique session ID
//! +Session    session_last_attached     Time session last attached
//! +Session    session_many_attached     1 if multiple clients attached
//! +Session    session_name           #S Name of session
//! +Session    session_stack             Window indexes in most recent order
//! +Session    session_windows           Number of windows in session
//! Server      socket_path               Server socket path
//! Server      start_time                Server start time
//! Server      version                   Server version
//! +Window     window_active             1 if window active
//! +Window     window_activity           Time of window last activity
//! +Window     window_activity_flag      1 if window has activity
//! +Window     window_bell_flag          1 if window has bell
//! +Window     window_bigger             1 if window is larger than client
//! +Window     window_end_flag           1 if window has the highest index
//! +Window     window_flags           #F Window flags
//! +Window     window_format             1 if format is for a window (not assuming the current)
//! +Window     window_height             Height of window
//! +Window     window_id                 Unique window ID
//! +Window     window_index           #I Index of window
//! +Window     window_last_flag          1 if window is the last used
//! +Window     window_layout             Window layout description, ignoring zoomed window panes
//! +Window     window_linked             1 if window is linked across sessions
//! +Window     window_name            #W Name of window
//! +Window     window_offset_x           X offset into window if larger than client
//! +Window     window_offset_y           Y offset into window if larger than client
//! +Window     window_panes              Number of panes in window
//! +Window     window_silence_flag       1 if window has silence alert
//! +Window     window_stack_index        Index in session most recent stack
//! +Window     window_start_flag         1 if window has the lowest index
//! +Window     window_visible_layout     Window layout description, respecting zoomed window panes
//! +Window     window_width              Width of window
//! +Window     window_zoomed_flag        1 if window is zoomed
//! Pane        wrap_flag                 Pane wrap flag
//! ```


pub mod tmux_interface;
pub mod session;
pub mod sessions;
pub mod window;
pub mod windows;
pub mod pane;
pub mod panes;
pub mod tmux_interface_error;
pub mod windows_and_panes;
pub mod clients_and_sessions;
pub mod key_bindings;
pub mod status_line;
pub mod options;
pub mod hooks;
pub mod buffers;
pub mod global_and_session_environment;
pub mod miscellaneous;
pub mod tmux_option;
pub mod layout_cell;
pub mod layout;
pub mod layout_checksum;
pub mod version;


pub use self::tmux_interface::TmuxInterface;
pub use self::clients_and_sessions::AttachSession;
pub use self::clients_and_sessions::DetachClient;
pub use self::clients_and_sessions::NewSession;
pub use self::clients_and_sessions::RefreshClient;
pub use self::clients_and_sessions::SwitchClient;

pub use self::windows_and_panes::BreakPane;
pub use self::windows_and_panes::CapturePane;
pub use self::windows_and_panes::ChooseClient;
pub use self::windows_and_panes::ChooseTree;
pub use self::windows_and_panes::FindWindow;
pub use self::windows_and_panes::JoinPane;
pub use self::windows_and_panes::LinkWindow;
pub use self::windows_and_panes::MovePane;
pub use self::windows_and_panes::MoveWindow;
pub use self::windows_and_panes::NewWindow;
pub use self::windows_and_panes::PipePane;
pub use self::windows_and_panes::ResizePane;
pub use self::windows_and_panes::ResizeWindow;
pub use self::windows_and_panes::RespawnPane;
pub use self::windows_and_panes::RespawnWindow;
pub use self::windows_and_panes::SelectLayot;
pub use self::windows_and_panes::SelectPane;
pub use self::windows_and_panes::SelectWindow;
pub use self::windows_and_panes::SplitWindow;
pub use self::windows_and_panes::SwapPane;

pub use self::key_bindings::BindKey;
pub use self::key_bindings::SendKeys;

pub use self::options::ShowOptions;
pub use self::tmux_option::TmuxOption;

pub use self::session::Session;
pub use self::session::SessionStack;
pub use self::sessions::Sessions;
pub use self::window::Window;
pub use self::windows::Windows;
pub use self::pane::Pane;
pub use self::panes::Panes;
pub use self::version::Version;
pub use self::layout::Layout;
pub use self::layout_cell::LayoutCell;
pub use self::layout_cell::LayoutType;
pub use self::layout_checksum::LayoutChecksum;

pub use self::tmux_interface_error::TmuxInterfaceError;


#[cfg(test)]
mod tmux_interface_tests;
mod session_tests;
mod sessions_tests;
mod window_tests;
mod windows_tests;
mod pane_tests;
mod panes_tests;
//mod options_tests;
mod tmux_option_tests;
mod layout_tests;
mod layout_cell_tests;
mod layout_checksum_tests;
mod version_tests;
