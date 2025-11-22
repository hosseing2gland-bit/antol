import { useState } from 'react';
import { useAuthStore } from '../store';

export default function Login() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [debugInfo, setDebugInfo] = useState('');
  const { login } = useAuthStore();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setDebugInfo('');

    try {
      // Check Tauri API availability
      const tauriAvailable = typeof (window as any).__TAURI__ !== 'undefined';
      console.log('Tauri API available:', tauriAvailable);
      setDebugInfo(`Tauri API: ${tauriAvailable ? 'Available ✓' : 'NOT FOUND ✗'}`);

      await login(email, password);
    } catch (err: any) {
      console.error('Login error:', err);
      console.error('Error details:', JSON.stringify(err, null, 2));
      
      let errorMessage = 'Login failed';
      let debugDetails = '';
      
      if (err.code === 'ERR_NETWORK') {
        errorMessage = 'Network Error: Cannot connect to server';
        debugDetails = `Code: ${err.code}\nMessage: ${err.message}\nServer: http://108.143.173.222:3000`;
      } else if (err.code === 'ECONNABORTED') {
        errorMessage = 'Request Timeout: Server took too long to respond';
        debugDetails = `Code: ${err.code}`;
      } else if (err.response?.status === 401) {
        errorMessage = 'Invalid email or password';
        debugDetails = `Status: 401`;
      } else if (err.response?.status === 400) {
        errorMessage = 'Bad request: Please check your credentials';
        debugDetails = `Status: 400`;
      } else if (err.message) {
        errorMessage = err.message;
        debugDetails = `Code: ${err.code || 'N/A'}\nStack: ${err.stack?.substring(0, 200) || 'N/A'}`;
      }
      
      setError(errorMessage);
      setDebugInfo(`Error Details:\n${debugDetails}\n\nTauri: ${typeof (window as any).__TAURI__ !== 'undefined' ? 'YES' : 'NO'}`);
    }
  };

  return (
    <div className="login-container">
      <div className="login-box">
        <h1>Anti-Detect Browser</h1>
        <h2>Admin Panel</h2>

        <form onSubmit={handleSubmit}>
          {error && (
            <div className="error-message" style={{ whiteSpace: 'pre-line', textAlign: 'left' }}>
              <strong>{error}</strong>
            </div>
          )}
          
          {debugInfo && (
            <div style={{ 
              background: '#2a2a2a', 
              color: '#00ff00', 
              padding: '10px', 
              borderRadius: '4px', 
              fontSize: '11px',
              fontFamily: 'monospace',
              whiteSpace: 'pre-line',
              textAlign: 'left',
              marginBottom: '10px'
            }}>
              {debugInfo}
            </div>
          )}

          <div className="form-group">
            <label>Email</label>
            <input
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              placeholder="admin@antidetect.local"
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

          <button type="submit" className="btn btn-primary">
            Login
          </button>
        </form>

        <div className="login-footer">
          <small>Default: admin@antidetect.local / admin123</small>
        </div>
      </div>
    </div>
  );
}
