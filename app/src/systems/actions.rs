use crate::app::bug_report::BugReportMetadata;
use std::cell::RefCell;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum AppAction {
    SaveAppSnapshot(PathBuf),
    RestoreAppSnapshot(PathBuf),
    DumpAppJSON(PathBuf),
    CreateBugReport {
        path: PathBuf,
        metadata: BugReportMetadata,
    },
    ReviewBugReport(PathBuf),
    RestoreBugReport,
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

    pub fn save_app_snapshot(&self, path: &Path) {
        self.push_action(AppAction::SaveAppSnapshot(path.to_path_buf()));
    }

    pub fn restore_app_snapshot(&self, path: &Path) {
        self.push_action(AppAction::RestoreAppSnapshot(path.to_path_buf()));
    }

    pub fn dump_app_json(&self, path: &Path) {
        self.push_action(AppAction::DumpAppJSON(path.to_path_buf()));
    }

    pub fn create_bug_report(&self, path: &Path, metadata: BugReportMetadata) {
        self.push_action(AppAction::CreateBugReport {
            path: path.to_path_buf(),
            metadata,
        });
    }

    pub fn review_bug_report(&self, path: &Path) {
        self.push_action(AppAction::ReviewBugReport(path.to_path_buf()));
    }

    pub fn restore_bug_report(&self) {
        self.push_action(AppAction::RestoreBugReport);
    }
}
