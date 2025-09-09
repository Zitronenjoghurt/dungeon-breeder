use std::cell::RefCell;
use std::path::PathBuf;

#[derive(Debug)]
pub enum AppAction {
    SaveAppSnapshot(PathBuf),
    RestoreAppSnapshot(PathBuf),
    DumpAppJSON(PathBuf),
}

#[derive(Debug, Default)]
pub struct AppActions {
    queue: RefCell<Vec<AppAction>>,
}

impl AppActions {
    pub fn take_actions(&self) -> Vec<AppAction> {
        if let Ok(mut queue) = self.queue.try_borrow_mut() {
            queue.drain(..).collect()
        } else {
            vec![]
        }
    }

    pub fn push_action(&self, action: AppAction) {
        if let Ok(mut queue) = self.queue.try_borrow_mut() {
            queue.push(action);
        }
    }

    pub fn save_app_snapshot(&self, path: PathBuf) {
        self.push_action(AppAction::SaveAppSnapshot(path));
    }

    pub fn restore_app_snapshot(&self, path: PathBuf) {
        self.push_action(AppAction::RestoreAppSnapshot(path));
    }

    pub fn dump_app_json(&self, path: PathBuf) {
        self.push_action(AppAction::DumpAppJSON(path));
    }
}
