#[test]
fn attach_session() {
    use crate::{AttachSession, AttachSessionBuilder, Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.0:
        // ```text
        // tmux attach-session [-dErx] [-c working-directory] [-t target-session]
        // (alias: attach)
        // ```
        //
        // tmux ^2.1:
        // ```text
        // tmux attach-session [-dEr] [-c working-directory] [-t target-session]
        // (alias: attach)
        // ```
        //
        // tmux ^1.9:
        // ```text
        // tmux attach-session [-dr] [-c working-directory] [-t target-session]
        // (alias: attach)
        // ```
        //
        // tmux ^1.2:
        // ```text
        // tmux attach-session [-dr] [-t target-session]
        // (alias: attach)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux attach-session [-d] [-t target-session]
        // (alias: attach)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("attach-session");
        #[cfg(feature = "tmux_0_8")]
        s.push("-d");
        #[cfg(feature = "tmux_2_1")]
        s.push("-E");
        #[cfg(feature = "tmux_1_2")]
        s.push("-r");
        #[cfg(feature = "tmux_3_0")]
        s.push("-x");
        #[cfg(feature = "tmux_1_9")]
        s.extend_from_slice(&["-c", "1"]);
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "2"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let target_session = TargetSession::Raw("2").to_string();

    let attach_session = AttachSession {
        #[cfg(feature = "tmux_0_8")]
        detach_other: Some(true),
        #[cfg(feature = "tmux_2_1")]
        not_update_env: Some(true),
        #[cfg(feature = "tmux_1_2")]
        read_only: Some(true),
        #[cfg(feature = "tmux_3_0")]
        parent_sighup: Some(true),
        #[cfg(feature = "tmux_1_9")]
        cwd: Some("1"),
        #[cfg(feature = "tmux_0_8")]
        target_session: Some(&target_session),
    };
    tmux.attach_session(Some(&attach_session)).unwrap_err();

    let mut builder = AttachSessionBuilder::new();
    #[cfg(feature = "tmux_0_8")]
    builder.detach_other();
    #[cfg(feature = "tmux_2_1")]
    builder.not_update_env();
    #[cfg(feature = "tmux_1_2")]
    builder.read_only();
    #[cfg(feature = "tmux_3_0")]
    builder.parent_sighup();
    #[cfg(feature = "tmux_1_9")]
    builder.cwd("1");
    #[cfg(feature = "tmux_0_8")]
    builder.target_session(&target_session);
    let attach_session = builder.build();
    tmux.attach_session(Some(&attach_session)).unwrap_err();
}
