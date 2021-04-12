use std::io::BufRead;
use std::io::Lines;

/// `%begin`
#[cfg(feature = "tmux_1_8")]
pub const OUTPUT_BLOCK_BEGIN: &str = "%begin";
/// `%end`
#[cfg(feature = "tmux_1_8")]
pub const OUTPUT_BLOCK_END: &str = "%end";
/// `%error`
#[cfg(feature = "tmux_1_8")]
pub const OUTPUT_BLOCK_ERROR: &str = "%error";

/// In control mode, tmux outputs notifications.  A notification will
/// never occur inside an output block. (tmux man)
///
/// `%client-detached client`
#[cfg(feature = "tmux_2_2")]
pub const NOTIFICATION_CLIENT_DETACHED: &str = "%client-detached";
/// `%client-session-changed client session-id name`
#[cfg(feature = "tmux_2_4")]
pub const NOTIFICATION_CLIENT_SESSION_CHANGED: &str = "%client-session-changed";
/// `%continue pane-id`
// NOTE: super new?
pub const NOTIFICATION_CONTINUE: &str = "%continue";
/// `%exit [reason]`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_EXIT: &str = "%exit";
/// `%extended-output pane-id age ... : value`
// NOTE super new?
pub const NOTIFICATION_EXTENDED_OUTPUT: &str = "%extended-output";
/// tmux ^2.2 `%layout-change window-id window-layout window-visible-layout`
/// tmux ^1.8 `%layout-change window-id window-layout`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_LAYOUT_CHANGE: &str = "%layout-change";
/// `%output pane-id value`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_OUTPUT: &str = "%output";
/// `%pane-mode-changed pane-id`
#[cfg(feature = "tmux_2_5")]
pub const NOTIFICATION_PANE_MODE_CHANGED: &str = "%pane-mode-changed";
/// `%pause pane-id`
// NOTE: super new?
pub const NOTIFICATION_PAUSE: &str = "%pause";
/// `%session-changed session-id name`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_SESSION_CHANGED: &str = "%session-changed";
/// `%session-renamed name`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_SESSION_RENAMED: &str = "%session-renamed";
/// `%session-window-changed session-id window-id`
#[cfg(feature = "tmux_2_5")]
pub const NOTIFICATION_SESSION_WINDOW_CHANGED: &str = "%session-window-changed";
/// `%sessions-changed`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_SESSIONS_CHANGED: &str = "%sessions-changed";
/// `%subscription-changed name session-id window-id window-index`
// NOTE: super new?
pub const NOTIFICATION_SUBSCRIPTION_CHANGED: &str = "%subscription-changed";
/// `%unlinked-window-add window-id`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_UNLINKED_WINDOW_ADD: &str = "%unlinked-window-add";
/// `%window-add window-id`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_WINDOW_ADD: &str = "%window-add";
/// `%window-close window-id`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_WINDOW_CLOSE: &str = "%window-close";
/// `%window-pane-changed window-id pane-id`
#[cfg(feature = "tmux_2_5")]
pub const NOTIFICATION_WINDOW_PANE_CHANGED: &str = "%window-pane-changed";
/// `%window-renamed window-id name`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_WINDOW_RENAMED: &str = "%window-renamed";

/// separator in notifications, and output block (`%begin<' '>1234<' '>0`)
pub const CONTROL_MODE_SEPARATOR: char = ' ';

//pub struct ControlMode {}
//impl ControlMode {}

/// Output block (`%begin ... data ... %end/%error`)
#[derive(Default, Debug, PartialEq)]
pub struct OutputBlock {
    pub time: usize,
    pub num: usize,
    // NOTE: undocumented in tmux man
    //pub x: usize,
    pub success: bool,
    pub data: Option<String>,
}

// ADR: data = array[u8] or str[] or vec<...>?
// str vs String
impl OutputBlock {}

