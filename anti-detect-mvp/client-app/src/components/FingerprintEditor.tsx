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
  media_devices: any[];
  client_rects_noise: boolean;
  battery_spoofing: boolean;
  battery_level: number | null;
  battery_charging: boolean | null;
  do_not_track: string | null;
  fingerprint_seed: string;
}

interface Props {
  fingerprint?: FingerprintConfig;
  onSave?: (fingerprint: FingerprintConfig) => void;
  onCancel?: () => void;
}

export default function FingerprintEditor({ fingerprint, onSave, onCancel }: Props) {
  const [config, setConfig] = useState<FingerprintConfig | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (fingerprint) {
      setConfig(fingerprint);
    } else {
      generateRandom();
    }
  }, [fingerprint]);

  const generateRandom = async () => {
    setLoading(true);
    try {
      const newConfig = await invoke<FingerprintConfig>('generate_fingerprint');
      setConfig(newConfig);
    } catch (err) {
      console.error('Failed to generate fingerprint:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleSave = () => {
    if (config && onSave) {
      onSave(config);
    }
  };

  if (loading || !config) {
    return (
      <div className="flex items-center justify-center p-8">
        <div className="text-gray-400">Loading fingerprint configuration...</div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Platform & Browser */}
      <div className="bg-gray-800 rounded-lg p-6">
        <h3 className="text-lg font-semibold mb-4 text-white">Platform & Browser</h3>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm text-gray-400 mb-2">Platform</label>
            <input
              type="text"
              value={config.platform}
              onChange={(e) => setConfig({ ...config, platform: e.target.value })}
              className="w-full bg-gray-700 text-white px-3 py-2 rounded border border-gray-600 focus:border-blue-500 outline-none"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-400 mb-2">Platform Version</label>
            <input
              type="text"
              value={config.platform_version}
              onChange={(e) => setConfig({ ...config, platform_version: e.target.value })}
              className="w-full bg-gray-700 text-white px-3 py-2 rounded border border-gray-600 focus:border-blue-500 outline-none"
            />
          </div>
          <div className="col-span-2">
            <label className="block text-sm text-gray-400 mb-2">User Agent</label>
            <textarea
              value={config.user_agent}
              onChange={(e) => setConfig({ ...config, user_agent: e.target.value })}
              className="w-full bg-gray-700 text-white px-3 py-2 rounded border border-gray-600 focus:border-blue-500 outline-none"
              rows={2}
            />
          </div>
        </div>
      </div>

      {/* Hardware */}
      <div className="bg-gray-800 rounded-lg p-6">
        <h3 className="text-lg font-semibold mb-4 text-white">Hardware</h3>
        <div className="grid grid-cols-3 gap-4">
          <div>
            <label className="block text-sm text-gray-400 mb-2">CPU Cores</label>
            <input
              type="number"
              value={config.hardware_concurrency}
              onChange={(e) => setConfig({ ...config, hardware_concurrency: parseInt(e.target.value) })}
              className="w-full bg-gray-700 text-white px-3 py-2 rounded border border-gray-600 focus:border-blue-500 outline-none"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-400 mb-2">Memory (GB)</label>
            <input
              type="number"
              value={config.device_memory}
              onChange={(e) => setConfig({ ...config, device_memory: parseInt(e.target.value) })}
              className="w-full bg-gray-700 text-white px-3 py-2 rounded border border-gray-600 focus:border-blue-500 outline-none"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-400 mb-2">Touch Points</label>
            <input
              type="number"
              value={config.max_touch_points}
              onChange={(e) => setConfig({ ...config, max_touch_points: parseInt(e.target.value) })}
              className="w-full bg-gray-700 text-white px-3 py-2 rounded border border-gray-600 focus:border-blue-500 outline-none"
            />
          </div>
        </div>
      </div>

      {/* Screen */}
      <div className="bg-gray-800 rounded-lg p-6">
        <h3 className="text-lg font-semibold mb-4 text-white">Screen</h3>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm text-gray-400 mb-2">Width</label>
            <input
              type="number"
              value={config.screen_width}
              onChange={(e) => setConfig({ ...config, screen_width: parseInt(e.target.value) })}
              className="w-full bg-gray-700 text-white px-3 py-2 rounded border border-gray-600 focus:border-blue-500 outline-none"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-400 mb-2">Height</label>
            <input
              type="number"
              value={config.screen_height}
              onChange={(e) => setConfig({ ...config, screen_height: parseInt(e.target.value) })}
              className="w-full bg-gray-700 text-white px-3 py-2 rounded border border-gray-600 focus:border-blue-500 outline-none"
            />
          </div>
        </div>
      </div>

      {/* Geolocation */}
      <div className="bg-gray-800 rounded-lg p-6">
        <h3 className="text-lg font-semibold mb-4 text-white">Geolocation</h3>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm text-gray-400 mb-2">Timezone</label>
            <input
              type="text"
              value={config.timezone}
              onChange={(e) => setConfig({ ...config, timezone: e.target.value })}
              className="w-full bg-gray-700 text-white px-3 py-2 rounded border border-gray-600 focus:border-blue-500 outline-none"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-400 mb-2">Language</label>
            <input
              type="text"
              value={config.language}
              onChange={(e) => setConfig({ ...config, language: e.target.value })}
              className="w-full bg-gray-700 text-white px-3 py-2 rounded border border-gray-600 focus:border-blue-500 outline-none"
            />
          </div>
        </div>
      </div>

      {/* WebGL */}
      <div className="bg-gray-800 rounded-lg p-6">
        <h3 className="text-lg font-semibold mb-4 text-white">WebGL</h3>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm text-gray-400 mb-2">Vendor</label>
            <input
              type="text"
              value={config.webgl_vendor}
              onChange={(e) => setConfig({ ...config, webgl_vendor: e.target.value })}
              className="w-full bg-gray-700 text-white px-3 py-2 rounded border border-gray-600 focus:border-blue-500 outline-none"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-400 mb-2">Renderer</label>
            <input
              type="text"
              value={config.webgl_renderer}
              onChange={(e) => setConfig({ ...config, webgl_renderer: e.target.value })}
              className="w-full bg-gray-700 text-white px-3 py-2 rounded border border-gray-600 focus:border-blue-500 outline-none"
            />
          </div>
        </div>
      </div>

      {/* Protection Features */}
      <div className="bg-gray-800 rounded-lg p-6">
        <h3 className="text-lg font-semibold mb-4 text-white">Protection Features</h3>
        <div className="space-y-3">
          <label className="flex items-center space-x-3">
            <input
              type="checkbox"
              checked={config.canvas_noise}
              onChange={(e) => setConfig({ ...config, canvas_noise: e.target.checked })}
              className="w-4 h-4"
            />
            <span className="text-white">Canvas Fingerprint Noise</span>
          </label>
          <label className="flex items-center space-x-3">
            <input
              type="checkbox"
              checked={config.webgl_noise}
              onChange={(e) => setConfig({ ...config, webgl_noise: e.target.checked })}
              className="w-4 h-4"
            />
            <span className="text-white">WebGL Fingerprint Noise</span>
          </label>
          <label className="flex items-center space-x-3">
            <input
              type="checkbox"
              checked={config.audio_noise}
              onChange={(e) => setConfig({ ...config, audio_noise: e.target.checked })}
              className="w-4 h-4"
            />
            <span className="text-white">Audio Context Noise</span>
          </label>
          <label className="flex items-center space-x-3">
            <input
              type="checkbox"
              checked={config.webrtc_leak_protection}
              onChange={(e) => setConfig({ ...config, webrtc_leak_protection: e.target.checked })}
              className="w-4 h-4"
            />
            <span className="text-white">WebRTC Leak Protection</span>
          </label>
          <label className="flex items-center space-x-3">
            <input
              type="checkbox"
              checked={config.fake_media_devices}
              onChange={(e) => setConfig({ ...config, fake_media_devices: e.target.checked })}
              className="w-4 h-4"
            />
            <span className="text-white">Fake Media Devices</span>
          </label>
          <label className="flex items-center space-x-3">
            <input
              type="checkbox"
              checked={config.client_rects_noise}
              onChange={(e) => setConfig({ ...config, client_rects_noise: e.target.checked })}
              className="w-4 h-4"
            />
            <span className="text-white">Client Rects Noise</span>
          </label>
          <label className="flex items-center space-x-3">
            <input
              type="checkbox"
              checked={config.battery_spoofing}
              onChange={(e) => setConfig({ ...config, battery_spoofing: e.target.checked })}
              className="w-4 h-4"
            />
            <span className="text-white">Battery API Spoofing</span>
          </label>
        </div>
      </div>

      {/* Fingerprint Seed */}
      <div className="bg-gray-800 rounded-lg p-6">
        <h3 className="text-lg font-semibold mb-4 text-white">Fingerprint Seed</h3>
        <div className="flex items-center space-x-2">
          <input
            type="text"
            value={config.fingerprint_seed}
            readOnly
            className="flex-1 bg-gray-700 text-gray-400 px-3 py-2 rounded border border-gray-600 outline-none font-mono text-sm"
          />
          <button
            onClick={generateRandom}
            className="px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded transition"
          >
            🔄 Regenerate
          </button>
        </div>
      </div>

      {/* Actions */}
      <div className="flex justify-end space-x-3">
        {onCancel && (
          <button
            onClick={onCancel}
            className="px-6 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded transition"
          >
            Cancel
          </button>
        )}
        <button
          onClick={handleSave}
          className="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded transition"
        >
          Save Fingerprint
        </button>
      </div>
    </div>
  );
}
