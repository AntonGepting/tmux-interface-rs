// auto-generated file
//

/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#DESCRIPTION)
///
/// # Manual
///
/// tmux >=3.6:
/// ```text
/// tmux [-2CDluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [-T features] [command [flags]]
/// ```
///
/// tmux >=3.2:
/// ```text
/// tmux [-2CDluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [-T features] [command [flags]]
/// ```
///
/// tmux >=2.1:
/// ```text
/// tmux [-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux >=1.9:
/// ```text
/// tmux [-2lCquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux >=1.8:
/// ```text
/// tmux [-28lCquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]
/// ```
///
/// tmux >=1.4:
/// ```text
/// tmux [-28lquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux >=1.1:
/// ```text
/// tmux [-28lquv] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux >=1.0:
/// ```text
/// tmux [-28dlqUuv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux >=0.9:
/// ```text
/// tmux [-28dqUuv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux >=0.8:
/// ```text
/// tmux [-28dqUuVv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
// Description of `some_command!` macro
//
// what it does
// * recursive call
// what it does not
//
// Matching parts:
//  * `@cmd ($cmd:expr)` - inner variable by recursive invokations
//  * `$($tail:tt)*` - rest of the macro command arguments (like args)
//  * `-x,`
//  * `-o $output:tt,`
//  * `$input:expr,`
//
// Matching arms:
//  * Set `SomeCommand` arguments:
//      1. flag (`[-a]`)
//      2. option (`[-o output]`)
//      3. parameter (`[input]`)
//  * ? Last argument (end of recursion)
//  * Create `SomeCommand` with no arguments
//  * Set `SomeCommand` arguments, do not create it (recursively)
//  * Create `SomeCommand` with arguments (recursively)
//
// Matching arms with implementation:
// * Set `SomeCommand` arguments
//     1. `-x` - single flag
//     ```
//        (@cmd ($cmd:expr) -x, $($tail:tt)*) => {{
//            $crate::some_command!(@cmd ({
//                $cmd.x()
//            }) $($tail)*)
//        }};
//     ```
//
//     2. `-x $file:expr` - single option with flag
//     ```
//        (@cmd ($cmd:expr) -o $output:tt, $($tail:tt)*) => {{
//            $crate::some_command!(@cmd ({
//                $cmd.output($output)
//            }) $($tail)*)
//        }};
//     ```
//
//     3. `$param:expr` - single parameter, without flag
//     ```
//        (@cmd ($cmd:expr) $input:expr, $($tail:tt)*) => {{
//            $crate::some_command!(@cmd ({
//                $cmd.input($input)
//            }) $($tail)*)
//        }};
//     ```
//
// * ? Last argument (end of recursion)
//     ```
//        (@cmd ($cmd:expr)) => {{
//            $cmd
//        }};
//     ```
//
// * Create `SomeCommand` with no arguments
//    ```
//       () => {{
//           $crate::SomeCommand::new()
//       }};
//    ```
//
// * Set arguments, do not create `SomeCommand` recursive call
//     ```
//        (($cmd:expr), $($tail:tt)*) => {{
//            $crate::some_command!(@cmd ($cmd) $($tail)*,)
//        }};
//     ```
//
// * Create `SomeCommand` with arguments, recursive call
//     ```
//        ($($tail:tt)*) => {{
//            $crate::some_command!(@cmd ({ $crate::SomeCommand::new() }) $($tail)*,)
//        }};
//     ```
//
#[macro_export]
macro_rules! tmux {
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

    // `[-D]` - Do not start the tmux server as a daemon
    (@cmd ($cmd:expr) -D, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.no_daemon()
        }) $($tail)*)
    }};

    // `[-h]` - Print usage information and exit
    (@cmd ($cmd:expr) -h, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.help()
        }) $($tail)*)
    }};

    // `[-l]` - Behave as a login shell
    (@cmd ($cmd:expr) -l, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.login_shell()
        }) $($tail)*)
    }};

    // `[-N]` - Do not start the server even if the command would normally do so (for example new-session or start-server)
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.no_start()
        }) $($tail)*)
    }};

    // `[-U]` - Unlock the server
    (@cmd ($cmd:expr) -U, $($tail:tt)*) => {{
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

    // `[-c shell-command]` - Execute shell-command using the default shell
    (@cmd ($cmd:expr) -c $shell_command:expr, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.shell_command($shell_command)
        }) $($tail)*)
    }};

    // `[-f file]` - Specify an alternative configuration file
    (@cmd ($cmd:expr) -f $file:expr, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.file($file)
        }) $($tail)*)
    }};

    // `[-L socket-name]` - Allow a different socket name to be specified
    (@cmd ($cmd:expr) -L $socket_name:expr, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.socket_name($socket_name)
        }) $($tail)*)
    }};

    // `[-S socket-path]` - Specify a full alternative path to the server socket
    (@cmd ($cmd:expr) -S $socket_path:expr, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.socket_path($socket_path)
        }) $($tail)*)
    }};

    // `[-T features]` - Set terminal features for the client
    (@cmd ($cmd:expr) -T $features:expr, $($tail:tt)*) => {{
        $crate::tmux!(@cmd ({
            $cmd.features($features)
        }) $($tail)*)
    }};

    // `[command]` -

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
    use crate::ListBuffers;
    use std::borrow::Cow;

    // [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#DESCRIPTION)
    //
    // # Manual
    //
    // tmux >=3.6:
    // ```text
    // tmux [-2CDluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [-T features] [command [flags]]
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
    // tmux >=1.9:
    // ```text
    // tmux [-2lCquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
    // ```
    //
    // tmux >=1.8:
    // ```text
    // tmux [-28lCquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]
    // ```
    //
    // tmux >=1.4:
    // ```text
    // tmux [-28lquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
    // ```
    //
    // tmux >=1.1:
    // ```text
    // tmux [-28lquv] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
    // ```
    //
    // tmux >=1.0:
    // ```text
    // tmux [-28dlqUuv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
    // ```
    //
    // tmux >=0.9:
    // ```text
    // tmux [-28dqUuv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
    // ```
    //
    // tmux >=0.8:
    // ```text
    // tmux [-28dqUuVv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
    // ```

    let tmux = tmux!();
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux!((tmux), -2);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    let tmux = tmux!((tmux), -8);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let tmux = tmux!((tmux), -d);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    let tmux = tmux!((tmux), -q);
    #[cfg(feature = "tmux_1_8")]
    let tmux = tmux!((tmux), -C);
    #[cfg(feature = "tmux_1_8")]
    let tmux = tmux!((tmux), -CC);
    #[cfg(feature = "tmux_3_2")]
    let tmux = tmux!((tmux), -D);
    #[cfg(feature = "tmux_3_6")]
    let tmux = tmux!((tmux), -h);
    #[cfg(feature = "tmux_1_5")]
    let tmux = tmux!((tmux), -l);
    #[cfg(feature = "tmux_3_4")]
    let tmux = tmux!((tmux), -N);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let tmux = tmux!((tmux), -U);
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux!((tmux), -u);
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux!((tmux), -v);
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux!((tmux), -V);
    #[cfg(feature = "tmux_1_5")]
    let tmux = tmux!((tmux), -c "1");
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux!((tmux), -f "2");
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux!((tmux), -L "3");
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux!((tmux), -S "4");
    #[cfg(feature = "tmux_3_2")]
    let tmux = tmux!((tmux), -T "5");
    #[cfg(feature = "tmux_0_8")]
    let tmux = tmux!((tmux), ListBuffers::new());

    let cmd = "tmux";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.push("-2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    s.push("-8");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.push("-d");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    s.push("-q");
    #[cfg(feature = "tmux_1_8")]
    s.push("-C");
    #[cfg(feature = "tmux_1_8")]
    s.push("-CC");
    #[cfg(feature = "tmux_3_2")]
    s.push("-D");
    #[cfg(feature = "tmux_3_6")]
    s.push("-h");
    #[cfg(feature = "tmux_1_5")]
    s.push("-l");
    #[cfg(feature = "tmux_3_4")]
    s.push("-N");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.push("-U");
    #[cfg(feature = "tmux_0_8")]
    s.push("-u");
    #[cfg(feature = "tmux_0_8")]
    s.push("-v");
    #[cfg(feature = "tmux_0_8")]
    s.push("-V");
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-f", "2"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-L", "3"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-S", "4"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-T", "5"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("list-buffers");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let tmux = tmux.build().to_vec();

    assert_eq!(tmux, s);
}
