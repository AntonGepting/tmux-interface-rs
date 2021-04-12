#[test]
fn control_mode_output() {
    use crate::control_mode::control_mode::{ControlModeOutput, Response};

    // %begin 1618054308 777 0
    let output = ControlModeOutput::check("%begin 1618054308 777").unwrap();
    assert_eq!(Response::OutputBlockBegin(1618054308, 777), output);

    // %end 1618054308 777 0
    let output = ControlModeOutput::check("%end 1618054308 777").unwrap();
    assert_eq!(Response::OutputBlockEnd(1618054308, 777), output);

    let output = ControlModeOutput::check("%error 1618054308 777").unwrap();
    assert_eq!(Response::OutputBlockError(1618054308, 777), output);

    let output = ControlModeOutput::check("1").unwrap();
    assert_eq!(Response::OutputBlockData("1".to_string()), output);

    let output = ControlModeOutput::check("%client-detached 1").unwrap();
    assert_eq!(Response::ClientDetached("1".to_string()), output);

    let output = ControlModeOutput::check("%client-session-changed 1 2 3").unwrap();
    assert_eq!(
        Response::ClientSessionChanged("1".to_string(), "2".to_string(), "3".to_string()),
        output
    );

    let output = ControlModeOutput::check("%continue 1").unwrap();
    assert_eq!(Response::Continue("1".to_string()), output);

    let output = ControlModeOutput::check("%exit 1").unwrap();
    assert_eq!(Response::Exit("1".to_string()), output);

    //let output = ControlMode::check("%extended-output 1").unwrap();
    //assert_eq!(Response::ExtendedOutput("1"), output);

    let output = ControlModeOutput::check("%layout-change 1 2 3").unwrap();
    assert_eq!(
        Response::LayoutChange("1".to_string(), "2".to_string(), "3".to_string()),
        output
    );

    // XXX: check spaces, only first is separator
    // %output %2 \015\012test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s\015\012\015\012
    let output = ControlModeOutput::check("%output 1 2").unwrap();
    assert_eq!(Response::Output("1".to_string(), "2".to_string()), output);

    let output = ControlModeOutput::check("%pane-mode-changed 1").unwrap();
    assert_eq!(Response::PaneModeChanged("1".to_string()), output);

    let output = ControlModeOutput::check("%pause 1").unwrap();
    assert_eq!(Response::Pause("1".to_string()), output);

    // %session-changed $0 0
    let output = ControlModeOutput::check("%session-changed $1 2").unwrap();
    assert_eq!(
        Response::SessionChanged("$1".to_string(), "2".to_string()),
        output
    );

    let output = ControlModeOutput::check("%session-renamed 1").unwrap();
    assert_eq!(Response::SessionRenamed("1".to_string()), output);

    let output = ControlModeOutput::check("%session-window-changed 1 2").unwrap();
    assert_eq!(
        Response::SessionWindowChanged("1".to_string(), "2".to_string()),
        output
    );

    let output = ControlModeOutput::check("%sessions-changed").unwrap();
    assert_eq!(Response::SessionsChanged, output);

    let output = ControlModeOutput::check("%subscription-changed 1 2 3 4").unwrap();
    assert_eq!(
        Response::SubscriptionChanged(
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string()
        ),
        output
    );

    let output = ControlModeOutput::check("%unlinked-window-add 1").unwrap();
    assert_eq!(Response::UnlinkedWindowAdd("1".to_string()), output);

    let output = ControlModeOutput::check("%window-add 1").unwrap();
    assert_eq!(Response::WindowAdd("1".to_string()), output);

    let output = ControlModeOutput::check("%window-close 1").unwrap();
    assert_eq!(Response::WindowClose("1".to_string()), output);

    let output = ControlModeOutput::check("%window-pane-changed 1 2").unwrap();
    assert_eq!(
        Response::WindowPaneChanged("1".to_string(), "2".to_string()),
        output
    );

    let output = ControlModeOutput::check("%window-renamed 1 2").unwrap();
    assert_eq!(
        Response::WindowRenamed("1".to_string(), "2".to_string()),
        output
    );
}

#[test]
fn next() {
    use crate::control_mode::control_mode::ControlModeOutput;

    let s = "%begin 1618081916 17688 1\n0: 3 windows (created Sat Apr 10 13:01:08 2021) (attached)\n%end 1618081916 17688 1\n%session-changed $0 0";

    let mut control_mode = ControlModeOutput::new(s.lines());
    let output = control_mode.next().unwrap();
    dbg!(output);
    let output = control_mode.next().unwrap();
    dbg!(output);
}

#[test]
fn for_loop() {
    use crate::control_mode::control_mode::ControlModeOutput;

    //let s = "%begin 1618060545 6300 0\n%end 1618060546 6300 0\n%session-changed $0 0";
    let s = "%begin 1618081916 17688 1\n0: 3 windows (created Sat Apr 10 13:01:08 2021) (attached)\n%end 1618081916 17688 1\n%session-changed $0 0";

    let mut control_mode = ControlModeOutput::new(s.lines());
    for output in control_mode {
        dbg!(output);
    }
}

#[test]
fn main_like() {
    // tmux open in C-mode
    // send commands if needed using like socket
    // listen like socket for notificeations
}
