//! Helper functions and filters.
//!
use std::net::{IpAddr, SocketAddr};
use warp::Filter;

// Get the first IP address from a comma-separared list.
fn parse_xff(s: &str) -> Option<SocketAddr> {
    s.split(",").next()
        .map(|s| s.trim())
        .and_then(|s| {
            // Now try to parse as IpAddr or SocketAddr.
            s.parse::<IpAddr>().map(|i| SocketAddr::new(i, 0))
                .or_else(|_| s.parse::<SocketAddr>())
                .ok()
        })
}

// Get the first for=<ipaddress>.
fn parse_fwd(s: &str) -> Option<SocketAddr> {
    // Of a list of comma-separated fields, get the first one.
    let field = s.split(",").map(|s| s.trim()).next()?;
    // Then split at ';' into fields again, lowercase, and find "for="
    field.split(";")
        .map(|s| s.trim().to_lowercase())
        .find(|s| s.starts_with("for="))
        .and_then(|s| {
            let s = s[4..].trim_matches('"');
            // Now try to parse as IpAddr or SocketAddr.
            s.parse::<IpAddr>().map(|i| SocketAddr::new(i, 0))
                .or_else(|_| s.parse::<SocketAddr>())
                .ok()
        })
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
                return Some(addr);
            }
        }
        // parse X-Real-Ip, if present.
        if let Some(ref v) = xri {
            if let Some(addr) = parse_xff(v) {
                return Some(addr);
            }
        }
        // parse Forwarded, if present.
        if let Some(ref v) = fwd {
            if let Some(addr) = parse_fwd(v) {
                return Some(addr);
            }
        }
    }
    addr
}

/// Like `warp::addr::remote()` but also takes XFF into account.
pub fn remoteip(do_xff: bool) -> impl Filter<Extract = (Option<SocketAddr>,), Error = warp::reject::Rejection> + Copy {
    warp::addr::remote()
        .and(warp::header::optional::<String>("X-Forwarded-For"))
        .and(warp::header::optional::<String>("X-Real-Ip"))
        .and(warp::header::optional::<String>("Forwarded"))
        .map(move |addr: Option<SocketAddr>, xff: Option<String>, xri: Option<String>, fwd: Option<String>| {
            parse_remoteip(addr, do_xff, xff, xri, fwd)
        })
}

