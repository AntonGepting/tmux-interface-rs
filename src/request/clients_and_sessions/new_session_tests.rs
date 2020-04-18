#[test]
fn new_session() {
    use crate::{Error, NewSession, NewSessionBuilder, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
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
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("new-session");
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
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let new_session = NewSession {
        #[cfg(feature = "tmux_1_8")]
        attach: Some(true),
        #[cfg(feature = "tmux_0_8")]
        detached: Some(true),
        #[cfg(feature = "tmux_1_8")]
        detach_other: Some(true),
        #[cfg(feature = "tmux_2_1")]
        not_update_env: Some(true),
        #[cfg(feature = "tmux_1_8")]
        print: Some(true),
        #[cfg(feature = "tmux_3_0")]
        parent_sighup: Some(true),
        #[cfg(feature = "tmux_1_9")]
        cwd: Some("1"),
        #[cfg(feature = "tmux_1_8")]
        format: Some("2"),
        #[cfg(feature = "tmux_0_8")]
        window_name: Some("3"),
        #[cfg(feature = "tmux_0_8")]
        session_name: Some("4"),
        #[cfg(feature = "tmux_2_4")]
        group_name: Some("5"),
        #[cfg(feature = "tmux_1_6")]
        width: Some(6),
        #[cfg(feature = "tmux_1_6")]
        height: Some(7),
        #[cfg(feature = "tmux_1_2")]
        shell_command: Some("8"),
    };
    tmux.new_session(Some(&new_session)).unwrap_err();

    let mut builder = NewSessionBuilder::new();
    #[cfg(feature = "tmux_1_8")]
    builder.attach();
    #[cfg(feature = "tmux_0_8")]
    builder.detached();
    #[cfg(feature = "tmux_1_8")]
    builder.detach_other();
    #[cfg(feature = "tmux_2_1")]
    builder.not_update_env();
    #[cfg(feature = "tmux_1_8")]
    builder.print();
    #[cfg(feature = "tmux_3_0")]
    builder.parent_sighup();
    #[cfg(feature = "tmux_1_9")]
    builder.cwd("1");
    #[cfg(feature = "tmux_1_8")]
    builder.format("2");
    #[cfg(feature = "tmux_0_8")]
    builder.window_name("3");
    #[cfg(feature = "tmux_0_8")]
    builder.session_name("4");
    #[cfg(feature = "tmux_2_4")]
    builder.group_name("5");
    #[cfg(feature = "tmux_1_6")]
    builder.width(6);
    #[cfg(feature = "tmux_1_6")]
    builder.height(7);
    #[cfg(feature = "tmux_1_2")]
    builder.shell_command("8");
    let new_session = builder.build();
    tmux.new_session(Some(&new_session)).unwrap_err();
}
