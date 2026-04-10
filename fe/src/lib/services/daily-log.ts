import { api } from './api';
import type { 
    DailyLog, 
    CreateDailyLogDto, 
    UpdateDailyLogDto, 
    ListDailyLogQuery 
} from '$lib/types/daily-log';

export const dailyLogService = {
    async create(dto: CreateDailyLogDto): Promise<DailyLog> {
        const response = await api.post('/daily-logs', dto);
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to create log');
        }
        const res = await response.json();
        return res.data;
    },

    async update(id: string, dto: UpdateDailyLogDto): Promise<DailyLog> {
        const response = await api.put(`/daily-logs/${id}`, dto);
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to update log');
        }
        const res = await response.json();
        return res.data;
    },

    async delete(id: string): Promise<void> {
        const response = await api.delete(`/daily-logs/${id}`);
        if (!response.ok) throw new Error('Failed to delete log');
    },

    async list(query?: ListDailyLogQuery): Promise<DailyLog[]> {
        const params = new URLSearchParams();
        if (query) {
            if (query.user_id) params.append('user_id', query.user_id);
            if (query.start_date) params.append('start_date', query.start_date);
            if (query.end_date) params.append('end_date', query.end_date);
        }
        const queryStr = params.toString() ? `?${params.toString()}` : '';
        const response = await api.get(`/daily-logs${queryStr}`);
        if (!response.ok) throw new Error('Failed to fetch logs');
        const res = await response.json();
        return res.data;
    }
};
