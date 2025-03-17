use anyhow::Result;

mod file_reader;
mod sysctl_conf;

use file_reader::read_file_to_vec_string;
pub use sysctl_conf::{SysctlConf, Value};

pub fn parse_sysctl_from_path(path: &str) -> Result<SysctlConf> {
    let lines = read_file_to_vec_string(path)?;

    Ok(SysctlConf::new(lines))
}
