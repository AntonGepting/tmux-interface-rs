#[test]
fn get_single() {
    use crate::{PaneOptions, PaneOptionsCtl};
    use std::collections::HashMap;

    let pane_options_ctl = PaneOptionsCtl::default();

    #[cfg(feature = "tmux_3_0")]
    let allow_rename = pane_options_ctl.get_allow_rename().unwrap();
    #[cfg(feature = "tmux_3_0")]
    let alternate_screen = pane_options_ctl.get_alternate_screen().unwrap();
    #[cfg(feature = "tmux_3_0")]
    let remain_on_exit = pane_options_ctl.get_remain_on_exit().unwrap();
    #[cfg(feature = "tmux_3_0")]
    let window_active_style = pane_options_ctl.get_window_active_style().unwrap();
    dbg!(&window_active_style);
    #[cfg(feature = "tmux_3_0")]
    let window_style = pane_options_ctl.get_window_style().unwrap();
    dbg!(&window_style);
    #[cfg(feature = "tmux_3_2")]
    let synchronize_panes = pane_options_ctl.get_synchronize_panes().unwrap();

    let pane_options = PaneOptions {
        #[cfg(feature = "tmux_3_0")]
        allow_rename: allow_rename,
        #[cfg(feature = "tmux_3_0")]
        alternate_screen: alternate_screen,
        #[cfg(feature = "tmux_3_0")]
        remain_on_exit: remain_on_exit,
        #[cfg(feature = "tmux_3_0")]
        window_active_style: window_active_style.and_then(|s| Some(s.into())),
        #[cfg(feature = "tmux_3_0")]
        window_style: window_style.and_then(|s| Some(s.into())),
        #[cfg(feature = "tmux_3_2")]
        synchronize_panes: synchronize_panes,

        user_options: HashMap::new(),
    };

    dbg!(pane_options);
}

#[test]
fn get_all() {
    use crate::PaneOptionsCtl;

    let pane_options = PaneOptionsCtl::default().get_all().unwrap();
    dbg!(pane_options);
}

#[test]
fn get_all_ext() {
    use crate::{PaneOptionsCtl, Tmux, TmuxCommand};

    let pane_options_ctl = PaneOptionsCtl::with_invoker(&|cmd: TmuxCommand| {
        Tmux::new().command(cmd.to_owned()).output()
    });

    let pane_options = pane_options_ctl.get_all().unwrap();

    dbg!(pane_options);
}
