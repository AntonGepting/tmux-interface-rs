use crate::Error;
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
#[cfg(feature = "tmux_3_2")]
pub const NOTIFICATION_CLIENT_DETACHED: &str = "%client-detached";
/// `%client-session-changed client session-id name`
#[cfg(feature = "tmux_2_4")]
pub const NOTIFICATION_CLIENT_SESSION_CHANGED: &str = "%client-session-changed";
/// `%continue pane-id`
#[cfg(feature = "tmux_X_X")]
pub const NOTIFICATION_CONTINUE: &str = "%continue";
/// `%exit [reason]`
#[cfg(feature = "tmux_1_8")]
pub const NOTIFICATION_EXIT: &str = "%exit";
/// `%extended-output pane-id age ... : value`
#[cfg(feature = "tmux_X_X")]
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
#[cfg(feature = "tmux_X_X")]
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
#[cfg(feature = "tmux_X_X")]
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
/// additional separator used in extended-output notification
pub const CONTROL_MODE_EXTENDED_OUTPUT_SEPARATOR: &str = " : ";

//pub struct ControlMode {}
//impl ControlMode {}

/// Output block (`%begin ... data ... %end/%error`)
#[derive(Default, Debug, PartialEq)]
pub struct OutputBlock {
    pub time: usize,
    pub num: usize,
    pub flags: usize,
    pub success: bool,
    pub data: Option<String>,
}

// ADR: data = array[u8] or str[] or vec<...>?
// str vs String
impl OutputBlock {}

// ADR: OutputBlock begin/end/error need some mid structure, this is output for user
//enum ControlModeOutput {
//OutputBlock(OutputBlock),
//Notification<Notification>
//}

