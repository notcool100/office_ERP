export interface Department {
    id: string;
    name: string;
    description: string | null;
    is_active: boolean;
    created_at: string;
    updated_at: string;
}

export interface CreateDepartmentRequest {
    name: string;
    description?: string;
}

export interface UpdateDepartmentRequest {
    name?: string;
    description?: string;
    is_active?: boolean;
}
