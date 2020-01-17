#[test]
fn save_buffer() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux save-buffer [-a] [-b buffer-name] path
        // (alias: saveb)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["save-buffer", "-a", "-b", "1", "2"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.save_buffer(Some(true), Some("1"), "2").unwrap_err();
}
