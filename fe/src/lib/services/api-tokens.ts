import { api } from './api';

export interface ApiToken {
    id: string;
    name: string;
    tokenPrefix: string;
    expiresAt: string | null;
    lastUsedAt: string | null;
    createdAt: string;
}

export interface CreatedApiToken extends ApiToken {
    /** Returned once, at creation. */
    secret: string;
}

export const apiTokenService = {
    async list(): Promise<ApiToken[]> {
        const res = await api.get('/api-tokens');
        if (!res.ok) throw new Error('Failed to fetch API tokens');
        return res.json();
    },

    async create(name: string, expiresInDays?: number): Promise<CreatedApiToken> {
        const res = await api.post('/api-tokens', { name, expiresInDays });
        if (!res.ok) {
            const err = await res.json().catch(() => ({}));
            throw new Error((err as any).message || 'Failed to create API token');
        }
        return res.json();
    },

    async revoke(id: string): Promise<void> {
        const res = await api.delete(`/api-tokens/${id}`);
        if (!res.ok) throw new Error('Failed to revoke API token');
    },
};
