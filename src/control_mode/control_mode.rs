use crate::control_mode::constants::*;
use crate::Error;
use crate::TmuxCommand;
use std::io::BufRead;
use std::io::Lines;
use std::io::Write;

// 1. send
// 2. receive
//   2.1. ControlModeLine - line by line reading
//   2.2. Response - build multiline block if needed
//   2.3. match response {}

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

// XXX: rename to notification
// XXX: String -> &'a str?
#[derive(Debug, PartialEq)]
pub enum Response {
    /// %begin seconds-from-epoch command-number flags
    #[cfg(feature = "tmux_1_8")]
    OutputBlockBegin {
        time: usize,
        num: usize,
        flags: usize,
    },
    /// %end seconds-from-epoch command-number flags
    #[cfg(feature = "tmux_1_8")]
    OutputBlockEnd {
        time: usize,
        num: usize,
        flags: usize,
    },
    /// %error seconds-from-epoch command-number flags
    #[cfg(feature = "tmux_1_8")]
    OutputBlockError {
        time: usize,
        num: usize,
        flags: usize,
    },
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
    ClientSessionChanged {
        client: String,
        session_id: String,
        name: String,
    },
    /// `%continue pane-id`
    #[cfg(feature = "tmux_X_X")]
    Continue(String),
    /// `%exit [reason]`
    #[cfg(feature = "tmux_1_8")]
    Exit(Option<String>),
    /// `%extended-output pane-id age ... : value`
    #[cfg(feature = "tmux_X_X")]
    ExtendedOutput {
        pane_id: String,
        age: String,
        reserved: Vec<String>,
        value: String,
    },
    /// tmux ^3.3 `%layout-change window-id window-layout window-visible-layout window-flags`
    /// tmux ^2.2 `%layout-change window-id window-layout window-visible-layout`
    /// tmux ^1.8 `%layout-change window-id window-layout`
    #[cfg(feature = "tmux_1_8")]
    LayoutChange {
        window_id: String,
        window_layout: String,
        #[cfg(feature = "tmux_2_2")]
        window_visible_layout: String,
        #[cfg(feature = "tmux_2_2")]
        window_flags: String,
    },
    /// `%output pane-id value`
    #[cfg(feature = "tmux_1_8")]
    Output { pane_id: String, value: String },
    /// `%pane-mode-changed pane-id`
    #[cfg(feature = "tmux_2_5")]
    PaneModeChanged(String),
    /// `%pause pane-id`
    #[cfg(feature = "tmux_X_X")]
    Pause(String),
    /// `%session-changed session-id name`
    #[cfg(feature = "tmux_1_8")]
    SessionChanged { session_id: String, name: String },
    /// `%session-renamed name`
    #[cfg(feature = "tmux_1_8")]
    SessionRenamed(String),
    /// `%session-window-changed session-id window-id`
    #[cfg(feature = "tmux_2_5")]
    SessionWindowChanged {
        session_id: String,
        window_id: String,
    },
    /// `%sessions-changed`
    #[cfg(feature = "tmux_1_8")]
    SessionsChanged,
    /// `%subscription-changed name session-id window-id window-index`
    #[cfg(feature = "tmux_X_X")]
    SubscriptionChanged {
        name: String,
        session_id: String,
        window_id: String,
        window_index: String,
    },
    /// `%unlinked-window-add window-id`
    #[cfg(feature = "tmux_1_8")]
    UnlinkedWindowAdd(String),
    /// `%unlinked-window-close window-id`
    #[cfg(feature = "tmux_3_3")]
    UnlinkedWindowClose(String),
    /// `%unlinked-window-renamed window-id`
    #[cfg(feature = "tmux_3_3")]
    UnlinkedWindowRenamed(String),
    /// `%window-add window-id`
    #[cfg(feature = "tmux_1_8")]
    WindowAdd(String),
    /// `%window-close window-id`
    #[cfg(feature = "tmux_1_8")]
    WindowClose(String),
    /// `%window-pane-changed window-id pane-id`
    #[cfg(feature = "tmux_2_5")]
    WindowPaneChanged { window_id: String, pane_id: String },
    /// `%window-renamed window-id name`
    #[cfg(feature = "tmux_1_8")]
    WindowRenamed { window_id: String, name: String },
}

