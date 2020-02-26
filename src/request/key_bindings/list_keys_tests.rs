#[test]
fn list_keys() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux list-keys [-T key-table]
        // (alias: lsk)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["list-keys", "-T", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.list_keys(Some("1")).unwrap_err();
}
