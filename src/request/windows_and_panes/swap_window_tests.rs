#[test]
fn swap_window() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux swap-window [-d] [-s src-window] [-t dst-window]
        // (alias: swapw)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["swap-window", "-d", "-s", "1", "-t", "2"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.swap_window(Some(true), Some("1"), Some("2"))
        .unwrap_err();
}
