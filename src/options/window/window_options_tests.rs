// Tests:
// * `::default()`
// * `.to_string()`
// * `::from_str()`

#[test]
fn default() {
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    use crate::ModeMouse;
    #[cfg(feature = "tmux_2_3")]
    use crate::PaneBorderStatus;
    #[cfg(feature = "tmux_2_9")]
    use crate::WindowSize;
    use crate::{ClockModeStyle, StatusKeys, Switch, WindowOptions};

    let default_window_options = WindowOptions::new();
    let window_options = WindowOptions::default();

    #[cfg(feature = "tmux_1_0")]
    let window_options = window_options.aggressive_resize(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    let window_options = window_options.allow_rename(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    let window_options = window_options.alternate_screen(Some(Switch::On));
    #[cfg(feature = "tmux_1_0")] // 0.8
    let window_options = window_options.automatic_rename(Some(Switch::On));
    #[cfg(feature = "tmux_1_9")]
    let window_options = window_options.automatic_rename_format(Some(
        "#{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}",
    ));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    let window_options = window_options.c0_change_interval(Some(""));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    let window_options = window_options.c0_change_trigger(Some(""));
    #[cfg(feature = "tmux_1_0")]
    let window_options = window_options.clock_mode_colour(Some("blue"));
    #[cfg(feature = "tmux_1_0")]
    let window_options = window_options.clock_mode_style(Some(ClockModeStyle::_24));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    let window_options = window_options.force_height(Some(0));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    let window_options = window_options.force_width(Some(0));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    let window_options = window_options.layout_history_limit(Some());
    #[cfg(feature = "tmux_1_0")]
    let window_options = window_options.main_pane_height(Some(24));
    #[cfg(feature = "tmux_1_0")]
    let window_options = window_options.main_pane_width(Some(80));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.mode_attr(Some());
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.mode_bg(Some());
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.mode_fg(Some());
    #[cfg(feature = "tmux_1_0")]
    let window_options = window_options.mode_keys(Some(StatusKeys::Vi));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    let window_options = window_options.mode_mouse(Some(ModeMouse::Off));
    #[cfg(feature = "tmux_1_9")]
    let window_options = window_options.mode_style(Some("fg=black,bg=yellow"));
    #[cfg(feature = "tmux_1_0")]
    let window_options = window_options.monitor_activity(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let window_options = window_options.monitor_content(Some());
    #[cfg(feature = "tmux_2_6")]
    let window_options = window_options.monitor_bell(Some(Switch::On));
    #[cfg(feature = "tmux_1_4")]
    let window_options = window_options.monitor_silence(Some(0));
    #[cfg(feature = "tmux_1_4")]
    let window_options = window_options.other_pane_height(Some(0));
    #[cfg(feature = "tmux_1_4")]
    let window_options = window_options.other_pane_width(Some(0));
    #[cfg(feature = "tmux_2_0")]
    let window_options = window_options.pane_active_border_style(Some("fg=green"));
    #[cfg(feature = "tmux_1_6")]
    let window_options = window_options.pane_base_index(Some(0));
    #[cfg(feature = "tmux_2_3")]
    let window_options = window_options.pane_border_format(Some(
        "#{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"",
    ));
    #[cfg(feature = "tmux_2_3")]
    let window_options = window_options.pane_border_status(Some(PaneBorderStatus::Off));
    #[cfg(feature = "tmux_2_0")]
    let window_options = window_options.pane_border_style(Some("fg=colour238,bg=colour235"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    let window_options = window_options.remain_on_exit(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    let window_options = window_options.synchronize_panes(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    let window_options = window_options.utf8(Some(Switch::Off));
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    let window_options = window_options.window_active_style(Some("fg=colour253,bg=colour235"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_bell_attr(Some());
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_bell_bg(Some());
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_bell_fg(Some());
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_content_attr(Some());
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_content_bg(Some());
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_content_fg(Some());
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_activity_attr(Some());
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_activity_bg(Some());
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_activity_fg(Some());
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_attr(Some());
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_bg(Some());
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_fg(Some());
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_current_attr(Some());
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_current_bg(Some());
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_current_fg(Some());
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    let window_options = window_options.window_status_alert_attr(Some());
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    let window_options = window_options.window_status_alert_bg(Some());
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    let window_options = window_options.window_status_alert_fg(Some());
    #[cfg(feature = "tmux_1_9")]
    let window_options = window_options.window_status_activity_style(Some("reverse"));
    #[cfg(feature = "tmux_1_9")]
    let window_options =
        window_options.window_status_bell_style(Some("fg=colour253,bg=colour1,bright"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let window_options = window_options.window_status_content_style(Some());
    #[cfg(feature = "tmux_1_2")]
    let window_options = window_options
        .window_status_current_format(Some("#I:#W#{?window_flags,#{window_flags}, }"));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_last_attr(Some());
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_last_bg(Some());
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_last_fg(Some());
    #[cfg(feature = "tmux_1_9")]
    let window_options =
        window_options.window_status_current_style(Some("fg=colour22,bg=colour114"));
    #[cfg(feature = "tmux_1_2")]
    let window_options =
        window_options.window_status_format(Some("#I:#W#{?window_flags,#{window_flags}, }"));
    #[cfg(feature = "tmux_1_9")]
    let window_options = window_options.window_status_last_style(Some("default"));
    #[cfg(feature = "tmux_1_7")]
    let window_options = window_options.window_status_separator(Some("\" \""));
    #[cfg(feature = "tmux_1_9")]
    let window_options = window_options.window_status_style(Some("default"));
    #[cfg(feature = "tmux_2_9")]
    let window_options = window_options.window_size(Some(WindowSize::Largest));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    let window_options = window_options.word_separators(Some());
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    let window_options = window_options.window_style(Some("default"));
    #[cfg(feature = "tmux_1_7")]
    let window_options = window_options.wrap_search(Some(Switch::On));
    #[cfg(feature = "tmux_1_0")]
    let window_options = window_options.xterm_keys(Some(Switch::On));

    assert_eq!(default_window_options, window_options);
}

#[test]
fn to_string() {
    use crate::WindowOptions;

    let window_options = WindowOptions::new().to_string();

    let mut v = Vec::new();

    #[cfg(feature = "tmux_1_0")]
    v.push("aggressive-resize off");
    // allow-passthrough off
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    v.push("allow-rename off");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    v.push("alternate-screen on");
    #[cfg(feature = "tmux_1_0")] // 0.8
    v.push("automatic-rename on");
    #[cfg(feature = "tmux_1_9")]
    v.push("automatic-rename-format #{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}");
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    v.push("c0-change_interval");
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    v.push("c0-change-trigger");
    #[cfg(feature = "tmux_1_0")]
    v.push("clock-mode-colour blue");
    #[cfg(feature = "tmux_1_0")]
    v.push("clock-mode-style 24");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    v.push("force-height 0");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    v.push("force-width 0");
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    v.push("layout-history-limit 0");
    #[cfg(feature = "tmux_1_0")]
    v.push("main-pane-height 24");
    #[cfg(feature = "tmux_1_0")]
    v.push("main-pane-width 80");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("mode-attr 0");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("mode-bg 0");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("mode-fg 0");
    #[cfg(feature = "tmux_1_0")]
    v.push("mode-keys vi");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    v.push("mode-mouse 0");
    #[cfg(feature = "tmux_1_9")]
    v.push("mode-style fg=black,bg=yellow");
    #[cfg(feature = "tmux_1_0")]
    v.push("monitor-activity off");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    v.push("monitor-content 0");
    #[cfg(feature = "tmux_2_6")]
    v.push("monitor-bell on");
    #[cfg(feature = "tmux_1_4")]
    v.push("monitor-silence 0");
    #[cfg(feature = "tmux_1_4")]
    v.push("other-pane-height 0");
    #[cfg(feature = "tmux_1_4")]
    v.push("other-pane-width 0");
    #[cfg(feature = "tmux_2_0")]
    v.push("pane-active-border-style fg=green");
    #[cfg(feature = "tmux_1_6")]
    v.push("pane-base-index 0");
    #[cfg(feature = "tmux_2_3")]
    v.push(
        "pane-border-format #{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"",
    );
    #[cfg(feature = "tmux_2_3")]
    v.push("pane-border-status off");
    #[cfg(feature = "tmux_2_0")]
    v.push("pane-border-style fg=colour238,bg=colour235");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    v.push("remain-on-exit off");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    v.push("synchronize-panes off");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    v.push("utf8 _");
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    v.push("window-active-style default");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-bell-attr _");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-bell-bg _");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-bell-fg _");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-content-attr _");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-content-bg _");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-content-fg _");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-activity-attr _");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-activity-bg _");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-activity-fg _");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("window-status-attr _");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("window-status-bg _");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("window-status-fg _");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("window-status-current-attr _");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("window-status-current-bg _");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("window-status-current-fg _");
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    v.push("window-status-alert-attr _");
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    v.push("window-status-alert-bg _");
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    v.push("window-status-alert-fg _");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-activity-style reverse");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-bell-style fg=colour253,bg=colour1,bright");
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    v.push("window-status-content-style _");
    #[cfg(feature = "tmux_1_2")]
    v.push("window-status-current-format #I:#W#{?window_flags,#{window_flags}, }");
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    v.push("window-status-last-attr _");
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    v.push("window-status-last-bg _");
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    v.push("window-status-last-fg _");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-current-style fg=colour22,bg=colour114");
    #[cfg(feature = "tmux_1_2")]
    v.push("window-status-format #I:#W#{?window_flags,#{window_flags}, }");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-last-style default");
    #[cfg(feature = "tmux_1_7")]
    v.push("window-status-separator \" \"");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-style default");
    #[cfg(feature = "tmux_2_9")]
    v.push("window-size largest");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    v.push("word-separators _");
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    v.push("window-style _");
    #[cfg(feature = "tmux_1_7")]
    v.push("wrap-search on");
    #[cfg(feature = "tmux_1_0")]
    v.push("xterm-keys on");

    let origin = v.join("\n");

    assert_eq!(origin, window_options);
}

#[test]
fn from_str() {
    use crate::WindowOptions;

    let mut v = Vec::new();
    #[cfg(feature = "tmux_1_0")]
    v.push("aggressive-resize off");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    v.push("allow-rename off");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    v.push("alternate-screen on");
    #[cfg(feature = "tmux_1_0")]
    v.push("automatic-rename on");
    #[cfg(feature = "tmux_1_9")]
    v.push("automatic-rename-format #{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}");
    // #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    // v.push("c0-change-interval "); // default?
    // #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    // v.push("c0-change-trigger "); // default?
    #[cfg(feature = "tmux_1_0")]
    v.push("clock-mode-colour blue");
    #[cfg(feature = "tmux_1_0")]
    v.push("clock-mode-style 24");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    v.push("force-height 0");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    v.push("force-width 0");
    // #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    // v.push("layout-history-limit "); // default?
    #[cfg(feature = "tmux_1_0")]
    v.push("main-pane-height 24");
    #[cfg(feature = "tmux_1_0")]
    v.push("main-pane-width 80");
    // #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    // v.push("mode-attr "); // default?
    // #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    // v.push("mode-bg "); // default?
    // #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    // v.push("mode-fg "); // default?
    #[cfg(feature = "tmux_1_0")]
    v.push("mode-keys vi");
    // #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    // v.push("mode-mouse "); // default?
    #[cfg(feature = "tmux_1_9")]
    v.push("mode-style fg=black,bg=yellow");
    #[cfg(feature = "tmux_1_0")]
    v.push("monitor-activity off");
    // #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    // v.push("monitor-content "); // default?
    #[cfg(feature = "tmux_2_6")]
    v.push("monitor-bell on");
    #[cfg(feature = "tmux_1_4")]
    v.push("monitor-silence 0");
    #[cfg(feature = "tmux_1_4")]
    v.push("other-pane-height 0");
    #[cfg(feature = "tmux_1_4")]
    v.push("other-pane-width 0");
    #[cfg(feature = "tmux_2_0")]
    v.push("pane-active-border-style fg=green");
    #[cfg(feature = "tmux_1_6")]
    v.push("pane-base-index 0");
    #[cfg(feature = "tmux_2_3")]
    v.push(
        "pane-border-format #{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"",
    );
    #[cfg(feature = "tmux_2_3")]
    v.push("pane-border-status off");
    #[cfg(feature = "tmux_2_0")]
    v.push("pane-border-style fg=colour238,bg=colour235");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    v.push("remain-on-exit off");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    v.push("synchronize-panes off");
    // #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    // v.push("utf8 "); // default?
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    v.push("window-active-style fg=colour253,bg=colour235");
    // #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    // v.push("window-status-bell-attr "); // default?
    // #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    // v.push("window-status-bell-bg "); // default?
    // #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    // v.push("window-status-bell-fg "); // default?
    // #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    // v.push("window-status-content-attr "); // default?
    // #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    // v.push("window-status-content-bg "); // default?
    // #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    // v.push("window-status-content-fg "); // default?
    // #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    // v.push("window-status-activity-attr "); // default?
    // #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    // v.push("window-status-activity-bg "); // default?
    // #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    // v.push("window-status-activity-fg "); // default?
    // #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    // v.push("window-status-attr "); // default?
    // #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    // v.push("window-status-bg "); // default?
    // #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    // v.push("window-status-fg "); // default?
    // #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    // v.push("window-status-alert-attr "); // default?
    // #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    // v.push("window-status-alert-bg "); // default?
    // #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    // v.push("window-status-alert-fg "); // default?
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-activity-style reverse");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-bell-style fg=colour253,bg=colour1,bright");
    // #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    // v.push("window-status-content-style "); // default?
    #[cfg(feature = "tmux_1_2")]
    v.push("window-status-current-format #I:#W#{?window_flags,#{window_flags}, }");
    // #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    // v.push("window-status-last-attr"); // default?
    // #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    // v.push("window-status-last-bg"); // default?
    // #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    // v.push("window-status-last-fg"); // default?
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-current-style fg=colour22,bg=colour114");
    #[cfg(feature = "tmux_1_2")]
    v.push("window-status-format #I:#W#{?window_flags,#{window_flags}, }");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-last-style default");
    #[cfg(feature = "tmux_1_7")]
    v.push("window-status-separator \" \"");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-style default");
    #[cfg(feature = "tmux_2_9")]
    v.push("window-size largest"); // default?
                                   // #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
                                   // v.push("word-separators"); // default?
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    v.push("window-style default");
    #[cfg(feature = "tmux_1_7")]
    v.push("wrap-search on");
    #[cfg(feature = "tmux_1_0")]
    v.push("xterm-keys on");
    // #[cfg(feature = "tmux_1_0")]
    // v.push("user-keys");

    let window_options_str = v.join("\n");
    let window_options = window_options_str.parse::<WindowOptions>().unwrap();

    let default_window_options = WindowOptions::new();

    assert_eq!(window_options, default_window_options);
}
