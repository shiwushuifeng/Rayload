//! UDP multicast discovery for Rayload devices on the same network.
//!
//! Beacons advertise only the device name, address and CPU architecture.
//! Credentials never leave the device here; callers verify the account with
//! an authenticated request before showing a peer as usable.

use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use serde::Serialize;

pub const DISCOVERY_PORT: u16 = 6768;
const BEACON_INTERVAL: Duration = Duration::from_secs(3);
const PEER_TTL: Duration = Duration::from_secs(12);
const MULTICAST_ADDR: Ipv4Addr = Ipv4Addr::new(239, 255, 67, 67);

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveredPeer {
    pub name: String,
    pub address: String,
    pub arch: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fingerprint: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<String>,
}

struct Entry {
    peer: DiscoveredPeer,
    seen: Instant,
}

fn peers() -> &'static Mutex<Vec<Entry>> {
    static PEERS: OnceLock<Mutex<Vec<Entry>>> = OnceLock::new();
    PEERS.get_or_init(|| Mutex::new(Vec::new()))
}

/// Fresh peers seen during the last beacon window.
pub fn current() -> Vec<DiscoveredPeer> {
    let mut peers = peers()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    peers.retain(|entry| entry.seen.elapsed() <= PEER_TTL);
    peers.iter().map(|entry| entry.peer.clone()).collect()
}

/// Starts the beacon listener once per process.
pub fn spawn(api_port: u16, name: String) {
    static STARTED: OnceLock<()> = OnceLock::new();
    if STARTED.set(()).is_err() {
        return;
    }
    std::thread::Builder::new()
        .name("rayload-lan-discovery".into())
        .spawn(move || run(api_port, &name))
        .ok();
}

/// Every usable local address, IPv4 first. Phones with Wi-Fi, VPN or a
/// hotspot can have several addresses at once, so callers must not assume one.
pub fn local_addresses() -> Vec<IpAddr> {
    let mut addresses = if_addrs::get_if_addrs()
        .unwrap_or_default()
        .into_iter()
        .map(|interface| interface.ip())
        .filter(|address| !address.is_loopback() && !address.is_unspecified())
        .collect::<Vec<_>>();
    addresses.sort_by_key(|address| (address.is_ipv6(), address.to_string()));
    addresses.dedup();
    addresses
}

pub fn local_ipv4() -> Option<Ipv4Addr> {
    if let Some(address) = local_addresses()
        .into_iter()
        .find_map(|address| match address {
            IpAddr::V4(address) => Some(address),
            IpAddr::V6(_) => None,
        })
    {
        return Some(address);
    }
    let socket = UdpSocket::bind(("0.0.0.0", 0)).ok()?;
    socket.connect((MULTICAST_ADDR, DISCOVERY_PORT)).ok()?;
    match socket.local_addr().ok()?.ip() {
        std::net::IpAddr::V4(ip) if !ip.is_loopback() && !ip.is_unspecified() => Some(ip),
        _ => None,
    }
}

fn run(api_port: u16, name: &str) {
    let Ok(socket) = UdpSocket::bind(("0.0.0.0", DISCOVERY_PORT)) else {
        log::warn!("lan_discovery: could not bind UDP {DISCOVERY_PORT}");
        return;
    };
    let _ = socket.set_broadcast(true);
    let _ = socket.join_multicast_v4(&MULTICAST_ADDR, &Ipv4Addr::UNSPECIFIED);
    let _ = socket.set_read_timeout(Some(Duration::from_secs(1)));

    let mut last_beacon = Instant::now() - BEACON_INTERVAL;
    let mut own_addresses = Vec::new();
    let mut buffer = [0u8; 2048];
    loop {
        if last_beacon.elapsed() >= BEACON_INTERVAL {
            let addresses = local_addresses()
                .into_iter()
                .map(|ip| match ip {
                    IpAddr::V4(ip) => format!("http://{ip}:{api_port}"),
                    IpAddr::V6(ip) => format!("http://[{ip}]:{api_port}"),
                })
                .collect::<Vec<_>>();
            if let Some(address) = addresses.first() {
                own_addresses = addresses.clone();
                let beacon = serde_json::json!({
                    "type": "rayload-lan",
                    "version": 1,
                    "name": name,
                    "address": address,
                    "addresses": addresses,
                    "arch": std::env::consts::ARCH,
                });
                if let Ok(bytes) = serde_json::to_vec(&beacon) {
                    let target = SocketAddr::from((MULTICAST_ADDR, DISCOVERY_PORT));
                    let _ = socket.send_to(&bytes, target);
                    let broadcast = SocketAddr::from((Ipv4Addr::BROADCAST, DISCOVERY_PORT));
                    let _ = socket.send_to(&bytes, broadcast);
                }
            }
            last_beacon = Instant::now();
        }

        let Ok((length, source)) = socket.recv_from(&mut buffer) else {
            continue;
        };
        let Ok(beacon) = serde_json::from_slice::<serde_json::Value>(&buffer[..length]) else {
            continue;
        };
        if beacon["type"] != "rayload-lan" || beacon["version"] != 1 {
            continue;
        }
        let (Some(name), Some(address)) = (beacon["name"].as_str(), beacon["address"].as_str())
        else {
            continue;
        };
        if own_addresses.iter().any(|own| own.as_str() == address)
            || (source.ip().is_loopback() && address.contains("127.0.0.1"))
        {
            continue;
        }
        let addresses = beacon["addresses"]
            .as_array()
            .map(|values| {
                values
                    .iter()
                    .filter_map(|value| value.as_str())
                    .collect::<Vec<_>>()
            })
            .filter(|values| !values.is_empty())
            .unwrap_or_else(|| vec![address]);
        let addresses = addresses
            .into_iter()
            .map(|value| value.trim_end_matches('/').to_string())
            .filter(|value| !own_addresses.iter().any(|own| own == value))
            .collect::<Vec<_>>();
        // One card per device: a host advertises every interface it has, but
        // the list shows a single entry and keeps the extras as fallbacks.
        let Some(primary) = addresses.first().cloned() else {
            continue;
        };
        let peer = DiscoveredPeer {
            name: name.chars().take(64).collect(),
            address: primary,
            arch: beacon["arch"].as_str().unwrap_or_default().to_string(),
            fingerprint: beacon["fingerprint"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            addresses: addresses.clone(),
        };
        let mut peers = peers()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        // Re-beaconing with a changed address set must still merge.
        peers.retain(|entry| {
            !entry
                .peer
                .addresses
                .iter()
                .any(|known| addresses.contains(known))
        });
        peers.push(Entry {
            peer,
            seen: Instant::now(),
        });
    }
}
