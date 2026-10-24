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
    css.push_str(include_str!("../assets/lcars.css"));
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
    css
}
