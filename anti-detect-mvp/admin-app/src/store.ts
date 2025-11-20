import { create } from 'zustand';
import axios from 'axios';

const API_URL = 'http://localhost:8080/api';

interface User {
  id: string;
  email: string;
  role: 'admin' | 'user';
  created_at: string;
  is_active: boolean;
}

interface License {
  id: string;
  key: string;
  user_id: string;
  expires_at: string;
  is_active: boolean;
  max_profiles: number;
}

interface Profile {
  id: string;
  user_id: string;
  name: string;
  fingerprint: any;
  proxy_id?: string;
  created_at: string;
  last_used?: string;
}

interface Proxy {
  id: string;
  user_id: string;
  name: string;
  type: 'http' | 'socks5';
  host: string;
  port: number;
  username?: string;
  password?: string;
  is_active: boolean;
}

interface AuthState {
  user: User | null;
  token: string | null;
  login: (email: string, password: string) => Promise<void>;
  logout: () => void;
  checkAuth: () => Promise<void>;
}

interface UsersState {
  users: User[];
  loading: boolean;
  fetchUsers: () => Promise<void>;
  createUser: (data: Partial<User>) => Promise<void>;
  updateUser: (id: string, data: Partial<User>) => Promise<void>;
  deleteUser: (id: string) => Promise<void>;
}

interface LicensesState {
  licenses: License[];
  loading: boolean;
  fetchLicenses: () => Promise<void>;
  createLicense: (data: Partial<License>) => Promise<void>;
  revokeLicense: (id: string) => Promise<void>;
}

interface ProfilesState {
  profiles: Profile[];
  loading: boolean;
  fetchProfiles: () => Promise<void>;
  createProfile: (data: Partial<Profile>) => Promise<void>;
  updateProfile: (id: string, data: Partial<Profile>) => Promise<void>;
  deleteProfile: (id: string) => Promise<void>;
}

interface ProxiesState {
  proxies: Proxy[];
  loading: boolean;
  fetchProxies: () => Promise<void>;
  createProxy: (data: Partial<Proxy>) => Promise<void>;
  updateProxy: (id: string, data: Partial<Proxy>) => Promise<void>;
  deleteProxy: (id: string) => Promise<void>;
  testProxy: (id: string) => Promise<boolean>;
}

export const useAuthStore = create<AuthState>((set, get) => ({
  user: null,
  token: localStorage.getItem('token'),
  
  login: async (email: string, password: string) => {
    try {
      const response = await axios.post(`${API_URL}/auth/login`, { email, password });
      const { token, user } = response.data;
      localStorage.setItem('token', token);
      axios.defaults.headers.common['Authorization'] = `Bearer ${token}`;
      set({ token, user });
    } catch (error) {
      throw error;
    }
  },
  
  logout: () => {
    localStorage.removeItem('token');
    delete axios.defaults.headers.common['Authorization'];
    set({ token: null, user: null });
  },
  
  checkAuth: async () => {
    const token = get().token;
    if (!token) return;
    
    try {
      axios.defaults.headers.common['Authorization'] = `Bearer ${token}`;
      const response = await axios.get(`${API_URL}/auth/me`);
      set({ user: response.data });
    } catch (error) {
      get().logout();
    }
  },
}));

export const useUsersStore = create<UsersState>((set) => ({
  users: [],
  loading: false,
  
  fetchUsers: async () => {
    set({ loading: true });
    try {
      const response = await axios.get(`${API_URL}/admin/users`);
      set({ users: response.data, loading: false });
    } catch (error) {
      set({ loading: false });
      throw error;
    }
  },
  
  createUser: async (data) => {
    try {
      const response = await axios.post(`${API_URL}/admin/users`, data);
      set((state) => ({ users: [...state.users, response.data] }));
    } catch (error) {
      throw error;
    }
  },
  
  updateUser: async (id, data) => {
    try {
      const response = await axios.put(`${API_URL}/admin/users/${id}`, data);
      set((state) => ({
        users: state.users.map((u) => (u.id === id ? response.data : u)),
      }));
    } catch (error) {
      throw error;
    }
  },
  
  deleteUser: async (id) => {
    try {
      await axios.delete(`${API_URL}/admin/users/${id}`);
      set((state) => ({
        users: state.users.filter((u) => u.id !== id),
      }));
    } catch (error) {
      throw error;
    }
  },
}));

