import { api } from './api';
import type { User, CreateUserRequest } from '../types/user';

export const userService = {
    async getAll(): Promise<User[]> {
        const response = await api.get('/users');
        if (!response.ok) throw new Error('Failed to fetch users');
        return await response.json();
    },

    async getById(id: string): Promise<User> {
        const response = await api.get(`/users/${id}`);
        if (!response.ok) throw new Error('Failed to fetch user');
        return await response.json();
    },

    async create(data: CreateUserRequest): Promise<User> {
        const response = await api.post('/users', data);
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to create user');
        }
        return await response.json();
    },

    async update(id: string, data: Partial<User>): Promise<User> {
        const response = await api.put(`/users/${id}`, data);
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to update user');
        }
        return await response.json();
    },

    async delete(id: string): Promise<void> {
        const response = await api.delete(`/users/${id}`);
        if (!response.ok) throw new Error('Failed to delete user');
    },

    async changePassword(id: string, newPassword: string): Promise<void> {
        const response = await api.put(`/users/${id}/password`, { newPassword });
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to change password');
        }
    }
};
