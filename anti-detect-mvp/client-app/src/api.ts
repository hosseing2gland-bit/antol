import axios, { AxiosError, AxiosRequestConfig, AxiosResponse } from 'axios';

// API Configuration
export const API_URL = 'http://108.143.173.222:3000/api';

// Check if we're running in Tauri
const isTauri = typeof window !== 'undefined' && (window as any).__TAURI__;

// Tauri HTTP wrapper
async function tauriHttpRequest(config: AxiosRequestConfig): Promise<AxiosResponse> {
  if (!isTauri) {
    throw new Error('Not in Tauri environment');
  }

  const { http } = (window as any).__TAURI__;
  const url = config.baseURL ? `${config.baseURL}${config.url || ''}` : config.url;
  
  const options: any = {
    method: config.method?.toUpperCase() || 'GET',
    headers: config.headers || {},
    timeout: config.timeout || 30000,
  };

  if (config.data) {
    options.body = {
      type: 'Json',
      payload: config.data,
    };
  }

  try {
    const response = await http.fetch(url, options);
    
    return {
      data: response.data,
      status: response.status,
      statusText: response.ok ? 'OK' : 'Error',
      headers: response.headers || {},
      config: config,
    } as AxiosResponse;
  } catch (error: any) {
    throw {
      message: error.message || 'Network request failed',
      code: 'ERR_NETWORK',
      config: config,
      response: error.response,
    };
  }
}

// Create axios instance with custom config
export const api = axios.create({
  baseURL: API_URL,
  timeout: 30000,
  headers: {
    'Content-Type': 'application/json',
  },
  validateStatus: (status) => status < 500,
});

// Override adapter for Tauri
if (isTauri) {
  (api.defaults as any).adapter = async (config: AxiosRequestConfig) => {
    return tauriHttpRequest(config);
  };
}

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