// XXX: rename
#[derive(Debug, PartialEq)]
pub enum Response {
    /// `%begin`
    #[cfg(feature = "tmux_1_8")]
    OutputBlockBegin(usize, usize),
    /// `%end`
    #[cfg(feature = "tmux_1_8")]
    OutputBlockEnd(usize, usize),
    /// `%error`
    #[cfg(feature = "tmux_1_8")]
    OutputBlockError(usize, usize),
    /// `...data...`
    #[cfg(feature = "tmux_1_8")]
    OutputBlockData(String),
    /// not exist as one-line tmux output, combined from parts (`%begin ... data ... %end/%error`)
    OutputBlock(OutputBlock),
    /// `%client-detached client`
    #[cfg(feature = "tmux_2_2")]
    ClientDetached(String),
    /// `%client-session-changed client session-id name`
    #[cfg(feature = "tmux_2_4")]
    ClientSessionChanged(String, String, String),
    /// `%continue pane-id`
    // NOTE: super new?
    Continue(String),
    /// `%exit [reason]`
    #[cfg(feature = "tmux_1_8")]
    Exit(String),
    /// `%extended-output pane-id age ... : value`
    // NOTE super new?
    ExtendedOutput(String, String, String),
    /// tmux ^2.2 `%layout-change window-id window-layout window-visible-layout`
    /// tmux ^1.8 `%layout-change window-id window-layout`
    #[cfg(feature = "tmux_2_2")]
    LayoutChange(String, String, String),
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_2")))]
    LayoutChange(String, String),
    /// `%output pane-id value`
    #[cfg(feature = "tmux_1_8")]
    Output(String, String),
    /// `%pane-mode-changed pane-id`
    #[cfg(feature = "tmux_2_5")]
    PaneModeChanged(String),
    /// `%pause pane-id`
    // NOTE: super new?
    Pause(String),
    /// `%session-changed session-id name`
    #[cfg(feature = "tmux_1_8")]
    SessionChanged(String, String),
    /// `%session-renamed name`
    #[cfg(feature = "tmux_1_8")]
    SessionRenamed(String),
    /// `%session-window-changed session-id window-id`
    #[cfg(feature = "tmux_2_5")]
    SessionWindowChanged(String, String),
    /// `%sessions-changed`
    #[cfg(feature = "tmux_1_8")]
    SessionsChanged,
    /// `%subscription-changed name session-id window-id window-index`
    // NOTE: super new?
    SubscriptionChanged(String, String, String, String),
    /// `%unlinked-window-add window-id`
    #[cfg(feature = "tmux_1_8")]
    UnlinkedWindowAdd(String),
    /// `%window-add window-id`
    #[cfg(feature = "tmux_1_8")]
    WindowAdd(String),
    /// `%window-close window-id`
    #[cfg(feature = "tmux_1_8")]
    WindowClose(String),
    /// `%window-pane-changed window-id pane-id`
    #[cfg(feature = "tmux_2_5")]
    WindowPaneChanged(String, String),
    /// `%window-renamed window-id name`
    #[cfg(feature = "tmux_1_8")]
    WindowRenamed(String, String),
}

#[derive(Debug)]
pub struct ControlModeOutput<B: BufRead>(pub Lines<B>);

impl<B: BufRead> ControlModeOutput<B> {
    pub fn new(s: Lines<B>) -> Self {
        ControlModeOutput(s)
    }

    // TODO: remove unwrap
    pub fn check_main(lines: &mut Lines<B>) -> Option<Response> {
        let mut _time: usize = 0;
        let mut _num: usize = 0;
        let mut output_block = OutputBlock::default();

        // checking in loop, bc 3 parts block may be returned (`%begin ... data .. %end/%error`)
        for line in lines {
            let output = ControlModeOutput::<B>::check(line.unwrap());
            if let Some(output) = output {
                // check if output is part of output block?
                match output {
                    // if output block detected combine it from parts (`%begin ... data ... %end/%error`)
                    // continue loop waiting for data and end/error
                    Response::OutputBlockBegin(t, n) => {
                        _time = t;
                        _num = n;
                    }
                    // end of output block (ended with success), break loop, got whole block
                    Response::OutputBlockEnd(t, n) => {
                        // XXX: check t, n
                        output_block.time = t;
                        output_block.num = n;
                        output_block.success = true;
                        return Some(Response::OutputBlock(output_block));
                    }
                    // end of output block (ended with an error), break loop, got whole block
                    Response::OutputBlockError(t, n) => {
                        // XXX: check t, n
                        output_block.time = t;
                        output_block.num = n;
                        output_block.success = false;
                        return Some(Response::OutputBlock(output_block));
                    }
                    // data inside of output block
                    Response::OutputBlockData(data) => output_block.data = Some(data),
                    // TODO: only as single line? output check, \n ?
                    // notification, break loop, got whole data
                    other => return Some(other),
                }
            } else {
                return None;
            }
        }
        None
    }

