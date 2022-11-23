/// # Manual
///
/// tmux ^3.3:
/// ```text
/// show-prompt-history [-T prompt-type]
/// (alias: showhist)
/// ```
#[macro_export]
macro_rules! show_prompt_history {
    // `[-T prompt-type]`
    (@cmd ($cmd:expr) -T $prompt_type:expr, $($tail:tt)*) => {{
        $crate::show_prompt_history!(@cmd ({
            $cmd.prompt_type($prompt_type)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ShowPromptHistory::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::show_prompt_history!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::show_prompt_history!(@cmd ({ $crate::ShowPromptHistory::new() }) $($tail)*,)
    }};
}

#[test]
fn show_prompt_history_macro() {
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.3:
    // ```text
    // clear-prompt-history [-T prompt-type]
    // (alias: clearphist)
    // ```
    let clear_prompt_history = show_prompt_history!();
    #[cfg(feature = "tmux_3_3")]
    let clear_prompt_history = clear_prompt_history!((show_prompt_history), -T "1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "clear-prompt-history";
    #[cfg(feature = "cmd_alias")]
    let cmd = "clearphist";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-T", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let clear_prompt_history = clear_prompt_history.build().to_vec();

    assert_eq!(clear_prompt_history, s);
}
