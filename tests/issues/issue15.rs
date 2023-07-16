// SplitWindow shell_command not working

// #[test]
// fn issue15() {
//     use tmux_interface::{NewSession, PaneSize, SplitWindow, Tmux};

//     let sesname = "issue15";

//     // When the shell command completes, the window closes (tmux man)
//     // first and last window close = session close = nothing created with commands like ls, who etc
//     let cmd = Tmux::with_command(
//         NewSession::new()
//             .session_name(sesname)
//             .detached()
//             .window_name("w1")
//             .shell_command("/bin/bash"),
//     )
//     .verbose_logging();
//     dbg!(cmd.clone().build().to_string());
//     let output = cmd.output().unwrap();

//     dbg!(output);

//     let cmd = Tmux::with_command(
//         SplitWindow::new()
//             .size(&PaneSize::Percentage(50))
//             .detached()
//             .print()
//             .target_window(sesname)
//             .shell_command("/bin/bash"),
//     )
//     .verbose_logging();
//     dbg!(cmd.clone().build().to_string());
//     let output = cmd.output().unwrap();

//     dbg!(output);
// }

// #[test]
// fn issue15_from_owner() {
//     use tmux_interface::{NewSession, SelectLayout, SplitWindow, Tmux};

//     let sesname = String::from("Testsession");
//     let targets = vec![
//         String::from("t1"),
//         String::from("t2"),
//         String::from("t3"),
//         String::from("t4"),
//         String::from("t5"),
//         String::from("t6"),
//         String::from("t7"),
//         String::from("t8"),
//     ];

//     let output = Tmux::with_command(
//         NewSession::new()
//             .detached()
//             .session_name(&sesname)
//             .window_name(&targets[0])
//             .shell_command(String::from("/bin/bash")),
//     )
//     .output()
//     .unwrap();
//     println!("Output: {:#?}", output);

//     let mut others = targets.clone().into_iter();
//     others.next();
//     for _target in others {
//         let output = Tmux::with_command(
//             SplitWindow::new()
//                 .size(&tmux_interface::commands::PaneSize::Percentage(1))
//                 .detached()
//                 .print()
//                 .target_window(&sesname)
//                 .shell_command(String::from("/bin/bash")),
//         )
//         .output()
//         .unwrap();
//         println!("Output: {:#?}", output);
//         // Depending on available size with tmux, this avoids "no space for pane" messages
//         let _ = Tmux::with_command(
//             SelectLayout::new()
//                 .target_pane(String::from("Testsession:1"))
//                 .layout_name("main-horizontal"),
//         )
//         .output()
//         .unwrap();
//     }
// }
