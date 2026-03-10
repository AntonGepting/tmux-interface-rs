// auto-generated file
//

// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#DESCRIPTION)
//
// # Manual
//
// tmux >=3.6:
// ```text
// tmux [-2CDhlNuVv] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [-T features] [command [flags]]
// ```
//
// tmux >=3.4:
// ```text
// tmux [-2CDlNuVv] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [-T features] [command [flags]]
// ```
//
// tmux >=3.2:
// ```text
// tmux [-2CDluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [-T features] [command [flags]]
// ```
//
// tmux >=2.1:
// ```text
// tmux [-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
// ```
//
// tmux >=1.8:
// ```text
// tmux [-28lCquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]
// ```
//
// tmux >=1.5:
// ```text
// tmux [-28lquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
// ```
//
// tmux >=0.8:
// ```text
// tmux [-28dqUuVv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
// ```
#[test]
fn tmux() {
    use crate::ListBuffers;
    use crate::Tmux;
    use std::borrow::Cow;

    let tmux = Tmux::new();
    // `[-2]` - Force tmux to assume the terminal supports 256 colours
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux.colours256();

    // `[-8]` - indicates that tmux supports 88 colours
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    let tmux = tmux.colours88();

    // `[-d]` - indicates that tmux supports defaults colours
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let tmux = tmux.default_colours();

    // `[-q]` - prevent the server sending various information messages
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    let tmux = tmux.prevent_msg();

    // `[-C]` - Start in control mode
    #[cfg(feature = "tmux_1_8")]
    let tmux = tmux.control_mode();

    // `[-CC]` - Disable echo
    #[cfg(feature = "tmux_1_8")]
    let tmux = tmux.disable_echo();

    // `[-D]` - Do not start the tmux server as a daemon
    #[cfg(feature = "tmux_3_2")]
    let tmux = tmux.no_daemon();

    // `[-h]` - Print usage information and exit
    #[cfg(feature = "tmux_3_6")]
    let tmux = tmux.help();

    // `[-l]` - Behave as a login shell
    #[cfg(feature = "tmux_1_5")]
    let tmux = tmux.login_shell();

    // `[-N]` - Do not start the server even if the command would normally do so (for example new-session or start-server)
    #[cfg(feature = "tmux_3_4")]
    let tmux = tmux.no_start();

    // `[-U]` - Unlock the server
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let tmux = tmux.unlock();

    // `[-u]` - Write UTF-8 output to the terminal
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux.force_utf8();

    // `[-v]` - Request verbose logging
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux.verbose_logging();

    // `[-V]` - Report the tmux version
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux.version();

    // `[-c shell-command]` - Execute shell-command using the default shell
    #[cfg(feature = "tmux_1_5")]
    let tmux = tmux.shell_command("1");

    // `[-f file]` - Specify an alternative configuration file
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux.file("2");

    // `[-L socket-name]` - Allow a different socket name to be specified
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux.socket_name("3");

    // `[-S socket-path]` - Specify a full alternative path to the server socket
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux.socket_path("4");

    // `[-T features]` - Set terminal features for the client
    #[cfg(feature = "tmux_3_2")]
    let tmux = tmux.features("5");

    // `[command]` -

    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux.command(ListBuffers::new());

    let cmd = "tmux";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    v.push("-2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    v.push("-8");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.push("-d");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    v.push("-q");
    #[cfg(feature = "tmux_1_8")]
    v.push("-C");
    #[cfg(feature = "tmux_1_8")]
    v.push("-CC");
    #[cfg(feature = "tmux_3_2")]
    v.push("-D");
    #[cfg(feature = "tmux_3_6")]
    v.push("-h");
    #[cfg(feature = "tmux_1_5")]
    v.push("-l");
    #[cfg(feature = "tmux_3_4")]
    v.push("-N");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.push("-U");
    #[cfg(feature = "tmux_0_8")]
    v.push("-u");
    #[cfg(feature = "tmux_0_8")]
    v.push("-v");
    #[cfg(feature = "tmux_0_8")]
    v.push("-V");
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-f", "2"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-L", "3"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-S", "4"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-T", "5"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("list-buffers");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let tmux = tmux.build().to_vec();

    assert_eq!(tmux, v);
}
