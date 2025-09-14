use crate::app::system_info::SystemInfo;
use crate::components::Component;
use crate::utils::formatting::{format_bytes, format_seconds};
use egui::{Grid, ScrollArea, Ui};

pub struct SystemInfoComponent<'a> {
    info: &'a SystemInfo,
}

impl<'a> SystemInfoComponent<'a> {
    pub fn new(info: &'a SystemInfo) -> Self {
        Self { info }
    }
}

impl Component for SystemInfoComponent<'_> {
    fn ui(self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            Grid::new("system_info_grid")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Name");
                    ui.label(self.info.name.as_ref().unwrap_or(&"Unknown".to_string()));
                    ui.end_row();

                    ui.label("Kernel version");
                    ui.label(&self.info.kernel_version);
                    ui.end_row();

                    ui.label("OS version");
                    ui.label(
                        self.info
                            .os_version
                            .as_ref()
                            .unwrap_or(&"Unknown".to_string()),
                    );
                    ui.end_row();

                    ui.label("CPU architecture");
                    ui.label(&self.info.cpu_architecture);
                    ui.end_row();

                    ui.label("CPU core count");
                    ui.label(
                        self.info
                            .cpu_core_count
                            .map(|count| count.to_string())
                            .unwrap_or_else(|| "Unknown".to_string()),
                    );
                    ui.end_row();

                    ui.label("CPU brand");
                    ui.label(
                        self.info
                            .cpu_brand
                            .as_ref()
                            .unwrap_or(&"Unknown".to_string()),
                    );
                    ui.end_row();

                    ui.label("CPU vendor id");
                    ui.label(
                        self.info
                            .cpu_vendor_id
                            .as_ref()
                            .unwrap_or(&"Unknown".to_string()),
                    );
                    ui.end_row();

                    ui.label("CPU frequency (Hz)");
                    ui.label(
                        self.info
                            .cpu_frequency
                            .map(|freq| freq.to_string())
                            .unwrap_or_else(|| "Unknown".to_string()),
                    );
                    ui.end_row();

                    ui.label("RAM total");
                    ui.label(format_bytes(self.info.ram_total_bytes));
                    ui.end_row();

                    ui.label("RAM free");
                    ui.label(format_bytes(self.info.ram_free_bytes));
                    ui.end_row();

                    ui.label("RAM used");
                    ui.label(format_bytes(self.info.ram_used_bytes));
                    ui.end_row();

                    ui.label("Swap total");
                    ui.label(format_bytes(self.info.swap_total_bytes));
                    ui.end_row();

                    ui.label("Swap free");
                    ui.label(format_bytes(self.info.swap_free_bytes));
                    ui.end_row();

                    ui.label("Swap used");
                    ui.label(format_bytes(self.info.swap_used_bytes));
                    ui.end_row();

                    ui.label("Uptime");
                    ui.label(format_seconds(self.info.uptime_seconds));
                    ui.end_row();
                })
        });
    }
}