// wrapper structure around Lines type, which is Iterator
//  which implements BufRead trait <B: BufRead>
#[derive(Debug)]
pub struct ControlModeOutput<B: BufRead>(pub Lines<B>);

use std::process::ChildStdin;

// two types of response
// 1. Output (%begin .. %output .. %end)
// 2. Notification
//
// XXX: mb. BufReader as input?
// iterator strategy because notification may consist of 3 output lines (%begin...%output...%end)
//  3 lines will be "merged" into one resulting output notification
//
//  NOTE:
//  > A notification will never occur inside an output block
//  [tmux man](https://man7.org/linux/man-pages/man1/tmux.1.html#CONTROL_MODE)
//
//  XXX: is it possible, two or more output blocks can be recieved mixed?
//  (similar like network packets -> buffering -> queueing -> merging)
impl<B: BufRead> ControlModeOutput<B> {
    // create new from multiple Lines
    pub fn new(s: Lines<B>) -> Self {
        ControlModeOutput(s)
    }

    //pub fn event_loop(mut cm_lines: ControlModeOutput<B>, cb: &mut dyn FnMut(Response)) {
    //while let Some(cm_line) = cm_lines.next() {
    //cb(cm_line);
    //}
    //}
    //}

    // TODO: error
    /// Send command to stdin of a child tmux process, opened in control mode, and get response
    /// data as `OutputBlock`
    ///
    /// Tmux Man
    /// > Each command will produce one block of output on standard output
    ///
    pub fn send<'a, T: Into<TmuxCommand<'a>>>(
        stdin: &mut ChildStdin,
        cmd: T,
        lines: &mut ControlModeOutput<B>,
    ) -> Result<OutputBlock, Error> {
        // send command
        writeln!(stdin, "{}", cmd.into())?;

        // receive response
        match lines.next() {
            Some(line) => match line {
                Response::OutputBlock(data) => Ok(data),
                _ => Err(Error::Tmux(String::from("error response"))),
            },
            None => Err(Error::Tmux(String::from("none"))),
        }
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
                    Response::OutputBlockBegin { time, num, flags } => {
                        _time = time;
                        _num = num;
                        _flags = flags;
                    }
                    // end of output block (ended with success), break loop, got whole block
                    Response::OutputBlockEnd { time, num, flags } => {
                        // XXX: check t, n
                        output_block.time = time;
                        output_block.num = num;
                        output_block.flags = flags;
                        output_block.success = true;
                        return Some(Response::OutputBlock(output_block));
                    }
                    // end of output block (ended with an error), break loop, got whole block
                    Response::OutputBlockError { time, num, flags } => {
                        // XXX: check t, n
                        output_block.time = time;
                        output_block.num = num;
                        output_block.flags = flags;
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

// https://dev.to/dandyvica/yarit-yet-another-rust-iterators-tutorial-46dk
//impl<B: BufRead> IntoIterator for &mut ControlModeOutput<B> {
//type Item = Response;

//fn next(&mut self) -> Option<Self::Item> {
//ControlModeOutput::check_main(&mut self.0)
//}
//}

//iterator which can return merged output block, or notification
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

fn parse_option_string(s: &str) -> Option<String> {
    if !s.is_empty() {
        Some(s.to_string())
    } else {
        None
    }
}

// fn option2usize(s: Option<&&str>) -> Result<usize, Error> {
// s.ok_or(Error::Hook)?.parse::<usize>().ok()
// }

//
impl<S: AsRef<str> + std::fmt::Display> ControlModeLine for S {
    // TODO: remove.ok_or(Error::CMParseNum)?
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
                let time = v.get(1).ok_or(Error::CMParseNum)?.parse::<usize>()?;
                let num = v.get(2).ok_or(Error::CMParseNum)?.parse::<usize>()?;
                let flags = v.get(3).ok_or(Error::CMParseNum)?.parse::<usize>()?;
                Ok(Response::OutputBlockBegin { time, num, flags })
            }

            // end of output block (successed)
            // %end seconds-from-epoch command-number flags
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(OUTPUT_BLOCK_END) => {
                let v: Vec<_> = s.splitn(4, CONTROL_MODE_SEPARATOR).collect();
                let time = v.get(1).ok_or(Error::CMParseNum)?.parse::<usize>()?;
                let num = v.get(2).ok_or(Error::CMParseNum)?.parse::<usize>()?;
                let flags = v.get(3).ok_or(Error::CMParseNum)?.parse::<usize>()?;
                Ok(Response::OutputBlockEnd { time, num, flags })
            }

            // end of output block (error)
            // %error seconds-from-epoch command-number flags
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(OUTPUT_BLOCK_ERROR) => {
                let v: Vec<_> = s.splitn(4, CONTROL_MODE_SEPARATOR).collect();
                let time = v.get(1).ok_or(Error::CMParseNum)?.parse::<usize>()?;
                let num = v.get(2).ok_or(Error::CMParseNum)?.parse::<usize>()?;
                let flags = v.get(3).ok_or(Error::CMParseNum)?.parse::<usize>()?;
                Ok(Response::OutputBlockError { time, num, flags })
            }

            // `%client-detached client`
            #[cfg(feature = "tmux_3_2")]
            s if s.starts_with(NOTIFICATION_CLIENT_DETACHED) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let client = v.get(1).ok_or(Error::CMParseStr)?;
                Ok(Response::ClientDetached(client.to_string()))
            }

            // `%client-session-changed client session-id name`
            #[cfg(feature = "tmux_2_4")]
            s if s.starts_with(NOTIFICATION_CLIENT_SESSION_CHANGED) => {
                let v: Vec<_> = s.splitn(4, CONTROL_MODE_SEPARATOR).collect();
                let client = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                let session_id = v.get(2).ok_or(Error::CMParseStr)?.to_string();
                let name = v.get(3).ok_or(Error::CMParseStr)?.to_string();
                Ok(Response::ClientSessionChanged {
                    client,
                    session_id,
                    name,
                })
            }

            // `%continue pane-id`
            #[cfg(feature = "tmux_X_X")]
            s if s.starts_with(NOTIFICATION_CONTINUE) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let pane_id = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                Ok(Response::Continue(pane_id))
            }

