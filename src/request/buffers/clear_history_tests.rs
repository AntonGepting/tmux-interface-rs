#[test]
fn clear_history() {
    use crate::{Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux clear-history [-t target-pane]
        // (alias: clearhist)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["clear-history", "-t", "1"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.clear_history(Some(&TargetSession::Raw("1")))
        .unwrap_err();
}
