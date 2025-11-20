import { useAuthStore } from '../store';

export default function Settings() {
  const { user } = useAuthStore();

  return (
    <div className="settings-page">
      <h1>Settings</h1>

      <div className="card">
        <h2>Account Information</h2>
        <div className="info-grid">
          <div className="info-item">
            <label>Email</label>
            <p>{user?.email}</p>
          </div>
          <div className="info-item">
            <label>Role</label>
            <p><span className="badge badge-primary">{user?.role}</span></p>
          </div>
        </div>
      </div>

      <div className="card">
        <h2>API Configuration</h2>
        <div className="info-grid">
          <div className="info-item">
            <label>Backend URL</label>
            <p><code>http://localhost:3000</code></p>
          </div>
          <div className="info-item">
            <label>API Version</label>
            <p><code>v1</code></p>
          </div>
        </div>
      </div>
    </div>
  );
}
