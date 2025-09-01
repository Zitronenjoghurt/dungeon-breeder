use crate::state::AppState;
use crate::views::{View, ViewManager};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct DBApp {
    state: AppState,
    view_manager: ViewManager,
}

impl DBApp {
    pub fn new(cc: &eframe::CreationContext) -> anyhow::Result<Self> {
        match cc.storage {
            Some(storage) => match eframe::get_value::<Self>(storage, eframe::APP_KEY) {
                Some(app) => Ok(app),
                None => {
                    if storage.get_string(eframe::APP_KEY).is_some() {
                        Err(anyhow!(
                            "Failed to deserialize app state - corrupted or incompatible format"
                        ))
                    } else {
                        Ok(Self::default())
                    }
                }
            },
            None => Ok(Self::default()),
        }
    }
}

impl eframe::App for DBApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.view_manager.render(ctx, &mut self.state);
        self.state.update(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