    // TODO: remove unwrap()
    /// function takes one line and matches it against output block keywords, notifications, or is
    /// it just data line without any keyword
    // TODO: Result/Option parsing errors?
    // mb. option for fields too, if parse errors occur?
    pub fn check<S: AsRef<str> + std::fmt::Display>(line: S) -> Option<Response> {
        match line.as_ref() {
            // start of output block
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(OUTPUT_BLOCK_BEGIN) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let time = v.get(1).unwrap().parse::<usize>().unwrap();
                let num = v.get(2).unwrap().parse::<usize>().unwrap();
                Some(Response::OutputBlockBegin(time, num))
            }

            // end of output block (successed)
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(OUTPUT_BLOCK_END) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let time = v.get(1).unwrap().parse::<usize>().unwrap();
                let num = v.get(2).unwrap().parse::<usize>().unwrap();
                Some(Response::OutputBlockEnd(time, num))
            }

            // end of output block (error)
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(OUTPUT_BLOCK_ERROR) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let time = v.get(1).unwrap().parse::<usize>().unwrap();
                let num = v.get(2).unwrap().parse::<usize>().unwrap();
                Some(Response::OutputBlockError(time, num))
            }

            // `%client-detached client`
            #[cfg(feature = "tmux_2_2")]
            s if s.starts_with(NOTIFICATION_CLIENT_DETACHED) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let client = v.get(1).unwrap();
                Some(Response::ClientDetached(client.to_string()))
            }

            // `%client-session-changed client session-id name`
            #[cfg(feature = "tmux_2_4")]
            s if s.starts_with(NOTIFICATION_CLIENT_SESSION_CHANGED) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let client = v.get(1).unwrap();
                let session_id = v.get(2).unwrap();
                let name = v.get(3).unwrap();
                Some(Response::ClientSessionChanged(
                    client.to_string(),
                    session_id.to_string(),
                    name.to_string(),
                ))
            }

            // `%continue pane-id`
            // NOTE: super new?
            s if s.starts_with(NOTIFICATION_CONTINUE) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let pane_id = v.get(1).unwrap();
                Some(Response::Continue(pane_id.to_string()))
            }

            // `%exit [reason]`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_EXIT) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                // NOTE: optional
                let reason = v.get(1).unwrap();
                Some(Response::Exit(reason.to_string()))
            }

            // `%extended-output pane-id age ... : value`
            // NOTE: super new?
            s if s.starts_with(NOTIFICATION_EXTENDED_OUTPUT) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let pane_id = v.get(1).unwrap();
                let age = v.get(2).unwrap();
                let value = v.get(3).unwrap();
                Some(Response::ExtendedOutput(
                    pane_id.to_string(),
                    age.to_string(),
                    value.to_string(),
                ))
            }

            // `%layout-change window-id window-layout window-visible-layout`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_LAYOUT_CHANGE) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).unwrap();
                let window_layout = v.get(2).unwrap();
                #[cfg(feature = "tmux_2_2")]
                let window_visible_layout = v.get(3).unwrap();

                #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_2")))]
                return Some(Response::LayoutChange(
                    window_id.to_string(),
                    window_layout.to_string(),
                ));

                #[cfg(feature = "tmux_2_2")]
                return Some(Response::LayoutChange(
                    window_id.to_string(),
                    window_layout.to_string(),
                    window_visible_layout.to_string(),
                ));
            }

            // `%output pane-id value`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_OUTPUT) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let pane_id = v.get(1).unwrap();
                let value = v.get(2).unwrap();
                Some(Response::Output(pane_id.to_string(), value.to_string()))
            }

            // `%pane-mode-changed pane-id`
            #[cfg(feature = "tmux_2_5")]
            s if s.starts_with(NOTIFICATION_PANE_MODE_CHANGED) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let pane_id = v.get(1).unwrap();
                Some(Response::PaneModeChanged(pane_id.to_string()))
            }

            // `%pause pane-id`
            // NOTE: super new?
            s if s.starts_with(NOTIFICATION_PAUSE) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let pane_id = v.get(1).unwrap();
                Some(Response::Pause(pane_id.to_string()))
            }

            // `%session-changed session-id name`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_SESSION_CHANGED) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let session_id = v.get(1).unwrap();
                let name = v.get(2).unwrap();
                Some(Response::SessionChanged(
                    session_id.to_string(),
                    name.to_string(),
                ))
            }

            // `%session-renamed name`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_SESSION_RENAMED) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let name = v.get(1).unwrap();
                Some(Response::SessionRenamed(name.to_string()))
            }

            // `%session-window-changed session-id window-id`
            #[cfg(feature = "tmux_2_5")]
            s if s.starts_with(NOTIFICATION_SESSION_WINDOW_CHANGED) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let session_id = v.get(1).unwrap();
                let window_id = v.get(2).unwrap();
                Some(Response::SessionWindowChanged(
                    session_id.to_string(),
                    window_id.to_string(),
                ))
            }

            // `%sessions-changed`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_SESSIONS_CHANGED) => Some(Response::SessionsChanged),

            // `%subscription-changed name session-id window-id window-index`
            // NOTE: super new?
            s if s.starts_with(NOTIFICATION_SUBSCRIPTION_CHANGED) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let name = v.get(1).unwrap();
                let session_id = v.get(2).unwrap();
                let window_id = v.get(3).unwrap();
                let window_index = v.get(4).unwrap();
                Some(Response::SubscriptionChanged(
                    name.to_string(),
                    session_id.to_string(),
                    window_id.to_string(),
                    window_index.to_string(),
                ))
            }

            // `%unlinked-window-add window-id`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_UNLINKED_WINDOW_ADD) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).unwrap();
                Some(Response::UnlinkedWindowAdd(window_id.to_string()))
            }

            // `%window-add window-id`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_WINDOW_ADD) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).unwrap();
                Some(Response::WindowAdd(window_id.to_string()))
            }

            // `%window-close window-id`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_WINDOW_CLOSE) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).unwrap();
                Some(Response::WindowClose(window_id.to_string()))
            }

            // `%window-pane-changed window-id pane-id`
            #[cfg(feature = "tmux_2_5")]
            s if s.starts_with(NOTIFICATION_WINDOW_PANE_CHANGED) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).unwrap();
                let pane_id = v.get(2).unwrap();
                Some(Response::WindowPaneChanged(
                    window_id.to_string(),
                    pane_id.to_string(),
                ))
            }

            // `%window-renamed window-id name`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_WINDOW_RENAMED) => {
                let v: Vec<_> = s.split(CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).unwrap();
                let name = v.get(2).unwrap();
                Some(Response::WindowRenamed(
                    window_id.to_string(),
                    name.to_string(),
                ))
            }

            // `...` - data inside `%begin ... %end`
            #[cfg(feature = "tmux_1_8")]
            _ => Some(Response::OutputBlockData(line.to_string())),
        }
    }
}

impl<B: BufRead> Iterator for ControlModeOutput<B> {
    type Item = Response;

    fn next(&mut self) -> Option<Self::Item> {
        ControlModeOutput::check_main(&mut self.0)
    }
}

//impl<'a> From<&'a str> for ControlModeOutput<'a> {
//fn from(item: &'a str) -> Self {
//ControlModeOutput::new(item)
//}
//}
