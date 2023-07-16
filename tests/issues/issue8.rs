// #[test]
// fn issue8() {
//     use tmux_interface::{
//         GetGlobalSessionOptionValue, GetSessionOptionTr, GlobalSessionOptionsCtl,
//         SessionOptionsCtl, StartServer, Tmux,
//     };

//     let value = Tmux::with_command(StartServer::new())
//         .add_command(GetGlobalSessionOptionValue::default_shell(None::<&str>))
//         .output()
//         .unwrap();
//     dbg!(value);

//     let value = Tmux::with_command(StartServer::new())
//         .add_command(GetGlobalSessionOptionValue::default_shell::<&str>(None))
//         .output()
//         .unwrap();
//     dbg!(value);

//     let value = GlobalSessionOptionsCtl::with_invoker(&|cmd| {
//         Tmux::with_command(StartServer::new())
//             .add_command(cmd)
//             .output()
//     })
//     .get_default_shell()
//     .unwrap();
//     dbg!(value);
// }
