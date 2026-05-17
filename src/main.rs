mod config;
mod modules;

use clap::{Parser, Subcommand};
use colored::*;
use regex::Regex;
use std::fs;
use std::collections::HashMap;
use crate::config::{Config, ModuleConfig};

#[derive(Parser)]
#[command(name = "rustfetch")]
struct Cli {
    #[arg(short, long)]
    logo: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
enum Commands {
    GenerateConfig,
}

#[derive(Debug, Clone)]
struct FastfetchLogoData {
    colors: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(Commands::GenerateConfig) = cli.command {
        if let Err(e) = config::generate_config() {
            eprintln!("Error: {}", e);
        } else {
            println!("Config generated inside ~/.config/rustfetch/config.json");
        }
        return;
    }

    let cfg = config::load_config();

    let username = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
    let hostname = fs::read_to_string("/proc/sys/kernel/hostname").unwrap_or_default().trim().to_string();
    
    let mut os_name = "Linux".to_string();
    let mut detected_distro = "arch".to_string();
    if let Ok(os_release) = fs::read_to_string("/etc/os-release") {
        for line in os_release.lines() {
            if line.starts_with("PRETTY_NAME=") {
                os_name = line.replace("PRETTY_NAME=", "").replace('"', "").trim().to_string();
            }
            if line.starts_with("ID=") {
                detected_distro = line.replace("ID=", "").replace('"', "").trim().to_lowercase();
            }
        }
    }

    let distro_logo = cli.logo.unwrap_or_else(|| {
        if os_name.to_lowercase().contains("endeavour") {
            "endeavouros".to_string()
        } else {
            detected_distro
        }
    });

    let kernel = fs::read_to_string("/proc/sys/kernel/osrelease").unwrap_or_default().trim().to_string();
    
    let mut uptime_secs = 0.0;
    if let Ok(uptime_str) = fs::read_to_string("/proc/uptime") {
        if let Some(first_part) = uptime_str.split_whitespace().next() {
            uptime_secs = first_part.parse::<f64>().unwrap_or(0.0);
        }
    }
    let hours = (uptime_secs / 3600.0) as u64;
    let minutes = ((uptime_secs % 3600.0) / 60.0) as u64;

    let hw = modules::hardware::get_hardware(&cfg);
    let sw = modules::software::get_software();
    let feat = modules::features::get_features();

    let u_color = &cfg.display.colors.primary;
    let user_host_line = format!("\x1b[{}m{}\x1b[0m@\x1b[{}m{}\x1b[0m", u_color, username, u_color, hostname);
    
    let sep_color = &cfg.display.colors.separator;
    let sep_line = format!("\x1b[{}m{}\x1b[0m", sep_color, "─".repeat(username.len() + 1 + hostname.len()));

    let mut info_lines = vec![user_host_line, sep_line];

    let mut add_line = |m_cfg: &ModuleConfig, value: &str| {
        if m_cfg.enabled {
            let label = if cfg.display.uppercase_labels {
                m_cfg.label.to_uppercase()
            } else {
                m_cfg.label.clone()
            };
            
            let icon_str = match m_cfg.label.as_str() {
                "OS"        => "   ",
                "Uptime"    => "   ",
                "Packages"  => "   ",
                "DE/WM"     => "   ",
                "Theme"     => "   ",
                "Terminal"  => "   ",
                "CPU"       => "   ",
                "GPU"       => "   ",
                "Memory"    => "   ",
                "Disk"      => "   ",
                "Battery"   => "   ",
                "Network"   => "   ",
                "Updates"   => "   ",
                _           => &m_cfg.icon,
            };

            let key = format!("{} {}", icon_str, label);
            let formatted_key = match m_cfg.color.as_str() {
                "red" => key.bold().red(),
                "green" => key.bold().green(),
                "yellow" => key.bold().yellow(),
                "blue" => key.bold().blue(),
                "magenta" => key.bold().magenta(),
                "cyan" => key.bold().cyan(),
                _ => key.bold().white(),
            };
            let padding = " ".repeat(cfg.display.key_width.saturating_sub(label.chars().count() + 2));
            info_lines.push(format!("{}{}\x1b[{}m ➔ \x1b[0m\x1b[{}m{}\x1b[0m", formatted_key, padding, cfg.display.colors.arrow, cfg.display.colors.value, value));
        }
    };