// XXX: rename
// XXX: String -> &'a str?
#[derive(Debug, PartialEq)]
pub enum Response {
    /// %begin seconds-from-epoch command-number flags
    #[cfg(feature = "tmux_1_8")]
    OutputBlockBegin(usize, usize, usize),
    /// %end seconds-from-epoch command-number flags
    #[cfg(feature = "tmux_1_8")]
    OutputBlockEnd(usize, usize, usize),
    /// %error seconds-from-epoch command-number flags
    #[cfg(feature = "tmux_1_8")]
    OutputBlockError(usize, usize, usize),
    /// `...data...`
    #[cfg(feature = "tmux_1_8")]
    OutputBlockData(String),
    /// not exist as one-line tmux output, combined from parts (`%begin ... data ... %end/%error`)
    #[cfg(feature = "tmux_1_8")]
    OutputBlock(OutputBlock),
    /// `%client-detached client`
    #[cfg(feature = "tmux_3_2")]
    ClientDetached(String),
    /// `%client-session-changed client session-id name`
    #[cfg(feature = "tmux_2_4")]
    ClientSessionChanged(String, String, String),
    /// `%continue pane-id`
    #[cfg(feature = "tmux_X_X")]
    Continue(String),
    /// `%exit [reason]`
    #[cfg(feature = "tmux_1_8")]
    Exit(Option<String>),
    /// `%extended-output pane-id age ... : value`
    #[cfg(feature = "tmux_X_X")]
    ExtendedOutput(String, String, Vec<String>, String),
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
    #[cfg(feature = "tmux_X_X")]
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
    #[cfg(feature = "tmux_X_X")]
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
    // TODO: rename
    pub fn check_main(lines: &mut Lines<B>) -> Option<Response> {
        let mut _time: usize = 0;
        let mut _num: usize = 0;
        let mut _flags: usize = 0;
        let mut output_block = OutputBlock::default();

        // checking in loop, because 3 parts block may be returned, which must be merged
        // (`%begin ...  data .. %end/%error`)
        for line in lines {
            let output = line.unwrap().control_mode_line();
            if let Ok(output) = output {
                // check if output is part of output block?
                match output {
                    // if output block detected combine it from parts (`%begin ... data ... %end/%error`)
                    // continue loop waiting for data and end/error
                    Response::OutputBlockBegin(t, n, f) => {
                        _time = t;
                        _num = n;
                        _flags = f;
                    }
                    // end of output block (ended with success), break loop, got whole block
                    Response::OutputBlockEnd(t, n, f) => {
                        // XXX: check t, n
                        output_block.time = t;
                        output_block.num = n;
                        output_block.flags = f;
                        output_block.success = true;
                        return Some(Response::OutputBlock(output_block));
                    }
                    // end of output block (ended with an error), break loop, got whole block
                    Response::OutputBlockError(t, n, f) => {
                        // XXX: check t, n
                        output_block.time = t;
                        output_block.num = n;
                        output_block.flags = f;
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
}

impl<B: BufRead> Iterator for ControlModeOutput<B> {
    type Item = Response;

    fn next(&mut self) -> Option<Self::Item> {
        ControlModeOutput::check_main(&mut self.0)
    }
}

// single line process
pub trait ControlModeLine {
    fn control_mode_line(&self) -> Result<Response, Error>;
}

//
impl<S: AsRef<str> + std::fmt::Display> ControlModeLine for S {
    // TODO: remove unwrap()
    /// function takes one line and matches it against output block keywords, notifications, or is
    /// it just data line without any keyword
    // TODO: Result/Option parsing errors?
    // mb. option for fields too, if parse errors occur?
    fn control_mode_line(&self) -> Result<Response, Error> {
        match self.as_ref() {
            // start of output block
            // %begin seconds-from-epoch command-number flags
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(OUTPUT_BLOCK_BEGIN) => {
                let v: Vec<_> = s.splitn(4, CONTROL_MODE_SEPARATOR).collect();
                let time = v.get(1).unwrap().parse::<usize>()?;
                let num = v.get(2).unwrap().parse::<usize>()?;
                let flags = v.get(3).unwrap().parse::<usize>()?;
                Ok(Response::OutputBlockBegin(time, num, flags))
            }

            // end of output block (successed)
            // %end seconds-from-epoch command-number flags
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(OUTPUT_BLOCK_END) => {
                let v: Vec<_> = s.splitn(4, CONTROL_MODE_SEPARATOR).collect();
                let time = v.get(1).unwrap().parse::<usize>()?;
                let num = v.get(2).unwrap().parse::<usize>()?;
                let flags = v.get(3).unwrap().parse::<usize>()?;
                Ok(Response::OutputBlockEnd(time, num, flags))
            }

            // end of output block (error)
            // %error seconds-from-epoch command-number flags
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(OUTPUT_BLOCK_ERROR) => {
                let v: Vec<_> = s.splitn(4, CONTROL_MODE_SEPARATOR).collect();
                let time = v.get(1).unwrap().parse::<usize>()?;
                let num = v.get(2).unwrap().parse::<usize>()?;
                let flags = v.get(3).unwrap().parse::<usize>()?;
                Ok(Response::OutputBlockError(time, num, flags))
            }

            // `%client-detached client`
            #[cfg(feature = "tmux_3_2")]
            s if s.starts_with(NOTIFICATION_CLIENT_DETACHED) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let client = v.get(1).unwrap();
                Ok(Response::ClientDetached(client.to_string()))
            }

            // `%client-session-changed client session-id name`
            #[cfg(feature = "tmux_2_4")]
            s if s.starts_with(NOTIFICATION_CLIENT_SESSION_CHANGED) => {
                let v: Vec<_> = s.splitn(4, CONTROL_MODE_SEPARATOR).collect();
                let client = v.get(1).unwrap();
                let session_id = v.get(2).unwrap();
                let name = v.get(3).unwrap();
                Ok(Response::ClientSessionChanged(
                    client.to_string(),
                    session_id.to_string(),
                    name.to_string(),
                ))
            }

            // `%continue pane-id`
            #[cfg(feature = "tmux_X_X")]
            s if s.starts_with(NOTIFICATION_CONTINUE) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let pane_id = v.get(1).unwrap();
                Ok(Response::Continue(pane_id.to_string()))
            }

            // `%exit [reason]`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_EXIT) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                // NOTE: optional
                let reason = v.get(1);
                Ok(Response::Exit(reason.map(|s| s.to_string())))
            }

            // TODO: check varargs?
            // `%extended-output pane-id age ... : value`
            #[cfg(feature = "tmux_X_X")]
            s if s.starts_with(NOTIFICATION_EXTENDED_OUTPUT) => {
                // split using " : " in two parts
                let v: Vec<_> = s
                    .splitn(2, CONTROL_MODE_EXTENDED_OUTPUT_SEPARATOR)
                    .collect();
                let s = v.get(0).unwrap();
                let value = v.get(1).unwrap();

                // split first part using ' '
                let v: Vec<_> = s
                    .split(CONTROL_MODE_SEPARATOR)
                    .map(|w| w.to_string())
                    .collect();
                let pane_id = v.get(1).unwrap();
                let age = v.get(2).unwrap();

                // XXX: if v[3] not exists?
                let reserved = v[3..].to_vec();
                Ok(Response::ExtendedOutput(
                    pane_id.to_string(),
                    age.to_string(),
                    reserved,
                    value.to_string(),
                ))
            }

            // `%layout-change window-id window-layout window-visible-layout`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_LAYOUT_CHANGE) => {
                #[cfg(feature = "tmux_2_2")]
                let v: Vec<_> = s.splitn(4, CONTROL_MODE_SEPARATOR).collect();
                #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_2")))]
                let v: Vec<_> = s.splitn(3, CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).unwrap();
                let window_layout = v.get(2).unwrap();
                #[cfg(feature = "tmux_2_2")]
                let window_visible_layout = v.get(3).unwrap();

                #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_2")))]
                return Ok(Response::LayoutChange(
                    window_id.to_string(),
                    window_layout.to_string(),
                ));

                #[cfg(feature = "tmux_2_2")]
                return Ok(Response::LayoutChange(
                    window_id.to_string(),
                    window_layout.to_string(),
                    window_visible_layout.to_string(),
                ));
            }

            // `%output pane-id value`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_OUTPUT) => {
                let v: Vec<_> = s.splitn(3, CONTROL_MODE_SEPARATOR).collect();
                let pane_id = v.get(1).unwrap();
                let value = v.get(2).unwrap();
                Ok(Response::Output(pane_id.to_string(), value.to_string()))
            }

            // `%pane-mode-changed pane-id`
            #[cfg(feature = "tmux_2_5")]
            s if s.starts_with(NOTIFICATION_PANE_MODE_CHANGED) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let pane_id = v.get(1).unwrap();
                Ok(Response::PaneModeChanged(pane_id.to_string()))
            }

            // `%pause pane-id`
            #[cfg(feature = "tmux_X_X")]
            s if s.starts_with(NOTIFICATION_PAUSE) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let pane_id = v.get(1).unwrap();
                Ok(Response::Pause(pane_id.to_string()))
            }

