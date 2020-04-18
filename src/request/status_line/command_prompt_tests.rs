#[test]
fn command_prompt() {
    use crate::{CommandPrompt, CommandPromptBuilder, Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux command-prompt [-1i] [-I inputs] [-p prompts] [-t target-client] [template]
        // (alias: lockc)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["command-prompt", "-1", "-i", "-I", "1", "-p", "2", "-t", "3", "4"]"#
        );
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("command-prompt");
        #[cfg(feature = "tmux_2_4")]
        s.push("-1");
        #[cfg(feature = "tmux_2_4")]
        s.push("-i");
        #[cfg(feature = "tmux_1_5")]
        s.extend_from_slice(&["-I", "1"]);
        #[cfg(feature = "tmux_1_0")]
        s.extend_from_slice(&["-p", "2"]);
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "3"]);
        #[cfg(feature = "tmux_0_8")]
        s.push("4");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let command_prompt = CommandPrompt {
        #[cfg(feature = "tmux_2_4")]
        one_keypress: Some(true),
        #[cfg(feature = "tmux_2_4")]
        on_input_change: Some(true),
        #[cfg(feature = "tmux_1_5")]
        inputs: Some("1"),
        #[cfg(feature = "tmux_1_0")]
        prompts: Some("2"),
        #[cfg(feature = "tmux_0_8")]
        target_client: Some("3"),
        #[cfg(feature = "tmux_0_8")]
        template: Some("4"),
    };
    tmux.command_prompt(Some(&command_prompt)).unwrap_err();

    let mut builder = CommandPromptBuilder::new();
    #[cfg(feature = "tmux_2_4")]
    builder.one_keypress();
    #[cfg(feature = "tmux_2_4")]
    builder.on_input_change();
    #[cfg(feature = "tmux_1_5")]
    builder.inputs("1");
    #[cfg(feature = "tmux_1_0")]
    builder.prompts("2");
    #[cfg(feature = "tmux_0_8")]
    builder.target_client("3");
    #[cfg(feature = "tmux_0_8")]
    builder.template("4");
    let command_prompt = builder.build();
    tmux.command_prompt(Some(&command_prompt)).unwrap_err();
}
