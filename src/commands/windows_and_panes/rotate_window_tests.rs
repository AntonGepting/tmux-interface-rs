#[test]
fn rotate_window() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.1:
        // ```text
        // tmux rotate-window [-DUZ] [-t target-window]
        // (alias: rotatew)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux rotate-window [-DU] [-t target-window]
        // (alias: rotatew)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("rotate-window");
        #[cfg(feature = "use_cmd_alias")]
        s.push("rotatew");
        #[cfg(feature = "tmux_0_8")]
        s.push("-D");
        #[cfg(feature = "tmux_0_8")]
        s.push("-U");
        #[cfg(feature = "tmux_3_1")]
        s.push("-Z");
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let target_window = TargetWindow::Raw("1").to_string();
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_1")))]
    tmux.rotate_window(Some(true), Some(true), Some(&target_window))
        .unwrap_err();

    #[cfg(feature = "tmux_3_1")]
    tmux.rotate_window(Some(true), Some(true), Some(true), Some(&target_window))
        .unwrap_err();
}
