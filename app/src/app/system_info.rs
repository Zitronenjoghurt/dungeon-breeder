use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub name: Option<String>,
    pub kernel_version: String,
    pub os_version: Option<String>,
    pub cpu_architecture: String,
    pub cpu_core_count: Option<usize>,
    pub cpu_brand: Option<String>,
    pub cpu_vendor_id: Option<String>,
    pub cpu_frequency: Option<u64>,
    pub ram_total_bytes: u64,
    pub ram_free_bytes: u64,
    pub ram_used_bytes: u64,
    pub swap_total_bytes: u64,
    pub swap_free_bytes: u64,
    pub swap_used_bytes: u64,
    pub uptime_seconds: u64,
}

impl SystemInfo {
    pub fn collect() -> Self {
        let mut sys = System::new();
        sys.refresh_cpu_all();
        sys.refresh_memory();

        let (cpu_brand, cpu_vendor_id, cpu_frequency) = sys
            .cpus()
            .first()
            .map(|cpu| {
                (
                    Some(cpu.brand().to_string()),
                    Some(cpu.vendor_id().to_string()),
                    Some(cpu.frequency()),
                )
            })
            .unwrap_or_else(|| (None, None, None));

        Self {
            name: System::name(),
            kernel_version: System::kernel_long_version(),
            os_version: System::long_os_version(),
            cpu_architecture: System::cpu_arch(),
            cpu_core_count: System::physical_core_count(),
            cpu_brand,
            cpu_vendor_id,
            cpu_frequency,
            ram_free_bytes: sys.free_memory(),
            ram_total_bytes: sys.total_memory(),
            ram_used_bytes: sys.used_memory(),
            swap_free_bytes: sys.free_swap(),
            swap_total_bytes: sys.total_swap(),
            swap_used_bytes: sys.used_swap(),
            uptime_seconds: System::uptime(),
        }
    }
}
