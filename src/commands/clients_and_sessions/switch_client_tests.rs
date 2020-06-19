#[test]
fn attach_session() {
    use crate::{Error, SwitchClient, SwitchClientBuilder, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.1:
        // ```text
        // tmux switch-client [-ElnprZ] [-c target-client] [-t target-session] [-T key-table]
        // (alias: switchc)
        // ```
        //
        // tmux ^2.1:
        // ```text
        // tmux switch-client [-Elnpr] [-c target-client] [-t target-session] [-T key-table]
        // (alias: switchc)
        // ```
        //
        // tmux ^1.6:
        // ```text
        // tmux switch-client [-lnpr] [-c target-client] [-t target-session]
        // (alias: switchc)
        // ```
        //
        // tmux ^1.4:
        // ```text
        // tmux switch-client [-lnp] [-c target-client] [-t target-session]
        // (alias: switchc)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux switch-client [-c target-client] [-t target-session]
        // (alias: switchc)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux switch-client [-c target-client -t target-session]
        // (alias: switchc)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("switch-client");
        #[cfg(feature = "tmux_2_1")]
        s.push("-E");
        #[cfg(feature = "tmux_1_4")]
        s.push("-l");
        #[cfg(feature = "tmux_1_4")]
        s.push("-n");
        #[cfg(feature = "tmux_1_4")]
        s.push("-p");
        #[cfg(feature = "tmux_1_6")]
        s.push("-r");
        #[cfg(feature = "tmux_3_1")]
        s.push("-Z");
        #[cfg(feature = "tmux_1_0")]
        s.extend_from_slice(&["-c", "1"]);
        #[cfg(feature = "tmux_1_0")]
        s.extend_from_slice(&["-t", "2"]);
        #[cfg(feature = "tmux_2_1")]
        s.extend_from_slice(&["-T", "3"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let target_session = TargetSession::Raw("2").to_string();
    let switch_client = SwitchClient {
        #[cfg(feature = "tmux_2_1")]
        not_update_env: Some(true),
        #[cfg(feature = "tmux_1_4")]
        last_session: Some(true),
        #[cfg(feature = "tmux_1_4")]
        next_session: Some(true),
        #[cfg(feature = "tmux_1_4")]
        previous_session: Some(true),
        #[cfg(feature = "tmux_1_6")]
        read_only: Some(true),
        #[cfg(feature = "tmux_3_1")]
        keep_zoomed: Some(true),
        #[cfg(feature = "tmux_1_0")]
        target_client: Some("1"),
        #[cfg(feature = "tmux_1_0")]
        target_session: Some(&target_session),
        #[cfg(feature = "tmux_2_1")]
        key_table: Some("3"),
    };
    tmux.switch_client(Some(&switch_client)).unwrap_err();

    let mut builder = SwitchClientBuilder::new();
    #[cfg(feature = "tmux_2_1")]
    builder.not_update_env();
    #[cfg(feature = "tmux_1_4")]
    builder.last_session();
    #[cfg(feature = "tmux_1_4")]
    builder.next_session();
    #[cfg(feature = "tmux_1_4")]
    builder.previous_session();
    #[cfg(feature = "tmux_1_6")]
    builder.read_only();
    #[cfg(feature = "tmux_3_1")]
    builder.keep_zoomed();
    #[cfg(feature = "tmux_1_0")]
    builder.target_client("1");
    #[cfg(feature = "tmux_1_0")]
    builder.target_session(&target_session);
    #[cfg(feature = "tmux_2_1")]
    builder.key_table("3");
    let switch_client = builder.build();
    tmux.switch_client(Some(&switch_client)).unwrap_err();
}
