#[test]
fn next_layout() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux next-layout [-t target-window]
        // (alias: nextl)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["next-layout", "-t", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.next_layout(Some(&TargetWindow::Raw("1"))).unwrap_err();
}
