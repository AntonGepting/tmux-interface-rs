// auto-generated file
//

// Open the command prompt in a client
//
// # Manual
//
//
// tmux >=3.6:
// ```text
// command-prompt [-1bFiklN] [-I inputs] [-p prompts] [-t target-client] [-T prompt-type] [template]
// ```
//
// tmux >=3.3:
// ```text
// command-prompt [-1bFikN] [-I inputs] [-p prompts] [-t target-client] [-T prompt-type] [template]
// ```
//
// tmux >=3.2:
// ```text
// command-prompt [-1ikNTW] [-I inputs] [-p prompts] [-t target-client] [template]
// ```
//
// tmux >=3.1:
// ```text
// command-prompt [-1ikN] [-I inputs] [-p prompts] [-t target-client] [template]
// ```
//
// tmux >=3.0a:
// ```text
// command-prompt [-1Ni] [-I inputs] [-p prompts] [-t target-client] [template]
// ```
//
// tmux >=2.4:
// ```text
// command-prompt [-1i] [-I inputs] [-p prompts] [-t target-client] [template]
// ```
//
// tmux >=1.5:
// ```text
// command-prompt [-I inputs] [-p prompts] [-t target-client] [template]
// ```
//
// tmux >=0.8:
// ```text
// command-prompt [-t target-client] [template]
// ```
#[test]
fn command_prompt() {
    #[cfg(feature = "tmux_3_3")]
    use crate::commands::PromptType;
    use crate::CommandPrompt;
    use std::borrow::Cow;

    let command_prompt = CommandPrompt::new();
    // `[-1]`
    #[cfg(feature = "tmux_2_4")]
    let command_prompt = command_prompt.one_keypress();

    // `[-b]`
    #[cfg(feature = "tmux_3_3")]
    let command_prompt = command_prompt.background();

    // `[-F]`
    #[cfg(feature = "tmux_3_3")]
    let command_prompt = command_prompt.expand_as_format();

    // `[-i]`
    #[cfg(feature = "tmux_2_4")]
    let command_prompt = command_prompt.on_input_change();

    // `[-k]`
    #[cfg(feature = "tmux_3_1")]
    let command_prompt = command_prompt.key_name();

    // `[-l]`
    #[cfg(feature = "tmux_3_6")]
    let command_prompt = command_prompt.disable_splitting();

    // `[-N]`
    #[cfg(feature = "tmux_3_0a")]
    let command_prompt = command_prompt.numeric();

    // `[-T]`
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    let command_prompt = command_prompt.prompt_type();

    // `[-W]`
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    let command_prompt = command_prompt.for_window();

    // `[-I inputs]`
    #[cfg(feature = "tmux_1_5")]
    let command_prompt = command_prompt.inputs("1");

    // `[-p prompts]`
    #[cfg(feature = "tmux_1_5")]
    let command_prompt = command_prompt.prompts("2");

    // `[-t target-client]`
    #[cfg(feature = "tmux_0_8")]
    let command_prompt = command_prompt.target_client("3");

    // `[-T prompt-type]`
    #[cfg(feature = "tmux_3_3")]
    let command_prompt = command_prompt.prompt_type(PromptType::Command);

    // `[template]`
    #[cfg(feature = "tmux_0_8")]
    let command_prompt = command_prompt.template("5");

    let cmd = "command-prompt";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_4")]
    v.push("-1");
    #[cfg(feature = "tmux_3_3")]
    v.push("-b");
    #[cfg(feature = "tmux_3_3")]
    v.push("-F");
    #[cfg(feature = "tmux_2_4")]
    v.push("-i");
    #[cfg(feature = "tmux_3_1")]
    v.push("-k");
    #[cfg(feature = "tmux_3_6")]
    v.push("-l");
    #[cfg(feature = "tmux_3_0a")]
    v.push("-N");
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    v.push("-T");
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    v.push("-W");
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-I", "1"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-p", "2"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_3_3")]
    v.extend_from_slice(&["-T", "command"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("5");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let command_prompt = command_prompt.build().to_vec();

    assert_eq!(command_prompt, v);
}
