#[test]
fn parse() {
    use crate::LayoutCell;
    use crate::LayoutType;

    let layout_orig = LayoutCell::new(176, 64, 1, 3, Some(2), LayoutType::WindowPane, None);
    let layout_cell: LayoutCell = "176x64,1,3,2".parse().unwrap();
    assert_eq!(layout_cell, layout_orig);

    let pane2 = LayoutCell::new(176, 32, 3, 4, Some(2), LayoutType::WindowPane, None);
    let pane3 = LayoutCell::new(177, 31, 5, 6, Some(3), LayoutType::WindowPane, None);
    let layout_orig = LayoutCell::new(
        178,
        64,
        1,
        2,
        None,
        LayoutType::TopBottom,
        Some(vec![pane2, pane3]),
    );
    let layout_cell: LayoutCell = "178x64,1,2[176x32,3,4,2,177x31,5,6,3]".parse().unwrap();
    assert_eq!(layout_cell, layout_orig);

    let pane2 = LayoutCell::new(177, 32, 3, 4, Some(2), LayoutType::WindowPane, None);
    let pane3 = LayoutCell::new(177, 31, 5, 6, Some(3), LayoutType::WindowPane, None);
    let layout_orig = LayoutCell::new(
        178,
        64,
        1,
        2,
        None,
        LayoutType::LeftRight,
        Some(vec![pane2, pane3]),
    );
    let layout_cell: LayoutCell = "178x64,1,2{177x32,3,4,2,177x31,5,6,3}".parse().unwrap();
    assert_eq!(layout_cell, layout_orig);

    let pane4 = LayoutCell::new(44, 32, 89, 7, Some(4), LayoutType::WindowPane, None);
    let pane5 = LayoutCell::new(43, 32, 134, 8, Some(5), LayoutType::WindowPane, None);
    let pane1 = LayoutCell::new(88, 32, 5, 6, Some(1), LayoutType::WindowPane, None);
    let window1 = LayoutCell::new(
        177,
        32,
        3,
        4,
        None,
        LayoutType::LeftRight,
        Some(vec![pane1, pane4, pane5]),
    );
    let pane2 = LayoutCell::new(88, 31, 0, 33, Some(2), LayoutType::WindowPane, None);
    let pane3 = LayoutCell::new(88, 31, 89, 33, Some(3), LayoutType::WindowPane, None);
    let window2 = LayoutCell::new(
        177,
        31,
        1,
        33,
        None,
        LayoutType::LeftRight,
        Some(vec![pane2, pane3]),
    );
    let layout_orig = LayoutCell::new(
        178,
        64,
        1,
        2,
        None,
        LayoutType::TopBottom,
        Some(vec![window1, window2]),
    );
    let layout_cell: LayoutCell = "178x64,1,2[177x32,3,4{88x32,5,6,1,44x32,89,7,4,43x32,134,8,5},177x31,1,33{88x31,0,33,2,88x31,89,33,3}]".parse().unwrap();
    assert_eq!(layout_cell, layout_orig);

    //let layout_orig = LayoutCell {
    //x: 177,
    //y: 64,
    //x_off: 1,
    //y_off: 3,
    //id: None,
    //style: LayoutType::TopBottom,
    //cells: None
    //};
    ////let layout = LayoutCell::parse("177x64,1,3[").unwrap();
    //let mut layout_cell = LayoutCell::new();
    //layout_cell.parse("177x64,1,3[");
    //assert_eq!(layout_cell, layout_orig);

    //let layout_orig = LayoutCell {
    //x: 177,
    //y: 64,
    //x_off: 1,
    //y_off: 3,
    //id: None,
    //style: LayoutType::LeftRight,
    //cells: None
    //};
    ////let layout = LayoutCell::parse("177x64,1,3{").unwrap();
    //let mut layout_cell = LayoutCell::new();
    //layout_cell.parse("177x64,1,3{");
    //assert_eq!(layout_cell, layout_orig);
}
