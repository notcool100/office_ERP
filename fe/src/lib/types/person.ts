export interface Person {
    id: string;
    first_name: string;
    middle_name?: string;
    last_name: string;
    email?: string;
    phone?: string;
    date_of_birth?: string;
    citizenship_number?: string;
    created_at: string;
}

export interface CreatePersonRequest {
    first_name: string;
    middle_name?: string;
    last_name: string;
    email?: string;
    phone?: string;
}

export interface UpdatePersonRequest {
    first_name?: string;
    middle_name?: string;
    last_name?: string;
    email?: string;
    phone?: string;
}

