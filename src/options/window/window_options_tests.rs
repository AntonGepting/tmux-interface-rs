#[test]
fn show_generated_struct() {
    use crate::WindowOptions;

    let _window_options = WindowOptions {
        ..Default::default()
    };
    //dbg!(window_options);
}

#[test]
fn bitflags() {
    use crate::{WINDOW_OPTIONS_ALL, WINDOW_OPTIONS_NONE};
    let bitflags =
        // 69___64_63____________________________32_31_____________________________0
        0b_0111111_11111111111111111111111111111111_11111111111111111111111111111111;
    //println!("{:b}", WINDOW_OPTIONS_ALL);
    //println!("{:b}", &bitflags);
    assert_eq!(bitflags, WINDOW_OPTIONS_ALL);
    assert_eq!(0, WINDOW_OPTIONS_NONE);
}

#[test]
fn parse() {
    use crate::WindowOptions;

    let window_options_str = r#"
        aggressive-resize off
        automatic-rename on
        automatic-rename-format ""
        clock-mode-colour colour135
        clock-mode-style 24
        force-height 0
        force-width 0
        main-pane-height 24
        main-pane-width 80
        mode-keys vi
        mode-style fg=colour196,bg=colour238,bright
        monitor-activity off
        monitor-bell on
        monitor-silence 0
        other-pane-height 0
        other-pane-width 0
        pane-active-border-style fg=colour114,bg=colour235
        pane-base-index 0
        pane-border-format ""
        pane-border-status off
        pane-border-style fg=colour238,bg=colour235
        synchronize-panes off
        window-status-activity-style reverse
        window-status-bell-style fg=colour253,bg=colour1,bright
        window-status-current-format ""
        window-status-current-style fg=colour22,bg=colour114
        window-status-format ""
        window-status-last-style default
        window-status-separator " "
        window-status-style fg=colour247
        wrap-search on
        xterm-keys on
    "#;
    let _window_options = window_options_str.parse::<WindowOptions>().unwrap();
    //dbg!(&window_options);
}

#[test]
fn to_string() {
    //use crate::{Switch, WindowOptionsBuilder};

    //let window_options = WindowOptionsBuilder::new().allow_rename(Switch::Off).build();
    //dbg!(&window_options.to_string());
}

#[test]
fn get() {
    use crate::WindowOptions;
    let _window_options = WindowOptions::get_all().unwrap();
    //dbg!(session_options);
    //assert_eq!(
    //window_options.window_status_separator,
    //Some(" ".to_string())
    //);
}
