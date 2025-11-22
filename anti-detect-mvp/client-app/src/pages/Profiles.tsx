import { useState, useEffect } from 'react';
import { apiClient } from '../lib/api';

interface Profile {
  id: number;
  name: string;
  user_agent: string;
  fingerprint: any;
  proxy_id?: number;
}

interface Proxy {
  id: number;
  name: string;
  protocol: string;
  host: string;
  port: number;
}

export default function Profiles() {
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [proxies, setProxies] = useState<Proxy[]>([]);
  const [loading, setLoading] = useState(true);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [editingProfile, setEditingProfile] = useState<Profile | null>(null);
  const [formData, setFormData] = useState({
    name: '',
    user_agent: '',
    proxy_id: '',
    canvas_noise: true,
    webgl_noise: true,
    audio_noise: true,
  });

  useEffect(() => {
    loadProfiles();
    loadProxies();
  }, []);

  const loadProfiles = async () => {
    try {
      const data = await apiClient.get('/api/profiles');
      setProfiles(data);
    } catch (error) {
      console.error('Failed to load profiles:', error);
    } finally {
      setLoading(false);
    }
  };

  const loadProxies = async () => {
    try {
      const data = await apiClient.get('/api/proxies');
      setProxies(data);
    } catch (error) {
      console.error('Failed to load proxies:', error);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    try {
      const fingerprint = {
        canvas_noise: formData.canvas_noise,
        webgl_noise: formData.webgl_noise,
        audio_noise: formData.audio_noise,
        timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
        languages: navigator.languages,
        platform: navigator.platform,
      };

      const payload = {
        name: formData.name,
        user_agent: formData.user_agent,
        fingerprint,
        proxy_id: formData.proxy_id ? parseInt(formData.proxy_id) : null,
      };

      if (editingProfile) {
        await apiClient.put(`/api/profiles/${editingProfile.id}`, payload);
      } else {
        await apiClient.post('/api/profiles', payload);
      }

      setShowCreateModal(false);
      setEditingProfile(null);
      resetForm();
      loadProfiles();
    } catch (error) {
      console.error('Failed to save profile:', error);
      alert('Failed to save profile');
    }
  };

  const handleEdit = (profile: Profile) => {
    setEditingProfile(profile);
    setFormData({
      name: profile.name,
      user_agent: profile.user_agent,
      proxy_id: profile.proxy_id?.toString() || '',
      canvas_noise: profile.fingerprint?.canvas_noise ?? true,
      webgl_noise: profile.fingerprint?.webgl_noise ?? true,
      audio_noise: profile.fingerprint?.audio_noise ?? true,
    });
    setShowCreateModal(true);
  };

  const handleDelete = async (id: number) => {
    if (!confirm('Are you sure you want to delete this profile?')) return;

    try {
      await apiClient.delete(`/api/profiles/${id}`);
      loadProfiles();
    } catch (error) {
      console.error('Failed to delete profile:', error);
      alert('Failed to delete profile');
    }
  };

  const handleLaunch = async (profile: Profile) => {
    try {
      // Launch browser with this profile
      alert(`Launching browser with profile: ${profile.name}\n\nThis will open a browser with:\n- User Agent: ${profile.user_agent}\n- Anti-detection features enabled\n- Proxy: ${profile.proxy_id ? 'Configured' : 'None'}`);
      
      // TODO: Implement actual browser launching via Tauri IPC
      // await invoke('launch_browser', { profileId: profile.id });
    } catch (error) {
      console.error('Failed to launch browser:', error);
      alert('Failed to launch browser');
    }
  };

  const resetForm = () => {
    setFormData({
      name: '',
      user_agent: '',
      proxy_id: '',
      canvas_noise: true,
      webgl_noise: true,
      audio_noise: true,
    });
  };

  const userAgents = [
    'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
    'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
    'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
    'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0',
  ];

  if (loading) {
    return <div className="flex items-center justify-center h-64">Loading profiles...</div>;
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">Browser Profiles</h1>
        <button
          onClick={() => {
            setEditingProfile(null);
            resetForm();
            setShowCreateModal(true);
          }}
          className="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700"
        >
          + Create Profile
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {profiles.map((profile) => (
          <div key={profile.id} className="bg-white rounded-lg shadow-md p-6 border border-gray-200">
            <div className="flex justify-between items-start mb-4">
              <h3 className="text-xl font-semibold">{profile.name}</h3>
              <div className="flex gap-2">
                <button
                  onClick={() => handleLaunch(profile)}
                  className="px-3 py-1 bg-green-600 text-white text-sm rounded hover:bg-green-700"
                >
                  Launch
                </button>
                <button
                  onClick={() => handleEdit(profile)}
                  className="px-3 py-1 bg-blue-600 text-white text-sm rounded hover:bg-blue-700"
                >
                  Edit
                </button>
                <button
                  onClick={() => handleDelete(profile.id)}
                  className="px-3 py-1 bg-red-600 text-white text-sm rounded hover:bg-red-700"
                >
                  Delete
                </button>
              </div>
            </div>
            
            <div className="space-y-2 text-sm text-gray-600">
              <p><strong>User Agent:</strong></p>
              <p className="text-xs break-all">{profile.user_agent}</p>
              
              <p><strong>Proxy:</strong> {profile.proxy_id ? 'Configured' : 'None'}</p>
              
              {profile.fingerprint && (
                <div className="mt-3 pt-3 border-t border-gray-200">
                  <p><strong>Anti-Detection:</strong></p>
                  <div className="flex gap-2 mt-1">
                    {profile.fingerprint.canvas_noise && (
                      <span className="px-2 py-1 bg-green-100 text-green-800 text-xs rounded">Canvas</span>
                    )}
                    {profile.fingerprint.webgl_noise && (
                      <span className="px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded">WebGL</span>
                    )}
                    {profile.fingerprint.audio_noise && (
                      <span className="px-2 py-1 bg-purple-100 text-purple-800 text-xs rounded">Audio</span>
                    )}
                  </div>
                </div>
              )}
            </div>
          </div>
        ))}

        {profiles.length === 0 && (
          <div className="col-span-full text-center py-12 text-gray-500">
            No profiles yet. Create one to get started!
          </div>
        )}
      </div>

      {showCreateModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-8 max-w-2xl w-full max-h-[90vh] overflow-y-auto">
            <h2 className="text-2xl font-bold mb-6">
              {editingProfile ? 'Edit Profile' : 'Create New Profile'}
            </h2>

            <form onSubmit={handleSubmit} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Profile Name
                </label>
                <input
                  type="text"
                  required
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg"
                  placeholder="My Profile"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  User Agent
                </label>
                <select
                  required
                  value={formData.user_agent}
                  onChange={(e) => setFormData({ ...formData, user_agent: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg"
                >
                  <option value="">Select User Agent</option>
                  {userAgents.map((ua, idx) => (
                    <option key={idx} value={ua}>
                      {ua.includes('Windows') ? 'Windows Chrome' :
                       ua.includes('Macintosh') ? 'macOS Chrome' :
                       ua.includes('Linux') ? 'Linux Chrome' :
                       'Firefox'}
                    </option>
                  ))}
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Proxy (Optional)
                </label>
                <select
                  value={formData.proxy_id}
                  onChange={(e) => setFormData({ ...formData, proxy_id: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg"
                >
                  <option value="">No Proxy</option>
                  {proxies.map((proxy) => (
                    <option key={proxy.id} value={proxy.id}>
                      {proxy.name} ({proxy.protocol}://{proxy.host}:{proxy.port})
                    </option>
                  ))}
                </select>
              </div>

              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700">
                  Anti-Detection Features
                </label>
                
                <div className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    id="canvas_noise"
                    checked={formData.canvas_noise}
                    onChange={(e) => setFormData({ ...formData, canvas_noise: e.target.checked })}
                    className="w-4 h-4"
                  />
                  <label htmlFor="canvas_noise" className="text-sm">
                    Canvas Fingerprint Noise
                  </label>
                </div>

                <div className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    id="webgl_noise"
                    checked={formData.webgl_noise}
                    onChange={(e) => setFormData({ ...formData, webgl_noise: e.target.checked })}
                    className="w-4 h-4"
                  />
                  <label htmlFor="webgl_noise" className="text-sm">
                    WebGL Fingerprint Noise
                  </label>
                </div>

                <div className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    id="audio_noise"
                    checked={formData.audio_noise}
                    onChange={(e) => setFormData({ ...formData, audio_noise: e.target.checked })}
                    className="w-4 h-4"
                  />
                  <label htmlFor="audio_noise" className="text-sm">
                    Audio Context Noise
                  </label>
                </div>
              </div>

              <div className="flex gap-3 pt-4">
                <button
                  type="submit"
                  className="flex-1 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700"
                >
                  {editingProfile ? 'Update Profile' : 'Create Profile'}
                </button>
                <button
                  type="button"
                  onClick={() => {
                    setShowCreateModal(false);
                    setEditingProfile(null);
                    resetForm();
                  }}
                  className="flex-1 px-4 py-2 bg-gray-300 text-gray-700 rounded-lg hover:bg-gray-400"
                >
                  Cancel
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}
