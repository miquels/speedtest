//! All the actual API handlers.
//!
use std::io;
use std::net::SocketAddr;
use std::time::{Duration, SystemTime};

use futures::future::{ok, err, FutureExt};
use futures::sink::SinkExt;
use futures::stream::{FuturesUnordered, StreamExt};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use tokio::time::{timeout, timeout_at, Elapsed, Instant};
use warp::filters::ws::{Message, WebSocket as WsStream};
use warp::{Filter, filters::BoxedFilter, Reply};
use warp::http::header::{HeaderMap, HeaderValue};

const SEND_TIMEOUT: Duration = Duration::from_secs(20);
const SINK_TIMEOUT: Duration = Duration::from_secs(20);

const BLOB_SIZE: usize = 1_000_0000;

static BLOB: Lazy<Vec<u8>> = Lazy::new(|| {
    let mut v = Vec::with_capacity(BLOB_SIZE);
    v.resize(BLOB_SIZE, 0);
    v
});

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("api: parsing json: {}", source))]
    JsonParse { source: serde_json::error::Error },
    #[snafu(display("api: serializing json: {}", source))]
    JsonSerialize { source: serde_json::error::Error },
    #[snafu(display("api: websocket: {}: {}", msg, source))]
    WebSocket {
        source: warp::Error,
        msg: &'static str,
    },
    #[snafu(display("api: message: not text"))]
    NotText,
    #[snafu(display("api_send: timeout"))]
    SendTimeout {
        source: Elapsed,
    },
    #[snafu(display("api_sink: timeout"))]
    SinkTimeout {
        source: Elapsed,
    },
    #[snafu(display("api: {}: {}", msg, source))]
    IoError {
        msg: &'static str,
        source: io::Error,
    },
}

// Result alias helper.
type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Serialize, Deserialize, Debug)]
struct SourceCmd {
    #[serde(rename = "Download", default)]
    download: String,
    #[serde(rename = "MessageSize", default)]
    message_size: usize,
    #[serde(rename = "MessageCount", default)]
    message_count: usize,
    #[serde(rename = "Period", default)]
    period: usize,
}

#[derive(Serialize, Deserialize)]
struct SinkResponse {
    timestamp: f64,
    messagesize: usize,
}

#[derive(Serialize, Deserialize)]
struct IpResponse {
    remoteip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    remoteport: Option<u16>,
}

// Time since the epoch in microseconds.
fn unix_microseconds() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_micros() as u64,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

// Send data to the client (download test).
async fn api_source(stream: WsStream) -> Result<()> {
    let (mut tx, mut rx) = stream.split();

    // read initial message. If at this point we hit EOF or some
    // error occurs, it's just the client that went away.
    let msg = match rx.next().await {
        Some(Ok(msg)) => msg,
        _ => return Ok(()),
    };
    if !msg.is_text() {
        log::error!("api_source: initial message is not text");
        return Ok(());
    }
    let text = msg.to_str().map_err(|_| Error::NotText)?;
    let mut cmd: SourceCmd = serde_json::from_str(&text).context(JsonParse {})?;

    // default size if 0.
    if cmd.message_size == 0 {
        cmd.message_size = 100_000;
    }
    // minimum size is 125.
    if cmd.message_size < 125 {
        cmd.message_size = 125;
    }
    // max size is 1M
    if cmd.message_size > BLOB_SIZE {
        cmd.message_size = BLOB_SIZE;
    }

    log::debug!("api_source: initial cmd: {:?}", cmd);

    // sender will just send data.
    let data = &BLOB[0..cmd.message_size];
    let sender = async move {
        loop {
            let t = unix_microseconds().to_be_bytes().to_vec();
            tx.send(Message::binary(t))
                .await
                .context(WebSocket { msg: "write" })?;
            tx.send(Message::binary(data))
                .await
                .context(WebSocket { msg: "write" })?;
        }
    };

    // receiver just waits for a message - any message.
    let receiver = async move {
        timeout(SEND_TIMEOUT, rx.next()).await.map(|_| ()).context(SendTimeout{})
    };

    // run sender and receiver in this task, so that if this task is
    // dropped, sender and receiver disappear as well.
    let mut local_executor = FuturesUnordered::new();
    local_executor.push(sender.boxed());
    local_executor.push(receiver.boxed());

    if let Some(Err(e)) = local_executor.next().await {
        log::debug!("api_source: {}", e);
    }
    Ok(())
}

