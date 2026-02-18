import { api } from './api';

export interface Position {
    id: string;
    name: string;
    description: string | null;
    department_id: string | null;
    is_active: boolean;
    created_at: string;
    updated_at: string;
}

export interface CreatePositionDto {
    name: string;
    description?: string;
    department_id: string;
}

export interface UpdatePositionDto {
    name?: string;
    description?: string;
    department_id?: string;
    is_active?: boolean;
}

export const positionService = {
    async getAll(isActive?: boolean): Promise<Position[]> {
        const params = new URLSearchParams();
        if (isActive !== undefined) {
            params.append('is_active', String(isActive));
        }
        const query = params.toString();
        const response = await api.get(`/positions${query ? `?${query}` : ''}`);
        if (!response.ok) throw new Error('Failed to fetch positions');
        return await response.json();
    },

    async getById(id: string): Promise<Position> {
        const response = await api.get(`/positions/${id}`);
        if (!response.ok) throw new Error('Failed to fetch position');
        return await response.json();
    },

    async create(data: CreatePositionDto): Promise<Position> {
        const response = await api.post('/positions', data);
        if (!response.ok) throw new Error('Failed to create position');
        return await response.json();
    },

    async update(id: string, data: UpdatePositionDto): Promise<Position> {
        const response = await api.put(`/positions/${id}`, data);
        if (!response.ok) throw new Error('Failed to update position');
        return await response.json();
    },

    async delete(id: string): Promise<void> {
        const response = await api.delete(`/positions/${id}`);
        if (!response.ok) throw new Error('Failed to delete position');
    }
};
