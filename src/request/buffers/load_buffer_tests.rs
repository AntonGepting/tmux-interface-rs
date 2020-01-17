#[test]
fn load_buffer() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux load-buffer [-b buffer-name] path
        // (alias: loadb)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["load-buffer", "-b", "1", "2"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.load_buffer(Some("1"), "2").unwrap_err();
}
