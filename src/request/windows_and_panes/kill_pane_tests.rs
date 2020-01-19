#[test]
fn kill_pane() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux kill-pane [-a] [-t target-pane]
        // (alias: killp)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["kill-pane", "-a", "-t", "1"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.kill_pane(Some(true), Some("1")).unwrap_err();
}
