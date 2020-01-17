#[test]
fn detach_client() {
    use crate::{DetachClient, Error, TmuxInterface};

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
        target_session: Some("2"),
        target_client: Some("3"),
    };
    tmux.detach_client(Some(&detach_client)).unwrap_err();
}
