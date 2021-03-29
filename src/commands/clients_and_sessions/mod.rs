#[cfg(feature = "tmux_1_1")]
use crate::LockClient;
#[cfg(feature = "tmux_1_1")]
use crate::LockSession;
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

/// All functions from man tmux "Clients and Sessions" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS))
impl<'a> TmuxCommand<'a> {
    #[cfg(feature = "tmux_0_8")]
    pub fn attach_session(&self) -> AttachSession<'a> {
        AttachSession::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn detach_client(&self) -> DetachClient<'a> {
        DetachClient::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn has_session(&self) -> HasSession<'a> {
        HasSession::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn kill_server(&self) -> KillServer<'a> {
        KillServer::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn kill_session(&self) -> KillSession<'a> {
        KillSession::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn list_clients(&self) -> ListClients<'a> {
        ListClients::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn list_commands(&self) -> ListCommands<'a> {
        ListCommands::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn list_sessions(&self) -> ListSessions<'a> {
        ListSessions::from(self)
    }

    #[cfg(feature = "tmux_1_1")]
    pub fn lock_client(&self) -> LockClient<'a> {
        LockClient::from(self)
    }

    #[cfg(feature = "tmux_1_1")]
    pub fn lock_session(&self) -> LockSession<'a> {
        LockSession::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn new_session(&self) -> NewSession<'a> {
        NewSession::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn refresh_client(&self) -> RefreshClient<'a> {
        RefreshClient::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn rename_session(&self) -> RenameSession<'a> {
        RenameSession::from(self)
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn show_messages(&self) -> ShowMessages<'a> {
        ShowMessages::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn source_file(&self) -> SourceFile<'a> {
        SourceFile::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn start_server(&self) -> StartServer<'a> {
        StartServer::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn suspend_client(&self) -> SuspendClient<'a> {
        SuspendClient::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn switch_client(&self) -> SwitchClient<'a> {
        SwitchClient::from(self)
    }
}
