#[test]
fn display_menu() {
    use crate::{DisplayMenu, DisplayMenuBuilder, Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.0:
        // ```text
        // tmux display-menu [-c target-client] [-t target-pane] [-T title]
        // [-x position] [-y position] name key command ...
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("display-menu");
        #[cfg(feature = "tmux_3_0")]
        s.extend_from_slice(&["-c", "1"]);
        #[cfg(feature = "tmux_3_0")]
        s.extend_from_slice(&["-t", "2"]);
        #[cfg(feature = "tmux_3_0")]
        s.extend_from_slice(&["-T", "3"]);
        #[cfg(feature = "tmux_3_0")]
        s.extend_from_slice(&["-x", "4"]);
        #[cfg(feature = "tmux_3_0")]
        s.extend_from_slice(&["-y", "5"]);
        s.push("6");
        s.push("7");
        s.push("8");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let display_menu = DisplayMenu {
        #[cfg(feature = "tmux_3_0")]
        target_client: Some("1"),
        #[cfg(feature = "tmux_3_0")]
        target_pane: Some(&TargetPane::Raw("2")),
        #[cfg(feature = "tmux_3_0")]
        title: Some("3"),
        #[cfg(feature = "tmux_3_0")]
        x: Some(4),
        #[cfg(feature = "tmux_3_0")]
        y: Some(5),
    };
    tmux.display_menu(Some(&display_menu), "6", "7", "8")
        .unwrap_err();

    let mut builder = DisplayMenuBuilder::new();
    #[cfg(feature = "tmux_3_0")]
    builder.target_client("1");
    #[cfg(feature = "tmux_3_0")]
    builder.target_pane(&TargetPane::Raw("2"));
    #[cfg(feature = "tmux_3_0")]
    builder.title("3");
    #[cfg(feature = "tmux_3_0")]
    builder.x(4);
    #[cfg(feature = "tmux_3_0")]
    builder.y(5);
    let display_menu = builder.build();
    tmux.display_menu(Some(&display_menu), "6", "7", "8")
        .unwrap_err();
}
