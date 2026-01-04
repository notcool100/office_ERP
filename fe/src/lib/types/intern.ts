export interface Intern {
    id: string;
    internId: string;
    personId: string;
    firstName: string;
    middleName?: string;
    lastName: string;
    email: string;
    phone?: string;
    department?: string;
    supervisorId?: string;
    startDate: string;
    endDate?: string;
    stipend?: number;
    university?: string;
    status: string;
}

export interface CreateInternRequest {
    internId: string;
    personId: string;
    department?: string;
    supervisorId?: string;
    startDate: string;
    endDate?: string;
    stipend?: number;
    university?: string;
}

export interface UpdateInternRequest {
    department?: string;
    supervisorId?: string;
    endDate?: string;
    stipend?: number;
    university?: string;
    status?: string;
}

export interface ListInternsResponse {
    interns: Intern[];
    total: number;
    page: number;
    pageSize: number;
}

export interface ListInternsQuery {
    page?: number;
    pageSize?: number;
    search?: string;
    department?: string;
    status?: string;
}
