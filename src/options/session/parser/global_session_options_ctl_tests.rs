#[test]
fn global_session_options_ctl() {
    use crate::{GlobalSessionOptionsCtl, SessionOptionsCtl};

    let session_options_ctl = GlobalSessionOptionsCtl::default();
    #[cfg(feature = "tmux_2_6")]
    let value = session_options_ctl.get_activity_action(Some(":")).unwrap();

    // assert_eq!(value, Some(crate::Action::Other));
}

#[test]
fn global_session_options_ctl_all() {
    use crate::{GlobalSessionOptionsCtl, SessionOptionsCtl};

    let session_options = GlobalSessionOptionsCtl::default().get_all().unwrap();

    // assert_eq!(session_options.activity_action, Some(crate::Action::Other));
}
