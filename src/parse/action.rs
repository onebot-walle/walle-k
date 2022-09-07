use walle_core::action::*;
use walle_core::prelude::{OneBot, PushToValueMap};

#[derive(Debug, PushToValueMap, OneBot)]
#[action]
pub enum KookAction {
    // meta
    GetLatestEvents(GetLatestEvents),
    GetSupportedActions,
    GetStatus,
    GetVersion,
    // message
    SendMessage(SendMessage),
}
