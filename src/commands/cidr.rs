use crate::ip_utils::parse_cidr;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Calculate network address from IP and prefix length
fn get_network_address(ip: IpAddr, prefix_len: u8) -> Result<IpAddr, Box<dyn std::error::Error>> {
    match ip {
        IpAddr::V4(ipv4) => {
            let ip_u32 = u32::from(ipv4);
            let mask = if prefix_len == 0 {
                0
            } else {
                0xffffffff << (32 - prefix_len)
            };
            let network_u32 = ip_u32 & mask;
            Ok(IpAddr::V4(Ipv4Addr::from(network_u32)))
        }
        IpAddr::V6(ipv6) => {
            let ip_u128 = u128::from(ipv6);
            let mask = if prefix_len == 0 {
                0
            } else {
                0xffffffffffffffffffffffffffffffff << (128 - prefix_len)
            };
            let network_u128 = ip_u128 & mask;
            Ok(IpAddr::V6(Ipv6Addr::from(network_u128)))
        }
    }
}

/// Check if two CIDR networks overlap
fn networks_overlap(
    ip1: IpAddr,
    prefix1: u8,
    ip2: IpAddr,
    prefix2: u8,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Different IP versions cannot overlap
    match (ip1, ip2) {
        (IpAddr::V4(_), IpAddr::V6(_)) | (IpAddr::V6(_), IpAddr::V4(_)) => return Ok(false),
        _ => {}
    }

    let network1 = get_network_address(ip1, prefix1)?;
    let network2 = get_network_address(ip2, prefix2)?;

    match (network1, network2) {
        (IpAddr::V4(net1), IpAddr::V4(net2)) => {
            let net1_u32 = u32::from(net1);
            let net2_u32 = u32::from(net2);

            // Calculate the smaller prefix (larger network)
            let min_prefix = prefix1.min(prefix2);
            let mask = if min_prefix == 0 {
                0
            } else {
                0xffffffff << (32 - min_prefix)
            };

            // Networks overlap if they have the same network address when masked with the smaller prefix
            Ok((net1_u32 & mask) == (net2_u32 & mask))
        }
        (IpAddr::V6(net1), IpAddr::V6(net2)) => {
            let net1_u128 = u128::from(net1);
            let net2_u128 = u128::from(net2);

            // Calculate the smaller prefix (larger network)
            let min_prefix = prefix1.min(prefix2);
            let mask = if min_prefix == 0 {
                0
            } else {
                0xffffffffffffffffffffffffffffffff << (128 - min_prefix)
            };

            // Networks overlap if they have the same network address when masked with the smaller prefix
            Ok((net1_u128 & mask) == (net2_u128 & mask))
        }
        _ => unreachable!(), // This case is handled above
    }
}

