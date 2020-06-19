#[test]
fn detach_client() {
    use crate::{DetachClient, DetachClientBuilder, Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^2.4:
        // ```text
        // tmux detach-client [-aP] [-E shell-command] [-s target-session] [-t target-client]
        // (alias: detach)
        // ```
        //
        // tmux ^2.2:
        // ```text
        // tmux detach-client [-aP] [-s target-session] [-t target-client]
        // (alias: detach)
        // ```
        //
        // tmux ^1.5:
        // ```text
        // tmux detach-client [-P] [-s target-session] [-t target-client]
        // (alias: detach)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux detach-client [-t target-client]
        // (alias: detach)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("detach-client");
        #[cfg(feature = "tmux_2_2")]
        s.push("-a");
        #[cfg(feature = "tmux_1_5")]
        s.push("-P");
        #[cfg(feature = "tmux_2_4")]
        s.extend_from_slice(&["-E", "1"]);
        #[cfg(feature = "tmux_1_5")]
        s.extend_from_slice(&["-s", "2"]);
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "3"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_session = TargetSession::Raw("2").to_string();
    let detach_client = DetachClient {
        #[cfg(feature = "tmux_2_2")]
        all: Some(true),
        #[cfg(feature = "tmux_1_5")]
        parent_sighup: Some(true),
        #[cfg(feature = "tmux_2_4")]
        shell_command: Some("1"),
        #[cfg(feature = "tmux_1_5")]
        target_session: Some(&target_session),
        #[cfg(feature = "tmux_0_8")]
        target_client: Some("3"),
    };
    tmux.detach_client(Some(&detach_client)).unwrap_err();

    let mut builder = DetachClientBuilder::new();
    #[cfg(feature = "tmux_2_2")]
    builder.all();
    #[cfg(feature = "tmux_1_5")]
    builder.parent_sighup();
    #[cfg(feature = "tmux_2_4")]
    builder.shell_command("1");
    #[cfg(feature = "tmux_1_5")]
    builder.target_session(&target_session);
    #[cfg(feature = "tmux_0_8")]
    builder.target_client("3");
    let detach_client = builder.build();
    tmux.detach_client(Some(&detach_client)).unwrap_err();
}
