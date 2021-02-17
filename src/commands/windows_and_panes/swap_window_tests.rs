#[test]
fn swap_window() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^0.8:
        // ```text
        // tmux swap-window [-d] [-s src-window] [-t dst-window]
        // (alias: swapw)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("swap-window");
        #[cfg(feature = "use_cmd_alias")]
        s.push("swapw");
        #[cfg(feature = "tmux_0_8")]
        s.push("-d");
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-s", "1"]);
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "2"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let src_window = TargetWindow::Raw("1").to_string();
    let dst_window = TargetWindow::Raw("2").to_string();

    tmux.swap_window(Some(true), Some(&src_window), Some(&dst_window))
        .unwrap_err();
}
