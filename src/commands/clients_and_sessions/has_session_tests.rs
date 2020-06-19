#[test]
fn has_session() {
    use crate::{Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux has-session [-t target-session]
        // (alias: has)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["has-session", "-t", "1"]"#
        );
        Err(Error::Hook)
    }));
    let target_session = &TargetSession::Raw("1").to_string();
    tmux.has_session(Some(&target_session)).unwrap_err();
}
