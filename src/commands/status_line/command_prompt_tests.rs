#[test]
fn command_prompt() {
    use crate::CommandPrompt;
    use std::borrow::Cow;

    // Structure for open the command prompt in a client
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // tmux command-prompt [-1ikNTW] [-I inputs] [-p prompts] [-t target-client] [template]
    // ```
    //
    // tmux ^3.1:
    // ```text
    // tmux command-prompt [-1ikN] [-I inputs] [-p prompts] [-t target-client] [template]
    // ```
    //
    // tmux ^3.0:
    // ```text
    // tmux command-prompt [-1Ni] [-I inputs] [-p prompts] [-t target-client] [template]
    // ```
    //
    // tmux ^2.4:
    // ```text
    // tmux command-prompt [-1i] [-I inputs] [-p prompts] [-t target-client] [template]
    // ```
    //
    // tmux ^1.5:
    // ```text
    // tmux command-prompt [-I inputs] [-p prompts] [-t target-client] [template]
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux command-prompt [-p prompts] [-t target-client] [template]
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux command-prompt [-t target-client] [template]
    // ```
    let mut command_prompt = CommandPrompt::new();
    #[cfg(feature = "tmux_2_4")]
    command_prompt.one_keypress();
    #[cfg(feature = "tmux_2_4")]
    command_prompt.on_input_change();
    #[cfg(feature = "tmux_3_1")]
    command_prompt.key_name();
    #[cfg(feature = "tmux_3_1")]
    command_prompt.numeric();
    #[cfg(feature = "tmux_3_2")]
    command_prompt.for_target();
    #[cfg(feature = "tmux_3_2")]
    command_prompt.for_window();
    #[cfg(feature = "tmux_1_5")]
    command_prompt.inputs("1");
    #[cfg(feature = "tmux_1_0")]
    command_prompt.prompts("2");
    #[cfg(feature = "tmux_0_8")]
    command_prompt.target_client("3");
    #[cfg(feature = "tmux_0_8")]
    command_prompt.template("4");

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
    #[cfg(feature = "tmux_3_2")]
    s.push("-T");
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
