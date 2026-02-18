import { api } from './api';
import type {
    CalendarEvent,
    CreateCalendarEventDto,
    UpdateCalendarEventDto,
} from '$lib/types/calendar';

export const calendarService = {
    async list(startDate?: string, endDate?: string): Promise<CalendarEvent[]> {
        const params = new URLSearchParams();
        if (startDate) params.append('start_date', startDate);
        if (endDate) params.append('end_date', endDate);
        const query = params.toString();
        const response = await api.get(`/calendar/events${query ? `?${query}` : ''}`);
        if (!response.ok) throw new Error('Failed to fetch events');
        return await response.json();
    },

    async create(data: CreateCalendarEventDto): Promise<CalendarEvent> {
        const response = await api.post('/calendar/events', data);
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to create event');
        }
        return await response.json();
    },

    async update(id: string, data: UpdateCalendarEventDto): Promise<CalendarEvent> {
        const response = await api.put(`/calendar/events/${id}`, data);
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to update event');
        }
        return await response.json();
    },

    async delete(id: string): Promise<void> {
        const response = await api.delete(`/calendar/events/${id}`);
        if (!response.ok) throw new Error('Failed to delete event');
    },
};
