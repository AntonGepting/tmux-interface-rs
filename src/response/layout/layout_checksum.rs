pub struct LayoutChecksum;

impl LayoutChecksum {
    // NOTE: source layout-custom.c
    pub fn calc(s: &str) -> usize {
        let mut csum: usize = 0;

        let chars = s.chars();
        for chr in chars {
            csum = (csum >> 1) + ((csum & 1) << 15);
            csum += chr as usize;
        }
        csum
    }
}
