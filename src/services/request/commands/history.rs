use std::sync::Arc;

use tokio::sync::oneshot::{self, Receiver};

use crate::{
    services::request::{
        commands::{CommandRequestService as Command, CommandsFactory},
        entity::RequestData,
    },
    utils::uuid::UUID,
};

impl CommandsFactory {
    pub fn rollback_request_data(id: UUID) -> Command {
        Box::new(move |mut service| {
            service.rollback_request_data(id);
            Ok(service)
        })
    }
}
