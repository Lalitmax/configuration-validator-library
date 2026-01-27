use config_validator::ByteSize;
use std::str::FromStr;

#[test]
fn parse_valid_byte_sizes() {
    assert_eq!(ByteSize::from_str("2048B").unwrap().0, 2048);
    assert_eq!(ByteSize::from_str("1024K").unwrap().0, 1024 * 1024);
    assert_eq!(ByteSize::from_str("512M").unwrap().0, 512 * 1024 * 1024);
    assert_eq!(ByteSize::from_str("100G").unwrap().0, 100 * 1024 * 1024 * 1024);
    assert_eq!(ByteSize::from_str("1T").unwrap().0, 1024_u64.pow(4));
}

#[test]
fn parse_lowercase_byte_sizes() {
    assert_eq!(ByteSize::from_str("100g").unwrap().0, 100 * 1024 * 1024 * 1024);
    assert_eq!(ByteSize::from_str("512m").unwrap().0, 512 * 1024 * 1024);
}

#[test]
fn reject_invalid_byte_sizes() {
    assert!(ByteSize::from_str("").is_err());
    assert!(ByteSize::from_str("abc").is_err());
    assert!(ByteSize::from_str("-100G").is_err());
    assert!(ByteSize::from_str("100X").is_err());
    assert!(ByteSize::from_str("G100").is_err());
    assert!(ByteSize::from_str("0G").is_err());
}

#[test]
fn display_formats_human_readable() {
    let size = ByteSize::from_str("1024K").unwrap();
    assert_eq!(size.to_string(), "1M");

    let size = ByteSize::from_str("2048B").unwrap();
    assert_eq!(size.to_string(), "2K");

    let size = ByteSize::from_str("2G").unwrap();
    assert_eq!(size.to_string(), "2G");
}
