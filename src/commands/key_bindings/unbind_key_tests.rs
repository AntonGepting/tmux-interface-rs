#[test]
fn unbind_key() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux unbind-key [-an] [-T key-table] key
        // (alias: unbind)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["unbind-key", "-a", "-n", "-T", "1", "2"]"#
        );
        Err(Error::Hook)
    }));
    tmux.unbind_key(Some(true), Some(true), Some("1"), "2")
        .unwrap_err();
}
