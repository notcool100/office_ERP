import { PUBLIC_API_URL } from '$env/static/public';

export interface NavigationItem {
    id: string;
    name: string;
    path: string;
    icon: string | null;
    parent_id: string | null;
    display_order: number;
    is_active: boolean;
    created_at: string;
    updated_at: string;
}

export interface UserNavigationItem {
    id: string;
    name: string;
    path: string;
    icon: string | null;
    parent_id: string | null;
    display_order: number;
    can_create: boolean;
    can_read: boolean;
    can_update: boolean;
    can_delete: boolean;
    children: UserNavigationItem[];
}

export interface CreateNavigationItemDto {
    name: string;
    path: string;
    icon?: string;
    parent_id?: string;
    display_order?: number;
}

export interface UpdateNavigationItemDto {
    name?: string;
    path?: string;
    icon?: string;
    parent_id?: string;
    display_order?: number;
    is_active?: boolean;
}

const getAuthHeaders = () => {
    const token = typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;
    return {
        'Content-Type': 'application/json',
        ...(token && { Authorization: `Bearer ${token}` })
    };
};

export const navigationService = {
    async getUserNavigation(): Promise<UserNavigationItem[]> {
        const response = await fetch(`${PUBLIC_API_URL}/navigation/user`, {
            headers: getAuthHeaders()
        });
        if (!response.ok) throw new Error('Failed to fetch user navigation');
        return await response.json();
    },

    async getAll(isActive?: boolean): Promise<NavigationItem[]> {
        const params = new URLSearchParams();
        if (isActive !== undefined) {
            params.append('is_active', String(isActive));
        }
        const response = await fetch(`${PUBLIC_API_URL}/navigation?${params.toString()}`, {
            headers: getAuthHeaders()
        });
        if (!response.ok) throw new Error('Failed to fetch navigation');
        return await response.json();
    },

    async getById(id: string): Promise<NavigationItem> {
        const response = await fetch(`${PUBLIC_API_URL}/navigation/${id}`, {
            headers: getAuthHeaders()
        });
        if (!response.ok) throw new Error('Failed to fetch navigation item');
        return await response.json();
    },

    async create(data: CreateNavigationItemDto): Promise<NavigationItem> {
        const response = await fetch(`${PUBLIC_API_URL}/navigation`, {
            method: 'POST',
            headers: getAuthHeaders(),
            body: JSON.stringify(data)
        });
        if (!response.ok) throw new Error('Failed to create navigation item');
        return await response.json();
    },

    async update(id: string, data: UpdateNavigationItemDto): Promise<NavigationItem> {
        const response = await fetch(`${PUBLIC_API_URL}/navigation/${id}`, {
            method: 'PUT',
            headers: getAuthHeaders(),
            body: JSON.stringify(data)
        });
        if (!response.ok) throw new Error('Failed to update navigation item');
        return await response.json();
    },

    async delete(id: string): Promise<void> {
        const response = await fetch(`${PUBLIC_API_URL}/navigation/${id}`, {
            method: 'DELETE',
            headers: getAuthHeaders()
        });
        if (!response.ok) throw new Error('Failed to delete navigation item');
    }
};
