#[test]
fn lock_client() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux lock-client [-t target-client]
        // (alias: lockc)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["lock-client", "-t", "1"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.lock_client(Some("1")).unwrap_err();
}