    add_line(&cfg.modules.os, &format!("{} | {} | systemd", os_name, kernel));
    add_line(&cfg.modules.uptime, &format!("{} hours, {} mins", hours, minutes));
    add_line(&cfg.modules.packages, &sw.packages);
    add_line(&cfg.modules.de_wm, &sw.de_wm);
    add_line(&cfg.modules.theme_icons, &sw.theme_icons);
    add_line(&cfg.modules.terminal, &sw.terminal_shell);
    add_line(&cfg.modules.cpu, &hw.cpu);
    add_line(&cfg.modules.gpu, &hw.gpu);
    add_line(&cfg.modules.memory, &hw.memory);
    add_line(&cfg.modules.disk, &hw.disk);
    add_line(&cfg.modules.battery, &hw.battery);
    add_line(&cfg.modules.network, &format!("Local IP: {}", feat.network));
    add_line(&cfg.modules.updates, &feat.updates);

    render_split(&distro_logo, info_lines, &cfg);
}

fn parse_ff_color(color_token: &str) -> String {
    let clean = color_token.trim().trim_end_matches(',').trim_matches('"').trim();
    match clean {
        "FF_COLOR_FG_BLACK"         => "\x1b[30m".to_string(),
        "FF_COLOR_FG_RED"           => "\x1b[31m".to_string(),
        "FF_COLOR_FG_GREEN"         => "\x1b[32m".to_string(),
        "FF_COLOR_FG_YELLOW"        => "\x1b[33m".to_string(),
        "FF_COLOR_FG_BLUE"          => "\x1b[34m".to_string(),
        "FF_COLOR_FG_MAGENTA"       => "\x1b[35m".to_string(),
        "FF_COLOR_FG_CYAN"          => "\x1b[36m".to_string(),
        "FF_COLOR_FG_WHITE"         => "\x1b[37m".to_string(),
        "FF_COLOR_FG_LIGHT_BLACK"   => "\x1b[90m".to_string(),
        "FF_COLOR_FG_LIGHT_RED"     => "\x1b[91m".to_string(),
        "FF_COLOR_FG_LIGHT_GREEN"   => "\x1b[92m".to_string(),
        "FF_COLOR_FG_LIGHT_YELLOW"  => "\x1b[93m".to_string(),
        "FF_COLOR_FG_LIGHT_BLUE"    => "\x1b[94m".to_string(),
        "FF_COLOR_FG_LIGHT_MAGENTA" => "\x1b[95m".to_string(),
        "FF_COLOR_FG_LIGHT_CYAN"    => "\x1b[96m".to_string(),
        "FF_COLOR_FG_LIGHT_WHITE"   => "\x1b[97m".to_string(),
        "FF_COLOR_FG_DEFAULT"       => "\x1b[39m".to_string(),
        _ => {
            if clean.contains("FF_COLOR_FG_256") {
                if let Some(start) = clean.find('"') {
                    if let Some(end) = clean[start+1..].find('"') {
                        let code = &clean[start+1..start+1+end];
                        return format!("\x1b[38;5;{}m", code);
                    }
                }
            }
            if clean.contains("FF_COLOR_FG_RGB") {
                if let Some(start) = clean.find('"') {
                    if let Some(end) = clean[start+1..].find('"') {
                        let code = &clean[start+1..start+1+end];
                        return format!("\x1b[38;2;{}m", code);
                    }
                }
            }
            "\x1b[37m".to_string()
        }
    }
}

fn parse_builtin_c() -> HashMap<String, FastfetchLogoData> {
    let mut map = HashMap::new();
    let content = include_str!("builtin.c");

    let mut current_names = Vec::new();
    let mut current_colors = Vec::new();
    let mut in_names = false;
    let mut in_colors = false;

    for line in content.lines() {
        let line_trimmed = line.trim();

        if line_trimmed.starts_with(".names") {
            current_names.clear();
            current_colors.clear();
            in_names = true;
            if let Some(start) = line_trimmed.find('{') {
                let sub = &line_trimmed[start+1..];
                for part in sub.split(',') {
                    let name = part.trim().trim_matches('}').trim().trim_matches('"').to_lowercase();
                    if !name.is_empty() { current_names.push(name); }
                }
            }
            if line_trimmed.contains('}') { in_names = false; }
            continue;
        }

        if in_names {
            for part in line_trimmed.split(',') {
                let name = part.trim().trim_matches('}').trim().trim_matches('"').to_lowercase();
                if !name.is_empty() { current_names.push(name); }
            }
            if line_trimmed.contains('}') { in_names = false; }
            continue;
        }

        if line_trimmed.starts_with(".colors") {
            in_colors = true;
            if let Some(start) = line_trimmed.find('{') {
                let sub = &line_trimmed[start+1..];
                for part in sub.split(',') {
                    let cleaned = part.trim().trim_matches('}').trim();
                    if !cleaned.is_empty() && !cleaned.starts_with("//") {
                        current_colors.push(parse_ff_color(cleaned));
                    }
                }
            }
            if line_trimmed.contains('}') {
                in_colors = false;
                if !current_names.is_empty() {
                    let logo_data = FastfetchLogoData { colors: current_colors.clone() };
                    for name in &current_names { map.insert(name.clone(), logo_data.clone()); }
                }
            }
            continue;
        }

        if in_colors {
            if line_trimmed.contains('}') {
                in_colors = false;
                let clean_line = line_trimmed.trim_matches('}').trim();
                if !clean_line.is_empty() && !clean_line.starts_with("//") {
                    current_colors.push(parse_ff_color(clean_line));
                }
                if !current_names.is_empty() {
                    let logo_data = FastfetchLogoData { colors: current_colors.clone() };
                    for name in &current_names { map.insert(name.clone(), logo_data.clone()); }
                }
            } else {
                let cleaned = line_trimmed.trim_end_matches(',');
                if !cleaned.is_empty() && !cleaned.starts_with("//") {
                    current_colors.push(parse_ff_color(cleaned));
                }
            }
            continue;
        }
    }
    map
}

fn get_visible_width(line: &str) -> usize {
    let re_ansi = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    let step1 = re_ansi.replace_all(line, "");

    let re_braces = Regex::new(r"\$\{[0-9]+\}").unwrap();
    let step2 = re_braces.replace_all(&step1, "");

    let re_simple = Regex::new(r"\$[0-9]").unwrap();
    let step3 = re_simple.replace_all(&step2, "");

    let re_reset = Regex::new(r"\$%(?:\{\})?").unwrap();
    let clean_line = re_reset.replace_all(&step3, "");

    clean_line.chars().count()
}

fn get_embedded_ascii(distro_name: &str) -> Option<&'static str> {
    match distro_name.to_lowercase().as_str() {
        "arch"        => Some(include_str!("../ascii/arch.txt")),
        "nixos"       => Some(include_str!("../ascii/nixos.txt")),
        "gentoo"      => Some(include_str!("../ascii/gentoo.txt")),
        "manjaro"     => Some(include_str!("../ascii/manjaro.txt")),
        "endeavouros" => Some(include_str!("../ascii/endeavouros.txt")),
        _             => None,
    }
}

