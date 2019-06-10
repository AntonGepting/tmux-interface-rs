

#[test]
fn show_options_get_int() {
    use crate::TmuxOption;

    let base_index = TmuxOption::get_int("base-index").unwrap_or(0);
    assert_eq!(base_index, 0);
}



