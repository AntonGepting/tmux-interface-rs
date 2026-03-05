// auto-generated file
//

// Create new session
//
// # Manual
//
// tmux >=3.2:
// ```text
// new-session [-AdDEPX] [-c start-directory] [-e environment] [-f flags] [-F format]
// [-n window-name] [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
// (alias: new)
// ```
//
// tmux >=3.0:
// ```text
// new-session [-AdDEPX] [-c start-directory] [-F format] [-n window-name] [-s session-name]
// [-t group-name] [-x width] [-y height] [shell-command]
// (alias: new)
// ```
//
// tmux >=2.4:
// ```text
// new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
// [-t group-name] [-x width] [-y height] [shell-command]
// (alias: new)
// ```
//
// tmux >=2.1:
// ```text
// new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
// [-t target-session] [-x width] [-y height] [shell-command]
// (alias: new)
// ```
//
// tmux >=1.9:
// ```text
// new-session [-AdDP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
// [-t target-session] [-x width] [-y height] [shell-command]
// (alias: new)
// ```
//
// tmux >=1.8:
// ```text
// new-session [-AdDP] [-F format] [-n window-name] [-s session-name] [-t target-session]
// [-x width] [-y height] [shell-command]
// (alias: new)
// ```
//
// tmux >=1.6:
// ```text
// new-session [-d] [-n window-name] [-s session-name] [-t target-session] [-x width]
// [-y height] [shell-command]
// (alias: new)
// ```
//
// tmux >=1.2:
// ```text
// new-session [-d] [-n window-name] [-s session-name] [-t target-session] [shell-command]
// (alias: new)
// ```
//
// tmux >=1.1:
// ```text
// new-session [-d] [-n window-name] [-s session-name] [-t target-session] [command]
// (alias: new)
// ```
//
// tmux >=0.8:
// ```text
// new-session [-d] [-n window-name] [-s session-name] [command]
// (alias: new)
// ```
#[test]
fn new_session() {
    #[cfg(feature = "tmux_3_2")]
    use crate::ClientFlags;
    use crate::NewSession;
    use std::borrow::Cow;

    let new_session = NewSession::new();
    // `[-A]`
    #[cfg(feature = "tmux_1_8")]
    let new_session = new_session.attach();

    // `[-d]`
    #[cfg(feature = "tmux_0_8")]
    let new_session = new_session.detached();

    // `[-D]`
    #[cfg(feature = "tmux_1_8")]
    let new_session = new_session.detach_other();

    // `[-E]`
    #[cfg(feature = "tmux_2_1")]
    let new_session = new_session.not_update_env();

    // `[-P]`
    #[cfg(feature = "tmux_1_8")]
    let new_session = new_session.print();

    // `[-X]`
    #[cfg(feature = "tmux_3_0a")]
    let new_session = new_session.parent_sighup();

    // `[-c start-directory]`
    #[cfg(feature = "tmux_1_9")]
    let new_session = new_session.start_directory("1");

    // `[-e environment]`
    #[cfg(feature = "tmux_3_2")]
    let new_session = new_session.environment("2", "3");

    // `[-f flags]`
    #[cfg(feature = "tmux_3_2")]
    let flags = ClientFlags {
        active_pane: Some(true),
        ..Default::default()
    };
    #[cfg(feature = "tmux_3_2")]
    let new_session = new_session.flags(flags);

    // `[-F format]`
    #[cfg(feature = "tmux_1_8")]
    let new_session = new_session.format("4");

    // `[-n window-name]`
    #[cfg(feature = "tmux_0_8")]
    let new_session = new_session.window_name("5");

    // `[-s session-name]`
    #[cfg(feature = "tmux_0_8")]
    let new_session = new_session.session_name("6");

    // `[-t target-session]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_4")))]
    let new_session = new_session.target_session("7");

    // `[-t group-name]`
    #[cfg(feature = "tmux_2_4")]
    let new_session = new_session.group_name("8");

    // `[-x width]`
    #[cfg(feature = "tmux_1_5")]
    let new_session = new_session.width(9);

    // `[-y height]`
    #[cfg(feature = "tmux_1_5")]
    let new_session = new_session.height(10);

    // `[command]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let new_session = new_session.command("11");

    // `[shell-command]`
    #[cfg(feature = "tmux_1_5")]
    let new_session = new_session.shell_command("12");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "new-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "new";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_8")]
    v.push("-A");
    #[cfg(feature = "tmux_0_8")]
    v.push("-d");
    #[cfg(feature = "tmux_1_8")]
    v.push("-D");
    #[cfg(feature = "tmux_2_1")]
    v.push("-E");
    #[cfg(feature = "tmux_1_8")]
    v.push("-P");
    #[cfg(feature = "tmux_3_0a")]
    v.push("-X");
    #[cfg(feature = "tmux_1_9")]
    v.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-e", "2=3"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-f", "active-pane"]);
    #[cfg(feature = "tmux_1_8")]
    v.extend_from_slice(&["-F", "4"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-n", "5"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-s", "6"]);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_4")))]
    v.extend_from_slice(&["-t", "7"]);
    #[cfg(feature = "tmux_2_4")]
    v.extend_from_slice(&["-t", "8"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-x", "9"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-y", "10"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.push("11");
    #[cfg(feature = "tmux_1_5")]
    v.push("12");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let new_session = new_session.build().to_vec();

    assert_eq!(new_session, v);
}
