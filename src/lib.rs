use std::net::IpAddr;

pub mod ip_utils {
    use super::*;

    #[derive(Debug)]
    pub enum IpParseError {
        InvalidFormat(String),
        InvalidCidr(String),
    }

    impl std::fmt::Display for IpParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                IpParseError::InvalidFormat(ip) => write!(f, "Invalid IP address format: {}", ip),
                IpParseError::InvalidCidr(cidr) => write!(f, "Invalid CIDR notation: {}", cidr),
            }
        }
    }

    impl std::error::Error for IpParseError {}

    /// Parse and validate an IP address string
    pub fn parse_ip_address(ip_str: &str) -> Result<IpAddr, IpParseError> {
        ip_str
            .parse()
            .map_err(|_| IpParseError::InvalidFormat(ip_str.to_string()))
    }

    /// Parse and validate a CIDR notation string
    pub fn parse_cidr(cidr_str: &str) -> Result<(IpAddr, u8), IpParseError> {
        let parts: Vec<&str> = cidr_str.split('/').collect();
        if parts.len() != 2 {
            return Err(IpParseError::InvalidCidr(cidr_str.to_string()));
        }

        let ip = parse_ip_address(parts[0])?;
        let prefix = parts[1]
            .parse::<u8>()
            .map_err(|_| IpParseError::InvalidCidr(cidr_str.to_string()))?;

        // Validate prefix length based on IP version
        let max_prefix = match ip {
            IpAddr::V4(_) => 32,
            IpAddr::V6(_) => 128,
        };

        if prefix > max_prefix {
            return Err(IpParseError::InvalidCidr(format!(
                "Invalid prefix length {} for {} address",
                prefix,
                if matches!(ip, IpAddr::V4(_)) {
                    "IPv4"
                } else {
                    "IPv6"
                }
            )));
        }

        Ok((ip, prefix))
    }

    /// Get IP address type information
    pub fn get_ip_info(ip: &IpAddr) -> String {
        match ip {
            IpAddr::V4(ipv4) => {
                if ipv4.is_loopback() {
                    "IPv4 Loopback".to_string()
                } else if ipv4.is_private() {
                    "IPv4 Private".to_string()
                } else if ipv4.is_multicast() {
                    "IPv4 Multicast".to_string()
                } else if ipv4.is_broadcast() {
                    "IPv4 Broadcast".to_string()
                } else {
                    "IPv4 Public".to_string()
                }
            }
            IpAddr::V6(ipv6) => {
                if ipv6.is_loopback() {
                    "IPv6 Loopback".to_string()
                } else if ipv6.is_multicast() {
                    "IPv6 Multicast".to_string()
                } else {
                    "IPv6".to_string()
                }
            }
        }
    }

    /// Print detailed IP information in verbose mode
    pub fn print_ip_details(ip: &IpAddr, verbose: bool) {
        if verbose {
            println!("IP Address: {}", ip);
            println!("Type: {}", get_ip_info(ip));
            match ip {
                IpAddr::V4(ipv4) => {
                    println!("Octets: {:?}", ipv4.octets());
                }
                IpAddr::V6(ipv6) => {
                    println!("Segments: {:?}", ipv6.segments());
                }
            }
        }
    }
}

pub mod commands;
pub mod crawler_sources;
