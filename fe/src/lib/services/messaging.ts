import { api, buildApiUrl } from './api';

export interface Channel {
    id: string;
    name: string;
    description?: string;
    is_private: boolean;
    created_at: string;
    created_by?: string;
    dm_other_user_name?: string | null;
}

export interface ChannelMember {
    id: string;
    display_name: string;
    email: string;
}

export interface Attachment {
    id: string;
    message_id: string;
    file_name: string;
    content_type: string;
    file_size: number;
    is_image: boolean;
    created_at: string;
}

export interface Message {
    id: string;
    channel_id: string;
    sender_id?: string;
    sender_name?: string;
    content: string;
    created_at: string;
    attachments: Attachment[];
}

export interface ChannelMediaItem extends Attachment {
    sender_id?: string;
    sender_name?: string;
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

    async sendMessage(
        channelId: string,
        content: string,
        files: File[] = [],
        parentId?: string,
    ): Promise<Message> {
        const formData = new FormData();
        formData.set('content', content);
        if (parentId) formData.set('parentId', parentId);
        for (const file of files) {
            formData.append('files', file);
        }
        const res = await api.postForm(`/messaging/channels/${channelId}/messages`, formData);
        if (!res.ok) {
            const text = await res.text();
            throw new Error(text || 'Failed to send message');
        }
        return res.json();
    },

    async listChannelMedia(channelId: string): Promise<ChannelMediaItem[]> {
        const res = await api.get(`/messaging/channels/${channelId}/media`);
        if (!res.ok) throw new Error('Failed to load shared media');
        return res.json();
    },

    attachmentUrl(channelId: string, attachmentId: string): string {
        return buildApiUrl(`/messaging/channels/${channelId}/attachments/${attachmentId}`);
    },

    async fetchAttachmentBlob(channelId: string, attachmentId: string): Promise<string> {
        const token =
            typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;
        const res = await fetch(this.attachmentUrl(channelId, attachmentId), {
            headers: token ? { Authorization: `Bearer ${token}` } : {},
        });
        if (!res.ok) throw new Error('Failed to load attachment');
        const blob = await res.blob();
        return URL.createObjectURL(blob);
    },

    async updateChannel(channelId: string, data: { name?: string; description?: string }): Promise<Channel> {
        const res = await api.put(`/messaging/channels/${channelId}`, data);
        if (!res.ok) throw new Error('Failed to update channel');
        return res.json();
    },

    async getChannelMembers(channelId: string): Promise<ChannelMember[]> {
        const res = await api.get(`/messaging/channels/${channelId}/members`);
        if (!res.ok) throw new Error('Failed to list members');
        return res.json();
    },

    async removeMember(channelId: string, userId: string): Promise<void> {
        const res = await api.delete(`/messaging/channels/${channelId}/members/${userId}`);
        if (!res.ok) throw new Error('Failed to remove member');
    }
};
