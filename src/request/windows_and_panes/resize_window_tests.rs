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
        #[cfg(feature = "tmux_2_9a")]
        s.push("-a");
        #[cfg(feature = "tmux_2_9a")]
        s.push("-A");
        #[cfg(feature = "tmux_2_9a")]
        s.push("-D");
        #[cfg(feature = "tmux_2_9a")]
        s.push("-L");
        #[cfg(feature = "tmux_2_9a")]
        s.push("-R");
        #[cfg(feature = "tmux_2_9a")]
        s.push("-U");
        #[cfg(feature = "tmux_2_9a")]
        s.extend_from_slice(&["-t", "1"]);
        #[cfg(feature = "tmux_2_9a")]
        s.extend_from_slice(&["-x", "2"]);
        #[cfg(feature = "tmux_2_9a")]
        s.extend_from_slice(&["-y", "3"]);
        #[cfg(feature = "tmux_2_9a")]
        s.push("4");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let resize_window = ResizeWindow {
        #[cfg(feature = "tmux_2_9a")]
        smallest: Some(true),
        #[cfg(feature = "tmux_2_9a")]
        largest: Some(true),
        #[cfg(feature = "tmux_2_9a")]
        down: Some(true),
        #[cfg(feature = "tmux_2_9a")]
        left: Some(true),
        #[cfg(feature = "tmux_2_9a")]
        right: Some(true),
        #[cfg(feature = "tmux_2_9a")]
        up: Some(true),
        #[cfg(feature = "tmux_2_9a")]
        target_window: Some(&TargetWindow::Raw("1")),
        #[cfg(feature = "tmux_2_9a")]
        width: Some(2),
        #[cfg(feature = "tmux_2_9a")]
        height: Some(3),
        #[cfg(feature = "tmux_2_9a")]
        adjustment: Some("4"),
    };
    tmux.resize_window(Some(&resize_window)).unwrap_err();

    let mut builder = ResizeWindowBuilder::new()
    #[cfg(feature = "tmux_2_9a")]
    builder.smallest()
    #[cfg(feature = "tmux_2_9a")]
    builder.largest()
    #[cfg(feature = "tmux_2_9a")]
    builder.down()
    #[cfg(feature = "tmux_2_9a")]
    builder.left()
    #[cfg(feature = "tmux_2_9a")]
    builder.right()
    #[cfg(feature = "tmux_2_9a")]
    builder.up()
    #[cfg(feature = "tmux_2_9a")]
    builder.target_window(&TargetWindow::Raw("1"))
    #[cfg(feature = "tmux_2_9a")]
    builder.width(2)
    #[cfg(feature = "tmux_2_9a")]
    builder.height(3)
    #[cfg(feature = "tmux_2_9a")]
    builder.adjustment("4")
    let resize_window = builder.build();
    tmux.resize_window(Some(&resize_window)).unwrap_err();
}
