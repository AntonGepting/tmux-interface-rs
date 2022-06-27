#[cfg(feature = "tmux_1_1")]
use crate::LockClient;
#[cfg(feature = "tmux_1_1")]
use crate::LockSession;
#[cfg(feature = "tmux_3_3")]
use crate::ServerAccess;
#[cfg(feature = "tmux_1_2")]
use crate::ShowMessages;
use crate::TmuxCommand;
#[cfg(feature = "tmux_0_8")]
use crate::{
    AttachSession, DetachClient, HasSession, KillServer, KillSession, ListClients, ListCommands,
    ListSessions, NewSession, RefreshClient, RenameSession, SourceFile, StartServer, SuspendClient,
    SwitchClient,
};

/// All functions from man tmux "Clients and Sessions" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS))
#[cfg(feature = "tmux_0_8")]
pub mod attach_session;
#[cfg(feature = "tmux_0_8")]
pub mod detach_client;
#[cfg(feature = "tmux_0_8")]
pub mod has_session;
#[cfg(feature = "tmux_0_8")]
pub mod kill_server;
#[cfg(feature = "tmux_0_8")]
pub mod kill_session;
#[cfg(feature = "tmux_0_8")]
pub mod list_clients;
#[cfg(feature = "tmux_0_8")]
pub mod list_commands;
#[cfg(feature = "tmux_0_8")]
pub mod list_sessions;
#[cfg(feature = "tmux_1_1")]
pub mod lock_client;
#[cfg(feature = "tmux_1_1")]
pub mod lock_session;
#[cfg(feature = "tmux_0_8")]
pub mod new_session;
#[cfg(feature = "tmux_0_8")]
pub mod refresh_client;
#[cfg(feature = "tmux_0_8")]
pub mod rename_session;
#[cfg(feature = "tmux_3_3")]
pub mod server_access;
#[cfg(feature = "tmux_1_2")]
pub mod show_messages;
#[cfg(feature = "tmux_0_8")]
pub mod source_file;
#[cfg(feature = "tmux_0_8")]
pub mod start_server;
#[cfg(feature = "tmux_0_8")]
pub mod suspend_client;
#[cfg(feature = "tmux_0_8")]
pub mod switch_client;

#[cfg(test)]
#[path = "."]
mod clients_and_sessions_tests {
    #[cfg(feature = "tmux_0_8")]
    pub mod attach_session_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod detach_client_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod has_session_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod kill_server_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod kill_session_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod list_clients_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod list_commands_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod list_sessions_tests;
    #[cfg(feature = "tmux_1_1")]
    pub mod lock_client_tests;
    #[cfg(feature = "tmux_1_1")]
    pub mod lock_session_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod new_session_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod refresh_client_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod rename_session_tests;
    #[cfg(feature = "tmux_1_2")]
    pub mod show_messages_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod source_file_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod start_server_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod suspend_client_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod switch_client_tests;
}

/// All functions from man tmux "Clients and Sessions" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS))
impl<'a> TmuxCommand<'a> {
    #[cfg(feature = "tmux_0_8")]
    pub fn attach_session() -> AttachSession<'a> {
        AttachSession::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn detach_client() -> DetachClient<'a> {
        DetachClient::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn has_session() -> HasSession<'a> {
        HasSession::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn kill_server() -> KillServer {
        KillServer::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn kill_session() -> KillSession<'a> {
        KillSession::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn list_clients() -> ListClients<'a> {
        ListClients::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn list_commands() -> ListCommands<'a> {
        ListCommands::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn list_sessions() -> ListSessions<'a> {
        ListSessions::new()
    }

    #[cfg(feature = "tmux_1_1")]
    pub fn lock_client() -> LockClient<'a> {
        LockClient::new()
    }

    #[cfg(feature = "tmux_1_1")]
    pub fn lock_session() -> LockSession<'a> {
        LockSession::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn new_session() -> NewSession<'a> {
        NewSession::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn refresh_client() -> RefreshClient<'a> {
        RefreshClient::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn rename_session() -> RenameSession<'a> {
        RenameSession::new()
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn show_messages() -> ShowMessages<'a> {
        ShowMessages::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn source_file() -> SourceFile<'a> {
        SourceFile::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn start_server() -> StartServer {
        StartServer::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn suspend_client() -> SuspendClient<'a> {
        SuspendClient::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn switch_client() -> SwitchClient<'a> {
        SwitchClient::new()
    }
}

// XXX: generic
impl<'a> From<AttachSession<'a>> for TmuxCommand<'a> {
    fn from(item: AttachSession<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<DetachClient<'a>> for TmuxCommand<'a> {
    fn from(item: DetachClient<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<HasSession<'a>> for TmuxCommand<'a> {
    fn from(item: HasSession<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<KillServer> for TmuxCommand<'a> {
    fn from(item: KillServer) -> Self {
        item.build()
    }
}

impl<'a> From<KillSession<'a>> for TmuxCommand<'a> {
    fn from(item: KillSession<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<ListClients<'a>> for TmuxCommand<'a> {
    fn from(item: ListClients<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<ListCommands<'a>> for TmuxCommand<'a> {
    fn from(item: ListCommands<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<ListSessions<'a>> for TmuxCommand<'a> {
    fn from(item: ListSessions<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<LockClient<'a>> for TmuxCommand<'a> {
    fn from(item: LockClient<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<LockSession<'a>> for TmuxCommand<'a> {
    fn from(item: LockSession<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<NewSession<'a>> for TmuxCommand<'a> {
    fn from(item: NewSession<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<RefreshClient<'a>> for TmuxCommand<'a> {
    fn from(item: RefreshClient<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<RenameSession<'a>> for TmuxCommand<'a> {
    fn from(item: RenameSession<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_3_3")]
impl<'a> From<ServerAccess<'a>> for TmuxCommand<'a> {
    fn from(item: ServerAccess<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<ShowMessages<'a>> for TmuxCommand<'a> {
    fn from(item: ShowMessages<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<SourceFile<'a>> for TmuxCommand<'a> {
    fn from(item: SourceFile<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<StartServer> for TmuxCommand<'a> {
    fn from(item: StartServer) -> Self {
        item.build()
    }
}

impl<'a> From<SuspendClient<'a>> for TmuxCommand<'a> {
    fn from(item: SuspendClient<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<SwitchClient<'a>> for TmuxCommand<'a> {
    fn from(item: SwitchClient<'a>) -> Self {
        item.build()
    }
}
