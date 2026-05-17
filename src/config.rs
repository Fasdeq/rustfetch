use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModuleConfig {
    pub enabled: bool,
    pub label: String,
    pub icon: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorPalette {
    pub primary: String,
    pub secondary: String,
    pub separator: String,
    pub arrow: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayConfig {
    pub key_width: usize,
    pub ascii_padding_right: usize,
    pub uppercase_labels: bool,
    pub memory_bar_symbol: String,
    pub memory_bar_empty_symbol: String,
    pub colors: ColorPalette,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Modules {
    pub os: ModuleConfig,
    pub uptime: ModuleConfig,
    pub packages: ModuleConfig,
    pub de_wm: ModuleConfig,
    pub theme_icons: ModuleConfig,
    pub terminal: ModuleConfig,
    pub cpu: ModuleConfig,
    pub gpu: ModuleConfig,
    pub memory: ModuleConfig,
    pub disk: ModuleConfig,
    pub battery: ModuleConfig,
    pub network: ModuleConfig,
    pub updates: ModuleConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub logo_path: Option<String>,
    pub ascii_distro: String,
    pub display: DisplayConfig,
    pub modules: Modules,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            logo_path: None,
            ascii_distro: "arch".to_string(),
            display: DisplayConfig {
                key_width: 14,
                ascii_padding_right: 44,
                uppercase_labels: true,
                memory_bar_symbol: "■".to_string(),
                memory_bar_empty_symbol: "·".to_string(),
                colors: ColorPalette {
                    primary: "38;5;135".to_string(),
                    secondary: "38;5;33".to_string(),
                    separator: "38;5;135".to_string(),
                    arrow: "38;5;255".to_string(),
                    value: "38;5;255".to_string(),
                },
            },
            modules: Modules {
                os: ModuleConfig { enabled: true, label: "OS".to_string(), icon: "  ".to_string(), color: "cyan".to_string() },
                uptime: ModuleConfig { enabled: true, label: "Uptime".to_string(), icon: "  ".to_string(), color: "yellow".to_string() },
                packages: ModuleConfig { enabled: true, label: "Packages".to_string(), icon: "  ".to_string(), color: "blue".to_string() },
                de_wm: ModuleConfig { enabled: true, label: "DE/WM".to_string(), icon: "  ".to_string(), color: "magenta".to_string() },
                theme_icons: ModuleConfig { enabled: true, label: "Theme".to_string(), icon: "  ".to_string(), color: "cyan".to_string() },
                terminal: ModuleConfig { enabled: true, label: "Terminal".to_string(), icon: "  ".to_string(), color: "green".to_string() },
                cpu: ModuleConfig { enabled: true, label: "CPU".to_string(), icon: "  ".to_string(), color: "red".to_string() },
                gpu: ModuleConfig { enabled: true, label: "GPU".to_string(), icon: "  ".to_string(), color: "green".to_string() },
                memory: ModuleConfig { enabled: true, label: "Memory".to_string(), icon: "  ".to_string(), color: "yellow".to_string() },
                disk: ModuleConfig { enabled: true, label: "Disk".to_string(), icon: "  ".to_string(), color: "blue".to_string() },
                battery: ModuleConfig { enabled: true, label: "Battery".to_string(), icon: "  ".to_string(), color: "green".to_string() },
                network: ModuleConfig { enabled: true, label: "Network".to_string(), icon: "  ".to_string(), color: "magenta".to_string() },
                updates: ModuleConfig { enabled: true, label: "Updates".to_string(), icon: "  ".to_string(), color: "red".to_string() },
            },
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let mut path = home::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".config");
    path.push("rustfetch");
    path
}

pub fn load_config() -> Config {
    let mut path = get_config_path();
    path.push("config.json");
    if !path.exists() { return Config::default(); }
    let file_content = fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&file_content).unwrap_or_else(|_| Config::default())
}

pub fn generate_config() -> std::io::Result<()> {
    let mut dir_path = get_config_path();
    fs::create_dir_all(&dir_path)?;
    dir_path.push("config.json");
    if dir_path.exists() { return Ok(()); }
    let default_config = Config::default();
    let json = serde_json::to_string_pretty(&default_config).unwrap();
    let mut file = File::create(&dir_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
