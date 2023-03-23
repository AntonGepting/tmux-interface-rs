#[test]
fn set_global_session_options_tests() {
    use crate::{
        Action, Activity, DetachOnDestroy, SetGlobalSessionOptions, SetSessionOptions,
        SetUserOptions, Status, StatusJustify, StatusKeys, StatusPosition, Switch,
    };

    let cmd = "set -g";
    let target = ":";
    let cmd = format!("{} -t {}", cmd, target);

    let mut v = Vec::new();

    #[cfg(feature = "tmux_2_6")]
    v.push(format!("{} {} {}", cmd, "activity-action", "other"));
    #[cfg(feature = "tmux_1_8")]
    v.push(format!("{} {} {}", cmd, "assume-paste-time", "1"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "base-index", "1"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "bell-action", "none"));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    //let options = options.bell_on_alert(Some(target), Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    //let options = options.buffer_limit(Some(target), Some(3));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "default-command", ""));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "default-shell", "/bin/bash"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.default_path(Some(target), Some("6"));
    #[cfg(feature = "tmux_2_9")]
    v.push(format!("{} {} {}", cmd, "default-size", "80x24"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    //let options = options.default_terminal(Some(target), Some("8"));
    #[cfg(feature = "tmux_1_4")]
    v.push(format!("{} {} {}", cmd, "destroy-unattached", "off"));
    #[cfg(feature = "tmux_1_4")]
    v.push(format!("{} {} {}", cmd, "detach-on-destroy", "on"));
    #[cfg(feature = "tmux_1_2")]
    v.push(format!(
        "{} {} {}",
        cmd, "display-panes-active-colour", "red"
    ));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "display-panes-colour", "blue"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "display-panes-time", "1000"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "display-time", "750"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "history-limit", "2000"));
    #[cfg(feature = "tmux_2_2")]
    v.push(format!("{} {} {}", cmd, "key-table", "root"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "lock-after-time", "0"));
    #[cfg(feature = "tmux_1_1")]
    v.push(format!("{} {} {}", cmd, "lock-command", "lock -np"));
    //#[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    //let options = options.lock_server(Some(target), Some("17"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_attr(Some(target), Some("18"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_bg(Some(target), Some("19"));
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_attr(Some(target), Some("20"));
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_bg(Some(target), Some("21"));
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_fg(Some(target), Some("22"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_fg(Some(target), Some("23"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!(
        "{} {} {}",
        cmd, "message-command-style", "fg=blue,bg=black"
    ));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    //let options = options.message_limit(Some(target), Some(25));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!(
        "{} {} {}",
        cmd, "message-style", "fg=colour232,bg=colour166,bold"
    ));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_resize_pane(Some(target), Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_select_pane(Some(target), Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_select_window(Some(target), Some(Switch::On));
    #[cfg(feature = "tmux_2_1")]
    v.push(format!("{} {} {}", cmd, "mouse", "on"));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    //let options = options.mouse_utf8(Some(target), Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_active_border_bg(Some(target), Some("27"));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_active_border_fg(Some(target), Some("28"));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_border_bg(Some(target), Some("29"));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_border_fg(Some(target), Some("30"));
    //#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    //let options = options.pane_active_border_style(Some(target), Some("31"));
    //#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    //let options = options.pane_border_style(Some(target), Some("32"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "prefix", "C-a"));
    #[cfg(feature = "tmux_1_6")]
    v.push(format!("{} {} {}", cmd, "prefix2", "None"));
    #[cfg(feature = "tmux_1_7")]
    v.push(format!("{} {} {}", cmd, "renumber-windows", "off"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "repeat-time", "500"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    //let options = options.set_remain_on_exit(Some(target), Some(Switch::On));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "set-titles", "on"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "set-titles-string", "#W"));
    #[cfg(feature = "tmux_2_6")]
    v.push(format!("{} {} {}", cmd, "silence-action", "other"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status", "on"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_attr(Some(target), Some("36"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "status-bg", "default"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "status-fg", "colour12"));
    #[cfg(feature = "tmux_2_9")]
    {
        v.push(format!("{} {} {}", cmd, "status-format[0]", "#[0]"));
        v.push(format!("{} {} {}", cmd, "status-format[1]", "#[1]"));
    }
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-interval", "2"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-justify", "left"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-keys", "emacs"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-left", "#(whoami)"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_attr(Some(target), Some("41"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_bg(Some(target), Some("42"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_fg(Some(target), Some("43"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-left-length", "50"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!("{} {} {}", cmd, "status-left-style", "default"));
    #[cfg(feature = "tmux_1_7")]
    v.push(format!("{} {} {}", cmd, "status-position", "bottom"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-right", "#[]"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_attr(Some(target), Some("47"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_bg(Some(target), Some("48"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_fg(Some(target), Some("49"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-right-length", "50"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!("{} {} {}", cmd, "status-right-style", "default"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!("{} {} {}", cmd, "status-style", "fg=colour247"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    //let options = options.status_utf8(Some(target), Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    //let options = options.terminal_overrides(Some(target), Some("53"));
    #[cfg(feature = "tmux_1_0")]
    {
        v.push(format!("{} {} {}", cmd, "update-environment[0]", "DISPLAY"));
        v.push(format!(
            "{} {} {}",
            cmd, "update-environment[1]", "KRB5CCNAME"
        ));
    }
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "visual-activity", "off"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "visual-bell", "off"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    //v.push(format!("{} {} {}", cmd, "visual-content", "fg=colour247"));
    #[cfg(feature = "tmux_1_4")]
    v.push(format!("{} {} {}", cmd, "visual-silence", "off"));
    #[cfg(feature = "tmux_1_6")]
    v.push(format!(
        "{} {} {}",
        cmd, "word-separators", "!\"#$%&'()*+,-./:;<=>?@[\\]^`{|}~"
    ));
    v.push(format!("{} {} {}", cmd, "@user-option-name", "value"));
    let origin = v.join(" ; ");

    let options = SetGlobalSessionOptions::new();

    #[cfg(feature = "tmux_2_6")]
    let options = options.activity_action(Some(target), Some(Action::Other));
    #[cfg(feature = "tmux_1_8")]
    let options = options.assume_paste_time(Some(target), Some(1));
    #[cfg(feature = "tmux_1_0")]
    let options = options.base_index(Some(target), Some(1));
    #[cfg(feature = "tmux_1_0")]
    let options = options.bell_action(Some(target), Some(Action::None));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    let options = options.bell_on_alert(Some(target), Some(Switch::On));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    let options = options.buffer_limit(Some(target), Some(3));
    #[cfg(feature = "tmux_1_0")]
    let options = options.default_command(Some(target), Some(""));
    #[cfg(feature = "tmux_1_0")]
    let options = options.default_shell(Some(target), Some("/bin/bash"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.default_path(Some(target), Some("6"));
    #[cfg(feature = "tmux_2_9")]
    let options = options.default_size(Some(target), Some((80, 24)));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    let options = options.default_terminal(Some(target), Some("8"));
    #[cfg(feature = "tmux_1_4")]
    let options = options.destroy_unattached(Some(target), Some(Switch::Off));
    #[cfg(feature = "tmux_1_4")]
    let options = options.detach_on_destroy(Some(target), Some(DetachOnDestroy::On));
    #[cfg(feature = "tmux_1_2")]
    let options = options.display_panes_active_colour(Some(target), Some("red"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_panes_colour(Some(target), Some("blue"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_panes_time(Some(target), Some(1000));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_time(Some(target), Some(750));
    #[cfg(feature = "tmux_1_0")]
    let options = options.history_limit(Some(target), Some(2000));
    #[cfg(feature = "tmux_2_2")]
    let options = options.key_table(Some(target), Some("root"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.lock_after_time(Some(target), Some(0));
    #[cfg(feature = "tmux_1_1")]
    let options = options.lock_command(Some(target), Some("lock -np"));
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    let options = options.lock_server(Some(target), Some("17"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_attr(Some(target), Some("18"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_bg(Some(target), Some("19"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_attr(Some(target), Some("20"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_bg(Some(target), Some("21"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_fg(Some(target), Some("22"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_fg(Some(target), Some("23"));
    #[cfg(feature = "tmux_1_9")]
    let options = options.message_command_style(Some(target), Some("fg=blue,bg=black"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    let options = options.message_limit(Some(target), Some(25));
    #[cfg(feature = "tmux_1_9")]
    let options = options.message_style(Some(target), Some("fg=colour232,bg=colour166,bold"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_resize_pane(Some(target), Some(Switch::On));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_select_pane(Some(target), Some(Switch::On));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_select_window(Some(target), Some(Switch::On));
    #[cfg(feature = "tmux_2_1")]
    let options = options.mouse(Some(target), Some(Switch::On));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    let options = options.mouse_utf8(Some(target), Some(Switch::On));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_active_border_bg(Some(target), Some("27"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_active_border_fg(Some(target), Some("28"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_border_bg(Some(target), Some("29"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_border_fg(Some(target), Some("30"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.pane_active_border_style(Some(target), Some("31"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.pane_border_style(Some(target), Some("32"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.prefix(Some(target), Some("C-a"));
    #[cfg(feature = "tmux_1_6")]
    let options = options.prefix2(Some(target), Some("None"));
    #[cfg(feature = "tmux_1_7")]
    let options = options.renumber_windows(Some(target), Some(Switch::Off));
    #[cfg(feature = "tmux_1_0")]
    let options = options.repeat_time(Some(target), Some(500));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    let options = options.set_remain_on_exit(Some(target), Some(Switch::On));
    #[cfg(feature = "tmux_1_0")]
    let options = options.set_titles(Some(target), Some(Switch::On));
    #[cfg(feature = "tmux_1_0")]
    let options = options.set_titles_string(Some(target), Some("#W"));
    #[cfg(feature = "tmux_2_6")]
    let options = options.silence_action(Some(target), Some(Action::Other));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status(Some(target), Some(Status::On));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_attr(Some(target), Some("36"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_bg(Some(target), Some("default"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_fg(Some(target), Some("colour12"));
    #[cfg(feature = "tmux_2_9")]
    let options = options.status_format(
        Some(target),
        Some(vec!["#[0]".to_string(), "#[1]".to_string()]),
    );
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_interval(Some(target), Some(2));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_justify(Some(target), Some(StatusJustify::Left));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_keys(Some(target), Some(StatusKeys::Emacs));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_left(Some(target), Some("#(whoami)"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_attr(Some(target), Some("41"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_bg(Some(target), Some("42"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_fg(Some(target), Some("43"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_left_length(Some(target), Some(50));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_left_style(Some(target), Some("default"));
    #[cfg(feature = "tmux_1_7")]
    let options = options.status_position(Some(target), Some(StatusPosition::Bottom));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_right(Some(target), Some("#[]"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_attr(Some(target), Some("47"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_bg(Some(target), Some("48"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_fg(Some(target), Some("49"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_right_length(Some(target), Some(50));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_right_style(Some(target), Some("default"));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_style(Some(target), Some("fg=colour247"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    let options = options.status_utf8(Some(target), Some(Switch::On));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let options = options.terminal_overrides(Some(target), Some("53"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.update_environment(
        Some(target),
        Some(vec!["DISPLAY".to_string(), "KRB5CCNAME".to_string()]),
    );
    #[cfg(feature = "tmux_1_0")]
    let options = options.visual_activity(Some(target), Some(Activity::Off));
    #[cfg(feature = "tmux_1_0")]
    let options = options.visual_bell(Some(target), Some(Activity::Off));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let options = options.visual_content(Some(target), Some(Switch::On));
    #[cfg(feature = "tmux_1_4")]
    let options = options.visual_silence(Some(target), Some(Activity::Off));
    #[cfg(feature = "tmux_1_6")]
    let options = options.word_separators(Some(target), Some("!\"#$%&'()*+,-./:;<=>?@[\\]^`{|}~"));
    let options = options.user_option(Some(target), "user-option-name", Some("value"));

    let options = options.build().to_string();

    assert_eq!(origin, options);
}
