export interface DailyLog {
    id: string;
    user_id: string;
    user_name: string;
    log_date: string;
    content: string;
    created_at: string;
    updated_at: string;
    links: DailyLogLink[];
}

export interface DailyLogLink {
    id: string;
    card_id: string;
    card_key: string;
    card_title: string;
}

export interface CreateDailyLogDto {
    log_date: string;
    content: string;
    card_ids?: string[];
}

export interface UpdateDailyLogDto {
    log_date?: string;
    content?: string;
    card_ids?: string[];
}

export interface ListDailyLogQuery {
    user_id?: string;
    start_date?: string;
    end_date?: string;
}
