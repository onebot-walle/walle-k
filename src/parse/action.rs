use walle_core::action::*;
use walle_core::prelude::{PushToValueMap, TryFromAction};

#[derive(Debug, PushToValueMap, TryFromAction)]
pub enum KookAction {
    // meta
    GetLatestEvents(GetLatestEvents),
    GetSupportedActions,
    GetStatus,
    GetVersion,
    // message
    SendMessage(SendMessage),
}
