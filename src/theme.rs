// Mixed provenance: Codex shared-theme foundation; Claude AI continuation (user-attributed)
// added reference palette/font/rail/accent refinements. See docs/WORK_PROVENANCE.md.
//! Shared palette for GTK widgets and Cairo instruments.
/// Width of the vertical elbow rail. Layout maths and the stylesheet share this.
pub const RAIL: i32 = 10;
pub const VOID: u32 = 0x000000;
pub const AMBER: u32 = 0xff9900;
pub const CORAL: u32 = 0xcd6667;
pub const LILAC: u32 = 0x9c99ff;
pub const ROSE: u32 = 0xcb99ce;
pub const MUTED: u32 = 0x9999cc;
pub const RULE: u32 = 0x49374b;
pub const PRESS: u32 = 0xffcc66;
pub const ALERT: u32 = 0xff7777;
pub const TAN: u32 = 0xffcc66;
pub const MAUVE: u32 = 0xb56e94;
/// Typeface shared by the stylesheet and the Cairo instruments.
pub const FONT: &str = "Oswald";
/// One accent per built-in system, in panel order, sampled from the reference.
pub const ACCENTS: [u32; 6] = [AMBER, CORAL, LILAC, ROSE, TAN, MAUVE];
pub const fn rgb(color: u32) -> (f64, f64, f64) {
    (
        ((color >> 16) & 255) as f64 / 255.0,
        ((color >> 8) & 255) as f64 / 255.0,
        (color & 255) as f64 / 255.0,
    )
}
pub fn stylesheet() -> String {
    let mut css = String::new();
    for (name, color) in [
        ("void", VOID),
        ("amber", AMBER),
        ("coral", CORAL),
        ("lilac", LILAC),
        ("rose", ROSE),
        ("muted", MUTED),
        ("rule", RULE),
        ("press", PRESS),
        ("alert", ALERT),
    ] {
        css.push_str(&format!("@define-color lcars_{name} #{color:06x};\n"));
    }
    css.push_str(include_str!("../assets/lcars.gtkcss"));
    css.push_str("label.sidebar-heading { color: @lcars_amber; font-weight: 700; font-size: 13px; opacity: 1; }\n");
    css.push_str(r#"
.history-panel progressbar { min-height: 22px; padding: 2px; }
.history-panel progressbar trough { min-height: 18px; border-radius: 10px; background-color: #211b2b; border: 1px solid #65516f; }
.history-panel progressbar progress { min-height: 18px; border-radius: 9px; }
.history-panel progressbar.cpu-meter progress { background-image: linear-gradient(to right, #ffd36f, #ff985e); }
.history-panel progressbar.ram-meter progress { background-image: linear-gradient(to right, #63d8c8, #9c99ff); }
.history-panel progressbar.storage-meter progress { background-image: linear-gradient(to right, #f18a9d, #cd6667); }
.history-panel label.cpu-meter-label { color: #ffd36f; }
.history-panel label.ram-meter-label { color: #72dfd0; }
.history-panel label.storage-meter-label { color: #f18a9d; }
"#);
    // Deep-space treatment is scoped to the Home dashboard; other pages keep their existing theme.
    css.push_str(r#"
.home-dashboard {
    background-color: #070b18;
    background-image: linear-gradient(135deg, #070b18 0%, #10172b 48%, #1b1028 100%);
    border: 1px solid #293653;
    border-radius: 18px;
    padding: 12px;
}
.home-dashboard .history-panel {
    background-color: rgba(14, 19, 38, 0.92);
    border: 1px solid #34415f;
    border-radius: 14px;
    padding: 12px;
}
"#);
    css.push_str("button.wifi-status-button { border-radius: 14px; padding: 6px 10px; min-width: 42px; min-height: 38px; }\n");
    // Rounded navigation pills for the persistent left sidebar only.
    css.push_str("button.sidebar-nav-button { border-radius: 18px; padding: 9px 12px; min-width: 120px; min-height: 42px; }\nbutton.sidebar-nav-button:hover, button.sidebar-nav-button:hover:focus { background-image: none; background-color: #ffcc66; color: #171020; border-color: #ffcc66; border-radius: 18px; }\nbutton.sidebar-nav-button:hover label, button.sidebar-nav-button:hover image { color: #171020; }\nbutton.sidebar-nav-button:active { background-image: none; background-color: #ff9900; color: #171020; border-radius: 18px; }\n");
    css.push_str(&format!(
        ".console-region {{ border-left-width:{RAIL}px; border-top-left-radius:{}px; }}\n",
        RAIL + 2
    ));
    css.push_str(&format!(
        "* {{ font-family:\"{FONT}\",\"Nimbus Sans Narrow\",sans-serif; }}\n"
    ));
    for (i, accent) in ACCENTS.iter().enumerate() {
        css.push_str(&format!(
            ".console-region.accent-{0} {{ border-left-color:#{1:06x}; }}\n\
             .accent-{0} .console-band-title {{ background:#{1:06x}; }}\n\
             .accent-{0} .system-band:hover .console-band-title {{ background:@lcars_press; }}\n",
            i + 1,
            accent
        ));
    }
    // Modern Files workspace: restrained navy surfaces, subtle borders, and
    // crisp LCARS amber/cyan accents. Scoped so other pages are unaffected.
    css.push_str(r#"
.modern-files-page { background-color: #0b1020; }
.modern-files-page .files-hero {
    background-image: linear-gradient(115deg, #17233a 0%, #17172d 58%, #24172c 100%);
    border: 1px solid #35445f; border-left: 5px solid #ff9900;
    border-radius: 14px; padding: 15px 18px; margin-bottom: 4px;
}
.modern-files-page .files-eyebrow { color: #75d9df; font-size: 11px; font-weight: 700; letter-spacing: 1.5px; }
.modern-files-page .files-hero-title { color: #f5f3ff; font-size: 24px; font-weight: 800; }
.modern-files-page .files-hero-subtitle { color: #aab7d0; font-size: 12px; }
.modern-files-page .files-toolbar,
.modern-files-page .files-shortcuts,
.modern-files-page .files-mounted-volumes { padding: 5px 0; }
.modern-files-page entry,
.modern-files-page searchentry { background-color: #151e31; color: #eef2ff; border: 1px solid #3a4a68; border-radius: 9px; padding: 8px 11px; }
.modern-files-page .files-status-line { color: #7dd8d8; font-size: 11px; padding: 2px 4px; }
.modern-files-page .files-list-card { background-color: #111a2b; border: 1px solid #34435d; border-radius: 12px; padding: 9px; }
.modern-files-page .files-list-heading { background-color: #1b2940; border-radius: 7px; padding: 9px 10px; }
.modern-files-page .file-row { padding: 7px 8px; border-bottom: 1px solid rgba(112, 137, 170, 0.16); }
.modern-files-page list.directory { background-color: transparent; }
.modern-files-page list.directory row { background-color: transparent; border-radius: 7px; }
.modern-files-page list.directory row:hover { background-color: #202d43; }
.modern-files-page list.directory row:selected { background-color: #293953; }
"#);
    // Make the Wi-Fi status symbol red only when the network is offline.
    css.push_str("button.wifi-status-button.network-offline image { color: #ff454f; }\nbutton.wifi-status-button.network-offline { color: #ff454f; }\n");
    // Apply the pill shape in the normal state as well as every interaction state.
    // Keep this rule after the imported theme so it wins over dock-button defaults.
    css.push_str(r#"
button.sidebar-nav-button,
button.sidebar-nav-button:disabled,
button.sidebar-nav-button:focus,
button.sidebar-nav-button:focus-visible,
button.sidebar-nav-button:hover,
button.sidebar-nav-button:active,
button.sidebar-nav-button:checked,
button.dock-button.sidebar-nav-button,
button.dock-button.sidebar-nav-button:disabled,
button.dock-button.sidebar-nav-button:focus,
button.dock-button.sidebar-nav-button:focus-visible,
button.dock-button.sidebar-nav-button:hover,
button.dock-button.sidebar-nav-button:active,
button.dock-button.sidebar-nav-button:checked {
    border-radius: 22px 22px 22px 22px;
    border-top-left-radius: 22px;
    border-top-right-radius: 22px;
    border-bottom-right-radius: 22px;
    border-bottom-left-radius: 22px;
    background-clip: border-box;
    box-shadow: none;
    padding: 10px 14px;
}
"#);
    css.push_str(r#"
button.sidebar-nav-button.dock-button,
button.sidebar-nav-button.dock-button:hover,
button.sidebar-nav-button.dock-button:focus,
button.sidebar-nav-button.dock-button:active {
    border-radius: 22px 22px 22px 22px;
    background-clip: border-box;
}
"#);
    css
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "requires a GTK display; run via scripts/verify.sh"]
    fn gtk_stylesheet_parses() {
        use std::{cell::RefCell, rc::Rc};
        gtk::init().expect("GTK display required for stylesheet validation");
        let provider = gtk::CssProvider::new();
        let errors = Rc::new(RefCell::new(Vec::new()));
        let captured = errors.clone();
        provider.connect_parsing_error(move |_, _, error| {
            captured.borrow_mut().push(error.to_string());
        });
        provider.load_from_data(&super::stylesheet());
        assert!(
            errors.borrow().is_empty(),
            "GTK stylesheet diagnostics: {:?}",
            errors.borrow()
        );
    }
}
