pub struct LayoutChecksum;

impl LayoutChecksum {
    // NOTE: tmux source -> layout-custom.c
    // BSD checksum algorithm (without in bounds AND 0b1111_1111)
    pub fn calc(s: &str) -> u16 {
        let mut csum: u16 = 0;

        let chars = s.chars();
        for chr in chars {
            csum = (csum >> 1) + ((csum & 1) << 15);
            csum += chr as u16;
        }
        csum
    }
}
