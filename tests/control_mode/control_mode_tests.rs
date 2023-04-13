// 1. start tmux in control mode
// 2. add listener (pipes: stdin, stdout)
// 3. add bufreader (not knowing EOL, EOF, therefore using buffer, splitting lines, reading line by
//    line?)
// 4. add controlmode line processor (bc multiple lines must be merged, depends on output or
//    notification is recieved, s. output block begin/error/end)
//    -> ControlModeBuf?
// 5. ControlModeBuf::recv()? send()?
// 5. ~add defaultproc handler (similar to GUI WinAPI WindowProc in windows)
// 6. add user defined notification handlers
//
//
// TODO: rename/create like manually and using interface fns
#[cfg(feature = "tmux_1_8")]
#[test]
fn control_mode() {
    use std::io::{BufRead, BufReader};
    use std::process::Stdio;
    use tmux_interface::control_mode::control_mode::{ControlModeOutput, Response};
    use tmux_interface::{AttachSession, NewWindow, Tmux};

    //let tmux = Command::new("tmux")
    //.args(&["-C", "attach", "-t", "3"])
    ////.args(&["-C", "new", "-t", "control_mode"])
    //.stdout(Stdio::piped())
    //.stdin(Stdio::piped())
    //.spawn()
    //.unwrap();

    let tmux = Tmux::new()
        .control_mode()
        .command(AttachSession::new().target_session("0"))
        .into_command()
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = tmux.stdout.unwrap();

    let mut stdin = tmux.stdin.unwrap();

    let reader = BufReader::new(stdout);

    let mut cm_lines = ControlModeOutput::new(reader.lines());

    //ControlModeOutput::event_loop(cm_lines, &mut |cm_line| match &cm_line {
    //Response::SessionWindowChanged(_, _) => {
    //let cmd = NewWindow::new().detached().build().to_string();
    //ControlModeOutput::send(&mut stdin, cmd, cm_lines).unwrap();
    //}
    //_ => {}
    //});

    // working example of check return of the sent command
    //
    while let Some(cm_line) = cm_lines.next() {
        dbg!(&cm_line);

        match &cm_line {
            #[cfg(feature = "tmux_2_5")]
            Response::SessionWindowChanged(_, _) => {
                let cmd = NewWindow::new().detached().build().to_string();
                ControlModeOutput::send(&mut stdin, cmd, &mut cm_lines).unwrap();
            }
            _ => {}
        }
    }

    // event loop
    //for cm_line in cm_lines {
    //// show all events
    //dbg!(&cm_line);
    ////let cm_line = cm_lines.next().unwrap();

    //match cm_line {
    //Response::SessionWindowChanged(_, _) => {
    //let cmd = NewWindow::new().detached().build().to_string();
    ////ControlModeOutput::send(&mut stdin, cmd, &mut cm_lines).unwrap();
    ////writeln!(stdin, "{}", cmd).unwrap();
    ////stdin.write_all(s.as_bytes()).unwrap();
    ////stdin.flush().unwrap();

    ////let r = cm_lines.next().unwrap();
    ////match r {
    ////Response::SessionChanged(a, b) => {
    ////dbg!(&a);
    ////}
    ////_ => {}
    ////}
    //}
    //_ => {}
    //}
    //}
}

// typical notifications on `tmux -C` -> `tmux -C ; new-session`
//
// 0. Empty successed `OutputBlock(...)`
//
// ```
// %begin 1234567890 1234 0
// %end   1234567890 1234 0
// ```
//
// 1. New window created by default `WindowAdd(...)`
// ```
// window-add: @1
// ```
//
// 2. New session created `SessionsChanged(...)`
// ```
// sessions-changed
// ```
//
// 3. Client attached to session `SessionChanged(...)`
// ```
// session-changed $1 1
// ```
//
// 4. Prompt `OutputBlock(...)`
// ```
// output %6 PROMPT
// ```
//#[test]
//fn control_mode2() {
//use std::io::{BufRead, BufReader};
//use tmux_interface::commands::tmux::Tmux;
//use tmux_interface::control_mode::control_mode::{ControlModeOutput, Response};
//use tmux_interface::{NewSession, TmuxCommand};

//let mut tmux = Tmux::new();
//tmux.control_mode();

////let child = tmux.spawn().unwrap();
//let stdout = child.stdout.unwrap();
//let mut stdin = child.stdin.unwrap();
//let reader = BufReader::new(stdout);

//let mut outputs = ControlModeOutput::new(reader.lines());
//let response = outputs.next().unwrap();
//if let Response::OutputBlock(data) = response {
//if data.success {
//println!("tmux -C successed");
//}
//}

//let mut new_session = NewSession::new().build();
////new_session.session_name("qq");

//loop {
//if let Some(output) = outputs.next() {
//dbg!(&output);
//} else {
//break;
//}
//}
//}
