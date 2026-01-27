use std::fmt;
use std::str::FromStr;

use crate::error::ValidationError;

/// ByteSize represents a size in bytes
/// Internally stored as u64
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteSize(pub u64);

impl FromStr for ByteSize {
    type Err = ValidationError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();

        if input.is_empty() {
            return Err(ValidationError::InvalidByteSize {
                input: input.to_string(),
                reason: "input cannot be empty".to_string(),
            });
        }

        // Split numeric part and suffix
        let (number_part, suffix_part) = input.split_at(
            input
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(input.len()),
        );

        if number_part.is_empty() {
            return Err(ValidationError::InvalidByteSize {
                input: input.to_string(),
                reason: "missing numeric value".to_string(),
            });
        }

        let value: u64 = number_part.parse().map_err(|_| {
            ValidationError::InvalidByteSize {
                input: input.to_string(),
                reason: "invalid number".to_string(),
            }
        })?;

        if value == 0 {
            return Err(ValidationError::InvalidByteSize {
                input: input.to_string(),
                reason: "size must be greater than zero".to_string(),
            });
        }

        let multiplier = match suffix_part.to_ascii_uppercase().as_str() {
            "" | "B" => 1,
            "K" => 1_024,
            "M" => 1_048_576,
            "G" => 1_073_741_824,
            "T" => 1_099_511_627_776,
            _ => {
                return Err(ValidationError::InvalidByteSize {
                    input: input.to_string(),
                    reason: "invalid size suffix".to_string(),
                })
            }
        };

        value
            .checked_mul(multiplier)
            .map(ByteSize)
            .ok_or_else(|| ValidationError::InvalidByteSize {
                input: input.to_string(),
                reason: "size overflow".to_string(),
            })
    }
}

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.0;

        const KB: u64 = 1_024;
        const MB: u64 = 1_048_576;
        const GB: u64 = 1_073_741_824;
        const TB: u64 = 1_099_511_627_776;

        if bytes % TB == 0 {
            write!(f, "{}T", bytes / TB)
        } else if bytes % GB == 0 {
            write!(f, "{}G", bytes / GB)
        } else if bytes % MB == 0 {
            write!(f, "{}M", bytes / MB)
        } else if bytes % KB == 0 {
            write!(f, "{}K", bytes / KB)
        } else {
            write!(f, "{}B", bytes)
        }
    }
}
