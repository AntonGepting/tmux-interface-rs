#[test]
fn set_local_session_options_tests() {
    use crate::{
        Action, Activity, DetachOnDestroy, SetGlobalSessionOptions, SetLocalSessionOptions,
        SetSessionOptions, SetUserOptions, Status, StatusJustify, StatusKeys, StatusPosition,
        Switch,
    };

    let cmd = "set";

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
    //let options = options.bell_on_alert(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    //let options = options.buffer_limit(Some(3));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "default-command", ""));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "default-shell", "/bin/bash"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.default_path(Some("6"));
    #[cfg(feature = "tmux_2_9")]
    v.push(format!("{} {} {}", cmd, "default-size", "80x24"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    //let options = options.default_terminal(Some("8"));
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
    //let options = options.lock_server(Some("17"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_attr(Some("18"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_bg(Some("19"));
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_attr(Some("20"));
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_bg(Some("21"));
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_fg(Some("22"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_fg(Some("23"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!(
        "{} {} {}",
        cmd, "message-command-style", "fg=blue,bg=black"
    ));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    //let options = options.message_limit(Some(25));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!(
        "{} {} {}",
        cmd, "message-style", "fg=colour232,bg=colour166,bold"
    ));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_resize_pane(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_select_pane(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_select_window(Some(Switch::On));
    #[cfg(feature = "tmux_2_1")]
    v.push(format!("{} {} {}", cmd, "mouse", "on"));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    //let options = options.mouse_utf8(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_active_border_bg(Some("27"));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_active_border_fg(Some("28"));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_border_bg(Some("29"));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_border_fg(Some("30"));
    //#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    //let options = options.pane_active_border_style(Some("31"));
    //#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    //let options = options.pane_border_style(Some("32"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "prefix", "C-a"));
    #[cfg(feature = "tmux_1_6")]
    v.push(format!("{} {} {}", cmd, "prefix2", "None"));
    #[cfg(feature = "tmux_1_7")]
    v.push(format!("{} {} {}", cmd, "renumber-windows", "off"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "repeat-time", "500"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    //let options = options.set_remain_on_exit(Some(Switch::On));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "set-titles", "on"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "set-titles-string", "#W"));
    #[cfg(feature = "tmux_2_6")]
    v.push(format!("{} {} {}", cmd, "silence-action", "other"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status", "on"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_attr(Some("36"));
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
    //let options = options.status_left_attr(Some("41"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_bg(Some("42"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_fg(Some("43"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-left-length", "50"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!("{} {} {}", cmd, "status-left-style", "default"));
    #[cfg(feature = "tmux_1_7")]
    v.push(format!("{} {} {}", cmd, "status-position", "bottom"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-right", "#[]"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_attr(Some("47"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_bg(Some("48"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_fg(Some("49"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-right-length", "50"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!("{} {} {}", cmd, "status-right-style", "default"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!("{} {} {}", cmd, "status-style", "fg=colour247"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    //let options = options.status_utf8(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    //let options = options.terminal_overrides(Some("53"));
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

    let options = SetLocalSessionOptions::new();

    #[cfg(feature = "tmux_2_6")]
    let options = options.activity_action(Some(Action::Other));
    #[cfg(feature = "tmux_1_8")]
    let options = options.assume_paste_time(Some(1));
    #[cfg(feature = "tmux_1_0")]
    let options = options.base_index(Some(1));
    #[cfg(feature = "tmux_1_0")]
    let options = options.bell_action(Some(Action::None));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    let options = options.bell_on_alert(Some(Switch::On));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    let options = options.buffer_limit(Some(3));
    #[cfg(feature = "tmux_1_0")]
    let options = options.default_command(Some(""));
    #[cfg(feature = "tmux_1_0")]
    let options = options.default_shell(Some("/bin/bash"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.default_path(Some("6"));
    #[cfg(feature = "tmux_2_9")]
    let options = options.default_size(Some((80, 24)));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    let options = options.default_terminal(Some("8"));
    #[cfg(feature = "tmux_1_4")]
    let options = options.destroy_unattached(Some(Switch::Off));
    #[cfg(feature = "tmux_1_4")]
    let options = options.detach_on_destroy(Some(DetachOnDestroy::On));
    #[cfg(feature = "tmux_1_2")]
    let options = options.display_panes_active_colour(Some("red"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_panes_colour(Some("blue"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_panes_time(Some(1000));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_time(Some(750));
    #[cfg(feature = "tmux_1_0")]
    let options = options.history_limit(Some(2000));
    #[cfg(feature = "tmux_2_2")]
    let options = options.key_table(Some("root"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.lock_after_time(Some(0));
    #[cfg(feature = "tmux_1_1")]
    let options = options.lock_command(Some("lock -np"));
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    let options = options.lock_server(Some("17"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_attr(Some("18"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_bg(Some("19"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_attr(Some("20"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_bg(Some("21"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_fg(Some("22"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_fg(Some("23"));
    #[cfg(feature = "tmux_1_9")]
    let options = options.message_command_style(Some("fg=blue,bg=black"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    let options = options.message_limit(Some(25));
    #[cfg(feature = "tmux_1_9")]
    let options = options.message_style(Some("fg=colour232,bg=colour166,bold"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_resize_pane(Some(Switch::On));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_select_pane(Some(Switch::On));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_select_window(Some(Switch::On));
    #[cfg(feature = "tmux_2_1")]
    let options = options.mouse(Some(Switch::On));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    let options = options.mouse_utf8(Some(Switch::On));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_active_border_bg(Some("27"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_active_border_fg(Some("28"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_border_bg(Some("29"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_border_fg(Some("30"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.pane_active_border_style(Some("31"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.pane_border_style(Some("32"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.prefix(Some("C-a"));
    #[cfg(feature = "tmux_1_6")]
    let options = options.prefix2(Some("None"));
    #[cfg(feature = "tmux_1_7")]
    let options = options.renumber_windows(Some(Switch::Off));
    #[cfg(feature = "tmux_1_0")]
    let options = options.repeat_time(Some(500));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    let options = options.set_remain_on_exit(Some(Switch::On));
    #[cfg(feature = "tmux_1_0")]
    let options = options.set_titles(Some(Switch::On));
    #[cfg(feature = "tmux_1_0")]
    let options = options.set_titles_string(Some("#W"));
    #[cfg(feature = "tmux_2_6")]
    let options = options.silence_action(Some(Action::Other));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status(Some(Status::On));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_attr(Some("36"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_bg(Some("default"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_fg(Some("colour12"));
    #[cfg(feature = "tmux_2_9")]
    let options = options.status_format(Some(vec!["#[0]".to_string(), "#[1]".to_string()]));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_interval(Some(2));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_justify(Some(StatusJustify::Left));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_keys(Some(StatusKeys::Emacs));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_left(Some("#(whoami)"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_attr(Some("41"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_bg(Some("42"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_fg(Some("43"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_left_length(Some(50));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_left_style(Some("default"));
    #[cfg(feature = "tmux_1_7")]
    let options = options.status_position(Some(StatusPosition::Bottom));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_right(Some("#[]"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_attr(Some("47"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_bg(Some("48"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_fg(Some("49"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_right_length(Some(50));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_right_style(Some("default"));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_style(Some("fg=colour247"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    let options = options.status_utf8(Some(Switch::On));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let options = options.terminal_overrides(Some("53"));
    #[cfg(feature = "tmux_1_0")]
    let options =
        options.update_environment(Some(vec!["DISPLAY".to_string(), "KRB5CCNAME".to_string()]));
    #[cfg(feature = "tmux_1_0")]
    let options = options.visual_activity(Some(Activity::Off));
    #[cfg(feature = "tmux_1_0")]
    let options = options.visual_bell(Some(Activity::Off));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let options = options.visual_content(Some(Switch::On));
    #[cfg(feature = "tmux_1_4")]
    let options = options.visual_silence(Some(Activity::Off));
    #[cfg(feature = "tmux_1_6")]
    let options = options.word_separators(Some("!\"#$%&'()*+,-./:;<=>?@[\\]^`{|}~"));
    let options = options.user_option("user-option-name", Some("value"));

    let options = options.build().to_string();

    assert_eq!(origin, options);
}

#[test]
fn set_global_session_options_tests() {
    use crate::{
        Action, Activity, DetachOnDestroy, SetGlobalSessionOptions, SetLocalSessionOptions,
        SetSessionOptions, SetUserOptions, Status, StatusJustify, StatusKeys, StatusPosition,
        Switch,
    };

    let cmd = "set -g";

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
    //let options = options.bell_on_alert(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    //let options = options.buffer_limit(Some(3));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "default-command", ""));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "default-shell", "/bin/bash"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.default_path(Some("6"));
    #[cfg(feature = "tmux_2_9")]
    v.push(format!("{} {} {}", cmd, "default-size", "80x24"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    //let options = options.default_terminal(Some("8"));
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
    //let options = options.lock_server(Some("17"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_attr(Some("18"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_bg(Some("19"));
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_attr(Some("20"));
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_bg(Some("21"));
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_fg(Some("22"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_fg(Some("23"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!(
        "{} {} {}",
        cmd, "message-command-style", "fg=blue,bg=black"
    ));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    //let options = options.message_limit(Some(25));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!(
        "{} {} {}",
        cmd, "message-style", "fg=colour232,bg=colour166,bold"
    ));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_resize_pane(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_select_pane(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_select_window(Some(Switch::On));
    #[cfg(feature = "tmux_2_1")]
    v.push(format!("{} {} {}", cmd, "mouse", "on"));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    //let options = options.mouse_utf8(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_active_border_bg(Some("27"));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_active_border_fg(Some("28"));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_border_bg(Some("29"));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_border_fg(Some("30"));
    //#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    //let options = options.pane_active_border_style(Some("31"));
    //#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    //let options = options.pane_border_style(Some("32"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "prefix", "C-a"));
    #[cfg(feature = "tmux_1_6")]
    v.push(format!("{} {} {}", cmd, "prefix2", "None"));
    #[cfg(feature = "tmux_1_7")]
    v.push(format!("{} {} {}", cmd, "renumber-windows", "off"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "repeat-time", "500"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    //let options = options.set_remain_on_exit(Some(Switch::On));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "set-titles", "on"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "set-titles-string", "#W"));
    #[cfg(feature = "tmux_2_6")]
    v.push(format!("{} {} {}", cmd, "silence-action", "other"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status", "on"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_attr(Some("36"));
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
    //let options = options.status_left_attr(Some("41"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_bg(Some("42"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_fg(Some("43"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-left-length", "50"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!("{} {} {}", cmd, "status-left-style", "default"));
    #[cfg(feature = "tmux_1_7")]
    v.push(format!("{} {} {}", cmd, "status-position", "bottom"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-right", "#[]"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_attr(Some("47"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_bg(Some("48"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_fg(Some("49"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-right-length", "50"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!("{} {} {}", cmd, "status-right-style", "default"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!("{} {} {}", cmd, "status-style", "fg=colour247"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    //let options = options.status_utf8(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    //let options = options.terminal_overrides(Some("53"));
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
    let options = options.activity_action(Some(Action::Other));
    #[cfg(feature = "tmux_1_8")]
    let options = options.assume_paste_time(Some(1));
    #[cfg(feature = "tmux_1_0")]
    let options = options.base_index(Some(1));
    #[cfg(feature = "tmux_1_0")]
    let options = options.bell_action(Some(Action::None));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    let options = options.bell_on_alert(Some(Switch::On));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    let options = options.buffer_limit(Some(3));
    #[cfg(feature = "tmux_1_0")]
    let options = options.default_command(Some(""));
    #[cfg(feature = "tmux_1_0")]
    let options = options.default_shell(Some("/bin/bash"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.default_path(Some("6"));
    #[cfg(feature = "tmux_2_9")]
    let options = options.default_size(Some((80, 24)));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    let options = options.default_terminal(Some("8"));
    #[cfg(feature = "tmux_1_4")]
    let options = options.destroy_unattached(Some(Switch::Off));
    #[cfg(feature = "tmux_1_4")]
    let options = options.detach_on_destroy(Some(DetachOnDestroy::On));
    #[cfg(feature = "tmux_1_2")]
    let options = options.display_panes_active_colour(Some("red"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_panes_colour(Some("blue"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_panes_time(Some(1000));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_time(Some(750));
    #[cfg(feature = "tmux_1_0")]
    let options = options.history_limit(Some(2000));
    #[cfg(feature = "tmux_2_2")]
    let options = options.key_table(Some("root"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.lock_after_time(Some(0));
    #[cfg(feature = "tmux_1_1")]
    let options = options.lock_command(Some("lock -np"));
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    let options = options.lock_server(Some("17"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_attr(Some("18"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_bg(Some("19"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_attr(Some("20"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_bg(Some("21"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_fg(Some("22"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_fg(Some("23"));
    #[cfg(feature = "tmux_1_9")]
    let options = options.message_command_style(Some("fg=blue,bg=black"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    let options = options.message_limit(Some(25));
    #[cfg(feature = "tmux_1_9")]
    let options = options.message_style(Some("fg=colour232,bg=colour166,bold"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_resize_pane(Some(Switch::On));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_select_pane(Some(Switch::On));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_select_window(Some(Switch::On));
    #[cfg(feature = "tmux_2_1")]
    let options = options.mouse(Some(Switch::On));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    let options = options.mouse_utf8(Some(Switch::On));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_active_border_bg(Some("27"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_active_border_fg(Some("28"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_border_bg(Some("29"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_border_fg(Some("30"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.pane_active_border_style(Some("31"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.pane_border_style(Some("32"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.prefix(Some("C-a"));
    #[cfg(feature = "tmux_1_6")]
    let options = options.prefix2(Some("None"));
    #[cfg(feature = "tmux_1_7")]
    let options = options.renumber_windows(Some(Switch::Off));
    #[cfg(feature = "tmux_1_0")]
    let options = options.repeat_time(Some(500));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    let options = options.set_remain_on_exit(Some(Switch::On));
    #[cfg(feature = "tmux_1_0")]
    let options = options.set_titles(Some(Switch::On));
    #[cfg(feature = "tmux_1_0")]
    let options = options.set_titles_string(Some("#W"));
    #[cfg(feature = "tmux_2_6")]
    let options = options.silence_action(Some(Action::Other));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status(Some(Status::On));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_attr(Some("36"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_bg(Some("default"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_fg(Some("colour12"));
    #[cfg(feature = "tmux_2_9")]
    let options = options.status_format(Some(vec!["#[0]".to_string(), "#[1]".to_string()]));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_interval(Some(2));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_justify(Some(StatusJustify::Left));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_keys(Some(StatusKeys::Emacs));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_left(Some("#(whoami)"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_attr(Some("41"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_bg(Some("42"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_fg(Some("43"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_left_length(Some(50));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_left_style(Some("default"));
    #[cfg(feature = "tmux_1_7")]
    let options = options.status_position(Some(StatusPosition::Bottom));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_right(Some("#[]"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_attr(Some("47"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_bg(Some("48"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_fg(Some("49"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_right_length(Some(50));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_right_style(Some("default"));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_style(Some("fg=colour247"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    let options = options.status_utf8(Some(Switch::On));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let options = options.terminal_overrides(Some("53"));
    #[cfg(feature = "tmux_1_0")]
    let options =
        options.update_environment(Some(vec!["DISPLAY".to_string(), "KRB5CCNAME".to_string()]));
    #[cfg(feature = "tmux_1_0")]
    let options = options.visual_activity(Some(Activity::Off));
    #[cfg(feature = "tmux_1_0")]
    let options = options.visual_bell(Some(Activity::Off));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let options = options.visual_content(Some(Switch::On));
    #[cfg(feature = "tmux_1_4")]
    let options = options.visual_silence(Some(Activity::Off));
    #[cfg(feature = "tmux_1_6")]
    let options = options.word_separators(Some("!\"#$%&'()*+,-./:;<=>?@[\\]^`{|}~"));
    let options = options.user_option("user-option-name", Some("value"));

    let options = options.build().to_string();

    assert_eq!(origin, options);
}
