#[test]
fn list_windows() {
    use crate::{Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^1.6:
        // ```text
        // tmux list-windows [-a] [-F format] [-t target-session]
        // (alias: lsw)
        // ```
        //
        // tmux ^1.5:
        // ```text
        // tmux list-windows [-a] [-t target-session]
        // (alias: lsw)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux list-windows [-t target-session]
        // (alias: lsw)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["list-windows", "-a", "-F", "1", "-t", "2"]"#
        );
        Err(Error::Hook)
    }));
    tmux.list_windows(Some(true), Some("1"), Some(&TargetSession::Raw("2")))
        .unwrap_err();
}
