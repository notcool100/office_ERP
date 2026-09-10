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

// The backend rotates the refresh token on every use (old one is invalidated
// as soon as a new one is issued). If several requests 401 at once — e.g.
// after the access token expires from being idle and a page fires a few
// parallel calls on load — each one independently calling /auth/refresh
// would race: the first rotates the token and succeeds, every other call
// still holds the now-stale refresh token and gets rejected, which used to
// trigger logout() and wipe the *valid* tokens the first call had just
// stored. Sharing a single in-flight refresh across all callers avoids that.
let refreshInFlight: Promise<{ accessToken: string; refreshToken: string } | null> | null = null;

function refreshTokens(): Promise<{ accessToken: string; refreshToken: string } | null> {
    if (!refreshInFlight) {
        refreshInFlight = (async () => {
            const refreshToken = localStorage.getItem('refresh_token');
            if (!refreshToken) return null;

            try {
                const refreshRes = await fetch(buildApiUrl('/auth/refresh'), {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ refreshToken })
                });

                if (!refreshRes.ok) {
                    console.error('Token refresh response not OK', refreshRes.status);
                    return null;
                }

                const data = await refreshRes.json();
                localStorage.setItem('access_token', data.accessToken);
                localStorage.setItem('refresh_token', data.refreshToken);
                return data;
            } catch (error) {
                console.error('Token refresh network failed', error);
                return null;
            }
        })().finally(() => {
            refreshInFlight = null;
        });
    }
    return refreshInFlight;
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

        if (!localStorage.getItem('refresh_token')) {
            // No refresh token, force logout
            logout();
            return response;
        }

        const tokens = await refreshTokens();
        if (!tokens) {
            logout();
            return response;
        }

        // Retry original request with the (possibly shared) new token
        options.headers = {
            ...options.headers,
            'Authorization': `Bearer ${tokens.accessToken}`
        };
        response = await fetch(fullUrl, options);
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
    postForm: (url: string, formData: FormData, options?: RequestOptions) =>
        customFetch(url, { ...options, method: 'POST', body: formData }),
    put: (url: string, body: any, options?: RequestOptions) => customFetch(url, { ...options, method: 'PUT', body: JSON.stringify(body), headers: { ...options?.headers, 'Content-Type': 'application/json' } }),
    delete: (url: string, options?: RequestOptions) => customFetch(url, { ...options, method: 'DELETE' })
};
