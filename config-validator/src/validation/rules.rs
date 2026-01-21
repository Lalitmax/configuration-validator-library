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