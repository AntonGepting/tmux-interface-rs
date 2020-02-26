#[test]
fn link_window() {
    use crate::{Error, LinkWindow, LinkWindowBuilder, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux link-window [-adk] [-s src-window] [-t dst-window]
        // (alias: linkw)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["link-window", "-a", "-d", "-k", "-s", "1", "-t", "2"]"#
        );
        Err(Error::Hook)
    }));

    let link_window = LinkWindow {
        add: Some(true),
        detached: Some(true),
        kill: Some(true),
        src_window: Some(&TargetWindow::Raw("1")),
        dst_window: Some(&TargetWindow::Raw("2")),
    };
    tmux.link_window(Some(&link_window)).unwrap_err();

    let link_window = LinkWindowBuilder::new()
        .add()
        .detached()
        .kill()
        .src_window(&TargetWindow::Raw("1"))
        .dst_window(&TargetWindow::Raw("2"))
        .build();
    tmux.link_window(Some(&link_window)).unwrap_err();
}
