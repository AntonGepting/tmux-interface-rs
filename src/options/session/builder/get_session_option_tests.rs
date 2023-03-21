#[test]
fn get_local_session_option_tests() {
    use crate::{GetLocalSessionOption, GetSessionOption, GetUserOption};

    let cmd = "show";
    let target = Some(":");

    #[cfg(feature = "tmux_2_6")]
    {
        let origin = format!("{} {}", cmd, "activity-action");
        let set_option = GetLocalSessionOption::activity_action(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_8")]
    {
        let origin = format!("{} {}", cmd, "assume-paste-time");
        let set_option = GetLocalSessionOption::assume_paste_time(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "base-index");
        let set_option = GetLocalSessionOption::base_index(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "bell-action");
        let set_option = GetLocalSessionOption::bell_action(target).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    //let options = options.bell_on_alert();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    //let options = options.buffer_limit();
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "default-command");
        let set_option = GetLocalSessionOption::default_command(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "default-shell");
        let set_option = GetLocalSessionOption::default_shell(target).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.default_path();
    #[cfg(feature = "tmux_2_9")]
    {
        let origin = format!("{} {}", cmd, "default-size");
        let set_option = GetLocalSessionOption::default_size(target).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    //let options = options.default_terminal();
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {}", cmd, "destroy-unattached");
        let set_option = GetLocalSessionOption::destroy_unattached(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {}", cmd, "detach-on-destroy");
        let set_option = GetLocalSessionOption::detach_on_destroy(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_2")]
    {
        let origin = format!("{} {}", cmd, "display-panes-active-colour");
        let set_option = GetLocalSessionOption::display_panes_active_colour(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "display-panes-colour");
        let set_option = GetLocalSessionOption::display_panes_colour(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "display-panes-time");
        let set_option = GetLocalSessionOption::display_panes_time(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "display-time");
        let set_option = GetLocalSessionOption::display_time(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "history-limit");
        let set_option = GetLocalSessionOption::history_limit(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_2")]
    {
        let origin = format!("{} {}", cmd, "key-table");
        let set_option = GetLocalSessionOption::key_table(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "lock-after-time");
        let set_option = GetLocalSessionOption::lock_after_time(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_1")]
    {
        let origin = format!("{} {}", cmd, "lock-command");
        let set_option = GetLocalSessionOption::lock_command(target).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    //let options = options.lock_server();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_attr();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_bg();
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_attr();
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_bg();
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_fg();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_fg();
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "message-command-style");
        let set_option = GetLocalSessionOption::message_command_style(target).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    //let options = options.message_limit();
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "message-style");
        let set_option = GetLocalSessionOption::message_style(target).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_resize_pane();
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_select_pane();
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_select_window();
    #[cfg(feature = "tmux_2_1")]
    {
        let origin = format!("{} {}", cmd, "mouse");
        let set_option = GetLocalSessionOption::mouse(target).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    //let options = options.mouse_utf8();
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_active_border_bg();
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_active_border_fg();
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_border_bg();
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_border_fg();
    //#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    //let options = options.pane_active_border_style();
    //#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    //let options = options.pane_border_style();
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "prefix");
        let set_option = GetLocalSessionOption::prefix(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_6")]
    {
        let origin = format!("{} {}", cmd, "prefix2");
        let set_option = GetLocalSessionOption::prefix2(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_7")]
    {
        let origin = format!("{} {}", cmd, "renumber-windows");
        let set_option = GetLocalSessionOption::renumber_windows(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "repeat-time");
        let set_option = GetLocalSessionOption::repeat_time(target).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    //let options = options.set_remain_on_exit();
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "set-titles");
        let set_option = GetLocalSessionOption::set_titles(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "set-titles-string");
        let set_option = GetLocalSessionOption::set_titles_string(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_6")]
    {
        let origin = format!("{} {}", cmd, "silence-action");
        let set_option = GetLocalSessionOption::silence_action(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "status");
        let set_option = GetLocalSessionOption::status(target).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_attr();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "status-bg");
        let set_option = GetLocalSessionOption::status_bg(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "status-fg");
        let set_option = GetLocalSessionOption::status_fg(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_9")]
    {
        let origin = format!("{} {}", cmd, "assume-paste-time");
        let set_option = GetLocalSessionOption::assume_paste_time(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_9")]
    {
        let origin = format!("{} {}", cmd, "status-format");
        let set_option = GetLocalSessionOption::status_format(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "status-interval");
        let set_option = GetLocalSessionOption::status_interval(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "status-justify");
        let set_option = GetLocalSessionOption::status_justify(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "status-keys");
        let set_option = GetLocalSessionOption::status_keys(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "status-left");
        let set_option = GetLocalSessionOption::status_left(target).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_attr();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_bg();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_fg();
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "status-left-length");
        let set_option = GetLocalSessionOption::status_left_length(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "status-left-style");
        let set_option = GetLocalSessionOption::status_left_style(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_7")]
    {
        let origin = format!("{} {}", cmd, "status-position");
        let set_option = GetLocalSessionOption::status_position(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "status-right");
        let set_option = GetLocalSessionOption::status_right(target).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_attr();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_bg();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_fg();
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "status-right-length");
        let set_option = GetLocalSessionOption::status_right_length(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "status-right-style");
        let set_option = GetLocalSessionOption::status_right_style(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "status-style");
        let set_option = GetLocalSessionOption::status_style(target).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    //let options = options.status_utf8();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    //let options = options.terminal_overrides();
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "assume-paste-time");
        let set_option = GetLocalSessionOption::assume_paste_time(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "update-environment");
        let set_option = GetLocalSessionOption::update_environment(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "visual-activity");
        let set_option = GetLocalSessionOption::visual_activity(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "visual-bell");
        let set_option = GetLocalSessionOption::visual_bell(target).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    //v.push(format!("{} {}", cmd, "visual-content", "fg=colour247"));
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {}", cmd, "visual-silence");
        let set_option = GetLocalSessionOption::visual_silence(target).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_6")]
    {
        let origin = format!("{} {}", cmd, "word-separators");
        let set_option = GetLocalSessionOption::word_separators(target).to_string();
        assert_eq!(origin, set_option);
    }
    {
        let origin = format!("{} {}", cmd, "@user-option-name");
        let set_option = GetLocalSessionOption::user_option(target, "user-option-name").to_string();
        assert_eq!(origin, set_option);
    }
}
