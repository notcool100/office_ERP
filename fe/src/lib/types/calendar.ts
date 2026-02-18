export type CalendarScope = 'personal' | 'department' | 'company';

export interface CalendarEvent {
    id: string;
    title: string;
    description: string | null;
    location: string | null;
    start_at: string;
    end_at: string;
    all_day: boolean;
    scope: CalendarScope | string;
    department_id: string | null;
    created_by: string | null;
    created_at: string;
    updated_at: string;
}

export interface CreateCalendarEventDto {
    title: string;
    description?: string;
    location?: string;
    start_at: string;
    end_at: string;
    all_day?: boolean;
    scope?: CalendarScope;
    department_id?: string;
}

export interface UpdateCalendarEventDto {
    title?: string;
    description?: string;
    location?: string;
    start_at?: string;
    end_at?: string;
    all_day?: boolean;
    scope?: CalendarScope;
    department_id?: string;
}