// Sink.
async fn api_sink(mut stream: WsStream) -> Result<()> {
    log::debug!("api_sink: start");
    let task = async {
        while let Some(msg) = stream.next().await {
            let msg = msg.context(WebSocket { msg: "read" })?;
            if msg.is_close() {
                break;
            }
            if !msg.is_binary() {
                continue;
            }
            let resp = serde_json::to_string(&SinkResponse {
                timestamp: unix_microseconds() as f64 / 1000f64,
                messagesize: msg.as_bytes().len(),
            })
            .context(JsonSerialize {})?;
            log::debug!("api_sink: send {}", resp);
            stream
                .send(Message::text(resp))
                .await
                .context(WebSocket { msg: "write" })?
        }
        Ok::<(), Error>(())
    };

    match timeout_at(Instant::now() + SINK_TIMEOUT, task).await.context(SinkTimeout{}) {
        Err(e) | Ok(Err(e)) => log::debug!("api_sink: {}", e),
        Ok(Ok(_)) => log::debug!("api_sink: done"),
    }
    Ok(())
}

// Return IP address via websocket message.
//
// This fixes some issues with HTTP/2 connection re-use,
// as websockets are HTTP/1.1 only.
async fn api_wsip(mut stream: WsStream, addr: Option<SocketAddr>) -> Result<()> {
    let (mut remoteip, remoteport) = match addr {
        Some(SocketAddr::V4(sa)) => (sa.ip().to_string(), sa.port()),
        Some(SocketAddr::V6(sa)) => (sa.ip().to_string(), sa.port()),
        None => ("unknown".to_string(), 0),
    };
    if remoteip.starts_with("::ffff:") {
        remoteip = remoteip.replace("::ffff:", "");
    }
    let resp = serde_json::to_string(&IpResponse {
        remoteip,
        remoteport: if remoteport != 0 { Some(remoteport) } else { None },
    }).context(JsonSerialize {})?;
    log::debug!("api_wsip: send {}", resp);
    stream
        .send(Message::text(resp))
        .await
        .context(WebSocket { msg: "write" })?;
    Ok(())
}

pub fn routes(do_xff: bool, dir: Option<String>) -> BoxedFilter<(impl Reply,)> {

    let sink = warp::path!("speedtest" / "sink")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(move |websocket| async move {
                if let Err(e) = api_sink(websocket).await {
                    log::warn!("/speedtest/sink: {}", e);
                }
            })
        });

    let source = warp::path!("speedtest" / "source")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(move |websocket| async move {
                if let Err(e) = api_source(websocket).await {
                    log::warn!("/speedtest/source: {}", e);
                }
            })
        });

    let ws_ipaddr = warp::path!("speedtest" / "wsip")
        .and(warp::ws())
        .and(crate::ip::remoteip(do_xff))
        .map(|ws: warp::ws::Ws, addr: Option<SocketAddr>| {
            ws.on_upgrade(move |websocket| async move {
                if let Err(e) = api_wsip(websocket, addr).await {
                    log::warn!("/speedtest/wsip: {}", e);
                }
            })
        });

    let mut headers = HeaderMap::new();
    headers.insert("cache-control", HeaderValue::from_static("no-store"));
    headers.insert("connection", HeaderValue::from_static("close"));

    let ipaddr = warp::get()
        .and(warp::path!("speedtest" / "ip"))
        .and(crate::ip::remoteip(do_xff))
        .map(|addr: Option<SocketAddr>| {
            let (mut remoteip, remoteport) = match addr {
                Some(SocketAddr::V4(sa)) => (sa.ip().to_string(), sa.port()),
                Some(SocketAddr::V6(sa)) => (sa.ip().to_string(), sa.port()),
                None => ("unknown".to_string(), 0),
            };
            if remoteip.starts_with("::ffff:") {
                remoteip = remoteip.replace("::ffff:", "");
            }
            serde_json::to_string(&IpResponse {
                remoteip,
                remoteport: if remoteport != 0 { Some(remoteport) } else { None },
            })
            .unwrap()
        })
        .with(warp::reply::with::headers(headers))
        .with(warp::cors().allow_any_origin());

    let do_dir = dir.is_some();
    let dir = dir.unwrap_or(String::new());
    let index = dir.clone() + "/index.html";

    let root = warp::get()
        .and_then(move || {
            match do_dir {
                true => ok(()),
                false => err(warp::reject::not_found()),
            }
        })
        .untuple_one()
        .and(warp::fs::dir(dir).or(warp::fs::file(index)));

    sink.or(source).or(ws_ipaddr).or(ipaddr).or(root).boxed()
}

