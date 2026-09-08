import { api } from './api';

export interface GithubConnection {
    connected: boolean;
    github_username: string | null;
    connected_at: string | null;
}

export const githubConnectorService = {
    async getConnection(): Promise<GithubConnection> {
        const res = await api.get('/integrations/github');
        if (!res.ok) throw new Error('Failed to fetch GitHub connection');
        return res.json();
    },

    async connect(token: string): Promise<GithubConnection> {
        const res = await api.post('/integrations/github', { token });
        if (!res.ok) {
            const err = await res.json().catch(() => ({}));
            throw new Error((err as any).message || 'Failed to connect GitHub account');
        }
        return res.json();
    },

    async disconnect(): Promise<void> {
        const res = await api.delete('/integrations/github');
        if (!res.ok) throw new Error('Failed to disconnect GitHub account');
    },
};
