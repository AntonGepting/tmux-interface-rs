#[test]
fn set_environment() {
    use crate::{Error, SetEnvironment, SetEnvironmentBuilder, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux set-environment [-gru] [-t target-session] name [value]
        // (alias: setenv)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["set-environment", "-g", "-r", "-u", "-t", "1", "2", "3"]"#
        );
        Err(Error::Hook)
    }));
    let set_environment = SetEnvironment {
        global: Some(true),
        remove: Some(true),
        unset: Some(true),
        target_session: Some(&TargetSession::Raw("1")),
        value: Some("3"),
    };
    tmux.set_environment(Some(&set_environment), "2")
        .unwrap_err();

    let set_environment = SetEnvironmentBuilder::new()
        .global()
        .remove()
        .unset()
        .target_session(&TargetSession::Raw("1"))
        .value("3")
        .build();
    tmux.set_environment(Some(&set_environment), "2")
        .unwrap_err();
}
