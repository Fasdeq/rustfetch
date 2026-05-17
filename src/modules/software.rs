use std::fs;
use std::path::Path;

pub struct SoftwareInfo {
    pub packages: String,
    pub de_wm: String,
    pub theme_icons: String,
    pub terminal_shell: String,
}

pub fn get_software() -> SoftwareInfo {
    let mut pacman_count = 0;
    if Path::new("/var/lib/pacman/local").exists() {
        if let Ok(entries) = fs::read_dir("/var/lib/pacman/local") {
            pacman_count = entries.count().saturating_sub(1);
        }
    }
    
    let mut flatpak_count = 0;
    if Path::new("/var/lib/flatpak/app").exists() {
        if let Ok(entries) = fs::read_dir("/var/lib/flatpak/app") {
            flatpak_count += entries.count();
        }
    }
    if Path::new("/var/lib/flatpak/runtime").exists() {
        if let Ok(entries) = fs::read_dir("/var/lib/flatpak/runtime") {
            flatpak_count += entries.count();
        }
    }
    let packages = format!("{} (pacman) | {} (flatpak)", pacman_count, flatpak_count);

    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "Unknown".to_string());
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "Unknown".to_string());
    let de_wm = format!("{} ({})", desktop, session_type);

    let mut theme = "Unknown".to_string();
    let mut icons = "Unknown".to_string();
    let home_dir = home::home_dir().unwrap_or_default();
    let gtk3_path = home_dir.join(".config/gtk-3.0/settings.ini");
    if let Ok(content) = fs::read_to_string(gtk3_path) {
        for line in content.lines() {
            if line.starts_with("gtk-theme-name=") {
                theme = line.replace("gtk-theme-name=", "").trim().to_string();
            } else if line.starts_with("gtk-icon-theme-name=") {
                icons = line.replace("gtk-icon-theme-name=", "").trim().to_string();
            }
        }
    }
    let theme_icons = format!("Theme: {} | Icons: {}", theme, icons);

    let term = std::env::var("TERM").unwrap_or_else(|_| "Unknown".to_string());
    let shell_path = std::env::var("SHELL").unwrap_or_else(|_| "Unknown".to_string());
    let shell = shell_path.split('/').last().unwrap_or("Unknown").to_string();
    
    let terminal_shell = format!("{} via {}", term, shell);

    SoftwareInfo { packages, de_wm, theme_icons, terminal_shell }
}
