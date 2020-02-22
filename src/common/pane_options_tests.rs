#[test]
fn parse() {
    use crate::PaneOptions;

    let pane_options_str = r#"
        allow-rename off
        alternate-screen on
        remain-on-exit off
        window-active-style fg=colour253,bg=colour235
        window-style fg=colour247,bg=colour238
    "#;
    let _pane_options = pane_options_str.parse::<PaneOptions>().unwrap();
    //dbg!(&pane_options);
}

#[test]
fn to_string() {
    use crate::{PaneOptionsBuilder, Switch};

    let _pane_options = PaneOptionsBuilder::new().allow_rename(Switch::Off).build();
    //dbg!(&pane_options.to_string());
}

#[test]
fn get() {
    use crate::PaneOptions;
    let _pane_options = PaneOptions::get_all().unwrap();
    //dbg!(pane_options);
}
