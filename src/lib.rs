use anyhow::Result;

mod file_io;
mod sysctl;

use file_io::*;

use sysctl::SysctlConfSchema;
pub use sysctl::{SysctlConf, Value};

pub fn parse_sysctl_from_path(
    sysctl_conf_file_path: &str,
    schema_file_path: &str,
) -> Result<SysctlConf> {
    let sysctl_conf_line_list = read_file_to_vec_string(sysctl_conf_file_path)?;
    let schema_line_list = read_file_to_vec_string(schema_file_path)?;

    let sysctl_conf_schema = SysctlConfSchema::new(schema_line_list)?;
    let sysctl_conf = SysctlConf::new(sysctl_conf_line_list, sysctl_conf_schema)?;

    Ok(sysctl_conf)
}
