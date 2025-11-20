import { useEffect } from 'react';
import { useProfilesStore } from '../store';

export default function Profiles() {
  const { profiles, fetchProfiles } = useProfilesStore();

  useEffect(() => {
    fetchProfiles();
  }, []);

  return (
    <div className="profiles-page">
      <div className="page-header">
        <h1>Profiles</h1>
      </div>

      <div className="card">
        <div className="table-container">
          <table>
            <thead>
              <tr>
                <th>Name</th>
                <th>User Agent</th>
                <th>Locale</th>
                <th>Timezone</th>
                <th>Created</th>
              </tr>
            </thead>
            <tbody>
              {profiles.map(profile => (
                <tr key={profile.id}>
                  <td>{profile.name}</td>
                  <td><small>{profile.user_agent.substring(0, 50)}...</small></td>
                  <td>{profile.locale}</td>
                  <td>{profile.timezone}</td>
                  <td>{new Date(profile.created_at).toLocaleDateString()}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
