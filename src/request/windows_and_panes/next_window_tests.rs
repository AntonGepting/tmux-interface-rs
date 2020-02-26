#[test]
fn next_window() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux next-window [-a] [-t target-session]
        // (alias: next)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["next-window", "-a", "-t", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.next_window(Some(true), Some("1")).unwrap_err();
}
