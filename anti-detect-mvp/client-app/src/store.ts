import { create } from 'zustand';
import { api, API_URL } from './api';


interface User {
  id: string;
  email: string;
  role: string;
}

interface AuthStore {
  user: User | null;
  token: string | null;
  login: (email: string, password: string) => Promise<void>;
  logout: () => void;
  checkAuth: () => void;
}

interface Profile {
  id: string;
  user_id: string;
  name: string;
  fingerprint: any;
  proxy_id?: string;
  user_agent: string;
  timezone: string;
  locale: string;
  webgl_vendor: string;
  webgl_renderer: string;
  canvas_noise: boolean;
  webgl_noise: boolean;
  audio_noise: boolean;
  is_active: boolean;
  created_at: string;
  last_used_at?: string;
}

interface ProfilesStore {
  profiles: Profile[];
  fetchProfiles: () => Promise<void>;
  createProfile: (data: any) => Promise<void>;
  updateProfile: (id: string, data: any) => Promise<void>;
  deleteProfile: (id: string) => Promise<void>;
}

interface Proxy {
  id: string;
  user_id: string;
  name: string;
  proxy_type: 'http' | 'https' | 'socks5';
  host: string;
  port: number;
  username?: string;
  password?: string;
  is_active: boolean;
  last_checked_at?: string;
  created_at: string;
}

interface ProxiesStore {
  proxies: Proxy[];
  fetchProxies: () => Promise<void>;
  createProxy: (data: any) => Promise<void>;
  updateProxy: (id: string, data: any) => Promise<void>;
  deleteProxy: (id: string) => Promise<void>;
  testProxy: (id: string) => Promise<any>;
}

interface License {
  id: string;
  key: string;
  user_id?: string;
  plan: 'trial' | 'basic' | 'pro' | 'enterprise';
  max_profiles: number;
  expires_at: string;
  is_active: boolean;
  created_at: string;
  activated_at?: string;
}

interface LicenseStore {
  license: License | null;
  fetchLicense: () => Promise<void>;
  activateLicense: (licenseKey: string) => Promise<void>;
}

// Auth Store
export const useAuthStore = create<AuthStore>((set) => ({
  user: null,
  token: localStorage.getItem('token'),
  
  login: async (email, password) => {
    const response = await api.post(`${API_URL}/auth/login`, { email, password });
    const { token, user } = response.data;
    localStorage.setItem('token', token);
    api.defaults.headers.common['Authorization'] = `Bearer ${token}`;
    set({ user, token });
  },
  
  logout: () => {
    localStorage.removeItem('token');
    delete api.defaults.headers.common['Authorization'];
    set({ user: null, token: null });
  },
  
  checkAuth: () => {
    const token = localStorage.getItem('token');
    if (token) {
      api.defaults.headers.common['Authorization'] = `Bearer ${token}`;
      // In production, verify token with backend
      // For now, just set it
    }
  },
}));

// Profiles Store
export const useProfilesStore = create<ProfilesStore>((set) => ({
  profiles: [],
  
  fetchProfiles: async () => {
    const response = await api.get(`${API_URL}/profiles`);
    set({ profiles: response.data });
  },
  
  createProfile: async (data) => {
    await api.post(`${API_URL}/profiles`, data);
  },
  
  updateProfile: async (id, data) => {
    await api.put(`${API_URL}/profiles/${id}`, data);
  },
  
  deleteProfile: async (id) => {
    await api.delete(`${API_URL}/profiles/${id}`);
  },
}));

// Proxies Store
export const useProxiesStore = create<ProxiesStore>((set) => ({
  proxies: [],
  
  fetchProxies: async () => {
    const response = await api.get(`${API_URL}/proxies`);
    set({ proxies: response.data });
  },
  
  createProxy: async (data) => {
    await api.post(`${API_URL}/proxies`, data);
  },
  
  updateProxy: async (id, data) => {
    await api.put(`${API_URL}/proxies/${id}`, data);
  },
  
  deleteProxy: async (id) => {
    await api.delete(`${API_URL}/proxies/${id}`);
  },
  
  testProxy: async (id) => {
    const response = await api.post(`${API_URL}/proxies/${id}/test`);
    return response.data;
  },
}));

// License Store
export const useLicenseStore = create<LicenseStore>((set) => ({
  license: null,
  
  fetchLicense: async () => {
    const response = await api.get(`${API_URL}/licenses`);
    // Get user's active license
    const userLicense = response.data.find((l: License) => l.is_active);
    set({ license: userLicense || null });
  },
  
  activateLicense: async (licenseKey) => {
    const userId = useAuthStore.getState().user?.id;
    await api.post(`${API_URL}/licenses/activate/${licenseKey}`, userId);
  },
}));
