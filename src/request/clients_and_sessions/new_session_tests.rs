#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn new_session() {
    use crate::{Error, NewSession, NewSessionBuilder, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux new-session [-AdDEPX] [-c start-directory] [-F format] [-n window-name]
        // [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
        // (alias: new)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["new-session", "-A", "-d", "-D", "-E", "-P", "-X", "-c", "1", "-F", "2", "-n", "3", "-s", "4", "-t", "5", "-x", "6", "-y", "7", "8"]"#
        );
        Err(Error::new("hook"))
    }));
    let new_session = NewSession {
        attach: Some(true),
        detached: Some(true),
        detach_other: Some(true),
        not_update_env: Some(true),
        print: Some(true),
        parent_sighup: Some(true),
        cwd: Some("1"),
        format: Some("2"),
        window_name: Some("3"),
        session_name: Some("4"),
        group_name: Some("5"),
        width: Some(6),
        height: Some(7),
        shell_command: Some("8"),
    };
    tmux.new_session(Some(&new_session)).unwrap_err();

    let new_session = NewSessionBuilder::new()
        .attach()
        .detached()
        .detach_other()
        .not_update_env()
        .print()
        .parent_sighup()
        .cwd("1")
        .format("2")
        .window_name("3")
        .session_name("4")
        .group_name("5")
        .width(6)
        .height(7)
        .shell_command("8")
        .build();
    tmux.new_session(Some(&new_session)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn new_session() {
    use crate::{Error, NewSession, NewSessionBuilder, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name]
        // [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
        // (alias: new)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["new-session", "-A", "-d", "-D", "-E", "-P", "-c", "1", "-F", "2", "-n", "3", "-s", "4", "-t", "5", "-x", "6", "-y", "7", "8"]"#
        );
        Err(Error::new("hook"))
    }));

    let new_session = NewSession {
        attach: Some(true),
        detached: Some(true),
        detach_other: Some(true),
        not_update_env: Some(true),
        print: Some(true),
        cwd: Some("1"),
        format: Some("2"),
        window_name: Some("3"),
        session_name: Some("4"),
        group_name: Some("5"),
        width: Some(6),
        height: Some(7),
        shell_command: Some("8"),
    };
    tmux.new_session(Some(&new_session)).unwrap_err();

    let new_session = NewSessionBuilder::new()
        .attach()
        .detached()
        .detach_other()
        .not_update_env()
        .print()
        .cwd("1")
        .format("2")
        .window_name("3")
        .session_name("4")
        .group_name("5")
        .width(6)
        .height(7)
        .shell_command("8")
        .build();
    tmux.new_session(Some(&new_session)).unwrap_err();
}
