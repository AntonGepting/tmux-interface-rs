#[test]
fn panes_parse() {
    use crate::Panes;

    let panes = Panes::parse("1 0 0 0 0 vim /var/log %0 0 asdf\n1 0 0 0 0 top /var/log %1 1 asdf").unwrap();
    assert_eq!(panes[0].id, 0);
}
