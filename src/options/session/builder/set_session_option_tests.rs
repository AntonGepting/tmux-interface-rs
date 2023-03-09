#[test]
fn set_local_session_option_tests() {
    use crate::{
        Action, Activity, DetachOnDestroy, SetLocalSessionOption, SetSessionOption, SetUserOption,
        Status, StatusJustify, StatusKeys, StatusPosition, Switch,
    };

    let cmd = "set";

    #[cfg(feature = "tmux_2_6")]
    {
        let origin = format!("{} {} {}", cmd, "activity-action", "other");
        let set_option = SetLocalSessionOption::activity_action(Some(Action::Other)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_8")]
    {
        let origin = format!("{} {} {}", cmd, "assume-paste-time", "1");
        let set_option = SetLocalSessionOption::assume_paste_time(Some(1)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "base-index", "1");
        let set_option = SetLocalSessionOption::base_index(Some(1)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "bell-action", "none");
        let set_option = SetLocalSessionOption::bell_action(Some(Action::None)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    //let options = options.bell_on_alert(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    //let options = options.buffer_limit(Some(3));
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "default-command", "");
        let set_option = SetLocalSessionOption::default_command(Some("")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "default-shell", "/bin/bash");
        let set_option = SetLocalSessionOption::default_shell(Some("/bin/bash")).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.default_path(Some("6"));
    #[cfg(feature = "tmux_2_9")]
    {
        let origin = format!("{} {} {}", cmd, "default-size", "80x24");
        let set_option = SetLocalSessionOption::default_size(Some((80, 24))).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    //let options = options.default_terminal(Some("8"));
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {} {}", cmd, "destroy-unattached", "off");
        let set_option = SetLocalSessionOption::destroy_unattached(Some(Switch::Off)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {} {}", cmd, "detach-on-destroy", "on");
        let set_option =
            SetLocalSessionOption::detach_on_destroy(Some(DetachOnDestroy::On)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_2")]
    {
        let origin = format!("{} {} {}", cmd, "display-panes-active-colour", "red");
        let set_option =
            SetLocalSessionOption::display_panes_active_colour(Some("red")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "display-panes-colour", "blue");
        let set_option = SetLocalSessionOption::display_panes_colour(Some("blue")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "display-panes-time", "1000");
        let set_option = SetLocalSessionOption::display_panes_time(Some(1000)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "display-time", "750");
        let set_option = SetLocalSessionOption::display_time(Some(750)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "history-limit", "2000");
        let set_option = SetLocalSessionOption::history_limit(Some(2000)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_2")]
    {
        let origin = format!("{} {} {}", cmd, "key-table", "root");
        let set_option = SetLocalSessionOption::key_table(Some("root")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "lock-after-time", "0");
        let set_option = SetLocalSessionOption::lock_after_time(Some(0)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_1")]
    {
        let origin = format!("{} {} {}", cmd, "lock-command", "lock -np");
        let set_option = SetLocalSessionOption::lock_command(Some("lock -np")).to_string();
        assert_eq!(origin, set_option);
    }
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
    {
        let origin = format!("{} {} {}", cmd, "message-command-style", "fg=blue,bg=black");
        let set_option =
            SetLocalSessionOption::message_command_style(Some("fg=blue,bg=black")).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    //let options = options.message_limit(Some(25));
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!(
            "{} {} {}",
            cmd, "message-style", "fg=colour232,bg=colour166,bold"
        );
        let set_option =
            SetLocalSessionOption::message_style(Some("fg=colour232,bg=colour166,bold"))
                .to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_resize_pane(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_select_pane(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_select_window(Some(Switch::On));
    #[cfg(feature = "tmux_2_1")]
    {
        let origin = format!("{} {} {}", cmd, "mouse", "on");
        let set_option = SetLocalSessionOption::mouse(Some(Switch::On)).to_string();
        assert_eq!(origin, set_option);
    }
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
    {
        let origin = format!("{} {} {}", cmd, "prefix", "C-a");
        let set_option = SetLocalSessionOption::prefix(Some("C-a")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_6")]
    {
        let origin = format!("{} {} {}", cmd, "prefix2", "None");
        let set_option = SetLocalSessionOption::prefix2(Some("None")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_7")]
    {
        let origin = format!("{} {} {}", cmd, "renumber-windows", "off");
        let set_option = SetLocalSessionOption::renumber_windows(Some(Switch::Off)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "repeat-time", "500");
        let set_option = SetLocalSessionOption::repeat_time(Some(500)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    //let options = options.set_remain_on_exit(Some(Switch::On));
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "set-titles", "on");
        let set_option = SetLocalSessionOption::set_titles(Some(Switch::On)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "set-titles-string", "#W");
        let set_option = SetLocalSessionOption::set_titles_string(Some("#W")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_6")]
    {
        let origin = format!("{} {} {}", cmd, "silence-action", "other");
        let set_option = SetLocalSessionOption::silence_action(Some(Action::Other)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status", "on");
        let set_option = SetLocalSessionOption::status(Some(Status::On)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_attr(Some("36"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {} {}", cmd, "status-bg", "default");
        let set_option = SetLocalSessionOption::status_bg(Some("default")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {} {}", cmd, "status-fg", "colour12");
        let set_option = SetLocalSessionOption::status_fg(Some("colour12")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_9")]
    {
        let origin = format!("{} {} {}", cmd, "assume-paste-time", "1");
        let set_option = SetLocalSessionOption::assume_paste_time(Some(1)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_9")]
    {
        let origin0 = format!("{} {} {}", cmd, "status-format[0]", "#[0]");
        let origin1 = format!("{} {} {}", cmd, "status-format[1]", "#[1]");
        let origin = format!("{} ; {}", origin0, origin1);
        let set_option = SetLocalSessionOption::status_format(Some(vec![
            "#[0]".to_string(),
            "#[1]".to_string(),
        ]))
        .to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-interval", "2");
        let set_option = SetLocalSessionOption::status_interval(Some(2)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-justify", "left");
        let set_option =
            SetLocalSessionOption::status_justify(Some(StatusJustify::Left)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-keys", "emacs");
        let set_option = SetLocalSessionOption::status_keys(Some(StatusKeys::Emacs)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-left", "#(whoami)");
        let set_option = SetLocalSessionOption::status_left(Some("#(whoami)")).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_attr(Some("41"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_bg(Some("42"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_fg(Some("43"));
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-left-length", "50");
        let set_option = SetLocalSessionOption::status_left_length(Some(50)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {} {}", cmd, "status-left-style", "default");
        let set_option = SetLocalSessionOption::status_left_style(Some("default")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_7")]
    {
        let origin = format!("{} {} {}", cmd, "status-position", "bottom");
        let set_option =
            SetLocalSessionOption::status_position(Some(StatusPosition::Bottom)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-right", "#[]");
        let set_option = SetLocalSessionOption::status_right(Some("#[]")).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_attr(Some("47"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_bg(Some("48"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_fg(Some("49"));
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-right-length", "50");
        let set_option = SetLocalSessionOption::status_right_length(Some(50)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {} {}", cmd, "status-right-style", "default");
        let set_option = SetLocalSessionOption::status_right_style(Some("default")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {} {}", cmd, "status-style", "fg=colour247");
        let set_option = SetLocalSessionOption::status_style(Some("fg=colour247")).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    //let options = options.status_utf8(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    //let options = options.terminal_overrides(Some("53"));
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "assume-paste-time", "1");
        let set_option = SetLocalSessionOption::assume_paste_time(Some(1)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin0 = format!("{} {} {}", cmd, "update-environment[0]", "DISPLAY");
        let origin1 = format!("{} {} {}", cmd, "update-environment[1]", "KRB5CCNAME");
        let origin = format!("{} ; {}", origin0, origin1);
        let set_option = SetLocalSessionOption::update_environment(Some(vec![
            "DISPLAY".to_string(),
            "KRB5CCNAME".to_string(),
        ]))
        .to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "visual-activity", "off");
        let set_option = SetLocalSessionOption::visual_activity(Some(Activity::Off)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "visual-bell", "off");
        let set_option = SetLocalSessionOption::visual_bell(Some(Activity::Off)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    //v.push(format!("{} {} {}", cmd, "visual-content", "fg=colour247"));
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {} {}", cmd, "visual-silence", "off");
        let set_option = SetLocalSessionOption::visual_silence(Some(Activity::Off)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_6")]
    {
        let origin = format!(
            "{} {} {}",
            cmd, "word-separators", "!\"#$%&'()*+,-./;<=>?@[\\]^`{|}~"
        );
        let set_option =
            SetLocalSessionOption::word_separators(Some("!\"#$%&'()*+,-./;<=>?@[\\]^`{|}~"))
                .to_string();
        assert_eq!(origin, set_option);
    }
    {
        let origin = format!("{} {} {}", cmd, "@user-option-name", "value");
        let set_option =
            SetLocalSessionOption::user_option("user-option-name", Some("value")).to_string();
        assert_eq!(origin, set_option);
    }
}

#[test]
fn set_global_session_option_tests() {
    use crate::{
        Action, Activity, DetachOnDestroy, SetGlobalSessionOption, SetSessionOption, SetUserOption,
        Status, StatusJustify, StatusKeys, StatusPosition, Switch,
    };

    let cmd = "set -g";

    #[cfg(feature = "tmux_2_6")]
    {
        let origin = format!("{} {} {}", cmd, "activity-action", "other");
        let set_option = SetGlobalSessionOption::activity_action(Some(Action::Other)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_8")]
    {
        let origin = format!("{} {} {}", cmd, "assume-paste-time", "1");
        let set_option = SetGlobalSessionOption::assume_paste_time(Some(1)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "base-index", "1");
        let set_option = SetGlobalSessionOption::base_index(Some(1)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "bell-action", "none");
        let set_option = SetGlobalSessionOption::bell_action(Some(Action::None)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    //let options = options.bell_on_alert(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    //let options = options.buffer_limit(Some(3));
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "default-command", "");
        let set_option = SetGlobalSessionOption::default_command(Some("")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "default-shell", "/bin/bash");
        let set_option = SetGlobalSessionOption::default_shell(Some("/bin/bash")).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.default_path(Some("6"));
    #[cfg(feature = "tmux_2_9")]
    {
        let origin = format!("{} {} {}", cmd, "default-size", "80x24");
        let set_option = SetGlobalSessionOption::default_size(Some((80, 24))).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    //let options = options.default_terminal(Some("8"));
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {} {}", cmd, "destroy-unattached", "off");
        let set_option = SetGlobalSessionOption::destroy_unattached(Some(Switch::Off)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {} {}", cmd, "detach-on-destroy", "on");
        let set_option =
            SetGlobalSessionOption::detach_on_destroy(Some(DetachOnDestroy::On)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_2")]
    {
        let origin = format!("{} {} {}", cmd, "display-panes-active-colour", "red");
        let set_option =
            SetGlobalSessionOption::display_panes_active_colour(Some("red")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "display-panes-colour", "blue");
        let set_option = SetGlobalSessionOption::display_panes_colour(Some("blue")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "display-panes-time", "1000");
        let set_option = SetGlobalSessionOption::display_panes_time(Some(1000)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "display-time", "750");
        let set_option = SetGlobalSessionOption::display_time(Some(750)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "history-limit", "2000");
        let set_option = SetGlobalSessionOption::history_limit(Some(2000)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_2")]
    {
        let origin = format!("{} {} {}", cmd, "key-table", "root");
        let set_option = SetGlobalSessionOption::key_table(Some("root")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "lock-after-time", "0");
        let set_option = SetGlobalSessionOption::lock_after_time(Some(0)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_1")]
    {
        let origin = format!("{} {} {}", cmd, "lock-command", "lock -np");
        let set_option = SetGlobalSessionOption::lock_command(Some("lock -np")).to_string();
        assert_eq!(origin, set_option);
    }
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
    {
        let origin = format!("{} {} {}", cmd, "message-command-style", "fg=blue,bg=black");
        let set_option =
            SetGlobalSessionOption::message_command_style(Some("fg=blue,bg=black")).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    //let options = options.message_limit(Some(25));
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!(
            "{} {} {}",
            cmd, "message-style", "fg=colour232,bg=colour166,bold"
        );
        let set_option =
            SetGlobalSessionOption::message_style(Some("fg=colour232,bg=colour166,bold"))
                .to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_resize_pane(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_select_pane(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_select_window(Some(Switch::On));
    #[cfg(feature = "tmux_2_1")]
    {
        let origin = format!("{} {} {}", cmd, "mouse", "on");
        let set_option = SetGlobalSessionOption::mouse(Some(Switch::On)).to_string();
        assert_eq!(origin, set_option);
    }
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
    {
        let origin = format!("{} {} {}", cmd, "prefix", "C-a");
        let set_option = SetGlobalSessionOption::prefix(Some("C-a")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_6")]
    {
        let origin = format!("{} {} {}", cmd, "prefix2", "None");
        let set_option = SetGlobalSessionOption::prefix2(Some("None")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_7")]
    {
        let origin = format!("{} {} {}", cmd, "renumber-windows", "off");
        let set_option = SetGlobalSessionOption::renumber_windows(Some(Switch::Off)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "repeat-time", "500");
        let set_option = SetGlobalSessionOption::repeat_time(Some(500)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    //let options = options.set_remain_on_exit(Some(Switch::On));
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "set-titles", "on");
        let set_option = SetGlobalSessionOption::set_titles(Some(Switch::On)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "set-titles-string", "#W");
        let set_option = SetGlobalSessionOption::set_titles_string(Some("#W")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_6")]
    {
        let origin = format!("{} {} {}", cmd, "silence-action", "other");
        let set_option = SetGlobalSessionOption::silence_action(Some(Action::Other)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status", "on");
        let set_option = SetGlobalSessionOption::status(Some(Status::On)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_attr(Some("36"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {} {}", cmd, "status-bg", "default");
        let set_option = SetGlobalSessionOption::status_bg(Some("default")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {} {}", cmd, "status-fg", "colour12");
        let set_option = SetGlobalSessionOption::status_fg(Some("colour12")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_9")]
    {
        let origin = format!("{} {} {}", cmd, "assume-paste-time", "1");
        let set_option = SetGlobalSessionOption::assume_paste_time(Some(1)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_9")]
    {
        let origin0 = format!("{} {} {}", cmd, "status-format[0]", "#[0]");
        let origin1 = format!("{} {} {}", cmd, "status-format[1]", "#[1]");
        let origin = format!("{} ; {}", origin0, origin1);
        let set_option = SetGlobalSessionOption::status_format(Some(vec![
            "#[0]".to_string(),
            "#[1]".to_string(),
        ]))
        .to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-interval", "2");
        let set_option = SetGlobalSessionOption::status_interval(Some(2)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-justify", "left");
        let set_option =
            SetGlobalSessionOption::status_justify(Some(StatusJustify::Left)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-keys", "emacs");
        let set_option = SetGlobalSessionOption::status_keys(Some(StatusKeys::Emacs)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-left", "#(whoami)");
        let set_option = SetGlobalSessionOption::status_left(Some("#(whoami)")).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_attr(Some("41"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_bg(Some("42"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_fg(Some("43"));
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-left-length", "50");
        let set_option = SetGlobalSessionOption::status_left_length(Some(50)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {} {}", cmd, "status-left-style", "default");
        let set_option = SetGlobalSessionOption::status_left_style(Some("default")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_7")]
    {
        let origin = format!("{} {} {}", cmd, "status-position", "bottom");
        let set_option =
            SetGlobalSessionOption::status_position(Some(StatusPosition::Bottom)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-right", "#[]");
        let set_option = SetGlobalSessionOption::status_right(Some("#[]")).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_attr(Some("47"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_bg(Some("48"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_fg(Some("49"));
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-right-length", "50");
        let set_option = SetGlobalSessionOption::status_right_length(Some(50)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {} {}", cmd, "status-right-style", "default");
        let set_option = SetGlobalSessionOption::status_right_style(Some("default")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {} {}", cmd, "status-style", "fg=colour247");
        let set_option = SetGlobalSessionOption::status_style(Some("fg=colour247")).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    //let options = options.status_utf8(Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    //let options = options.terminal_overrides(Some("53"));
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "assume-paste-time", "1");
        let set_option = SetGlobalSessionOption::assume_paste_time(Some(1)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin0 = format!("{} {} {}", cmd, "update-environment[0]", "DISPLAY");
        let origin1 = format!("{} {} {}", cmd, "update-environment[1]", "KRB5CCNAME");
        let origin = format!("{} ; {}", origin0, origin1);
        let set_option = SetGlobalSessionOption::update_environment(Some(vec![
            "DISPLAY".to_string(),
            "KRB5CCNAME".to_string(),
        ]))
        .to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "visual-activity", "off");
        let set_option = SetGlobalSessionOption::visual_activity(Some(Activity::Off)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "visual-bell", "off");
        let set_option = SetGlobalSessionOption::visual_bell(Some(Activity::Off)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    //v.push(format!("{} {} {}", cmd, "visual-content", "fg=colour247"));
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {} {}", cmd, "visual-silence", "off");
        let set_option = SetGlobalSessionOption::visual_silence(Some(Activity::Off)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_6")]
    {
        let origin = format!(
            "{} {} {}",
            cmd, "word-separators", "!\"#$%&'()*+,-./;<=>?@[\\]^`{|}~"
        );
        let set_option =
            SetGlobalSessionOption::word_separators(Some("!\"#$%&'()*+,-./;<=>?@[\\]^`{|}~"))
                .to_string();
        assert_eq!(origin, set_option);
    }
    {
        let origin = format!("{} {} {}", cmd, "@user-option-name", "value");
        let set_option =
            SetGlobalSessionOption::user_option("user-option-name", Some("value")).to_string();
        assert_eq!(origin, set_option);
    }
}
