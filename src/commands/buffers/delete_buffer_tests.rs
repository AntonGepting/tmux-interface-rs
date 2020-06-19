#[test]
fn delete_buffer() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux delete-buffer [-b buffer-name]
        // (alias: deleteb)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["delete-buffer", "-b", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.delete_buffer(Some("1")).unwrap_err();
}
