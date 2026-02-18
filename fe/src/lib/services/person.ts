import { api } from './api';

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
        const queryString = params.toString();
        const response = await api.get(`/persons${queryString ? `?${queryString}` : ''}`);

        if (!response.ok) throw new Error('Failed to fetch persons');
        return response.json();
    },

    async getById(id: string): Promise<Person> {
        const response = await api.get(`/persons/${id}`);

        if (!response.ok) throw new Error('Failed to fetch person');
        return response.json();
    },

    async create(data: CreatePersonRequest): Promise<Person> {
        const response = await api.post('/persons', data);

        if (!response.ok) throw new Error('Failed to create person');
        return response.json();
    },

    async update(id: string, data: UpdatePersonRequest): Promise<Person> {
        const response = await api.put(`/persons/${id}`, data);

        if (!response.ok) throw new Error('Failed to update person');
        return response.json();
    },

    async delete(id: string): Promise<void> {
        const response = await api.delete(`/persons/${id}`);

        if (!response.ok) throw new Error('Failed to delete person');
    }
};
