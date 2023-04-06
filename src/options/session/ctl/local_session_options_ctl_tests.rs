#[test]
fn get_single() {
    use crate::{LocalSessionOptionsCtl, SessionOptions, SessionOptionsCtl, Tmux};

    let session_options_ctl = LocalSessionOptionsCtl::default();
    #[cfg(feature = "tmux_2_6")]
    let activity_action = session_options_ctl.get_activity_action().unwrap();

    // let key_table = session_options_ctl.get_key_table().unwrap();

    let origin = SessionOptions::default();
    assert_eq!(activity_action, origin.activity_action);

    // assert_eq!(key_table, origin.key_table);
}

#[test]
fn local_session_options_ctl_all() {
    use crate::{LocalSessionOptionsCtl, SessionOptions, SessionOptionsCtl, Tmux};

    // let session_options = LocalSessionOptionsCtl::default().get_all().unwrap();
    let session_options =
        LocalSessionOptionsCtl::new(Some(""), |cmd| Tmux::with_command(cmd).output())
            .get_all()
            .unwrap();

    let origin = SessionOptions::default();
    assert_eq!(origin, session_options);
}
