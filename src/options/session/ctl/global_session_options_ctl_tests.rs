#[test]
fn get() {
    use crate::{GlobalSessionOptionsCtl, SessionOptionsCtl};

    let session_options_ctl = GlobalSessionOptionsCtl::default();
    #[cfg(feature = "tmux_2_6")]
    let value = session_options_ctl.get_activity_action().unwrap();

    dbg!(value);
    // assert_eq!(value, Some(crate::Action::Other));
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
