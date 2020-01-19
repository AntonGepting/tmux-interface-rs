#[test]
fn list_windows() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux list-windows [-a] [-F format] [-t target-session]
        // (alias: lsw)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["list-windows", "-a", "-F", "1", "-t", "2"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.list_windows(Some(true), Some("1"), Some("2"))
        .unwrap_err();
}
