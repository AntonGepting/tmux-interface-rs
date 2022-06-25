use crate::formats::formats_output::FormatsOutput;
use crate::Error;
use crate::Layout;
use crate::WindowFlags;

// NOTE: variables were first intoduced in tmux 1.6

pub const WINDOW_VARS_SEPARATOR: char = '\'';

// accordingly to tmux.h: Formats
// XXX: check all types, optionality
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Window {
    /// window_active - 1 if window active
    #[cfg(feature = "tmux_1_6")]
    pub active: Option<bool>,
    /// window_active_clients - Number of clients viewing this window
    #[cfg(feature = "tmux_3_1")]
    pub active_clients: Option<usize>,
    /// window_active_clients_list - List of clients viewing this window
    #[cfg(feature = "tmux_3_1")]
    pub active_clients_list: Option<String>,
    /// window_active_sessions - Number of sessions on which this window is active
    #[cfg(feature = "tmux_3_1")]
    pub active_sessions: Option<usize>,
    /// window_active_sessions_list - List of sessions on which this window is active
    #[cfg(feature = "tmux_3_1")]
    pub active_sessions_list: Option<String>,
    /// window_activity - Time of window last activity
    #[cfg(feature = "tmux_2_1")]
    pub activity: Option<usize>,
    /// session_activity_string - String time of session last activity
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub activity_string: Option<String>,
    /// window_activity_flag - 1 if window has activity
    #[cfg(any(
        all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
        feature = "tmux_2_3"
    ))]
    pub activity_flag: Option<bool>,
    /// window_bell_flag - 1 if window has bell
    #[cfg(feature = "tmux_1_9")]
    pub bell_flag: Option<bool>,
    /// window_content_flag - 1 if window has content alert
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub content_flag: Option<bool>,
    /// 1 if window is larger than client
    #[cfg(feature = "tmux_2_9")]
    pub bigger: Option<bool>,
    /// window_cell_height - Height of each cell in pixels
    #[cfg(feature = "tmux_3_1")]
    pub cell_height: Option<usize>,
    /// window_cell_width - Width of each cell in pixels
    #[cfg(feature = "tmux_3_1")]
    pub cell_width: Option<usize>,
    /// 1 if window has the highest index
    #[cfg(feature = "tmux_2_9")]
    pub end_flag: Option<bool>,
    /// window_find_matches - Matched data from the find-window command if available
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub find_matches: Option<String>,
    /// window_flags - #F Window flags
    #[cfg(feature = "tmux_1_6")]
    pub flags: Option<WindowFlags>,
    /// 1 if format is for a window
    #[cfg(feature = "tmux_2_6")]
    pub format: Option<bool>,
    /// window_height - Height of window
    #[cfg(feature = "tmux_1_6")]
    pub height: Option<usize>,
    /// window_id - Unique window ID
    #[cfg(feature = "tmux_1_7")]
    pub id: Option<usize>,
    /// window_index - #I Index of window
    #[cfg(feature = "tmux_1_6")]
    pub index: Option<usize>,
    /// window_last_flag - 1 if window is the last used
    #[cfg(feature = "tmux_2_0")]
    pub last_flag: Option<bool>,
    /// window_layout - Window layout description, ignoring zoomed window panes
    #[cfg(feature = "tmux_1_6")]
    pub layout: Option<Layout>,
    /// window_linked - 1 if window is linked across sessions
    #[cfg(feature = "tmux_2_1")]
    pub linked: Option<bool>,
    /// window_linked_sessions - Number of sessions this window is linked to
    #[cfg(feature = "tmux_3_1")]
    pub linked_sessions: Option<usize>,
    /// window_linked_sessions_list - List of sessions this window is linked to
    #[cfg(feature = "tmux_3_1")]
    pub linked_sessions_list: Option<String>,
    /// window_marked_flag - 1 if window contains the marked pane
    #[cfg(feature = "tmux_3_1")]
    pub marked_flag: Option<bool>,
    /// window_name - #W Name of window
    #[cfg(feature = "tmux_1_6")]
    pub name: Option<String>,
    /// X offset into window if larger than client
    #[cfg(feature = "tmux_2_9")]
    pub offset_x: Option<usize>,
    /// Y offset into window if larger than client
    #[cfg(feature = "tmux_2_9")]
    pub offset_y: Option<usize>,
    /// window_panes - Number of panes in window
    #[cfg(feature = "tmux_1_7")]
    pub panes: Option<usize>,
    /// window_silence_flag - 1 if window has silence alert
    #[cfg(feature = "tmux_1_9")]
    pub silence_flag: Option<bool>,
    /// Index in session most recent stack
    #[cfg(feature = "tmux_2_5")]
    pub stack_index: Option<usize>,
    /// 1 if window has the lowest index
    #[cfg(feature = "tmux_2_9")]
    pub start_flag: Option<bool>,
    /// Window layout description, respecting zoomed window panes
    #[cfg(feature = "tmux_2_2")]
    pub visible_layout: Option<Layout>,
    /// window_width - Width of window
    #[cfg(feature = "tmux_1_6")]
    pub width: Option<usize>,
    /// window_zoomed_flag - 1 if window is zoomed
    #[cfg(feature = "tmux_2_0")]
    pub zoomed_flag: Option<bool>,
}