fn render_split(distro_name: &str, info_lines: Vec<String>, cfg: &Config) {
    let mut current_logo_name = distro_name.to_lowercase();
    
    let ascii_content = match get_embedded_ascii(&current_logo_name) {
        Some(content) => content,
        None => {
            current_logo_name = "arch".to_string(); 
            get_embedded_ascii("arch").unwrap_or("")
        }
    };
    
    let ascii_lines: Vec<&str> = ascii_content.lines().collect();
    let max_lines = std::cmp::max(ascii_lines.len(), info_lines.len());
    
    let builtin_colors = parse_builtin_c();
    
    let logo_data = builtin_colors.get(&current_logo_name).cloned().unwrap_or_else(|| {
        FastfetchLogoData {
            colors: vec![
                "\x1b[36m".to_string(), 
                "\x1b[37m".to_string(), 
                "\x1b[34m".to_string(), 
                "\x1b[35m".to_string(), 
                "\x1b[32m".to_string(), 
            ]
        }
    });
    
    let custom_fallback = format!("\x1b[{}m", cfg.display.colors.primary);
    let fallback_color = logo_data.colors.first().cloned().unwrap_or(custom_fallback);

    let pad_width = cfg.display.ascii_padding_right;

    for i in 0..max_lines {
        if i < ascii_lines.len() {
            let orig_line = ascii_lines[i];
            let visible_len = get_visible_width(orig_line);
            
            let mut colored_line = orig_line.to_string();
            let mut patterns_exist = false;
            
            for idx in 1..10 {
                let p1 = format!("${}", idx);
                let p2 = format!("${{{}}}", idx);
                if colored_line.contains(&p1) || colored_line.contains(&p2) {
                    patterns_exist = true;
                    break;
                }
            }

            for idx in 0..15 {
                let placeholder_simple = format!("${}", idx + 1);
                let placeholder_braces = format!("${{{}}}", idx + 1);
                
                let color_code = if idx < logo_data.colors.len() {
                    &logo_data.colors[idx]
                } else {
                    "\x1b[37m" 
                };

                colored_line = colored_line.replace(&placeholder_simple, color_code);
                colored_line = colored_line.replace(&placeholder_braces, color_code);
            }
            
            colored_line = colored_line.replace("$%", "\x1b[0m");
            colored_line = colored_line.replace("$%{}", "\x1b[0m");

            if !patterns_exist {
                print!("{}{}\x1b[0m", fallback_color, colored_line);
            } else {
                print!("{}\x1b[0m", colored_line);
            }

            if visible_len < pad_width {
                print!("{}", " ".repeat(pad_width - visible_len));
            }
        } else {
            print!("{}", " ".repeat(pad_width));
        }

        if i < info_lines.len() {
            print!("{}", info_lines[i]);
        }
        println!();
    }
}
