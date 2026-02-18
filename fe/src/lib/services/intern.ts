import { api } from './api';
import type {
    Intern,
    CreateInternRequest,
    UpdateInternRequest,
    ListInternsResponse,
    ListInternsQuery
} from '$lib/types/intern';

export async function createIntern(data: CreateInternRequest): Promise<Intern> {
    const res = await api.post('/interns', data);

    if (!res.ok) {
        throw new Error('Failed to create intern');
    }

    return await res.json();
}

export async function getIntern(id: string): Promise<Intern> {
    const res = await api.get(`/interns/${id}`);

    if (!res.ok) {
        throw new Error('Failed to fetch intern');
    }

    return await res.json();
}

export async function listInterns(query?: ListInternsQuery): Promise<ListInternsResponse> {
    const params = new URLSearchParams();

    if (query?.page) params.append('page', query.page.toString());
    if (query?.pageSize) params.append('pageSize', query.pageSize.toString());
    if (query?.search) params.append('search', query.search);
    if (query?.department) params.append('department', query.department);
    if (query?.status) params.append('status', query.status);

    const queryString = params.toString();
    const res = await api.get(`/interns${queryString ? `?${queryString}` : ''}`);

    if (!res.ok) {
        throw new Error('Failed to fetch interns');
    }

    return await res.json();
}

export async function updateIntern(id: string, data: UpdateInternRequest): Promise<Intern> {
    const res = await api.put(`/interns/${id}`, data);

    if (!res.ok) {
        throw new Error('Failed to update intern');
    }

    return await res.json();
}

export async function deleteIntern(id: string): Promise<void> {
    const res = await api.delete(`/interns/${id}`);

    if (!res.ok) {
        throw new Error('Failed to delete intern');
    }
}
