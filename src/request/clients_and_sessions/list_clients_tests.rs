#[test]
fn list_clients() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux list-clients [-F format] [-t target-session]
        // (alias: lsc)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["list-clients", "-F", "1", "-t", "2"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.list_clients(Some("1"), Some("2")).unwrap_err();
}
