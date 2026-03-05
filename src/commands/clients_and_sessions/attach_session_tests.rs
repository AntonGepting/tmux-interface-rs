// auto-generated file
//

// Attach client to already existing session
//
// # Manual
//
// tmux >=3.2:
// ```text
// attach-session [-dErx] [-c working-directory] [-f flags] [-t target-session]
// (alias: attach)
// ```
//
// tmux >=3.0a:
// ```text
// attach-session [-dErx] [-c working-directory] [-t target-session]
// (alias: attach)
// ```
//
// tmux >=2.1:
// ```text
// attach-session [-dEr] [-c working-directory] [-t target-session]
// (alias: attach)
// ```
//
// tmux >=1.9:
// ```text
// attach-session [-dr] [-c working-directory] [-t target-session]
// (alias: attach)
// ```
//
// tmux >=1.5:
// ```text
// attach-session [-dr] [-t target-session]
// (alias: attach)
// ```
//
// tmux >=0.8:
// ```text
// attach-session [-d] [-t target-session]
// (alias: attach)
// ```
#[test]
fn attach_session() {
    use crate::AttachSession;
    #[cfg(feature = "tmux_3_2")]
    use crate::ClientFlags;
    use std::borrow::Cow;

    let attach_session = AttachSession::new();
    // `[-d]`
    #[cfg(feature = "tmux_0_8")]
    let attach_session = attach_session.detach_other();

    // `[-E]`
    #[cfg(feature = "tmux_2_1")]
    let attach_session = attach_session.not_update_env();

    // `[-r]`
    #[cfg(feature = "tmux_1_5")]
    let attach_session = attach_session.read_only();

    // `[-x]`
    #[cfg(feature = "tmux_3_0a")]
    let attach_session = attach_session.parent_sighup();

    // `[-c working-directory]`
    #[cfg(feature = "tmux_1_9")]
    let attach_session = attach_session.working_directory("1");

    // `[-f flags]`
    // `[-f flags]`
    #[cfg(feature = "tmux_3_2")]
    let flags = ClientFlags {
        active_pane: Some(true),
        ..Default::default()
    };
    #[cfg(feature = "tmux_3_2")]
    let attach_session = attach_session.flags(flags);

    // `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    let attach_session = attach_session.target_session("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "attach-session";
    #[cfg(feature = "cmd_alias")]
    let cmd = "attach";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    v.push("-d");
    #[cfg(feature = "tmux_2_1")]
    v.push("-E");
    #[cfg(feature = "tmux_1_5")]
    v.push("-r");
    #[cfg(feature = "tmux_3_0a")]
    v.push("-x");
    #[cfg(feature = "tmux_1_9")]
    v.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-f", "active-pane"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "3"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let attach_session = attach_session.build().to_vec();

    assert_eq!(attach_session, v);
}
