import { useEffect } from 'react';
import { useProxiesStore } from '../store';

export default function Proxies() {
  const { proxies, fetchProxies } = useProxiesStore();

  useEffect(() => {
    fetchProxies();
  }, []);

  return (
    <div className="proxies-page">
      <div className="page-header">
        <h1>Proxies</h1>
      </div>

      <div className="card">
        <div className="table-container">
          <table>
            <thead>
              <tr>
                <th>Type</th>
                <th>Host</th>
                <th>Port</th>
                <th>Country</th>
                <th>Created</th>
              </tr>
            </thead>
            <tbody>
              {proxies.map(proxy => (
                <tr key={proxy.id}>
                  <td><span className="badge badge-info">{proxy.proxy_type}</span></td>
                  <td>{proxy.host}</td>
                  <td>{proxy.port}</td>
                  <td>{proxy.country || 'N/A'}</td>
                  <td>{new Date(proxy.created_at).toLocaleDateString()}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
