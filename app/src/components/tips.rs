use crate::app::GameApp;
use crate::components::Component;
use egui::{Grid, Label, ScrollArea, Ui};

pub struct TipsComponent<'a> {
    pub app: &'a mut GameApp,
}

impl<'a> TipsComponent<'a> {
    pub fn new(app: &'a mut GameApp) -> Self {
        Self { app }
    }

    fn show_tips_relevant(&mut self, ui: &mut Ui) {
        let mut selected_tip = self.app.tips.selected_tip;
        for tip in self.app.tips.tips_relevant() {
            let selected = Some(tip) == self.app.tips.selected_tip;
            if ui.selectable_label(selected, tip.get_title()).clicked() {
                if selected {
                    selected_tip = None;
                } else {
                    selected_tip = Some(tip);
                }
            }
        }
        self.app.tips.selected_tip = selected_tip;
    }

    fn show_tips_all(&mut self, ui: &mut Ui) {
        let mut selected_tip = self.app.tips.selected_tip;
        for tip in self.app.tips.tips_all() {
            let selected = Some(tip) == self.app.tips.selected_tip;
            if ui.selectable_label(selected, tip.get_title()).clicked() {
                if selected {
                    selected_tip = None;
                } else {
                    selected_tip = Some(tip);
                }
            }
        }
        self.app.tips.selected_tip = selected_tip;
    }
}

impl Component for TipsComponent<'_> {
    fn ui(mut self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui| {
            if ui
                .selectable_label(self.app.tips.show_all, "Show all")
                .clicked()
            {
                self.app.tips.show_all = !self.app.tips.show_all;
            }

            ScrollArea::vertical()
                .id_salt("tips_window_tips_list_scroll")
                .show(ui, |ui| {
                    Grid::new("tips_window_tips_list")
                        .striped(true)
                        .num_columns(1)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                if self.app.tips.show_all {
                                    self.show_tips_all(ui);
                                } else {
                                    self.show_tips_relevant(ui);
                                }
                            });
                            ui.end_row();
                        });
                });

            if let Some(tip) = self.app.tips.selected_tip {
                ui.separator();

                ui.add(Label::new(tip.get_text()).wrap());

                if !self.app.tips.has_read_tip(tip) && ui.button("Read").clicked() {
                    self.app.tips.read_tip(tip);
                    self.app.tips.selected_tip = None;
                }
            }
        });
    }
}
