#[test]
fn set_environment() {
    use crate::{Error, SetEnvironment, SetEnvironmentBuilder, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^1.0:
        // ```text
        // tmux set-environment [-gru] [-t target-session] name [value]
        // (alias: setenv)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("set-environment");
        #[cfg(feature = "use_cmd_alias")]
        s.push("setenv");
        #[cfg(feature = "tmux_1_0")]
        s.push("-g");
        #[cfg(feature = "tmux_1_0")]
        s.push("-r");
        #[cfg(feature = "tmux_1_0")]
        s.push("-u");
        #[cfg(feature = "tmux_1_0")]
        s.extend_from_slice(&["-t", "1"]);
        #[cfg(feature = "tmux_1_0")]
        s.push("2");
        #[cfg(feature = "tmux_1_0")]
        s.push("3");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_session = TargetSession::Raw("1").to_string();
    let set_environment = SetEnvironment {
        #[cfg(feature = "tmux_1_0")]
        global: Some(true),
        #[cfg(feature = "tmux_1_0")]
        remove: Some(true),
        #[cfg(feature = "tmux_1_0")]
        unset: Some(true),
        #[cfg(feature = "tmux_1_0")]
        target_session: Some(&target_session),
        #[cfg(feature = "tmux_1_0")]
        value: Some("3"),
    };
    tmux.set_environment(Some(&set_environment), "2")
        .unwrap_err();

    let mut builder = SetEnvironmentBuilder::new();
    #[cfg(feature = "tmux_1_0")]
    builder.global();
    #[cfg(feature = "tmux_1_0")]
    builder.remove();
    #[cfg(feature = "tmux_1_0")]
    builder.unset();
    #[cfg(feature = "tmux_1_0")]
    builder.target_session(&target_session);
    #[cfg(feature = "tmux_1_0")]
    builder.value("3");
    let set_environment = builder.build();
    tmux.set_environment(Some(&set_environment), "2")
        .unwrap_err();
}
