use std::sync::Arc;

use chrono::{Offset, TimeZone};
use tracing_subscriber::filter::LevelFilter;
use walle_core::{obc::ImplOBC, OneBot};
use walle_k::{load_from_file, KHandler};

const LOG_PATH: &str = "log";

#[tokio::main]
async fn main() {
    if let Some(config) = load_from_file() {
        let level: LevelFilter = config.meta.log_level.into();
        let offset = chrono::Local
            .timestamp_opt(0, 0)
            .unwrap()
            .offset()
            .fix()
            .local_minus_utc();
        let timer = tracing_subscriber::fmt::time::OffsetTime::new(
            time::UtcOffset::from_whole_seconds(offset).unwrap(),
            time::macros::format_description!(
                "[year repr:last_two]-[month]-[day] [hour]:[minute]:[second]"
            ),
        );
        let filter = tracing_subscriber::filter::Targets::new()
            .with_default(LevelFilter::INFO)
            .with_targets([
                (walle_k::WALLE_K, level),
                (walle_core::WALLE_CORE, level),
                (walle_core::obc::OBC, level),
                (kook::KOOK, level),
            ]);
        let file_appender =
            tracing_appender::rolling::daily(LOG_PATH, format!("{}.log", walle_k::WALLE_K));
        use tracing_subscriber::{
            prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Layer,
        };
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_timer(timer.clone())
                    .with_filter(filter),
            )
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file_appender)
                    .with_timer(timer)
                    .with_ansi(false)
                    .with_filter(
                        tracing_subscriber::filter::Targets::new().with_default(LevelFilter::WARN),
                    ),
            )
            .init();
        let ob = Arc::new(OneBot::new(
            KHandler::default(),
            ImplOBC::new(walle_k::WALLE_K.to_owned()),
        ));
        ob.start(config.kook, config.onebot, true).await.unwrap();
        ob.wait_all().await;
    }
}
