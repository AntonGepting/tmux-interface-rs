// auto-generated file
//

// Put a window into window choice mode
//
// # Manual
//
// tmux >=1.7 && <=2.5:
// ```text
// choose-window [-F format] [-t target-window] [template]
// ```
//
// tmux >=1.5:
// ```text
// choose-window [-t target-window] [template]
// ```
//
// tmux >=0.8:
// ```text
// choose-window [-t target-window]
// ```
#[test]
fn choose_window() {
    use crate::ChooseWindow;
    use std::borrow::Cow;

    let choose_window = ChooseWindow::new();
    // `[-F format]`
    #[cfg(feature = "tmux_1_7")]
    let choose_window = choose_window.format("1");

    // `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    let choose_window = choose_window.target_window("2");

    // `[template]`
    #[cfg(feature = "tmux_1_5")]
    let choose_window = choose_window.template("3");

    let cmd = "choose-window";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    v.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_1_5")]
    v.push("3");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let choose_window = choose_window.build().to_vec();

    assert_eq!(choose_window, v);
}
