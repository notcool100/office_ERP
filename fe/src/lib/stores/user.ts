import { writable } from 'svelte/store';
import { authService } from '../services/auth.service';

interface User {
    id: string;
    userName: string;
    email: string;
    phone: string;
    personId: string;
    isAdmin: boolean;
    createdAt: string;
    // Add other fields as needed based on backend User struct
}

function createUserStore() {
    const { subscribe, set, update } = writable<{
        user: User | null;
        isAuthenticated: boolean;
        loading: boolean;
    }>({
        user: null,
        isAuthenticated: false,
        loading: true
    });

    return {
        subscribe,
        login: async (userName: string, password: string) => {
            update(state => ({ ...state, loading: true }));
            try {
                const data = await authService.login(userName, password);
                if (typeof window !== 'undefined') {
                    localStorage.setItem('access_token', data.accessToken);
                    localStorage.setItem('refresh_token', data.refreshToken);
                }

                // Fetch profile immediate after login
                const user = await authService.getProfile();
                set({ user, isAuthenticated: true, loading: false });
                return true;
            } catch (error) {
                set({ user: null, isAuthenticated: false, loading: false });
                throw error;
            }
        },
        logout: () => {
            authService.logout();
            set({ user: null, isAuthenticated: false, loading: false });
        },
        init: async () => {
            if (typeof window === 'undefined') return;

            const token = localStorage.getItem('access_token');
            if (!token) {
                set({ user: null, isAuthenticated: false, loading: false });
                return;
            }

            try {
                const user = await authService.getProfile();
                set({ user, isAuthenticated: true, loading: false });
            } catch (error) {
                // Token might be invalid or expired and refresh failed
                set({ user: null, isAuthenticated: false, loading: false });
                authService.logout();
            }
        }
    };
}

export const userStore = createUserStore();
