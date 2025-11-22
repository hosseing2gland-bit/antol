use serde::{Deserialize, Serialize};
use rand::Rng;
use uuid::Uuid;

/// Complete fingerprint configuration for a browser profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerprintConfig {
    // Platform & Browser
    pub platform: String,
    pub platform_version: String,
    pub user_agent: String,
    pub browser_version: String,
    pub vendor: String,
    
    // Hardware
    pub hardware_concurrency: u8,
    pub device_memory: u8,
    pub max_touch_points: u8,
    
    // Screen
    pub screen_width: u32,
    pub screen_height: u32,
    pub available_width: u32,
    pub available_height: u32,
    pub color_depth: u8,
    pub pixel_depth: u8,
    
    // Geolocation
    pub timezone: String,
    pub timezone_offset: i32,
    pub language: String,
    pub languages: Vec<String>,
    
    // Canvas Fingerprint
    pub canvas_noise: bool,
    pub canvas_noise_level: f64,
    
    // WebGL Fingerprint
    pub webgl_vendor: String,
    pub webgl_renderer: String,
    pub webgl_noise: bool,
    pub webgl_noise_level: f64,
    
    // Audio Context
    pub audio_noise: bool,
    pub audio_noise_level: f64,
    
    // Fonts
    pub fonts: Vec<String>,
    
    // WebRTC
    pub webrtc_leak_protection: bool,
    pub webrtc_local_ip_hiding: bool,
    
    // Media Devices
    pub fake_media_devices: bool,
    pub media_devices: Vec<MediaDevice>,
    
    // Client Rects
    pub client_rects_noise: bool,
    
    // Battery API
    pub battery_spoofing: bool,
    pub battery_level: Option<f64>,
    pub battery_charging: Option<bool>,
    
    // Do Not Track
    pub do_not_track: Option<String>,
    
    // Other
    pub fingerprint_seed: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaDevice {
    pub device_id: String,
    pub kind: String,
    pub label: String,
    pub group_id: String,
}

impl Default for FingerprintConfig {
    fn default() -> Self {
        Self::generate_random()
    }
}

impl FingerprintConfig {
    /// Generate a random but realistic fingerprint configuration
    pub fn generate_random() -> Self {
        let mut rng = rand::thread_rng();
        let seed = Uuid::new_v4().to_string();
        
        // Random platform selection
        let platforms = vec![
            ("Win32", "10.0", "Windows"),
            ("MacIntel", "14.0", "macOS"),
            ("Linux x86_64", "5.15", "Linux"),
        ];
        let (platform, platform_version, os_name) = platforms[rng.gen_range(0..platforms.len())];
        
        // Browser version (Chrome 120-131)
        let browser_version = format!("{}.0.{}.{}", 
            rng.gen_range(120..132),
            rng.gen_range(6000..7000),
            rng.gen_range(10..100)
        );
        
        // Hardware concurrency (2, 4, 6, 8, 12, 16)
        let hardware_concurrency = *[2, 4, 6, 8, 12, 16].iter()
            .nth(rng.gen_range(0..6))
            .unwrap();
        
        // Device memory (2, 4, 8, 16, 32)
        let device_memory = *[2, 4, 8, 16, 32].iter()
            .nth(rng.gen_range(0..5))
            .unwrap();
        
        // Screen resolutions
        let resolutions = vec![
            (1920, 1080),
            (1366, 768),
            (1536, 864),
            (1440, 900),
            (2560, 1440),
            (1600, 900),
            (1280, 720),
        ];
        let (screen_width, screen_height) = resolutions[rng.gen_range(0..resolutions.len())];
        
        // Timezones
        let timezones = vec![
            "America/New_York",
            "America/Los_Angeles",
            "America/Chicago",
            "Europe/London",
            "Europe/Paris",
            "Asia/Tokyo",
            "Asia/Shanghai",
            "Australia/Sydney",
        ];
        let timezone = timezones[rng.gen_range(0..timezones.len())].to_string();
        
        // Languages
        let languages = vec![
            vec!["en-US", "en"],
            vec!["en-GB", "en"],
            vec!["es-ES", "es"],
            vec!["fr-FR", "fr"],
            vec!["de-DE", "de"],
            vec!["ja-JP", "ja"],
            vec!["zh-CN", "zh"],
        ];
        let language_set = &languages[rng.gen_range(0..languages.len())];
        
        // WebGL vendors and renderers
        let webgl_configs = vec![
            ("Google Inc. (NVIDIA)", "ANGLE (NVIDIA GeForce GTX 1060 Direct3D11 vs_5_0 ps_5_0)"),
            ("Google Inc. (Intel)", "ANGLE (Intel(R) UHD Graphics 630 Direct3D11 vs_5_0 ps_5_0)"),
            ("Google Inc. (AMD)", "ANGLE (AMD Radeon RX 580 Series Direct3D11 vs_5_0 ps_5_0)"),
            ("Intel Inc.", "Intel Iris OpenGL Engine"),
            ("Apple Inc.", "Apple M1 GPU"),
        ];
        let (webgl_vendor, webgl_renderer) = webgl_configs[rng.gen_range(0..webgl_configs.len())];
        
        // Common fonts
        let fonts = vec![
            "Arial", "Verdana", "Helvetica", "Times New Roman", "Courier New",
            "Georgia", "Palatino", "Garamond", "Bookman", "Comic Sans MS",
            "Trebuchet MS", "Arial Black", "Impact", "Tahoma", "Calibri",
        ].iter().map(|&s| s.to_string()).collect();
        
        // Generate fake media devices
        let media_devices = Self::generate_media_devices(&seed);
        
        Self {
            platform: platform.to_string(),
            platform_version: platform_version.to_string(),
            user_agent: Self::generate_user_agent(&browser_version, os_name),
            browser_version,
            vendor: "Google Inc.".to_string(),
            
            hardware_concurrency,
            device_memory,
            max_touch_points: if platform == "Win32" { 10 } else { 0 },
            
            screen_width,
            screen_height,
            available_width: screen_width,
            available_height: screen_height - if platform == "Win32" { 40 } else { 25 },
            color_depth: 24,
            pixel_depth: 24,
            
            timezone: timezone.clone(),
            timezone_offset: Self::calculate_timezone_offset(&timezone),
            language: language_set[0].to_string(),
            languages: language_set.iter().map(|&s| s.to_string()).collect(),
            
            canvas_noise: true,
            canvas_noise_level: rng.gen_range(0.001..0.01),
            
            webgl_vendor: webgl_vendor.to_string(),
            webgl_renderer: webgl_renderer.to_string(),
            webgl_noise: true,
            webgl_noise_level: rng.gen_range(0.001..0.01),
            
            audio_noise: true,
            audio_noise_level: rng.gen_range(0.0001..0.001),
            
            fonts,
            
            webrtc_leak_protection: true,
            webrtc_local_ip_hiding: true,
            
            fake_media_devices: true,
            media_devices,
            
            client_rects_noise: true,
            
            battery_spoofing: true,
            battery_level: Some(rng.gen_range(0.3..1.0)),
            battery_charging: Some(rng.gen_bool(0.5)),
            
            do_not_track: Some("1".to_string()),
            
            fingerprint_seed: seed,
        }
    }
    
