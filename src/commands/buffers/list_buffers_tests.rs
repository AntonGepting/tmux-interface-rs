#[test]
fn list_buffers() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux list-buffers [-F format]
        // (alias: lsb)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["list-buffers", "-F", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.list_buffers(Some("1")).unwrap_err();
}
