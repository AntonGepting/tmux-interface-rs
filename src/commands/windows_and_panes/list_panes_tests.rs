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
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("list-panes");
        #[cfg(feature = "use_cmd_alias")]
        s.push("lsp");
        #[cfg(feature = "tmux_1_5")]
        s.push("-a");
        #[cfg(feature = "tmux_1_5")]
        s.push("-s");
        #[cfg(feature = "tmux_1_6")]
        s.extend_from_slice(&["-F", "1"]);
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "2"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_window = TargetWindow::Raw("2").to_string();
    tmux.list_panes(Some(true), Some(true), Some("1"), Some(&target_window))
        .unwrap_err();
}
