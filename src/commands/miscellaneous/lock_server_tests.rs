#[test]
fn lock_server() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux lock-server
        // (alias: lock)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["lock-server"]"#
        );
        Err(Error::Hook)
    }));
    tmux.lock_server().unwrap_err();
}
