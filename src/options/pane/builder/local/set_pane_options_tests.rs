#[test]
fn set_server_options() {
    use crate::{RemainOnExit, SetPaneOptions, SetPaneOptionsTr, Switch};

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-option";
    #[cfg(feature = "cmd_alias")]
    let cmd = "set";

    let target = ":";
    let cmd = format!("{} -p -t {}", cmd, target);

    let mut v = Vec::new();

    #[cfg(feature = "tmux_3_0")]
    v.push(format!("{} {} {}", cmd, "allow-rename", "off"));
    #[cfg(feature = "tmux_3_0")]
    v.push(format!("{} {} {}", cmd, "alternate-screen", "off"));
    #[cfg(feature = "tmux_3_0")]
    v.push(format!("{} {} {}", cmd, "remain-on-exit", "off"));
    #[cfg(feature = "tmux_3_0")]
    v.push(format!("{} {} {}", cmd, "window-active-style", "default"));
    #[cfg(feature = "tmux_3_0")]
    v.push(format!("{} {} {}", cmd, "window-style", "default"));
    #[cfg(feature = "tmux_3_2")]
    v.push(format!("{} {} {}", cmd, "synchronize-panes", "off"));
    // #[cfg(feature = "tmux_3_0")]
    // v.push(format!("{} {} {}", cmd, "@user_option_name"));
    let origin = v.join(" ; ");

    let options = SetPaneOptions::new();
    #[cfg(feature = "tmux_3_0")]
    let options = options.allow_rename(Some(target), Some(Switch::Off));
    #[cfg(feature = "tmux_3_0")]
    let options = options.alternate_screen(Some(target), Some(Switch::Off));
    #[cfg(feature = "tmux_3_0")]
    let options = options.remain_on_exit(Some(target), Some(RemainOnExit::Off));
    #[cfg(feature = "tmux_3_0")]
    let options = options.window_active_style(Some(target), Some("default"));
    #[cfg(feature = "tmux_3_0")]
    let options = options.window_style(Some(target), Some("default"));
    #[cfg(feature = "tmux_3_2")]
    let options = options.synchronize_panes(Some(target), Some(Switch::Off));
    // #[cfg(feature = "tmux_3_0")]
    // let options = options.user_option(Some(target), "user_option_name", Some());

    let options = options.build().to_string();

    assert_eq!(options, origin);
}
