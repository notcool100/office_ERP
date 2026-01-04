import { writable } from 'svelte/store';
import { PUBLIC_API_URL } from '$env/static/public';

export const loggedIn = writable(false);
export const accessToken = writable<string | null>(null);

if (typeof window !== 'undefined') {
    const token = localStorage.getItem('access_token');
    if (token) {
        loggedIn.set(true);
        accessToken.set(token);
    }
}

export async function authenticate(userName: string, password: string) {
    const res = await fetch(`${PUBLIC_API_URL}/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ userName, password })
    });

    if (!res.ok) {
        throw new Error('Login failed');
    }

    const data = await res.json();
    loggedIn.set(true);
    accessToken.set(data.access_token);

    if (typeof window !== 'undefined') {
        localStorage.setItem('access_token', data.access_token);
        localStorage.setItem('refresh_token', data.refresh_token);
    }
}

export function logout() {
    loggedIn.set(false);
    accessToken.set(null);
    if (typeof window !== 'undefined') {
        localStorage.removeItem('access_token');
        localStorage.removeItem('refresh_token');
    }
}
