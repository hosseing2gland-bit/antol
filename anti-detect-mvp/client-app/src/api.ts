import axios, { AxiosError } from 'axios';

// API Configuration
export const API_URL = 'http://108.143.173.222:3000/api';

// Check if we're running in Tauri
const isTauri = typeof window !== 'undefined' && '__TAURI__' in window;

// Create axios instance with custom config
export const api = axios.create({
  baseURL: API_URL,
  timeout: 30000, // 30 seconds timeout
  headers: {
    'Content-Type': 'application/json',
  },
  // Important for VPN and CORS compatibility
  validateStatus: (status) => status < 500,
  // Disable XMLHttpRequest in Tauri
  adapter: isTauri ? undefined : undefined,
});

// Request interceptor to add auth token
api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Response interceptor for better error handling
api.interceptors.response.use(
  (response) => response,
  (error: AxiosError) => {
    if (error.code === 'ECONNABORTED') {
      console.error('Request timeout - check your network connection');
    } else if (error.code === 'ERR_NETWORK') {
      console.error('Network error - check if server is accessible');
    } else if (error.response?.status === 401) {
      // Unauthorized - clear token
      localStorage.removeItem('token');
      window.location.reload();
    }
    return Promise.reject(error);
  }
);
