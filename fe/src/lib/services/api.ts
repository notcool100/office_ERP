import { PUBLIC_API_URL } from '$env/static/public';

const RAW_API_BASE = (PUBLIC_API_URL || '').trim();
const API_BASE = RAW_API_BASE.endsWith('/')
    ? RAW_API_BASE.slice(0, -1)
    : RAW_API_BASE;

function normalizePath(path: string) {
    return path.startsWith('/') ? path : `/${path}`;
}

export function buildApiUrl(url: string) {
    if (url.startsWith('http')) return url;
    if (API_BASE && url.startsWith(API_BASE)) return url;
    const path = normalizePath(url);
    if (!API_BASE) return path;
    return `${API_BASE}${path}`;
}

export function buildWsUrl(path: string) {
    const normalizedPath = normalizePath(path);

    if (API_BASE.startsWith('http')) {
        return `${API_BASE.replace(/^http/, 'ws')}${normalizedPath}`;
    }

    if (typeof window !== 'undefined') {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        return `${protocol}//${window.location.host}${API_BASE}${normalizedPath}`;
    }

    return `${API_BASE}${normalizedPath}`;
}

interface RequestOptions extends RequestInit {
    headers?: Record<string, string>;
}

async function customFetch(url: string, options: RequestOptions = {}): Promise<Response> {
    const fullUrl = buildApiUrl(url);

    // Add Authorization header if token exists
    const token = typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;
    if (token) {
        options.headers = {
            ...options.headers,
            'Authorization': `Bearer ${token}`
        };
    }

    let response: Response;
    try {
        response = await fetch(fullUrl, options);
    } catch (error) {
        // Network error - potentially server down
        console.error('Fetch failed:', error);
        throw error;
    }

    // Handle 401 (Unauthorized) - Attempt Refresh
    if (response.status === 401) {
        if (typeof window === 'undefined') return response;

        const refreshToken = localStorage.getItem('refresh_token');
        if (!refreshToken) {
            // No refresh token, force logout
            logout();
            return response;
        }

        try {
            // Attempt to refresh token
            const refreshRes = await fetch(buildApiUrl('/auth/refresh'), {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ refreshToken })
            });

            if (refreshRes.ok) {
                const data = await refreshRes.json();
                localStorage.setItem('access_token', data.accessToken);
                localStorage.setItem('refresh_token', data.refreshToken);

                // Retry original request with new token
                options.headers = {
                    ...options.headers,
                    'Authorization': `Bearer ${data.accessToken}`
                };
                response = await fetch(fullUrl, options);
            } else {
                // Refresh failed (could be 401, 403, 502, etc.)
                console.error('Token refresh response not OK', refreshRes.status);
                logout();
                return refreshRes; // Return the refresh error response
            }
        } catch (error) {
            console.error('Token refresh network failed', error);
            logout();
            throw error;
        }
    }

    return response;
}

function logout() {
    if (typeof window !== 'undefined') {
        localStorage.removeItem('access_token');
        localStorage.removeItem('refresh_token');
        window.location.href = '/login';
    }
}

export const api = {
    get: (url: string, options?: RequestOptions) => customFetch(url, { ...options, method: 'GET' }),
    post: (url: string, body: any, options?: RequestOptions) => customFetch(url, { ...options, method: 'POST', body: JSON.stringify(body), headers: { ...options?.headers, 'Content-Type': 'application/json' } }),
    put: (url: string, body: any, options?: RequestOptions) => customFetch(url, { ...options, method: 'PUT', body: JSON.stringify(body), headers: { ...options?.headers, 'Content-Type': 'application/json' } }),
    delete: (url: string, options?: RequestOptions) => customFetch(url, { ...options, method: 'DELETE' })
};
