// auto-generated file
//

// Clear status prompt history
//
// # Manual
//
// tmux >=3.3:
// ```text
// clear-prompt-history [-T prompt-type]
// (alias: clearphist)
// ```
#[test]
fn clear_prompt_history() {
    use crate::ClearPromptHistory;
    use std::borrow::Cow;

    let clear_prompt_history = ClearPromptHistory::new();
    // `[-T prompt-type]`
    #[cfg(feature = "tmux_3_3")]
    let clear_prompt_history = clear_prompt_history.prompt_type("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "clear-prompt-history";
    #[cfg(feature = "cmd_alias")]
    let cmd = "clearphist";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_3")]
    v.extend_from_slice(&["-T", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let clear_prompt_history = clear_prompt_history.build().to_vec();

    assert_eq!(clear_prompt_history, v);
}
