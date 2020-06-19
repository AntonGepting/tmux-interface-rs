#[test]
fn list_panes() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^1.6:
        // ```text
        // tmux list-panes [-as] [-F format] [-t target]
        // (alias: lsp)
        // ```
        //
        // tmux ^1.5:
        // ```text
        // tmux list-panes [-as] [-t target]
        // (alias: lsp)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux list-panes [-t target]
        // (alias: lsp)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["list-panes", "-a", "-s", "-F", "1", "-t", "2"]"#
        );
        Err(Error::Hook)
    }));
    let target_window = TargetWindow::Raw("2").to_string();
    tmux.list_panes(Some(true), Some(true), Some("1"), Some(&target_window))
        .unwrap_err();
}
