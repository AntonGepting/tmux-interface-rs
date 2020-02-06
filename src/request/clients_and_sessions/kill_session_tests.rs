#[test]
fn kill_session() {
    use crate::{Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux kill-session [-aC] [-t target-session]
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["kill-session", "-a", "-C", "-t", "1"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.kill_session(Some(true), Some(true), Some(&TargetSession::Raw("1")))
        .unwrap_err();
}
