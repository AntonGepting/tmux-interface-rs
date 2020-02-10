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
        Err(Error::new("hook"))
    }));

    let command_prompt = CommandPrompt {
        one_keypress: Some(true),
        on_input_change: Some(true),
        inputs: Some("1"),
        prompts: Some("2"),
        target_client: Some("3"),
        template: Some("4"),
    };
    tmux.command_prompt(Some(&command_prompt)).unwrap_err();

    let command_prompt = CommandPromptBuilder::new()
        .one_keypress()
        .on_input_change()
        .inputs("1")
        .prompts("2")
        .target_client("3")
        .template("4")
        .build();
    tmux.command_prompt(Some(&command_prompt)).unwrap_err();
}