export const useLicensesStore = create<LicensesState>((set) => ({
  licenses: [],
  loading: false,
  
  fetchLicenses: async () => {
    set({ loading: true });
    try {
      const response = await axios.get(`${API_URL}/admin/licenses`);
      set({ licenses: response.data, loading: false });
    } catch (error) {
      set({ loading: false });
      throw error;
    }
  },
  
  createLicense: async (data) => {
    try {
      const response = await axios.post(`${API_URL}/admin/licenses`, data);
      set((state) => ({ licenses: [...state.licenses, response.data] }));
    } catch (error) {
      throw error;
    }
  },
  
  revokeLicense: async (id) => {
    try {
      await axios.delete(`${API_URL}/admin/licenses/${id}`);
      set((state) => ({
        licenses: state.licenses.map((l) =>
          l.id === id ? { ...l, is_active: false } : l
        ),
      }));
    } catch (error) {
      throw error;
    }
  },
}));

export const useProfilesStore = create<ProfilesState>((set) => ({
  profiles: [],
  loading: false,
  
  fetchProfiles: async () => {
    set({ loading: true });
    try {
      const response = await axios.get(`${API_URL}/profiles`);
      set({ profiles: response.data, loading: false });
    } catch (error) {
      set({ loading: false });
      throw error;
    }
  },
  
  createProfile: async (data) => {
    try {
      const response = await axios.post(`${API_URL}/profiles`, data);
      set((state) => ({ profiles: [...state.profiles, response.data] }));
    } catch (error) {
      throw error;
    }
  },
  
  updateProfile: async (id, data) => {
    try {
      const response = await axios.put(`${API_URL}/profiles/${id}`, data);
      set((state) => ({
        profiles: state.profiles.map((p) => (p.id === id ? response.data : p)),
      }));
    } catch (error) {
      throw error;
    }
  },
  
  deleteProfile: async (id) => {
    try {
      await axios.delete(`${API_URL}/profiles/${id}`);
      set((state) => ({
        profiles: state.profiles.filter((p) => p.id !== id),
      }));
    } catch (error) {
      throw error;
    }
  },
}));

export const useProxiesStore = create<ProxiesState>((set) => ({
  proxies: [],
  loading: false,
  
  fetchProxies: async () => {
    set({ loading: true });
    try {
      const response = await axios.get(`${API_URL}/proxies`);
      set({ proxies: response.data, loading: false });
    } catch (error) {
      set({ loading: false });
      throw error;
    }
  },
  
  createProxy: async (data) => {
    try {
      const response = await axios.post(`${API_URL}/proxies`, data);
      set((state) => ({ proxies: [...state.proxies, response.data] }));
    } catch (error) {
      throw error;
    }
  },
  
  updateProxy: async (id, data) => {
    try {
      const response = await axios.put(`${API_URL}/proxies/${id}`, data);
      set((state) => ({
        proxies: state.proxies.map((p) => (p.id === id ? response.data : p)),
      }));
    } catch (error) {
      throw error;
    }
  },
  
  deleteProxy: async (id) => {
    try {
      await axios.delete(`${API_URL}/proxies/${id}`);
      set((state) => ({
        proxies: state.proxies.filter((p) => p.id !== id),
      }));
    } catch (error) {
      throw error;
    }
  },
  
  testProxy: async (id) => {
    try {
      const response = await axios.post(`${API_URL}/proxies/${id}/test`);
      return response.data.success;
    } catch (error) {
      return false;
    }
  },
}));
