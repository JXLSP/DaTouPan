use serde::{ Deserialize, Serialize };
use sysinfo::{ Components, Disks, Networks, System };

#[derive(Serialize, Deserialize)]
pub struct SysInfoCollector {
    total_memory: u32,
    used_memory: u32,
    total_swap: u32,
    used_swap: u32,
    name: String,
    host: String,
    disk_usage: u32,
}

impl SysInfoCollector {
    pub fn new() -> Self {
        SysInfoCollector{}
    }
}
