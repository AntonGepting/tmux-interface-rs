#[test]
fn previous_layout() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^1.3:
        // ```text
        // tmux previous-layout [-t target-window]
        // (alias: prevl)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["previous-layout", "-t", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.previous_layout(Some(&TargetWindow::Raw("1")))
        .unwrap_err();
}
