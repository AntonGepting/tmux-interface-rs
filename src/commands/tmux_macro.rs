/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#DESCRIPTION)
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux [-2CDluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [-T features] [command [flags]]
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux [-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^1.9:
/// ```text
/// tmux [-2lCquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^1.8:
/// ```text
/// tmux [-28lCquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]
/// ```
///
/// tmux ^1.4:
/// ```text
/// tmux [-28lquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^1.1:
/// ```text
/// tmux [-28lquv] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux [-28dlqUuv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^0.9:
/// ```text
/// tmux [-28dqUuv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux [-28dqUuVv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
#[macro_export]
macro_rules! tmux {
    // `[-c shell-command]` - Execute shell-command using the default shell
    (@cmd ($cmd:expr) -c $shell_command:tt, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.shell_command($shell_command)
        }) $($tail)*)
    }};
    // `[-f file]` - Specify an alternative configuration file
    (@cmd ($cmd:expr) -f $file:tt, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.file($file)
        }) $($tail)*)
    }};
    // `[-L socket-name]` - Allow a different socket name to be specified
    (@cmd ($cmd:expr) -L $socket_name:tt, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.socket_name($socket_name)
        }) $($tail)*)
    }};
    // `[-S socket-path]` - Specify a full alternative path to the server socket
    (@cmd ($cmd:expr) -S $socket_path:tt, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.socket_path($socket_path)
        }) $($tail)*)
    }};
    // `[-T features]` - Set terminal features for the client
    (@cmd ($cmd:expr) -T $features:tt, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.features($features)
        }) $($tail)*)
    }};
    // `[command]`
    (@cmd ($cmd:expr) $command:tt, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.command($command)
        }) $($tail)*)
    }};
    // `[-2]` - Force tmux to assume the terminal supports 256 colours
    (@cmd ($cmd:expr) -2, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.colours256()
        }) $($tail)*)
    }};
    // `[-8]` - indicates that tmux supports 88 colours
    (@cmd ($cmd:expr) -8, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.colours88()
        }) $($tail)*)
    }};
    // `[-d]` - indicates that tmux supports defaults colours
    (@cmd ($cmd:expr) -d, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.default_colours()
        }) $($tail)*)
    }};
    // `[-q]` - prevent the server sending various information messages
    (@cmd ($cmd:expr) -q, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.prevent_msg()
        }) $($tail)*)
    }};
    // `[-C]` - Start in control mode
    (@cmd ($cmd:expr) -C, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.control_mode()
        }) $($tail)*)
    }};
    // `[-CC]` - Disable echo
    (@cmd ($cmd:expr) -CC, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.disable_echo()
        }) $($tail)*)
    }};
    // `[-D]` - Do not start the tmux server as a daemon. This also turns the exit-empty option off.  With -D, command may not be specified.
    (@cmd ($cmd:expr) -D, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.no_daemon()
        }) $($tail)*)
    }};
    // `[-l]` - Behave as a login shell
    (@cmd ($cmd:expr) -l, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.login_shell()
        }) $($tail)*)
    }};
    // `[-N]` - Do not start the server even if the command would normally do so (for example new-session or start-server).
    (@cmd ($cmd:expr) -D, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.no_start()
        }) $($tail)*)
    }};
    // `[-U]` - Unlock the server
    (@cmd ($cmd:expr) -D, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.unlock()
        }) $($tail)*)
    }};
    // `[-u]` - Write UTF-8 output to the terminal
    (@cmd ($cmd:expr) -u, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.force_utf8()
        }) $($tail)*)
    }};
    // `[-v]` - Request verbose logging
    (@cmd ($cmd:expr) -v, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.verbose_logging()
        }) $($tail)*)
    }};
    // `[-V]` - Report the tmux version
    (@cmd ($cmd:expr) -V, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.version()
        }) $($tail)*)
    }};
    // `[command [flags]]`
    (@cmd ($cmd:expr) $command:expr, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.command($command)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::Tmux::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::tmux!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::tmux!(@cmd ({ $crate::Tmux::new() }) $($tail)*,)
    }};
}

#[test]
fn tmux_macro() {
    use crate::attach_session;
    use crate::Tmux;
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
    let mut tmux = tmux!();
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux!((tmux), -2);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    let tmux = tmux!((tmux), -8);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
    let tmux = tmux!((tmux), -d);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    let tmux = tmux!((tmux), -q);
    #[cfg(feature = "tmux_1_8")]
    let tmux = tmux!((tmux), -C);
    #[cfg(feature = "tmux_1_8")]
    let tmux = tmux!((tmux), -CC);
    #[cfg(feature = "tmux_3_2")]
    let tmux = tmux!((tmux), -D);
    #[cfg(feature = "tmux_1_0")]
    let tmux = tmux!((tmux), -l);
    #[cfg(feature = "tmux_3_2")]
    let tmux = tmux!((tmux), -N);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
    let tmux = tmux!((tmux), -U);
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux!((tmux), -u);
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux!((tmux), -v);
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux!((tmux), -V);
    #[cfg(feature = "tmux_1_1")]
    let tmux = tmux!((tmux), -c "1");
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux!((tmux), -f "2");
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux!((tmux), -L "3");
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux!((tmux), -S "4");
    #[cfg(feature = "tmux_3_2")]
    let tmux = tmux!((tmux), -T "5");

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