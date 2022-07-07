use crate::Error;
use crate::FormatsOutput;
#[cfg(feature = "tmux_2_5")]
use crate::SessionStack;

// XXX: number of all flags, needed for array init
// NOTE: variables were first intoduced in tmux 1.6
// accordingly to tmux.h: Formats
// XXX: check all types
#[derive(Default, PartialEq, Clone, Debug)]
pub struct Session {
    // NOTE: u64
    /// session_activity - Time of session last activity
    #[cfg(feature = "tmux_2_1")]
    pub activity: Option<usize>,
    /// session_activity_string - String time of session last activity
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub activity_string: Option<String>,
    /// session_alerts - List of window indexes with alerts
    #[cfg(feature = "tmux_2_1")]
    pub alerts: Option<String>,
    /// session_attached - Number of clients session is attached to
    #[cfg(feature = "tmux_1_6")]
    pub attached: Option<usize>,
    /// session_attached_list - List of clients session is attached to
    #[cfg(feature = "tmux_3_1")]
    pub attached_list: Option<usize>,
    // NOTE: u64
    /// session_created - Time session created
    #[cfg(feature = "tmux_1_6")]
    pub created: Option<usize>,
    /// String time session created
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    pub created_string: Option<String>,
    /// 1 if format is for a session (not assuming the current)
    #[cfg(feature = "tmux_2_6")]
    pub format: Option<bool>,
    /// session_group - Name of session group
    #[cfg(feature = "tmux_1_6")]
    pub group: Option<String>,
    /// session_group_attached - Number of clients sessions in group are attached >
    #[cfg(feature = "tmux_3_1")]
    pub group_attached: Option<usize>,
    /// session_group_attached_list - List of clients sessions in group are attached to
    #[cfg(feature = "tmux_3_1")]
    pub group_attached_list: Option<String>,
    /// session_group_list - List of sessions in group
    #[cfg(feature = "tmux_2_7")]
    pub group_list: Option<String>,
    /// session_group_many_attached - 1 if multiple clients attached to sessions in gro
    #[cfg(feature = "tmux_3_1")]
    pub group_many_attached: Option<bool>,
    /// session_size - Size of session group
    #[cfg(feature = "tmux_2_7")]
    pub group_size: Option<String>,
    /// session_grouped - 1 if session in a group
    #[cfg(feature = "tmux_1_6")]
    pub grouped: Option<bool>,
    /// session_height - Height of session
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    pub height: Option<usize>,
    /// session_width - Width of session
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    pub width: Option<usize>,
    /// session_id - Unique session ID
    #[cfg(feature = "tmux_1_8")]
    pub id: Option<usize>,
    // NOTE: u64
    /// session_last_attached - Time session last attached
    #[cfg(feature = "tmux_2_1")]
    pub last_attached: Option<usize>,
    /// session_last_attached_string - String time session last attached
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub last_attached_string: Option<String>,
    /// session_many_attached - 1 if multiple clients attached
    #[cfg(feature = "tmux_2_0")]
    pub many_attached: Option<bool>,
    /// session_name - #S Name of session
    #[cfg(feature = "tmux_1_6")]
    pub name: Option<String>,
    /// session_stack - Window indexes in most recent order
    #[cfg(feature = "tmux_2_5")]
    pub stack: Option<SessionStack>,
    /// session_windows - Number of windows in session
    #[cfg(feature = "tmux_1_6")]
    pub windows: Option<usize>,
}

impl Session {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_str<S: AsRef<str>>(s: S) -> Result<Self, Error> {
        let mut session = Session::new();
        let mut format = FormatsOutput::new();
        format.separator(':');

        #[cfg(feature = "tmux_2_1")]
        format.session_activity(&mut session.activity);
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        format.session_activity_string(&mut session.activity_string);
        #[cfg(feature = "tmux_2_1")]
        format.session_alerts(&mut session.alerts);
        #[cfg(feature = "tmux_1_6")]
        format.session_attached(&mut session.attached);
        #[cfg(feature = "tmux_3_1")]
        format.session_attached_list(&mut session.attached_list);
        #[cfg(feature = "tmux_1_6")]
        format.session_created(&mut session.created);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        format.session_created_string(&mut session.created_string);
        #[cfg(feature = "tmux_2_6")]
        format.session_format(&mut session.format);
        #[cfg(feature = "tmux_1_6")]
        format.session_group(&mut session.group);
        #[cfg(feature = "tmux_3_1")]
        format.session_group_attached(&mut session.group_attached);
        #[cfg(feature = "tmux_3_1")]
        format.session_group_attached_list(&mut session.group_attached_list);
        #[cfg(feature = "tmux_2_7")]
        format.session_group_list(&mut session.group_list);
        #[cfg(feature = "tmux_3_1")]
        format.session_group_many_attached(&mut session.group_many_attached);
        #[cfg(feature = "tmux_2_7")]
        format.session_group_size(&mut session.group_size);
        #[cfg(feature = "tmux_1_6")]
        format.session_grouped(&mut session.grouped);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
        format.session_height(&mut session.height);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
        format.session_width(&mut session.width);
        #[cfg(feature = "tmux_1_8")]
        format.session_id(&mut session.id);
        #[cfg(feature = "tmux_2_1")]
        format.session_last_attached(&mut session.last_attached);
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        format.session_last_attached_string(&mut session.last_attached_string);
        #[cfg(feature = "tmux_2_0")]
        format.session_many_attached(&mut session.many_attached);
        #[cfg(feature = "tmux_1_6")]
        format.session_name(&mut session.name);
        #[cfg(feature = "tmux_2_5")]
        format.session_stack(&mut session.stack);
        #[cfg(feature = "tmux_1_6")]
        format.session_windows(&mut session.windows);

        FormatsOutput::from_string_ext(s.as_ref(), &mut format);
        Ok(session)
    }

    // XXX: wrapper with format generating and result parsing using callback
}
