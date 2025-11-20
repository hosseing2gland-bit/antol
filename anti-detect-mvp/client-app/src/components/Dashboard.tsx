import { useEffect } from 'react';
import { UserCircle, Globe, Key, Activity } from 'lucide-react';
import { useProfilesStore, useProxiesStore, useLicenseStore } from '../store';

export default function Dashboard() {
  const { profiles, fetchProfiles } = useProfilesStore();
  const { proxies, fetchProxies } = useProxiesStore();
  const { license, fetchLicense } = useLicenseStore();

  useEffect(() => {
    fetchProfiles();
    fetchProxies();
    fetchLicense();
  }, []);

  const stats = [
    {
      icon: UserCircle,
      label: 'Profiles',
      value: profiles.length,
      max: license?.max_profiles || 0,
      color: 'blue',
    },
    {
      icon: Globe,
      label: 'Proxies',
      value: proxies.length,
      color: 'green',
    },
    {
      icon: Key,
      label: 'License',
      value: license?.plan.toUpperCase() || 'N/A',
      color: 'purple',
    },
    {
      icon: Activity,
      label: 'Status',
      value: license?.is_active ? 'Active' : 'Inactive',
      color: license?.is_active ? 'green' : 'red',
    },
  ];

  return (
    <div className="dashboard">
      <h1>Dashboard</h1>

      <div className="stats-grid">
        {stats.map(({ icon: Icon, label, value, max, color }) => (
          <div key={label} className={`stat-card stat-${color}`}>
            <Icon size={32} />
            <div>
              <div className="stat-value">
                {typeof value === 'number' && max ? `${value}/${max}` : value}
              </div>
              <div className="stat-label">{label}</div>
            </div>
          </div>
        ))}
      </div>

      <div className="card">
        <h2>Quick Actions</h2>
        <div style={{ display: 'flex', gap: '1rem', flexWrap: 'wrap' }}>
          <button className="btn btn-primary">
            <UserCircle size={18} />
            New Profile
          </button>
          <button className="btn btn-success">
            <Globe size={18} />
            Add Proxy
          </button>
        </div>
      </div>

      {license && (
        <div className="card">
          <h2>License Information</h2>
          <div className="license-info">
            <div className="info-item">
              <label>Plan</label>
              <p><span className="badge badge-primary">{license.plan}</span></p>
            </div>
            <div className="info-item">
              <label>Status</label>
              <p>
                <span className={`badge ${license.is_active ? 'badge-success' : 'badge-danger'}`}>
                  {license.is_active ? 'Active' : 'Inactive'}
                </span>
              </p>
            </div>
            <div className="info-item">
              <label>Expires</label>
              <p>{new Date(license.expires_at).toLocaleDateString()}</p>
            </div>
            <div className="info-item">
              <label>Max Profiles</label>
              <p>{license.max_profiles}</p>
            </div>
          </div>
        </div>
      )}

      {profiles.length > 0 && (
        <div className="card">
          <h2>Recent Profiles</h2>
          <div className="table-container">
            <table>
              <thead>
                <tr>
                  <th>Name</th>
                  <th>User Agent</th>
                  <th>Resolution</th>
                  <th>Created</th>
                </tr>
              </thead>
              <tbody>
                {profiles.slice(0, 5).map(profile => (
                  <tr key={profile.id}>
                    <td>{profile.name}</td>
                    <td><small>{profile.user_agent.substring(0, 40)}...</small></td>
                    <td>{profile.locale}</td>
                    <td>{new Date(profile.created_at).toLocaleDateString()}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  );
}
