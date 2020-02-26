#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn display_panes() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux display-panes [-b] [-d duration] [-t target-client] [template] (alias: displayp)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["display-panes", "-b", "-d", "1", "-t", "2", "3"]"#
        );
        Err(Error::Hook)
    }));
    let _ = tmux.display_panes(Some(true), Some("1"), Some("2"), Some("3"));
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn display_panes() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux display-panes [-d duration] [-t target-client] [template]
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["display-panes", "-d", "1", "-t", "2", "3"]"#
        );
        Err(Error::Hook)
    }));
    let _ = tmux.display_panes(Some("1"), Some("2"), Some("3"));
}
