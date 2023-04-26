#[test]
fn get_single() {
    #[cfg(feature = "tmux_2_6")]
    use crate::SessionOptionsCtl;
    use crate::{GlobalSessionOptionsCtl, SessionOptions};

    let session_options_ctl = GlobalSessionOptionsCtl::default();
    #[cfg(feature = "tmux_2_6")]
    let activity_action = session_options_ctl.get_activity_action().unwrap();

    // let key_table = session_options_ctl.get_key_table().unwrap();

    let origin = SessionOptions::default();

    #[cfg(feature = "tmux_2_6")]
    assert_eq!(activity_action, origin.activity_action);

    // assert_eq!(key_table, origin.key_table);
}

#[test]
fn get_all() {
    use crate::{GlobalSessionOptionsCtl, SessionOptionsCtl, Tmux};

    // let session_options = GlobalSessionOptionsCtl::default().get_all().unwrap();

    let session_options = GlobalSessionOptionsCtl::new(&|cmd| Tmux::with_command(cmd).output())
        .get_all()
        .unwrap();

    dbg!(session_options);
    // assert_eq!(session_options.activity_action, Some(crate::Action::Other));
}
