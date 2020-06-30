#[macro_use]
extern crate clap;

use std::io;
use std::net::{AddrParseError, IpAddr, SocketAddr};
use std::path::PathBuf;
use std::time::SystemTime;

use futures::future::{ok, err, FutureExt};
use futures::sink::SinkExt;
use futures::stream::{FuturesUnordered, StreamExt};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use tokio::task;
use warp::filters::ws::{Message, WebSocket as WsStream};
use warp::Filter;

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
        let _ = rx.next().await;
        Ok::<_, Error>(())
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
            .context(WebSocket { msg: "write" })?;
    }
    log::debug!("api_sink: done");
    Ok(())
}

#[derive(Debug, PartialEq)]
enum ListenOn {
    V4Only,
    V6Only,
    Both,
}

static LISTEN_ON: Lazy<ListenOn> = Lazy::new(|| {
    use socket2::{Domain, Socket, Type};
    match Socket::new(Domain::ipv6(), Type::stream(), None) {
        Ok(sock) => match sock.only_v6() {
            Ok(true) => ListenOn::Both,
            Ok(false) => ListenOn::V6Only,
            Err(_) => ListenOn::V4Only,
        },
        Err(_) => ListenOn::V4Only,
    }
});

fn add_listener(addr: &str, listen: &mut Vec<(SocketAddr, String)>) -> Result<(), AddrParseError> {
    if let Ok(port) = addr.parse::<u16>() {
        match &*LISTEN_ON {
            &ListenOn::V4Only => {
                listen.push((
                    SocketAddr::new(IpAddr::V4(0u32.into()), port),
                    format!("*:{}", port)
                ));
            }
            &ListenOn::V6Only => {
                listen.push((
                    SocketAddr::new(IpAddr::V6(0u128.into()), port),
                    format!("*:{}", port)
                ));
            }
            &ListenOn::Both => {
                listen.push((
                    SocketAddr::new(IpAddr::V4(0u32.into()), port),
                    format!("0.0.0.0:{}", port),
                ));
                listen.push((
                    SocketAddr::new(IpAddr::V6(0u128.into()), port),
                    format!("[::]:{}", port),
                ));
            }
        }
        return Ok(());
    }
    listen.push((addr.parse::<SocketAddr>()?, addr.to_string()));
    Ok(())
}

macro_rules! die {
    (log => $($tt:tt)*) => ({
        log::error!($($tt)*);
        std::process::exit(1);
    });
    (std => $($tt:tt)*) => ({
        eprintln!($($tt)*);
        std::process::exit(1);
    });
}

fn resolve(dir: &str, file: &str) -> PathBuf {
    let mut p = file.parse::<PathBuf>().unwrap();
    if p.is_relative() && p.metadata().is_err() {
        let mut d = dir.parse::<PathBuf>().unwrap();
        d.push(&p);
        p = d;
    }
    if let Err(e) = p.metadata() {
        die!(std => "{:?}: {}", p, e);
    }
    p
}

// Get the first IP address from a comma-separared list.
fn parse_xff(s: &str) -> Option<IpAddr> {
    s.split(",").next().and_then(|s| s.trim().parse::<IpAddr>().ok())
}

// Get the first by=<ipaddress> from a semicolon-separated list.
fn parse_fwd(s: &str) -> Option<IpAddr> {
    let iter = s.split(";").map(|s| s.trim());
    iter.filter(|s| s.starts_with("by=")).find_map(|s| s[2..].parse::<IpAddr>().ok())
}

fn parse_remoteip(addr: Option<SocketAddr>, xff_headers: bool, xff: Option<String>, xri: Option<String>, fwd: Option<String>) -> Option<SocketAddr> {
    let is_loopback = match addr {
        Some(SocketAddr::V4(ref addr)) => addr.ip().is_loopback(),
        Some(SocketAddr::V6(ref addr)) => addr.ip().is_loopback(),
        None => false,
    };
    if is_loopback || xff_headers {
        // parse X-Forwarded-For, if present.
        if let Some(ref v) = xff {
            if let Some(addr) = parse_xff(v) {
                return Some(SocketAddr::new(addr, 0));
            }
        }
        // parse X-Real-Ip, if present.
        if let Some(ref v) = xri {
            if let Some(addr) = parse_xff(v) {
                return Some(SocketAddr::new(addr, 0));
            }
        }
        // parse Forwarded, if present.
        if let Some(ref v) = fwd {
            if let Some(addr) = parse_fwd(v) {
                return Some(SocketAddr::new(addr, 0));
            }
        }
    }
    addr
}

