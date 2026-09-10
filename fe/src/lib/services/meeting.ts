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
    }
};
