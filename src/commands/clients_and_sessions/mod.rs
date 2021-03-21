use crate::TmuxCommand;
use crate::{
    AttachSession, DetachClient, HasSession, KillServer, KillSession, ListClients, ListCommands,
    ListSessions, LockClient, LockSession, NewSession, RefreshClient, RenameSession, ShowMessages,
    SourceFile, StartServer, SuspendClient, SwitchClient,
};

/// All functions from man tmux "Clients and Sessions" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS)
#[cfg(feature = "tmux_1_0")]
pub mod attach_session;
#[cfg(feature = "tmux_1_0")]
pub mod detach_client;
#[cfg(feature = "tmux_1_0")]
pub mod has_session;
#[cfg(feature = "tmux_1_0")]
pub mod kill_server;
#[cfg(feature = "tmux_1_0")]
pub mod kill_session;
#[cfg(feature = "tmux_1_0")]
pub mod list_clients;
#[cfg(feature = "tmux_1_0")]
pub mod list_commands;
#[cfg(feature = "tmux_1_0")]
pub mod list_sessions;
#[cfg(feature = "tmux_1_1")]
pub mod lock_client;
#[cfg(feature = "tmux_1_1")]
pub mod lock_session;
#[cfg(feature = "tmux_1_0")]
pub mod new_session;
#[cfg(feature = "tmux_1_0")]
pub mod refresh_client;
#[cfg(feature = "tmux_1_0")]
pub mod rename_session;
#[cfg(feature = "tmux_1_2")]
pub mod show_messages;
#[cfg(feature = "tmux_1_0")]
pub mod source_file;
#[cfg(feature = "tmux_1_0")]
pub mod start_server;
#[cfg(feature = "tmux_1_0")]
pub mod suspend_client;
#[cfg(feature = "tmux_1_0")]
pub mod switch_client;

#[cfg(feature = "tmux_1_0")]
pub mod attach_session_tests;
#[cfg(feature = "tmux_1_0")]
pub mod detach_client_tests;
#[cfg(feature = "tmux_1_0")]
pub mod has_session_tests;
#[cfg(feature = "tmux_1_0")]
pub mod kill_server_tests;
#[cfg(feature = "tmux_1_0")]
pub mod kill_session_tests;
#[cfg(feature = "tmux_1_0")]
pub mod list_clients_tests;
#[cfg(feature = "tmux_1_0")]
pub mod list_commands_tests;
#[cfg(feature = "tmux_1_0")]
pub mod list_sessions_tests;
#[cfg(feature = "tmux_1_1")]
pub mod lock_client_tests;
#[cfg(feature = "tmux_1_1")]
pub mod lock_session_tests;
#[cfg(feature = "tmux_1_0")]
pub mod new_session_tests;
#[cfg(feature = "tmux_1_0")]
pub mod refresh_client_tests;
#[cfg(feature = "tmux_1_0")]
pub mod rename_session_tests;
#[cfg(feature = "tmux_1_2")]
pub mod show_messages_tests;
#[cfg(feature = "tmux_1_0")]
pub mod source_file_tests;
#[cfg(feature = "tmux_1_0")]
pub mod start_server_tests;
#[cfg(feature = "tmux_1_0")]
pub mod suspend_client_tests;
#[cfg(feature = "tmux_1_0")]
pub mod switch_client_tests;

impl<'a> TmuxCommand<'a> {
    pub fn attach_session(&self) -> AttachSession<'a> {
        AttachSession::from(self)
    }

    pub fn detach_client(&self) -> DetachClient<'a> {
        DetachClient::from(self)
    }

    pub fn has_session(&self) -> HasSession<'a> {
        HasSession::from(self)
    }

    pub fn kill_server(&self) -> KillServer<'a> {
        KillServer::from(self)
    }

    pub fn kill_session(&self) -> KillSession<'a> {
        KillSession::from(self)
    }

    pub fn list_clients(&self) -> ListClients<'a> {
        ListClients::from(self)
    }

    pub fn list_commands(&self) -> ListCommands<'a> {
        ListCommands::from(self)
    }

    pub fn list_sessions(&self) -> ListSessions<'a> {
        ListSessions::from(self)
    }

    pub fn lock_client(&self) -> LockClient<'a> {
        LockClient::from(self)
    }

    pub fn lock_session(&self) -> LockSession<'a> {
        LockSession::from(self)
    }

    pub fn new_session(&self) -> NewSession<'a> {
        NewSession::from(self)
    }

    pub fn refresh_client(&self) -> RefreshClient<'a> {
        RefreshClient::from(self)
    }

    pub fn rename_session(&self) -> RenameSession<'a> {
        RenameSession::from(self)
    }

    pub fn show_messages(&self) -> ShowMessages<'a> {
        ShowMessages::from(self)
    }

    pub fn source_file(&self) -> SourceFile<'a> {
        SourceFile::from(self)
    }

    pub fn start_server(&self) -> StartServer<'a> {
        StartServer::from(self)
    }

    pub fn suspend_client(&self) -> SuspendClient<'a> {
        SuspendClient::from(self)
    }

    pub fn switch_client(&self) -> SwitchClient<'a> {
        SwitchClient::from(self)
    }
}
