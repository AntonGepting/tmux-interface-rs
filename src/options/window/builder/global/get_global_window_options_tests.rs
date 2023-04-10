#[test]
fn get_global_window_options() {
    use crate::{GetGlobalWindowOptions, GetUserOptions, GetWindowOptionsTr};

    let options = GetGlobalWindowOptions::new();
    let target = ":";

    #[cfg(feature = "tmux_1_0")]
    let options = options.aggressive_resize(Some(target));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    let options = options.allow_rename(Some(target));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    let options = options.alternate_screen(Some(target));
    #[cfg(feature = "tmux_1_0")] // 0.8
    let options = options.automatic_rename(Some(target));
    #[cfg(feature = "tmux_1_9")]
    let options = options.automatic_rename_format(Some(target));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    let options = options.c0_change_interval(Some(target));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    let options = options.c0_change_trigger(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.clock_mode_colour(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.clock_mode_style(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    let options = options.force_height(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    let options = options.force_width(Some(target));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    let options = options.layout_history_limit(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.main_pane_height(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.main_pane_width(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.mode_attr(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.mode_bg(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.mode_fg(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.mode_keys(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    let options = options.mode_mouse(Some(target));
    #[cfg(feature = "tmux_1_9")]
    let options = options.mode_style(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.monitor_activity(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let options = options.monitor_content(Some(target));
    #[cfg(feature = "tmux_2_6")]
    let options = options.monitor_bell(Some(target));
    #[cfg(feature = "tmux_1_4")]
    let options = options.monitor_silence(Some(target));
    #[cfg(feature = "tmux_1_4")]
    let options = options.other_pane_height(Some(target));
    #[cfg(feature = "tmux_1_4")]
    let options = options.other_pane_width(Some(target));
    #[cfg(feature = "tmux_2_0")]
    let options = options.pane_active_border_style(Some(target));
    #[cfg(feature = "tmux_1_6")]
    let options = options.pane_base_index(Some(target));
    #[cfg(feature = "tmux_2_3")]
    let options = options.pane_border_format(Some(target));
    #[cfg(feature = "tmux_2_3")]
    let options = options.pane_border_status(Some(target));
    #[cfg(feature = "tmux_2_0")]
    let options = options.pane_border_style(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    let options = options.remain_on_exit(Some(target));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    let options = options.synchronize_panes(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    let options = options.utf8(Some(target));
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    let options = options.window_active_style(Some(target));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_bell_attr(Some(target));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_bell_bg(Some(target));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_bell_fg(Some(target));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_content_attr(Some(target));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_content_bg(Some(target));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_content_fg(Some(target));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_activity_attr(Some(target));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_activity_bg(Some(target));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_activity_fg(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_attr(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_bg(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_fg(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_current_attr(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_current_bg(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_current_fg(Some(target));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    let options = options.window_status_alert_attr(Some(target));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    let options = options.window_status_alert_bg(Some(target));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    let options = options.window_status_alert_fg(Some(target));
    #[cfg(feature = "tmux_1_9")]
    let options = options.window_status_activity_style(Some(target));
    #[cfg(feature = "tmux_1_9")]
    let options = options.window_status_bell_style(Some(target));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.window_status_content_style(Some(target));
    #[cfg(feature = "tmux_1_2")]
    let options = options.window_status_current_format(Some(target));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    let options = options.window_status_last_attr(Some(target));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    let options = options.window_status_last_bg(Some(target));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    let options = options.window_status_last_fg(Some(target));
    #[cfg(feature = "tmux_1_9")]
    let options = options.window_status_current_style(Some(target));
    #[cfg(feature = "tmux_1_2")]
    let options = options.window_status_format(Some(target));
    #[cfg(feature = "tmux_1_9")]
    let options = options.window_status_last_style(Some(target));
    #[cfg(feature = "tmux_1_7")]
    let options = options.window_status_separator(Some(target));
    #[cfg(feature = "tmux_1_9")]
    let options = options.window_status_style(Some(target));
    #[cfg(feature = "tmux_2_9")]
    let options = options.window_size(Some(target));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    let options = options.word_separators(Some(target));
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    let options = options.window_style(Some(target));
    #[cfg(feature = "tmux_1_7")]
    let options = options.wrap_search(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.xterm_keys(Some(target));

    let options = options.user_option_ext(Some(target), "user-option-name");

    let options = options.options.to_string();

    let cmd = "show -g -w";
    let cmd = format!("{} -t {}", cmd, target);
    let separator = " ; ";

    let mut origin = Vec::new();
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "aggressive-resize"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    origin.push(format!("{} {}", cmd, "allow-rename"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    origin.push(format!("{} {}", cmd, "alternate-screen"));
    #[cfg(feature = "tmux_1_0")] // 0.8
    origin.push(format!("{} {}", cmd, "automatic-rename"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "automatic-rename-format"));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    origin.push(format!("{} {}", cmd, "c0-change-interval"));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    origin.push(format!("{} {}", cmd, "c0-change-trigger"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "clock-mode-colour"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "clock-mode-style"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    origin.push(format!("{} {}", cmd, "force-height"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    origin.push(format!("{} {}", cmd, "force-width"));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    origin.push(format!("{} {}", cmd, "layout-history-limit"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "main-pane-height"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "main-pane-width"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "mode-attr"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "mode-bg"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "mode-fg"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "mode-keys"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    origin.push(format!("{} {}", cmd, "mode-mouse"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "mode-style"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "monitor-activity"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    origin.push(format!("{} {}", cmd, "monitor-content"));
    #[cfg(feature = "tmux_2_6")]
    origin.push(format!("{} {}", cmd, "monitor-bell"));
    #[cfg(feature = "tmux_1_4")]
    origin.push(format!("{} {}", cmd, "monitor-silence"));
    #[cfg(feature = "tmux_1_4")]
    origin.push(format!("{} {}", cmd, "other-pane-height"));
    #[cfg(feature = "tmux_1_4")]
    origin.push(format!("{} {}", cmd, "other-pane-width"));
    #[cfg(feature = "tmux_2_0")]
    origin.push(format!("{} {}", cmd, "pane-active-border-style"));
    #[cfg(feature = "tmux_1_6")]
    origin.push(format!("{} {}", cmd, "pane-base-index"));
    #[cfg(feature = "tmux_2_3")]
    origin.push(format!("{} {}", cmd, "pane-border-format"));
    #[cfg(feature = "tmux_2_3")]
    origin.push(format!("{} {}", cmd, "pane-border-status"));
    #[cfg(feature = "tmux_2_0")]
    origin.push(format!("{} {}", cmd, "pane-border-style"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    origin.push(format!("{} {}", cmd, "remain-on-exit"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    origin.push(format!("{} {}", cmd, "synchronize-panes"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    origin.push(format!("{} {}", cmd, "utf8"));
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    origin.push(format!("{} {}", cmd, "window-active-style"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-bell-attr"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-bell-bg"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-bell-fg"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-content-attr"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-content-bg"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-content-fg"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-activity-attr"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-activity-bg"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-activity-fg"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-attr"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-bg"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-fg"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-current-attr"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-current-bg"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-current-fg"));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    origin.push(format!("{} {}", cmd, "window-status-alert-attr"));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    origin.push(format!("{} {}", cmd, "window-status-alert-bg"));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    origin.push(format!("{} {}", cmd, "window-status-alert-fg"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "window-status-activity-style"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "window-status-bell-style"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    origin.push(format!("{} {}", cmd, "window-status-content-style"));
    #[cfg(feature = "tmux_1_2")]
    origin.push(format!("{} {}", cmd, "window-status-current-format"));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-last-attr"));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-last-bg"));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "window-status-last-fg"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "window-status-current-style"));
    #[cfg(feature = "tmux_1_2")]
    origin.push(format!("{} {}", cmd, "window-status-format"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "window-status-last-style"));
    #[cfg(feature = "tmux_1_7")]
    origin.push(format!("{} {}", cmd, "window-status-separator"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "window-status-style"));
    #[cfg(feature = "tmux_2_9")]
    origin.push(format!("{} {}", cmd, "window-size"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    origin.push(format!("{} {}", cmd, "word-separators"));
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    origin.push(format!("{} {}", cmd, "window-style"));
    #[cfg(feature = "tmux_1_7")]
    origin.push(format!("{} {}", cmd, "wrap-search"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "xterm-keys"));

    origin.push(format!("{} {}", cmd, "@user-option-name"));

    let origin = origin.join(separator);

    assert_eq!(options, origin);
}
