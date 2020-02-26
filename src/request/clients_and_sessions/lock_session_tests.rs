#[test]
fn lock_session() {
    use crate::{Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux lock-session [-t target-session]
        // (alias: locks)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["lock-session", "-t", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.lock_session(Some(&TargetSession::Raw("1")))
        .unwrap_err();
}
