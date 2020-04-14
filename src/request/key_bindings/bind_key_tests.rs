#[test]
fn bind_key() {
    use crate::{BindKey, BindKeyBuilder, Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux bind-key [-nr] [-N note] [-T key-table] key command [arguments]
        // (alias: bind)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["bind-key", "-n", "-r", "-N", "1", "-T", "2", "3", "4", "5"]"#
        );
        Err(Error::Hook)
    }));

    let bind_key = BindKey {
        root: Some(true),
        repeat: Some(true),
        note: Some("1"),
        key_table: Some("2"),
        arguments: Some("5"),
    };
    tmux.bind_key(Some(&bind_key), "3", "4").unwrap_err();

    let bind_key = BindKeyBuilder::new()
        .root()
        .repeat()
        .note("1")
        .key_table("2")
        .arguments("5")
        .build();
    tmux.bind_key(Some(&bind_key), "3", "4").unwrap_err();
}
