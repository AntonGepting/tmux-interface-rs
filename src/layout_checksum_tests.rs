#[test]
fn calc() {
    use crate::LayoutChecksum;

    let checksum_orig = usize::from_str_radix("e211", 16).unwrap();
    let checksum = LayoutChecksum::calc("177x64,0,0,22");
    assert_eq!(checksum_orig, checksum);

    let checksum_orig = usize::from_str_radix("d964", 16).unwrap();
    let checksum = LayoutChecksum::calc("177x64,0,0[177x48,0,0,1,177x15,0,49,2]");
    assert_eq!(checksum_orig, checksum);
}
