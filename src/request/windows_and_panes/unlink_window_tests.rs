#[test]
fn unlink_window() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux unlink-window [-k] [-t target-window]
        // (alias: unlinkw)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["unlink-window", "-k", "-t", "1"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.unlink_window(Some(true), Some("1")).unwrap_err();
}
