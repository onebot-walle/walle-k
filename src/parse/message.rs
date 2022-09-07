use std::convert::TryFrom;

use kook::{
    card::{cards_decode, cards_encode, Card, CardItem, CardModule},
    MessageType,
};
use walle_core::{
    error::{WalleError, WalleResult},
    prelude::{OneBot, PushToValueMap},
    resp::{resp_error, RespError},
    segment::{Image, MessageSegment, Segments, Text},
};

#[derive(Debug, OneBot, PushToValueMap)]
#[segment]
pub enum KookSegment {
    Text(Text),
    Image(Image),
}

pub fn k2o(s: &str) -> WalleResult<Segments> {
    let _cards = cards_decode(s).map_err(|e| WalleError::Other(e.to_string()))?;
    todo!()
}

pub fn o2k(mut segments: Segments) -> Result<(String, MessageType), RespError> {
    if segments.len() == 1 {
        match KookSegment::try_from(segments.pop().unwrap()) {
            Ok(segment) => match segment {
                KookSegment::Text(t) => return Ok((t.text, MessageType::Text)),
                KookSegment::Image(i) => return Ok((i.file_id, MessageType::Image)), //todo
            },
            Err(e) => return Err(resp_error::unsupported_segment(e.to_string())),
        }
    }
    todo!()
}
