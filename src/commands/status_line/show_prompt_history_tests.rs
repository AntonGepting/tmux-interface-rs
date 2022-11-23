#[test]
fn show_prompt_history() {
    use crate::ClearPromptHistory;
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.3:
    // ```text
    // clear-prompt-history [-T prompt-type]
    // (alias: clearphist)
    // ```
    let clear_prompt_history = ClearPromptHistory::new();
    #[cfg(feature = "tmux_3_3")]
    let clear_prompt_history = clear_prompt_history.prompt_type("1");

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
