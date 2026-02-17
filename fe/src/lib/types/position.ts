export interface Position {
    id: string;
    name: string;
    description: string | null;
    department_id: string | null;
    is_active: boolean;
    created_at: string;
    updated_at: string;
}

export interface CreatePositionRequest {
    name: string;
    description?: string;
    department_id: string;
}

export interface UpdatePositionRequest {
    name?: string;
    description?: string;
    department_id?: string;
    is_active?: boolean;
}
