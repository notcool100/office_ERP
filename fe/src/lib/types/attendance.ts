export interface AttendanceRecord {
    id: string;
    employeeId: string;
    employeeName: string;
    date: string;
    checkIn?: string;
    checkOut?: string;
    totalHours?: number;
    status: string;
    notes?: string;
    checkInImage?: string;
    checkInLat?: number;
    checkInLong?: number;
    checkOutLat?: number;
    checkOutLong?: number;
}

export interface CheckInRequest {
    employeeId: string;
    notes?: string;
    image?: string;
    latitude?: number;
    longitude?: number;
    method?: string;
}

export interface CheckOutRequest {
    notes?: string;
    latitude?: number;
    longitude?: number;
    method?: string;
}

export interface ListAttendanceResponse {
    records: AttendanceRecord[];
    total: number;
    page: number;
    pageSize: number;
}

export interface ListAttendanceQuery {
    page?: number;
    pageSize?: number;
    employeeId?: string;
    startDate?: string;
    endDate?: string;
    status?: string;
}

export interface AttendanceSummary {
    employeeId: string;
    employeeName: string;
    totalDays: number;
    presentDays: number;
    lateDays: number;
    absentDays: number;
    totalHours: number;
}
