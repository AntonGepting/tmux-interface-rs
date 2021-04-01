#[test]
fn new_session() {
    use crate::NewSession;
    use std::borrow::Cow;

    // Structure for creating a new session
    //
    // # Manual
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
    let mut new_session = NewSession::new();
    #[cfg(feature = "tmux_1_8")]
    new_session.attach();
    #[cfg(feature = "tmux_0_8")]
    new_session.detached();
    #[cfg(feature = "tmux_1_8")]
    new_session.detach_other();
    #[cfg(feature = "tmux_2_1")]
    new_session.not_update_env();
    #[cfg(feature = "tmux_1_8")]
    new_session.print();
    #[cfg(feature = "tmux_3_0")]
    new_session.parent_sighup();
    #[cfg(feature = "tmux_1_9")]
    new_session.start_directory("1");
    #[cfg(feature = "tmux_1_8")]
    new_session.format("2");
    #[cfg(feature = "tmux_0_8")]
    new_session.window_name("3");
    #[cfg(feature = "tmux_0_8")]
    new_session.session_name("4");
    #[cfg(feature = "tmux_2_4")]
    new_session.group_name("5");
    #[cfg(feature = "tmux_1_6")]
    new_session.width(6);
    #[cfg(feature = "tmux_1_6")]
    new_session.height(7);
    #[cfg(feature = "tmux_1_2")]
    new_session.shell_command("8");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "new-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "new";

    let mut s = Vec::new();
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
    #[cfg(feature = "tmux_1_8")]
    s.extend_from_slice(&["-F", "2"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-n", "3"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-s", "4"]);
    #[cfg(feature = "tmux_2_4")]
    s.extend_from_slice(&["-t", "5"]);
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-x", "6"]);
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-y", "7"]);
    #[cfg(feature = "tmux_1_2")]
    s.push("8");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(new_session.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(new_session.0.bin_args, None);
    assert_eq!(new_session.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(new_session.0.cmd_args, Some(s));
}
