use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc;
use std::time::Duration;

#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    pub cpu: f64,
    pub memory_used: u64,
    pub memory_total: u64,
    pub uptime: u64,
    pub received: u64,
    pub transmitted: u64,
    pub network: String,
    pub storage: String,
    pub error: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Preferences {
    pub sound: bool,
    pub reduced_motion: bool,
    #[serde(default)]
    pub workspace_mode: bool,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            sound: true,
            reduced_motion: true,
            workspace_mode: false,
        }
    }
}

pub fn home() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/"))
}

fn config_path() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(".config"))
        .join("lcars-desktop/preferences.json")
}

impl Preferences {
    pub fn load() -> Self {
        fs::read(config_path())
            .ok()
            .and_then(|s| serde_json::from_slice(&s).ok())
            .unwrap_or_default()
    }
    pub fn save(&self) -> Result<(), String> {
        let path = config_path();
        fs::create_dir_all(path.parent().ok_or("No configuration directory")?)
            .map_err(|e| e.to_string())?;
        let temp = path.with_extension("tmp");
        fs::write(
            &temp,
            serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        fs::rename(temp, path).map_err(|e| e.to_string())
    }
}

pub fn command(program: &str, args: &[&str]) -> Result<String, String> {
    let out = Command::new("timeout")
        .env("LC_ALL", "C")
        .arg("12")
        .arg(program)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        let message = String::from_utf8_lossy(&out.stderr).trim().to_string();
        Err(if message.is_empty() {
            format!("{program}: timeout or unsuccessful result ({})", out.status)
        } else {
            message
        })
    }
}

pub fn network_state(raw: &str) -> &'static str {
    let mut parts = raw.trim().split(':');
    let state = parts.next().unwrap_or("");
    let connectivity = parts.next().unwrap_or("");
    if state == "disconnected" || state == "asleep" {
        "OFFLINE"
    } else if connectivity == "full" {
        "ONLINE"
    } else if connectivity == "portal" {
        "CAPTIVE PORTAL"
    } else if connectivity == "limited" {
        "LIMITED"
    } else if state.starts_with("connected") {
        "LOCAL / UNVERIFIED"
    } else {
        "UNKNOWN"
    }
}

fn cpu_ticks(text: &str) -> Option<(u64, u64)> {
    let values: Vec<u64> = text
        .lines()
        .next()?
        .split_whitespace()
        .skip(1)
        .take(8)
        .map(str::parse)
        .collect::<Result<_, _>>()
        .ok()?;
    if values.len() < 5 {
        return None;
    }
    Some((values.iter().sum(), values[3] + values[4]))
}

fn cpu_percent(before: (u64, u64), after: (u64, u64)) -> f64 {
    let total = after.0.saturating_sub(before.0);
    let idle = after.1.saturating_sub(before.1);
    if total == 0 {
        0.0
    } else {
        (100.0 * total.saturating_sub(idle) as f64 / total as f64).clamp(0.0, 100.0)
    }
}

fn traffic() -> (u64, u64) {
    let text = fs::read_to_string("/proc/net/dev").unwrap_or_default();
    text.lines()
        .filter_map(|line| line.split_once(':'))
        .filter(|(name, _)| name.trim() != "lo")
        .fold((0, 0), |(rx, tx), (_, data)| {
            let fields: Vec<&str> = data.split_whitespace().collect();
            (
                rx + fields
                    .first()
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(0),
                tx + fields
                    .get(8)
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(0),
            )
        })
}

pub fn monitor() -> mpsc::Receiver<Snapshot> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut previous_cpu = None;
        let mut previous_traffic = traffic();
        let mut previous_time = std::time::Instant::now();
        let mut network = "CHECKING".to_string();
        let mut storage = "Checking storage…".to_string();
        let mut iteration = 0u64;
        loop {
            let mut snap = Snapshot::default();
            match fs::read_to_string("/proc/stat")
                .ok()
                .and_then(|s| cpu_ticks(&s))
            {
                Some(now) => {
                    snap.cpu = previous_cpu.map(|old| cpu_percent(old, now)).unwrap_or(0.0);
                    previous_cpu = Some(now);
                }
                None => snap.error = "CPU telemetry unavailable".into(),
            }
            if let Ok(mem) = fs::read_to_string("/proc/meminfo") {
                let read = |name: &str| -> u64 {
                    mem.lines()
                        .find(|l| l.starts_with(name))
                        .and_then(|l| l.split_whitespace().nth(1))
                        .and_then(|v| v.parse::<u64>().ok())
                        .unwrap_or(0)
                        * 1024
                };
                snap.memory_total = read("MemTotal:");
                snap.memory_used = snap.memory_total.saturating_sub(read("MemAvailable:"));
            }
            snap.uptime = fs::read_to_string("/proc/uptime")
                .ok()
                .and_then(|s| s.split_whitespace().next()?.parse::<f64>().ok())
                .unwrap_or(0.0) as u64;
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(previous_time).as_secs_f64().max(0.001);
            let bytes = traffic();
            snap.received = (bytes.0.saturating_sub(previous_traffic.0) as f64 / elapsed) as u64;
            snap.transmitted = (bytes.1.saturating_sub(previous_traffic.1) as f64 / elapsed) as u64;
            previous_traffic = bytes;
            previous_time = now;
            if iteration.is_multiple_of(5) {
                network = command("nmcli", &["-t", "-f", "STATE,CONNECTIVITY", "general"])
                    .map(|s| network_state(&s).to_string())
                    .unwrap_or_else(|_| "UNKNOWN".into());
            }
            if iteration.is_multiple_of(15) {
                storage = command("df", &["-h", "--output=size,used,avail,pcent", "/home"])
                    .ok()
                    .and_then(|s| s.lines().nth(1).map(str::trim).map(str::to_owned))
                    .unwrap_or_else(|| "Unavailable".into());
            }
            snap.network.clone_from(&network);
            snap.storage.clone_from(&storage);
            if tx.send(snap).is_err() {
                break;
            }
            iteration += 1;
            std::thread::sleep(Duration::from_secs(1));
        }
    });
    rx
}

