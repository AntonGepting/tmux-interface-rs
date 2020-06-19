#[test]
fn suspend_client() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux suspend-client [-t target-client]
        // (alias: suspendc)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["suspend-client", "-t", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.suspend_client(Some("1")).unwrap_err();
}
