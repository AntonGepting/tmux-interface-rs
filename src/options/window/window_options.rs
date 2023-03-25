use super::*;
use crate::options::common::{cow_parse, get_parts, option_to_string};
use crate::options::StatusKeys;
use crate::Error;
use crate::Switch;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
use crate::ModeMouse;

// TODO: check types
// 31 Available window options are:
#[derive(PartialEq, Clone, Debug)]
pub struct WindowOptions<'a> {
    //aggressive-resize [on | off]
    #[cfg(feature = "tmux_1_0")]
    pub aggressive_resize: Option<Switch>,
    //allow-rename [on | off]
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    pub allow_rename: Option<Switch>,
    //alternate-screen [on | off]
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    pub alternate_screen: Option<Switch>,
    //automatic-rename [on | off]
    #[cfg(feature = "tmux_1_0")] // 0.8
    pub automatic_rename: Option<Switch>,
    //automatic-rename-format format
    #[cfg(feature = "tmux_1_9")]
    pub automatic_rename_format: Option<Cow<'a, str>>,
    //c0-change-interval interval
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub c0_change_interval: Option<usize>,
    //c0-change-trigger trigger
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub c0_change_trigger: Option<Cow<'a, str>>,
    //clock-mode-colour colour
    #[cfg(feature = "tmux_1_0")]
    pub clock_mode_colour: Option<Cow<'a, str>>,
    //clock-mode-style [12 | 24]
    #[cfg(feature = "tmux_1_0")]
    pub clock_mode_style: Option<ClockModeStyle>,
    //force-height height
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    pub force_height: Option<usize>,
    //force-width width
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    pub force_width: Option<usize>,
    //layout-history-limit limit
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    pub layout_history_limit: Option<usize>,
    //main-pane-height height
    #[cfg(feature = "tmux_1_0")]
    pub main_pane_height: Option<usize>,
    //main-pane-width width
    #[cfg(feature = "tmux_1_0")]
    pub main_pane_width: Option<usize>,
    //mode-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub mode_attr: Option<Cow<'a, str>>,
    // mode-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub mode_bg: Option<Cow<'a, str>>,
    // mode-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub mode_fg: Option<Cow<'a, str>>,
    //mode-keys [vi | emacs]
    #[cfg(feature = "tmux_1_0")]
    pub mode_keys: Option<StatusKeys>,
    //mode-mouse [on | off]
    //tmux 1.6: mode-mouse [on | off | copy-mode]
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub mode_mouse: Option<ModeMouse>,
    //mode-style style
    #[cfg(feature = "tmux_1_9")]
    pub mode_style: Option<Cow<'a, str>>,
    //monitor-activity [on | off]
    #[cfg(feature = "tmux_1_0")]
    pub monitor_activity: Option<Switch>,
    //monitor-content match-string
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub monitor_content: Option<Cow<'a, str>>,
    //monitor-bell [on | off]
    #[cfg(feature = "tmux_2_6")]
    pub monitor_bell: Option<Switch>,
    //monitor-silence [interval]
    #[cfg(feature = "tmux_1_4")]
    pub monitor_silence: Option<usize>,
    //other-pane-height height
    #[cfg(feature = "tmux_1_4")]
    pub other_pane_height: Option<usize>,
    //other-pane-width width
    #[cfg(feature = "tmux_1_4")]
    pub other_pane_width: Option<usize>,
    //pane-active-border-style style
    #[cfg(feature = "tmux_2_0")]
    pub pane_active_border_style: Option<Cow<'a, str>>,
    //pane-base-index index
    #[cfg(feature = "tmux_1_6")]
    pub pane_base_index: Option<usize>,
    //pane-border-format format
    #[cfg(feature = "tmux_2_3")]
    pub pane_border_format: Option<Cow<'a, str>>,
    //pane-border-status [off | top | bottom]
    #[cfg(feature = "tmux_2_3")]
    pub pane_border_status: Option<PaneBorderStatus>,
    //pane-border-style style
    #[cfg(feature = "tmux_2_0")]
    pub pane_border_style: Option<Cow<'a, str>>,
    //remain-on-exit [on | off]
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    pub remain_on_exit: Option<Switch>,
    //synchronize-panes [on | off]
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    pub synchronize_panes: Option<Switch>,
    //utf8 [on | off]
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub utf8: Option<Switch>,
    //window-active-style style
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub window_active_style: Option<Cow<'a, str>>,
    //window-status-bell-attr attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_bell_attr: Option<Cow<'a, str>>,
    //window-status-bell-bg colour
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_bell_bg: Option<Cow<'a, str>>,
    //window-status-bell-fg colour
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_bell_fg: Option<Cow<'a, str>>,
    //window-status-content-attr attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_content_attr: Option<Cow<'a, str>>,
    //window-status-content-bg colour
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_content_bg: Option<Cow<'a, str>>,
    //window-status-content-fg colour
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_content_fg: Option<Cow<'a, str>>,
    //window-status-activity-attr attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_activity_attr: Option<Cow<'a, str>>,
    //window-status-activity-bg attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_activity_bg: Option<Cow<'a, str>>,
    //window-status-activity-fg attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_activity_fg: Option<Cow<'a, str>>,
    //window-status-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_attr: Option<Cow<'a, str>>,
    //window-status-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_bg: Option<Cow<'a, str>>,
    //window-status-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_fg: Option<Cow<'a, str>>,
    //window-status-current-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_current_attr: Option<Cow<'a, str>>,
    //window-status-current-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_current_bg: Option<Cow<'a, str>>,
    //window-status-current-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_current_fg: Option<Cow<'a, str>>,
    //window-status-alert-attr attributes
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub window_status_alert_attr: Option<Cow<'a, str>>,
    //window-status-alert-bg colour
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub window_status_alert_bg: Option<Cow<'a, str>>,
    //window-status-alert-fg colour
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub window_status_alert_fg: Option<Cow<'a, str>>,
    //window-status-activity-style style
    #[cfg(feature = "tmux_1_9")]
    pub window_status_activity_style: Option<Cow<'a, str>>,
    //window-status-bell-style style
    #[cfg(feature = "tmux_1_9")]
    pub window_status_bell_style: Option<Cow<'a, str>>,
    //window-status-content-style style
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub window_status_content_style: Option<Cow<'a, str>>,
    //window-status-current-format string
    #[cfg(feature = "tmux_1_2")]
    pub window_status_current_format: Option<Cow<'a, str>>,
    //window-status-last-attr attributes
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub window_status_last_attr: Option<Cow<'a, str>>,
    //window-status-last-bg colour
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub window_status_last_bg: Option<Cow<'a, str>>,
    //window-status-last-fg colour
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub window_status_last_fg: Option<Cow<'a, str>>,
    //window-status-current-style style
    #[cfg(feature = "tmux_1_9")]
    pub window_status_current_style: Option<Cow<'a, str>>,
    //window-status-format string
    #[cfg(feature = "tmux_1_2")]
    pub window_status_format: Option<Cow<'a, str>>,
    //window-status-last-style style
    #[cfg(feature = "tmux_1_9")]
    pub window_status_last_style: Option<Cow<'a, str>>,
    //window-status-separator string
    #[cfg(feature = "tmux_1_7")]
    pub window_status_separator: Option<Cow<'a, str>>,
    //window-status-style style
    #[cfg(feature = "tmux_1_9")]
    pub window_status_style: Option<Cow<'a, str>>,
    //window-size largest | smallest | manual | latest
    #[cfg(feature = "tmux_2_9")]
    pub window_size: Option<WindowSize>,
    //word-separators string
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    pub word_separators: Option<Cow<'a, str>>,
    //window-style style
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub window_style: Option<Cow<'a, str>>,
    //wrap-search [on | off]
    #[cfg(feature = "tmux_1_7")]
    pub wrap_search: Option<Switch>,
    // xterm-keys [on | off]
    #[cfg(feature = "tmux_1_0")]
    pub xterm_keys: Option<Switch>,
    // XXX: user options?
    pub user_options: HashMap<String, Option<Cow<'a, str>>>,
}

