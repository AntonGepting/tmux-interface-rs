#[test]
fn command_prompt() {
    #[cfg(feature = "tmux_3_3")]
    use crate::commands::PromptType;
    use crate::CommandPrompt;
    use std::borrow::Cow;

    // Structure for open the command prompt in a client
    //
    // # Manual
    //
    // tmux ^3.3:
    // ```text
    // command-prompt [-1bFikN] [-I inputs] [-p prompts] [-t target-client] [-T prompt-type] [template]
    // ```
    //
    // tmux ^3.2:
    // ```text
    // command-prompt [-1ikNTW] [-I inputs] [-p prompts] [-t target-client] [template]
    // ```
    //
    // tmux ^3.1:
    // ```text
    // command-prompt [-1ikN] [-I inputs] [-p prompts] [-t target-client] [template]
    // ```
    //
    // tmux ^3.0:
    // ```text
    // command-prompt [-1Ni] [-I inputs] [-p prompts] [-t target-client] [template]
    // ```
    //
    // tmux ^2.4:
    // ```text
    // command-prompt [-1i] [-I inputs] [-p prompts] [-t target-client] [template]
    // ```
    //
    // tmux ^1.5:
    // ```text
    // command-prompt [-I inputs] [-p prompts] [-t target-client] [template]
    // ```
    //
    // tmux ^1.0:
    // ```text
    // command-prompt [-p prompts] [-t target-client] [template]
    // ```
    //
    // tmux ^0.8:
    // ```text
    // command-prompt [-t target-client] [template]
    // ```
    let command_prompt = CommandPrompt::new();
    #[cfg(feature = "tmux_2_4")]
    let command_prompt = command_prompt.one_keypress();
    #[cfg(feature = "tmux_2_4")]
    let command_prompt = command_prompt.on_input_change();
    #[cfg(feature = "tmux_3_1")]
    let command_prompt = command_prompt.key_name();
    #[cfg(feature = "tmux_3_1")]
    let command_prompt = command_prompt.numeric();
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    let command_prompt = command_prompt.for_target();
    #[cfg(feature = "tmux_3_3")]
    let command_prompt = command_prompt.prompt_type(PromptType::Command);
    #[cfg(feature = "tmux_3_2")]
    let command_prompt = command_prompt.for_window();
    #[cfg(feature = "tmux_1_5")]
    let command_prompt = command_prompt.inputs("1");
    #[cfg(feature = "tmux_1_0")]
    let command_prompt = command_prompt.prompts("2");
    #[cfg(feature = "tmux_0_8")]
    let command_prompt = command_prompt.target_client("3");
    #[cfg(feature = "tmux_0_8")]
    let command_prompt = command_prompt.template("4");

    let cmd = "command-prompt";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_4")]
    s.push("-1");
    #[cfg(feature = "tmux_2_4")]
    s.push("-i");
    #[cfg(feature = "tmux_3_1")]
    s.push("-k");
    #[cfg(feature = "tmux_3_1")]
    s.push("-N");
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    s.push("-T");
    #[cfg(feature = "tmux_3_3")]
    s.extend_from_slice(&["-T", "command"]);
    #[cfg(feature = "tmux_3_2")]
    s.push("-W");
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-I", "1"]);
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-p", "2"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let command_prompt = command_prompt.build().to_vec();

    assert_eq!(command_prompt, s);
}