    fn generate_user_agent(browser_version: &str, os_name: &str) -> String {
        match os_name {
            "Windows" => format!(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36",
                browser_version
            ),
            "macOS" => format!(
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36",
                browser_version
            ),
            "Linux" => format!(
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36",
                browser_version
            ),
            _ => format!(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36",
                browser_version
            ),
        }
    }
    
    fn calculate_timezone_offset(timezone: &str) -> i32 {
        // Simplified timezone offset calculation
        match timezone {
            "America/New_York" => -300,
            "America/Los_Angeles" => -480,
            "America/Chicago" => -360,
            "Europe/London" => 0,
            "Europe/Paris" => 60,
            "Asia/Tokyo" => 540,
            "Asia/Shanghai" => 480,
            "Australia/Sydney" => 600,
            _ => 0,
        }
    }
    
    fn generate_media_devices(seed: &str) -> Vec<MediaDevice> {
        // Generate consistent but random-looking device IDs based on seed
        let hash = format!("{:x}", md5::compute(seed));
        
        vec![
            MediaDevice {
                device_id: format!("default"),
                kind: "audioinput".to_string(),
                label: "Default - Microphone (Built-in)".to_string(),
                group_id: hash[..16].to_string(),
            },
            MediaDevice {
                device_id: format!("communications"),
                kind: "audioinput".to_string(),
                label: "Communications - Microphone (Built-in)".to_string(),
                group_id: hash[..16].to_string(),
            },
            MediaDevice {
                device_id: format!("default"),
                kind: "audiooutput".to_string(),
                label: "Default - Speakers (Built-in)".to_string(),
                group_id: hash[16..32].to_string(),
            },
            MediaDevice {
                device_id: format!("{}", &hash[..32]),
                kind: "videoinput".to_string(),
                label: "HD Webcam (Built-in)".to_string(),
                group_id: hash[..16].to_string(),
            },
        ]
    }
    
    /// Convert fingerprint config to JSON for injection into browser
    pub fn to_injection_script(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_random_fingerprint() {
        let fp1 = FingerprintConfig::generate_random();
        let fp2 = FingerprintConfig::generate_random();
        
        // Each fingerprint should be unique
        assert_ne!(fp1.fingerprint_seed, fp2.fingerprint_seed);
        
        // Validate ranges
        assert!(fp1.hardware_concurrency >= 2 && fp1.hardware_concurrency <= 16);
        assert!(fp1.device_memory >= 2 && fp1.device_memory <= 32);
        assert!(fp1.screen_width > 0);
        assert!(fp1.screen_height > 0);
    }
}
