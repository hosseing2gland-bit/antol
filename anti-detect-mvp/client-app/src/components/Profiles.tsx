import { useEffect, useState } from 'react';
import { Plus, Play, Trash2, UserCircle, Square } from 'lucide-react';
import { useProfilesStore, useProxiesStore } from '../store';
import { generateFingerprint, launchBrowser, stopBrowser, getActiveBrowsers, type FingerprintConfig } from '../lib/tauri';

export default function Profiles() {
  const { profiles, fetchProfiles, createProfile, deleteProfile } = useProfilesStore();
  const { proxies, fetchProxies } = useProxiesStore();
  const [showModal, setShowModal] = useState(false);
  const [activeBrowsers, setActiveBrowsers] = useState<string[]>([]);
  const [formData, setFormData] = useState({
    name: '',
    user_agent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
    locale: 'en-US',
    timezone: 'America/New_York',
    webgl_vendor: 'Intel Inc.',
    webgl_renderer: 'Intel Iris OpenGL Engine',
    canvas_noise: true,
    audio_noise: true,
    webgl_noise: true,
    proxy_id: '',
  });

  useEffect(() => {
    fetchProfiles();
    fetchProxies();
    loadActiveBrowsers();
    
    // Poll active browsers every 5 seconds
    const interval = setInterval(loadActiveBrowsers, 5000);
    return () => clearInterval(interval);
  }, []);
  
  const loadActiveBrowsers = async () => {
    try {
      const active = await getActiveBrowsers();
      setActiveBrowsers(active);
    } catch (error) {
      console.error('Failed to load active browsers:', error);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await createProfile({
        ...formData,
        user_id: 'current-user-id', // Get from auth store
      });
      setShowModal(false);
      resetForm();
      fetchProfiles();
    } catch (error) {
      console.error('Failed to create profile:', error);
    }
  };

  const resetForm = () => {
    setFormData({
      name: '',
      user_agent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
      locale: 'en-US',
      timezone: 'America/New_York',
      webgl_vendor: 'Intel Inc.',
      webgl_renderer: 'Intel Iris OpenGL Engine',
      canvas_noise: true,
      audio_noise: true,
      webgl_noise: true,
      proxy_id: '',
    });
  };

  const handleDelete = async (id: string) => {
    if (confirm('Are you sure you want to delete this profile?')) {
      await deleteProfile(id);
      fetchProfiles();
    }
  };

  const handleLaunch = async (profile: any) => {
    try {
      // Get fingerprint from profile or generate new one
      let fp: FingerprintConfig;
      
      if (profile.fingerprint && typeof profile.fingerprint === 'object') {
        fp = profile.fingerprint as FingerprintConfig;
      } else {
        fp = await generateFingerprint();
      }
      
      // Get proxy if configured
      const proxy = profile.proxy_id 
        ? proxies.find(p => p.id === profile.proxy_id)
        : null;
      
      const proxyConfig = proxy ? {
        protocol: proxy.proxy_type,
        host: proxy.host,
        port: proxy.port,
        username: proxy.username || undefined,
        password: proxy.password || undefined,
      } : undefined;
      
      await launchBrowser(profile.id, profile.name, fp, proxyConfig);
      
      // Reload active browsers
      await loadActiveBrowsers();
    } catch (error) {
      console.error('Failed to launch browser:', error);
      alert('Failed to launch browser: ' + error);
    }
  };
  
  const handleStop = async (profileId: string) => {
    try {
      await stopBrowser(profileId);
      await loadActiveBrowsers();
    } catch (error) {
      console.error('Failed to stop browser:', error);
      alert('Failed to stop browser: ' + error);
    }
  };
  
  const isBrowserActive = (profileId: string) => {
    return activeBrowsers.includes(profileId);
  };

  if (profiles.length === 0) {
    return (
      <div>
        <div className="page-header">
          <h1>Profiles</h1>
          <button className="btn btn-primary" onClick={() => setShowModal(true)}>
            <Plus size={18} />
            New Profile
          </button>
        </div>

        <div className="empty-state">
          <UserCircle size={64} />
          <h3>No profiles yet</h3>
          <p>Create your first browser profile to get started</p>
          <button className="btn btn-primary" onClick={() => setShowModal(true)}>
            <Plus size={18} />
            Create Profile
          </button>
        </div>

        {showModal && (
          <div className="modal-overlay" onClick={() => setShowModal(false)}>
            <div className="modal" onClick={(e) => e.stopPropagation()}>
              <h2>New Profile</h2>
              <form onSubmit={handleSubmit}>
                <div className="form-group">
                  <label>Profile Name</label>
                  <input
                    type="text"
                    value={formData.name}
                    onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                    placeholder="My Profile"
                    required
                  />
                </div>

                <div className="form-group">
                  <label>User Agent</label>
                  <input
                    type="text"
                    value={formData.user_agent}
                    onChange={(e) => setFormData({ ...formData, user_agent: e.target.value })}
                  />
                </div>

                <div className="form-group">
                  <label>Locale</label>
                  <select
                    value={formData.locale}
                    onChange={(e) => setFormData({ ...formData, locale: e.target.value })}
                  >
                    <option value="en-US">English (US)</option>
                    <option value="en-GB">English (UK)</option>
                    <option value="es-ES">Spanish</option>
                    <option value="fr-FR">French</option>
                  </select>
                </div>

                <div className="form-group">
                  <label>Timezone</label>
                  <select
                    value={formData.timezone}
                    onChange={(e) => setFormData({ ...formData, timezone: e.target.value })}
                  >
                    <option value="America/New_York">America/New_York</option>
                    <option value="Europe/London">Europe/London</option>
                    <option value="Asia/Tokyo">Asia/Tokyo</option>
                    <option value="Asia/Dubai">Asia/Dubai</option>
                  </select>
                </div>

                <div className="form-group">
                  <label>Proxy</label>
                  <select
                    value={formData.proxy_id}
                    onChange={(e) => setFormData({ ...formData, proxy_id: e.target.value })}
                  >
                    <option value="">No Proxy</option>
                    {proxies.map(proxy => (
                      <option key={proxy.id} value={proxy.id}>
                        {proxy.name} ({proxy.host}:{proxy.port})
                      </option>
                    ))}
                  </select>
                </div>

                <div className="form-group">
                  <label className="checkbox-label">
                    <input
                      type="checkbox"
                      checked={formData.canvas_noise}
                      onChange={(e) => setFormData({ ...formData, canvas_noise: e.target.checked })}
                    />
                    Canvas Noise Protection
                  </label>
                </div>

                <div className="form-group">
                  <label className="checkbox-label">
                    <input
                      type="checkbox"
                      checked={formData.audio_noise}
                      onChange={(e) => setFormData({ ...formData, audio_noise: e.target.checked })}
                    />
                    Audio Noise Protection
                  </label>
                </div>

                <div className="modal-actions">
                  <button type="button" className="btn btn-outline" onClick={() => setShowModal(false)}>
                    Cancel
                  </button>
                  <button type="submit" className="btn btn-primary">
                    Create Profile
                  </button>
                </div>
              </form>
            </div>
          </div>
        )}
      </div>
    );
  }

  return (
    <div>
      <div className="page-header">
        <h1>Profiles</h1>
        <button className="btn btn-primary" onClick={() => setShowModal(true)}>
          <Plus size={18} />
          New Profile
        </button>
      </div>

      <div className="profile-grid">
        {profiles.map(profile => (
          <div key={profile.id} className="profile-card">
            <div className="profile-header">
              <div>
                <div className="profile-name">{profile.name}</div>
                <div className="profile-info">{profile.locale}</div>
              </div>
              <div className="profile-actions">
                {isBrowserActive(profile.id) ? (
                  <button className="btn-icon btn-danger" onClick={() => handleStop(profile.id)} title="Stop Browser">
                    <Square size={16} />
                  </button>
                ) : (
                  <button className="btn-icon btn-success" onClick={() => handleLaunch(profile)} title="Launch Browser">
                    <Play size={16} />
                  </button>
                )}
                <button className="btn-icon" onClick={() => handleDelete(profile.id)} title="Delete Profile">
                  <Trash2 size={16} />
                </button>
              </div>
            </div>

            <div className="profile-info">
              <strong>Timezone:</strong> {profile.timezone}
            </div>
            <div className="profile-info">
              <strong>Locale:</strong> {profile.locale}
            </div>
            <div className="profile-info">
              <strong>Protection:</strong>{' '}
              {profile.canvas_noise && <span className="badge badge-success">Canvas</span>}
              {profile.audio_noise && <span className="badge badge-success">Audio</span>}
            </div>
          </div>
        ))}
      </div>

      {showModal && (
        <div className="modal-overlay" onClick={() => setShowModal(false)}>
          <div className="modal" onClick={(e) => e.stopPropagation()}>
            <h2>New Profile</h2>
            <form onSubmit={handleSubmit}>
              <div className="form-group">
                <label>Profile Name</label>
                <input
                  type="text"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  placeholder="My Profile"
                  required
                />
              </div>

              <div className="form-group">
                <label>Locale</label>
                <select
                  value={formData.locale}
                  onChange={(e) => setFormData({ ...formData, locale: e.target.value })}
                >
                  <option value="en-US">English (US)</option>
                  <option value="en-GB">English (UK)</option>
                  <option value="es-ES">Spanish</option>
                  <option value="fr-FR">French</option>
                </select>
              </div>

              <div className="form-group">
                <label>Timezone</label>
                <select
                  value={formData.timezone}
                  onChange={(e) => setFormData({ ...formData, timezone: e.target.value })}
                >
                  <option value="America/New_York">America/New_York</option>
                  <option value="Europe/London">Europe/London</option>
                  <option value="Asia/Tokyo">Asia/Tokyo</option>
                </select>
              </div>

              <div className="form-group">
                <label>Proxy</label>
                <select
                  value={formData.proxy_id}
                  onChange={(e) => setFormData({ ...formData, proxy_id: e.target.value })}
                >
                  <option value="">No Proxy</option>
                  {proxies.map(proxy => (
                    <option key={proxy.id} value={proxy.id}>
                      {proxy.host}:{proxy.port}
                    </option>
                  ))}
                </select>
              </div>

              <div className="modal-actions">
                <button type="button" className="btn btn-outline" onClick={() => setShowModal(false)}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary">
                  Create Profile
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}
