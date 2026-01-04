import { PUBLIC_API_URL } from '$env/static/public';

const getAuthHeaders = (): Record<string, string> => {
    const token = typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;
    return token ? { 'Authorization': `Bearer ${token}` } : {};
};

export interface Person {
    id: string;
    first_name: string;
    middle_name?: string;
    last_name: string;
    created_at: string;
}

export interface CreatePersonRequest {
    first_name: string;
    middle_name?: string;
    last_name: string;
}

export interface UpdatePersonRequest {
    first_name?: string;
    middle_name?: string;
    last_name?: string;
}

export interface ListPersonsQuery {
    page?: number;
    pageSize?: number;
    search?: string;
}

export interface ListPersonsResponse {
    persons: Person[];
    total: number;
    page: number;
    pageSize: number;
}

export const personService = {
    async getAll(query: ListPersonsQuery): Promise<ListPersonsResponse> {
        const params = new URLSearchParams();
        if (query.page) params.append('page', query.page.toString());
        if (query.pageSize) params.append('page_size', query.pageSize.toString());
        if (query.search) params.append('search', query.search);

        const response = await fetch(`${PUBLIC_API_URL}/persons?${params.toString()}`, {
            headers: getAuthHeaders()
        });

        if (!response.ok) throw new Error('Failed to fetch persons');
        return response.json();
    },

    async getById(id: string): Promise<Person> {
        const response = await fetch(`${PUBLIC_API_URL}/persons/${id}`, {
            headers: getAuthHeaders()
        });

        if (!response.ok) throw new Error('Failed to fetch person');
        return response.json();
    },

    async create(data: CreatePersonRequest): Promise<Person> {
        const response = await fetch(`${PUBLIC_API_URL}/persons`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                ...getAuthHeaders()
            },
            body: JSON.stringify(data)
        });

        if (!response.ok) throw new Error('Failed to create person');
        return response.json();
    },

    async update(id: string, data: UpdatePersonRequest): Promise<Person> {
        const response = await fetch(`${PUBLIC_API_URL}/persons/${id}`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
                ...getAuthHeaders()
            },
            body: JSON.stringify(data)
        });

        if (!response.ok) throw new Error('Failed to update person');
        return response.json();
    },

    async delete(id: string): Promise<void> {
        const response = await fetch(`${PUBLIC_API_URL}/persons/${id}`, {
            method: 'DELETE',
            headers: getAuthHeaders()
        });

        if (!response.ok) throw new Error('Failed to delete person');
    }
};
