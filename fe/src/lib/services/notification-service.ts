import { PUBLIC_API_URL } from '$env/static/public';
import { notifications } from '../stores/notification';
import { userStore } from '../stores/user';
import { get } from 'svelte/store';

let socket: WebSocket | null = null;
let reconnectTimer: any = null;

export const notificationService = {
    connect() {
        if (socket || typeof window === 'undefined') return;

        const user = get(userStore).user;
        if (!user) {
            console.warn('Cannot connect to notifications: No user logged in');
            return;
        }

        const wsUrl = PUBLIC_API_URL.replace('http', 'ws') + '/ws/notifications';

        const token = localStorage.getItem('access_token');
        socket = new WebSocket(`${wsUrl}?token=${token}`);

        socket.onmessage = (event) => {
            try {
                const data = JSON.parse(event.data);
                if (data.message_type === 'new_message') {
                    const msg = data.payload;

                    const currentPath = window.location.pathname;
                    if (!currentPath.includes(msg.channel_id)) {
                        notifications.add({
                            title: `New message from ${msg.sender_name || 'User'}`,
                            message: msg.content,
                            type: 'info',
                            duration: 5000
                        });

                        this.updateTabTitle();
                    }
                }
            } catch (e) {
                console.error('Error parsing notification message:', e);
            }
        };

        socket.onclose = () => {
            socket = null;
            console.log('Notification WS closed, reconnecting in 5s...');
            clearTimeout(reconnectTimer);
            reconnectTimer = setTimeout(() => this.connect(), 5000);
        };

        socket.onerror = (err) => {
            console.error('Notification WS error:', err);
        };
    },

    disconnect() {
        if (socket) {
            socket.close();
            socket = null;
        }
        clearTimeout(reconnectTimer);
    },

    updateTabTitle() {
        if (typeof document === 'undefined') return;
        const originalTitle = document.title.replace(/^\(\d+\)\s/, '');
        document.title = `(*) ${originalTitle}`;

        window.addEventListener('focus', () => {
            document.title = originalTitle;
        }, { once: true });
    }
};
