#[test]
fn tmux() {
    use crate::commands::tmux::Tmux;
    use std::borrow::Cow;

    // This structure is used to store execution parameters of `tmux`, including binary
    // name. Full description of fields can be found using `man tmux`.
    // [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#DESCRIPTION)
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // tmux [-2CDluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [-T features] [command [flags]]
    // ```
    //
    // tmux ^2.1:
    // ```text
    // tmux [-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
    // ```
    //
    // tmux ^1.9:
    // ```text
    // tmux [-2lCquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
    // ```
    //
    // tmux ^1.8:
    // ```text
    // tmux [-28lCquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]
    // ```
    //
    // tmux ^1.4:
    // ```text
    // tmux [-28lquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
    // ```
    //
    // tmux ^1.1:
    // ```text
    // tmux [-28lquv] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux [-28dlqUuv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
    // ```
    //
    // tmux ^0.9:
    // ```text
    // tmux [-28dqUuv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux [-28dqUuVv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
    // ```
    let mut tmux = Tmux::new();
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux.colours256();
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    let tmux = tmux.colours88();
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
    let tmux = tmux.default_colours();
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    let tmux = tmux.prevent_msg();
    #[cfg(feature = "tmux_1_8")]
    let tmux = tmux.control_mode();
    #[cfg(feature = "tmux_1_8")]
    let tmux = tmux.disable_echo();
    #[cfg(feature = "tmux_3_2")]
    let tmux = tmux.no_daemon();
    #[cfg(feature = "tmux_1_0")]
    let tmux = tmux.login_shell();
    #[cfg(feature = "tmux_3_2")]
    let tmux = tmux.no_start();
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
    let tmux = tmux.unlock();
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux.force_utf8();
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux.verbose_logging();
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux.version();
    #[cfg(feature = "tmux_1_1")]
    let tmux = tmux.shell_cmd("1");
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux.file("2");
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux.socket_name("3");
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux.socket_path("4");
    #[cfg(feature = "tmux_3_2")]
    let tmux = tmux.features("5");

    let mut s = Vec::new();

    s.push("tmux");

    #[cfg(feature = "tmux_0_8")]
    s.push("-2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    s.push("-8");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
    s.push("-d");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    s.push("-q");
    #[cfg(feature = "tmux_1_8")]
    s.push("-C");
    #[cfg(feature = "tmux_1_8")]
    s.push("-CC");
    #[cfg(feature = "tmux_3_2")]
    s.push("-D");
    #[cfg(feature = "tmux_1_0")]
    s.push("-l");
    #[cfg(feature = "tmux_3_2")]
    s.push("-N");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
    s.push("-U");
    #[cfg(feature = "tmux_0_8")]
    s.push("-u");
    #[cfg(feature = "tmux_0_8")]
    s.push("-v");
    #[cfg(feature = "tmux_0_8")]
    s.push("-V");
    #[cfg(feature = "tmux_1_1")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-f", "2"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-L", "3"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-S", "4"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-T", "5"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let tmux = tmux.build().to_vec();

    assert_eq!(tmux, s);
}
