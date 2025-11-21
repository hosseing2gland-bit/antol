#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod anti_detect;

use anti_detect::{FingerprintConfig, BrowserProfile, BrowserManager, ProxyConfig};
use std::sync::Mutex;
use tauri::State;

/// Application state
struct AppState {
    browser_manager: Mutex<BrowserManager>,
}

/// Generate a new random fingerprint configuration
#[tauri::command]
fn generate_fingerprint() -> Result<FingerprintConfig, String> {
    Ok(FingerprintConfig::generate_random())
}

/// Launch a browser with a profile
#[tauri::command]
async fn launch_browser(
    profile_id: String,
    profile_name: String,
    fingerprint: FingerprintConfig,
    proxy: Option<ProxyConfig>,
    state: State<'_, AppState>
) -> Result<(), String> {
    let profile = BrowserProfile::new(profile_id, profile_name, fingerprint, proxy);
    
    let mut manager = state.browser_manager.lock()
        .map_err(|e| format!("Failed to lock browser manager: {}", e))?;
    
    manager.launch_profile(&profile)?;
    
    Ok(())
}

/// Stop a browser by profile ID
#[tauri::command]
async fn stop_browser(
    profile_id: String,
    state: State<'_, AppState>
) -> Result<(), String> {
    let mut manager = state.browser_manager.lock()
        .map_err(|e| format!("Failed to lock browser manager: {}", e))?;
    
    manager.stop_profile(&profile_id)?;
    
    Ok(())
}

/// Get list of active browser profiles
#[tauri::command]
async fn get_active_browsers(
    state: State<'_, AppState>
) -> Result<Vec<String>, String> {
    let manager = state.browser_manager.lock()
        .map_err(|e| format!("Failed to lock browser manager: {}", e))?;
    
    Ok(manager.get_active_profiles())
}

/// Stop all active browsers
#[tauri::command]
async fn stop_all_browsers(
    state: State<'_, AppState>
) -> Result<(), String> {
    let mut manager = state.browser_manager.lock()
        .map_err(|e| format!("Failed to lock browser manager: {}", e))?;
    
    manager.stop_all();
    
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            browser_manager: Mutex::new(BrowserManager::new()),
        })
        .invoke_handler(tauri::generate_handler![
            generate_fingerprint,
            launch_browser,
            stop_browser,
            get_active_browsers,
            stop_all_browsers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
