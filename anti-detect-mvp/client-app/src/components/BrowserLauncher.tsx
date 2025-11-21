import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface FingerprintConfig {
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

interface MediaDevice {
  device_id: string;
  kind: string;
  label: string;
  group_id: string;
}

interface ProxyConfig {
  protocol: string;
  host: string;
  port: number;
  username?: string;
  password?: string;
}

interface Profile {
  id: string;
  name: string;
  fingerprint?: FingerprintConfig;
  proxy?: ProxyConfig;
}

export default function BrowserLauncher() {
  const [activeBrowsers, setActiveBrowsers] = useState<string[]>([]);
  const [loading, setLoading] = useState<{ [key: string]: boolean }>({});
  const [error, setError] = useState<string>('');

  useEffect(() => {
    loadActiveBrowsers();
    const interval = setInterval(loadActiveBrowsers, 5000);
    return () => clearInterval(interval);
  }, []);

  const loadActiveBrowsers = async () => {
    try {
      const browsers = await invoke<string[]>('get_active_browsers');
      setActiveBrowsers(browsers);
    } catch (err) {
      console.error('Failed to load active browsers:', err);
    }
  };

  const launchBrowser = async (profile: Profile) => {
    setLoading({ ...loading, [profile.id]: true });
    setError('');

    try {
      // Generate fingerprint if not exists
      let fingerprint = profile.fingerprint;
      if (!fingerprint) {
        fingerprint = await invoke<FingerprintConfig>('generate_fingerprint');
      }

      await invoke('launch_browser', {
        profileId: profile.id,
        profileName: profile.name,
        fingerprint,
        proxy: profile.proxy || null,
      });

      await loadActiveBrowsers();
    } catch (err: any) {
      setError(err.toString());
      console.error('Failed to launch browser:', err);
    } finally {
      setLoading({ ...loading, [profile.id]: false });
    }
  };

  const stopBrowser = async (profileId: string) => {
    setLoading({ ...loading, [profileId]: true });
    setError('');

    try {
      await invoke('stop_browser', { profileId });
      await loadActiveBrowsers();
    } catch (err: any) {
      setError(err.toString());
      console.error('Failed to stop browser:', err);
    } finally {
      setLoading({ ...loading, [profileId]: false });
    }
  };

  const stopAllBrowsers = async () => {
    setError('');
    try {
      await invoke('stop_all_browsers');
      await loadActiveBrowsers();
    } catch (err: any) {
      setError(err.toString());
      console.error('Failed to stop all browsers:', err);
    }
  };

  const isActive = (profileId: string) => activeBrowsers.includes(profileId);

  return {
    activeBrowsers,
    loading,
    error,
    launchBrowser,
    stopBrowser,
    stopAllBrowsers,
    isActive,
  };
}