            // `%session-changed session-id name`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_SESSION_CHANGED) => {
                let v: Vec<_> = s.splitn(3, CONTROL_MODE_SEPARATOR).collect();
                let session_id = v.get(1).unwrap();
                let name = v.get(2).unwrap();
                Ok(Response::SessionChanged(
                    session_id.to_string(),
                    name.to_string(),
                ))
            }

            // `%session-renamed name`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_SESSION_RENAMED) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let name = v.get(1).unwrap();
                Ok(Response::SessionRenamed(name.to_string()))
            }

            // `%session-window-changed session-id window-id`
            #[cfg(feature = "tmux_2_5")]
            s if s.starts_with(NOTIFICATION_SESSION_WINDOW_CHANGED) => {
                let v: Vec<_> = s.splitn(3, CONTROL_MODE_SEPARATOR).collect();
                let session_id = v.get(1).unwrap();
                let window_id = v.get(2).unwrap();
                Ok(Response::SessionWindowChanged(
                    session_id.to_string(),
                    window_id.to_string(),
                ))
            }

            // `%sessions-changed`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_SESSIONS_CHANGED) => Ok(Response::SessionsChanged),

            // `%subscription-changed name session-id window-id window-index`
            #[cfg(feature = "tmux_X_X")]
            s if s.starts_with(NOTIFICATION_SUBSCRIPTION_CHANGED) => {
                let v: Vec<_> = s.splitn(5, CONTROL_MODE_SEPARATOR).collect();
                let name = v.get(1).unwrap();
                let session_id = v.get(2).unwrap();
                let window_id = v.get(3).unwrap();
                let window_index = v.get(4).unwrap();
                Ok(Response::SubscriptionChanged(
                    name.to_string(),
                    session_id.to_string(),
                    window_id.to_string(),
                    window_index.to_string(),
                ))
            }

            // `%unlinked-window-add window-id`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_UNLINKED_WINDOW_ADD) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).unwrap();
                Ok(Response::UnlinkedWindowAdd(window_id.to_string()))
            }

            // `%window-add window-id`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_WINDOW_ADD) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).unwrap();
                Ok(Response::WindowAdd(window_id.to_string()))
            }

            // `%window-close window-id`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_WINDOW_CLOSE) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).unwrap();
                Ok(Response::WindowClose(window_id.to_string()))
            }

            // `%window-pane-changed window-id pane-id`
            #[cfg(feature = "tmux_2_5")]
            s if s.starts_with(NOTIFICATION_WINDOW_PANE_CHANGED) => {
                let v: Vec<_> = s.splitn(3, CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).unwrap();
                let pane_id = v.get(2).unwrap();
                Ok(Response::WindowPaneChanged(
                    window_id.to_string(),
                    pane_id.to_string(),
                ))
            }

            // `%window-renamed window-id name`
            // example:
            // %window-renamed @0 asdfasdf asdf
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_WINDOW_RENAMED) => {
                let v: Vec<_> = s.splitn(3, CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).unwrap();
                let name = v.get(2).unwrap();
                Ok(Response::WindowRenamed(
                    window_id.to_string(),
                    name.to_string(),
                ))
            }

            // `...` - data inside `%begin ... %end`
            #[cfg(feature = "tmux_1_8")]
            _ => Ok(Response::OutputBlockData(self.to_string())),
        }
    }
}

//impl<'a> From<&'a str> for ControlModeOutput<'a> {
//fn from(item: &'a str) -> Self {
//ControlModeOutput::new(item)
//}
//}
