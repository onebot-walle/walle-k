use std::{convert::TryFrom, sync::Arc};

use kook::{prelude::Config as KookConfig, Kook};
use once_cell::sync::OnceCell;
use tokio::task::JoinHandle;
use walle_core::{
    action::{Action, SendMessage},
    alt::ColoredAlt,
    prelude::{async_trait, WalleError, WalleResult},
    resp::{resp_error, Resp, RespError},
    structs::{Selft, SendMessageResp, Version},
    ActionHandler, EventHandler, GetSelfs, GetStatus, GetVersion, OneBot,
};

use crate::parse::{event_parse, segments_to_str, KookAction};

pub type RespReault = Result<Resp, RespError>;

pub fn to_resp(r: RespReault) -> WalleResult<Resp> {
    Ok(match r {
        Ok(r) => r,
        Err(e) => e.into(),
    })
}

#[derive(Default)]
pub struct KHandler {
    _self_id: OnceCell<String>,
    _kook: OnceCell<Arc<Kook>>,
}

impl KHandler {
    pub fn self_id(&self) -> String {
        self._self_id.get().cloned().unwrap_or_default()
    }
    pub fn kook(&self) -> Result<&Arc<Kook>, RespError> {
        self._kook
            .get()
            .ok_or(resp_error::bad_handler("Kook not inited"))
    }
}

#[async_trait]
impl GetSelfs for KHandler {
    async fn get_selfs(&self) -> Vec<walle_core::structs::Selft> {
        vec![Selft {
            user_id: self.self_id(),
            platform: kook::KOOK.to_owned(),
        }]
    }
    async fn get_impl(&self, _selft: &Selft) -> String {
        crate::WALLE_K.to_owned()
    }
}

#[async_trait]
impl GetStatus for KHandler {
    async fn is_good(&self) -> bool {
        self.kook().is_ok()
    }
}

impl GetVersion for KHandler {
    fn get_version(&self) -> Version {
        Version {
            implt: crate::WALLE_K.to_owned(),
            version: crate::VERSION.to_owned(),
            onebot_version: 12.to_string(),
        }
    }
}

#[async_trait]
impl ActionHandler for KHandler {
    type Config = KookConfig;
    async fn start<AH, EH>(
        &self,
        ob: &Arc<OneBot<AH, EH>>,
        config: Self::Config,
    ) -> WalleResult<Vec<JoinHandle<()>>>
    where
        AH: ActionHandler + Send + Sync + 'static,
        EH: EventHandler + Send + Sync + 'static,
    {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let kook = Arc::new(Kook::new_from_config(config, tx));
        self._self_id
            .set(
                kook.get_me()
                    .await
                    .map_err(|e| WalleError::Other(e.to_string()))?
                    .id,
            )
            .ok();
        self._kook.set(kook.clone()).ok();
        let ob = ob.clone();
        let mut tasks = vec![];
        let id = self.self_id();
        tasks.push(tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                if let Some(event) = event_parse(event, id.clone()).await {
                    tracing::info!(target: crate::WALLE_K, "{}", event.colored_alt());
                    ob.handle_event(event) //todo
                        .await
                        .ok();
                }
            }
        }));
        tasks.push(tokio::spawn(async move {
            kook.start_ws().await.ok();
        }));
        Ok(tasks)
    }
    async fn call<AH, EH>(&self, action: Action, _ob: &Arc<OneBot<AH, EH>>) -> WalleResult<Resp>
    where
        AH: ActionHandler + Send + Sync + 'static,
        EH: EventHandler + Send + Sync + 'static,
    {
        let a = action.action.clone();
        match KookAction::try_from(action) {
            Ok(action) => match action {
                KookAction::GetLatestEvents(_) => Ok(Vec::<()>::new().into()), //todo
                KookAction::GetStatus => Ok(self.get_status().await.into()),
                KookAction::GetSupportedActions => Ok(vec![
                    "get_latest_events",
                    "get_supported_actions",
                    "get_status",
                    "get_version",
                    "send_message",
                ]
                .into()),
                KookAction::GetVersion => Ok(Version {
                    implt: crate::WALLE_K.to_owned(),
                    onebot_version: "12".to_owned(),
                    version: crate::VERSION.to_owned(),
                }
                .into()),

                KookAction::SendMessage(c) => to_resp(self.send_message(c).await),
            },
            Err(_) => Ok(resp_error::unsupported_action(a).into()),
        }
    }
}

impl KHandler {
    pub async fn send_message(&self, content: SendMessage) -> RespReault {
        match content.detail_type.as_str() {
            "channel" => {
                if let Some(ref channel_id) = content.channel_id {
                    match segments_to_str(self.kook()?, content.message).await {
                        Ok((s, ty)) => {
                            let r = self
                                .kook()?
                                .create_message(
                                    Some(ty.into()),
                                    channel_id,
                                    &s,
                                    None,
                                    None, //todo
                                    None, //todo
                                )
                                .await
                                .map_err(|e| resp_error::bad_handler(e.to_string()))?;
                            Ok(SendMessageResp {
                                message_id: r.msg_id,
                                time: r.msg_timestamp as f64,
                            }
                            .into())
                        }
                        Err(e) => Ok(e.into()),
                    }
                } else {
                    Err(resp_error::bad_param("channel_id required"))
                }
            }
            "private" => {
                if let Some(ref user_id) = content.user_id {
                    match segments_to_str(self.kook()?, content.message).await {
                        Ok((s, ty)) => {
                            let r = self
                                .kook()?
                                .create_direct_message(
                                    Some(user_id),
                                    None,
                                    &s,
                                    Some(ty as u8),
                                    None, //todo
                                    None, //todo
                                )
                                .await
                                .map_err(|e| resp_error::bad_handler(e.to_string()))?;
                            Ok(SendMessageResp {
                                message_id: r.msg_id,
                                time: r.msg_timestamp as f64,
                            }
                            .into())
                        }
                        Err(e) => Ok(e.into()),
                    }
                } else {
                    Err(resp_error::bad_param("user_id required"))
                }
            }
            ty => Ok(resp_error::unsupported_param(format!("detail_type:{}", ty)).into()),
        }
    }
}
