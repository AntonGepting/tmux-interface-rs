#[test]
fn show_environment() {
    use crate::{Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux show-environment [-gs] [-t target-session] [variable]
        // (alias: showenv)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["show-environment", "-g", "-s", "-t", "1", "2"]"#
        );
        Err(Error::Hook)
    }));
    tmux.show_environment(
        Some(true),
        Some(true),
        Some(&TargetSession::Raw("1")),
        Some("2"),
    )
    .unwrap_err();
}
