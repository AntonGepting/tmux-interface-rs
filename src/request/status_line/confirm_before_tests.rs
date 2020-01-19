#[test]
fn confirm_before() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux confirm-before [-p prompt] [-t target-client] command
        // (alias: confirm)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["confirm-before", "-p", "1", "-t", "2", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.confirm_before(Some("1"), Some("2"), "3").unwrap_err();
}
