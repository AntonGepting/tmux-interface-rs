#[test]
fn get_local_window_option_value_tests() {
    use crate::{GetLocalWindowOptionValue, GetUserOption, GetWindowOption};

    let cmd = "show -v -w";
    let target = ":";
    let cmd = format!("{} -t {}", cmd, target);

    // ### Manual
    //
    // tmux ^1.0:
    // ```text
    // aggressive-resize [on | off]
    // ```
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "aggressive-resize");
        let set_option = GetLocalWindowOptionValue::aggressive_resize(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.6 v3.0:
    // ```text
    // allow-rename [on | off]
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    {
        let origin = format!("{} {}", cmd, "allow-rename");
        let set_option = GetLocalWindowOptionValue::allow_rename(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.2 v3.0:
    // ```text
    // alternate-screen [on | off]
    // ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    {
        let origin = format!("{} {}", cmd, "alternate-screen");
        let set_option = GetLocalWindowOptionValue::alternate_screen(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0:
    // ```text
    // automatic-rename [on | off]
    // ```
    #[cfg(feature = "tmux_1_0")] // 0.8
    {
        let origin = format!("{} {}", cmd, "automatic-rename");
        let set_option = GetLocalWindowOptionValue::automatic_rename(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.9:
    // ```text
    // automatic-rename-format format
    // ```
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "automatic-rename-format");
        let set_option =
            GetLocalWindowOptionValue::automatic_rename_format(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.7 v2.1:
    // ```text
    // c0-change-interval interval
    // ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    {
        let origin = format!("{} {}", cmd, "c0-change-interval");
        let set_option = GetLocalWindowOptionValue::c0_change_interval(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.7 v2.1:
    // ```text
    // c0-change-trigger trigger
    // ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    {
        let origin = format!("{} {}", cmd, "c0-change-trigger");
        let set_option = GetLocalWindowOptionValue::c0_change_trigger(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0:
    // ```text
    // clock-mode-colour colour
    // ```
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "clock-mode-colour");
        let set_option = GetLocalWindowOptionValue::clock_mode_colour(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0:
    // ```text
    // clock-mode-style [12 | 24]
    // ```
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "clock-mode-style");
        let set_option = GetLocalWindowOptionValue::clock_mode_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0 v2.9:
    // ```text
    // force-height height
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    {
        let origin = format!("{} {}", cmd, "force-height");
        let set_option = GetLocalWindowOptionValue::force_height(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0 v2.9:
    // ```text
    // force-width width
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    {
        let origin = format!("{} {}", cmd, "force-width");
        let set_option = GetLocalWindowOptionValue::force_width(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.7 v1.8:
    // ```text
    // layout-history-limit limit
    // ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    {
        let origin = format!("{} {}", cmd, "layout-history-limit");
        let set_option = GetLocalWindowOptionValue::layout_history_limit(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0:
    // ```text
    // main-pane-height height
    // ```
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "main-pane-height");
        let set_option = GetLocalWindowOptionValue::main_pane_height(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0:
    // ```text
    // main-pane-width width
    // ```
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "main-pane-width");
        let set_option = GetLocalWindowOptionValue::main_pane_width(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0 v1.9:
    // ```text
    // mode-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "mode-attr");
        let set_option = GetLocalWindowOptionValue::mode_attr(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0 v1.9:
    // ```text
    // mode-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "mode-bg");
        let set_option = GetLocalWindowOptionValue::mode_bg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0 v1.9:
    // ```text
    // mode-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "mode-fg");
        let set_option = GetLocalWindowOptionValue::mode_fg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0:
    // ```text
    // mode-keys [vi | emacs]
    // ```
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "mode-keys");
        let set_option = GetLocalWindowOptionValue::mode_keys(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0 v2.1:
    // ```text
    // mode-mouse [on | off]
    // ```
    //
    // tmux 1.6:
    // ```text
    // mode-mouse [on | off | copy-mode]
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    {
        let origin = format!("{} {}", cmd, "mode_mouse");
        let set_option = GetLocalWindowOptionValue::mode_mouse(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.9:
    // ```text
    // mode-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "mode-style");
        let set_option = GetLocalWindowOptionValue::mode_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0:
    // ```text
    // monitor-activity [on | off]
    // ```
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "monitor-activity");
        let set_option = GetLocalWindowOptionValue::monitor_activity(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0 v2.0:
    // ```text
    // monitor-content match-string
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    {
        let origin = format!("{} {}", cmd, "monitor-content");
        let set_option = GetLocalWindowOptionValue::monitor_content(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^2.6:
    // ```text
    // monitor-bell [on | off]
    // ```
    #[cfg(feature = "tmux_2_6")]
    {
        let origin = format!("{} {}", cmd, "monitor-bell");
        let set_option = GetLocalWindowOptionValue::monitor_bell(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.4:
    // ```text
    // monitor-silence [interval]
    // ```
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {}", cmd, "monitor-silence");
        let set_option = GetLocalWindowOptionValue::monitor_silence(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.4:
    // ```text
    // other-pane-height height
    // ```
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {}", cmd, "other-pane-height");
        let set_option = GetLocalWindowOptionValue::other_pane_height(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.4:
    // ```text
    // other-pane-width width
    // ```
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {}", cmd, "other-pane-width");
        let set_option = GetLocalWindowOptionValue::other_pane_width(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^2.0:
    // ```text
    // pane-active-border-style style
    // ```
    #[cfg(feature = "tmux_2_0")]
    {
        let origin = format!("{} {}", cmd, "pane-active-border-style");
        let set_option =
            GetLocalWindowOptionValue::pane_active_border_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.6:
    // ```text
    // pane-base-index index
    // ```
    #[cfg(feature = "tmux_1_6")]
    {
        let origin = format!("{} {}", cmd, "pane-base-index");
        let set_option = GetLocalWindowOptionValue::pane_base_index(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^2.3:
    // ```text
    // pane-border-format format
    // ```
    #[cfg(feature = "tmux_2_3")]
    {
        let origin = format!("{} {}", cmd, "pane-border-format");
        let set_option = GetLocalWindowOptionValue::pane_border_format(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^2.3:
    // ```text
    // pane-border-status [off | top | bottom]
    // ```
    #[cfg(feature = "tmux_2_3")]
    {
        let origin = format!("{} {}", cmd, "pane-border-status");
        let set_option = GetLocalWindowOptionValue::pane_border_status(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^2.0:
    // ```text
    // pane-border-style style
    // ```
    #[cfg(feature = "tmux_2_0")]
    {
        let origin = format!("{} {}", cmd, "pane-border-style");
        let set_option = GetLocalWindowOptionValue::pane_border_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0 v3.0:
    // ```text
    // remain-on-exit [on | off]
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    {
        let origin = format!("{} {}", cmd, "remain-on-exit");
        let set_option = GetLocalWindowOptionValue::remain_on_exit(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.2 v3.2:
    // ```text
    // synchronize-panes [on | off]
    // ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    {
        let origin = format!("{} {}", cmd, "synchronize-panes");
        let set_option = GetLocalWindowOptionValue::synchronize_panes(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0 v2.2:
    // ```text
    // utf8 [on | off]
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    {
        let origin = format!("{} {}", cmd, "utf8");
        let set_option = GetLocalWindowOptionValue::utf8(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^2.1 v3.0:
    // ```text
    // window-active-style style
    // ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    {
        let origin = format!("{} {}", cmd, "window-active-style");
        let set_option = GetLocalWindowOptionValue::window_active_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-bell-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-bell-attr");
        let set_option =
            GetLocalWindowOptionValue::window_status_bell_attr(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-bell-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-bell-bg");
        let set_option = GetLocalWindowOptionValue::window_status_bell_bg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-bell-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-bell-fg");
        let set_option = GetLocalWindowOptionValue::window_status_bell_fg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-content-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-content-attr");
        let set_option =
            GetLocalWindowOptionValue::window_status_content_attr(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-content-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-content-bg");
        let set_option =
            GetLocalWindowOptionValue::window_status_content_bg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-content-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-content-fg");
        let set_option =
            GetLocalWindowOptionValue::window_status_content_fg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-activity-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-activity-attr");
        let set_option =
            GetLocalWindowOptionValue::window_status_activity_attr(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-activity-bg attributes
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-activity-bg");
        let set_option =
            GetLocalWindowOptionValue::window_status_activity_bg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.6 v1.9:
    // ```text
    // window-status-activity-fg attributes
    // ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-activity-fg");
        let set_option =
            GetLocalWindowOptionValue::window_status_activity_fg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0 v1.9:
    // ```text
    // window-status-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-attr");
        let set_option = GetLocalWindowOptionValue::window_status_attr(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0 v1.9:
    // ```text
    // window-status-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-bg");
        let set_option = GetLocalWindowOptionValue::window_status_bg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0 v1.9:
    // ```text
    // window-status-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-fg");
        let set_option = GetLocalWindowOptionValue::window_status_fg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0 v1.9:
    // ```text
    // window-status-current-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-current-attr");
        let set_option =
            GetLocalWindowOptionValue::window_status_current_attr(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0 v1.9:
    // ```text
    // window-status-current-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-current-bg");
        let set_option =
            GetLocalWindowOptionValue::window_status_current_bg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0 v1.9:
    // ```text
    // window-status-current-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-current-fg");
        let set_option =
            GetLocalWindowOptionValue::window_status_current_fg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.3 v1.6:
    // ```text
    // window-status-alert-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    {
        let origin = format!("{} {}", cmd, "window-status-alert-attr");
        let set_option =
            GetLocalWindowOptionValue::window_status_alert_attr(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.3 v1.6:
    // ```text
    // window-status-alert-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    {
        let origin = format!("{} {}", cmd, "window-status-alert-bg");
        let set_option =
            GetLocalWindowOptionValue::window_status_alert_bg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.3 v1.6:
    // ```text
    // window-status-alert-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    {
        let origin = format!("{} {}", cmd, "window-status-alert-fg");
        let set_option =
            GetLocalWindowOptionValue::window_status_alert_fg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.9:
    // ```text
    // window-status-activity-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "window-status-activity-style");
        let set_option =
            GetLocalWindowOptionValue::window_status_activity_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.9:
    // ```text
    // window-status-bell-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "window-status-bell-style");
        let set_option =
            GetLocalWindowOptionValue::window_status_bell_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.9 v2.0:
    // ```text
    // window-status-content-style style
    // ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    {
        let origin = format!("{} {}", cmd, "window-status-content-style");
        let set_option =
            GetLocalWindowOptionValue::window_status_content_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.2:
    // ```text
    // window-status-current-format string
    // ```
    #[cfg(feature = "tmux_1_2")]
    {
        let origin = format!("{} {}", cmd, "window-status-current-format");
        let set_option =
            GetLocalWindowOptionValue::window_status_current_format(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.8 v1.9:
    // ```text
    // window-status-last-attr attributes
    // ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-last-attr");
        let set_option =
            GetLocalWindowOptionValue::window_status_last_attr(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.8 v1.9:
    // ```text
    // window-status-last-bg colour
    // ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-last-bg");
        let set_option = GetLocalWindowOptionValue::window_status_last_bg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.8 v1.9:
    // ```text
    // window-status-last-fg colour
    // ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "window-status-last-fg");
        let set_option = GetLocalWindowOptionValue::window_status_last_fg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.9:
    // ```text
    // window-status-current-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "window-status-current-style");
        let set_option =
            GetLocalWindowOptionValue::window_status_current_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.2:
    // ```text
    // window-status-format string
    // ```
    #[cfg(feature = "tmux_1_2")]
    {
        let origin = format!("{} {}", cmd, "window-status-format");
        let set_option = GetLocalWindowOptionValue::window_status_format(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.9:
    // ```text
    // window-status-last-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "window-status-last-style");
        let set_option =
            GetLocalWindowOptionValue::window_status_last_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.7:
    // ```text
    // window-status-separator string
    // ```
    #[cfg(feature = "tmux_1_7")]
    {
        let origin = format!("{} {}", cmd, "window-status-separator");
        let set_option =
            GetLocalWindowOptionValue::window_status_separator(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.9:
    // ```text
    // window-status-style style
    // ```
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "window-status-style");
        let set_option = GetLocalWindowOptionValue::window_status_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^2.9:
    // ```text
    // window-size largest | smallest | manual | latest
    // ```
    #[cfg(feature = "tmux_2_9")]
    {
        let origin = format!("{} {}", cmd, "window-size");
        let set_option = GetLocalWindowOptionValue::window_size(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.2 v1.6:
    // ```text
    // word-separators string
    // ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    {
        let origin = format!("{} {}", cmd, "word-separators");
        let set_option = GetLocalWindowOptionValue::word_separators(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^2.1 v3.0:
    // ```text
    // window-style style
    // ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    {
        let origin = format!("{} {}", cmd, "window-style");
        let set_option = GetLocalWindowOptionValue::window_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.7:
    // ```text
    // wrap-search [on | off]
    // ```
    #[cfg(feature = "tmux_1_7")]
    {
        let origin = format!("{} {}", cmd, "wrap-search");
        let set_option = GetLocalWindowOptionValue::wrap_search(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    // ### Manual
    //
    // tmux ^1.0:
    // ```text
    // xterm-keys [on | off]
    // ```
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "xterm-keys");
        let set_option = GetLocalWindowOptionValue::xterm_keys(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }

    {
        let origin = format!("{} {}", cmd, "@user-option-name");
        let set_option =
            GetLocalWindowOptionValue::user_option_ext(Some(target), "user-option-name")
                .to_string();
        assert_eq!(origin, set_option);
    }
}
