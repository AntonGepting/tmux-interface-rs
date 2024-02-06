// parse control mode output text in `control_mode::Response` structure tests
#[test]
fn control_mode_line() {
    use crate::control_mode::control_mode::{ControlModeLine, Response};

    // %begin seconds-from-epoch command-number flags
    #[cfg(feature = "tmux_1_8")]
    {
        let output = "%begin 1618054308 777 1".control_mode_line().unwrap();
        assert_eq!(
            Response::OutputBlockBegin {
                time: 1618054308,
                num: 777,
                flags: 1
            },
            output
        );
    }

    // %end seconds-from-epoch command-number flags
    #[cfg(feature = "tmux_1_8")]
    {
        let output = "%end 1618054308 777 1".control_mode_line().unwrap();
        assert_eq!(
            Response::OutputBlockEnd {
                time: 1618054308,
                num: 777,
                flags: 1
            },
            output
        );
    }

    // %error seconds-from-epoch command-number flags
    #[cfg(feature = "tmux_1_8")]
    {
        let output = "%error 1618054308 777 1".control_mode_line().unwrap();
        assert_eq!(
            Response::OutputBlockError {
                time: 1618054308,
                num: 777,
                flags: 1
            },
            output
        );
    }

    // ... data ...
    #[cfg(feature = "tmux_1_8")]
    {
        let output = "1".control_mode_line().unwrap();
        assert_eq!(Response::OutputBlockData("1".to_string()), output);
    }

    // %client-detached client
    #[cfg(feature = "tmux_3_2")]
    {
        let output = "%client-detached 1".control_mode_line().unwrap();
        assert_eq!(Response::ClientDetached("1".to_string()), output);
    }

    // %client-session-changed client session-id name
    #[cfg(feature = "tmux_2_4")]
    {
        let output = "%client-session-changed 1 2 3".control_mode_line().unwrap();
        assert_eq!(
            Response::ClientSessionChanged {
                client: "1".to_string(),
                session_id: "2".to_string(),
                name: "3".to_string()
            },
            output
        );
    }

    // %continue pane-id
    #[cfg(feature = "tmux_X_X")]
    {
        let output = "%continue 1".control_mode_line().unwrap();
        assert_eq!(Response::Continue("1".to_string()), output);
    }

    // %exit [reason]
    #[cfg(feature = "tmux_1_8")]
    {
        let output = "%exit 1".control_mode_line().unwrap();
        assert_eq!(Response::Exit(Some("1".to_string())), output);

        let output = "%exit".control_mode_line().unwrap();
        assert_eq!(Response::Exit(None), output);
    }

    // `%extended-output pane-id age ... : value`
    #[cfg(feature = "tmux_X_X")]
    {
        let output = "%extended-output 1 2 3 : 4".control_mode_line().unwrap();
        assert_eq!(
            Response::ExtendedOutput {
                pane_id: "1".to_string(),
                age: "2".to_string(),
                other: vec!["3".to_string()],
                value: "4".to_string()
            },
            output
        );
    }

    // tmux ^3.3 `%layout-change window-id window-layout window-visible-layout window-flags`
    // tmux ^2.2 `%layout-change window-id window-layout window-visible-layout`
    // tmux ^1.8 `%layout-change window-id window-layout`
    #[cfg(feature = "tmux_1_8")]
    {
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_8")))]
        let output = "%layout-change 1 2".control_mode_line().unwrap();
        #[cfg(all(feature = "tmux_2_8", not(feature = "tmux_3_3")))]
        let output = "%layout-change 1 2 3".control_mode_line().unwrap();
        #[cfg(feature = "tmux_3_3")]
        let output = "%layout-change 1 2 3 4".control_mode_line().unwrap();
        assert_eq!(
            Response::LayoutChange {
                window_id: "1".to_string(),
                window_layout: "2".to_string(),
                #[cfg(feature = "tmux_2_2")]
                window_visible_layout: "3".to_string(),
                #[cfg(feature = "tmux_3_3")]
                window_flags: "4".to_string()
            },
            output
        );
    }

    // %output pane-id value
    // XXX: check spaces, only first is separator
    // %output %2 \015\012test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s\015\012\015\012
    #[cfg(feature = "tmux_1_8")]
    {
        let output = "%output 1 2 3".control_mode_line().unwrap();
        assert_eq!(
            Response::Output {
                pane_id: "1".to_string(),
                value: "2 3".to_string()
            },
            output
        );
    }

    // %pane-mode-changed pane-id
    #[cfg(feature = "tmux_2_5")]
    {
        let output = "%pane-mode-changed 1".control_mode_line().unwrap();
        assert_eq!(Response::PaneModeChanged("1".to_string()), output);
    }

    // %pause pane-id
    #[cfg(feature = "tmux_X_X")]
    {
        let output = "%pause 1".control_mode_line().unwrap();
        assert_eq!(Response::Pause("1".to_string()), output);
    }

    // %session-changed session-id name
    #[cfg(feature = "tmux_1_8")]
    {
        let output = "%session-changed $1 2".control_mode_line().unwrap();
        assert_eq!(
            Response::SessionChanged {
                session_id: "1".to_string(),
                name: "2".to_string()
            },
            output
        );
    }

    // %session-renamed name
    #[cfg(feature = "tmux_1_8")]
    {
        let output = "%session-renamed 1 2".control_mode_line().unwrap();
        assert_eq!(Response::SessionRenamed("1 2".to_string()), output);
    }

    // %session-window-changed session-id window-id
    #[cfg(feature = "tmux_2_5")]
    {
        let output = "%session-window-changed 1 2".control_mode_line().unwrap();
        assert_eq!(
            Response::SessionWindowChanged {
                session_id: "1".to_string(),
                window_id: "2".to_string()
            },
            output
        );
    }

    // %sessions-changed
    #[cfg(feature = "tmux_1_8")]
    {
        let output = "%sessions-changed".control_mode_line().unwrap();
        assert_eq!(Response::SessionsChanged, output);
    }

    // %subscription-changed name session-id window-id window-index
    #[cfg(feature = "tmux_X_X")]
    {
        let output = "%subscription-changed 1 2 3 4".control_mode_line().unwrap();
        assert_eq!(
            Response::SubscriptionChanged(
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string()
            ),
            output
        );
    }

    // %unlinked-window-add window-id
    #[cfg(feature = "tmux_1_8")]
    {
        let output = "%unlinked-window-add 1".control_mode_line().unwrap();
        assert_eq!(Response::UnlinkedWindowAdd("1".to_string()), output);
    }

    // %window-add window-id
    #[cfg(feature = "tmux_1_8")]
    {
        let output = "%window-add 1".control_mode_line().unwrap();
        assert_eq!(Response::WindowAdd("1".to_string()), output);
    }

    // %window-close window-id
    #[cfg(feature = "tmux_1_8")]
    {
        let output = "%window-close 1".control_mode_line().unwrap();
        assert_eq!(Response::WindowClose("1".to_string()), output);
    }

    // %window-pane-changed window-id pane-id
    #[cfg(feature = "tmux_2_5")]
    {
        let output = "%window-pane-changed 1 2".control_mode_line().unwrap();
        assert_eq!(
            Response::WindowPaneChanged {
                window_id: "1".to_string(),
                pane_id: "2".to_string()
            },
            output
        );
    }

    // %window-renamed window-id name
    #[cfg(feature = "tmux_1_8")]
    {
        let output = "%window-renamed 1 2 3".control_mode_line().unwrap();
        assert_eq!(
            Response::WindowRenamed {
                window_id: "1".to_string(),
                name: "2 3".to_string()
            },
            output
        );
    }
}

