use config_validator::config::{Host, HostRole};
use config_validator::Validate;

#[test]
fn valid_host_should_pass() {
    let host = Host {
        hostname: "manager-01".to_string(),
        ip_address: "192.168.1.10".to_string(),
        role: HostRole::Manager,
        enabled: true,
    };

    assert!(host.validate().is_ok());
}

#[test]
fn invalid_hostname_should_fail() {
    let host = Host {
        hostname: "-bad-host".to_string(),
        ip_address: "192.168.1.10 ".to_string(),
        role: HostRole::Manager,
        enabled: true,
    };

    assert!(host.validate().is_err());
    
}
