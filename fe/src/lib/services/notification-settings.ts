import { api } from './api';

export interface NotificationSetting {
    key: string;
    label: string;
    description: string | null;
    emails: string[];
}

export const notificationSettingsService = {
    async list(): Promise<NotificationSetting[]> {
        const res = await api.get('/notification-settings');
        if (!res.ok) throw new Error('Failed to fetch notification settings');
        return res.json();
    },

    async get(key: string): Promise<NotificationSetting> {
        const res = await api.get(`/notification-settings/${key}`);
        if (!res.ok) throw new Error('Failed to fetch notification setting');
        return res.json();
    },

    async update(key: string, emails: string[]): Promise<NotificationSetting> {
        const res = await api.put(`/notification-settings/${key}`, { emails });
        if (!res.ok) {
            const err = await res.json().catch(() => ({}));
            throw new Error((err as any).message || 'Failed to update notification setting');
        }
        return res.json();
    },
};
