use std::borrow::Cow;


use super::tmux_interface::*;
use super::tmux_interface_error::TmuxInterfaceError;


/// Structure for creating new window, using `tmux new-window` command
/// ```text
/// new-window [-adkP] [-c start-directory] [-F format] [-n window-name] [-t target-window]
/// [shell-command]
/// (alias: neww)
/// ```
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
///
pub struct NewWindow<'a> {
    pub add: Option<bool>,                      // [-a]
    pub detached: Option<bool>,                 // [-d]
    pub kill: Option<bool>,                     // [-k]
    pub print: Option<bool>,                    // [-P]
    pub cwd: Option<Cow<'a, str>>,              // [-c start-directory]
    pub format: Option<Cow<'a, str>>,           // [-F format]
    pub window_name: Option<Cow<'a, str>>,      // [-n window-name]
    pub target_window: Option<Cow<'a, str>>,    // [-t target-window]
    pub shell_command: Option<Cow<'a, str>>,    // [shell-command]
}


impl<'a> Default for NewWindow<'a> {
    fn default() -> Self {
        NewWindow {
            add: None,
            detached: None,
            kill: None,
            print: None,
            cwd: None,
            format: None,
            window_name: None,
            target_window: None,
            shell_command: None
        }
    }
}

impl<'a> NewWindow<'a> {
    pub fn new() -> NewWindow<'a> {
        Default::default()
    }
}


pub struct SplitWindow<'a> {
    pub before: Option<bool>,                   // [-b]
    pub detached: Option<bool>,                 // [-d]
    pub full: Option<bool>,                     // [-f]
    pub horizontal: Option<bool>,               // [-h]
    pub vertical: Option<bool>,                 // [-v]
    pub print: Option<bool>,                    // [-P]
    pub cwd: Option<Cow<'a, str>>,              // [-c start-directory]
    pub size: Option<usize>,                    // [-l size]
    pub percentage: Option<usize>,              // [-p percentage]
    pub target_pane: Option<Cow<'a, str>>,      // [-t target-pane]
    pub shell_command: Option<Cow<'a, str>>,    // [shell-command]
    pub format: Option<Cow<'a, str>>,           // [-F format]
}

impl<'a> Default for SplitWindow<'a> {
    fn default() -> Self {
        SplitWindow {
            before: None,
            detached: None,
            full: None,
            horizontal: None,
            vertical: None,
            print: None,
            cwd: None,
            size: None,
            percentage: None,
            target_pane: None,
            shell_command: None,
            format: None
        }
    }
}


impl<'a> SplitWindow<'a> {
    pub fn new() -> SplitWindow<'a> {
        Default::default()
    }
}


pub struct SelectPane<'a> {
    pub down: Option<bool>,                     // [-D]
    pub disable: Option<bool>,                  // [-d]
    pub enable: Option<bool>,                   // [-e]
    pub show_style: Option<bool>,               // [-g]
    pub left: Option<bool>,                     // [-L]
    pub last: Option<bool>,                     // [-l]
    pub set_marked: Option<bool>,               // [-M]
    pub clear_marked: Option<bool>,             // [-m]
    pub right: Option<bool>,                    // [-R]
    pub up: Option<bool>,                       // [-U]
    pub style: Option<&'a str>,                 // [-P style]
    pub title: Option<&'a str>,                 // [-T title]
    pub target_pane: Option<&'a str>,           // [-t target-pane]
}


impl<'a> Default for SelectPane<'a> {
    fn default() -> Self {
        SelectPane {
            down: None,
            disable: None,
            enable: None,
            show_style: None,
            left: None,
            last: None,
            set_marked: None,
            clear_marked: None,
            right: None,
            up: None,
            style: None,
            title: None,
            target_pane: None
        }
    }
}

impl<'a> SelectPane<'a> {
    pub fn new() -> SelectPane<'a> {
        Default::default()
    }
}


pub struct SelectWindow<'a> {
    pub last: Option<bool>,                     // [-l]
    pub next: Option<bool>,                     // [-n]
    pub previous: Option<bool>,                 // [-p]
    pub switch: Option<bool>,                   // [-T]
    pub target_window: Option<&'a str>          // [-t target-window]
}


