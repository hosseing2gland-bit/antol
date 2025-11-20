import { useEffect, useState } from 'react';
import { Key, CheckCircle, XCircle } from 'lucide-react';
import { useLicenseStore } from '../store';

export default function License() {
  const { license, fetchLicense, activateLicense } = useLicenseStore();
  const [showActivation, setShowActivation] = useState(false);
  const [licenseKey, setLicenseKey] = useState('');
  const [error, setError] = useState('');

  useEffect(() => {
    fetchLicense();
  }, []);

  const handleActivate = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    try {
      await activateLicense(licenseKey);
      setShowActivation(false);
      setLicenseKey('');
      fetchLicense();
      alert('License activated successfully!');
    } catch (err: any) {
      setError(err.message || 'Failed to activate license');
    }
  };

  return (
    <div>
      <div className="page-header">
        <h1>License</h1>
        {!license && (
          <button className="btn btn-primary" onClick={() => setShowActivation(true)}>
            <Key size={18} />
            Activate License
          </button>
        )}
      </div>

      {license ? (
        <div>
          <div className="card">
            <div style={{ display: 'flex', alignItems: 'center', gap: '1rem', marginBottom: '1.5rem' }}>
              {license.is_active ? (
                <CheckCircle size={32} color="var(--success)" />
              ) : (
                <XCircle size={32} color="var(--danger)" />
              )}
              <div>
                <h2 style={{ margin: 0 }}>License Status</h2>
                <p style={{ margin: '0.25rem 0 0', color: 'var(--text-secondary)', fontSize: '0.875rem' }}>
                  {license.is_active ? 'Your license is active' : 'Your license is inactive'}
                </p>
              </div>
            </div>

            <div className="license-info">
              <div className="info-item">
                <label>License Key</label>
                <p><code className="license-key">{license.key}</code></p>
              </div>

              <div className="info-item">
                <label>Plan</label>
                <p><span className="badge badge-primary">{license.plan.toUpperCase()}</span></p>
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
                <label>Max Profiles</label>
                <p>{license.max_profiles}</p>
              </div>

              <div className="info-item">
                <label>Expires</label>
                <p>{new Date(license.expires_at).toLocaleDateString()}</p>
              </div>

              {license.activated_at && (
                <div className="info-item">
                  <label>Activated</label>
                  <p>{new Date(license.activated_at).toLocaleDateString()}</p>
                </div>
              )}
            </div>
          </div>

          <div className="card">
            <h2>Features Included</h2>
            <ul style={{ paddingLeft: '1.5rem', color: 'var(--text-secondary)' }}>
              <li style={{ marginBottom: '0.5rem' }}>Up to {license.max_profiles} browser profiles</li>
              <li style={{ marginBottom: '0.5rem' }}>Canvas fingerprint protection</li>
              <li style={{ marginBottom: '0.5rem' }}>WebGL fingerprint protection</li>
              <li style={{ marginBottom: '0.5rem' }}>Audio fingerprint protection</li>
              <li style={{ marginBottom: '0.5rem' }}>Font fingerprint protection</li>
              <li style={{ marginBottom: '0.5rem' }}>Proxy integration</li>
              <li style={{ marginBottom: '0.5rem' }}>User agent rotation</li>
            </ul>
          </div>
        </div>
      ) : (
        <div className="empty-state">
          <Key size={64} />
          <h3>No license activated</h3>
          <p>Activate a license key to start using the anti-detect browser</p>
          <button className="btn btn-primary" onClick={() => setShowActivation(true)}>
            <Key size={18} />
            Activate License
          </button>
        </div>
      )}

      {showActivation && (
        <div className="modal-overlay" onClick={() => setShowActivation(false)}>
          <div className="modal" onClick={(e) => e.stopPropagation()}>
            <h2>Activate License</h2>
            <form onSubmit={handleActivate}>
              {error && <div className="error-message">{error}</div>}

              <div className="form-group">
                <label>License Key</label>
                <input
                  type="text"
                  value={licenseKey}
                  onChange={(e) => setLicenseKey(e.target.value)}
                  placeholder="XXXX-XXXX-XXXX-XXXX"
                  required
                  style={{ fontFamily: 'Monaco, monospace' }}
                />
                <small style={{ color: 'var(--text-secondary)', fontSize: '0.75rem' }}>
                  Enter the license key you received
                </small>
              </div>

              <div className="modal-actions">
                <button type="button" className="btn btn-outline" onClick={() => setShowActivation(false)}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary">
                  <Key size={18} />
                  Activate
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}
