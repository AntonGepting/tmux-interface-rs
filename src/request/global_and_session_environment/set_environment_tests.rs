#[test]
fn set_environment() {
    use crate::{Error, SetEnvironment, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux set-environment [-gru] [-t target-session] name [value]
        // (alias: setenv)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["set-environment", "-g", "-r", "-u", "-t", "1", "2", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    let set_environment = SetEnvironment {
        global: Some(true),
        remove: Some(true),
        unset: Some(true),
        target_session: Some("1"),
        value: Some("3"),
    };
    tmux.set_environment(Some(&set_environment), "2")
        .unwrap_err();
}
