use crate::app::GameApp;
use crate::components::column_config::{
    SortedSpecimenTableColumnConfig, SortedSpecimenTableColumnConfigEdit,
};
use crate::components::{Component, EnumSelect, SortedSpecimenTable};
use crate::modals::AppModal;
use dungeon_breeder_core::state::specimen::collection::SpecimenCollectionSort;
use dungeon_breeder_core::state::specimen::SpecimenId;
use egui::{Context, Id, Ui};
use serde::{Deserialize, Serialize};

#[allow(clippy::type_complexity)]
#[derive(Serialize, Deserialize)]
pub struct SpecimenSelectionModal {
    options: SpecimenSelectionModalOptions,
    column_config: SortedSpecimenTableColumnConfig,
    sort_config: SpecimenCollectionSort,
    #[serde(skip, default)]
    callback: Option<Box<dyn Fn(Option<SpecimenId>, &mut GameApp)>>,
    #[serde(skip, default)]
    sorted_ids: Vec<SpecimenId>,
    #[serde(skip, default = "default_true")]
    sort_dirty: bool,
}

impl Default for SpecimenSelectionModal {
    fn default() -> Self {
        Self {
            options: SpecimenSelectionModalOptions::default(),
            column_config: SortedSpecimenTableColumnConfig::default(),
            sort_config: SpecimenCollectionSort::default(),
            callback: None,
            sorted_ids: Vec::new(),
            sort_dirty: true,
        }
    }
}

impl SpecimenSelectionModal {
    pub fn open(
        &mut self,
        options: SpecimenSelectionModalOptions,
        callback: impl Fn(Option<SpecimenId>, &mut GameApp) + 'static,
    ) {
        self.sort_dirty = true;
        self.options = options;
        self.callback = Some(Box::new(callback));
    }

    fn select(&mut self, specimen_id: Option<SpecimenId>, app: &mut GameApp) {
        if let Some(callback) = self.callback.take() {
            callback(specimen_id, app);
        }
    }

    fn sort(&mut self, app: &mut GameApp) {
        self.update_excluded_ids(app);
        self.sorted_ids = app.game.state.specimen.sorted_ids(&self.sort_config);
    }

    fn update_excluded_ids(&mut self, app: &mut GameApp) {
        self.sort_config.excluded_ids.clear();

        if self.options.exclude_specimen_assigned_to_dungeon_layer_slot {
            self.sort_config
                .excluded_ids
                .extend(app.game.state.dungeon.iter_layer_slot_assigned_specimen());
        }

        if self.options.exclude_specimen_on_breeding_cooldown {
            self.sort_config.excluded_ids.extend(
                app.game
                    .state
                    .specimen
                    .iter_on_breeding_cooldown()
                    .map(|specimen| specimen.id),
            );
        }
    }
}

impl AppModal for SpecimenSelectionModal {
    fn id(&self) -> Id {
        Id::new("specimen_selection_modal")
    }

    fn is_open(&self) -> bool {
        self.callback.is_some()
    }

    fn close(&mut self) {
        self.callback = None;
    }

    fn before_close(&mut self, _ctx: &Context, _app: &mut GameApp) {
        self.sorted_ids.clear();
        self.sort_dirty = true;
    }

    fn update_content(&mut self, ui: &mut Ui, app: &mut GameApp) {
        if self.sort_dirty {
            self.sort_dirty = false;
            self.sort(app);
        }

        ui.vertical_centered_justified(|ui| {
            ui.heading("Select specimen");
        });

        ui.separator();

        let old_sort_field = self.sort_config.sort_field;
        let old_sort_direction = self.sort_config.sort_direction;
        ui.horizontal(|ui| {
            SortedSpecimenTableColumnConfigEdit::new(&mut self.column_config).ui(ui);
            EnumSelect::new(&mut self.sort_config.sort_field, "select_sort_field").ui(ui);
            EnumSelect::new(
                &mut self.sort_config.sort_direction,
                "select_sort_direction",
            )
            .ui(ui);
        });

        if self.sort_config.sort_field != old_sort_field
            || self.sort_config.sort_direction != old_sort_direction
        {
            self.sort_dirty = true;
        }

        SortedSpecimenTable::new(
            &app.game.state.specimen,
            &self.sorted_ids,
            &mut self.options.selected_specimen_id,
        )
        .max_height(500.0)
        .column_config(self.column_config)
        .ui(ui);

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Select").clicked() {
                self.select(self.options.selected_specimen_id, app);
            }
            if ui.button("Clear").clicked() {
                self.select(None, app);
            }
        });
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct SpecimenSelectionModalOptions {
    selected_specimen_id: Option<SpecimenId>,
    exclude_specimen_assigned_to_dungeon_layer_slot: bool,
    exclude_specimen_on_breeding_cooldown: bool,
}

impl SpecimenSelectionModalOptions {
    pub fn new(selected_specimen_id: Option<SpecimenId>) -> Self {
        Self {
            selected_specimen_id,
            ..Self::default()
        }
    }

    pub fn exclude_assigned_to_dungeon_layer_slot(mut self, exclude: bool) -> Self {
        self.exclude_specimen_assigned_to_dungeon_layer_slot = exclude;
        self
    }

    pub fn exclude_on_breeding_cooldown(mut self, exclude: bool) -> Self {
        self.exclude_specimen_on_breeding_cooldown = exclude;
        self
    }
}

fn default_true() -> bool {
    true
}