impl<'a> Default for SelectWindow<'a> {
    fn default() -> Self {
        SelectWindow {
            last: None,
            next: None,
            previous: None,
            switch: None,
            target_window: None
        }
    }
}

impl<'a> SelectWindow<'a> {
    pub fn new() -> SelectWindow<'a> {
        Default::default()
    }
}


/// Windows and panes
impl<'a> TmuxInterface<'a> {

    const KILL_WINDOW: &'static str = "kill-window";
    const NEW_WINDOW: &'static str = "new-window";
    const LIST_WINDOWS: &'static str = "list-windows";
    const RENAME_WINDOW: &'static str = "rename-window";
    const SPLIT_WINDOW: &'static str = "split-window";
    const LIST_PANES: &'static str = "list-panes";
    const SELECT_WINDOW: &'static str = "select-window";
    const SELECT_PANE: &'static str = "select-pane";


    /// ```text
    /// copy-mode [-Meu] [-t target-pane]
    /// ```
    pub fn copy_mode() {
        unimplemented!();
    }


    /// ```text
    /// break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
    /// (alias: breakp)
    /// ```
    pub fn break_pane() {
        unimplemented!();
    }


    /// ```text
    /// capture-pane [-aepPqCJ] [-b buffer-name] [-E end-line] [-S start-line]
    /// [-t target-pane]
    /// (alias: capturep)
    /// ```
    pub fn capture_pane() {
        unimplemented!();
    }


