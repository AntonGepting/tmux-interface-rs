#[test]
fn display_menu() {
    use crate::{DisplayMenu, Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux display-menu [-c target-client] [-t target-pane] [-T title]
        // [-x position] [-y position] name key command ...
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["display-menu", "-c", "1", "-t", "2", "-T", "3", "-x", "4", "-y", "5", "6", "7", "8"]"#
        );
        Err(Error::new("hook"))
    }));
    let display_menu = DisplayMenu {
        target_client: Some("1"),
        target_pane: Some("2"),
        title: Some("3"),
        x: Some(4),
        y: Some(5),
    };
    tmux.display_menu(Some(&display_menu), "6", "7", "8")
        .unwrap_err();
}
