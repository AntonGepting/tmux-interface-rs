use super::tmux_interface::*;


/// Buffers
impl<'a> TmuxInterface<'a> {


    /// # Manual
    ///
    /// ```text
    /// tmux choose-buffer [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    pub fn chose_buffer() {
        unimplemented!();
    }


    /// # Manual
    ///
    /// ```text
    /// tmux clear-history [-t target-pane]
    /// (alias: clearhist)
    /// ```
    pub fn clear_history() {
        unimplemented!();
    }


    /// # Manual
    ///
    /// ```text
    /// tmux delete-buffer [-b buffer-name]
    /// (alias: deleteb)
    /// ```
    pub fn delete_buffer() {
        unimplemented!();
    }


    /// # Manual
    ///
    /// ```text
    /// tmux list-buffers [-F format]
    /// (alias: lsb)
    /// ```
    pub fn list_buffers() {
        unimplemented!();
    }


    /// # Manual
    ///
    /// ```text
    /// tmux load-buffer [-b buffer-name] path
    /// (alias: loadb)
    /// ```
    pub fn load_buffer() {
        unimplemented!();
    }


    /// # Manual
    ///
    /// ```text
    /// tmux paste-buffer [-dpr] [-b buffer-name] [-s separator] [-t target-pane]
    /// (alias: pasteb)
    /// ```
    pub fn paste_buffer() {
        unimplemented!();
    }


    /// # Manual
    ///
    /// ```text
    /// tmux save-buffer [-a] [-b buffer-name] path
    /// (alias: saveb)
    /// ```
    pub fn save_buffer() {
        unimplemented!();
    }


    /// # Manual
    ///
    /// ```text
    /// tmux set-buffer [-a] [-b buffer-name] [-n new-buffer-name] data
    /// (alias: setb)
    /// ```
    pub fn set_buffer() {
        unimplemented!();
    }


    /// # Manual
    ///
    /// ```text
    /// tmux show-buffer [-b buffer-name]
    /// (alias: showb)
    /// ```
    pub fn show_buffer() {
        unimplemented!();
    }


}
