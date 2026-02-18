import { api } from './api';
import type {
    LeaveType,
    LeaveRequest,
    CreateLeaveRequestRequest,
    ApproveRejectLeaveRequest,
    ListLeaveRequestsResponse,
    ListLeaveRequestsQuery,
    LeaveBalance
} from '$lib/types/leave';

export async function createLeaveRequest(data: CreateLeaveRequestRequest): Promise<LeaveRequest> {
    const res = await api.post('/leave/requests', data);

    if (!res.ok) {
        throw new Error('Failed to create leave request');
    }

    return await res.json();
}

export async function getLeaveRequest(id: string): Promise<LeaveRequest> {
    const res = await api.get(`/leave/requests/${id}`);

    if (!res.ok) {
        throw new Error('Failed to fetch leave request');
    }

    return await res.json();
}

export async function listLeaveRequests(
    query?: ListLeaveRequestsQuery
): Promise<ListLeaveRequestsResponse> {
    const params = new URLSearchParams();

    if (query?.page) params.append('page', query.page.toString());
    if (query?.pageSize) params.append('pageSize', query.pageSize.toString());
    if (query?.employeeId) params.append('employeeId', query.employeeId);
    if (query?.status) params.append('status', query.status);
    if (query?.startDate) params.append('startDate', query.startDate);
    if (query?.endDate) params.append('endDate', query.endDate);

    const queryString = params.toString();
    const res = await api.get(`/leave/requests${queryString ? `?${queryString}` : ''}`);

    if (!res.ok) {
        throw new Error('Failed to fetch leave requests');
    }

    return await res.json();
}

export async function approveLeave(
    id: string,
    data?: ApproveRejectLeaveRequest
): Promise<LeaveRequest> {
    const res = await api.put(`/leave/requests/${id}/approve`, data ?? {});

    if (!res.ok) {
        throw new Error('Failed to approve leave');
    }

    return await res.json();
}

export async function rejectLeave(
    id: string,
    data?: ApproveRejectLeaveRequest
): Promise<LeaveRequest> {
    const res = await api.put(`/leave/requests/${id}/reject`, data ?? {});

    if (!res.ok) {
        throw new Error('Failed to reject leave');
    }

    return await res.json();
}

export async function getLeaveTypes(): Promise<LeaveType[]> {
    const res = await api.get('/leave/types');

    if (!res.ok) {
        throw new Error('Failed to fetch leave types');
    }

    return await res.json();
}

export async function getLeaveBalance(employeeId: string): Promise<LeaveBalance[]> {
    const res = await api.get(`/leave/balance/${employeeId}`);

    if (!res.ok) {
        throw new Error('Failed to fetch leave balance');
    }

    return await res.json();
}
