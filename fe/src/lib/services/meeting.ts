import { api } from './api';

export interface Meeting {
    id: string;
    title: string;
    channel_id: string | null;
    host_id: string | null;
    host_name: string | null;
    status: 'active' | 'ended';
    created_at: string;
    ended_at: string | null;
}

export interface MeetingParticipant {
    user_id: string;
    display_name: string;
    email: string;
    role: 'host' | 'participant';
    joined_at: string;
}

export interface MeetingAttachment {
    id: string;
    meeting_id: string;
    uploaded_by: string | null;
    uploaded_by_name: string | null;
    category: 'document' | 'recording';
    file_name: string;
    content_type: string;
    file_size: number;
    created_at: string;
}

export interface MeetingMinutes {
    meeting_id: string;
    content: string;
    updated_by: string | null;
    updated_by_name: string | null;
    updated_at: string;
}

export class MeetingApiError extends Error {
    status: number;
    constructor(message: string, status: number) {
        super(message);
        this.name = 'MeetingApiError';
        this.status = status;
    }
}

export const meetingService = {
    async createMeeting(data: { title?: string; channel_id?: string }): Promise<Meeting> {
        const res = await api.post('/meetings', data);
        if (!res.ok) throw new Error('Failed to create meeting');
        return res.json();
    },

    async getMeeting(meetingId: string): Promise<Meeting> {
        const res = await api.get(`/meetings/${meetingId}`);
        if (!res.ok) throw new Error('Failed to get meeting');
        return res.json();
    },

    async listMyMeetings(): Promise<Meeting[]> {
        const res = await api.get('/meetings');
        if (!res.ok) throw new Error('Failed to load meetings');
        return res.json();
    },

    async joinMeeting(meetingId: string): Promise<MeetingParticipant[]> {
        const res = await api.post(`/meetings/${meetingId}/join`, {});
        if (!res.ok) throw new Error('Failed to join meeting');
        return res.json();
    },

    async leaveMeeting(meetingId: string): Promise<void> {
        const res = await api.post(`/meetings/${meetingId}/leave`, {});
        if (!res.ok) throw new Error('Failed to leave meeting');
    },

    async endMeeting(meetingId: string): Promise<void> {
        const res = await api.post(`/meetings/${meetingId}/end`, {});
        if (!res.ok) throw new Error('Failed to end meeting');
    },

    async listParticipants(meetingId: string): Promise<MeetingParticipant[]> {
        const res = await api.get(`/meetings/${meetingId}/participants`);
        if (!res.ok) throw new Error('Failed to list participants');
        return res.json();
    },

    async listAttachments(meetingId: string): Promise<MeetingAttachment[]> {
        const res = await api.get(`/meetings/${meetingId}/attachments`);
        if (!res.ok) throw new MeetingApiError('Failed to load attachments', res.status);
        return res.json();
    },

    async uploadAttachment(
        meetingId: string,
        file: File,
        category?: 'document' | 'recording'
    ): Promise<MeetingAttachment> {
        const formData = new FormData();
        formData.append('file', file);
        if (category) formData.append('category', category);
        const res = await api.postForm(`/meetings/${meetingId}/attachments`, formData);
        if (!res.ok) {
            const err = await res.json().catch(() => ({}));
            throw new MeetingApiError(err.message || 'Failed to upload attachment', res.status);
        }
        return res.json();
    },

    async deleteAttachment(meetingId: string, attachmentId: string): Promise<void> {
        const res = await api.delete(`/meetings/${meetingId}/attachments/${attachmentId}`);
        if (!res.ok) throw new MeetingApiError('Failed to delete attachment', res.status);
    },

    async downloadAttachment(meetingId: string, attachmentId: string, fileName: string): Promise<void> {
        const res = await api.get(`/meetings/${meetingId}/attachments/${attachmentId}/download`);
        if (!res.ok) throw new MeetingApiError('Failed to download attachment', res.status);
        const blob = await res.blob();
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.href = url;
        link.download = fileName;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        URL.revokeObjectURL(url);
    },

    async getMinutes(meetingId: string): Promise<MeetingMinutes> {
        const res = await api.get(`/meetings/${meetingId}/minutes`);
        if (!res.ok) throw new MeetingApiError('Failed to load minutes', res.status);
        return res.json();
    },

    async updateMinutes(meetingId: string, content: string): Promise<MeetingMinutes> {
        const res = await api.put(`/meetings/${meetingId}/minutes`, { content });
        if (!res.ok) throw new MeetingApiError('Failed to save minutes', res.status);
        return res.json();
    }
};
