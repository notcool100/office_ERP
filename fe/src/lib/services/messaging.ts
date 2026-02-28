import { api } from './api';

export interface Channel {
    id: string;
    name: string;
    description?: string;
    is_private: boolean;
    created_at: string;
    created_by?: string;
}

export interface Message {
    id: string;
    channel_id: string;
    sender_id?: string;
    sender_name?: string;
    content: string;
    created_at: string;
}

export const messagingService = {
    async listChannels(): Promise<Channel[]> {
        const res = await api.get('/messaging/channels');
        if (!res.ok) throw new Error('Failed to load channels');
        return res.json();
    },

    async getChannel(channelId: string): Promise<Channel> {
        const res = await api.get(`/messaging/channels/${channelId}`);
        if (!res.ok) throw new Error('Failed to get channel');
        return res.json();
    },

    async createChannel(data: { name: string; description?: string; is_private: boolean; members?: string[] }): Promise<Channel> {
        const res = await api.post('/messaging/channels', data);
        if (!res.ok) throw new Error('Failed to create channel');
        return res.json();
    },

    async addMember(channelId: string, userId: string): Promise<void> {
        const res = await api.post(`/messaging/channels/${channelId}/members`, { user_id: userId });
        if (!res.ok) throw new Error('Failed to add member');
    },

    async listMessages(channelId: string): Promise<Message[]> {
        const res = await api.get(`/messaging/channels/${channelId}/messages`);
        if (!res.ok) throw new Error('Failed to load messages');
        return res.json();
    },

    async sendMessage(channelId: string, content: string): Promise<Message> {
        const res = await api.post(`/messaging/channels/${channelId}/messages`, { content });
        if (!res.ok) throw new Error('Failed to send message');
        return res.json();
    },

    async updateChannel(channelId: string, data: { name?: string; description?: string }): Promise<Channel> {
        const res = await api.put(`/messaging/channels/${channelId}`, data);
        if (!res.ok) throw new Error('Failed to update channel');
        return res.json();
    },

    async getChannelMembers(channelId: string): Promise<any[]> {
        const res = await api.get(`/messaging/channels/${channelId}/members`);
        if (!res.ok) throw new Error('Failed to list members');
        return res.json();
    },

    async removeMember(channelId: string, userId: string): Promise<void> {
        const res = await api.delete(`/messaging/channels/${channelId}/members/${userId}`);
        if (!res.ok) throw new Error('Failed to remove member');
    }
};
