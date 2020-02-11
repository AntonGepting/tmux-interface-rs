#[test]
fn select_window() {
    use crate::{Error, SelectWindow, SelectWindowBuilder, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux select-window [-lnpT] [-t target-window]
        // (alias: selectw)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["select-window", "-l", "-n", "-p", "-T", "-t", "1"]"#
        );
        Err(Error::new("hook"))
    }));

    let select_window = SelectWindow {
        last: Some(true),
        next: Some(true),
        previous: Some(true),
        switch: Some(true),
        target_window: Some(&TargetWindow::Raw("1")),
    };
    tmux.select_window(Some(&select_window)).unwrap_err();

    let select_window = SelectWindowBuilder::new()
        .last()
        .next()
        .previous()
        .switch()
        .target_window(&TargetWindow::Raw("1"))
        .build();
    tmux.select_window(Some(&select_window)).unwrap_err();
}