    /// ```text
    /// choose-client [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    pub fn choose_client() {
        unimplemented!();
    }


    /// ```text
    /// choose-tree [-GNswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    pub fn choose_tree() {
        unimplemented!();
    }


    /// ```text
    /// display-panes [-b] [-d duration] [-t target-client] [template] (alias: displayp)
    /// ```
    pub fn display_panes() {
        unimplemented!();
    }


    /// ```text
    /// find-window [-CNTZ] [-t target-pane] match-string
    /// (alias: findw)
    /// ```
    pub fn find_window() {
        unimplemented!();
    }


    /// ```text
    /// join-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    /// (alias: joinp)
    /// ```
    pub fn join_pane() {
        unimplemented!();
    }


    /// ```text
    /// kill-pane [-a] [-t target-pane]
    /// (alias: killp)
    /// ```
    pub fn kill_pane() {
        unimplemented!();
    }


    /// ```text
    /// kill-window [-a] [-t target-window]
    /// (alias: killw)
    /// ```
    pub fn kill_window(&self, all: Option<bool>, target_window: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) { args.push(a_KEY); }
        target_window.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::KILL_WINDOW, &args)?;
        Ok(output.status.success())
    }


    /// ```text
    /// last-pane [-de] [-t target-window]
    /// (alias: lastp)
    /// ```
    pub fn last_pane() {
        unimplemented!();
    }


    /// ```text
    /// last-window [-t target-session]
    /// (alias: last)
    /// ```
    pub fn last_window() {
        unimplemented!();
    }


    /// ```text
    /// link-window [-adk] [-s src-window] [-t dst-window]
    /// (alias: linkw)
    /// ```
    pub fn link_window() {
        unimplemented!();
    }


    /// ```text
    /// tmux list-panes [-as] [-F format] [-t target]
    /// (alias: lsp)
    /// ```
    pub fn list_panes(&self, all: bool, session: bool, format: Option<&str>, target: Option<&str>)
        -> Result<String, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if all { args.push(a_KEY); }
        if session { args.push(s_KEY); }
        format.and_then(|s| Some(args.extend_from_slice(&[F_KEY, &s])));
        target.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::LIST_PANES, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }


    /// ```text
    /// tmux list-windows [-a] [-F format] [-t target-session]
    /// (alias: lsw)
    /// ```
    pub fn list_windows(&self, all: bool, format: Option<&str>, target_session: Option<&str>)
        -> Result<String, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if all { args.push(a_KEY); }
        format.and_then(|s| Some(args.extend_from_slice(&[F_KEY, s])));
        target_session.and_then(|s| Some(args.extend_from_slice(&[t_KEY, s])));
        let output = self.subcommand(TmuxInterface::LIST_WINDOWS, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }


    /// ```text
    /// move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    /// (alias: movep)
    /// ```
    pub fn move_pane() {
        unimplemented!();
    }


    /// ```text
    /// tmux move-window [-ardk] [-s src-window] [-t dst-window]
    /// (alias: movew)
    /// ```
    pub fn move_window() {
        unimplemented!();
    }


    /// ```text
    /// tmux new-window [-adkP] [-c start-directory] [-F format] [-n window-name] [-t target-window] [shell-command]
    /// (alias: neww)
    /// ```
    pub fn new_window(&self, new_window: NewWindow) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if new_window.add.unwrap_or(false) { args.push(a_KEY); }
        if new_window.detached.unwrap_or(false) { args.push(d_KEY); }
        if new_window.kill.unwrap_or(false) { args.push(k_KEY); }
        if new_window.print.unwrap_or(false) { args.push(P_KEY); }
        new_window.cwd.as_ref().and_then(|s| Some(args.extend_from_slice(&[c_KEY, &s])));
        new_window.window_name.as_ref().and_then(|s| Some(args.extend_from_slice(&[n_KEY, &s])));
        new_window.format.as_ref().and_then(|s| Some(args.extend_from_slice(&[F_KEY, &s])));
        new_window.target_window.as_ref().and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        new_window.shell_command.as_ref().and_then(|s| Some(args.push(&s)));
        let output = self.subcommand(TmuxInterface::NEW_WINDOW, &args)?;
        Ok(output.status.success())
    }


    /// ```text
    /// pipe-pane [-IOo] [-t target-pane] [shell-command]
    /// (alias: pipep)
    /// ```
    pub fn pipe_pane() {
        unimplemented!();
    }


    /// ```text
    /// previous-layout [-t target-window]
    /// (alias: prevl)
    /// ```
    pub fn previous_layout() {
        unimplemented!();
    }


    /// ```text
    /// previous-window [-a] [-t target-session]
    /// (alias: prev)
    /// ```
    pub fn previous_window() {
        unimplemented!();
    }


    /// ```text
    /// rename-window [-t target-window] new-name
    /// (alias: renamew)
    /// ```
    pub fn rename_window(&self, target_window: Option<&str>, new_name: &str) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        target_window.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        args.push(new_name);
        let output = self.subcommand(TmuxInterface::RENAME_WINDOW, &args)?;
        Ok(output.status.success())
    }


    /// ```text
    /// resize-pane [-DLMRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
    /// (alias: resizep)
    /// ```
    pub fn resize_pane() {
        unimplemented!();
    }


    /// ```text
    /// resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
    /// (alias: resizew)
    /// ```
    pub fn resize_window() {
        unimplemented!();
    }


    /// ```text
    /// respawn-pane [-k] [-c start-directory] [-e environment] [-t target-pane] [shell-command]
    /// (alias: respawnp)
    /// ```
    pub fn respawn_pane() {
        unimplemented!();
    }


    /// ```text
    /// respawn-window [-k] [-c start-directory] [-e environment] [-t target-window] [shell-command]
    /// (alias: respawnw)
    /// ```
    pub fn respawn_window() {
        unimplemented!();
    }


    /// ```text
    /// rotate-window [-DU] [-t target-window]
    /// (alias: rotatew)
    /// ```
    pub fn rotate_window() {
        unimplemented!();
    }


    /// ```text
    /// select-layout [-Enop] [-t target-pane] [layout-name]
    /// (alias: selectl)
    /// ```
    pub fn select_layout() {
        unimplemented!();
    }


    /// ```text
    /// tmux select-pane [-DdegLlMmRU] [-P style] [-T title] [-t target-pane]
    /// (alias: selectp)
    /// ```
    pub fn select_pane(&self, select_pane: &SelectPane) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if select_pane.down.unwrap_or(false) { args.push(D_KEY); }
        if select_pane.disable.unwrap_or(false) { args.push(d_KEY); }
        if select_pane.enable.unwrap_or(false) { args.push(e_KEY); }
        if select_pane.show_style.unwrap_or(false) { args.push(g_KEY); }
        if select_pane.left.unwrap_or(false) { args.push(L_KEY); }
        if select_pane.last.unwrap_or(false) { args.push(l_KEY); }
        if select_pane.set_marked.unwrap_or(false) { args.push(M_KEY); }
        if select_pane.clear_marked.unwrap_or(false) { args.push(m_KEY); }
        if select_pane.right.unwrap_or(false) { args.push(R_KEY); }
        if select_pane.up.unwrap_or(false) { args.push(U_KEY); }
        select_pane.style.and_then(|s| Some(args.extend_from_slice(&[P_KEY, s])));
        select_pane.title.and_then(|s| Some(args.extend_from_slice(&[T_KEY, s])));
        select_pane.target_pane.and_then(|s| Some(args.extend_from_slice(&[t_KEY, s])));
        let output = self.subcommand(TmuxInterface::SELECT_PANE, &args)?;
        Ok(output.status.success())
    }


    /// ```text
    /// tmux select-window [-lnpT] [-t target-window]
    /// (alias: selectw)
    /// ```
    pub fn select_window(&self, select_window: &SelectWindow) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if select_window.last.unwrap_or(false) { args.push(l_KEY); }
        if select_window.next.unwrap_or(false) { args.push(n_KEY); }
        if select_window.previous.unwrap_or(false) { args.push(p_KEY); }
        if select_window.switch.unwrap_or(false) { args.push(T_KEY); }
        select_window.target_window.and_then(|s| Some(args.extend_from_slice(&[t_KEY, s])));
        let output = self.subcommand(TmuxInterface::SELECT_WINDOW, &args)?;
        Ok(output.status.success())
    }


    /// ```text
    /// tmux split-window [-bdfhvP] [-c start-directory] [-l size | -p percentage]
    /// [-t target-pane] [shell-command] [-F format]
    /// (alias: splitw)
    /// ```
    pub fn split_window(&self, split_window: &SplitWindow) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if split_window.before.unwrap_or(false) { args.push(b_KEY); }
        if split_window.detached.unwrap_or(false) { args.push(d_KEY); }
        if split_window.full.unwrap_or(false) { args.push(f_KEY); }
        if split_window.horizontal.unwrap_or(false) { args.push(h_KEY); }
        if split_window.vertical.unwrap_or(false) { args.push(v_KEY); }
        if split_window.print.unwrap_or(false) { args.push(P_KEY); }
        split_window.cwd.as_ref().and_then(|s| Some(args.extend_from_slice(&[c_KEY, &s])));
        let mut s;
        if let Some(size) = split_window.size {
            s = size.to_string();
            args.extend_from_slice(&[l_KEY, &s]);
        }
        let mut p;
        if let Some(percentage) = split_window.percentage {
            p = percentage.to_string();
            args.extend_from_slice(&[p_KEY, &p]);
        }
        split_window.target_pane.as_ref().and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        split_window.shell_command.as_ref().and_then(|s| Some(args.push(&s)));
        split_window.format.as_ref().and_then(|s| Some(args.extend_from_slice(&[F_KEY, &s])));
        let output = self.subcommand(TmuxInterface::SPLIT_WINDOW, &args)?;
        Ok(output.status.success())
    }


    /// ```text
    /// swap-pane [-dDU] [-s src-pane] [-t dst-pane]
    /// (alias: swapp)
    /// ```
    pub fn swap_pane() {
        unimplemented!();
    }


    /// ```text
    /// swap-window [-d] [-s src-window] [-t dst-window]
    /// (alias: swapw)
    /// ```
    pub fn swap_window() {
        unimplemented!();
    }


    /// ```text
    /// unlink-window [-k] [-t target-window]
    /// (alias: unlinkw)
    /// ```
    pub fn unlink_window() {
        unimplemented!();
    }


}
