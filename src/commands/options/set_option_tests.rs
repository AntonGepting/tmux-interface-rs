#[test]
fn set_option() {
    use crate::{Error, SetOption, SetOptionBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.0:
        // ```text
        // tmux set-option [-aFgopqsuw] [-t target-pane] option value
        // (alias: set)
        // ```
        //
        // tmux ^2.6:
        // ```text
        // tmux set-option [-aFgoqsuw] [-t target-session | target-window] option value
        // (alias: set)
        // ```
        //
        // tmux ^1.8:
        // ```text
        // tmux set-option [-agoqsuw] [-t target-session | target-window] option value
        // (alias: set)
        // ```
        //
        // tmux ^1.7:
        // ```text
        // tmux set-option [-agqsuw] [-t target-session | target-window] option value
        // (alias: set)
        // ```
        //
        // tmux ^1.2:
        // ```text
        // tmux set-option [-agsuw] [-t target-session | target-window] option value
        // (alias: set)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux set-option [-agu] [-t target-session] option value
        // (alias: set)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux set-option [-gu] [-t target-session] option value
        // (alias: set)
        // ```
        // FIXME: target, target-sesion, target-window
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("set-option");
        #[cfg(feature = "tmux_1_0")]
        s.push("-a");
        #[cfg(feature = "tmux_2_6")]
        s.push("-F");
        #[cfg(feature = "tmux_0_8")]
        s.push("-g");
        #[cfg(feature = "tmux_1_8")]
        s.push("-o");
        #[cfg(feature = "tmux_3_0")]
        s.push("-p");
        #[cfg(feature = "tmux_1_7")]
        s.push("-q");
        #[cfg(feature = "tmux_1_2")]
        s.push("-s");
        #[cfg(feature = "tmux_0_8")]
        s.push("-u");
        #[cfg(feature = "tmux_1_2")]
        s.push("-w");
        #[cfg(feature = "tmux_3_0")]
        s.extend_from_slice(&["-t", "1"]);
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
        s.extend_from_slice(&["-t", "1"]);
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
        s.extend_from_slice(&["-t", "1"]);
        s.push("2");
        s.push("3");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let target_pane = TargetPane::Raw("1").to_string();
    let set_option = SetOption {
        #[cfg(feature = "tmux_1_0")]
        append: Some(true),
        #[cfg(feature = "tmux_2_6")]
        format: Some(true),
        #[cfg(feature = "tmux_0_8")]
        global: Some(true),
        #[cfg(feature = "tmux_1_8")]
        not_overwrite: Some(true),
        #[cfg(feature = "tmux_3_0")]
        pane: Some(true),
        #[cfg(feature = "tmux_1_7")]
        quiet: Some(true),
        #[cfg(feature = "tmux_1_2")]
        server: Some(true),
        #[cfg(feature = "tmux_0_8")]
        unset: Some(true),
        #[cfg(feature = "tmux_1_2")]
        window: Some(true),
        #[cfg(feature = "tmux_3_0")]
        target_pane: Some(&target_pane),
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
        target: Some(&target_pane),
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
        target_session: Some(&target_pane),
    };
    tmux.set_option(Some(&set_option), "2", "3").unwrap_err();

    let mut builder = SetOptionBuilder::new();
    #[cfg(feature = "tmux_1_0")]
    builder.append();
    #[cfg(feature = "tmux_2_6")]
    builder.format();
    #[cfg(feature = "tmux_0_8")]
    builder.global();
    #[cfg(feature = "tmux_1_8")]
    builder.not_overwrite();
    #[cfg(feature = "tmux_3_0")]
    builder.pane();
    #[cfg(feature = "tmux_1_7")]
    builder.quiet();
    #[cfg(feature = "tmux_1_2")]
    builder.server();
    #[cfg(feature = "tmux_0_8")]
    builder.unset();
    #[cfg(feature = "tmux_1_2")]
    builder.window();
    #[cfg(feature = "tmux_3_0")]
    builder.target_pane(&target_pane);
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    builder.target(&target_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_2")))]
    builder.target_session(&target_pane);

    let set_option = builder.build();
    tmux.set_option(Some(&set_option), "2", "3").unwrap_err();
}
