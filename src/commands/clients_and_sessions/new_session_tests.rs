#[test]
fn new_session() {
    #[cfg(feature = "tmux_3_2")]
    use crate::ClientFlags;
    use crate::NewSession;
    use std::borrow::Cow;

    // Structure for creating a new session
    //
    // # Manual
    //
    // tmux 3.2:
    // ```text
    // tmux new-session [-AdDEPX] [-c start-directory] [-e environment] [-f flags] [-F format]
    // [-n window-name] [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux 3.0:
    // ```text
    // tmux new-session [-AdDEPX] [-c start-directory] [-F format] [-n window-name] [-s session-name]
    // [-t group-name] [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux 2.4:
    // ```text
    // tmux new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
    // [-t group-name] [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux 2.1:
    // ```text
    // tmux new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
    // [-t target-session] [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux 1.9:
    // ```text
    // tmux new-session [-AdDP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
    // [-t target-session] [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux 1.8:
    // ```text
    // tmux new-session [-AdDP] [-F format] [-n window-name] [-s session-name] [-t target-session]
    // [-x width] [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux 1.6:
    // ```text
    // tmux new-session [-d] [-n window-name] [-s session-name] [-t target-session] [-x width]
    // [-y height] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux 1.2:
    // ```text
    // tmux new-session [-d] [-n window-name] [-s session-name] [-t target-session] [shell-command]
    // (alias: new)
    // ```
    //
    // tmux 1.1:
    // ```text
    // tmux new-session [-d] [-n window-name] [-s session-name] [-t target-session] [command]
    // (alias: new)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux new-session [-d] [-n window-name] [-s session-name] [command]
    // (alias: new)
    // ```
    let new_session = NewSession::new();
    #[cfg(feature = "tmux_1_8")]
    let new_session = new_session.attach();
    #[cfg(feature = "tmux_0_8")]
    let new_session = new_session.detached();
    #[cfg(feature = "tmux_1_8")]
    let new_session = new_session.detach_other();
    #[cfg(feature = "tmux_2_1")]
    let new_session = new_session.not_update_env();
    #[cfg(feature = "tmux_1_8")]
    let new_session = new_session.print();
    #[cfg(feature = "tmux_3_0")]
    let new_session = new_session.parent_sighup();
    #[cfg(feature = "tmux_1_9")]
    let new_session = new_session.start_directory("1");
    #[cfg(feature = "tmux_3_2")]
    let new_session = new_session.environment("2", "3");
    #[cfg(feature = "tmux_3_2")]
    let flags = ClientFlags {
        active_pane: Some(true),
        ..Default::default()
    };
    #[cfg(feature = "tmux_3_2")]
    let new_session = new_session.flags(flags);
    #[cfg(feature = "tmux_1_8")]
    let new_session = new_session.format("4");
    #[cfg(feature = "tmux_0_8")]
    let new_session = new_session.window_name("5");
    #[cfg(feature = "tmux_0_8")]
    let new_session = new_session.session_name("6");
    #[cfg(feature = "tmux_2_4")]
    let new_session = new_session.group_name("7");
    #[cfg(feature = "tmux_1_6")]
    let new_session = new_session.width(8);
    #[cfg(feature = "tmux_1_6")]
    let new_session = new_session.height(9);
    #[cfg(feature = "tmux_1_2")]
    let new_session = new_session.shell_command("10");

    //let new = new_session.to_tmux_bin_command();

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "new-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "new";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_8")]
    s.push("-A");
    #[cfg(feature = "tmux_0_8")]
    s.push("-d");
    #[cfg(feature = "tmux_1_8")]
    s.push("-D");
    #[cfg(feature = "tmux_2_1")]
    s.push("-E");
    #[cfg(feature = "tmux_1_8")]
    s.push("-P");
    #[cfg(feature = "tmux_3_0")]
    s.push("-X");
    #[cfg(feature = "tmux_1_9")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-e", "2=3"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-f", "active-pane"]);
    #[cfg(feature = "tmux_1_8")]
    s.extend_from_slice(&["-F", "4"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-n", "5"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-s", "6"]);
    #[cfg(feature = "tmux_2_4")]
    s.extend_from_slice(&["-t", "7"]);
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-x", "8"]);
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-y", "9"]);
    #[cfg(feature = "tmux_1_2")]
    s.push("10");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let new_session = new_session.build().to_vec();

    assert_eq!(new_session, s);
}

#[test]
fn new_session_inner() {
    //use crate::NewSession;

    //let a = NewSession::new().session_name("asdf");
    //dbg!(a.as_ref().cmd);
}
