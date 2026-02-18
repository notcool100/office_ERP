import { api } from './api';

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

export const departmentService = {
    async getAll(isActive?: boolean): Promise<Department[]> {
        const params = new URLSearchParams();
        if (isActive !== undefined) {
            params.append('is_active', String(isActive));
        }
        const query = params.toString();
        const response = await api.get(`/departments${query ? `?${query}` : ''}`);
        if (!response.ok) throw new Error('Failed to fetch departments');
        return await response.json();
    },

    async getById(id: string): Promise<Department> {
        const response = await api.get(`/departments/${id}`);
        if (!response.ok) throw new Error('Failed to fetch department');
        return await response.json();
    },

    async create(data: CreateDepartmentDto): Promise<Department> {
        const response = await api.post('/departments', data);
        if (!response.ok) throw new Error('Failed to create department');
        return await response.json();
    },

    async update(id: string, data: UpdateDepartmentDto): Promise<Department> {
        const response = await api.put(`/departments/${id}`, data);
        if (!response.ok) throw new Error('Failed to update department');
        return await response.json();
    },

    async delete(id: string): Promise<void> {
        const response = await api.delete(`/departments/${id}`);
        if (!response.ok) throw new Error('Failed to delete department');
    }
};
