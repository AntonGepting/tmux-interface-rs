#[test]
fn show_options() {
    use crate::{Error, ShowOptions, ShowOptionsBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.0:
        // ```text
        // tmux show-options [-AgHpqsvw] [-t target-pane] [option]
        // (alias: show)
        // ```
        //
        // tmux ^1.8:
        // ```text
        // tmux show-options [-gqsvw] [-t target-session | target-window] [option]
        // (alias: show)
        // ```
        //
        // tmux ^1.7:
        // ```text
        // tmux show-options [-gsw] [-t target-session | target-window] [option]
        // (alias: show)
        // ```
        //
        // tmux ^1.2:
        // ```text
        // tmux show-options [-gsw] [-t target-session | target-window]
        // (alias: show)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux show-options [-t target-session]
        // (alias: show)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux show-options [-t target-session] option value
        // (alias: show)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("show-options");
        #[cfg(feature = "tmux_3_0")]
        s.push("-A");
        #[cfg(feature = "tmux_1_2")]
        s.push("-g");
        #[cfg(feature = "tmux_3_0")]
        s.push("-H");
        #[cfg(feature = "tmux_3_0")]
        s.push("-p");
        #[cfg(feature = "tmux_1_8")]
        s.push("-q");
        #[cfg(feature = "tmux_1_2")]
        s.push("-s");
        #[cfg(feature = "tmux_1_8")]
        s.push("-v");
        #[cfg(feature = "tmux_1_2")]
        s.push("-w");
        s.extend_from_slice(&["-t", "1"]);
        #[cfg(feature = "tmux_1_7")]
        s.push("2");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let show_options = ShowOptions {
        #[cfg(feature = "tmux_3_0")]
        include_inherited: Some(true),
        #[cfg(feature = "tmux_1_2")]
        global: Some(true),
        #[cfg(feature = "tmux_3_0")]
        hooks: Some(true),
        #[cfg(feature = "tmux_3_0")]
        pane: Some(true),
        #[cfg(feature = "tmux_1_8")]
        quiet: Some(true),
        #[cfg(feature = "tmux_1_2")]
        server: Some(true),
        #[cfg(feature = "tmux_1_8")]
        value: Some(true),
        #[cfg(feature = "tmux_1_2")]
        window: Some(true),
        target: Some(&TargetPane::Raw("1")),
        #[cfg(feature = "tmux_1_7")]
        option: Some("2"),
    };
    tmux.show_options(Some(&show_options)).unwrap_err();

    let mut builder = ShowOptionsBuilder::new();
    #[cfg(feature = "tmux_3_0")]
    builder.include_inherited();
    #[cfg(feature = "tmux_1_2")]
    builder.global();
    #[cfg(feature = "tmux_3_0")]
    builder.hooks();
    #[cfg(feature = "tmux_3_0")]
    builder.pane();
    #[cfg(feature = "tmux_1_8")]
    builder.quiet();
    #[cfg(feature = "tmux_1_2")]
    builder.server();
    #[cfg(feature = "tmux_1_8")]
    builder.value();
    #[cfg(feature = "tmux_1_2")]
    builder.window();
    builder.target(&TargetPane::Raw("1"));
    #[cfg(feature = "tmux_1_7")]
    builder.option("2");
    let show_options = builder.build();
    tmux.show_options(Some(&show_options)).unwrap_err();
}
