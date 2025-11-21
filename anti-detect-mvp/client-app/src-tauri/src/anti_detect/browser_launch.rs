use super::FingerprintConfig;
use super::injection::generate_injection_script;
use std::process::{Command, Child};
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub protocol: String, // "http", "https", "socks5"
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserProfile {
    pub id: String,
    pub name: String,
    pub fingerprint: FingerprintConfig,
    pub proxy: Option<ProxyConfig>,
    pub user_data_dir: PathBuf,
}

impl BrowserProfile {
    /// Create a new browser profile
    pub fn new(id: String, name: String, fingerprint: FingerprintConfig, proxy: Option<ProxyConfig>) -> Self {
        let user_data_dir = Self::get_profile_data_dir(&id);
        
        Self {
            id,
            name,
            fingerprint,
            proxy,
            user_data_dir,
        }
    }
    
    /// Get the user data directory for a profile
    fn get_profile_data_dir(profile_id: &str) -> PathBuf {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("AntiDetect");
        path.push("Profiles");
        path.push(profile_id);
        path
    }
    
    /// Setup profile directory and injection scripts
    pub fn setup(&self) -> Result<(), String> {
        // Create profile directory
        fs::create_dir_all(&self.user_data_dir)
            .map_err(|e| format!("Failed to create profile directory: {}", e))?;
        
        // Create injection script
        let injection_script = generate_injection_script(&self.fingerprint);
        let script_path = self.user_data_dir.join("injection.js");
        
        let mut file = fs::File::create(&script_path)
            .map_err(|e| format!("Failed to create injection script: {}", e))?;
        
        file.write_all(injection_script.as_bytes())
            .map_err(|e| format!("Failed to write injection script: {}", e))?;
        
        Ok(())
    }
    
    /// Launch Chrome/Chromium with this profile
    pub fn launch(&self) -> Result<Child, String> {
        // Setup profile first
        self.setup()?;
        
        let chrome_path = self.find_chrome_executable()?;
        let mut args = vec![
            format!("--user-data-dir={}", self.user_data_dir.display()),
            "--no-first-run".to_string(),
            "--no-default-browser-check".to_string(),
            "--disable-background-networking".to_string(),
            "--disable-sync".to_string(),
            "--disable-translate".to_string(),
            "--disable-features=TranslateUI".to_string(),
            "--disable-component-update".to_string(),
        ];
        
        // Add fingerprint-specific flags
        args.push(format!("--user-agent={}", self.fingerprint.user_agent));
        
        // WebRTC leak protection
        if self.fingerprint.webrtc_leak_protection {
            args.push("--disable-webrtc".to_string());
            args.push("--enforce-webrtc-ip-permission-check".to_string());
        }
        
        // Add proxy if configured
        if let Some(proxy) = &self.proxy {
            let proxy_server = if let (Some(username), Some(password)) = (&proxy.username, &proxy.password) {
                format!("{}://{}:{}@{}:{}", 
                    proxy.protocol, username, password, proxy.host, proxy.port)
            } else {
                format!("{}://{}:{}", proxy.protocol, proxy.host, proxy.port)
            };
            args.push(format!("--proxy-server={}", proxy_server));
        }
        
        // Inject our script using an extension
        let extension_dir = self.create_injection_extension()?;
        args.push(format!("--load-extension={}", extension_dir.display()));
        
        // Launch Chrome
        Command::new(&chrome_path)
            .args(&args)
            .spawn()
            .map_err(|e| format!("Failed to launch browser: {}", e))
    }
    
    /// Create a temporary Chrome extension for script injection
    fn create_injection_extension(&self) -> Result<PathBuf, String> {
        let ext_dir = self.user_data_dir.join("extension");
        fs::create_dir_all(&ext_dir)
            .map_err(|e| format!("Failed to create extension directory: {}", e))?;
        
        // Create manifest.json
        let manifest = r#"{
  "manifest_version": 3,
  "name": "Anti-Detection",
  "version": "1.0.0",
  "permissions": [],
  "content_scripts": [
    {
      "matches": ["<all_urls>"],
      "js": ["content.js"],
      "run_at": "document_start",
      "all_frames": true
    }
  ]
}"#;
        
