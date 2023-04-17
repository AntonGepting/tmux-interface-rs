#[test]
fn set_local_window_option_tests() {
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    use crate::ModeMouse;
    #[cfg(feature = "tmux_2_3")]
    use crate::PaneBorderStatus;
    #[cfg(feature = "tmux_2_9")]
    use crate::WindowSize;
    use crate::{
        ClockModeStyle, SetLocalWindowOption, SetUserOption, SetWindowOptionTr, StatusKeys, Switch,
    };

    let cmd = "set -w";
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
        let origin = format!("{} {} {}", cmd, "aggressive-resize", "on");
        let set_option =
            SetLocalWindowOption::aggressive_resize(Some(target), Some(Switch::On)).to_string();
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
        let origin = format!("{} {} {}", cmd, "allow-rename", "on");
        let set_option =
            SetLocalWindowOption::allow_rename(Some(target), Some(Switch::On)).to_string();
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
        let origin = format!("{} {} {}", cmd, "alternate-screen", "on");
        let set_option =
            SetLocalWindowOption::alternate_screen(Some(target), Some(Switch::On)).to_string();
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
        let origin = format!("{} {} {}", cmd, "automatic-rename", "on");
        let set_option =
            SetLocalWindowOption::automatic_rename(Some(target), Some(Switch::On)).to_string();
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
        let origin = format!("{} {} {}", cmd, "automatic-rename-format", "2");
        let set_option =
            SetLocalWindowOption::automatic_rename_format(Some(target), Some("2")).to_string();
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
        let origin = format!("{} {} {}", cmd, "c0-change-interval", "2");
        let set_option =
            SetLocalWindowOption::c0_change_interval(Some(target), Some(2)).to_string();
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
        let origin = format!("{} {} {}", cmd, "c0-change-trigger", "3");
        let set_option = SetLocalWindowOption::c0_change_trigger(Some(target), Some(3)).to_string();
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
        let origin = format!("{} {} {}", cmd, "clock-mode-colour", "colour");
        let set_option =
            SetLocalWindowOption::clock_mode_colour(Some(target), Some("colour")).to_string();
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
        let origin = format!("{} {} {}", cmd, "clock-mode-style", "12");
        let set_option =
            SetLocalWindowOption::clock_mode_style(Some(target), Some(ClockModeStyle::_12))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "force-height", "1");
        let set_option = SetLocalWindowOption::force_height(Some(target), Some(1)).to_string();
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
        let origin = format!("{} {} {}", cmd, "force-width", "2");
        let set_option = SetLocalWindowOption::force_width(Some(target), Some(2)).to_string();
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
        let origin = format!("{} {} {}", cmd, "layout-history-limit", "3");
        let set_option =
            SetLocalWindowOption::layout_history_limit(Some(target), Some(3)).to_string();
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
        let origin = format!("{} {} {}", cmd, "main-pane-height", "4");
        let set_option = SetLocalWindowOption::main_pane_height(Some(target), Some(4)).to_string();
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
        let origin = format!("{} {} {}", cmd, "main-pane-width", "5");
        let set_option = SetLocalWindowOption::main_pane_width(Some(target), Some(5)).to_string();
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
        let origin = format!("{} {} {}", cmd, "mode-attr", "6");
        let set_option = SetLocalWindowOption::mode_attr(Some(target), Some(6)).to_string();
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
        let origin = format!("{} {} {}", cmd, "mode-bg", "colour");
        let set_option = SetLocalWindowOption::mode_bg(Some(target), Some(colour)).to_string();
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
        let origin = format!("{} {} {}", cmd, "mode-fg", "colour");
        let set_option = SetLocalWindowOption::mode_fg(Some(target), Some(colour)).to_string();
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
        let origin = format!("{} {} {}", cmd, "mode-keys", "emacs");
        let set_option =
            SetLocalWindowOption::mode_keys(Some(target), Some(StatusKeys::Emacs)).to_string();
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
        let origin = format!("{} {} {}", cmd, "mode_mouse", "on");
        let set_option =
            SetLocalWindowOption::mode_mouse(Some(target), Some(ModeMouse::On)).to_string();
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
        let origin = format!("{} {} {}", cmd, "mode-style", "style");
        let set_option = SetLocalWindowOption::mode_style(Some(target), Some("style")).to_string();
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
        let origin = format!("{} {} {}", cmd, "monitor-activity", "on");
        let set_option =
            SetLocalWindowOption::monitor_activity(Some(target), Some(Switch::On)).to_string();
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
        let origin = format!("{} {} {}", cmd, "monitor-content", "match-string");
        let set_option =
            SetLocalWindowOption::monitor_content(Some(target), Some("match-string")).to_string();
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
        let origin = format!("{} {} {}", cmd, "monitor-bell", "on");
        let set_option =
            SetLocalWindowOption::monitor_bell(Some(target), Some(Switch::On)).to_string();
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
        let origin = format!("{} {} {}", cmd, "monitor-silence", "1");
        let set_option = SetLocalWindowOption::monitor_silence(Some(target), Some(1)).to_string();
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
        let origin = format!("{} {} {}", cmd, "other-pane-height", "1");
        let set_option = SetLocalWindowOption::other_pane_height(Some(target), Some(1)).to_string();
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
        let origin = format!("{} {} {}", cmd, "other-pane-width", "2");
        let set_option = SetLocalWindowOption::other_pane_width(Some(target), Some(2)).to_string();
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
        let origin = format!("{} {} {}", cmd, "pane-active-border-style", "style");
        let set_option =
            SetLocalWindowOption::pane_active_border_style(Some(target), Some("style")).to_string();
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
        let origin = format!("{} {} {}", cmd, "pane-base-index", "5");
        let set_option = SetLocalWindowOption::pane_base_index(Some(target), Some(5)).to_string();
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
        let origin = format!("{} {} {}", cmd, "pane-border-format", "format");
        let set_option =
            SetLocalWindowOption::pane_border_format(Some(target), Some("format")).to_string();
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
        let origin = format!("{} {} {}", cmd, "pane-border-status", "top");
        let set_option =
            SetLocalWindowOption::pane_border_status(Some(target), Some(PaneBorderStatus::Top))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "pane-border-style", "style");
        let set_option =
            SetLocalWindowOption::pane_border_style(Some(target), Some("style")).to_string();
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
        let origin = format!("{} {} {}", cmd, "remain-on-exit", "on");
        let set_option =
            SetLocalWindowOption::remain_on_exit(Some(target), Some(Switch::On)).to_string();
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
        let origin = format!("{} {} {}", cmd, "synchronize-panes", "on");
        let set_option =
            SetLocalWindowOption::synchronize_panes(Some(target), Some(Switch::On)).to_string();
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
        let origin = format!("{} {} {}", cmd, "utf8", "on");
        let set_option = SetLocalWindowOption::utf8(Some(target), Some(Switch::On)).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-active-style", "style");
        let set_option =
            SetLocalWindowOption::window_active_style(Some(target), Some("style")).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-bell-attr", "attributes");
        let set_option =
            SetLocalWindowOption::window_status_bell_attr(Some(target), Some("attributes"))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-bell-bg", "colour");
        let set_option =
            SetLocalWindowOption::window_status_bell_bg(Some(target), Some("colour")).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-bell-fg", "colour");
        let set_option =
            SetLocalWindowOption::window_status_bell_fg(Some(target), Some("colour")).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-content-attr", "attributes");
        let set_option =
            SetLocalWindowOption::window_status_content_attr(Some(target), Some("attributes"))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-content-bg", "colour");
        let set_option =
            SetLocalWindowOption::window_status_content_bg(Some(target), Some("colour"))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-content-fg", "colour");
        let set_option =
            SetLocalWindowOption::window_status_content_fg(Some(target), Some("colour"))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-activity-attr", "attributes");
        let set_option =
            SetLocalWindowOption::window_status_activity_attr(Some(target), Some("attributes"))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-activity-bg", "attributes");
        let set_option =
            SetLocalWindowOption::window_status_activity_bg(Some(target), Some("attributes"))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-activity-fg", "attributes");
        let set_option =
            SetLocalWindowOption::window_status_activity_fg(Some(target), Some("attributes"))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-attr", "attributes");
        let set_option =
            SetLocalWindowOption::window_status_attr(Some(target), Some("attributes")).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-bg", "colour");
        let set_option =
            SetLocalWindowOption::window_status_bg(Some(target), Some("colour")).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-fg", "colour");
        let set_option =
            SetLocalWindowOption::window_status_fg(Some(target), Some("colour")).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-current-attr", "attributes");
        let set_option =
            SetLocalWindowOption::window_status_current_attr(Some(target), Some("attributes"))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-current-bg", "colour");
        let set_option =
            SetLocalWindowOption::window_status_current_bg(Some(target), Some("colour"))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-current-fg", "colour");
        let set_option =
            SetLocalWindowOption::window_status_current_fg(Some(target), Some("colour"))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-alert-attr", "attributes");
        let set_option =
            SetLocalWindowOption::window_status_alert_attr(Some(target), Some("attributes"))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-alert-bg", "colour");
        let set_option =
            SetLocalWindowOption::window_status_alert_bg(Some(target), Some("colour")).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-alert-fg", "colour");
        let set_option =
            SetLocalWindowOption::window_status_alert_fg(Some(target), Some("colour")).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-activity-style", "style");
        let set_option =
            SetLocalWindowOption::window_status_activity_style(Some(target), Some("style"))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-bell-style", "style");
        let set_option =
            SetLocalWindowOption::window_status_bell_style(Some(target), Some("style")).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-content-style", "style");
        let set_option =
            SetLocalWindowOption::window_status_content_style(Some(target), Some("style"))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-current-format", "string");
        let set_option =
            SetLocalWindowOption::window_status_current_format(Some(target), Some("string"))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-last-attr", "attributes");
        let set_option =
            SetLocalWindowOption::window_status_last_attr(Some(target), Some("attributes"))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-last-bg", "colour");
        let set_option =
            SetLocalWindowOption::window_status_last_bg(Some(target), Some("colour")).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-last-fg", "colour");
        let set_option =
            SetLocalWindowOption::window_status_last_fg(Some(target), Some("colour")).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-current-style", "style");
        let set_option =
            SetLocalWindowOption::window_status_current_style(Some(target), Some("style"))
                .to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-format", "string");
        let set_option =
            SetLocalWindowOption::window_status_format(Some(target), Some("string")).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-last-style", "style");
        let set_option =
            SetLocalWindowOption::window_status_last_style(Some(target), Some("style")).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-separator", "string");
        let set_option =
            SetLocalWindowOption::window_status_separator(Some(target), Some("string")).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-status-style", "style");
        let set_option =
            SetLocalWindowOption::window_status_style(Some(target), Some("style")).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-size", "largest");
        let set_option =
            SetLocalWindowOption::window_size(Some(target), Some(WindowSize::Largest)).to_string();
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
        let origin = format!("{} {} {}", cmd, "word-separators", "string");
        let set_option =
            SetLocalWindowOption::word_separators(Some(target), Some("string")).to_string();
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
        let origin = format!("{} {} {}", cmd, "window-style", "style");
        let set_option =
            SetLocalWindowOption::window_style(Some(target), Some("style")).to_string();
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
        let origin = format!("{} {} {}", cmd, "wrap-search", "on");
        let set_option =
            SetLocalWindowOption::wrap_search(Some(target), Some(Switch::On)).to_string();
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
        let origin = format!("{} {} {}", cmd, "xterm-keys", "on");
        let set_option =
            SetLocalWindowOption::xterm_keys(Some(target), Some(Switch::On)).to_string();
        assert_eq!(origin, set_option);
    }

    {
        let origin = format!("{} {} {}", cmd, "@user-option-name", "value");
        let set_option =
            SetLocalWindowOption::user_option_ext(Some(target), "user-option-name", Some("value"))
                .to_string();
        assert_eq!(origin, set_option);
    }
}
