//! Explicit user-task demand; telemetry is deliberately not an input.
use crate::workspace::PanelId;
#[derive(Clone, Copy, Debug)]
pub enum Task {
    Preview,
    Transfer,
    Diagnostics,
}
impl Task {
    fn index(self) -> usize {
        self as usize
    }
    fn panel(self) -> PanelId {
        match self {
            Self::Preview => PanelId::Files,
            Self::Transfer => PanelId::Network,
            Self::Diagnostics => PanelId::Utilities,
        }
    }
}
#[derive(Default, Debug)]
pub struct Demand {
    order: Vec<Task>,
}
impl Demand {
    /// Idempotent start/end; the most recently started eligible task gets space.
    pub fn set(&mut self, task: Task, active: bool) {
        let present = self.order.iter().any(|t| t.index() == task.index());
        if present == active {
            return;
        }
        self.order.retain(|t| t.index() != task.index());
        if active {
            self.order.push(task);
        }
    }
    pub fn preferred(&self, mut visible: impl FnMut(PanelId) -> bool) -> Option<PanelId> {
        self.order
            .iter()
            .rev()
            .map(|t| t.panel())
            .find(|p| visible(*p))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recent_task_wins_and_ending_restores_previous() {
        let mut d = Demand::default();
        d.set(Task::Preview, true);
        d.set(Task::Transfer, true);
        assert_eq!(d.preferred(|_| true), Some(PanelId::Network));
        d.set(Task::Preview, true); // routine repeated update cannot reorder tasks
        assert_eq!(d.preferred(|_| true), Some(PanelId::Network));
        d.set(Task::Transfer, false);
        assert_eq!(d.preferred(|_| true), Some(PanelId::Files));
        d.set(Task::Preview, false);
        assert_eq!(d.preferred(|_| true), None);
    }
    #[test]
    fn hidden_tasks_never_reopen_and_completion_is_idempotent() {
        let mut d = Demand::default();
        d.set(Task::Preview, true);
        d.set(Task::Diagnostics, true);
        assert_eq!(
            d.preferred(|p| p != PanelId::Utilities),
            Some(PanelId::Files)
        );
        assert_eq!(d.preferred(|_| false), None);
        d.set(Task::Diagnostics, false);
        d.set(Task::Diagnostics, false);
        assert_eq!(d.preferred(|_| true), Some(PanelId::Files));
    }
}
