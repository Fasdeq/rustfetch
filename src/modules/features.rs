use std::net::UdpSocket;
use std::fs;

pub struct FeaturesInfo {
    pub network: String,
    pub updates: String,
}

pub fn get_features() -> FeaturesInfo {
    let local_ip = UdpSocket::bind("0.0.0.0:0")
        .and_then(|socket| {
            socket.connect("1.1.1.1:80")?;
            socket.local_addr()
        })
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string());

    let mut pending_updates = "0".to_string();
    if let Ok(entries) = fs::read_dir("/var/lib/pacman/local") {
        let count = entries.flatten().filter(|e| {
            e.file_name().to_string_lossy().contains("sync")
        }).count();
        if count > 0 {
            pending_updates = count.to_string();
        }
    }

    FeaturesInfo { network: local_ip, updates: format!("{} updates available", pending_updates) }
}
