#[test]
fn paste_buffer() {
    use crate::{Error, PasteBuffer, PasteBufferBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
        // (alias: pasteb)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["paste-buffer", "-d", "-p", "-r", "-b", "1", "-s", "2", "-t", "3"]"#
        );
        Err(Error::Hook)
    }));

    let paste_buffer = PasteBuffer {
        delete: Some(true),
        bracket_codes: Some(true),
        no_replacement: Some(true),
        buffer_name: Some("1"),
        separator: Some("2"),
        target_pane: Some(&TargetPane::Raw("3")),
    };
    tmux.paste_buffer(Some(&paste_buffer)).unwrap_err();

    let paste_buffer = PasteBufferBuilder::new()
        .delete()
        .bracket_codes()
        .no_replacement()
        .buffer_name("1")
        .separator("2")
        .target_pane(&TargetPane::Raw("3"))
        .build();
    tmux.paste_buffer(Some(&paste_buffer)).unwrap_err();
}
