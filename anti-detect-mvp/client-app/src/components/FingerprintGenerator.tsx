import { useState, useEffect } from 'react';
import { RefreshCw, Copy, Check } from 'lucide-react';
import { generateFingerprint, type FingerprintConfig } from '../lib/tauri';

export default function FingerprintGenerator() {
  const [fingerprint, setFingerprint] = useState<FingerprintConfig | null>(null);
  const [isGenerating, setIsGenerating] = useState(false);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    handleGenerate();
  }, []);

  const handleGenerate = async () => {
    setIsGenerating(true);
    try {
      const fp = await generateFingerprint();
      setFingerprint(fp);
    } catch (error) {
      console.error('Failed to generate fingerprint:', error);
      alert('Failed to generate fingerprint');
    } finally {
      setIsGenerating(false);
    }
  };

  const handleCopy = () => {
    if (fingerprint) {
      navigator.clipboard.writeText(JSON.stringify(fingerprint, null, 2));
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  if (!fingerprint) {
    return (
      <div className="flex items-center justify-center p-8">
        <div className="text-gray-400">Generating fingerprint...</div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h2 className="text-2xl font-bold text-white">Fingerprint Generator</h2>
        <div className="flex gap-2">
          <button 
            className="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded transition"
            onClick={handleCopy}
            disabled={!fingerprint}
          >
            {copied ? <Check size={18} className="inline mr-2" /> : <Copy size={18} className="inline mr-2" />}
            {copied ? 'Copied!' : 'Copy JSON'}
          </button>
          <button 
            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded transition"
            onClick={handleGenerate}
            disabled={isGenerating}
          >
            <RefreshCw size={18} className={`inline mr-2 ${isGenerating ? 'animate-spin' : ''}`} />
            Generate New
          </button>
        </div>
      </div>

      <div className="space-y-4">
        {/* Platform & Browser */}
        <div className="bg-gray-800 rounded-lg p-6">
          <h3 className="text-lg font-semibold mb-4 text-white">Platform & Browser</h3>
          <div className="grid grid-cols-2 gap-4">
            <div>
              <span className="text-gray-400 text-sm">Platform:</span>
              <div className="text-white">{fingerprint.platform}</div>
            </div>
            <div>
              <span className="text-gray-400 text-sm">Platform Version:</span>
              <div className="text-white">{fingerprint.platform_version}</div>
            </div>
            <div>
              <span className="text-gray-400 text-sm">Browser Version:</span>
              <div className="text-white">{fingerprint.browser_version}</div>
            </div>
            <div>
              <span className="text-gray-400 text-sm">Vendor:</span>
              <div className="text-white">{fingerprint.vendor}</div>
            </div>
            <div className="col-span-2">
              <span className="text-gray-400 text-sm">User Agent:</span>
              <div className="text-white font-mono text-xs break-all">{fingerprint.user_agent}</div>
            </div>
          </div>
        </div>

        {/* Hardware */}
        <div className="bg-gray-800 rounded-lg p-6">
          <h3 className="text-lg font-semibold mb-4 text-white">Hardware</h3>
          <div className="grid grid-cols-3 gap-4">
            <div>
              <span className="text-gray-400 text-sm">CPU Cores:</span>
              <div className="text-white">{fingerprint.hardware_concurrency}</div>
            </div>
            <div>
              <span className="text-gray-400 text-sm">Memory (GB):</span>
              <div className="text-white">{fingerprint.device_memory}</div>
            </div>
            <div>
              <span className="text-gray-400 text-sm">Max Touch Points:</span>
              <div className="text-white">{fingerprint.max_touch_points}</div>
            </div>
          </div>
        </div>

        {/* Screen */}
        <div className="bg-gray-800 rounded-lg p-6">
          <h3 className="text-lg font-semibold mb-4 text-white">Screen</h3>
          <div className="grid grid-cols-2 gap-4">
            <div>
              <span className="text-gray-400 text-sm">Resolution:</span>
              <div className="text-white">{fingerprint.screen_width} × {fingerprint.screen_height}</div>
            </div>
            <div>
              <span className="text-gray-400 text-sm">Available:</span>
              <div className="text-white">{fingerprint.available_width} × {fingerprint.available_height}</div>
            </div>
            <div>
              <span className="text-gray-400 text-sm">Color Depth:</span>
              <div className="text-white">{fingerprint.color_depth} bit</div>
            </div>
            <div>
              <span className="text-gray-400 text-sm">Pixel Depth:</span>
              <div className="text-white">{fingerprint.pixel_depth} bit</div>
            </div>
          </div>
        </div>

        {/* Geolocation */}
        <div className="bg-gray-800 rounded-lg p-6">
          <h3 className="text-lg font-semibold mb-4 text-white">Geolocation</h3>
          <div className="grid grid-cols-2 gap-4">
            <div>
              <span className="text-gray-400 text-sm">Timezone:</span>
              <div className="text-white">{fingerprint.timezone}</div>
            </div>
            <div>
              <span className="text-gray-400 text-sm">Language:</span>
              <div className="text-white">{fingerprint.language}</div>
            </div>
          </div>
        </div>

        {/* Protection Features */}
        <div className="bg-gray-800 rounded-lg p-6">
          <h3 className="text-lg font-semibold mb-4 text-white">Protection Features</h3>
          <div className="grid grid-cols-2 gap-3">
            <div className="flex items-center space-x-2">
              <div className={`w-3 h-3 rounded-full ${fingerprint.canvas_noise ? 'bg-green-500' : 'bg-red-500'}`}></div>
              <span className="text-white text-sm">Canvas Noise</span>
            </div>
            <div className="flex items-center space-x-2">
              <div className={`w-3 h-3 rounded-full ${fingerprint.webgl_noise ? 'bg-green-500' : 'bg-red-500'}`}></div>
              <span className="text-white text-sm">WebGL Noise</span>
            </div>
            <div className="flex items-center space-x-2">
              <div className={`w-3 h-3 rounded-full ${fingerprint.audio_noise ? 'bg-green-500' : 'bg-red-500'}`}></div>
              <span className="text-white text-sm">Audio Noise</span>
            </div>
            <div className="flex items-center space-x-2">
              <div className={`w-3 h-3 rounded-full ${fingerprint.webrtc_leak_protection ? 'bg-green-500' : 'bg-red-500'}`}></div>
              <span className="text-white text-sm">WebRTC Protection</span>
            </div>
            <div className="flex items-center space-x-2">
              <div className={`w-3 h-3 rounded-full ${fingerprint.fake_media_devices ? 'bg-green-500' : 'bg-red-500'}`}></div>
              <span className="text-white text-sm">Fake Media Devices</span>
            </div>
            <div className="flex items-center space-x-2">
              <div className={`w-3 h-3 rounded-full ${fingerprint.battery_spoofing ? 'bg-green-500' : 'bg-red-500'}`}></div>
              <span className="text-white text-sm">Battery Spoofing</span>
            </div>
          </div>
        </div>

        {/* Fingerprint ID */}
        <div className="bg-gray-800 rounded-lg p-6">
          <h3 className="text-lg font-semibold mb-4 text-white">Fingerprint ID</h3>
          <code className="text-sm text-gray-400 font-mono">{fingerprint.fingerprint_seed}</code>
        </div>
      </div>
    </div>
  );
}
