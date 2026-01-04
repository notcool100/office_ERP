import { PUBLIC_API_URL } from '$env/static/public';
import type {
    LeaveType,
    LeaveRequest,
    CreateLeaveRequestRequest,
    ApproveRejectLeaveRequest,
    ListLeaveRequestsResponse,
    ListLeaveRequestsQuery,
    LeaveBalance
} from '$lib/types/leave';

const getAuthHeaders = () => {
    const token = typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;
    return {
        'Content-Type': 'application/json',
        ...(token && { Authorization: `Bearer ${token}` })
    };
};

export async function createLeaveRequest(data: CreateLeaveRequestRequest): Promise<LeaveRequest> {
    const res = await fetch(`${PUBLIC_API_URL}/leave/requests`, {
        method: 'POST',
        headers: getAuthHeaders(),
        body: JSON.stringify(data)
    });

    if (!res.ok) {
        throw new Error('Failed to create leave request');
    }

    return await res.json();
}

export async function getLeaveRequest(id: string): Promise<LeaveRequest> {
    const res = await fetch(`${PUBLIC_API_URL}/leave/requests/${id}`, {
        headers: getAuthHeaders()
    });

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

    const res = await fetch(`${PUBLIC_API_URL}/leave/requests?${params}`, {
        headers: getAuthHeaders()
    });

    if (!res.ok) {
        throw new Error('Failed to fetch leave requests');
    }

    return await res.json();
}

export async function approveLeave(
    id: string,
    data?: ApproveRejectLeaveRequest
): Promise<LeaveRequest> {
    const res = await fetch(`${PUBLIC_API_URL}/leave/requests/${id}/approve`, {
        method: 'PUT',
        headers: getAuthHeaders(),
        body: JSON.stringify(data || {})
    });

    if (!res.ok) {
        throw new Error('Failed to approve leave');
    }

    return await res.json();
}

export async function rejectLeave(
    id: string,
    data?: ApproveRejectLeaveRequest
): Promise<LeaveRequest> {
    const res = await fetch(`${PUBLIC_API_URL}/leave/requests/${id}/reject`, {
        method: 'PUT',
        headers: getAuthHeaders(),
        body: JSON.stringify(data || {})
    });

    if (!res.ok) {
        throw new Error('Failed to reject leave');
    }

    return await res.json();
}

export async function getLeaveTypes(): Promise<LeaveType[]> {
    const res = await fetch(`${PUBLIC_API_URL}/leave/types`, {
        headers: getAuthHeaders()
    });

    if (!res.ok) {
        throw new Error('Failed to fetch leave types');
    }

    return await res.json();
}

export async function getLeaveBalance(employeeId: string): Promise<LeaveBalance[]> {
    const res = await fetch(`${PUBLIC_API_URL}/leave/balance/${employeeId}`, {
        headers: getAuthHeaders()
    });

    if (!res.ok) {
        throw new Error('Failed to fetch leave balance');
    }

    return await res.json();
}
