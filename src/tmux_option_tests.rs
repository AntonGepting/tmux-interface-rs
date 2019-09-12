

#[test]
fn show_options_get_int() {
    use crate::TmuxOption;

    let base_index = TmuxOption::get_int("base-index").unwrap();
    assert_eq!(base_index, 0);

    let base_index = TmuxOption::get_int("pane-base-index").unwrap();
    assert_eq!(base_index, 0);
}
