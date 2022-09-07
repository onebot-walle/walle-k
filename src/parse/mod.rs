use kook::prelude::{Event as _KookEvent, EventExtra};
use walle_core::event::Event;

mod action;
mod message;
pub use action::KookAction;
pub use message::*;

type KookEvent = _KookEvent<EventExtra>;

pub async fn event_parse(_event: KookEvent) -> Event {
    todo!()
}
