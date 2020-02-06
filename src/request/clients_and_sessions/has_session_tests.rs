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
        Err(Error::new("hook"))
    }));
    tmux.has_session(Some(&TargetSession::Raw("1")))
        .unwrap_err();
}
