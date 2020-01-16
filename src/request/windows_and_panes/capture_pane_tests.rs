#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn capture_pane() {
    use crate::{CapturePane, Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux capture-pane [-aepPqCJN] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
        // (alias: capturep)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["capture-pane", "-a", "-e", "-p", "-P", "-q", "-C", "-J", "-N", "-b", "1", "-E", "2", "-S", "3", "-t", "4"]"#
        );
        Err(Error::new("hook"))
    }));
    let capture_pane = CapturePane {
        alternate_screen: Some(true),
        escape_sequences: Some(true),
        stdout: Some(true),
        pane: Some(true),
        quite: Some(true),
        escape_non_printable: Some(true),
        join: Some(true),
        trailing_spaces: Some(true),
        buffer_name: Some("1"),
        end_line: Some("2"),
        start_line: Some("3"),
        target_pane: Some("4"),
    };
    tmux.capture_pane(Some(&capture_pane)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn capture_pane() {
    use crate::{CapturePane, Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux capture-pane [-aepPqCJ] [-b buffer-name] [-E end-line] [-S start-line] [-t target-pane]
        // (alias: capturep)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["capture-pane", "-a", "-e", "-p", "-P", "-q", "-C", "-J", "-b", "1", "-E", "2", "-S", "3", "-t", "4"]"#
        );
        Err(Error::new("hook"))
    }));
    let capture_pane = CapturePane {
        alternate_screen: Some(true),
        escape_sequences: Some(true),
        stdout: Some(true),
        pane: Some(true),
        quite: Some(true),
        escape_non_printable: Some(true),
        join: Some(true),
        buffer_name: Some("1"),
        end_line: Some("2"),
        start_line: Some("3"),
        target_pane: Some("4"),
    };
    tmux.capture_pane(Some(&capture_pane)).unwrap_err();
}
