// auto-generated file
//

// Display a menu on target-client
//
// # Manual
//
// tmux >=3.6:
// ```text
// display-popup [-BCEkN] [-b border-lines] [-c target-client] [-d start-directory] [-e environment]
// [-h height] [-s style] [-S border-style] [-t target-pane] [-T title] [-w width] [-x position]
// [-y position] [shell-command]
// ```
//
// tmux >=3.3:
// ```text
// display-popup [-BCE] [-b border-lines] [-c target-client] [-d start-directory] [-e environment]
// [-h height] [-s style] [-S border-style] [-t target-pane] [-T title] [-w width] [-x position]
// [-y position] [shell-command]
// ```
//
// tmux >=3.2:
// ```text
// display-popup [-CE] [-c target-client] [-d start-directory] [-h height] [-t target-pane]
// [-w width] [-x position] [-y position] [shell-command]
// (alias: popup)
// ```
#[test]
fn display_popup() {
    use crate::DisplayPopup;
    use std::borrow::Cow;

    let display_popup = DisplayPopup::new();
    // `[-B]`
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup.no_border();

    // `[-C]`
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.close();

    // `[-E]`
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.close_on_exit();

    // `[-EE]`
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.close_on_success();

    // `[-k]`
    #[cfg(feature = "tmux_3_6")]
    let display_popup = display_popup.any_key_dismiss();

    // `[-N]`
    #[cfg(feature = "tmux_3_6")]
    let display_popup = display_popup.disable_previous();

    // `[-b border-lines]`
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup.border_lines("1");

    // `[-c target-client]`
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.target_client("2");

    // `[-d start-directory]`
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.start_directory("3");

    // `[-e environment]`
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup.environment("4");

    // `[-h height]`
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.height(5);

    // `[-s style]`
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup.style("6");

    // `[-S border-style]`
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup.border_style("7");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.target_pane("8");

    // `[-T title]`
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup.title("9");

    // `[-w width]`
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.width(10);

    // `[-x position]`
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.x(11);

    // `[-y position]`
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.y(12);

    // `[shell-command]`
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.shell_command("13");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "display-popup";
    #[cfg(feature = "cmd_alias")]
    let cmd = "popup";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_3")]
    v.push("-B");
    #[cfg(feature = "tmux_3_2")]
    v.push("-C");
    #[cfg(feature = "tmux_3_2")]
    v.push("-E");
    #[cfg(feature = "tmux_3_2")]
    v.push("-EE");
    #[cfg(feature = "tmux_3_6")]
    v.push("-k");
    #[cfg(feature = "tmux_3_6")]
    v.push("-N");
    #[cfg(feature = "tmux_3_3")]
    v.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-c", "2"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-d", "3"]);
    #[cfg(feature = "tmux_3_3")]
    v.extend_from_slice(&["-e", "4"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-h", "5"]);
    #[cfg(feature = "tmux_3_3")]
    v.extend_from_slice(&["-s", "6"]);
    #[cfg(feature = "tmux_3_3")]
    v.extend_from_slice(&["-S", "7"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-t", "8"]);
    #[cfg(feature = "tmux_3_3")]
    v.extend_from_slice(&["-T", "9"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-w", "10"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-x", "11"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-y", "12"]);
    #[cfg(feature = "tmux_3_2")]
    v.push("13");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let display_popup = display_popup.build().to_vec();

    assert_eq!(display_popup, v);
}
