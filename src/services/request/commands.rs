use crate::services::request::commands::CommandRequestService as Command;
use crate::services::request::service::RequestServiceInstance;
use crate::utils::commands::{CommandClosureType, CommandsUtils, ErrAtomic};

pub type CommandRequestService = CommandClosureType<RequestServiceInstance>;

pub struct CommandsFactory;

impl CommandsFactory {
    pub fn do_nothing() -> Command {
        Box::new(|service| Ok(service))
    }
}

pub mod crud;
pub mod history;
