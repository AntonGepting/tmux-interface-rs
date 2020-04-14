#[test]
fn show_options() {
    use crate::{Error, ShowOptions, ShowOptionsBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux show-options [-AgHpqsvw] [-t target-pane] [option]
        // (alias: show)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["show-options", "-A", "-g", "-H", "-p", "-q", "-s", "-v", "-w", "-t", "1", "2"]"#
        );
        Err(Error::Hook)
    }));

    let show_options = ShowOptions {
        include_inherited: Some(true),
        global: Some(true),
        hooks: Some(true),
        pane: Some(true),
        quiet: Some(true),
        server: Some(true),
        value: Some(true),
        window: Some(true),
        target: Some(&TargetPane::Raw("1")),
        option: Some("2"),
    };
    tmux.show_options(Some(&show_options)).unwrap_err();

    let show_options = ShowOptionsBuilder::new()
        .include_inherited()
        .global()
        .hooks()
        .pane()
        .quiet()
        .server()
        .value()
        .window()
        .target(&TargetPane::Raw("1"))
        .option("2")
        .build();
    tmux.show_options(Some(&show_options)).unwrap_err();
}
