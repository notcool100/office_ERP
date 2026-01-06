import { api } from './api';

export interface Permission {
    id: string;
    department_id: string | null;
    department_name: string | null;
    position_id: string | null;
    position_name: string | null;
    navigation_item_id: string;
    navigation_name: string;
    navigation_path: string;
    can_create: boolean;
    can_read: boolean;
    can_update: boolean;
    can_delete: boolean;
    created_at: string;
}

export interface AssignPermissionDto {
    department_id?: string;
    position_id?: string;
    navigation_item_id: string;
    can_create: boolean;
    can_read: boolean;
    can_update: boolean;
    can_delete: boolean;
}

export const permissionService = {
    async getAll(filters?: {
        department_id?: string;
        position_id?: string;
        navigation_item_id?: string;
    }): Promise<Permission[]> {
        const params = new URLSearchParams();
        if (filters?.department_id) params.append('department_id', filters.department_id);
        if (filters?.position_id) params.append('position_id', filters.position_id);
        if (filters?.navigation_item_id) params.append('navigation_item_id', filters.navigation_item_id);

        const response = await api.get(`/permissions?${params.toString()}`);
        if (!response.ok) throw new Error('Failed to fetch permissions');
        return await response.json();
    },

    async assign(data: AssignPermissionDto): Promise<void> {
        const response = await api.post('/permissions', data);
        if (!response.ok) throw new Error('Failed to assign permission');
    },

    async delete(id: string): Promise<void> {
        const response = await api.delete(`/permissions/${id}`);
        if (!response.ok) throw new Error('Failed to delete permission');
    }
};
