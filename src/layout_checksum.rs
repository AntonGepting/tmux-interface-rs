pub struct LayoutChecksum;

impl LayoutChecksum {
    // NOTE: source layout-custom.c
    pub fn calc(s: &str) -> usize {
        let mut csum: usize = 0;

        let mut chars = s.chars();
        loop {
            if let Some(chr) = chars.next() {
                csum = (csum >> 1) + ((csum & 1) << 15);
                csum += chr as usize;
            } else {
                break;
            }
        }
        csum
    }
}
