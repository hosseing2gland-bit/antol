import { Link, useLocation } from 'react-router-dom';
import { 
  LayoutDashboard, 
  UserCircle, 
  Globe, 
  Key,
  Settings, 
  LogOut 
} from 'lucide-react';
import { useAuthStore } from '../store';

export default function Sidebar() {
  const location = useLocation();
  const { logout, user } = useAuthStore();

  const isActive = (path: string) => location.pathname === path;

  const navItems = [
    { path: '/dashboard', icon: LayoutDashboard, label: 'Dashboard' },
    { path: '/profiles', icon: UserCircle, label: 'Profiles' },
    { path: '/proxies', icon: Globe, label: 'Proxies' },
    { path: '/license', icon: Key, label: 'License' },
    { path: '/settings', icon: Settings, label: 'Settings' },
  ];

  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <h2>Anti-Detect</h2>
        <span className="badge badge-success">Client</span>
      </div>

      <nav className="sidebar-nav">
        {navItems.map(({ path, icon: Icon, label }) => (
          <Link
            key={path}
            to={path}
            className={`nav-item ${isActive(path) ? 'active' : ''}`}
          >
            <Icon size={20} />
            <span>{label}</span>
          </Link>
        ))}
      </nav>

      <div className="sidebar-footer">
        <div className="user-info">
          <UserCircle size={24} />
          <div>
            <div className="user-email">{user?.email}</div>
          </div>
        </div>
        <button onClick={logout} className="btn btn-outline" style={{ width: '100%' }}>
          <LogOut size={18} />
          Logout
        </button>
      </div>
    </aside>
  );
}
