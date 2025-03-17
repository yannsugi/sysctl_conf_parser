use anyhow::Result;
use sysctl_conf_parser::parse_sysctl_from_path;

#[test]
fn test_parse_sysctl_from_path() -> Result<()> {
    let sysctl_conf_file_path = "tests/source/sample_sysctl.conf";
    let sysctl_conf_schema_file_path = "tests/source/sample_sysctl.schema";

    let sysctl_conf = parse_sysctl_from_path(sysctl_conf_file_path, sysctl_conf_schema_file_path)?;

    assert_eq!(
        sysctl_conf.get("endpoint"),
        Some(&"localhost:3000".to_string())
    );
    assert_eq!(sysctl_conf.get("debug"), Some(&"true".to_string()));
    assert_eq!(
        sysctl_conf.get("log.file"),
        Some(&"/var/log/console.log".to_string())
    );
    assert_eq!(
        sysctl_conf.get("log.name"),
        Some(&"default.log".to_string())
    );
    assert_eq!(sysctl_conf.get("vm.dirty_ratio"), Some(&"0.2".to_string()));
    Ok(())
}
