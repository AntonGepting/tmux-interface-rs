// auto-generated file
//

// Put a pane into customize mode
//
// # Manual
//
// tmux >=3.2:
// ```text
// customize-mode [-NZ] [-F format] [-f filter] [-t target-pane] [template]
// ```
#[test]
fn customize_mode() {
    use crate::CustomizeMode;
    use std::borrow::Cow;

    let customize_mode = CustomizeMode::new();
    // `[-N]`
    #[cfg(feature = "tmux_3_2")]
    let customize_mode = customize_mode.without_option_info();

    // `[-Z]`
    #[cfg(feature = "tmux_3_2")]
    let customize_mode = customize_mode.zoom();

    // `[-F format]`
    #[cfg(feature = "tmux_3_2")]
    let customize_mode = customize_mode.format("");

    // `[-f filter]`
    #[cfg(feature = "tmux_3_2")]
    let customize_mode = customize_mode.filter("");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_3_2")]
    let customize_mode = customize_mode.target_pane("");

    // `[template]`
    #[cfg(feature = "tmux_3_2")]
    let customize_mode = customize_mode.template("");

    let cmd = "customize-mode";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    v.push("-N");
    #[cfg(feature = "tmux_3_2")]
    v.push("-Z");
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-F", ""]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-f", ""]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-t", ""]);
    #[cfg(feature = "tmux_3_2")]
    v.push("");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let customize_mode = customize_mode.build().to_vec();

    assert_eq!(customize_mode, v);
}
