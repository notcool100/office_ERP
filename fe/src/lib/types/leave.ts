export interface LeaveType {
    id: string;
    name: string;
    description?: string;
    maxDaysPerYear?: number;
    requiresApproval: boolean;
    carryForward: boolean;
}

export interface LeaveRequest {
    id: string;
    employeeId: string;
    employeeName: string;
    leaveTypeId: string;
    leaveTypeName: string;
    startDate: string;
    endDate: string;
    totalDays: number;
    reason?: string;
    status: string;
    approvedBy?: string;
    approverName?: string;
    notes?: string;
}

export interface CreateLeaveRequestRequest {
    employeeId: string;
    leaveTypeId: string;
    startDate: string;
    endDate: string;
    reason?: string;
}

export interface ApproveRejectLeaveRequest {
    notes?: string;
}

export interface ListLeaveRequestsResponse {
    requests: LeaveRequest[];
    total: number;
    page: number;
    pageSize: number;
}

export interface ListLeaveRequestsQuery {
    page?: number;
    pageSize?: number;
    employeeId?: string;
    status?: string;
    startDate?: string;
    endDate?: string;
}

export interface LeaveBalance {
    employeeId: string;
    leaveTypeId: string;
    leaveTypeName: string;
    totalAllowed: number;
    used: number;
    remaining: number;
}
