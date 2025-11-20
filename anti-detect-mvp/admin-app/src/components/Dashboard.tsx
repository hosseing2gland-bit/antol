import { useEffect } from 'react';
import { Users, Key, UserCircle, Globe } from 'lucide-react';
import { 
  useUsersStore, 
  useLicensesStore, 
  useProfilesStore, 
  useProxiesStore 
} from '../store';

export default function Dashboard() {
  const { users, fetchUsers } = useUsersStore();
  const { licenses, fetchLicenses } = useLicensesStore();
  const { profiles, fetchProfiles } = useProfilesStore();
  const { proxies, fetchProxies } = useProxiesStore();

  useEffect(() => {
    fetchUsers();
    fetchLicenses();
    fetchProfiles();
    fetchProxies();
  }, []);

  const stats = [
    {
      icon: Users,
      label: 'Total Users',
      value: users.length,
      color: 'blue',
    },
    {
      icon: Key,
      label: 'Active Licenses',
      value: licenses.filter(l => l.is_active).length,
      color: 'green',
    },
    {
      icon: UserCircle,
      label: 'Profiles',
      value: profiles.length,
      color: 'purple',
    },
    {
      icon: Globe,
      label: 'Proxies',
      value: proxies.length,
      color: 'orange',
    },
  ];

  return (
    <div className="dashboard">
      <h1>Dashboard</h1>

      <div className="stats-grid">
        {stats.map(({ icon: Icon, label, value, color }) => (
          <div key={label} className={`stat-card stat-${color}`}>
            <Icon size={32} />
            <div>
              <div className="stat-value">{value}</div>
              <div className="stat-label">{label}</div>
            </div>
          </div>
        ))}
      </div>

      <div className="dashboard-grid">
        <div className="card">
          <h2>Recent Users</h2>
          <div className="table-container">
            <table>
              <thead>
                <tr>
                  <th>Email</th>
                  <th>Role</th>
                  <th>Status</th>
                </tr>
              </thead>
              <tbody>
                {users.slice(0, 5).map(user => (
                  <tr key={user.id}>
                    <td>{user.email}</td>
                    <td><span className="badge badge-info">{user.role}</span></td>
                    <td>
                      <span className={`badge ${user.is_active ? 'badge-success' : 'badge-danger'}`}>
                        {user.is_active ? 'Active' : 'Inactive'}
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        <div className="card">
          <h2>Recent Licenses</h2>
          <div className="table-container">
            <table>
              <thead>
                <tr>
                  <th>Plan</th>
                  <th>Profiles</th>
                  <th>Status</th>
                </tr>
              </thead>
              <tbody>
                {licenses.slice(0, 5).map(license => (
                  <tr key={license.id}>
                    <td><span className="badge badge-primary">{license.plan}</span></td>
                    <td>{license.max_profiles}</td>
                    <td>
                      <span className={`badge ${license.is_active ? 'badge-success' : 'badge-danger'}`}>
                        {license.is_active ? 'Active' : 'Revoked'}
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
}
