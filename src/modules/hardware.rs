use std::fs;
use std::path::Path;
use crate::config::Config;

pub struct HardwareInfo {
    pub cpu: String,
    pub gpu: String,
    pub memory: String,
    pub disk: String,
    pub battery: String,
}

pub fn get_hardware(cfg: &Config) -> HardwareInfo {
    let mut cpu_model = "Unknown CPU".to_string();
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if line.starts_with("model name") {
                cpu_model = line.split(':').nth(1).unwrap_or("").trim().to_string();
                break;
            }
        }
    }
    
    let mut cpu_temp = "N/A".to_string();
    if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(name) = fs::read_to_string(path.join("name")) {
                let n = name.trim();
                if n == "k10temp" || n == "coretemp" || n == "zenpower" {
                    if let Ok(temp) = fs::read_to_string(path.join("temp1_input")) {
                        if let Ok(t) = temp.trim().parse::<f64>() {
                            cpu_temp = format!("{:.1}°C", t / 1000.0);
                            break;
                        }
                    }
                }
            }
        }
    }
    let cpu = format!("{} ({})", cpu_model, cpu_temp);

    let mut gpu_model = "Unknown GPU".to_string();
    let mut vram_info = String::new();
    let mut driver_type = "Unknown Driver".to_string();
    
    if Path::new("/sys/module/nvidia").exists() {
        driver_type = "Proprietary (nvidia)".to_string();
        gpu_model = "NVIDIA GeForce Graphics".to_string();
        if let Ok(mem) = fs::read_to_string("/sys/class/drm/card0/device/mem_info_vram_total") {
            if let Ok(bytes) = mem.trim().parse::<u64>() {
                vram_info = format!(" [{} MiB]", bytes / 1024 / 1024);
            }
        }
    } else if Path::new("/sys/module/nouveau").exists() {
        driver_type = "Open-Source (nouveau)".to_string();
        gpu_model = "NVIDIA GPU".to_string();
    } else if Path::new("/sys/module/amdgpu").exists() {
        driver_type = "Open-Source (amdgpu)".to_string();
        gpu_model = "AMD Radeon GPU".to_string();
    }
    let gpu = format!("{} {}{}", gpu_model, driver_type, vram_info);

    let mut mem_total = 0;
    let mut mem_available = 0;
    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                mem_total = line.split_whitespace().nth(1).unwrap_or("0").parse::<u64>().unwrap_or(0);
            } else if line.starts_with("MemAvailable:") {
                mem_available = line.split_whitespace().nth(1).unwrap_or("0").parse::<u64>().unwrap_or(0);
                break;
            }
        }
    }
    let mem_used = mem_total - mem_available;
    let mem_used_gib = mem_used as f64 / 1024.0 / 1024.0;
    let mem_total_gib = mem_total as f64 / 1024.0 / 1024.0;
    let mem_pct = if mem_total > 0 { (mem_used * 100) / mem_total } else { 0 };
    
    let bars = (mem_pct / 10) as usize;
    let sym = &cfg.display.memory_bar_symbol;
    let empty_sym = &cfg.display.memory_bar_empty_symbol;
    let bar_str = format!("[{}{}]", sym.repeat(bars), empty_sym.repeat(10 - bars));
    let memory = format!("{:.2} GiB / {:.2} GiB ({}%) {}", mem_used_gib, mem_total_gib, mem_pct, bar_str);

    let mut disk_info = "Unknown".to_string();
    if let Ok(mounts) = fs::read_to_string("/proc/mounts") {
        for line in mounts.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 1 && parts[1] == "/" {
                let dev_path = parts[0];
                let disk_type = if dev_path.contains("nvme") { "NVMe SSD" } else { "SATA SSD/HDD" };
                let mut health = "Good".to_string();
                
                if dev_path.contains("nvme") {
                    if let Some(dev_name) = dev_path.split('/').last() {
                        if let Ok(life) = fs::read_to_string(format!("/sys/class/block/{}/device/percentage_used", dev_name)) {
                            health = format!("Wear: {}%", life.trim());
                        }
                    }
                }
                disk_info = format!("{} ({}) [{}]", dev_path, disk_type, health);
                break;
            }
        }
    }

    let mut battery = "Desktop / No Battery".to_string();
    if Path::new("/sys/class/power_supply/BAT0").exists() {
        let cap = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").unwrap_or_default().trim().to_string();
        let status = fs::read_to_string("/sys/class/power_supply/BAT0/status").unwrap_or_default().trim().to_string();
        let health = fs::read_to_string("/sys/class/power_supply/BAT0/capacity_level").unwrap_or_else(|_| "Good".to_string()).trim().to_string();
        battery = format!("{}% [{}] (Health: {})", cap, status, health);
    }

    HardwareInfo { cpu, gpu, memory, disk: disk_info, battery }
}
