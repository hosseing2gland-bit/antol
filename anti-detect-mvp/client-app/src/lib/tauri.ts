import { invoke } from '@tauri-apps/api/tauri';

// ============= Types =============

export interface FingerprintConfig {
  platform: string;
  platform_version: string;
  user_agent: string;
  browser_version: string;
  vendor: string;
  hardware_concurrency: number;
  device_memory: number;
  max_touch_points: number;
  screen_width: number;
  screen_height: number;
  available_width: number;
  available_height: number;
  color_depth: number;
  pixel_depth: number;
  timezone: string;
  timezone_offset: number;
  language: string;
  languages: string[];
  canvas_noise: boolean;
  canvas_noise_level: number;
  webgl_vendor: string;
  webgl_renderer: string;
  webgl_noise: boolean;
  webgl_noise_level: number;
  audio_noise: boolean;
  audio_noise_level: number;
  fonts: string[];
  webrtc_leak_protection: boolean;
  webrtc_local_ip_hiding: boolean;
  fake_media_devices: boolean;
  media_devices: MediaDevice[];
  client_rects_noise: boolean;
  battery_spoofing: boolean;
  battery_level: number | null;
  battery_charging: boolean | null;
  do_not_track: string | null;
  fingerprint_seed: string;
}

export interface MediaDevice {
  device_id: string;
  kind: string;
  label: string;
  group_id: string;
}

export interface ProxyConfig {
  protocol: string;
  host: string;
  port: number;
  username?: string;
  password?: string;
}

// ============= Tauri Commands =============

/**
 * Generate a new random fingerprint configuration
 */
export async function generateFingerprint(): Promise<FingerprintConfig> {
  return await invoke<FingerprintConfig>('generate_fingerprint');
}

/**
 * Launch a browser with a profile
 */
export async function launchBrowser(
  profileId: string,
  profileName: string,
  fingerprint: FingerprintConfig,
  proxy?: ProxyConfig
): Promise<void> {
  await invoke('launch_browser', {
    profileId,
    profileName,
    fingerprint,
    proxy: proxy || null,
  });
}

/**
 * Stop a browser by profile ID
 */
export async function stopBrowser(profileId: string): Promise<void> {
  await invoke('stop_browser', {
    profileId,
  });
}

/**
 * Get list of active browser profiles
 */
export async function getActiveBrowsers(): Promise<string[]> {
  return await invoke<string[]>('get_active_browsers');
}

/**
 * Stop all active browsers
 */
export async function stopAllBrowsers(): Promise<void> {
  await invoke('stop_all_browsers');
}
