#[test]
fn resize_window() {
    use crate::{Error, ResizeWindow, ResizeWindowBuilder, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^2.9a:
        // ```text
        // tmux resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
        // (alias: resizew)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["resize-window", "-a", "-A", "-D", "-L", "-R", "-U", "-t", "1", "-x", "2", "-y", "3", "4"]"#
        );
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("resize-window");
        #[cfg(feature = "tmux_2_9")]
        s.push("-a");
        #[cfg(feature = "tmux_2_9")]
        s.push("-A");
        #[cfg(feature = "tmux_2_9")]
        s.push("-D");
        #[cfg(feature = "tmux_2_9")]
        s.push("-L");
        #[cfg(feature = "tmux_2_9")]
        s.push("-R");
        #[cfg(feature = "tmux_2_9")]
        s.push("-U");
        #[cfg(feature = "tmux_2_9")]
        s.extend_from_slice(&["-t", "1"]);
        #[cfg(feature = "tmux_2_9")]
        s.extend_from_slice(&["-x", "2"]);
        #[cfg(feature = "tmux_2_9")]
        s.extend_from_slice(&["-y", "3"]);
        #[cfg(feature = "tmux_2_9")]
        s.push("4");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let resize_window = ResizeWindow {
        #[cfg(feature = "tmux_2_9")]
        smallest: Some(true),
        #[cfg(feature = "tmux_2_9")]
        largest: Some(true),
        #[cfg(feature = "tmux_2_9")]
        down: Some(true),
        #[cfg(feature = "tmux_2_9")]
        left: Some(true),
        #[cfg(feature = "tmux_2_9")]
        right: Some(true),
        #[cfg(feature = "tmux_2_9")]
        up: Some(true),
        #[cfg(feature = "tmux_2_9")]
        target_window: Some(&TargetWindow::Raw("1")),
        #[cfg(feature = "tmux_2_9")]
        width: Some(2),
        #[cfg(feature = "tmux_2_9")]
        height: Some(3),
        #[cfg(feature = "tmux_2_9")]
        adjustment: Some("4"),
    };
    tmux.resize_window(Some(&resize_window)).unwrap_err();

    let mut builder = ResizeWindowBuilder::new();
    #[cfg(feature = "tmux_2_9")]
    builder.smallest();
    #[cfg(feature = "tmux_2_9")]
    builder.largest();
    #[cfg(feature = "tmux_2_9")]
    builder.down();
    #[cfg(feature = "tmux_2_9")]
    builder.left();
    #[cfg(feature = "tmux_2_9")]
    builder.right();
    #[cfg(feature = "tmux_2_9")]
    builder.up();
    #[cfg(feature = "tmux_2_9")]
    builder.target_window(&TargetWindow::Raw("1"));
    #[cfg(feature = "tmux_2_9")]
    builder.width(2);
    #[cfg(feature = "tmux_2_9")]
    builder.height(3);
    #[cfg(feature = "tmux_2_9")]
    builder.adjustment("4");
    let resize_window = builder.build();
    tmux.resize_window(Some(&resize_window)).unwrap_err();
}
