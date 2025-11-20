import { useEffect, useState } from 'react';
import { Plus, Key, Ban } from 'lucide-react';
import { useLicensesStore } from '../store';

export default function Licenses() {
  const { licenses, fetchLicenses, createLicense, revokeLicense } = useLicensesStore();
  const [showModal, setShowModal] = useState(false);
  const [formData, setFormData] = useState<{
    plan: 'trial' | 'basic' | 'pro' | 'enterprise';
    max_profiles: number;
  }>({
    plan: 'basic',
    max_profiles: 3,
  });

  useEffect(() => {
    fetchLicenses();
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await createLicense(formData);
      setShowModal(false);
      setFormData({ plan: 'basic', max_profiles: 3 });
      fetchLicenses();
    } catch (error) {
      console.error('Failed to create license:', error);
    }
  };

  const handleRevoke = async (id: string) => {
    if (confirm('Are you sure you want to revoke this license?')) {
      await revokeLicense(id);
      fetchLicenses();
    }
  };

  return (
    <div className="licenses-page">
      <div className="page-header">
        <h1>Licenses</h1>
        <button className="btn btn-primary" onClick={() => setShowModal(true)}>
          <Plus size={18} />
          Generate License
        </button>
      </div>

      <div className="card">
        <div className="table-container">
          <table>
            <thead>
              <tr>
                <th>License Key</th>
                <th>Plan</th>
                <th>Max Profiles</th>
                <th>Status</th>
                <th>Expires</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              {licenses.map(license => (
                <tr key={license.id}>
                  <td>
                    <code className="license-key">{license.key}</code>
                  </td>
                  <td><span className="badge badge-primary">{license.plan}</span></td>
                  <td>{license.max_profiles}</td>
                  <td>
                    <span className={`badge ${license.is_active ? 'badge-success' : 'badge-danger'}`}>
                      {license.is_active ? 'Active' : 'Revoked'}
                    </span>
                  </td>
                  <td>{new Date(license.expires_at).toLocaleDateString()}</td>
                  <td className="actions">
                    {license.is_active && (
                      <button
                        className="btn-icon"
                        onClick={() => handleRevoke(license.id)}
                      >
                        <Ban size={16} />
                      </button>
                    )}
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
            <h2>Generate License</h2>
            <form onSubmit={handleSubmit}>
              <div className="form-group">
                <label>Plan</label>
                <select
                  value={formData.plan}
                  onChange={(e) => setFormData({ ...formData, plan: e.target.value as 'trial' | 'basic' | 'pro' | 'enterprise' })}
                >
                  <option value="trial">Trial (7 days, 1 profile)</option>
                  <option value="basic">Basic (30 days, 3 profiles)</option>
                  <option value="pro">Pro (90 days, 10 profiles)</option>
                  <option value="enterprise">Enterprise (365 days, 50 profiles)</option>
                </select>
              </div>

              <div className="form-group">
                <label>Max Profiles</label>
                <input
                  type="number"
                  min="1"
                  value={formData.max_profiles}
                  onChange={(e) => setFormData({ ...formData, max_profiles: parseInt(e.target.value) })}
                  required
                />
              </div>

              <div className="modal-actions">
                <button type="button" className="btn btn-outline" onClick={() => setShowModal(false)}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary">
                  <Key size={18} />
                  Generate
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}
