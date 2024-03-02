use crate::control_mode::control_mode::{ControlModeOutput, Response};
use crate::NewWindow;
use std::io::{BufRead, BufReader};
use std::process::{Child, ChildStdin, ChildStdout};

pub fn control_proc(
    stdin: &mut ChildStdin,
    lines: &mut ControlModeOutput<BufReader<ChildStdout>>,
    response: Response,
) -> bool {
    //dbg!(&response);

    match &response {
        #[cfg(feature = "tmux_2_5")]
        Response::SessionWindowChanged {
            session_id,
            window_id,
        } => {
            let cmd = NewWindow::new().detached();
            ControlModeOutput::send(stdin, cmd, lines).unwrap();
        }
        _ => {}
    }

    true
}

pub struct ControlModeCtl;

impl ControlModeCtl {
    // tmux - as Child Process
    // control_proc - function for processing messages from tmux
    //  ChildStdin - tmux.stdin
    //  ControlModeOutput<BufReader<ChildStdout>> - tmux output reader
    //  Response - tmux output, already parsed into Response Struct
    pub fn monitor(
        tmux: Child,
        control_proc: &dyn Fn(
            &mut ChildStdin,
            &mut ControlModeOutput<BufReader<ChildStdout>>,
            Response,
        ) -> bool,
    ) {
        let stdout = tmux.stdout.unwrap();
        let mut stdin = tmux.stdin.unwrap();

        let reader = BufReader::new(stdout);

        let mut lines = ControlModeOutput::new(reader.lines());

        while let Some(response) = lines.next() {
            if !control_proc(&mut stdin, &mut lines, response) {
                break;
            }
        }
    }
}

pub fn tmux_control_proc(
    stdin: &mut ChildStdin,
    lines: &mut ControlModeOutput<BufReader<ChildStdout>>,
    response: Response,
) -> bool {
    //dbg!("response: ", &response);

    match &response {
        #[cfg(feature = "tmux_1_8")]
        Response::WindowClose(_) => {
            let cmd = NewWindow::new().detached();
            let r = ControlModeOutput::send(stdin, cmd, lines);
            //dbg!("result: ", r);
        }
        #[cfg(feature = "tmux_3_3")]
        Response::UnlinkedWindowClose(_) => {
            let cmd = NewWindow::new().detached();
            let r = ControlModeOutput::send(stdin, cmd, lines);
            //dbg!("result: ", r);
        }
        _ => {}
    }

    true
}

#[test]
fn monitor_test() {
    use crate::{AttachSession, StdIO, Tmux};

    let tmux = Tmux::with_command(AttachSession::new().target_session("0"))
        .control_mode()
        .stdout(Some(StdIO::Piped))
        .stdin(Some(StdIO::Piped))
        .spawn()
        .unwrap();

    ControlModeCtl::monitor(tmux, &tmux_control_proc);
}
