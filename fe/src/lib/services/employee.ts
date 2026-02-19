import { api } from './api';
import type {
    Employee,
    CreateEmployeeRequest,
    UpdateEmployeeRequest,
    ListEmployeesResponse,
    ListEmployeesQuery
} from '$lib/types/employee';

export async function createEmployee(data: CreateEmployeeRequest): Promise<Employee> {
    const res = await api.post('/employees', data);

    if (!res.ok) {
        throw new Error('Failed to create employee');
    }

    return await res.json();
}

export async function getEmployee(id: string): Promise<Employee> {
    const res = await api.get(`/employees/${id}`);

    if (!res.ok) {
        throw new Error('Failed to fetch employee');
    }

    return await res.json();
}

export async function getMyEmployee(): Promise<Employee> {
    const res = await api.get('/employees/me');

    if (!res.ok) {
        throw new Error('Failed to fetch current employee');
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

    const queryString = params.toString();
    const res = await api.get(`/employees${queryString ? `?${queryString}` : ''}`);

    if (!res.ok) {
        throw new Error('Failed to fetch employees');
    }

    return await res.json();
}

export async function updateEmployee(id: string, data: UpdateEmployeeRequest): Promise<Employee> {
    const res = await api.put(`/employees/${id}`, data);

    if (!res.ok) {
        throw new Error('Failed to update employee');
    }

    return await res.json();
}

export async function deleteEmployee(id: string): Promise<void> {
    const res = await api.delete(`/employees/${id}`);

    if (!res.ok) {
        throw new Error('Failed to delete employee');
    }
}

export async function updateFaceDescriptor(id: string, descriptor: Float32Array): Promise<void> {
    const res = await api.post(`/employees/${id}/face-descriptor`, {
        descriptor: JSON.stringify(Array.from(descriptor))
    });

    if (!res.ok) {
        throw new Error('Failed to update face descriptor');
    }
}

export async function getAllFaceDescriptors(): Promise<[string, string][]> {
    const res = await api.get('/employees/config/descriptors');

    if (!res.ok) {
        throw new Error('Failed to fetch face descriptors');
    }

    return await res.json();
}
