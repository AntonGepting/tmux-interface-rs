#[test]
fn rename_session() {
    use crate::{Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux rename-session [-t target-session] new-name
        // (alias: rename)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["rename-session", "-t", "1", "2"]"#
        );
        Err(Error::Hook)
    }));
    tmux.rename_session(Some(&TargetSession::Raw("1")), "2")
        .unwrap_err();
}
