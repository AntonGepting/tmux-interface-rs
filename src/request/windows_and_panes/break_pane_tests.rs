#[test]
fn break_pane() {
    use crate::{BreakPane, BreakPaneBuilder, Error, TargetPane, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^2.4:
        // ```text
        // tmux break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
        // (alias: breakp)
        // ```
        //
        // tmux ^2.2:
        // ```text
        // tmux break-pane [-dP] [-F format] [-s src-pane] [-t dst-window]
        // (alias: breakp)
        // ```
        //
        // tmux ^2.1:
        // ```text
        // tmux break-pane [-dP] [-F format] [-s src-pane] [-t dst-pane]
        // (alias: breakp)
        // ```
        //
        // tmux ^1.7:
        // ```text
        // tmux break-pane [-dP] [-F format] [-t target-pane]
        // (alias: breakp)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux break-pane [-d] [-t target-window]
        // (alias: breakp)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux break-pane [-d] [-p pane-index] [-t target-window]
        // (alias: breakp)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["break-pane", "-d", "-P", "-F", "1", "-n", "2", "-s", "3", "-t", "4"]"#
        );
        Err(Error::Hook)
    }));

    let break_pane = BreakPane {
        detached: Some(true),
        print: Some(true),
        format: Some("1"),
        window_name: Some("2"),
        src_pane: Some(&TargetPane::Raw("3")),
        dst_window: Some(&TargetWindow::Raw("4")),
    };
    tmux.break_pane(Some(&break_pane)).unwrap_err();

    let break_pane = BreakPaneBuilder::new()
        .detached()
        .print()
        .format("1")
        .window_name("2")
        .src_pane(&TargetPane::Raw("3"))
        .dst_window(&TargetWindow::Raw("4"))
        .build();
    tmux.break_pane(Some(&break_pane)).unwrap_err();
}
