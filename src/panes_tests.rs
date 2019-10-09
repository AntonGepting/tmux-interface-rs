#[test]
fn panes_parse() {
    use crate::Pane;
    use crate::Panes;

    let panes_str = "1'1'1'1'1'63'bash'/home/user'0''1'64'%0'0'0'0'0'0'0''1945'0'176'''0'8,16,24,\
                     32,40,48,56,64,72,80,88,96,104,112,120,128,136,144,152,160,168,176'asus'0'\
                     /dev/pts/2'177\n\
                     1'0'1'1'1'45'vim'/home/user/Documents/abc/cde-fgh/ijk'0''1'46'%1'0'0'0'0'0'0\
                     ''13587'0'176'''0'8,16,24,32,40,48,56,64,72,80,88,96,104,112,120,128,136,144,\
                     152,160,168,176'asus'0'/dev/pts/3'1771";
    let panes = Panes::from_str(panes_str, Pane::PANE_ALL).unwrap();
    assert_eq!(panes[0].id, Some(0));
}
