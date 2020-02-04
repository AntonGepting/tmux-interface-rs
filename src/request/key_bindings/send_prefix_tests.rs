#[test]
fn send_prefix() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux send-prefix [-2] [-t target-pane]
        // (alias: unbind)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["send-prefix", "-2", "-t", "1"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.send_prefix(Some(true), Some("1")).unwrap_err();
}