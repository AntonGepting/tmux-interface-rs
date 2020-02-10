#[test]
fn detach_client() {
    use crate::{DetachClient, DetachClientBuilder, Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux detach-client [-aP] [-E shell-command] [-s target-session] [-t target-client]
        // (alias: detach)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["detach-client", "-a", "-P", "-E", "1", "-s", "2", "-t", "3"]"#
        );
        Err(Error::new("hook"))
    }));

    let detach_client = DetachClient {
        all: Some(true),
        parent_sighup: Some(true),
        shell_command: Some("1"),
        target_session: Some(&TargetSession::Raw("2")),
        target_client: Some("3"),
    };
    tmux.detach_client(Some(&detach_client)).unwrap_err();

    let detach_client = DetachClientBuilder::new()
        .all()
        .parent_sighup()
        .shell_command("1")
        .target_session(&TargetSession::Raw("2"))
        .target_client("3")
        .build();
    tmux.detach_client(Some(&detach_client)).unwrap_err();
}