        let manifest_path = ext_dir.join("manifest.json");
        fs::write(&manifest_path, manifest)
            .map_err(|e| format!("Failed to write manifest: {}", e))?;
        
        // Create content.js
        let injection_script = generate_injection_script(&self.fingerprint);
        let content_script = format!(r#"
const script = document.createElement('script');
script.textContent = `{}`;
(document.head || document.documentElement).appendChild(script);
script.remove();
"#, 
            injection_script
                .replace('\\', "\\\\")
                .replace('`', "\\`")
                .replace('$', "\\$")
        );
        
        let content_path = ext_dir.join("content.js");
        fs::write(&content_path, content_script)
            .map_err(|e| format!("Failed to write content script: {}", e))?;
        
        Ok(ext_dir)
    }
    
    /// Find Chrome/Chromium executable
    fn find_chrome_executable(&self) -> Result<PathBuf, String> {
        #[cfg(target_os = "windows")]
        {
            let paths = vec![
                "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
                "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe",
                "C:\\Users\\{}\\AppData\\Local\\Google\\Chrome\\Application\\chrome.exe",
            ];
            
            for path in paths {
                let expanded = shellexpand::env(path).ok()
                    .map(|s| PathBuf::from(s.to_string()));
                if let Some(p) = expanded {
                    if p.exists() {
                        return Ok(p);
                    }
                }
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            let paths = vec![
                "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
                "/Applications/Chromium.app/Contents/MacOS/Chromium",
            ];
            
            for path in paths {
                let p = PathBuf::from(path);
                if p.exists() {
                    return Ok(p);
                }
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            let paths = vec![
                "/usr/bin/google-chrome",
                "/usr/bin/chromium",
                "/usr/bin/chromium-browser",
                "/snap/bin/chromium",
            ];
            
            for path in paths {
                let p = PathBuf::from(path);
                if p.exists() {
                    return Ok(p);
                }
            }
        }
        
        Err("Chrome/Chromium not found. Please install Google Chrome.".to_string())
    }
}

/// Browser instance manager
pub struct BrowserManager {
    active_browsers: Vec<(String, Child)>, // (profile_id, process)
}

impl BrowserManager {
    pub fn new() -> Self {
        Self {
            active_browsers: Vec::new(),
        }
    }
    
    /// Launch a browser with a profile
    pub fn launch_profile(&mut self, profile: &BrowserProfile) -> Result<(), String> {
        let process = profile.launch()?;
        self.active_browsers.push((profile.id.clone(), process));
        Ok(())
    }
    
    /// Stop a browser by profile ID
    pub fn stop_profile(&mut self, profile_id: &str) -> Result<(), String> {
        if let Some(index) = self.active_browsers.iter().position(|(id, _)| id == profile_id) {
            let (_, mut process) = self.active_browsers.remove(index);
            process.kill().map_err(|e| format!("Failed to kill browser: {}", e))?;
            Ok(())
        } else {
            Err("Profile not found in active browsers".to_string())
        }
    }
    
    /// Get list of active profile IDs
    pub fn get_active_profiles(&self) -> Vec<String> {
        self.active_browsers.iter().map(|(id, _)| id.clone()).collect()
    }
    
    /// Stop all browsers
    pub fn stop_all(&mut self) {
        for (_, mut process) in self.active_browsers.drain(..) {
            let _ = process.kill();
        }
    }
}

impl Drop for BrowserManager {
    fn drop(&mut self) {
        self.stop_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_profile() {
        let fingerprint = FingerprintConfig::default();
        let profile = BrowserProfile::new(
            "test-profile".to_string(),
            "Test Profile".to_string(),
            fingerprint,
            None
        );
        
        assert_eq!(profile.id, "test-profile");
        assert_eq!(profile.name, "Test Profile");
    }
}
