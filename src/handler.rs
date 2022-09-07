use std::{convert::TryFrom, sync::Arc};

use kook::{prelude::Config as KookConfig, Kook};
use once_cell::sync::OnceCell;
use tokio::task::JoinHandle;
use walle_core::{
    action::{Action, SendMessage},
    event::Event,
    prelude::{async_trait, WalleError, WalleResult},
    resp::{resp_error, Resp, RespError},
    structs::{Selft, Version},
    ActionHandler, EventHandler, GetSelfs, GetStatus, OneBot,
};

use crate::parse::{event_parse, o2k, KookAction};

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

#[async_trait]
impl ActionHandler<Event, Action, Resp> for KHandler {
    type Config = KookConfig;
    async fn start<AH, EH>(
        &self,
        ob: &Arc<OneBot<AH, EH>>,
        config: Self::Config,
    ) -> WalleResult<Vec<JoinHandle<()>>>
    where
        AH: ActionHandler<Event, Action, Resp> + Send + Sync + 'static,
        EH: EventHandler<Event, Action, Resp> + Send + Sync + 'static,
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
        tasks.push(tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                ob.handle_event(event_parse(event).await).await.ok();
            }
        }));
        tasks.push(tokio::spawn(async move {
            kook.start_ws().await.ok();
        }));
        Ok(tasks)
    }
    async fn call(&self, action: Action) -> WalleResult<Resp> {
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
                    platform: kook::KOOK.to_owned(),
                    onebot_version: "12".to_owned(),
                    version: crate::VERSION.to_owned(),
                }
                .into()),

                KookAction::SendMessage(c) => self.send_message(c).await,
            },
            Err(_) => Ok(resp_error::unsupported_action(a).into()),
        }
    }
}

impl KHandler {
    pub async fn send_message(&self, content: SendMessage) -> WalleResult<Resp> {
        match content.detail_type.as_str() {
            "channel" => match o2k(content.message) {
                Ok((s, ty)) => todo!(),
                Err(e) => Ok(e.into()),
            },
            "private" => {
                todo!()
            }
            ty => Ok(resp_error::unsupported_param(format!("detail_type:{}", ty)).into()),
        }
    }
}
