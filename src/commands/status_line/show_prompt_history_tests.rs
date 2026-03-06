// auto-generated file
//

// Display status prompt history
//
// # Manual
//
// tmux >=3.3:
// ```text
// show-prompt-history [-T prompt-type]
// (alias: showphist)
// ```
#[test]
fn show_prompt_history() {
    use crate::ShowPromptHistory;
    use std::borrow::Cow;

    let show_prompt_history = ShowPromptHistory::new();
    // `[-T prompt-type]`
    #[cfg(feature = "tmux_3_3")]
    let show_prompt_history = show_prompt_history.prompt_type("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-prompt-history";
    #[cfg(feature = "cmd_alias")]
    let cmd = "showphist";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_3")]
    v.extend_from_slice(&["-T", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let show_prompt_history = show_prompt_history.build().to_vec();

    assert_eq!(show_prompt_history, v);
}
