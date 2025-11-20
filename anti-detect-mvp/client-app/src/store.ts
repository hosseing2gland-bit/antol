import { create } from 'zustand';
import axios from 'axios';

const API_URL = 'http://localhost:3000/api';

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
  user_agent: string;
  screen_resolution: string;
  timezone: string;
  language: string;
  webgl_vendor?: string;
  webgl_renderer?: string;
  canvas_noise: boolean;
  audio_noise: boolean;
  fonts?: string[];
  proxy_id?: string;
  created_at: string;
  updated_at: string;
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
  proxy_type: string;
  host: string;
  port: number;
  username?: string;
  password?: string;
  country?: string;
  created_at: string;
  updated_at: string;
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
  license_key: string;
  plan: string;
  max_profiles: number;
  user_id?: string;
  is_active: boolean;
  expires_at: string;
  activated_at?: string;
  created_at: string;
  updated_at: string;
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
    const response = await axios.post(`${API_URL}/auth/login`, { email, password });
    const { token, user } = response.data;
    localStorage.setItem('token', token);
    axios.defaults.headers.common['Authorization'] = `Bearer ${token}`;
    set({ user, token });
  },
  
  logout: () => {
    localStorage.removeItem('token');
    delete axios.defaults.headers.common['Authorization'];
    set({ user: null, token: null });
  },
  
  checkAuth: () => {
    const token = localStorage.getItem('token');
    if (token) {
      axios.defaults.headers.common['Authorization'] = `Bearer ${token}`;
      // In production, verify token with backend
      // For now, just set it
    }
  },
}));

// Profiles Store
export const useProfilesStore = create<ProfilesStore>((set) => ({
  profiles: [],
  
  fetchProfiles: async () => {
    const response = await axios.get(`${API_URL}/profiles`);
    set({ profiles: response.data });
  },
  
  createProfile: async (data) => {
    await axios.post(`${API_URL}/profiles`, data);
  },
  
  updateProfile: async (id, data) => {
    await axios.put(`${API_URL}/profiles/${id}`, data);
  },
  
  deleteProfile: async (id) => {
    await axios.delete(`${API_URL}/profiles/${id}`);
  },
}));

// Proxies Store
export const useProxiesStore = create<ProxiesStore>((set) => ({
  proxies: [],
  
  fetchProxies: async () => {
    const response = await axios.get(`${API_URL}/proxies`);
    set({ proxies: response.data });
  },
  
  createProxy: async (data) => {
    await axios.post(`${API_URL}/proxies`, data);
  },
  
  updateProxy: async (id, data) => {
    await axios.put(`${API_URL}/proxies/${id}`, data);
  },
  
  deleteProxy: async (id) => {
    await axios.delete(`${API_URL}/proxies/${id}`);
  },
  
  testProxy: async (id) => {
    const response = await axios.post(`${API_URL}/proxies/${id}/test`);
    return response.data;
  },
}));

// License Store
export const useLicenseStore = create<LicenseStore>((set) => ({
  license: null,
  
  fetchLicense: async () => {
    const response = await axios.get(`${API_URL}/licenses`);
    // Get user's active license
    const userLicense = response.data.find((l: License) => l.is_active);
    set({ license: userLicense || null });
  },
  
  activateLicense: async (licenseKey) => {
    const userId = useAuthStore.getState().user?.id;
    await axios.post(`${API_URL}/licenses/activate/${licenseKey}`, userId);
  },
}));
