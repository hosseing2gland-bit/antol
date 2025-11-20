import { useEffect, useState } from 'react';
import { Plus, Trash2, Activity } from 'lucide-react';
import { useProxiesStore } from '../store';

export default function Proxies() {
  const { proxies, fetchProxies, createProxy, deleteProxy, testProxy } = useProxiesStore();
  const [showModal, setShowModal] = useState(false);
  const [testing, setTesting] = useState<string | null>(null);
  const [formData, setFormData] = useState({
    proxy_type: 'http',
    host: '',
    port: 8080,
    username: '',
    password: '',
    country: '',
  });

  useEffect(() => {
    fetchProxies();
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await createProxy({
        ...formData,
        user_id: 'current-user-id',
      });
      setShowModal(false);
      resetForm();
      fetchProxies();
    } catch (error) {
      console.error('Failed to create proxy:', error);
    }
  };

  const resetForm = () => {
    setFormData({
      proxy_type: 'http',
      host: '',
      port: 8080,
      username: '',
      password: '',
      country: '',
    });
  };

  const handleDelete = async (id: string) => {
    if (confirm('Are you sure you want to delete this proxy?')) {
      await deleteProxy(id);
      fetchProxies();
    }
  };

  const handleTest = async (id: string) => {
    setTesting(id);
    try {
      const result = await testProxy(id);
      alert(result.success ? 'Proxy connection successful!' : 'Proxy connection failed!');
    } catch (error) {
      alert('Failed to test proxy');
    } finally {
      setTesting(null);
    }
  };

  return (
    <div>
      <div className="page-header">
        <h1>Proxies</h1>
        <button className="btn btn-primary" onClick={() => setShowModal(true)}>
          <Plus size={18} />
          Add Proxy
        </button>
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
                <th>Actions</th>
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
                  <td className="actions">
                    <button 
                      className="btn-icon" 
                      onClick={() => handleTest(proxy.id)}
                      disabled={testing === proxy.id}
                    >
                      <Activity size={16} />
                    </button>
                    <button className="btn-icon" onClick={() => handleDelete(proxy.id)}>
                      <Trash2 size={16} />
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {showModal && (
        <div className="modal-overlay" onClick={() => setShowModal(false)}>
          <div className="modal" onClick={(e) => e.stopPropagation()}>
            <h2>Add Proxy</h2>
            <form onSubmit={handleSubmit}>
              <div className="form-group">
                <label>Proxy Type</label>
                <select
                  value={formData.proxy_type}
                  onChange={(e) => setFormData({ ...formData, proxy_type: e.target.value })}
                >
                  <option value="http">HTTP</option>
                  <option value="https">HTTPS</option>
                  <option value="socks5">SOCKS5</option>
                </select>
              </div>

              <div className="form-group">
                <label>Host</label>
                <input
                  type="text"
                  value={formData.host}
                  onChange={(e) => setFormData({ ...formData, host: e.target.value })}
                  placeholder="proxy.example.com"
                  required
                />
              </div>

              <div className="form-group">
                <label>Port</label>
                <input
                  type="number"
                  value={formData.port}
                  onChange={(e) => setFormData({ ...formData, port: parseInt(e.target.value) })}
                  min="1"
                  max="65535"
                  required
                />
              </div>

              <div className="form-group">
                <label>Username (Optional)</label>
                <input
                  type="text"
                  value={formData.username}
                  onChange={(e) => setFormData({ ...formData, username: e.target.value })}
                />
              </div>

              <div className="form-group">
                <label>Password (Optional)</label>
                <input
                  type="password"
                  value={formData.password}
                  onChange={(e) => setFormData({ ...formData, password: e.target.value })}
                />
              </div>

              <div className="form-group">
                <label>Country (Optional)</label>
                <input
                  type="text"
                  value={formData.country}
                  onChange={(e) => setFormData({ ...formData, country: e.target.value })}
                  placeholder="US, UK, etc."
                />
              </div>

              <div className="modal-actions">
                <button type="button" className="btn btn-outline" onClick={() => setShowModal(false)}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary">
                  Add Proxy
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}
