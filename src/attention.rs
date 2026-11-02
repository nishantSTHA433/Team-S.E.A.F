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

/// Track distinct touch sequences: ending one finger cannot release another.
#[derive(Debug)]
pub struct Contacts<K> {
    touches: Vec<K>,
    pointer: bool,
}
impl<K> Default for Contacts<K> {
    fn default() -> Self {
        Self {
            touches: Vec::new(),
            pointer: false,
        }
    }
}
impl<K: PartialEq> Contacts<K> {
    pub fn begin(&mut self, key: K) {
        if !self.touches.contains(&key) {
            self.touches.push(key);
        }
    }
    pub fn end(&mut self, key: &K) {
        self.touches.retain(|k| k != key);
    }
    pub fn pointer(&mut self, down: bool) {
        self.pointer = down;
    }
    pub fn active(&self) -> bool {
        self.pointer || !self.touches.is_empty()
    }
    pub fn clear(&mut self) {
        self.touches.clear();
        self.pointer = false;
    }
}
#[cfg(test)]
mod contact_tests {
    use super::*;
    #[test]
    fn all_contacts_must_end_including_pointer() {
        let mut c = Contacts::default();
        c.begin(1);
        c.begin(1);
        c.begin(2);
        c.pointer(true);
        c.end(&1);
        assert!(c.active());
        c.pointer(false);
        assert!(c.active());
        c.end(&2);
        assert!(!c.active());
        c.begin(3);
        c.clear();
        assert!(!c.active());
        c.end(&3);
        assert!(!c.active());
    }
}
