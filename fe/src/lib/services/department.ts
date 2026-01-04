import { PUBLIC_API_URL } from '$env/static/public';

export interface Department {
    id: string;
    name: string;
    description: string | null;
    is_active: boolean;
    created_at: string;
    updated_at: string;
}

export interface CreateDepartmentDto {
    name: string;
    description?: string;
}

export interface UpdateDepartmentDto {
    name?: string;
    description?: string;
    is_active?: boolean;
}

const getAuthHeaders = () => {
    const token = typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;
    return {
        'Content-Type': 'application/json',
        ...(token && { Authorization: `Bearer ${token}` })
    };
};

export const departmentService = {
    async getAll(isActive?: boolean): Promise<Department[]> {
        const params = new URLSearchParams();
        if (isActive !== undefined) {
            params.append('is_active', String(isActive));
        }
        const response = await fetch(`${PUBLIC_API_URL}/departments?${params.toString()}`, {
            headers: getAuthHeaders()
        });
        if (!response.ok) throw new Error('Failed to fetch departments');
        return await response.json();
    },

    async getById(id: string): Promise<Department> {
        const response = await fetch(`${PUBLIC_API_URL}/departments/${id}`, {
            headers: getAuthHeaders()
        });
        if (!response.ok) throw new Error('Failed to fetch department');
        return await response.json();
    },

    async create(data: CreateDepartmentDto): Promise<Department> {
        const response = await fetch(`${PUBLIC_API_URL}/departments`, {
            method: 'POST',
            headers: getAuthHeaders(),
            body: JSON.stringify(data)
        });
        if (!response.ok) throw new Error('Failed to create department');
        return await response.json();
    },

    async update(id: string, data: UpdateDepartmentDto): Promise<Department> {
        const response = await fetch(`${PUBLIC_API_URL}/departments/${id}`, {
            method: 'PUT',
            headers: getAuthHeaders(),
            body: JSON.stringify(data)
        });
        if (!response.ok) throw new Error('Failed to update department');
        return await response.json();
    },

    async delete(id: string): Promise<void> {
        const response = await fetch(`${PUBLIC_API_URL}/departments/${id}`, {
            method: 'DELETE',
            headers: getAuthHeaders()
        });
        if (!response.ok) throw new Error('Failed to delete department');
    }
};
