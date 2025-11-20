import { useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { useAuthStore } from './store';
import Login from './components/Login';
import Dashboard from './components/Dashboard';
import Users from './components/Users';
import Licenses from './components/Licenses';
import Profiles from './components/Profiles';
import Proxies from './components/Proxies';
import Settings from './components/Settings';
import Sidebar from './components/Sidebar';
import './App.css';

function App() {
  const { user, checkAuth } = useAuthStore();

  useEffect(() => {
    checkAuth();
  }, [checkAuth]);

  if (!user) {
    return <Login />;
  }

  return (
    <Router>
      <div className="app">
        <Sidebar />
        <main className="main-content">
          <Routes>
            <Route path="/" element={<Navigate to="/dashboard" replace />} />
            <Route path="/dashboard" element={<Dashboard />} />
            <Route path="/users" element={<Users />} />
            <Route path="/licenses" element={<Licenses />} />
            <Route path="/profiles" element={<Profiles />} />
            <Route path="/proxies" element={<Proxies />} />
            <Route path="/settings" element={<Settings />} />
          </Routes>
        </main>
      </div>
    </Router>
  );
}

export default App;
