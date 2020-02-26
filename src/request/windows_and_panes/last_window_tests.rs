#[test]
fn last_window() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux last-window [-t target-session]
        // (alias: last)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["last-window", "-t", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.last_window(Some("1")).unwrap_err();
}
