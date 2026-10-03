use gtk::{gio, glib, prelude::*};
use std::process::{Command, Stdio};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};
use std::time::{Duration, Instant};

pub fn subscribe(callback: impl Fn(String) + 'static) {
    gio::DBusProxy::for_bus(
        gio::BusType::System,
        gio::DBusProxyFlags::NONE,
        None,
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager",
        "org.freedesktop.NetworkManager",
        None::<&gio::Cancellable>,
        move |result| {
            let Ok(proxy) = result else {
                callback("UNKNOWN".into());
                return;
            };
            let callback = std::rc::Rc::new(callback);
            let update = move |p: &gio::DBusProxy| {
                if p.name_owner().is_none() {
                    callback("UNKNOWN".into());
                    return;
                }
                let state = p
                    .cached_property("State")
                    .and_then(|v| v.get::<u32>())
                    .unwrap_or(0);
                let connectivity = p
                    .cached_property("Connectivity")
                    .and_then(|v| v.get::<u32>())
                    .unwrap_or(0);
                callback(classify(state, connectivity).into());
            };
            update(&proxy);
            let update = std::rc::Rc::new(update);
            let owner_update = update.clone();
            proxy.connect_notify_local(Some("g-name-owner"), move |p, _| owner_update(p));
            let p = proxy.clone();
            proxy.connect_local("g-properties-changed", false, move |_| {
                update(&p);
                None
            });
            // Retain the proxy for the application's lifetime, including service restarts.
            glib::timeout_add_local(Duration::from_secs(60), move || {
                let _ = proxy.name_owner();
                glib::ControlFlow::Continue
            });
        },
    );
}

pub fn speed_test(cancel: Arc<AtomicBool>, tx: mpsc::Sender<String>) -> Result<String, String> {
    let mut results = Vec::new();
    for (phase, size, upload) in [
        ("LATENCY", 0, false),
        ("DOWNLOAD", 10_000_000, false),
        ("UPLOAD", 2_000_000, true),
    ] {
        if cancel.load(Ordering::Relaxed) {
            return Err("Cancelled".into());
        }
        let _ = tx.send(format!("{phase} / measuring…"));
        let mut cmd = Command::new("curl");
        cmd.args([
            "--silent",
            "--show-error",
            "--fail",
            "--connect-timeout",
            "5",
            "--max-time",
            "25",
            "--output",
            "/dev/null",
            "--write-out",
            "%{time_starttransfer} %{speed_download} %{speed_upload}",
        ]);
        let payload = std::env::temp_dir().join(format!("lcars-speed-{}.bin", std::process::id()));
        if upload {
            use std::io::Write;
            use std::os::unix::fs::OpenOptionsExt;
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(&payload)
                .map_err(|e| e.to_string())?;
            file.write_all(&vec![0u8; size])
                .map_err(|e| e.to_string())?;
            cmd.args(["--request", "POST", "--data-binary"])
                .arg(format!("@{}", payload.display()))
                .arg("https://speed.cloudflare.com/__up");
        } else {
            cmd.arg(format!("https://speed.cloudflare.com/__down?bytes={size}"));
        }
        let mut child = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;
        let started = Instant::now();
        loop {
            if cancel.load(Ordering::Relaxed) || started.elapsed() > Duration::from_secs(30) {
                let _ = child.kill();
                let _ = child.wait();
                if upload {
                    let _ = std::fs::remove_file(&payload);
                }
                return Err("Cancelled or timed out".into());
            }
            if child.try_wait().map_err(|e| e.to_string())?.is_some() {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        let out = child.wait_with_output().map_err(|e| e.to_string())?;
        if upload {
            let _ = std::fs::remove_file(&payload);
        }
        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).into_owned());
        }
        let vals: Vec<f64> = String::from_utf8_lossy(&out.stdout)
            .split_whitespace()
            .filter_map(|v| v.parse().ok())
            .collect();
        if vals.len() != 3 {
            return Err("Invalid measurement response".into());
        }
        results.push(match phase {
            "LATENCY" => format!("HTTP RESPONSE  {:.0} ms", vals[0] * 1000.0),
            "DOWNLOAD" => format!("DOWNLOAD  {:.2} Mbps", vals[1] * 8.0 / 1_000_000.0),
            _ => format!("UPLOAD  {:.2} Mbps", vals[2] * 8.0 / 1_000_000.0),
        });
    }
    Ok(format!(
        "{}\nCloudflare • bounded single-connection sample, not maximum line capacity",
        results.join("\n")
    ))
}

fn classify(state: u32, connectivity: u32) -> &'static str {
    match (state, connectivity) {
        (0, _) => "UNKNOWN",
        (10 | 20 | 30, _) => "OFFLINE",
        (_, 4) => "ONLINE",
        (_, 2) => "CAPTIVE PORTAL",
        (_, 3) => "LIMITED",
        _ => "LOCAL / UNVERIFIED",
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn disconnection_overrides_stale_full_connectivity() {
        assert_eq!(classify(20, 4), "OFFLINE");
        assert_eq!(classify(70, 4), "ONLINE");
        assert_eq!(classify(50, 2), "CAPTIVE PORTAL");
        assert_eq!(classify(50, 0), "LOCAL / UNVERIFIED");
        assert_eq!(classify(0, 4), "UNKNOWN");
    }
    #[test]
    fn cancelled_test_does_not_start_network_io() {
        let (tx, _rx) = mpsc::channel();
        assert_eq!(
            speed_test(Arc::new(AtomicBool::new(true)), tx),
            Err("Cancelled".into())
        );
    }
}
