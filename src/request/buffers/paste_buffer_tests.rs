#[test]
fn paste_buffer() {
    use crate::{Error, PasteBuffer, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
        // (alias: pasteb)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["paste-buffer", "-d", "-p", "-r", "-b", "1", "-s", "2", "-t", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    let paste_buffer = PasteBuffer {
        delete: Some(true),
        bracket_codes: Some(true),
        no_replacement: Some(true),
        buffer_name: Some("1"),
        separator: Some("2"),
        target_pane: Some("3"),
    };
    tmux.paste_buffer(Some(&paste_buffer)).unwrap_err();
}