// TODO: check default values from tmux sources
/// ```text
/// tmux show-options -g -w
/// ```
///
/// ```text
/// aggressive-resize off
/// //allow-passthrough off
/// allow-rename off
/// alternate-screen on
/// automatic-rename on
/// automatic-rename-format "#{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}"
/// //c0-change-interval
/// //c0-change-trigger //?
/// clock-mode-colour blue // 4
/// clock-mode-style 24 // 1
/// force-height 0
/// force-width 0
/// //layout-history-limit //?
/// main-pane-height 24
/// main-pane-width 80
/// mode-attr 0 // 0
/// mode-bg yellow // 3
/// mode-fg black // 0
/// mode-keys vi // modekey_emacs
/// mode-mouse
/// mode-style fg=black,bg=yellow
/// monitor-activity off
/// monitor-content //?
/// monitor-bell on
/// monitor-silence 0
/// other-pane-height 0
/// other-pane-width 0
/// pane-active-border-style fg=green
/// pane-active-border-bg 8 // 8
/// pane-active-border-fg 2 // 2
/// pane-base-index 0
/// pane-border-bg 8 // 8
/// pane-border-fg 8 // 8
/// pane-border-format #{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"
/// pane-border-status off
/// pane-border-style default
/// remain-on-exit off
/// synchronize-panes off
/// window-active-style default
/// window-size latest
/// window-style default
/// window-status-activity-attr grid_attr_reverse // ?
/// window-status-activity-bg 8 // 8
/// window-status-activity-fg 8 // 8
/// window-status-activity-style reverse
/// window-status-attr 0 //
/// window-status-bell-attr gridattrreverse // ?
/// window-status-bell-bg 8 // 8
/// window-status-bell-fg 8 // 8
/// window-status-bell-style reverse
/// window-status-bg 8 // 8
/// window-status-current-attr 0 //
/// window-status-current-bg 8 //
/// window-status-current-fg 8 //
/// window-status-current-format #I:#W#{?window_flags,#{window_flags}, }
/// window-status-current-style default
/// window-status-fg 8 // 8
/// window-status-format #I:#W#{?window_flags,#{window_flags}, }
/// window-status-last-attr 0 //
/// window-status-last-bg 8 //
/// window-status-last-fg 8 //
/// window-status-last-style default
/// window-status-separator " "
/// window-status-style default
/// wrap-search on
/// xterm-keys on
/// ```
///
impl<'a> Default for WindowOptions<'a> {
    fn default() -> Self {
        let options = WindowOptions::new();

        #[cfg(feature = "tmux_1_0")]
        let options = options.aggressive_resize(Some(Switch::Off));
        // allow-passthrough off
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
        let options = options.allow_rename(Some(Switch::Off));
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
        let options = options.alternate_screen(Some(Switch::On));
        #[cfg(feature = "tmux_1_0")] // 0.8
        let options = options.automatic_rename(Some(Switch::On));
        #[cfg(feature = "tmux_1_9")]
        let options = options.automatic_rename_format(Some(
            "#{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}",
        ));
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
        let options = options.c0_change_interval(None);
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
        let options = options.c0_change_trigger(None);
        #[cfg(feature = "tmux_1_0")]
        let options = options.clock_mode_colour(Some("blue"));
        #[cfg(feature = "tmux_1_0")]
        let options = options.clock_mode_style(Some(ClockModeStyle::_24));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
        let options = options.force_height(Some(0));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
        let options = options.force_width(Some(0));
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
        let options = options.layout_history_limit(Some());
        #[cfg(feature = "tmux_1_0")]
        let options = options.main_pane_height(Some(24));
        #[cfg(feature = "tmux_1_0")]
        let options = options.main_pane_width(Some(80));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.mode_attr(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.mode_bg(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.mode_fg(Some());
        #[cfg(feature = "tmux_1_0")]
        let options = options.mode_keys(Some(StatusKeys::Vi));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
        let options = options.mode_mouse(Some(ModeMouse::Off));
        #[cfg(feature = "tmux_1_9")]
        let options = options.mode_style(Some("fg=black,bg=yellow"));
        #[cfg(feature = "tmux_1_0")]
        let options = options.monitor_activity(Some(Switch::Off));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        let options = options.monitor_content(Some());
        #[cfg(feature = "tmux_2_6")]
        let options = options.monitor_bell(Some(Switch::On));
        #[cfg(feature = "tmux_1_4")]
        let options = options.monitor_silence(Some(0));
        #[cfg(feature = "tmux_1_4")]
        let options = options.other_pane_height(Some(0));
        #[cfg(feature = "tmux_1_4")]
        let options = options.other_pane_width(Some(0));
        #[cfg(feature = "tmux_2_0")]
        let options = options.pane_active_border_style(Some("fg=green"));
        #[cfg(feature = "tmux_1_6")]
        let options = options.pane_base_index(Some(0));
        #[cfg(feature = "tmux_2_3")]
        let options = options.pane_border_format(Some(
            "#{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"",
        ));
        #[cfg(feature = "tmux_2_3")]
        let options = options.pane_border_status(Some(PaneBorderStatus::Off));
        #[cfg(feature = "tmux_2_0")]
        let options = options.pane_border_style(Some("fg=colour238,bg=colour235"));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
        let options = options.remain_on_exit(Some(Switch::Off));
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
        let options = options.synchronize_panes(Some(Switch::Off));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
        let options = options.utf8(Some(Switch::Off));
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
        let options = options.window_active_style(Some("fg=colour253,bg=colour235"));
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_bell_attr(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_bell_bg(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_bell_fg(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_content_attr(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_content_bg(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_content_fg(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_activity_attr(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_activity_bg(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_activity_fg(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_attr(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_bg(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_fg(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_current_attr(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_current_bg(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_current_fg(Some());
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        let options = options.window_status_alert_attr(Some());
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        let options = options.window_status_alert_bg(Some());
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        let options = options.window_status_alert_fg(Some());
        #[cfg(feature = "tmux_1_9")]
        let options = options.window_status_activity_style(Some("reverse"));
        #[cfg(feature = "tmux_1_9")]
        let options = options.window_status_bell_style(Some("fg=colour253,bg=colour1,bright"));
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        let options = options.window_status_content_style(Some());
        #[cfg(feature = "tmux_1_2")]
        let options =
            options.window_status_current_format(Some("#I:#W#{?window_flags,#{window_flags}, }"));
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        let options = options.window_status_last_attr(Some());
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        let options = options.window_status_last_bg(Some());
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        let options = options.window_status_last_fg(Some());
        #[cfg(feature = "tmux_1_9")]
        let options = options.window_status_current_style(Some("fg=colour22,bg=colour114"));
        #[cfg(feature = "tmux_1_2")]
        let options = options.window_status_format(Some("#I:#W#{?window_flags,#{window_flags}, }"));
        #[cfg(feature = "tmux_1_9")]
        let options = options.window_status_last_style(Some("default"));
        #[cfg(feature = "tmux_1_7")]
        let options = options.window_status_separator(Some("\" \""));
        #[cfg(feature = "tmux_1_9")]
        let options = options.window_status_style(Some("default"));
        #[cfg(feature = "tmux_2_9")]
        let options = options.window_size(Some(WindowSize::Largest));
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
        let options = options.word_separators(Some());
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
        let options = options.window_style(Some("default"));
        #[cfg(feature = "tmux_1_7")]
        let options = options.wrap_search(Some(Switch::On));
        #[cfg(feature = "tmux_1_0")]
        let options = options.xterm_keys(Some(Switch::On));
        options
    }
}

impl<'a> fmt::Display for WindowOptions<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, AGGRESSIVE_RESIZE, &self.aggressive_resize);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
        option_to_string(&mut v, ALLOW_RENAME, &self.allow_rename);
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
        option_to_string(&mut v, ALTERNATE_SCREEN, &self.alternate_screen);
        #[cfg(feature = "tmux_1_0")] // 0.8
        option_to_string(&mut v, AUTOMATIC_RENAME, &self.automatic_rename);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(
            &mut v,
            AUTOMATIC_RENAME_FORMAT,
            &self.automatic_rename_format,
        );
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
        option_to_string(&mut v, C0_CHANGE_INTERVAL, &self.c0_change_interval);
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
        option_to_string(&mut v, C0_CHANGE_TRIGGER, &self.c0_change_trigger);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, CLOCK_MODE_COLOUR, &self.clock_mode_colour);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, CLOCK_MODE_STYLE, &self.clock_mode_style);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
        option_to_string(&mut v, FORCE_HEIGHT, &self.force_height);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
        option_to_string(&mut v, FORCE_WIDTH, &self.force_width);
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
        option_to_string(&mut v, LAYOUT_HISTORY_LIMIT, &self.layout_history_limit);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, MAIN_PANE_HEIGHT, &self.main_pane_height);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, MAIN_PANE_WIDTH, &self.main_pane_width);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, MODE_ATTR, &self.mode_attr);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, MODE_BG, &self.mode_bg);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, MODE_FG, &self.mode_fg);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, MODE_KEYS, &self.mode_keys);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
        option_to_string(&mut v, MODE_MOUSE, &self.mode_mouse);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(&mut v, MODE_STYLE, &self.mode_style);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, MONITOR_ACTIVITY, &self.monitor_activity);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        option_to_string(&mut v, MONITOR_CONTENT, &self.monitor_content);
        #[cfg(feature = "tmux_2_6")]
        option_to_string(&mut v, MONITOR_BELL, &self.monitor_bell);
        #[cfg(feature = "tmux_1_4")]
        option_to_string(&mut v, MONITOR_SILENCE, &self.monitor_silence);
        #[cfg(feature = "tmux_1_4")]
        option_to_string(&mut v, OTHER_PANE_HEIGHT, &self.other_pane_height);
        #[cfg(feature = "tmux_1_4")]
        option_to_string(&mut v, OTHER_PANE_WIDTH, &self.other_pane_width);
        #[cfg(feature = "tmux_2_0")]
        option_to_string(
            &mut v,
            PANE_ACTIVE_BORDER_STYLE,
            &self.pane_active_border_style,
        );
        #[cfg(feature = "tmux_1_6")]
        option_to_string(&mut v, PANE_BASE_INDEX, &self.pane_base_index);
        #[cfg(feature = "tmux_2_3")]
        option_to_string(&mut v, PANE_BORDER_FORMAT, &self.pane_border_format);
        #[cfg(feature = "tmux_2_3")]
        option_to_string(&mut v, PANE_BORDER_STATUS, &self.pane_border_status);
        #[cfg(feature = "tmux_2_0")]
        option_to_string(&mut v, PANE_BORDER_STYLE, &self.pane_border_style);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
        option_to_string(&mut v, REMAIN_ON_EXIT, &self.remain_on_exit);
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
        option_to_string(&mut v, SYNCHRONIZE_PANES, &self.synchronize_panes);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
        option_to_string(&mut v, UTF8, &self.utf8);
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
        option_to_string(&mut v, WINDOW_ACTIVE_STYLE, &self.window_active_style);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_BELL_ATTR,
            &self.window_status_bell_attr,
        );
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, WINDOW_STATUS_BELL_BG, &self.window_status_bell_bg);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, WINDOW_STATUS_BELL_FG, &self.window_status_bell_fg);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CONTENT_ATTR,
            &self.window_status_content_attr,
        );
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CONTENT_BG,
            &self.window_status_content_bg,
        );
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CONTENT_FG,
            &self.window_status_content_fg,
        );
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_ACTIVITY_ATTR,
            &self.window_status_activity_attr,
        );
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_ACTIVITY_BG,
            &self.window_status_activity_bg,
        );
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_ACTIVITY_FG,
            &self.window_status_activity_fg,
        );
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, WINDOW_STATUS_ATTR, &self.window_status_attr);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, WINDOW_STATUS_BG, &self.window_status_bg);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, WINDOW_STATUS_FG, &self.window_status_fg);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CURRENT_ATTR,
            &self.window_status_current_attr,
        );
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CURRENT_BG,
            &self.window_status_current_bg,
        );
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CURRENT_FG,
            &self.window_status_current_fg,
        );
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_ALERT_ATTR,
            &self.window_status_alert_attr,
        );
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        option_to_string(&mut v, WINDOW_STATUS_ALERT_BG, &self.window_status_alert_bg);
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        option_to_string(&mut v, WINDOW_STATUS_ALERT_FG, &self.window_status_alert_fg);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(
            &mut v,
            WINDOW_STATUS_ACTIVITY_STYLE,
            &self.window_status_activity_style,
        );
        #[cfg(feature = "tmux_1_9")]
        option_to_string(
            &mut v,
            WINDOW_STATUS_BELL_STYLE,
            &self.window_status_bell_style,
        );
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CONTENT_STYLE,
            &self.window_status_content_style,
        );
        #[cfg(feature = "tmux_1_2")]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CURRENT_FORMAT,
            &self.window_status_current_format,
        );
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_LAST_ATTR,
            &self.window_status_last_attr,
        );
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, WINDOW_STATUS_LAST_BG, &self.window_status_last_bg);
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, WINDOW_STATUS_LAST_FG, &self.window_status_last_fg);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CURRENT_STYLE,
            &self.window_status_current_style,
        );
        #[cfg(feature = "tmux_1_2")]
        option_to_string(&mut v, WINDOW_STATUS_FORMAT, &self.window_status_format);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(
            &mut v,
            WINDOW_STATUS_LAST_STYLE,
            &self.window_status_last_style,
        );
        #[cfg(feature = "tmux_1_7")]
        option_to_string(
            &mut v,
            WINDOW_STATUS_SEPARATOR,
            &self.window_status_separator,
        );
        #[cfg(feature = "tmux_1_9")]
        option_to_string(&mut v, WINDOW_STATUS_STYLE, &self.window_status_style);
        #[cfg(feature = "tmux_2_9")]
        option_to_string(&mut v, WINDOW_SIZE, &self.window_size);
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
        option_to_string(&mut v, WORD_SEPARATORS, &self.word_separators);
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
        option_to_string(&mut v, WINDOW_STYLE, &self.window_style);
        #[cfg(feature = "tmux_1_7")]
        option_to_string(&mut v, WRAP_SEARCH, &self.wrap_search);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, XTERM_KEYS, &self.xterm_keys);
        // option_to_string(&mut v, USER_OPTIONS, &self.user_options);
        let s = v.join("\n");
        write!(f, "{}", s)
    }
}
impl<'a> WindowOptions<'a> {
    pub fn new() -> Self {
        WindowOptions {
            #[cfg(feature = "tmux_1_0")]
            aggressive_resize: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
            allow_rename:None,
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
            alternate_screen: None,
            #[cfg(feature = "tmux_1_0")] // 0.8
            automatic_rename: None,
            #[cfg(feature = "tmux_1_9")]
            automatic_rename_format: None,
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
            c0_change_interval: None,
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
            c0_change_trigger: None,
            #[cfg(feature = "tmux_1_0")]
            clock_mode_colour: None,
            #[cfg(feature = "tmux_1_0")]
            clock_mode_style: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
            force_height: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
            force_width: None,
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
            layout_history_limit: None,
            #[cfg(feature = "tmux_1_0")]
            main_pane_height: None,
            #[cfg(feature = "tmux_1_0")]
            main_pane_width: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            mode_attr: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            mode_bg: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            mode_fg: None,
            #[cfg(feature = "tmux_1_0")]
            mode_keys: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
            mode_mouse: None,
            #[cfg(feature = "tmux_1_9")]
            mode_style: None,
            #[cfg(feature = "tmux_1_0")]
            monitor_activity: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
            monitor_content: None,
            #[cfg(feature = "tmux_2_6")]
            monitor_bell: None,
            #[cfg(feature = "tmux_1_4")]
            monitor_silence: None,
            #[cfg(feature = "tmux_1_4")]
            other_pane_height: None,
            #[cfg(feature = "tmux_1_4")]
            other_pane_width: None,
            #[cfg(feature = "tmux_2_0")]
            pane_active_border_style: None,
            #[cfg(feature = "tmux_1_6")]
            pane_base_index: None,
            #[cfg(feature = "tmux_2_3")]
            pane_border_format: None,
            #[cfg(feature = "tmux_2_3")]
            pane_border_status: None,
            #[cfg(feature = "tmux_2_0")]
            pane_border_style: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
            remain_on_exit: None,
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
            synchronize_panes: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
            utf8: None,
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
            window_active_style: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_bell_attr: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_bell_bg: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_bell_fg: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_content_attr: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_content_bg: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_content_fg: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_activity_attr: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_activity_bg: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_activity_fg: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_attr: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_bg: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_fg: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_current_attr: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_current_bg: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_current_fg: None,
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
            window_status_alert_attr: None,
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
            window_status_alert_bg: None,
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
            window_status_alert_fg: None,
            #[cfg(feature = "tmux_1_9")]
            window_status_activity_style: None,
            #[cfg(feature = "tmux_1_9")]
            window_status_bell_style: None,
            #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
            window_status_content_style: None,
            #[cfg(feature = "tmux_1_2")]
            window_status_current_format: None,
            #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
            window_status_last_attr: None,
            #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
            window_status_last_bg: None,
            #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
            window_status_last_fg: None,
            #[cfg(feature = "tmux_1_9")]
            window_status_current_style: None,
            #[cfg(feature = "tmux_1_2")]
            window_status_format: None,
            #[cfg(feature = "tmux_1_9")]
            window_status_last_style: None,
            #[cfg(feature = "tmux_1_7")]
            window_status_separator: None,
            #[cfg(feature = "tmux_1_9")]
            window_status_style: None,
            #[cfg(feature = "tmux_2_9")]
            window_size: None,
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
            word_separators: None,
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
            window_style: None,
            #[cfg(feature = "tmux_1_7")]
            wrap_search: None,
            #[cfg(feature = "tmux_1_0")]
            xterm_keys: None,
            user_options: HashMap::new()
        }
    }

