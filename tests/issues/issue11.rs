// issue 11: Order of option matters?! Shouldn't, imo.
//
// order of arguments unified, builder pattern for commands
//
// XXX: mb all commands tests as they will be someday a product of templating
// engine, place arguments in reverse order, for auto checking that issue
#[test]
fn issue11() {
    use std::borrow::Cow;
    use tmux_interface::SelectLayout;

    let target = "target";
    let layout_name = "layout_name";

    let select_layout1 = SelectLayout::new();
    #[cfg(feature = "tmux_2_7")]
    let select_layout1 = select_layout1.spread();
    #[cfg(feature = "tmux_1_5")]
    let select_layout1 = select_layout1.next_layout();
    #[cfg(feature = "tmux_2_1")]
    let select_layout1 = select_layout1.last_layout();
    #[cfg(feature = "tmux_1_5")]
    let select_layout1 = select_layout1.previous_layout();
    #[cfg(feature = "tmux_0_9")]
    let select_layout1 = select_layout1.target_pane(target);
    #[cfg(feature = "tmux_1_0")]
    let select_layout1 = select_layout1.layout_name(layout_name);

    let select_layout1 = select_layout1.build().to_vec();

    let select_layout2 = SelectLayout::new();
    #[cfg(feature = "tmux_1_0")]
    let select_layout2 = select_layout2.layout_name(layout_name);
    #[cfg(feature = "tmux_0_9")]
    let select_layout2 = select_layout2.target_pane(target);
    #[cfg(feature = "tmux_1_5")]
    let select_layout2 = select_layout2.previous_layout();
    #[cfg(feature = "tmux_2_1")]
    let select_layout2 = select_layout2.last_layout();
    #[cfg(feature = "tmux_1_5")]
    let select_layout2 = select_layout2.next_layout();
    #[cfg(feature = "tmux_2_7")]
    let select_layout2 = select_layout2.spread();

    let select_layout2 = select_layout2.build().to_vec();

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "select-layout";
    #[cfg(feature = "cmd_alias")]
    let cmd = "selectl";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_7")]
    s.push("-E");
    #[cfg(feature = "tmux_1_5")]
    s.push("-n");
    #[cfg(feature = "tmux_2_1")]
    s.push("-o");
    #[cfg(feature = "tmux_1_5")]
    s.push("-p");
    #[cfg(feature = "tmux_0_9")]
    s.extend_from_slice(&["-t", target]);
    #[cfg(feature = "tmux_1_0")]
    s.push(layout_name);
    let expected: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(expected, select_layout1);
    assert_eq!(expected, select_layout2);
}
