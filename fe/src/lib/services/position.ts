import { PUBLIC_API_URL } from '$env/static/public';

export interface Position {
    id: string;
    name: string;
    description: string | null;
    is_active: boolean;
    created_at: string;
    updated_at: string;
}

export interface CreatePositionDto {
    name: string;
    description?: string;
}

export interface UpdatePositionDto {
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

export const positionService = {
    async getAll(isActive?: boolean): Promise<Position[]> {
        const params = new URLSearchParams();
        if (isActive !== undefined) {
            params.append('is_active', String(isActive));
        }
        const response = await fetch(`${PUBLIC_API_URL}/positions?${params.toString()}`, {
            headers: getAuthHeaders()
        });
        if (!response.ok) throw new Error('Failed to fetch positions');
        return await response.json();
    },

    async getById(id: string): Promise<Position> {
        const response = await fetch(`${PUBLIC_API_URL}/positions/${id}`, {
            headers: getAuthHeaders()
        });
        if (!response.ok) throw new Error('Failed to fetch position');
        return await response.json();
    },

    async create(data: CreatePositionDto): Promise<Position> {
        const response = await fetch(`${PUBLIC_API_URL}/positions`, {
            method: 'POST',
            headers: getAuthHeaders(),
            body: JSON.stringify(data)
        });
        if (!response.ok) throw new Error('Failed to create position');
        return await response.json();
    },

    async update(id: string, data: UpdatePositionDto): Promise<Position> {
        const response = await fetch(`${PUBLIC_API_URL}/positions/${id}`, {
            method: 'PUT',
            headers: getAuthHeaders(),
            body: JSON.stringify(data)
        });
        if (!response.ok) throw new Error('Failed to update position');
        return await response.json();
    },

    async delete(id: string): Promise<void> {
        const response = await fetch(`${PUBLIC_API_URL}/positions/${id}`, {
            method: 'DELETE',
            headers: getAuthHeaders()
        });
        if (!response.ok) throw new Error('Failed to delete position');
    }
};
