#[test]
fn list_sessions() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux list-sessions [-F format]
        // (alias: ls)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["list-sessions", "-F", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.list_sessions(Some("1")).unwrap_err();
}