async fn async_main() {
    let matches = clap_app!(speedtest_server =>
        (version: "0.2")
        (@arg LISTEN: -l --listen +takes_value +multiple "[addr:]port to listen on (4000)")
        (@arg DIR: -d --dir +takes_value "Directory to serve")
        (@arg CHAIN: --chain +takes_value "TLS certificate chain file")
        (@arg KEY: --key +takes_value "TLS certificate key file")
        (@arg XFF: --("--xff-headers") "Use X-Forwarded-For/X-Real-Ip/Forwarded headers")
    )
    .get_matches();

    // Get the listen address(es)
    let mut listen = Vec::new();
    if let Some(listen_args) = matches.values_of("LISTEN") {
        for l in listen_args {
            if let Err(e) = add_listener(l, &mut listen) {
                die!(std => "{}: {}", l, e);
            }
        }
    }
    if listen.len() == 0 {
        add_listener("4000", &mut listen).unwrap();
    }

    let tls = match (matches.value_of("KEY"), matches.value_of("CHAIN")) {
        (Some(k), Some(v)) => {
            let key = resolve("/etc/ssl/private", k);
            let chn = resolve("/etc/ssl/certs", v);
            Some((key, chn))
        },
        (Some(_), None) => die!(std => "missing --chain option"),
        (None, Some(_)) => die!(std => "missing --key option"),
        (None, None) => None,
    };

    let dir = matches.value_of("DIR");
    let do_xff = matches.is_present("XFF");

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

    let ipaddr = warp::get()
        .and(warp::path!("speedtest" / "ip"))
        .and(warp::addr::remote())
        .and(warp::header::optional::<String>("X-Forwarded-For"))
        .and(warp::header::optional::<String>("X-Real-Ip"))
        .and(warp::header::optional::<String>("Forwarded"))
        .map(move |addr: Option<SocketAddr>, xff: Option<String>, xri: Option<String>, fwd: Option<String>| {
            let addr = parse_remoteip(addr, do_xff, xff, xri, fwd);
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
        .with(warp::cors().allow_any_origin());

    let do_dir = dir.is_some();
    let dir = dir.unwrap_or("").to_owned();
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

    let routes = sink.or(source).or(ipaddr).or(root);

    // Run all servers.
    let mut handles = Vec::new();
    for (ref addr, ref name) in &listen {
        let srv = warp::serve(routes.clone());
        if let Some((ref key, ref cert)) = tls {
            // why no try_bind_ephemeral in the TlsServer?
            let srv = srv.tls().key_path(key).cert_path(cert).bind(addr.clone());
            log::info!("Listening on {}", name);
            handles.push(task::spawn(srv));
        } else {
            match warp::serve(routes.clone()).try_bind_ephemeral(addr.clone()) {
                Ok((_, srv)) => {
                    log::info!("Listening on {}", name);
                    handles.push(task::spawn(srv));
                }
                Err(e) => {
                    die!(log => "{}: {}", name, e);
                }
            }
        }
    }

    // The tasks should never return, only on error. So _if_ one
    // returns, abort the entire process.
    let mut task_waiter = FuturesUnordered::new();
    for handle in handles.drain(..) {
        task_waiter.push(handle);
    }
    if let Some(Err(err)) = task_waiter.next().await {
        if let Ok(cause) = err.try_into_panic() {
            if let Some(err) = cause.downcast_ref::<String>() {
                die!(log => "fatal: {}", err);
            }
        }
    }
    die!(log => "server exited unexpectedly");
}

fn main() {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .on_thread_start(|| {
            let hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |info| {
                match info.payload().downcast_ref::<String>() {
                    Some(msg) if msg.contains("error binding to") => {},
                    _ => hook(info),
                }
            }));
        })
        .build()
        .unwrap();
    rt.block_on(async_main());
}

