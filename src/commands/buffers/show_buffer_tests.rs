#[test]
fn show_buffer() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux show-buffer [-b buffer-name]
        // (alias: showb)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["show-buffer", "-b", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.show_buffer(Some("1")).unwrap_err();
}
