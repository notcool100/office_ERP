import { api } from './api';

export type ContentType = 'post' | 'story' | 'reel' | 'blog' | 'email' | 'ad' | 'video' | 'campaign';
export type Channel = 'instagram' | 'facebook' | 'linkedin' | 'twitter' | 'youtube' | 'tiktok' | 'email' | 'blog' | 'google_ads';
export type ContentStatus = 'draft' | 'in_review' | 'scheduled' | 'published' | 'cancelled';

export interface ContentCalendarItem {
    id: string;
    title: string;
    description: string | null;
    content_type: ContentType;
    channel: Channel;
    scheduled_date: string;
    scheduled_time: string | null;
    status: ContentStatus;
    campaign_name: string | null;
    assigned_to: string | null;
    assigned_to_name: string | null;
    created_by: string;
    created_by_name: string | null;
    notes: string | null;
    created_at: string;
    updated_at: string;
}

export interface CreateContentItemDto {
    title: string;
    description?: string;
    content_type: string;
    channel: string;
    scheduled_date: string;
    scheduled_time?: string;
    status?: string;
    campaign_name?: string;
    assigned_to?: string;
    notes?: string;
}

export interface UpdateContentItemDto {
    title?: string;
    description?: string;
    content_type?: string;
    channel?: string;
    scheduled_date?: string;
    scheduled_time?: string;
    status?: string;
    campaign_name?: string;
    assigned_to?: string;
    notes?: string;
}

export interface ListContentItemsQuery {
    start_date?: string;
    end_date?: string;
    status?: string;
    channel?: string;
}

export const CHANNELS: { value: Channel; label: string; color: string }[] = [
    { value: 'instagram',  label: 'Instagram',   color: '#E1306C' },
    { value: 'facebook',   label: 'Facebook',    color: '#1877F2' },
    { value: 'linkedin',   label: 'LinkedIn',    color: '#0A66C2' },
    { value: 'twitter',    label: 'X / Twitter', color: '#000000' },
    { value: 'youtube',    label: 'YouTube',     color: '#FF0000' },
    { value: 'tiktok',     label: 'TikTok',      color: '#69C9D0' },
    { value: 'email',      label: 'Email',       color: '#F59E0B' },
    { value: 'blog',       label: 'Blog',        color: '#10B981' },
    { value: 'google_ads', label: 'Google Ads',  color: '#EA4335' },
];

export const CONTENT_TYPES: { value: ContentType; label: string }[] = [
    { value: 'post',     label: 'Post' },
    { value: 'story',    label: 'Story' },
    { value: 'reel',     label: 'Reel / Short' },
    { value: 'blog',     label: 'Blog Article' },
    { value: 'email',    label: 'Email Newsletter' },
    { value: 'ad',       label: 'Ad / Sponsored' },
    { value: 'video',    label: 'Video' },
    { value: 'campaign', label: 'Campaign' },
];

export const STATUSES: { value: ContentStatus; label: string; badge: string }[] = [
    { value: 'draft',      label: 'Draft',      badge: 'badge-neutral' },
    { value: 'in_review',  label: 'In Review',  badge: 'badge-warning' },
    { value: 'scheduled',  label: 'Scheduled',  badge: 'badge-info' },
    { value: 'published',  label: 'Published',  badge: 'badge-success' },
    { value: 'cancelled',  label: 'Cancelled',  badge: 'badge-error' },
];

export function getChannelColor(channel: string): string {
    return CHANNELS.find(c => c.value === channel)?.color ?? '#888';
}

export function getChannelLabel(channel: string): string {
    return CHANNELS.find(c => c.value === channel)?.label ?? channel;
}

export function getStatusBadge(status: string): string {
    return STATUSES.find(s => s.value === status)?.badge ?? 'badge-neutral';
}

export function getStatusLabel(status: string): string {
    return STATUSES.find(s => s.value === status)?.label ?? status;
}

export const contentCalendarService = {
    async list(query: ListContentItemsQuery = {}): Promise<ContentCalendarItem[]> {
        const params = new URLSearchParams();
        if (query.start_date) params.append('start_date', query.start_date);
        if (query.end_date)   params.append('end_date',   query.end_date);
        if (query.status)     params.append('status',     query.status);
        if (query.channel)    params.append('channel',    query.channel);
        const qs = params.toString();
        const res = await api.get(`/content-calendar${qs ? `?${qs}` : ''}`);
        if (!res.ok) throw new Error('Failed to fetch content calendar');
        return res.json();
    },

    async create(data: CreateContentItemDto): Promise<ContentCalendarItem> {
        const res = await api.post('/content-calendar', data);
        if (!res.ok) throw new Error('Failed to create content item');
        return res.json();
    },

    async update(id: string, data: UpdateContentItemDto): Promise<ContentCalendarItem> {
        const res = await api.put(`/content-calendar/${id}`, data);
        if (!res.ok) throw new Error('Failed to update content item');
        return res.json();
    },

    async delete(id: string): Promise<void> {
        const res = await api.delete(`/content-calendar/${id}`);
        if (!res.ok) throw new Error('Failed to delete content item');
    },
};
