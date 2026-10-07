//! Shared workspace vocabulary and pure transitions.
//!
//! This model performs no I/O and owns no GTK widgets. The existing renderer uses
//! the catalogue; state/event adapters are migrated separately after platform checks.
use std::collections::BTreeSet;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(usize)]
pub enum PanelId {
    System,
    Files,
    Network,
    Applications,
    Utilities,
    Preferences,
}
pub const PANEL_COUNT: usize = 6;
pub const PANEL_CATALOG: [(&str, &str); PANEL_COUNT] = [
    ("dashboard", "SYSTEM"),
    ("files", "FILES"),
    ("network", "NETWORK"),
    ("apps", "APPLICATIONS"),
    ("tools", "UTILITIES"),
    ("settings", "PREFERENCES"),
];
impl PanelId {
    pub const ALL: [Self; PANEL_COUNT] = [
        Self::System,
        Self::Files,
        Self::Network,
        Self::Applications,
        Self::Utilities,
        Self::Preferences,
    ];
    pub const fn index(self) -> usize {
        self as usize
    }
    pub fn from_route(route: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|id| PANEL_CATALOG[id.index()].0 == route)
    }
    pub const fn route(self) -> &'static str {
        PANEL_CATALOG[self.index()].0
    }
    pub const fn title(self) -> &'static str {
        PANEL_CATALOG[self.index()].1
    }
    pub fn contract(self) -> ContentContract {
        use AspectPreference::*;
        use ContentRole::*;
        let (minimum, preferred, aspect, essential, secondary) = match self {
            Self::System => (
                (240, 220),
                (360, 460),
                Tall,
                &[Status, Values][..],
                "extended history / sensor details",
            ),
            Self::Files => (
                (280, 240),
                (640, 480),
                Wide,
                &[Navigation, Selection, Actions][..],
                "secondary metadata columns",
            ),
            Self::Network => (
                (260, 240),
                (440, 440),
                Balanced,
                &[Status, Progress, Cancel][..],
                "historical traffic / diagnostic detail",
            ),
            Self::Applications => (
                (240, 200),
                (440, 360),
                Wide,
                &[Navigation, Actions][..],
                "descriptions / additional columns",
            ),
            Self::Utilities => (
                (280, 240),
                (560, 440),
                Wide,
                &[Values, Progress, Cancel][..],
                "extended diagnostic report",
            ),
            Self::Preferences => (
                (260, 220),
                (440, 380),
                Balanced,
                &[Selection, Actions][..],
                "explanatory text",
            ),
        };
        ContentContract {
            minimum: minimum.into(),
            preferred: preferred.into(),
            aspect,
            protected: essential,
            secondary,
        }
    }
}
/// Logical pixels, excluding shell chrome. Minima are initial design contracts,
/// not yet solver-enforced or validated touch-hardware measurements.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}
impl From<(u32, u32)> for Size {
    fn from(v: (u32, u32)) -> Self {
        Self {
            width: v.0,
            height: v.1,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AspectPreference {
    Tall,
    Wide,
    Balanced,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContentRole {
    Status,
    Values,
    Navigation,
    Selection,
    Actions,
    Progress,
    Cancel,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentContract {
    pub minimum: Size,
    pub preferred: Size,
    pub aspect: AspectPreference,
    /// Content that must remain available at every presentation size.
    pub protected: &'static [ContentRole],
    pub secondary: &'static str,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MediaKind {
    Image,
    Video,
    Other,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Selection {
    pub path: PathBuf,
    pub kind: MediaKind,
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ContentState {
    pub location: Option<PathBuf>,
    pub selection: Option<Selection>,
    pub query: String,
    pub scroll_row: usize,
    pub playback_ms: u64,
    pub playing: bool,
    pub result: String,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Activity {
    #[default]
    Idle,
    Browsing,
    Previewing,
    SpeedTest,
    Diagnostics,
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PanelState {
    pub content: ContentState,
    pub activity: Activity,
}
/// Only durable workspace preferences; session content/jobs/focus are not persisted here.
/// Existing geometry and settings formats remain owned by console/services until migration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DurableState {
    pub minimized: [bool; PANEL_COUNT],
}
impl Default for DurableState {
    fn default() -> Self {
        Self {
            minimized: [false; PANEL_COUNT],
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Arrangement {
    Basic,
    Advanced,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Keyboard {
    pub owner: PanelId,
    pub occluded_height: u32,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Workspace {
    durable: DurableState,
    panels: [PanelState; PANEL_COUNT],
    focus: Option<PanelId>,
    attention: Option<PanelId>,
    arranging: Option<Arrangement>,
    viewport: Size,
    keyboard: Option<Keyboard>,
    active_contacts: BTreeSet<u64>,
    pending_reflow: bool,
    keyboard_reflow_pending: bool,
}
impl Default for Workspace {
    fn default() -> Self {
        Self::restore(DurableState::default())
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    Activate(PanelId),
    Maximize(PanelId),
    Minimize(PanelId),
    Home,
    ReplaceContent(PanelId, ContentState),
    SetActivity(PanelId, Activity),
    RequestAttention(PanelId),
    ReleaseAttention(PanelId),
    Viewport(Size),
    Keyboard(Option<Keyboard>),
    ContactBegin(u64),
    ContactEnd(u64),
    BeginArrange,
    AdvancedArrange,
    EndArrange,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Effects {
    pub changed: bool,
    pub reflow: bool,
    pub durable_changed: bool,
}
impl Workspace {
    pub fn restore(durable: DurableState) -> Self {
        Self {
            durable,
            panels: std::array::from_fn(|_| PanelState::default()),
            focus: None,
            attention: None,
            arranging: None,
            viewport: Size::default(),
            keyboard: None,
            active_contacts: BTreeSet::new(),
            pending_reflow: false,
            keyboard_reflow_pending: false,
        }
    }
    pub fn durable(&self) -> &DurableState {
        &self.durable
    }
    pub fn panel(&self, id: PanelId) -> &PanelState {
        &self.panels[id.index()]
    }
    pub fn focus(&self) -> Option<PanelId> {
        self.focus
    }
    pub fn attention(&self) -> Option<PanelId> {
        self.attention
    }
    pub fn arrangement(&self) -> Option<Arrangement> {
        self.arranging
    }
    pub fn visible(&self, id: PanelId) -> bool {
        !self.durable.minimized[id.index()] && self.focus.is_none_or(|f| f == id)
    }
    pub fn usable_size(&self) -> Size {
        Size {
            width: self.viewport.width,
            height: self
                .viewport
                .height
                .saturating_sub(self.keyboard.map_or(0, |k| k.occluded_height)),
        }
    }
    /// Geometry is deferred while touching or arranging. Keyboard avoidance is
    /// allowed for its own event; subsequent attention waits until typing ends.
    pub fn apply(&mut self, event: Event) -> Effects {
        let before = self.clone();
        let mut layout = false;
        let keyboard_event = matches!(event, Event::Keyboard(_));
        match event {
            Event::Activate(id) => {
                if self.arranging.is_none() {
                    self.durable.minimized[id.index()] = false;
                    self.focus = if self.focus == Some(id) {
                        None
                    } else {
                        Some(id)
                    };
                    self.attention = None;
                    self.keyboard = None;
                    layout = true;
                }
            }
            Event::Maximize(id) => {
                self.durable.minimized[id.index()] = false;
                self.focus = Some(id);
                self.attention = None;
                self.arranging = None;
                self.keyboard = None;
                layout = true;
            }
            Event::Minimize(id) => {
                self.durable.minimized[id.index()] = true;
                if self.focus == Some(id) {
                    self.focus = None;
                }
                if self.attention == Some(id) {
                    self.attention = None;
                }
                if self.keyboard.is_some_and(|k| k.owner == id) {
                    self.keyboard = None;
                }
                layout = true;
            }
            Event::Home => {
                self.focus = None;
                self.attention = None;
                self.arranging = None;
                self.keyboard = None;
                layout = true;
            }
            Event::ReplaceContent(id, content) => {
                self.panels[id.index()].content = content;
            }
            Event::SetActivity(id, activity) => {
                let valid = match activity {
                    Activity::SpeedTest => id == PanelId::Network,
                    Activity::Diagnostics => {
                        matches!(id, PanelId::System | PanelId::Network | PanelId::Utilities)
                    }
                    Activity::Previewing => id == PanelId::Files,
                    Activity::Browsing => matches!(id, PanelId::Files | PanelId::Applications),
                    Activity::Idle => true,
                };
                if valid {
                    self.panels[id.index()].activity = activity;
                }
            }
            Event::RequestAttention(id) => {
                if self.focus.is_none()
                    && self.arranging.is_none()
                    && !self.durable.minimized[id.index()]
                {
                    self.attention = Some(id);
                    layout = true;
                }
            }
            Event::ReleaseAttention(id) => {
                if self.attention == Some(id) {
                    self.attention = None;
                    layout = true;
                }
            }
            Event::Viewport(size) => {
                self.viewport = size;
                layout = true;
            }
            Event::Keyboard(keyboard) => {
                if keyboard.is_none_or(|k| self.visible(k.owner)) {
                    self.keyboard = keyboard;
                    layout = true;
                }
            }
            Event::ContactBegin(token) => {
                self.active_contacts.insert(token);
            }
            Event::ContactEnd(token) => {
                self.active_contacts.remove(&token);
            }
            Event::BeginArrange => {
                self.focus = None;
                self.attention = None;
                self.keyboard = None;
                self.arranging = Some(Arrangement::Basic);
                layout = true;
            }
            Event::AdvancedArrange => {
                if self.arranging.is_some() {
                    self.arranging = Some(Arrangement::Advanced);
                }
            }
            Event::EndArrange => {
                self.arranging = None;
                layout = true;
            }
        }
        let changed = *self != before;
        self.pending_reflow |= layout && changed;
        self.keyboard_reflow_pending |= keyboard_event && layout && changed;
        let reflow = self.pending_reflow
            && self.active_contacts.is_empty()
            && self.arranging.is_none()
            && (self.keyboard.is_none() || self.keyboard_reflow_pending);
        if reflow {
            self.pending_reflow = false;
            self.keyboard_reflow_pending = false;
        }
        Effects {
            changed: changed || reflow,
            reflow,
            durable_changed: self.durable != before.durable,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn catalogue_preserves_routes_and_complete_contracts() {
        for id in PanelId::ALL {
            assert_eq!(PanelId::from_route(id.route()), Some(id));
            assert!(!id.title().is_empty());
            let c = id.contract();
            assert!(c.preferred.width >= c.minimum.width && c.preferred.height >= c.minimum.height);
            assert!(!c.protected.is_empty());
        }
        assert_eq!(PanelId::from_route("missing"), None);
    }
    #[test]
    fn focus_return_and_minimize_preserve_content_and_jobs() {
        let mut w = Workspace::default();
        let c = ContentState {
            location: Some("/home/fedora".into()),
            selection: Some(Selection {
                path: "/home/fedora/movie.webm".into(),
                kind: MediaKind::Video,
            }),
            query: "movie".into(),
            scroll_row: 7,
            playback_ms: 1200,
            playing: true,
            result: "metadata".into(),
        };
        w.apply(Event::ReplaceContent(PanelId::Files, c.clone()));
        w.apply(Event::SetActivity(PanelId::Network, Activity::SpeedTest));
        for e in [
            Event::Activate(PanelId::Files),
            Event::Activate(PanelId::Files),
            Event::Maximize(PanelId::Utilities),
            Event::Minimize(PanelId::Files),
            Event::Home,
            Event::Activate(PanelId::Files),
        ] {
            w.apply(e);
        }
        assert_eq!(w.panel(PanelId::Files).content, c);
        assert_eq!(w.panel(PanelId::Network).activity, Activity::SpeedTest);
        assert!(!w.durable().minimized[PanelId::Files.index()]);
    }
    #[test]
    fn active_title_returns_home_and_hidden_panels_do_not_receive_input() {
        let mut w = Workspace::default();
        w.apply(Event::Activate(PanelId::Files));
        assert_eq!(
            PanelId::ALL.into_iter().filter(|&id| w.visible(id)).count(),
            1
        );
        w.apply(Event::Activate(PanelId::Files));
        assert_eq!(w.focus(), None);
        w.apply(Event::Minimize(PanelId::Files));
        assert!(!w.visible(PanelId::Files));
        w.apply(Event::RequestAttention(PanelId::Files));
        assert_eq!(w.attention(), None);
    }
    #[test]
    fn incidental_content_and_activity_do_not_rearrange_or_steal_focus() {
        let mut w = Workspace::default();
        w.apply(Event::Maximize(PanelId::Files));
        assert!(
            !w.apply(Event::SetActivity(PanelId::Network, Activity::SpeedTest))
                .reflow
        );
        assert!(!w.apply(Event::RequestAttention(PanelId::Network)).reflow);
        assert_eq!(w.focus(), Some(PanelId::Files));
        assert!(
            !w.apply(Event::SetActivity(PanelId::Files, Activity::SpeedTest))
                .changed
        );
    }
    #[test]
    fn attention_is_temporary_and_does_not_persist() {
        let mut w = Workspace::default();
        w.apply(Event::RequestAttention(PanelId::Files));
        w.apply(Event::ReleaseAttention(PanelId::Network));
        assert_eq!(w.attention(), Some(PanelId::Files));
        w.apply(Event::ReleaseAttention(PanelId::Files));
        assert_eq!(w.attention(), None);
        w.apply(Event::Minimize(PanelId::Utilities));
        w.apply(Event::SetActivity(PanelId::Network, Activity::SpeedTest));
        let restored = Workspace::restore(w.durable().clone());
        assert!(!restored.visible(PanelId::Utilities));
        assert_eq!(restored.panel(PanelId::Network).activity, Activity::Idle);
    }
    #[test]
    fn multipoint_touch_defers_reflow_until_last_contact_ends() {
        let mut w = Workspace::default();
        w.apply(Event::ContactBegin(1));
        w.apply(Event::ContactBegin(2));
        assert!(!w.apply(Event::Viewport((900, 600).into())).reflow);
        assert!(!w.apply(Event::ContactEnd(99)).reflow);
        assert!(!w.apply(Event::ContactEnd(1)).reflow);
        assert!(w.apply(Event::ContactEnd(2)).reflow);
        assert!(!w.apply(Event::ContactEnd(2)).reflow);
    }
    #[test]
    fn keyboard_avoids_occlusion_and_defers_unrelated_attention() {
        let mut w = Workspace::default();
        w.apply(Event::Viewport((900, 600).into()));
        assert!(
            w.apply(Event::Keyboard(Some(Keyboard {
                owner: PanelId::Files,
                occluded_height: 240
            })))
            .reflow
        );
        assert_eq!(w.usable_size(), Size::from((900, 360)));
        assert!(!w.apply(Event::RequestAttention(PanelId::Files)).reflow);
        assert!(w.apply(Event::Keyboard(None)).reflow);
        w.apply(Event::Keyboard(Some(Keyboard {
            owner: PanelId::Files,
            occluded_height: 900,
        })));
        assert_eq!(w.usable_size().height, 0);
    }
    #[test]
    fn keyboard_avoidance_queued_during_touch_runs_on_release() {
        let mut w = Workspace::default();
        w.apply(Event::Viewport((900, 600).into()));
        w.apply(Event::ContactBegin(1));
        assert!(
            !w.apply(Event::Keyboard(Some(Keyboard {
                owner: PanelId::Files,
                occluded_height: 200
            })))
            .reflow
        );
        assert!(w.apply(Event::ContactEnd(1)).reflow);
        assert_eq!(w.usable_size().height, 400);
    }
    #[test]
    fn arrangement_always_enters_basic_and_blocks_attention() {
        let mut w = Workspace::default();
        w.apply(Event::AdvancedArrange);
        assert_eq!(w.arrangement(), None);
        w.apply(Event::BeginArrange);
        w.apply(Event::AdvancedArrange);
        assert_eq!(w.arrangement(), Some(Arrangement::Advanced));
        w.apply(Event::Activate(PanelId::Files));
        w.apply(Event::RequestAttention(PanelId::Network));
        assert_eq!(w.focus(), None);
        assert_eq!(w.attention(), None);
        w.apply(Event::EndArrange);
        w.apply(Event::BeginArrange);
        assert_eq!(w.arrangement(), Some(Arrangement::Basic));
    }
    #[test]
    fn idempotent_events_do_not_trigger_repeated_layouts() {
        let mut w = Workspace::default();
        w.apply(Event::Viewport((900, 600).into()));
        assert_eq!(
            w.apply(Event::Viewport((900, 600).into())),
            Effects::default()
        );
        w.apply(Event::RequestAttention(PanelId::Files));
        assert_eq!(
            w.apply(Event::RequestAttention(PanelId::Files)),
            Effects::default()
        );
    }
}
