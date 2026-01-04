import { PUBLIC_API_URL } from '$env/static/public';
import type {
    Intern,
    CreateInternRequest,
    UpdateInternRequest,
    ListInternsResponse,
    ListInternsQuery
} from '$lib/types/intern';

const getAuthHeaders = () => {
    const token = typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;
    return {
        'Content-Type': 'application/json',
        ...(token && { Authorization: `Bearer ${token}` })
    };
};

export async function createIntern(data: CreateInternRequest): Promise<Intern> {
    const res = await fetch(`${PUBLIC_API_URL}/interns`, {
        method: 'POST',
        headers: getAuthHeaders(),
        body: JSON.stringify(data)
    });

    if (!res.ok) {
        throw new Error('Failed to create intern');
    }

    return await res.json();
}

export async function getIntern(id: string): Promise<Intern> {
    const res = await fetch(`${PUBLIC_API_URL}/interns/${id}`, {
        headers: getAuthHeaders()
    });

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

    const res = await fetch(`${PUBLIC_API_URL}/interns?${params}`, {
        headers: getAuthHeaders()
    });

    if (!res.ok) {
        throw new Error('Failed to fetch interns');
    }

    return await res.json();
}

export async function updateIntern(id: string, data: UpdateInternRequest): Promise<Intern> {
    const res = await fetch(`${PUBLIC_API_URL}/interns/${id}`, {
        method: 'PUT',
        headers: getAuthHeaders(),
        body: JSON.stringify(data)
    });

    if (!res.ok) {
        throw new Error('Failed to update intern');
    }

    return await res.json();
}

export async function deleteIntern(id: string): Promise<void> {
    const res = await fetch(`${PUBLIC_API_URL}/interns/${id}`, {
        method: 'DELETE',
        headers: getAuthHeaders()
    });

    if (!res.ok) {
        throw new Error('Failed to delete intern');
    }
}