#[test]
fn next() {
    use crate::control_mode::control_mode::{ControlModeOutput, OutputBlock, Response};
    use std::io::{BufRead, BufReader};

    let s = "%begin 1618081916 17688 1\n0: 3 windows (created Sat Apr 10 13:01:08 2021) (attached)\n%end 1618081916 17688 1\n%session-changed 0 $0 0";
    let s = BufReader::new(s.as_bytes());
    let lines = s.lines();

    let mut cm_mode_lines = ControlModeOutput::new(lines);
    let cm_mode_line = cm_mode_lines.next().unwrap();
    let output_block = Response::OutputBlock(OutputBlock {
        time: 1618081916,
        num: 17688,
        flags: 1,
        success: true,
        data: Some("0: 3 windows (created Sat Apr 10 13:01:08 2021) (attached)".to_string()),
    });
    assert_eq!(cm_mode_line, output_block);

    let cm_mode_line = cm_mode_lines.next().unwrap();
    let output_block = Response::SessionChanged {
        session_id: "$0".to_string(),
        name: "0".to_string(),
    };
    assert_eq!(cm_mode_line, output_block);
}

// parse if multiple outputs as `control_mode::Response` and check case if
#[test]
fn for_loop() {
    use crate::control_mode::control_mode::{ControlModeOutput, Response};
    use std::io::{BufRead, BufReader};

    let s = "%begin 1618081916 17688 0\n%end 1618081916 17688 0\n%session-changed 0 $0 0";
    let s = BufReader::new(s.as_bytes());
    let lines = s.lines();

    let cm_mode_lines = ControlModeOutput::new(lines);
    for cm_mode_line in cm_mode_lines {
        match cm_mode_line {
            Response::OutputBlock(_) => {}
            Response::SessionChanged { session_id, name } => {}
            _ => {}
        }
    }
}

//#[test]
//fn main_like() {
//// tmux open in C-mode
//// send commands if needed using like socket
//// listen like socket for notificeations
//}
