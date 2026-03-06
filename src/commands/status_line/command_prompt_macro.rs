// auto-generated file
//

/// Open the command prompt in a client
///
/// # Manual
///
///
/// tmux >=3.6:
/// ```text
/// command-prompt [-1bFiklN] [-I inputs] [-p prompts] [-t target-client] [-T prompt-type] [template]
/// ```
///
/// tmux >=3.3:
/// ```text
/// command-prompt [-1bFikN] [-I inputs] [-p prompts] [-t target-client] [-T prompt-type] [template]
/// ```
///
/// tmux >=3.2:
/// ```text
/// command-prompt [-1ikNTW] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux >=3.1:
/// ```text
/// command-prompt [-1ikN] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux >=3.0a:
/// ```text
/// command-prompt [-1Ni] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux >=2.4:
/// ```text
/// command-prompt [-1i] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux >=1.5:
/// ```text
/// command-prompt [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
///
/// tmux >=0.8:
/// ```text
/// command-prompt [-t target-client] [template]
/// ```
#[macro_export]
macro_rules! command_prompt {
    // `[-1]`
    (@cmd ($cmd:expr) -1, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.one_keypress()
        }) $($tail)*)
    }};

    // `[-b]`
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.background()
        }) $($tail)*)
    }};

    // `[-F]`
    (@cmd ($cmd:expr) -F, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.expand_as_format()
        }) $($tail)*)
    }};

    // `[-i]`
    (@cmd ($cmd:expr) -i, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.on_input_change()
        }) $($tail)*)
    }};

    // `[-k]`
    (@cmd ($cmd:expr) -k, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.key_name()
        }) $($tail)*)
    }};

    // `[-l]`
    (@cmd ($cmd:expr) -l, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.disable_splitting()
        }) $($tail)*)
    }};

    // `[-N]`
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.numeric()
        }) $($tail)*)
    }};

    // `[-T]`
    (@cmd ($cmd:expr) -T, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.prompt_type()
        }) $($tail)*)
    }};

    // `[-W]`
    (@cmd ($cmd:expr) -W, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.for_window()
        }) $($tail)*)
    }};

    // `[-I inputs]`
    (@cmd ($cmd:expr) -I $inputs:expr, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.inputs($inputs)
        }) $($tail)*)
    }};

    // `[-p prompts]`
    (@cmd ($cmd:expr) -p $prompts:expr, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.prompts($prompts)
        }) $($tail)*)
    }};

    // `[-t target-client]`
    (@cmd ($cmd:expr) -t $target_client:expr, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};

    // `[-T prompt-type]`
    (@cmd ($cmd:expr) -T $prompt_type:expr, $($tail:tt)*) => {{
        $crate::command_prompt!(@cmd ({
            $cmd.prompt_type($prompt_type)
        }) $($tail)*)
    }};

    // `[template]`
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

    let command_prompt = command_prompt!();
    #[cfg(feature = "tmux_2_4")]
    let command_prompt = command_prompt!((command_prompt), -1);
    #[cfg(feature = "tmux_3_3")]
    let command_prompt = command_prompt!((command_prompt), -b);
    #[cfg(feature = "tmux_3_3")]
    let command_prompt = command_prompt!((command_prompt), -F);
    #[cfg(feature = "tmux_2_4")]
    let command_prompt = command_prompt!((command_prompt), -i);
    #[cfg(feature = "tmux_3_1")]
    let command_prompt = command_prompt!((command_prompt), -k);
    #[cfg(feature = "tmux_3_6")]
    let command_prompt = command_prompt!((command_prompt), -l);
    #[cfg(feature = "tmux_3_0a")]
    let command_prompt = command_prompt!((command_prompt), -N);
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    let command_prompt = command_prompt!((command_prompt), -T);
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    let command_prompt = command_prompt!((command_prompt), -W);
    #[cfg(feature = "tmux_1_5")]
    let command_prompt = command_prompt!((command_prompt), -I "1");
    #[cfg(feature = "tmux_1_5")]
    let command_prompt = command_prompt!((command_prompt), -p "2");
    #[cfg(feature = "tmux_0_8")]
    let command_prompt = command_prompt!((command_prompt), -t "3");
    #[cfg(feature = "tmux_3_3")]
    let command_prompt = command_prompt!((command_prompt), -T PromptType::Command);
    #[cfg(feature = "tmux_0_8")]
    let command_prompt = command_prompt!((command_prompt), "5");

    let cmd = "command-prompt";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_4")]
    s.push("-1");
    #[cfg(feature = "tmux_3_3")]
    s.push("-b");
    #[cfg(feature = "tmux_3_3")]
    s.push("-F");
    #[cfg(feature = "tmux_2_4")]
    s.push("-i");
    #[cfg(feature = "tmux_3_1")]
    s.push("-k");
    #[cfg(feature = "tmux_3_6")]
    s.push("-l");
    #[cfg(feature = "tmux_3_0a")]
    s.push("-N");
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    s.push("-T");
    #[cfg(all(feature = "tmux_3_2", not(feature = "tmux_3_3")))]
    s.push("-W");
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-I", "1"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-p", "2"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_3_3")]
    s.extend_from_slice(&["-T", "command"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("5");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let command_prompt = command_prompt.build().to_vec();

    assert_eq!(command_prompt, s);
}