            // `%exit [reason]`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_EXIT) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                // NOTE: optional
                let reason = v.get(1).map(|s| s.to_string());
                Ok(Response::Exit(reason))
            }

            // TODO: check varargs?
            // `%extended-output pane-id age ... : value`
            #[cfg(feature = "tmux_X_X")]
            s if s.starts_with(NOTIFICATION_EXTENDED_OUTPUT) => {
                // split using " : " in two parts
                let v: Vec<_> = s
                    .splitn(2, CONTROL_MODE_EXTENDED_OUTPUT_SEPARATOR)
                    .collect();
                let s = v.get(0).ok_or(Error::CMParseStr)?;
                let value = v.get(1).ok_or(Error::CMParseStr)?.to_string();

                // split first part using ' '
                let v: Vec<_> = s
                    .split(CONTROL_MODE_SEPARATOR)
                    .map(|w| w.to_string())
                    .collect();
                let pane_id = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                let age = v.get(2).ok_or(Error::CMParseStr)?.to_string();

                // XXX: if v[3] not exists?
                let reserved = v[3..].to_vec();
                Ok(Response::ExtendedOutput {
                    pane_id,
                    age,
                    reserved,
                    value,
                })
            }

            // `%layout-change window-id window-layout window-visible-layout window-flags`
            // `%layout-change window-id window-layout`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_LAYOUT_CHANGE) => {
                #[cfg(feature = "tmux_2_2")]
                let v: Vec<_> = s.splitn(5, CONTROL_MODE_SEPARATOR).collect();
                #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_2")))]
                let v: Vec<_> = s.splitn(3, CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                let window_layout = v.get(2).ok_or(Error::CMParseStr)?.to_string();
                #[cfg(feature = "tmux_2_2")]
                let window_visible_layout = v.get(3).ok_or(Error::CMParseStr)?.to_string();
                #[cfg(feature = "tmux_2_2")]
                let window_flags = v.get(4).ok_or(Error::CMParseStr)?.to_string();

                #[cfg(feature = "tmux_1_8")]
                return Ok(Response::LayoutChange {
                    window_id,
                    window_layout,
                    #[cfg(feature = "tmux_2_2")]
                    window_visible_layout,
                    #[cfg(feature = "tmux_2_2")]
                    window_flags,
                });
            }

            // `%output pane-id value`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_OUTPUT) => {
                let v: Vec<_> = s.splitn(3, CONTROL_MODE_SEPARATOR).collect();
                let pane_id = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                let value = v.get(2).ok_or(Error::CMParseStr)?.to_string();
                Ok(Response::Output { pane_id, value })
            }

            // `%pane-mode-changed pane-id`
            #[cfg(feature = "tmux_2_5")]
            s if s.starts_with(NOTIFICATION_PANE_MODE_CHANGED) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let pane_id = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                Ok(Response::PaneModeChanged(pane_id))
            }

            // `%pause pane-id`
            #[cfg(feature = "tmux_X_X")]
            s if s.starts_with(NOTIFICATION_PAUSE) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let pane_id = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                Ok(Response::Pause(pane_id))
            }

            // `%session-changed session-id name`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_SESSION_CHANGED) => {
                let v: Vec<_> = s.splitn(3, CONTROL_MODE_SEPARATOR).collect();
                let session_id = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                let name = v.get(2).ok_or(Error::CMParseStr)?.to_string();
                Ok(Response::SessionChanged {
                    session_id,
                    name: name,
                })
            }

            // `%session-renamed name`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_SESSION_RENAMED) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let name = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                Ok(Response::SessionRenamed(name))
            }

            // `%session-window-changed session-id window-id`
            #[cfg(feature = "tmux_2_5")]
            s if s.starts_with(NOTIFICATION_SESSION_WINDOW_CHANGED) => {
                let v: Vec<_> = s.splitn(3, CONTROL_MODE_SEPARATOR).collect();
                let session_id = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                let window_id = v.get(2).ok_or(Error::CMParseStr)?.to_string();
                Ok(Response::SessionWindowChanged {
                    session_id,
                    window_id,
                })
            }

            // `%sessions-changed`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_SESSIONS_CHANGED) => Ok(Response::SessionsChanged),

            // `%subscription-changed name session-id window-id window-index`
            #[cfg(feature = "tmux_X_X")]
            s if s.starts_with(NOTIFICATION_SUBSCRIPTION_CHANGED) => {
                let v: Vec<_> = s.splitn(5, CONTROL_MODE_SEPARATOR).collect();
                let name = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                let session_id = v.get(2).ok_or(Error::CMParseStr)?.to_string();
                let window_id = v.get(3).ok_or(Error::CMParseStr)?.to_string();
                let window_index = v.get(4).ok_or(Error::CMParseStr)?.to_string();
                Ok(Response::SubscriptionChanged {
                    name,
                    session_id,
                    window_id,
                    window_index,
                })
            }

            // `%unlinked-window-add window-id`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_UNLINKED_WINDOW_ADD) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                Ok(Response::UnlinkedWindowAdd(window_id))
            }

            // `%unlinked-window-close window-id`
            #[cfg(feature = "tmux_3_3")]
            s if s.starts_with(NOTIFICATION_UNLINKED_WINDOW_CLOSE) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                Ok(Response::UnlinkedWindowClose(window_id))
            }

            // `%unlinked-window-renamed window-id`
            #[cfg(feature = "tmux_3_3")]
            s if s.starts_with(NOTIFICATION_UNLINKED_WINDOW_RENAMED) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                Ok(Response::UnlinkedWindowRenamed(window_id))
            }

            // `%window-add window-id`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_WINDOW_ADD) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                Ok(Response::WindowAdd(window_id))
            }

            // `%window-close window-id`
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_WINDOW_CLOSE) => {
                let v: Vec<_> = s.splitn(2, CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                Ok(Response::WindowClose(window_id))
            }

            // `%window-pane-changed window-id pane-id`
            #[cfg(feature = "tmux_2_5")]
            s if s.starts_with(NOTIFICATION_WINDOW_PANE_CHANGED) => {
                let v: Vec<_> = s.splitn(3, CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                let pane_id = v.get(2).ok_or(Error::CMParseStr)?.to_string();
                Ok(Response::WindowPaneChanged { window_id, pane_id })
            }

            // `%window-renamed window-id name`
            // example:
            // %window-renamed @0 asdfasdf asdf
            #[cfg(feature = "tmux_1_8")]
            s if s.starts_with(NOTIFICATION_WINDOW_RENAMED) => {
                let v: Vec<_> = s.splitn(3, CONTROL_MODE_SEPARATOR).collect();
                let window_id = v.get(1).ok_or(Error::CMParseStr)?.to_string();
                let name = v.get(2).ok_or(Error::CMParseStr)?.to_string();
                Ok(Response::WindowRenamed { window_id, name })
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
