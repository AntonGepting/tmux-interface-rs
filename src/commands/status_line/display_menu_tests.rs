// auto-generated file
//

// Display a menu on target-client
//
// # Manual
//
// tmux >=3.5:
// ```text
// display-menu [-OM] [-b border-lines] [-c target-client] [-C starting-choice] [-H selected-style] [-s style] [-S border-style] [-t target-pane] [-T title] [-x position] [-y position] name key command ...
// alias: menu
// ```
//
// tmux >=3.4:
// ```text
// display-menu [-O] [-b border-lines] [-c target-client] [-C starting-choice] [-H selected-style] [-s style] [-S border-style] [-t target-pane] [-T title] [-x position] [-y position] name key command …
// alias: menu
// ```
//
// tmux >=3.2:
// ```text
// display-menu [-O] [-c target-client] [-t target-pane] [-T title] [-x position] [-y position] name key command ...
// alias: menu
// ```
//
// tmux >=3.0:
// ```text
// display-menu [-c target-client] [-t target-pane] [-T title] [-x position] [-y position] name key command ...
// alias: menu
// ```
#[test]
fn display_menu() {
    use crate::DisplayMenu;
    use std::borrow::Cow;

    let display_menu = DisplayMenu::new();
    // `[-O]`
    #[cfg(feature = "tmux_3_2")]
    let display_menu = display_menu.not_close();

    // `[-M]`
    #[cfg(feature = "tmux_3_5")]
    let display_menu = display_menu.mouse_in_menu();

    // `[-c target-client]`
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu.target_client("1");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu.target_pane("2");

    // `[-T title]`
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu.title("3");

    // `[-x position]`
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu.x(4);

    // `[-y position]`
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu.y(5);

    // `[-H selected-style]`
    #[cfg(feature = "tmux_3_4")]
    let display_menu = display_menu.selected_style("6");

    // `[-s style]`
    #[cfg(feature = "tmux_3_4")]
    let display_menu = display_menu.style("7");

    // `[-S border-style]`
    #[cfg(feature = "tmux_3_4")]
    let display_menu = display_menu.border_style("8");

    // `[name]`
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu.name("9");

    // `[key]`
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu.key("10");

    // `[command]`
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu.command("11");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "display-menu";
    #[cfg(feature = "cmd_alias")]
    let cmd = "menu";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    v.push("-O");
    #[cfg(feature = "tmux_3_5")]
    v.push("-M");
    #[cfg(feature = "tmux_3_0")]
    v.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_0")]
    v.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_3_0")]
    v.extend_from_slice(&["-T", "3"]);
    #[cfg(feature = "tmux_3_0")]
    v.extend_from_slice(&["-x", "4"]);
    #[cfg(feature = "tmux_3_0")]
    v.extend_from_slice(&["-y", "5"]);
    #[cfg(feature = "tmux_3_4")]
    v.extend_from_slice(&["-H", "6"]);
    #[cfg(feature = "tmux_3_4")]
    v.extend_from_slice(&["-s", "7"]);
    #[cfg(feature = "tmux_3_4")]
    v.extend_from_slice(&["-S", "8"]);
    #[cfg(feature = "tmux_3_0")]
    v.push("9");
    #[cfg(feature = "tmux_3_0")]
    v.push("10");
    #[cfg(feature = "tmux_3_0")]
    v.push("11");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let display_menu = display_menu.build().to_vec();

    assert_eq!(display_menu, v);
}
