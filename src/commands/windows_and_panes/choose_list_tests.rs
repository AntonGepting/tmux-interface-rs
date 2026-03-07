// auto-generated file
//

// Put a window into list choice mode
//
// # Manual
//
// tmux >=1.7 && <=1.9a:
// ```text
// choose-list [-l items] [-t target-window] [template]
// ```
#[test]
fn choose_list() {
    use crate::ChooseList;
    use std::borrow::Cow;

    let choose_list = ChooseList::new();
    // `[-l items]`
    #[cfg(feature = "tmux_1_7")]
    let choose_list = choose_list.items("1");

    // `[-t target-window]`
    #[cfg(feature = "tmux_1_7")]
    let choose_list = choose_list.target_window("2");

    // `[template]`
    #[cfg(feature = "tmux_1_7")]
    let choose_list = choose_list.template("3");

    let cmd = "choose-list";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    v.extend_from_slice(&["-l", "1"]);
    #[cfg(feature = "tmux_1_7")]
    v.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_1_7")]
    v.push("3");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let choose_list = choose_list.build().to_vec();

    assert_eq!(choose_list, v);
}
