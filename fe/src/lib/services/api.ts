import { PUBLIC_API_URL } from '$env/static/public';

interface RequestOptions extends RequestInit {
    headers?: Record<string, string>;
}

async function customFetch(url: string, options: RequestOptions = {}): Promise<Response> {
    const fullUrl = url.startsWith('http') ? url : `${PUBLIC_API_URL}${url}`;

    // Add Authorization header if token exists
    const token = typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;
    if (token) {
        options.headers = {
            ...options.headers,
            'Authorization': `Bearer ${token}`
        };
    }

    let response = await fetch(fullUrl, options);

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
            const refreshRes = await fetch(`${PUBLIC_API_URL}/auth/refresh`, {
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
                // Refresh failed
                logout();
            }
        } catch (error) {
            console.error('Token refresh failed', error);
            logout();
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
