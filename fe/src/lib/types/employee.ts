export interface Employee {
    id: string;
    employeeId: string;
    personId: string;
    firstName: string;
    middleName?: string;
    lastName: string;
    email: string;
    phone?: string;
    department?: string;
    position?: string;
    hireDate: string; // ISO date string
    employmentType?: string;
    salary?: number;
    managerId?: string;
    status: string;
}

export interface CreateEmployeeRequest {
    personId: string;
    department?: string;
    position?: string;
    hireDate: string;
    employmentType?: string;
    salary?: number;
    managerId?: string;
}

export interface UpdateEmployeeRequest {
    department?: string;
    position?: string;
    employmentType?: string;
    salary?: number;
    managerId?: string;
    status?: string;
}

export interface ListEmployeesResponse {
    employees: Employee[];
    total: number;
    page: number;
    pageSize: number;
}

export interface ListEmployeesQuery {
    page?: number;
    pageSize?: number;
    search?: string;
    department?: string;
    status?: string;
}

// Minimal, non-privileged record served by the kiosk-scoped endpoints -
// just enough to label a recognized face. Reachable by any authenticated
// employee, unlike `Employee`/`ListEmployeesResponse` which require HR-admin
// permissions.
export interface KioskEmployeeSummary {
    id: string;
    employeeId: string;
    firstName: string;
    lastName: string;
    department?: string;
}
