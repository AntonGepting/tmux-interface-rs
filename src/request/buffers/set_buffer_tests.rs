#[test]
fn set_buffer() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux set-buffer [-a] [-b buffer-name] [-n new-buffer-name] data
        // (alias: setb)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["set-buffer", "-a", "-b", "1", "-n", "2", "3"]"#
        );
        Err(Error::Hook)
    }));
    tmux.set_buffer(Some(true), Some("1"), Some("2"), "3")
        .unwrap_err();
}
