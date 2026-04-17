use std::net::Ipv4Addr;
use std::str::FromStr;

use std::process::Command;

use anyhow::{Result, anyhow};

use std::fs;

/// Chooses a random IP from a specific CIDR range.
///
/// # Arguments
/// * `range` - The range in "IP/CIDR" format (e.g., "192.168.1.0/24")
/// * `seed`  - Seed value used for random number generation
///
/// # Returns
/// A random IP address as a string within the specified CIDR range.
pub fn get_rand_ip(range: &str, seed: u32) -> Result<String> {
    // Split net IP/CIDR from the range string.
    let mut parts = range.splitn(2, '/');

    let (s_ip, cidr_str) = match (parts.next(), parts.next()) {
        (Some(ip), Some(cidr)) => (ip, cidr),
        _ => return Err(anyhow::anyhow!("Invalid CIDR format")),
    };

    // Parse the CIDR prefix length.
    let cidr: u8 = match cidr_str.parse() {
        Ok(v) if v <= 32 => v,
        _ => return Err(anyhow::anyhow!("Invalid CIDR value")),
    };

    // Parse the base IP address into a u32.
    let ip_addr = match Ipv4Addr::from_str(s_ip) {
        Ok(ip) => u32::from(ip),
        Err(_) => return Err(anyhow::anyhow!("Invalid IP address")),
    };

    // Build the host mask: the complement of the network mask.
    // e.g. /24  =>  mask = 0x000000FF
    let mask: u32 = if cidr == 0 {
        u32::MAX // 0 means the entire address space is the host part
    } else {
        (1u32 << (32 - cidr)).wrapping_sub(1)
    };

    // Cheap LCG that mimics the behaviour of a seeded rand_r call.
    let rand_num = lcg_rand(seed);

    // Combine the network prefix with the random host part.
    let rand_ip_u32 = (ip_addr & !mask) | (mask & rand_num);

    Ok(Ipv4Addr::from(rand_ip_u32).to_string())
}

/// Internal function that acts as a minimal LCG random number generator that mirrors the single-step
/// output of a typical `rand_r` implementation (glibc / POSIX).
///
/// glibc rand_r does:  next = next * 1103515245 + 12345
/// and returns        (next >> 16) & 0x7FFF  (a 15-bit value)
///
/// We return the full 32-bit scrambled seed so the host bits are filled
/// across the whole mask (important for prefix lengths smaller than 17).
fn lcg_rand(seed: u32) -> u32 {
    seed.wrapping_mul(1_103_515_245).wrapping_add(12_345)
}

/// Retrieves the source MAC address of a network interface.
///
/// Reads from `/sys/class/net/<dev>/address`, which on Linux exposes
/// the interface's MAC as a human-readable `"aa:bb:cc:dd:ee:ff\n"` string.
///
/// # Arguments
/// * `dev` - The interface/device name (e.g. `"eth0"`, `"ens3"`)
///
/// # Returns
/// `Ok([u8; 6])` containing the 6 MAC address bytes on success,
/// or an `Err` describing what went wrong.
pub fn get_src_mac_addr(dev: &str) -> Result<[u8; 6]> {
    let path = format!("/sys/class/net/{}/address", dev);

    // Read and trim the file in one shot.
    let contents =
        fs::read_to_string(&path).map_err(|e| anyhow!("failed to open '{}': {}", path, e))?;

    let mac_str = contents.trim();

    // Parse "aa:bb:cc:dd:ee:ff" into 6 bytes.
    let bytes: Vec<u8> = mac_str
        .split(':')
        .map(|octet| u8::from_str_radix(octet, 16))
        .collect::<Result<_, _>>()
        .map_err(|e| anyhow!("failed to parse MAC address '{}': {}", mac_str, e))?;

    // Ensure we got exactly 6 bytes.
    let mac = <[u8; 6]>::try_from(bytes.as_slice())
        .map_err(|_| anyhow!("expected 6 MAC octets, received {}", bytes.len()))?;

    Ok(mac)
}

/// Retrieves the Ethernet MAC address of the host's default gateway.
///
/// # Notes
/// Shells out to `ip` to find the default gateway, then looks it up
/// in the table to get its MAC address.
/// e.g., `ip -4 route list 0/0 | cut -d' ' -f3`
///
/// # Returns
/// `Ok([u8; 6])` containing the 6 MAC address bytes on success,
/// or an `Err` describing what went wrong.
pub fn get_gw_mac() -> Result<[u8; 6]> {
    // Get the default gateway IP: `ip -4 route list 0/0 | cut -d' ' -f3`
    let gw_ip = Command::new("ip")
        .args(["-4", "route", "list", "0/0"])
        .output()
        .map_err(|e| anyhow!("failed to run 'ip route': {}", e))?;

    let gw_ip = String::from_utf8_lossy(&gw_ip.stdout);
    let gw_ip = gw_ip
        .split_whitespace()
        .nth(2)
        .ok_or_else(|| anyhow!("failed to parse default gateway from 'ip route' output"))?;

    // Look up the gateway in the neighbour table.
    // NOTE: Try `ip neigh show <gw_ip>`
    let neigh = Command::new("ip")
        .args(["neigh", "show", gw_ip])
        .output()
        .map_err(|e| anyhow!("failed to run 'ip neigh {}': {}", gw_ip, e))?;

    let neigh = String::from_utf8_lossy(&neigh.stdout);
    let mac_str = neigh
        .split_whitespace()
        .find(|s| s.chars().filter(|c| *c == ':').count() == 5)
        .ok_or_else(|| {
            anyhow!(
                "failed to find MAC address in 'ip neigh' output for '{}'",
                gw_ip
            )
        })?;

    // Parse MAC address into bytes.
    let bytes: Vec<u8> = mac_str
        .split(':')
        .map(|octet| u8::from_str_radix(octet, 16))
        .collect::<Result<_, _>>()
        .map_err(|e| anyhow!("failed to parse MAC address '{}': {}", mac_str, e))?;

    <[u8; 6]>::try_from(bytes.as_slice())
        .map_err(|_| anyhow!("expected 6 MAC octets, received {}", bytes.len()))
}

/// Attempts to determine the network interface name associated with a given source IP address.
///
/// # Arguments
/// * `src_ip` - The source IP address to look up (e.g., "192.168.1.1")
///
/// # Returns
/// `Ok(String)` containing the interface name (e.g., "eth0") on success,
/// or an `Err` describing what went wrong.
pub fn get_ifname_from_src_ip(src_ip: &str) -> Result<String> {
    let output = Command::new("ip")
        .args(["-o", "-4", "addr", "show", "to", src_ip])
        .output()
        .map_err(|e| anyhow!("failed to run 'ip addr show': {}", e))?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let ifname = output_str
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| anyhow!("failed to parse interface name from 'ip addr' output"))?;

    Ok(ifname.to_string())
}
