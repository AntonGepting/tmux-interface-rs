#[test]
fn copy_mode() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux copy-mode [-Meu] [-t target-pane]
        assert_eq!(
            format!("{:?} {:?} {:?}", bin, options, subcmd),
            r#""tmux" [] ["copy-mode", "-M", "-e", "-u", "-t", "1"]"#
        );
        Err(Error::new("hook"))
    }));
    let _ = tmux.copy_mode(Some(true), Some(true), Some(true), Some("1"));
}

#[test]
fn break_pane() {
    use crate::{BreakPane, Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        //println!("prehook: {:?} {:?} {:?}", bin, options, subcmd);
        // tmux break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["break-pane", "-d", "-P", "-F", "1", "-n", "2", "-s", "3", "-t", "4"]"#
        );
        Err(Error::new("hook"))
    }));
    let break_pane = BreakPane {
        detached: Some(true),
        print: Some(true),
        format: Some("1"),
        window_name: Some("2"),
        src_pane: Some("3"),
        dst_window: Some("4"),
    };
    let _ = tmux.break_pane(Some(&break_pane));
}
