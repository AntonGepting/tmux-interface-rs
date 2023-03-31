#[test]
fn pane_options_ctl() {
    use crate::{PaneOptionsCtl, Tmux, TmuxCommand};

    let pane_options_ctl = PaneOptionsCtl::new(None, |cmd: TmuxCommand| {
        Tmux::new().command(cmd.to_owned()).output()
    });

    let pane_options = pane_options_ctl.get_all().unwrap();
    dbg!(pane_options);

    #[cfg(feature = "tmux_3_0")]
    let allow_rename = pane_options_ctl.get_allow_rename().unwrap();
    #[cfg(feature = "tmux_3_0")]
    let alternate_screen = pane_options_ctl.get_alternate_screen().unwrap();
    #[cfg(feature = "tmux_3_0")]
    let remain_on_exit = pane_options_ctl.get_remain_on_exit().unwrap();
    #[cfg(feature = "tmux_3_0")]
    let window_active_style = pane_options_ctl.get_window_active_style().unwrap();
    #[cfg(feature = "tmux_3_0")]
    let window_style = pane_options_ctl.get_window_style().unwrap();
    #[cfg(feature = "tmux_3_2")]
    let synchronize_panes = pane_options_ctl.get_synchronize_panes().unwrap();

    dbg!(
        allow_rename,
        alternate_screen,
        remain_on_exit,
        window_active_style,
        window_style,
        synchronize_panes
    );
}
