#[macro_use]
extern crate clap;

use std::net::{AddrParseError, IpAddr, SocketAddr};
use std::path::PathBuf;

use futures::stream::{FuturesUnordered, StreamExt};
use once_cell::sync::Lazy;
use tokio::task;

mod ip;
mod api;

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

    let dir = matches.value_of("DIR").map(|s| s.to_string());
    let do_xff = matches.is_present("XFF");
    let routes = api::routes(do_xff, dir);

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

