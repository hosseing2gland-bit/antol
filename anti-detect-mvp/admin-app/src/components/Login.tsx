import { useState } from 'react';
import { useAuthStore } from '../store';

export default function Login() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const { login } = useAuthStore();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    try {
      await login(email, password);
    } catch (err: any) {
      console.error('Login error:', err);
      
      let errorMessage = 'Login failed';
      
      if (err.code === 'ERR_NETWORK') {
        errorMessage = 'Network Error: Cannot connect to server. Please check:\n1. Your internet connection\n2. VPN if enabled\n3. Server is running at http://108.143.173.222:3000';
      } else if (err.code === 'ECONNABORTED') {
        errorMessage = 'Request Timeout: Server took too long to respond';
      } else if (err.response?.status === 401) {
        errorMessage = 'Invalid email or password';
      } else if (err.response?.status === 400) {
        errorMessage = 'Bad request: Please check your credentials';
      } else if (err.message) {
        errorMessage = err.message;
      }
      
      setError(errorMessage);
    }
  };

  return (
    <div className="login-container">
      <div className="login-box">
        <h1>Anti-Detect Browser</h1>
        <h2>Admin Panel</h2>

        <form onSubmit={handleSubmit}>
          {error && <div className="error-message">{error}</div>}

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
