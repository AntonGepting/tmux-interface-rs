#[test]
fn display_menu() {
    use crate::{DisplayMenu, DisplayMenuBuilder, Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux display-menu [-c target-client] [-t target-pane] [-T title]
        // [-x position] [-y position] name key command ...
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["display-menu", "-c", "1", "-t", "2", "-T", "3", "-x", "4", "-y", "5", "6", "7", "8"]"#
        );
        Err(Error::Hook)
    }));

    let display_menu = DisplayMenu {
        target_client: Some("1"),
        target_pane: Some(&TargetPane::Raw("2")),
        title: Some("3"),
        x: Some(4),
        y: Some(5),
    };
    tmux.display_menu(Some(&display_menu), "6", "7", "8")
        .unwrap_err();

    let display_menu = DisplayMenuBuilder::new()
        .target_client("1")
        .target_pane(&TargetPane::Raw("2"))
        .title("3")
        .x(4)
        .y(5)
        .build();
    tmux.display_menu(Some(&display_menu), "6", "7", "8")
        .unwrap_err();
}
