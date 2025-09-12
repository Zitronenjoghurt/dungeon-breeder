use crate::app::bug_report::BugReport;

#[derive(Debug, Default)]
pub struct BugReportReviewSystem {
    bug_report: Option<BugReport>,
}

impl BugReportReviewSystem {
    pub fn set_bug_report(&mut self, bug_report: BugReport) {
        self.bug_report = Some(bug_report);
    }

    pub fn reset(&mut self) {
        self.bug_report = None;
    }

    pub fn bug_report(&self) -> Option<&BugReport> {
        self.bug_report.as_ref()
    }

    pub fn take_bug_report(&mut self) -> Option<BugReport> {
        self.bug_report.take()
    }

    pub fn has_bug_report(&self) -> bool {
        self.bug_report.is_some()
    }
}
