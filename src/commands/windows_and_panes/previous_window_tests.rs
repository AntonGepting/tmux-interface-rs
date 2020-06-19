#[test]
fn previous_window() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^0.9:
        // ```text
        // tmux previous-window [-a] [-t target-session]
        // (alias: prev)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux previous-window [-t target-session]
        // (alias: prev)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["previous-window", "-a", "-t", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.previous_window(Some(true), Some("1")).unwrap_err();
}
