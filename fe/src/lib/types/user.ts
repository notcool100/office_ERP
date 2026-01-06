export interface User {
    id: string;
    userName: string;
    email: string;
    phone: string;
    personId: string;
    isAdmin: boolean;
    createdAt: string;
}

export interface CreateUserRequest {
    personId: string;
    userName: string;
    password: string;
    email: string;
    phone: string;
    isAdmin: boolean;
}
