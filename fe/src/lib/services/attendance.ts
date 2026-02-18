import { api } from './api';
import type {
    AttendanceRecord,
    CheckInRequest,
    CheckOutRequest,
    ListAttendanceResponse,
    ListAttendanceQuery,
    AttendanceSummary
} from '$lib/types/attendance';

export async function checkIn(data: CheckInRequest): Promise<AttendanceRecord> {
    const res = await api.post('/attendance/check-in', data);

    if (!res.ok) {
        throw new Error('Failed to check in');
    }

    return await res.json();
}

export async function checkOut(employeeId: string, data?: CheckOutRequest): Promise<AttendanceRecord> {
    const res = await api.post(`/attendance/check-out/${employeeId}`, data ?? {});

    if (!res.ok) {
        throw new Error('Failed to check out');
    }

    return await res.json();
}

export async function listAttendance(query?: ListAttendanceQuery): Promise<ListAttendanceResponse> {
    const params = new URLSearchParams();

    if (query?.page) params.append('page', query.page.toString());
    if (query?.pageSize) params.append('pageSize', query.pageSize.toString());
    if (query?.employeeId) params.append('employeeId', query.employeeId);
    if (query?.startDate) params.append('startDate', query.startDate);
    if (query?.endDate) params.append('endDate', query.endDate);
    if (query?.status) params.append('status', query.status);

    const queryString = params.toString();
    const res = await api.get(`/attendance/records${queryString ? `?${queryString}` : ''}`);

    if (!res.ok) {
        throw new Error('Failed to fetch attendance records');
    }

    return await res.json();
}

export async function getAttendanceSummary(
    employeeId: string,
    startDate: string,
    endDate: string
): Promise<AttendanceSummary> {
    const res = await api.get(
        `/attendance/summary/${employeeId}/${startDate}/${endDate}`
    );

    if (!res.ok) {
        throw new Error('Failed to fetch attendance summary');
    }

    return await res.json();
}