impl Window {
    pub fn new() -> Self {
        Default::default()
    }

    // XXX: mb deserialize like serde something?
    pub fn from_str<S: AsRef<str>>(s: S) -> Result<Self, Error> {
        let mut window = Window::new();
        let mut format = FormatsOutput::new();

        format.separator(WINDOW_VARS_SEPARATOR);

        #[cfg(feature = "tmux_1_6")]
        format.window_active(&mut window.active);
        #[cfg(feature = "tmux_3_1")]
        format.window_active_clients(&mut window.active_clients);
        #[cfg(feature = "tmux_3_1")]
        format.window_active_clients_list(&mut window.active_clients_list);
        #[cfg(feature = "tmux_3_1")]
        format.window_active_sessions(&mut window.active_sessions);
        #[cfg(feature = "tmux_3_1")]
        format.window_active_sessions_list(&mut window.active_sessions_list);
        #[cfg(feature = "tmux_2_1")]
        format.window_activity(&mut window.activity);
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        format.window_activity_string(&mut window.activity_string);
        #[cfg(any(
            all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
            feature = "tmux_2_3"
        ))]
        format.window_activity_flag(&mut window.activity_flag);
        #[cfg(feature = "tmux_1_9")]
        format.window_bell_flag(&mut window.bell_flag);
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        format.window_content_flag(&mut window.content_flag);
        #[cfg(feature = "tmux_2_9")]
        format.window_bigger(&mut window.bigger);
        #[cfg(feature = "tmux_3_1")]
        format.window_cell_height(&mut window.cell_height);
        #[cfg(feature = "tmux_3_1")]
        format.window_cell_width(&mut window.cell_width);
        #[cfg(feature = "tmux_2_9")]
        format.window_end_flag(&mut window.end_flag);
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
        format.window_find_matches(&mut window.find_matches);
        #[cfg(feature = "tmux_1_6")]
        format.window_flags(&mut window.flags);
        #[cfg(feature = "tmux_2_6")]
        format.window_format(&mut window.format);
        #[cfg(feature = "tmux_1_6")]
        format.window_height(&mut window.height);
        #[cfg(feature = "tmux_1_7")]
        format.window_id(&mut window.id);
        #[cfg(feature = "tmux_1_6")]
        format.window_index(&mut window.index);
        #[cfg(feature = "tmux_2_0")]
        format.window_last_flag(&mut window.last_flag);
        #[cfg(feature = "tmux_1_6")]
        format.window_layout(&mut window.layout);
        #[cfg(feature = "tmux_2_1")]
        format.window_linked(&mut window.linked);
        #[cfg(feature = "tmux_3_1")]
        format.window_linked_sessions(&mut window.linked_sessions);
        #[cfg(feature = "tmux_3_1")]
        format.window_linked_sessions_list(&mut window.linked_sessions_list);
        #[cfg(feature = "tmux_3_1")]
        format.window_marked_flag(&mut window.marked_flag);
        #[cfg(feature = "tmux_1_6")]
        format.window_name(&mut window.name);
        #[cfg(feature = "tmux_2_9")]
        format.window_offset_x(&mut window.offset_x);
        #[cfg(feature = "tmux_2_9")]
        format.window_offset_y(&mut window.offset_y);
        #[cfg(feature = "tmux_1_7")]
        format.window_panes(&mut window.panes);
        #[cfg(feature = "tmux_1_9")]
        format.window_silence_flag(&mut window.silence_flag);
        #[cfg(feature = "tmux_2_5")]
        format.window_stack_index(&mut window.stack_index);
        #[cfg(feature = "tmux_2_9")]
        format.window_start_flag(&mut window.start_flag);
        #[cfg(feature = "tmux_2_2")]
        format.window_visible_layout(&mut window.visible_layout);
        #[cfg(feature = "tmux_1_6")]
        format.window_width(&mut window.width);
        #[cfg(feature = "tmux_2_0")]
        format.window_zoomed_flag(&mut window.zoomed_flag);

        FormatsOutput::from_string_ext(s.as_ref(), &mut format);
        Ok(window)
    }
}