pub fn diagnostics() -> String {
    let mut report =
        String::from("LOCAL DIAGNOSTICS\nRead-only inspection; no connections were changed.\n\n");
    for (label, program, args) in [
        ("NETWORKMANAGER", "nmcli", vec!["general", "status"]),
        ("DEVICES", "nmcli", vec!["device", "status"]),
        ("IP ADDRESSES", "ip", vec!["-brief", "address"]),
        ("ROUTES", "ip", vec!["route"]),
        (
            "DNS CONFIGURATION",
            "nmcli",
            vec!["-g", "IP4.DNS,IP6.DNS", "device", "show"],
        ),
    ] {
        report.push_str(&format!(
            "{label}\n{}\n\n",
            command(program, &args).unwrap_or_else(|e| e)
        ));
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn network_does_not_confuse_link_and_internet() {
        assert_eq!(network_state("connected:full"), "ONLINE");
        assert_eq!(network_state("connected:unknown"), "LOCAL / UNVERIFIED");
        assert_eq!(network_state("connected:portal"), "CAPTIVE PORTAL");
        assert_eq!(network_state("disconnected:none"), "OFFLINE");
        assert_eq!(network_state(""), "UNKNOWN");
    }
    #[test]
    fn cpu_handles_resets_and_ignores_guest_double_counting() {
        assert_eq!(cpu_ticks("cpu 10 0 10 70 10 0 0 0 5 0"), Some((100, 80)));
        assert_eq!(cpu_percent((100, 80), (200, 150)), 30.0);
        assert_eq!(cpu_percent((200, 150), (100, 80)), 0.0);
    }
}

pub fn drive_labels(ids: &[String]) -> Result<Vec<String>, String> {
    let path = config_path().with_file_name("drive-labels.json");
    let mut map: std::collections::BTreeMap<String, String> = match fs::read(&path) {
        Ok(bytes) => serde_json::from_slice(&bytes).map_err(|e| format!("Drive mappings: {e}"))?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Default::default(),
        Err(e) => return Err(e.to_string()),
    };
    let result = assign_drive_labels(&mut map, ids);
    fs::create_dir_all(path.parent().ok_or("No config directory")?).map_err(|e| e.to_string())?;
    let temp = path.with_extension("tmp");
    fs::write(
        &temp,
        serde_json::to_vec_pretty(&map).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    fs::rename(temp, path).map_err(|e| e.to_string())?;
    Ok(result)
}
fn assign_drive_labels(
    map: &mut std::collections::BTreeMap<String, String>,
    ids: &[String],
) -> Vec<String> {
    ids.iter()
        .map(|id| {
            if let Some(label) = map.get(id) {
                return label.clone();
            }
            let name = (b'D'..=b'Z')
                .map(|c| format!("{}:", c as char))
                .find(|s| !map.values().any(|v| v == s))
                .unwrap_or_else(|| format!("VOL-{}", map.len() + 1));
            map.insert(id.clone(), name.clone());
            name
        })
        .collect()
}
#[cfg(test)]
mod drive_tests {
    use super::*;
    #[test]
    fn drive_labels_survive_reordering_and_unplugging() {
        let mut map = std::collections::BTreeMap::new();
        assert_eq!(
            assign_drive_labels(&mut map, &["disk-a".into(), "disk-b".into()]),
            ["D:", "E:"]
        );
        assert_eq!(
            assign_drive_labels(&mut map, &["disk-b".into(), "disk-c".into()]),
            ["E:", "F:"]
        );
        assert_eq!(assign_drive_labels(&mut map, &["disk-a".into()]), ["D:"]);
    }
}

#[cfg(test)]
mod preference_tests {
    use super::Preferences;
    #[test]
    fn old_preferences_preserve_sound_and_default_to_single_page() {
        let p: Preferences =
            serde_json::from_str(r#"{"sound":false,"reduced_motion":true}"#).unwrap();
        assert!(!p.sound);
        assert!(!p.workspace_mode);
    }
    #[test]
    fn workspace_mode_survives_serialization() {
        let p = Preferences {
            workspace_mode: true,
            ..Preferences::default()
        };
        let restored: Preferences =
            serde_json::from_str(&serde_json::to_string(&p).unwrap()).unwrap();
        assert!(restored.workspace_mode);
    }
}
