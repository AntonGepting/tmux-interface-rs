#[test]
fn default() {
    use crate::{PaneOptions, RemainOnExit, Switch};

    let pane_options = PaneOptions::default();

    let options = PaneOptions::new();
    #[cfg(feature = "tmux_3_0")]
    let options = options.allow_rename(Some(Switch::Off));
    #[cfg(feature = "tmux_3_0")]
    let options = options.alternate_screen(Some(Switch::On));
    #[cfg(feature = "tmux_3_0")]
    let options = options.remain_on_exit(Some(RemainOnExit::Off));
    #[cfg(feature = "tmux_3_0")]
    let options = options.window_active_style(Some("default"));
    #[cfg(feature = "tmux_3_0")]
    let options = options.window_style(Some("default"));
    #[cfg(feature = "tmux_3_2")]
    let options = options.synchronize_panes(Some(Switch::Off));

    assert_eq!(pane_options, options);
}

#[test]
fn parse() {
    use crate::PaneOptions;

    let pane_options_str = r#"
        allow-rename off
        alternate-screen on
        remain-on-exit off
        window-active-style fg=colour253,bg=colour235
        window-style fg=colour247,bg=colour238
    "#;
    let _pane_options = pane_options_str.parse::<PaneOptions>().unwrap();
    //dbg!(&pane_options);
}

#[test]
fn to_string() {
    use crate::{PaneOptions, Switch};

    let _pane_options = PaneOptions::new().allow_rename(Some(Switch::Off));
    //dbg!(&pane_options.to_string());
}
