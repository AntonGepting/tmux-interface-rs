#[test]
fn break_pane() {
    use crate::{BreakPane, Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
        // (alias: breakp)
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
    tmux.break_pane(Some(&break_pane)).unwrap_err();
}
