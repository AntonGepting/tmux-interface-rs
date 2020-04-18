#[test]
fn set_hook() {
    use crate::{Error, SetHook, SetHookBuilder, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^3.0:
        // ```text
        // tmux set-hook [-agRu] [-t target-session] hook-name command
        // ```
        //
        // tmux ^2.8:
        // ```text
        // tmux set-hook [-gRu] [-t target-session] hook-name command
        // ```
        //
        // tmux ^2.4:
        // ```text
        // tmux set-hook [-gu] [-t target-session] hook-name command
        // ```
        //
        // tmux ^2.2:
        // ```text
        // tmux set-hook [-g] [-t target-session] hook-name command
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("set-hook");
        #[cfg(feature = "tmux_3_0")]
        s.push("-a");
        #[cfg(feature = "tmux_2_2")]
        s.push("-g");
        #[cfg(feature = "tmux_2_8")]
        s.push("-R");
        #[cfg(feature = "tmux_2_4")]
        s.push("-u");
        #[cfg(feature = "tmux_2_2")]
        s.extend_from_slice(&["-t", "1"]);
        s.push("2");
        s.push("3");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let set_hook = SetHook {
        #[cfg(feature = "tmux_3_0")]
        append: Some(true),
        #[cfg(feature = "tmux_2_2")]
        global: Some(true),
        #[cfg(feature = "tmux_2_8")]
        run: Some(true),
        #[cfg(feature = "tmux_2_4")]
        unset: Some(true),
        #[cfg(feature = "tmux_2_2")]
        target_session: Some(&TargetSession::Raw("1")),
    };
    tmux.set_hook(Some(&set_hook), "2", "3").unwrap_err();

    let mut builder = SetHookBuilder::new();
    #[cfg(feature = "tmux_3_0")]
    builder.append();
    #[cfg(feature = "tmux_2_2")]
    builder.global();
    #[cfg(feature = "tmux_2_8")]
    builder.run();
    #[cfg(feature = "tmux_2_4")]
    builder.unset();
    #[cfg(feature = "tmux_2_2")]
    builder.target_session(&TargetSession::Raw("1"));
    let set_hook = builder.build();
    tmux.set_hook(Some(&set_hook), "2", "3").unwrap_err();
}
