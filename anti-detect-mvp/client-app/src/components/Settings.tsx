import { useAuthStore } from '../store';

export default function Settings() {
  const { user } = useAuthStore();

  return (
    <div>
      <h1>Settings</h1>

      <div className="card">
        <h2>Account Information</h2>
        <div className="license-info">
          <div className="info-item">
            <label>Email</label>
            <p>{user?.email}</p>
          </div>
          <div className="info-item">
            <label>Account Type</label>
            <p><span className="badge badge-primary">User</span></p>
          </div>
        </div>
      </div>

      <div className="card">
        <h2>Application Settings</h2>
        <div className="license-info">
          <div className="info-item">
            <label>Backend URL</label>
            <p><code>http://localhost:3000</code></p>
          </div>
          <div className="info-item">
            <label>Version</label>
            <p><code>1.0.0</code></p>
          </div>
        </div>
      </div>

      <div className="card">
        <h2>Anti-Detection Features</h2>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
          <div className="checkbox-label">
            <input type="checkbox" defaultChecked />
            Canvas Fingerprinting Protection
          </div>
          <div className="checkbox-label">
            <input type="checkbox" defaultChecked />
            WebGL Fingerprinting Protection
          </div>
          <div className="checkbox-label">
            <input type="checkbox" defaultChecked />
            Audio Fingerprinting Protection
          </div>
          <div className="checkbox-label">
            <input type="checkbox" defaultChecked />
            Font Fingerprinting Protection
          </div>
          <div className="checkbox-label">
            <input type="checkbox" defaultChecked />
            WebRTC Leak Protection
          </div>
        </div>
      </div>

      <div className="card">
        <h2>About</h2>
        <p style={{ color: 'var(--text-secondary)', fontSize: '0.875rem' }}>
          Anti-Detect Browser Client Application<br />
          Version 1.0.0<br />
          Built with React, TypeScript, and Tauri
        </p>
      </div>
    </div>
  );
}
