#[test]
fn show_environment() {
    use crate::{Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^2.1:
        // ```text
        // tmux show-environment [-gs] [-t target-session] [variable]
        // (alias: showenv)
        // ```
        //
        // tmux ^1.7:
        // ```text
        // tmux show-environment [-g] [-t target-session] [variable]
        // (alias: showenv)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux show-environment [-g] [-t target-session]
        // (alias: showenv)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("show-environment");
        #[cfg(feature = "use_cmd_alias")]
        s.push("showenv");
        s.push("-g");
        s.push("-s");
        s.extend_from_slice(&["-t", "1"]);
        s.push("2");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_session = TargetSession::Raw("1").to_string();
    tmux.show_environment(Some(true), Some(true), Some(&target_session), Some("2"))
        .unwrap_err();
}
