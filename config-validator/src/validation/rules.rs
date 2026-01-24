use crate::error::ValidationError;

// Validate hostname
pub fn validate_hostname(hostname: &str) -> Result<(), ValidationError> {
    if hostname.is_empty() {
        return Err(ValidationError::InvalidHostname {
            hostname: hostname.to_string(),
            reason: "Hostname cannot be empty".to_string(),
        })
    }

    if hostname.starts_with(" ") || hostname.ends_with(" ") {
        return Err(ValidationError::InvalidHostname {
            hostname: hostname.to_string(),
            reason: "Hostname cannot start or end with a space".to_string(),
        })
    }

    if hostname.starts_with('-') || hostname.ends_with('-') {
        return Err(ValidationError::InvalidHostname {
            hostname: hostname.to_string(),
            reason: "cannot start or end with '-'".to_string(),
        });
    }

    if hostname.len() > 255 {
        return Err(ValidationError::InvalidHostname {
            hostname: hostname.to_string(),
            reason: "Hostname cannot be longer than 255 characters".to_string(),
        })
    }

    if !hostname.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '.') {
        return Err(ValidationError::InvalidHostname {
            hostname: hostname.to_string(),
            reason: "Hostname can only contain alphanumeric characters, '-' and '.'".to_string(),
        })
    }

    Ok(())

}

// Validate IPv4 address
pub fn validate_ipv4(ip: &str) -> Result<(), ValidationError> {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return Err(ValidationError::InvalidIpAddress {
            ip: ip.to_string(),
            reason: "must contain 4 octets".to_string(),
        });
    }

    for part in parts {
        let num: u8 = part.parse().map_err(|_| ValidationError::InvalidIpAddress {
            ip: ip.to_string(),
            reason: "each octet must be 0-255".to_string(),
        })?;
        let _ = num;
    }

    Ok(())
}

// Validate network interface name
pub fn validate_interface_name(name: &str) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::InvalidNetworkInterface {
            name: name.to_string(),
            reason: "interface name cannot be empty".to_string(),
        });
    }

    let mut chars = name.chars();

    // Must start with letters
    let mut seen_digit = false;
    for c in chars.by_ref() {
        if c.is_ascii_digit() {
            seen_digit = true;
            break;
        }
        if !c.is_ascii_alphabetic() {
            return Err(ValidationError::InvalidNetworkInterface {
                name: name.to_string(),
                reason: "must start with letters".to_string(),
            });
        }
    }

    // After digits start, all must be digits
    for c in chars {
        if !c.is_ascii_digit() {
            return Err(ValidationError::InvalidNetworkInterface {
                name: name.to_string(),
                reason: "letters must come before digits".to_string(),
            });
        }
    }

    if !seen_digit {
        return Err(ValidationError::InvalidNetworkInterface {
            name: name.to_string(),
            reason: "must end with digits".to_string(),
        });
    }

    Ok(())
}
