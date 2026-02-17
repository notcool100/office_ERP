import { api } from './api';
import type {
    Project,
    CreateProjectDto,
    UpdateProjectDto,
    ProjectMember,
    Board,
    Card,
    CreateCardDto,
    UpdateCardDto,
} from '$lib/types/project';

export const projectService = {
    async list(): Promise<Project[]> {
        const response = await api.get('/projects');
        if (!response.ok) throw new Error('Failed to fetch projects');
        return await response.json();
    },

    async getById(id: string): Promise<Project> {
        const response = await api.get(`/projects/${id}`);
        if (!response.ok) throw new Error('Failed to fetch project');
        return await response.json();
    },

    async create(data: CreateProjectDto): Promise<Project> {
        const response = await api.post('/projects', data);
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to create project');
        }
        return await response.json();
    },

    async update(id: string, data: UpdateProjectDto): Promise<Project> {
        const response = await api.put(`/projects/${id}`, data);
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to update project');
        }
        return await response.json();
    },

    async listMembers(projectId: string): Promise<ProjectMember[]> {
        const response = await api.get(`/projects/${projectId}/members`);
        if (!response.ok) throw new Error('Failed to fetch project members');
        return await response.json();
    },

    async addMember(projectId: string, data: { user_id: string; role: string }): Promise<ProjectMember> {
        const response = await api.post(`/projects/${projectId}/members`, data);
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to add member');
        }
        return await response.json();
    },

    async getBoard(projectId: string): Promise<Board> {
        const response = await api.get(`/projects/${projectId}/board`);
        if (!response.ok) throw new Error('Failed to fetch board');
        return await response.json();
    },

    async listCards(projectId: string, columnId?: string): Promise<Card[]> {
        const query = columnId ? `?column_id=${columnId}` : '';
        const response = await api.get(`/projects/${projectId}/cards${query}`);
        if (!response.ok) throw new Error('Failed to fetch cards');
        return await response.json();
    },

    async createCard(projectId: string, data: CreateCardDto): Promise<Card> {
        const response = await api.post(`/projects/${projectId}/cards`, data);
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to create card');
        }
        return await response.json();
    },

    async updateCard(projectId: string, cardId: string, data: UpdateCardDto): Promise<Card> {
        const response = await api.put(`/projects/${projectId}/cards/${cardId}`, data);
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to update card');
        }
        return await response.json();
    },

    async deleteCard(projectId: string, cardId: string): Promise<void> {
        const response = await api.delete(`/projects/${projectId}/cards/${cardId}`);
        if (!response.ok) throw new Error('Failed to delete card');
    },
};
