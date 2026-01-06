import { api } from './api';
import type { User, CreateUserRequest } from '../types/user';

export const userService = {
    async getAll(): Promise<User[]> {
        const response = await api.get('/users');
        if (!response.ok) throw new Error('Failed to fetch users');
        return await response.json();
    },

    async create(data: CreateUserRequest): Promise<User> {
        const response = await api.post('/users', data);
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to create user');
        }
        return await response.json();
    }
};
