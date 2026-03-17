import { writable } from 'svelte/store';

export type NotificationType = 'info' | 'success' | 'warning' | 'error';

export interface Notification {
    id: string;
    message: string;
    type: NotificationType;
    title?: string;
    duration?: number;
}

function createNotificationStore() {
    const { subscribe, update } = writable<Notification[]>([]);

    return {
        subscribe,
        add: (notification: Omit<Notification, 'id'>) => {
            const id = Math.random().toString(36).substring(2, 9);
            const newNotification = { ...notification, id };

            update((n) => [...n, newNotification]);

            if (notification.duration !== 0) {
                setTimeout(() => {
                    update((n) => n.filter((item) => item.id !== id));
                }, notification.duration || 5000);
            }
        },
        remove: (id: string) => {
            update((n) => n.filter((item) => item.id !== id));
        },
    };
}

export const notifications = createNotificationStore();
