use kook::prelude::{Event as _KookEvent, EventExtra, SystemExtra};
use walle_core::{event::Event, segment::MessageExt, structs::Selft, util::ValueMap, value_map};

mod action;
mod message;
pub use action::KookAction;
pub use message::*;

type KookEvent = _KookEvent<EventExtra>;

fn build_event(event: &KookEvent, tys: (&str, &str, &str), extra: ValueMap) -> Option<Event> {
    Some(Event {
        id: event.msg_id.clone(),
        time: event.msg_timestamp as f64,
        ty: tys.0.to_owned(),
        detail_type: tys.1.to_owned(),
        sub_type: tys.2.to_owned(),
        extra,
    })
}

pub async fn event_parse(event: KookEvent, user_id: String) -> Option<Event> {
    let selft = Selft {
        user_id,
        platform: crate::WALLE_K.to_owned(),
    };
    match event.extra {
        EventExtra::GroupMessage(ref g) => {
            let segs = match kook_message_to_segment(&event.content, event.ty) {
                Ok(segs) => segs,
                Err(e) => {
                    tracing::warn!(target: crate::WALLE_K, "parse segs error:{}", e);
                    return None;
                }
            };
            build_event(
                &event,
                ("message", "channel", ""),
                value_map! {
                    "message_id": event.msg_id,
                    "alt_message": segs.extract_plain_text(),
                    "message": segs,
                    "guild_id": g.guild_id,
                    "channel_id": event.target_id,
                    "user_id": g.author.id,
                    "self": selft
                },
            )
        }
        EventExtra::PersonMessage(p) => {
            let segs = match kook_message_to_segment(&event.content, event.ty) {
                Ok(segs) => segs,
                Err(e) => {
                    tracing::warn!(target: crate::WALLE_K, "parse segs error:{}", e);
                    return None;
                }
            };
            Some(Event {
                id: event.msg_id.clone(),
                time: event.msg_timestamp as f64,
                ty: "message".to_owned(),
                detail_type: "private".to_owned(),
                sub_type: "".to_owned(),
                extra: value_map! {
                    "message_id": event.msg_id,
                    "alt_message": segs.extract_plain_text(),
                    "message": segs,
                    "user_id": p.author.id,
                    "self": selft
                },
            })
        }
        EventExtra::System(SystemExtra::AddedReaction {
            channel_id,
            emoji,
            user_id,
            msg_id,
        }) => Some(Event {
            id: event.msg_id,
            time: event.msg_timestamp as f64,
            ty: "message".to_owned(),
            detail_type: "channel".to_owned(),
            sub_type: "reaction_increase".to_owned(),
            extra: value_map! {
                "message_id": msg_id,
                "alt_message": emoji.name,
                "message": [{
                    "type": "face",
                    "data": {
                        "id": emoji.id,
                        "name": emoji.name
                    }
                }],
                "guild_id": event.target_id,
                "channel_id": channel_id,
                "user_id": user_id,
                "self": selft
            },
        }),
        EventExtra::System(SystemExtra::DeletedReaction {
            channel_id,
            emoji,
            user_id,
            msg_id,
        }) => Some(Event {
            id: event.msg_id,
            time: event.msg_timestamp as f64,
            ty: "notice".to_owned(),
            detail_type: "reaction_decrease".to_owned(),
            sub_type: "channel".to_owned(),
            extra: value_map! {
                "message_id": msg_id,
                "emoji_name": emoji.name,
                "emoji_id": emoji.id,
                "guild_id": event.target_id,
                "channel_id": channel_id,
                "user_id": user_id,
                "self": selft
            },
        }),
        EventExtra::System(SystemExtra::PrivateAddedReaction {
            msg_id,
            user_id,
            emoji,
            ..
        }) => Some(Event {
            id: event.msg_id,
            time: event.msg_timestamp as f64,
            ty: "message".to_owned(),
            detail_type: "private".to_owned(),
            sub_type: "reaction_increase".to_owned(),
            extra: value_map! {
                "message_id": msg_id,
                "alt_message": emoji.name,
                "message": [{
                    "type": "face",
                    "data": {
                        "id": emoji.id,
                        "name": emoji.name
                    }
                }],
                "user_id": user_id,
                "self": selft
            },
        }),
        EventExtra::System(SystemExtra::PrivateDeletedReaction {
            msg_id,
            user_id,
            emoji,
            ..
        }) => Some(Event {
            id: event.msg_id,
            time: event.msg_timestamp as f64,
            ty: "notice".to_owned(),
            detail_type: "reaction_decrease".to_owned(),
            sub_type: "private".to_owned(),
            extra: value_map! {
                "message_id": msg_id,
                "emoji_name": emoji.name,
                "emoji_id": emoji.id,
                "user_id": user_id,
                "self": selft
            },
        }),
        EventExtra::System(SystemExtra::UpdateMessage {
            channel_id,
            content,
            msg_id,
            ..
        }) => Some(Event {
            id: event.msg_id.clone(),
            time: event.msg_timestamp as f64,
            ty: "message".to_owned(),
            detail_type: "channel".to_owned(),
            sub_type: "update".to_owned(),
            extra: value_map! {
                "message_id": msg_id,
                "alt_message": content,
                "message": [{
                    "type": "text",
                    "data": {
                        "text": content
                    }
                }],
                "guild_id": event.target_id,
                "channel_id": channel_id,
                "user_id": event.author_id,
                "self": selft
            },
        }),
        EventExtra::System(SystemExtra::UpdatedPrivateMessage {
            msg_id,
            author_id,
            content,
            ..
        }) => Some(Event {
            id: event.msg_id.clone(),
            time: event.msg_timestamp as f64,
            ty: "message".to_owned(),
            detail_type: "private".to_owned(),
            sub_type: "".to_owned(),
            extra: value_map! {
                "message_id": msg_id,
                "alt_message": content,
                "message": [{
                    "type": "text",
                    "data": {
                        "text": content
                    }
                }],
                "user_id": author_id,
                "self": selft
            },
        }),
        EventExtra::System(SystemExtra::DeletedMessage { channel_id, msg_id }) => Some(Event {
            id: event.msg_id,
            time: event.msg_timestamp as f64,
            ty: "notice".to_owned(),
            detail_type: "channel_message_delete".to_owned(),
            sub_type: "".to_owned(),
            extra: value_map! {
                "guild_id": event.target_id,
                "channel_id": channel_id,
                "message_id": msg_id,
                "user_id": "",
                "operator_id": event.author_id
            },
        }),
        _ => None,
    }
}
