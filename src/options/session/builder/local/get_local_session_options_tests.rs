#[test]
fn get_local_session_options() {
    use crate::{GetLocalSessionOptions, GetSessionOptionsTr, GetUserOptions};

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-options";
    #[cfg(feature = "cmd_alias")]
    let cmd = "show";

    let target = ":";
    let separator = " ; ";
    let cmd = format!("{} -t {}", cmd, target);

    let mut origin = Vec::new();
    #[cfg(feature = "tmux_2_6")]
    origin.push(format!("{} {}", cmd, "activity-action"));
    #[cfg(feature = "tmux_1_8")]
    origin.push(format!("{} {}", cmd, "assume-paste-time"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "base-index"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "bell-action"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    origin.push(format!("{} {}", cmd, "bell-on-alert"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    origin.push(format!("{} {}", cmd, "buffer-limit"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "default-command"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "default-shell"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "default-path"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    origin.push(format!("{} {}", cmd, "default-terminal"));
    #[cfg(feature = "tmux_2_9")]
    origin.push(format!("{} {}", cmd, "default-size"));
    #[cfg(feature = "tmux_1_5")]
    origin.push(format!("{} {}", cmd, "destroy-unattached"));
    #[cfg(feature = "tmux_1_4")]
    origin.push(format!("{} {}", cmd, "detach-on-destroy"));
    #[cfg(feature = "tmux_1_2")]
    origin.push(format!("{} {}", cmd, "display-panes-active-colour"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "display-panes-colour"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "display-panes-time"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "display-time"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "history-limit"));
    #[cfg(feature = "tmux_2_2")]
    origin.push(format!("{} {}", cmd, "key-table"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "lock-after-time"));
    #[cfg(feature = "tmux_1_1")]
    origin.push(format!("{} {}", cmd, "lock-command"));
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    origin.push(format!("{} {}", cmd, "lock-server"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "message-attr"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "message-bg"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "message-command-attr"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "message-command-bg"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "message-command-fg"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "message-fg"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "message-command-style"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    origin.push(format!("{} {}", cmd, "message-limit"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "message-style"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    origin.push(format!("{} {}", cmd, "mouse-resize-pane"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    origin.push(format!("{} {}", cmd, "mouse-select-pane"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    origin.push(format!("{} {}", cmd, "mouse-select-window"));
    #[cfg(feature = "tmux_2_1")]
    origin.push(format!("{} {}", cmd, "mouse"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    origin.push(format!("{} {}", cmd, "mouse-utf8"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "pane-active-border-bg"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "pane-active-border-fg"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "pane-border-bg"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "pane-border-fg"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    origin.push(format!("{} {}", cmd, "pane-active-border-style"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    origin.push(format!("{} {}", cmd, "pane-border-style"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "prefix"));
    #[cfg(feature = "tmux_1_6")]
    origin.push(format!("{} {}", cmd, "prefix2"));
    #[cfg(feature = "tmux_1_7")]
    origin.push(format!("{} {}", cmd, "renumber-windows"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "repeat-time"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    origin.push(format!("{} {}", cmd, "set-remain-on-exit"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "set-titles"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "set-titles-string"));
    #[cfg(feature = "tmux_2_6")]
    origin.push(format!("{} {}", cmd, "silence-action"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "status"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-attr"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-bg"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-fg"));
    #[cfg(feature = "tmux_2_9")]
    origin.push(format!("{} {}", cmd, "status-format"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "status-interval"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "status-justify"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "status-keys"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "status-left"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-left-attr"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-left-bg"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-left-fg"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "status-left-length"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "status-left-style"));
    #[cfg(feature = "tmux_1_7")]
    origin.push(format!("{} {}", cmd, "status-position"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "status-right"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-right-attr"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-right-bg"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-right-fg"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "status-right-length"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "status-right-style"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "status-style"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    origin.push(format!("{} {}", cmd, "status-utf8"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    origin.push(format!("{} {}", cmd, "terminal-overrides"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "update-environment"));
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    origin.push(format!("{} {}", cmd, "user-keys"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "visual-activity"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "visual-bell"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    origin.push(format!("{} {}", cmd, "visual-content"));
    #[cfg(feature = "tmux_1_4")]
    origin.push(format!("{} {}", cmd, "visual-silence"));
    #[cfg(feature = "tmux_1_6")]
    origin.push(format!("{} {}", cmd, "word-separators"));
    origin.push(format!("{} {}", cmd, "@user-option-name"));

    let origin = origin.join(separator);

    let options = GetLocalSessionOptions::new();
    #[cfg(feature = "tmux_2_6")]
    let options = options.activity_action(Some(target));
    #[cfg(feature = "tmux_1_8")]
    let options = options.assume_paste_time(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.base_index(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.bell_action(Some(target));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    let options = options.bell_on_alert(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    let options = options.buffer_limit(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.default_command(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.default_shell(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.default_path(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    let options = options.default_terminal(Some(target));
    #[cfg(feature = "tmux_2_9")]
    let options = options.default_size(Some(target));
    #[cfg(feature = "tmux_1_5")]
    let options = options.destroy_unattached(Some(target));
    #[cfg(feature = "tmux_1_4")]
    let options = options.detach_on_destroy(Some(target));
    #[cfg(feature = "tmux_1_2")]
    let options = options.display_panes_active_colour(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_panes_colour(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_panes_time(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_time(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.history_limit(Some(target));
    #[cfg(feature = "tmux_2_2")]
    let options = options.key_table(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.lock_after_time(Some(target));
    #[cfg(feature = "tmux_1_1")]
    let options = options.lock_command(Some(target));
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    let options = options.lock_server(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_attr(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_bg(Some(target));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_attr(Some(target));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_bg(Some(target));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_fg(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_fg(Some(target));
    #[cfg(feature = "tmux_1_9")]
    let options = options.message_command_style(Some(target));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    let options = options.message_limit(Some(target));
    #[cfg(feature = "tmux_1_9")]
    let options = options.message_style(Some(target));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_resize_pane(Some(target));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_select_pane(Some(target));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_select_window(Some(target));
    #[cfg(feature = "tmux_2_1")]
    let options = options.mouse(Some(target));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    let options = options.mouse_utf8(Some(target));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_active_border_bg(Some(target));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_active_border_fg(Some(target));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_border_bg(Some(target));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_border_fg(Some(target));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.pane_active_border_style(Some(target));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.pane_border_style(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.prefix(Some(target));
    #[cfg(feature = "tmux_1_6")]
    let options = options.prefix2(Some(target));
    #[cfg(feature = "tmux_1_7")]
    let options = options.renumber_windows(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.repeat_time(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    let options = options.set_remain_on_exit(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.set_titles(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.set_titles_string(Some(target));
    #[cfg(feature = "tmux_2_6")]
    let options = options.silence_action(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_attr(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_bg(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_fg(Some(target));
    #[cfg(feature = "tmux_2_9")]
    let options = options.status_format(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_interval(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_justify(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_keys(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_left(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_attr(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_bg(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_fg(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_left_length(Some(target));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_left_style(Some(target));
    #[cfg(feature = "tmux_1_7")]
    let options = options.status_position(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_right(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_attr(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_bg(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_fg(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_right_length(Some(target));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_right_style(Some(target));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_style(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    let options = options.status_utf8(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let options = options.terminal_overrides(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.update_environment(Some(target));
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    let options = options.user_keys(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.visual_activity(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.visual_bell(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let options = options.visual_content(Some(target));
    #[cfg(feature = "tmux_1_4")]
    let options = options.visual_silence(Some(target));
    #[cfg(feature = "tmux_1_6")]
    let options = options.word_separators(Some(target));
    let options = options.user_option_ext(Some(target), "user-option-name");

    let options = options.options.to_string();

    assert_eq!(options, origin);
}
