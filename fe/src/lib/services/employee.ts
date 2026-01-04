import { PUBLIC_API_URL } from '$env/static/public';
import type {
    Employee,
    CreateEmployeeRequest,
    UpdateEmployeeRequest,
    ListEmployeesResponse,
    ListEmployeesQuery
} from '$lib/types/employee';

const getAuthHeaders = () => {
    const token = typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;
    return {
        'Content-Type': 'application/json',
        ...(token && { Authorization: `Bearer ${token}` })
    };
};

export async function createEmployee(data: CreateEmployeeRequest): Promise<Employee> {
    const res = await fetch(`${PUBLIC_API_URL}/employees`, {
        method: 'POST',
        headers: getAuthHeaders(),
        body: JSON.stringify(data)
    });

    if (!res.ok) {
        throw new Error('Failed to create employee');
    }

    return await res.json();
}

export async function getEmployee(id: string): Promise<Employee> {
    const res = await fetch(`${PUBLIC_API_URL}/employees/${id}`, {
        headers: getAuthHeaders()
    });

    if (!res.ok) {
        throw new Error('Failed to fetch employee');
    }

    return await res.json();
}

export async function listEmployees(query?: ListEmployeesQuery): Promise<ListEmployeesResponse> {
    const params = new URLSearchParams();

    if (query?.page) params.append('page', query.page.toString());
    if (query?.pageSize) params.append('pageSize', query.pageSize.toString());
    if (query?.search) params.append('search', query.search);
    if (query?.department) params.append('department', query.department);
    if (query?.status) params.append('status', query.status);

    const res = await fetch(`${PUBLIC_API_URL}/employees?${params}`, {
        headers: getAuthHeaders()
    });

    if (!res.ok) {
        throw new Error('Failed to fetch employees');
    }

    return await res.json();
}

export async function updateEmployee(id: string, data: UpdateEmployeeRequest): Promise<Employee> {
    const res = await fetch(`${PUBLIC_API_URL}/employees/${id}`, {
        method: 'PUT',
        headers: getAuthHeaders(),
        body: JSON.stringify(data)
    });

    if (!res.ok) {
        throw new Error('Failed to update employee');
    }

    return await res.json();
}

export async function deleteEmployee(id: string): Promise<void> {
    const res = await fetch(`${PUBLIC_API_URL}/employees/${id}`, {
        method: 'DELETE',
        headers: getAuthHeaders()
    });

    if (!res.ok) {
        throw new Error('Failed to delete employee');
    }
}

export async function updateFaceDescriptor(id: string, descriptor: Float32Array): Promise<void> {
    const res = await fetch(`${PUBLIC_API_URL}/employees/${id}/face-descriptor`, {
        method: 'POST',
        headers: getAuthHeaders(),
        body: JSON.stringify({ descriptor: JSON.stringify(Array.from(descriptor)) })
    });

    if (!res.ok) {
        throw new Error('Failed to update face descriptor');
    }
}

export async function getAllFaceDescriptors(): Promise<[string, string][]> {
    const res = await fetch(`${PUBLIC_API_URL}/employees/config/descriptors`, {
        headers: getAuthHeaders()
    });

    if (!res.ok) {
        throw new Error('Failed to fetch face descriptors');
    }

    return await res.json();
}