    // XXX: bitmask is overkill now, mb later use for multiple select
    // NOTE: not allows selective get by bitmask
}

// command_alias[0] = "alias1" => command_alias["alias1"]
// command_alias[1] = "alias2" => command_alias["alias2"]
// ...
// command_alias[n] = "aliasN" => command_alias["aliasN"]
// TODO: optimization, merge server, session, window, pane?
//impl FromStr for WindowOptions {
//type Err = Error;

impl<'a> WindowOptions<'a> {
    #[cfg(feature = "tmux_1_0")]
    pub fn aggressive_resize(mut self, aggressive_resize: Option<Switch>) -> Self {
        self.aggressive_resize = aggressive_resize;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    pub fn allow_rename(mut self, allow_rename: Option<Switch>) -> Self {
        self.allow_rename = allow_rename;
        self
    }
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    pub fn alternate_screen(mut self, alternate_screen: Option<Switch>) -> Self {
        self.alternate_screen = alternate_screen;
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn automatic_rename(mut self, automatic_rename: Option<Switch>) -> Self {
        self.automatic_rename = automatic_rename;
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn automatic_rename_format<S: Into<Cow<'a, str>>>(
        mut self,
        automatic_rename_format: Option<S>,
    ) -> Self {
        self.automatic_rename_format = automatic_rename_format.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub fn c0_change_interval(mut self, c0_change_interval: Option<usize>) -> Self {
        self.c0_change_interval = c0_change_interval;
        self
    }

    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub fn c0_change_trigger<S: Into<Cow<'a, str>>>(
        mut self,
        c0_change_trigger: Option<S>,
    ) -> Self {
        self.c0_change_trigger = c0_change_trigger.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn clock_mode_colour<S: Into<Cow<'a, str>>>(
        mut self,
        clock_mode_colour: Option<S>,
    ) -> Self {
        self.clock_mode_colour = clock_mode_colour.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn clock_mode_style(mut self, clock_mode_style: Option<ClockModeStyle>) -> Self {
        self.clock_mode_style = clock_mode_style;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    pub fn force_height(mut self, force_height: Option<usize>) -> Self {
        self.force_height = force_height;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    pub fn force_width(mut self, force_width: Option<usize>) -> Self {
        self.force_width = force_width;
        self
    }

    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    pub fn layout_history_limit(mut self, layout_history_limit: Option<usize>) -> Self {
        self.layout_history_limit = layout_history_limit;
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn main_pane_height(mut self, main_pane_height: Option<usize>) -> Self {
        self.main_pane_height = main_pane_height;
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn main_pane_width(mut self, main_pane_width: Option<usize>) -> Self {
        self.main_pane_width = main_pane_width;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn mode_attr<S: Into<Cow<'a, str>>>(mut self, mode_attr: Option<S>) -> Self {
        self.mode_attr = mode_attr;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn mode_bg<S: Into<Cow<'a, str>>>(mut self, mode_bg: Option<S>) -> Self {
        self.mode_bg = mode_bg;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn mode_fg<S: Into<Cow<'a, str>>>(mut self, mode_fg: Option<S>) -> Self {
        self.mode_fg = mode_fg;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn mode_keys(mut self, mode_keys: Option<StatusKeys>) -> Self {
        self.mode_keys = mode_keys;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub fn mode_mouse(mut self, mode_mouse: Option<ModeMouse>) -> Self {
        self.mode_mouse = mode_mouse;
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn mode_style<S: Into<Cow<'a, str>>>(mut self, mode_style: Option<S>) -> Self {
        self.mode_style = mode_style.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn monitor_activity(mut self, monitor_activity: Option<Switch>) -> Self {
        self.monitor_activity = monitor_activity;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub fn monitor_content(mut self, monitor_content: Option<usize>) -> Self {
        self.monitor_content = monitor_content;
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn monitor_bell(mut self, monitor_bell: Option<Switch>) -> Self {
        self.monitor_bell = monitor_bell;
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn monitor_silence(mut self, monitor_silence: Option<usize>) -> Self {
        self.monitor_silence = monitor_silence;
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn other_pane_height(mut self, other_pane_height: Option<usize>) -> Self {
        self.other_pane_height = other_pane_height;
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn other_pane_width(mut self, other_pane_width: Option<usize>) -> Self {
        self.other_pane_width = other_pane_width;
        self
    }

    #[cfg(feature = "tmux_2_0")]
    pub fn pane_active_border_style<S: Into<Cow<'a, str>>>(
        mut self,
        pane_active_border_style: Option<S>,
    ) -> Self {
        self.pane_active_border_style = pane_active_border_style.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn pane_base_index(mut self, pane_base_index: Option<usize>) -> Self {
        self.pane_base_index = pane_base_index;
        self
    }

    #[cfg(feature = "tmux_2_3")]
    pub fn pane_border_format<S: Into<Cow<'a, str>>>(
        mut self,
        pane_border_format: Option<S>,
    ) -> Self {
        self.pane_border_format = pane_border_format.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_2_3")]
    pub fn pane_border_status(mut self, pane_border_status: Option<PaneBorderStatus>) -> Self {
        self.pane_border_status = pane_border_status;
        self
    }

    #[cfg(feature = "tmux_2_0")]
    pub fn pane_border_style<S: Into<Cow<'a, str>>>(
        mut self,
        pane_border_style: Option<S>,
    ) -> Self {
        self.pane_border_style = pane_border_style.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    pub fn remain_on_exit(mut self, remain_on_exit: Option<Switch>) -> Self {
        self.remain_on_exit = remain_on_exit;
        self
    }

    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    pub fn synchronize_panes(mut self, synchronize_panes: Option<Switch>) -> Self {
        self.synchronize_panes = synchronize_panes;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub fn utf8(mut self, utf8: Option<Switch>) -> Self {
        self.utf8 = utf8;
        self
    }

    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub fn window_active_style<S: Into<Cow<'a, str>>>(
        mut self,
        window_active_style: Option<S>,
    ) -> Self {
        self.window_active_style = window_active_style.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_bell_attr(mut self, window_status_bell_attr: Option<Switch>) -> Self {
        self.window_status_bell_attr = window_status_bell_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_bell_bg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_bell_bg: Option<S>,
    ) -> Self {
        self.window_status_bell_bg = window_status_bell_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_bell_fg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_bell_fg: Option<S>,
    ) -> Self {
        self.window_status_bell_fg = window_status_bell_fg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_content_attr<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_content_attr: Option<S>,
    ) -> Self {
        self.window_status_content_attr = window_status_content_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_content_bg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_content_bg: Option<S>,
    ) -> Self {
        self.window_status_content_bg = window_status_content_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_content_fg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_content_fg: Option<S>,
    ) -> Self {
        self.window_status_content_fg = window_status_content_fg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_activity_attr<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_activity_attr: Option<S>,
    ) -> Self {
        self.window_status_activity_attr = window_status_activity_attr;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_activity_bg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_activity_bg: Option<S>,
    ) -> Self {
        self.window_status_activity_bg = window_status_activity_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_activity_fg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_activity_fg: Option<S>,
    ) -> Self {
        self.window_status_activity_fg = window_status_activity_fg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_attr<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_attr: Option<S>,
    ) -> Self {
        self.window_status_attr = window_status_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_bg<S: Into<Cow<'a, str>>>(mut self, window_status_bg: Option<S>) -> Self {
        self.window_status_bg = window_status_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_fg<S: Into<Cow<'a, str>>>(mut self, window_status_fg: Option<S>) -> Self {
        self.window_status_fg = window_status_fg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_current_attr<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_current_attr: Option<S>,
    ) -> Self {
        self.window_status_current_attr = window_status_current_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_current_bg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_current_bg: Option<S>,
    ) -> Self {
        self.window_status_current_bg = window_status_current_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_current_fg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_current_fg: Option<S>,
    ) -> Self {
        self.window_status_current_fg = window_status_current_fg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_alert_attr<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_alert_attr: Option<S>,
    ) -> Self {
        self.window_status_alert_attr = window_status_alert_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_alert_bg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_alert_bg: Option<S>,
    ) -> Self {
        self.window_status_alert_bg = window_status_alert_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_alert_fg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_alert_fg: Option<S>,
    ) -> Self {
        self.window_status_alert_fg = window_status_alert_fg.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_activity_style<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_activity_style: Option<S>,
    ) -> Self {
        self.window_status_activity_style = window_status_activity_style.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_bell_style<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_bell_style: Option<S>,
    ) -> Self {
        self.window_status_bell_style = window_status_bell_style.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub fn window_status_content_style<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_content_style: Option<S>,
    ) -> Self {
        self.window_status_content_style = window_status_content_style;
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn window_status_current_format<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_current_format: Option<S>,
    ) -> Self {
        self.window_status_current_format = window_status_current_format.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub fn window_status_last_attr<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_last_attr: Option<S>,
    ) -> Self {
        self.window_status_last_attr = window_status_last_attr;
        self
    }

    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub fn window_status_last_bg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_last_bg: Option<S>,
    ) -> Self {
        self.window_status_last_bg = window_status_last_bg;
        self
    }

    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub fn window_status_last_fg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_last_fg: Option<S>,
    ) -> Self {
        self.window_status_last_fg = window_status_last_fg;
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_current_style<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_current_style: Option<S>,
    ) -> Self {
        self.window_status_current_style = window_status_current_style.map(|s| s.into());
        self
    }
    #[cfg(feature = "tmux_1_2")]
    pub fn window_status_format<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_format: Option<S>,
    ) -> Self {
        self.window_status_format = window_status_format.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_last_style<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_last_style: Option<S>,
    ) -> Self {
        self.window_status_last_style = window_status_last_style.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn window_status_separator<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_separator: Option<S>,
    ) -> Self {
        self.window_status_separator = window_status_separator.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_style<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_style: Option<S>,
    ) -> Self {
        self.window_status_style = window_status_style.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_2_9")]
    pub fn window_size(mut self, window_size: Option<WindowSize>) -> Self {
        self.window_size = window_size;
        self
    }

    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    pub fn word_separators(mut self, word_separators: Option<Switch>) -> Self {
        self.word_separators = word_separators;
        self
    }

    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub fn window_style<S: Into<Cow<'a, str>>>(mut self, window_style: Option<S>) -> Self {
        self.window_style = window_style.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn wrap_search(mut self, wrap_search: Option<Switch>) -> Self {
        self.wrap_search = wrap_search;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn xterm_keys(mut self, xterm_keys: Option<Switch>) -> Self {
        self.xterm_keys = xterm_keys;
        self
    }
}

impl<'a> FromStr for WindowOptions<'a> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut window_options = WindowOptions::new();

        for line in s.lines() {
            if let Some((name, _i, value)) = get_parts(line) {
                match name {
                    #[cfg(feature = "tmux_1_0")]
                    AGGRESSIVE_RESIZE => {
                        window_options.aggressive_resize = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
                    ALLOW_RENAME => {
                        window_options.allow_rename = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
                    ALTERNATE_SCREEN => {
                        window_options.alternate_screen = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")] // 0.8
                    AUTOMATIC_RENAME => {
                        window_options.automatic_rename = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_9")]
                    AUTOMATIC_RENAME_FORMAT => {
                        window_options.automatic_rename_format = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
                    C0_CHANGE_INTERVAL => {
                        window_options.c0_change_interval = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
                    C0_CHANGE_TRIGGER => window_options.c0_change_trigger = cow_parse(value),
                    #[cfg(feature = "tmux_1_0")]
                    CLOCK_MODE_COLOUR => window_options.clock_mode_colour = cow_parse(value),
                    #[cfg(feature = "tmux_1_0")]
                    CLOCK_MODE_STYLE => {
                        window_options.clock_mode_style = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
                    FORCE_HEIGHT => {
                        window_options.force_height = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
                    FORCE_WIDTH => window_options.force_width = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
                    LAYOUT_HISTORY_LIMIT => {
                        window_options.layout_history_limit = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    MAIN_PANE_HEIGHT => {
                        window_options.main_pane_height = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    MAIN_PANE_WIDTH => {
                        window_options.main_pane_width = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    MODE_ATTR => window_options.mode_attr = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    MODE_BG => window_options.mode_bg = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    MODE_FG => window_options.mode_fg = value.and_then(|s| s.parse().ok()),
                    #[cfg(feature = "tmux_1_0")]
                    MODE_KEYS => window_options.mode_keys = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
                    MODE_MOUSE => window_options.mode_mouse = value.and_then(|s| s.parse().ok()),
                    #[cfg(feature = "tmux_1_9")]
                    MODE_STYLE => window_options.mode_style = cow_parse(value),
                    #[cfg(feature = "tmux_1_0")]
                    MONITOR_ACTIVITY => {
                        window_options.monitor_activity = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
                    MONITOR_CONTENT => {
                        window_options.monitor_content = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_2_6")]
                    MONITOR_BELL => {
                        window_options.monitor_bell = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_4")]
                    MONITOR_SILENCE => {
                        window_options.monitor_silence = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_4")]
                    OTHER_PANE_HEIGHT => {
                        window_options.other_pane_height = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_4")]
                    OTHER_PANE_WIDTH => {
                        window_options.other_pane_width = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_2_0")]
                    PANE_ACTIVE_BORDER_STYLE => {
                        window_options.pane_active_border_style = cow_parse(value)
                    }
                    #[cfg(feature = "tmux_1_6")]
                    PANE_BASE_INDEX => {
                        window_options.pane_base_index = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_2_3")]
                    PANE_BORDER_FORMAT => window_options.pane_border_format = cow_parse(value),
                    #[cfg(feature = "tmux_2_3")]
                    PANE_BORDER_STATUS => {
                        window_options.pane_border_status = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_2_0")]
                    PANE_BORDER_STYLE => window_options.pane_border_style = cow_parse(value),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
                    REMAIN_ON_EXIT => {
                        window_options.remain_on_exit = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
                    SYNCHRONIZE_PANES => {
                        window_options.synchronize_panes = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
                    UTF8 => window_options.utf8 = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
                    WINDOW_ACTIVE_STYLE => window_options.window_active_style = cow_parse(value),
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_BELL_ATTR => {
                        window_options.window_status_bell_attr = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_BELL_BG => {
                        window_options.window_status_bell_bg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_BELL_FG => {
                        window_options.window_status_bell_fg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_CONTENT_ATTR => {
                        window_options.window_status_content_attr =
                            value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_CONTENT_BG => {
                        window_options.window_status_content_bg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_CONTENT_FG => {
                        window_options.window_status_content_fg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_ACTIVITY_ATTR => {
                        window_options.window_status_activity_attr =
                            value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_ACTIVITY_BG => {
                        window_options.window_status_activity_bg =
                            value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_ACTIVITY_FG => {
                        window_options.window_status_activity_fg =
                            value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_ATTR => {
                        window_options.window_status_attr = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_BG => {
                        window_options.window_status_bg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_FG => {
                        window_options.window_status_fg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_CURRENT_ATTR => {
                        window_options.window_status_current_attr =
                            value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_CURRENT_BG => {
                        window_options.window_status_current_bg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_CURRENT_FG => {
                        window_options.window_status_current_fg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
                    WINDOW_STATUS_ALERT_ATTR => {
                        window_options.window_status_alert_attr = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
                    WINDOW_STATUS_ALERT_BG => {
                        window_options.window_status_alert_bg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
                    WINDOW_STATUS_ALERT_FG => {
                        window_options.window_status_alert_fg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_9")]
                    WINDOW_STATUS_ACTIVITY_STYLE => {
                        window_options.window_status_activity_style = cow_parse(value)
                    }
                    #[cfg(feature = "tmux_1_9")]
                    WINDOW_STATUS_BELL_STYLE => {
                        window_options.window_status_bell_style = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
                    WINDOW_STATUS_CONTENT_STYLE => {
                        window_options.window_status_content_style =
                            value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_2")]
                    WINDOW_STATUS_CURRENT_FORMAT => {
                        window_options.window_status_current_format = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_LAST_ATTR => {
                        window_options.window_status_last_attr = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_LAST_BG => {
                        window_options.window_status_last_bg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_LAST_FG => {
                        window_options.window_status_last_fg = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_9")]
                    WINDOW_STATUS_CURRENT_STYLE => {
                        window_options.window_status_current_style = cow_parse(value)
                    }
                    #[cfg(feature = "tmux_1_2")]
                    WINDOW_STATUS_FORMAT => window_options.window_status_format = cow_parse(value),
                    #[cfg(feature = "tmux_1_9")]
                    WINDOW_STATUS_LAST_STYLE => {
                        window_options.window_status_last_style = cow_parse(value)
                    }
                    #[cfg(feature = "tmux_1_7")]
                    WINDOW_STATUS_SEPARATOR => {
                        window_options.window_status_separator = cow_parse(value)
                    }
                    #[cfg(feature = "tmux_1_9")]
                    WINDOW_STATUS_STYLE => window_options.window_status_style = cow_parse(value),
                    #[cfg(feature = "tmux_2_9")]
                    WINDOW_SIZE => window_options.window_size = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
                    WORD_SEPARATORS => {
                        window_options.word_separators = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
                    WINDOW_STYLE => window_options.window_style = cow_parse(value),
                    #[cfg(feature = "tmux_1_7")]
                    WRAP_SEARCH => window_options.wrap_search = value.and_then(|s| s.parse().ok()),
                    #[cfg(feature = "tmux_1_0")]
                    XTERM_KEYS => window_options.xterm_keys = value.and_then(|s| s.parse().ok()),
                    _ => {
                        // if user option (@user_option value)
                        if let Some(name) = name.strip_prefix('@') {
                            window_options
                                .user_options
                                .insert(name.to_string(), cow_parse(value));
                        }
                    }
                }
            }
        }

        Ok(window_options)
    }
}
