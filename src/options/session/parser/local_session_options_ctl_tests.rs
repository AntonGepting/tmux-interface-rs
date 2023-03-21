#[test]
fn local_session_options_ctl() {
    use crate::{LocalSessionOptionsCtl, SessionOptionsCtl};

    let session_options_ctl = LocalSessionOptionsCtl::default();
    #[cfg(feature = "tmux_2_6")]
    let value = session_options_ctl.get_activity_action(Some(":")).unwrap();

    assert_eq!(value, None);
}

#[test]
fn local_session_options_ctl_all() {
    use crate::{LocalSessionOptionsCtl, SessionOptionsCtl};

    let session_options = LocalSessionOptionsCtl::default().get_all().unwrap();

    assert_eq!(session_options.activity_action, None);
}
