/// Structure for open the command prompt in a client
///
/// # Manual
///
/// tmux ^3.3:
/// ```text
/// command-prompt [-1bFikN] [-I inputs] [-p prompts] [-t target-client] [-T prompt-type] [template]
/// ```
///
/// tmux ^3.2:
/// ```text
/// command-prompt [-1ikNTW] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux ^3.1:
/// ```text
/// command-prompt [-1ikN] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux ^3.0:
/// ```text
/// command-prompt [-1Ni] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux ^2.4:
/// ```text
/// command-prompt [-1i] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux ^1.5:
/// ```text
/// command-prompt [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux ^1.0:
/// ```text
/// command-prompt [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux ^0.8:
/// ```text
/// command-prompt [-t target-client] [template]
/// ```
#[macro_export]
macro_rules! command_prompt {
    // `[-1]` - makes the prompt only accept one key press
    (@cmd ($cmd:expr) -1, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.one_keypress()
        }) $($tail)*)
    }};
    // `[-b]` - the prompt is shown in the background and the invoking client does not exit until it is dismissed
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.background()
        }) $($tail)*)
    }};
    // `[-F]` -
    (@cmd ($cmd:expr) -F, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.expand_as_format()
        }) $($tail)*)
    }};
    // `[-i]` execute the command every time the prompt input changes
    (@cmd ($cmd:expr) -i, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.on_input_change()
        }) $($tail)*)
    }};
    // `[-k]` - the key press is translated to a key name
    (@cmd ($cmd:expr) -k, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.key_name()
        }) $($tail)*)
    }};
    // `[-N]` - makes the prompt only accept numeric key presses
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.numeric()
        }) $($tail)*)
    }};
    // `[-T]` - tells tmux that the prompt is for a target which affects what completions are offered when Tab is pressed
    (@cmd ($cmd:expr) -T, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.for_target()
        }) $($tail)*)
    }};
    // `[-W]` - indicates the prompt is for a window.
    (@cmd ($cmd:expr) -W, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.for_window()
        }) $($tail)*)
    }};
    // `[-T prompt-type]` - tells tmux the prompt type. This affects what completions are offered when Tab is pressed
    (@cmd ($cmd:expr) -T $prompt_type:expr, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.prompt_type($prompt_type)
        }) $($tail)*)
    }};
    // `[-I inputs]` - comma-separated list of the initial text for each prompt
    (@cmd ($cmd:expr) -I $inputs:expr, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.inputs($inputs)
        }) $($tail)*)
    }};
    // `[-p prompts]` - prompts is a comma-separated list of prompts which are displayed in order
    (@cmd ($cmd:expr) -p $prompts:expr, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.prompts($prompts)
        }) $($tail)*)
    }};
    // `[-t target-client]` - target-client
    (@cmd ($cmd:expr) -t $target_client:expr, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};
    // `[template]` - template
    (@cmd ($cmd:expr) $template:expr, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.template($template)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::CommandPrompt::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({ $crate::CommandPrompt::new() }) $($tail)*,)
    }};
}

#[test]
fn command_prompt_macro() {
    #[cfg(feature = "tmux_3_3")]
    use crate::commands::PromptType;
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
    let command_prompt = command_prompt!();
    #[cfg(feature = "tmux_2_4")]
    let command_prompt = command_prompt!((command_prompt), -1);
    #[cfg(feature = "tmux_2_4")]
    let command_prompt = command_prompt!((command_prompt), -i);
    #[cfg(feature = "tmux_3_1")]
    let command_prompt = command_prompt!((command_prompt), -k);
    #[cfg(feature = "tmux_3_1")]
    let command_prompt = command_prompt!((command_prompt), -N);
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    let command_prompt = command_prompt!((command_prompt), -T);
    #[cfg(feature = "tmux_3_3")]
    let command_prompt = command_prompt!((command_prompt), -T PromptType::Command);
    #[cfg(feature = "tmux_3_2")]
    let command_prompt = command_prompt!((command_prompt), -W);
    #[cfg(feature = "tmux_1_5")]
    let command_prompt = command_prompt!((command_prompt), -I "1");
    #[cfg(feature = "tmux_1_0")]
    let command_prompt = command_prompt!((command_prompt), -p "2");
    #[cfg(feature = "tmux_0_8")]
    let command_prompt = command_prompt!((command_prompt), -t "3");
    #[cfg(feature = "tmux_0_8")]
    let command_prompt = command_prompt!((command_prompt), "4");

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
