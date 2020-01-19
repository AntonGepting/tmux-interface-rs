#[test]
fn move_window() {
    use crate::{Error, MoveWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux move-window [-ardk] [-s src-window] [-t dst-window]
        // (alias: movew)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["move-window", "-a", "-r", "-d", "-k", "-s", "1", "-t", "2"]"#
        );
        Err(Error::new("hook"))
    }));
    let move_window = MoveWindow {
        add: Some(true),
        renumber: Some(true),
        detached: Some(true),
        kill: Some(true),
        src_window: Some("1"),
        dst_window: Some("2"),
    };
    tmux.move_window(Some(&move_window)).unwrap_err();
}
