use crate::app::GameApp;
use crate::modals::AppModal;
use egui::{Id, Ui};

pub struct ConfirmModalOptions {
    title: &'static str,
    description: &'static str,
    yes: Option<&'static str>,
    no: Option<&'static str>,
}

impl ConfirmModalOptions {
    pub fn new(title: &'static str, description: &'static str) -> Self {
        Self {
            title,
            description,
            yes: None,
            no: None,
        }
    }

    pub fn yes(mut self, yes: &'static str) -> Self {
        self.yes = Some(yes);
        self
    }

    pub fn no(mut self, no: &'static str) -> Self {
        self.no = Some(no);
        self
    }
}

#[derive(Default)]
pub struct ConfirmModal {
    options: Option<ConfirmModalOptions>,
    success_callback: Option<Box<dyn Fn(&mut GameApp)>>,
}

impl ConfirmModal {
    pub fn open(
        &mut self,
        options: ConfirmModalOptions,
        success_callback: impl Fn(&mut GameApp) + 'static,
    ) {
        self.options = Some(options);
        self.success_callback = Some(Box::new(success_callback));
    }

    fn success(&mut self, app: &mut GameApp) {
        if let Some(callback) = self.success_callback.take() {
            callback(app);
        }
    }
}

impl AppModal for ConfirmModal {
    fn id(&self) -> Id {
        Id::new("confirm_modal")
    }

    fn is_open(&self) -> bool {
        self.success_callback.is_some() && self.options.is_some()
    }

    fn close(&mut self) {
        self.success_callback = None;
        self.options = None;
    }

    fn update_content(&mut self, ui: &mut Ui, app: &mut GameApp) {
        let Some(options) = &self.options else {
            return;
        };

        ui.set_width(300.0);

        ui.vertical_centered_justified(|ui| {
            ui.heading(options.title);
            ui.separator();
            ui.label(options.description);
            ui.separator();
        });

        let yes_label = options.yes.unwrap_or("Yes");
        let no_label = options.no.unwrap_or("No");
        ui.columns(3, |columns| {
            columns[0].vertical_centered_justified(|ui| {
                if ui.button(yes_label).clicked() {
                    self.success(app);
                }
            });
            columns[2].vertical_centered_justified(|ui| {
                if ui.button(no_label).clicked() {
                    self.close();
                }
            });
        });
    }
}
