#[test]
fn list_commands() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux list-commands [-F format]
        // (alias: lscm)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["list-commands", "-F", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.list_commands(Some("1")).unwrap_err();
}
