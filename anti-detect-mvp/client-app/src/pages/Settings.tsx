import { useState } from 'react';

export default function Settings() {
  const [settings, setSettings] = useState({
    auto_launch: false,
    minimize_to_tray: true,
    check_updates: true,
    telemetry: false,
    language: 'en',
    theme: 'light',
  });

  const handleSave = () => {
    // Save settings to local storage or backend
    localStorage.setItem('app_settings', JSON.stringify(settings));
    alert('Settings saved successfully!');
  };

  return (
    <div className="space-y-6 max-w-3xl">
      <h1 className="text-3xl font-bold">Settings</h1>

      <div className="bg-white rounded-lg shadow-md p-6 space-y-6">
        <div>
          <h2 className="text-xl font-semibold mb-4">General Settings</h2>
          
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <h3 className="font-medium">Auto Launch on Startup</h3>
                <p className="text-sm text-gray-500">Start application when system boots</p>
              </div>
              <input
                type="checkbox"
                checked={settings.auto_launch}
                onChange={(e) => setSettings({ ...settings, auto_launch: e.target.checked })}
                className="w-5 h-5"
              />
            </div>

            <div className="flex items-center justify-between">
              <div>
                <h3 className="font-medium">Minimize to Tray</h3>
                <p className="text-sm text-gray-500">Keep app running in system tray</p>
              </div>
              <input
                type="checkbox"
                checked={settings.minimize_to_tray}
                onChange={(e) => setSettings({ ...settings, minimize_to_tray: e.target.checked })}
                className="w-5 h-5"
              />
            </div>

            <div className="flex items-center justify-between">
              <div>
                <h3 className="font-medium">Check for Updates</h3>
                <p className="text-sm text-gray-500">Automatically check for new versions</p>
              </div>
              <input
                type="checkbox"
                checked={settings.check_updates}
                onChange={(e) => setSettings({ ...settings, check_updates: e.target.checked })}
                className="w-5 h-5"
              />
            </div>
          </div>
        </div>

        <hr />

        <div>
          <h2 className="text-xl font-semibold mb-4">Privacy</h2>
          
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <h3 className="font-medium">Send Anonymous Usage Data</h3>
                <p className="text-sm text-gray-500">Help us improve by sending anonymous telemetry</p>
              </div>
              <input
                type="checkbox"
                checked={settings.telemetry}
                onChange={(e) => setSettings({ ...settings, telemetry: e.target.checked })}
                className="w-5 h-5"
              />
            </div>
          </div>
        </div>

        <hr />

        <div>
          <h2 className="text-xl font-semibold mb-4">Appearance</h2>
          
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Language
              </label>
              <select
                value={settings.language}
                onChange={(e) => setSettings({ ...settings, language: e.target.value })}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg"
              >
                <option value="en">English</option>
                <option value="fa">فارسی</option>
                <option value="ru">Русский</option>
                <option value="zh">中文</option>
              </select>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Theme
              </label>
              <select
                value={settings.theme}
                onChange={(e) => setSettings({ ...settings, theme: e.target.value })}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg"
              >
                <option value="light">Light</option>
                <option value="dark">Dark</option>
                <option value="auto">Auto (System)</option>
              </select>
            </div>
          </div>
        </div>

        <hr />

        <div>
          <h2 className="text-xl font-semibold mb-4">About</h2>
          
          <div className="space-y-2 text-sm text-gray-600">
            <p><strong>Version:</strong> 1.0.0</p>
            <p><strong>Build:</strong> 2024.11.22</p>
            <p><strong>License:</strong> Commercial</p>
            <p className="pt-2">
              <a href="#" className="text-indigo-600 hover:underline">View Changelog</a>
              {' • '}
              <a href="#" className="text-indigo-600 hover:underline">Report Issue</a>
              {' • '}
              <a href="#" className="text-indigo-600 hover:underline">Documentation</a>
            </p>
          </div>
        </div>

        <div className="flex gap-3 pt-4">
          <button
            onClick={handleSave}
            className="px-6 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700"
          >
            Save Settings
          </button>
          <button
            onClick={() => {
              setSettings({
                auto_launch: false,
                minimize_to_tray: true,
                check_updates: true,
                telemetry: false,
                language: 'en',
                theme: 'light',
              });
            }}
            className="px-6 py-2 bg-gray-300 text-gray-700 rounded-lg hover:bg-gray-400"
          >
            Reset to Defaults
          </button>
        </div>
      </div>
    </div>
  );
}