pub fn check_cidr_overlap(
    network1: &str,
    network2: &str,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Checking CIDR overlap between {} and {}...",
        network1, network2
    );

    let (ip1, prefix1) = parse_cidr(network1)?;
    let (ip2, prefix2) = parse_cidr(network2)?;

    if verbose {
        println!("Verbose mode enabled for CIDR check");
        println!(
            "Network 1: {} -> IP: {}, Prefix: {}",
            network1, ip1, prefix1
        );
        println!(
            "Network 2: {} -> IP: {}, Prefix: {}",
            network2, ip2, prefix2
        );
    }

    // Calculate network addresses
    let network_addr1 = get_network_address(ip1, prefix1)?;
    let network_addr2 = get_network_address(ip2, prefix2)?;

    if verbose {
        println!("Network address 1: {}/{}", network_addr1, prefix1);
        println!("Network address 2: {}/{}", network_addr2, prefix2);
    }

    // Check for overlap
    let overlaps = networks_overlap(ip1, prefix1, ip2, prefix2)?;

    if overlaps {
        println!("✓ Networks {} and {} OVERLAP", network1, network2);
        if verbose {
            let smaller_prefix = prefix1.min(prefix2);
            let larger_network = if prefix1 < prefix2 {
                network1
            } else {
                network2
            };
            println!(
                "  The network {} contains or overlaps with the other",
                larger_network
            );
            println!(
                "  Effective overlap determined by /{} prefix",
                smaller_prefix
            );
        }
    } else {
        println!("✓ Networks {} and {} do NOT overlap", network1, network2);
        if verbose {
            println!("  These networks are in separate address spaces");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;

    #[test]
    fn test_get_network_address_ipv4() {
        let ip = "192.168.1.100".parse::<IpAddr>().unwrap();
        let network = get_network_address(ip, 24).unwrap();
        assert_eq!(network.to_string(), "192.168.1.0");

        let ip = "10.0.5.200".parse::<IpAddr>().unwrap();
        let network = get_network_address(ip, 16).unwrap();
        assert_eq!(network.to_string(), "10.0.0.0");

        let ip = "172.16.32.1".parse::<IpAddr>().unwrap();
        let network = get_network_address(ip, 12).unwrap();
        assert_eq!(network.to_string(), "172.16.0.0");
    }

    #[test]
    fn test_get_network_address_ipv6() {
        let ip = "2001:db8:1234:5678::1".parse::<IpAddr>().unwrap();
        let network = get_network_address(ip, 64).unwrap();
        assert_eq!(network.to_string(), "2001:db8:1234:5678::");

        let ip = "2001:db8:abcd:ef01::1".parse::<IpAddr>().unwrap();
        let network = get_network_address(ip, 32).unwrap();
        assert_eq!(network.to_string(), "2001:db8::");
    }

    #[test]
    fn test_networks_overlap_ipv4_overlapping() {
        let ip1 = "192.168.1.0".parse::<IpAddr>().unwrap();
        let ip2 = "192.168.0.0".parse::<IpAddr>().unwrap();

        // 192.168.1.0/24 is contained in 192.168.0.0/16
        let result = networks_overlap(ip1, 24, ip2, 16).unwrap();
        assert!(result);

        // Test the reverse order
        let result = networks_overlap(ip2, 16, ip1, 24).unwrap();
        assert!(result);
    }

    #[test]
    fn test_networks_overlap_ipv4_same_network() {
        let ip1 = "192.168.1.0".parse::<IpAddr>().unwrap();
        let ip2 = "192.168.1.0".parse::<IpAddr>().unwrap();

        // Same network should overlap
        let result = networks_overlap(ip1, 24, ip2, 24).unwrap();
        assert!(result);
    }

    #[test]
    fn test_networks_overlap_ipv4_partial_overlap() {
        let ip1 = "192.168.1.0".parse::<IpAddr>().unwrap();
        let ip2 = "192.168.1.128".parse::<IpAddr>().unwrap();

        // 192.168.1.0/25 and 192.168.1.128/25 should not overlap
        let result = networks_overlap(ip1, 25, ip2, 25).unwrap();
        assert!(!result);

        // But 192.168.1.0/24 and 192.168.1.128/25 should overlap
        let result = networks_overlap(ip1, 24, ip2, 25).unwrap();
        assert!(result);
    }

    #[test]
    fn test_networks_overlap_ipv4_no_overlap() {
        let ip1 = "192.168.1.0".parse::<IpAddr>().unwrap();
        let ip2 = "10.0.0.0".parse::<IpAddr>().unwrap();

        // Completely different networks
        let result = networks_overlap(ip1, 24, ip2, 8).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_networks_overlap_ipv6_overlapping() {
        let ip1 = "2001:db8::".parse::<IpAddr>().unwrap();
        let ip2 = "2001:db8:1::".parse::<IpAddr>().unwrap();

        // 2001:db8:1::/48 is contained in 2001:db8::/32
        let result = networks_overlap(ip1, 32, ip2, 48).unwrap();
        assert!(result);
    }

    #[test]
    fn test_networks_overlap_ipv6_no_overlap() {
        let ip1 = "2001:db8::".parse::<IpAddr>().unwrap();
        let ip2 = "2002:db8::".parse::<IpAddr>().unwrap();

        // Different IPv6 networks
        let result = networks_overlap(ip1, 32, ip2, 32).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_networks_overlap_different_ip_versions() {
        let ipv4 = "192.168.1.0".parse::<IpAddr>().unwrap();
        let ipv6 = "2001:db8::".parse::<IpAddr>().unwrap();

        // IPv4 and IPv6 should never overlap
        let result = networks_overlap(ipv4, 24, ipv6, 32).unwrap();
        assert!(!result);

        let result = networks_overlap(ipv6, 32, ipv4, 24).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_check_cidr_overlap_integration() {
        // Test with overlapping networks
        let result = check_cidr_overlap("192.168.1.0/24", "192.168.0.0/16", false);
        assert!(result.is_ok());

        // Test with non-overlapping networks
        let result = check_cidr_overlap("192.168.1.0/24", "10.0.0.0/8", false);
        assert!(result.is_ok());

        // Test with invalid CIDR notation
        let result = check_cidr_overlap("invalid", "192.168.0.0/16", false);
        assert!(result.is_err());

        // Test with invalid prefix length
        let result = check_cidr_overlap("192.168.1.0/33", "192.168.0.0/16", false);
        assert!(result.is_err());
    }

    #[test]
    fn test_edge_cases() {
        // Test with /0 prefix (entire IP space)
        let ip1 = "0.0.0.0".parse::<IpAddr>().unwrap();
        let ip2 = "192.168.1.0".parse::<IpAddr>().unwrap();
        let result = networks_overlap(ip1, 0, ip2, 24).unwrap();
        assert!(result); // /0 contains everything

        // Test with maximum prefix lengths
        let ip1 = "192.168.1.1".parse::<IpAddr>().unwrap();
        let ip2 = "192.168.1.2".parse::<IpAddr>().unwrap();
        let result = networks_overlap(ip1, 32, ip2, 32).unwrap();
        assert!(!result); // /32 are individual hosts
    }
}
