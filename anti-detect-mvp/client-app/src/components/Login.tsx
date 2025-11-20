import { useState } from 'react';
import { useAuthStore } from '../store';

export default function Login() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [showActivation, setShowActivation] = useState(false);
  const [licenseKey, setLicenseKey] = useState('');
  const { login } = useAuthStore();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    try {
      await login(email, password);
    } catch (err: any) {
      setError(err.message || 'Login failed');
    }
  };

  return (
    <div className="login-container">
      <div className="login-box">
        <h1>Anti-Detect Browser</h1>
        <h2>Client Application</h2>

        {!showActivation ? (
          <form onSubmit={handleSubmit}>
            {error && <div className="error-message">{error}</div>}

            <div className="form-group">
              <label>Email</label>
              <input
                type="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                placeholder="your@email.com"
                required
              />
            </div>

            <div className="form-group">
              <label>Password</label>
              <input
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                placeholder="••••••••"
                required
              />
            </div>

            <button type="submit" className="btn btn-primary" style={{ width: '100%' }}>
              Login
            </button>
          </form>
        ) : (
          <form onSubmit={(e) => {
            e.preventDefault();
            // Handle license activation
            setShowActivation(false);
          }}>
            <div className="form-group">
              <label>License Key</label>
              <input
                type="text"
                value={licenseKey}
                onChange={(e) => setLicenseKey(e.target.value)}
                placeholder="XXXX-XXXX-XXXX-XXXX"
                required
              />
            </div>

            <button type="submit" className="btn btn-primary" style={{ width: '100%' }}>
              Activate License
            </button>
          </form>
        )}

        <div className="login-footer">
          <button 
            className="btn btn-outline" 
            style={{ width: '100%', marginTop: '1rem' }}
            onClick={() => setShowActivation(!showActivation)}
          >
            {showActivation ? 'Back to Login' : 'Activate License'}
          </button>
        </div>
      </div>
    </div>
  );
}
