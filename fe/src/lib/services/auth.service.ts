import { api } from './api';

export const authService = {
    async login(userName: string, password: string) {
        const response = await api.post('/auth/login', { userName, password });
        if (!response.ok) {
            throw new Error('Login failed');
        }
        return await response.json();
    },

    async getProfile() {
        const response = await api.get('/auth/me');
        if (!response.ok) {
            throw new Error('Failed to fetch profile');
        }
        return await response.json();
    },

    logout() {
        if (typeof window !== 'undefined') {
            localStorage.removeItem('access_token');
            localStorage.removeItem('refresh_token');
            window.location.href = '/login';
        }
    }
};
