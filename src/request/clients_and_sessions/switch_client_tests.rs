#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn attach_session() {
    use crate::{Error, SwitchClient, SwitchClientBuilder, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux switch-client [-ElnprZ] [-c target-client] [-t target-session] [-T key-table]
        // (alias: switchc)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["switch-client", "-E", "-l", "-n", "-p", "-r", "-Z", "-c", "1", "-t", "2", "-T", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    let switch_client = SwitchClient {
        not_update_env: Some(true),
        last: Some(true),
        next: Some(true),
        previous: Some(true),
        read_only: Some(true),
        keep_zoomed: Some(true),
        target_client: Some("1"),
        target_session: Some(&TargetSession::Raw("2")),
        key_table: Some("3"),
    };
    tmux.switch_client(Some(&switch_client)).unwrap_err();

    let switch_client = SwitchClientBuilder::new()
        .not_update_env()
        .last()
        .next()
        .previous()
        .read_only()
        .keep_zoomed()
        .target_client("1")
        .target_session(&TargetSession::Raw("2"))
        .key_table("3")
        .build();
    tmux.switch_client(Some(&switch_client)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn attach_session() {
    use crate::{Error, SwitchClient, SwitchClientBuilder, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux switch-client [-Elnpr] [-c target-client] [-t target-session] [-T key-table]
        // (alias: switchc)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["switch-client", "-E", "-l", "-n", "-p", "-r", "-c", "1", "-t", "2", "-T", "3"]"#
        );
        Err(Error::new("hook"))
    }));

    let switch_client = SwitchClient {
        not_update_env: Some(true),
        last: Some(true),
        next: Some(true),
        previous: Some(true),
        read_only: Some(true),
        target_client: Some("1"),
        target_session: Some(&TargetSession::Raw("2")),
        key_table: Some("3"),
    };
    tmux.switch_client(Some(&switch_client)).unwrap_err();

    let switch_client = SwitchClientBuilder::new()
        .not_update_env()
        .last()
        .next()
        .previous()
        .read_only()
        .target_client("1")
        .target_session(&TargetSession::Raw("2"))
        .key_table("3")
        .build();
    tmux.switch_client(Some(&switch_client)).unwrap_err();
}
