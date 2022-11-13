/// Execute commands from path
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// source-file [-Fnqv] path ...
/// (alias: source)
/// ```
///
/// tmux ^3.0:
/// ```text
/// source-file [-nqv] path
/// (alias: source)
/// ```
///
/// tmux ^2.3:
/// ```text
/// source-file path
/// (alias: source)
///
/// ```
/// tmux ^0.8:
/// ```text
/// source-file path
/// (alias: source)
/// ```
#[macro_export]
macro_rules! source_file {
    (@cmd ($cmd:expr) -F, $($tail:tt)*) => {{
        $crate::source_file!(@cmd ({
            $cmd.expand()
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -n, $($tail:tt)*) => {{
        $crate::source_file!(@cmd ({
            $cmd.not_execute()
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -q, $($tail:tt)*) => {{
        $crate::source_file!(@cmd ({
            $cmd.quiet()
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -v, $($tail:tt)*) => {{
        $crate::source_file!(@cmd ({
            $cmd.verbose()
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) $path:expr, $($tail:tt)*) => {{
        $crate::source_file!(@cmd ({
            $cmd.path($path)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::SourceFile::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::source_file!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::source_file!(@cmd ({ $crate::SourceFile::new() }) $($tail)*,)
    }};
}

#[test]
fn source_file_macro() {
    use crate::SourceFile;
    use std::borrow::Cow;

    // Execute commands from path
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // source-file [-Fnqv] path ...
    // (alias: source)
    // ```
    //
    // tmux ^3.0:
    // ```text
    // source-file [-nqv] path
    // (alias: source)
    // ```
    //
    // tmux ^2.3:
    // ```text
    // source-file path
    // (alias: source)
    //
    // ```
    // tmux ^0.8:
    // ```text
    // source-file path
    // (alias: source)
    // ```
    let source_file = source_file!();
    #[cfg(feature = "tmux_3_2")]
    let source_file = source_file!((source_file), -F);
    #[cfg(feature = "tmux_3_0")]
    let source_file = source_file!((source_file), -n);
    #[cfg(feature = "tmux_3_0")]
    let source_file = source_file!((source_file), -q);
    #[cfg(feature = "tmux_3_0")]
    let source_file = source_file!((source_file), -v);
    #[cfg(feature = "tmux_0_8")]
    let source_file = source_file!((source_file), "1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "source-file";
    #[cfg(feature = "cmd_alias")]
    let cmd = "source";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    s.push("-F");
    #[cfg(feature = "tmux_3_0")]
    s.push("-n");
    #[cfg(feature = "tmux_3_0")]
    s.push("-q");
    #[cfg(feature = "tmux_3_0")]
    s.push("-v");
    #[cfg(feature = "tmux_0_8")]
    s.push("1");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let source_file = source_file.build().to_vec();

    assert_eq!(source_file, s);
}
