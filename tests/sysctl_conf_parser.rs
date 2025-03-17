use anyhow::Result;
use sysctl_conf_parser::parse_sysctl_from_path;

#[test]
fn test_parse_sysctl_from_path() -> Result<()> {
    let path = "test_data/sysctl.conf";
    let sysctl_conf = parse_sysctl_from_path(path)?;

    assert_eq!(
        sysctl_conf.get("net.ipv4.ip_forward"),
        Some(&"1".to_string())
    );
    assert_eq!(sysctl_conf.get("vm.swappiness"), Some(&"10".to_string()));
    Ok(())
}
