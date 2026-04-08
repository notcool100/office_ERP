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
    CardComment,
    CreateCardCommentDto,
    CardAttachment,
    CardActivity,
    Sprint,
    CreateSprintDto,
    UpdateSprintDto,
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

    async listCards(projectId: string, columnId?: string, sprintId?: string): Promise<Card[]> {
        const params = new URLSearchParams();
        if (columnId) params.append('column_id', columnId);
        if (sprintId) params.append('sprint_id', sprintId);
        const query = params.toString() ? `?${params.toString()}` : '';

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

    async listCardComments(projectId: string, cardId: string): Promise<CardComment[]> {
        const response = await api.get(`/projects/${projectId}/cards/${cardId}/comments`);
        if (!response.ok) throw new Error('Failed to fetch card comments');
        return await response.json();
    },

    async createCardComment(
        projectId: string,
        cardId: string,
        data: CreateCardCommentDto,
    ): Promise<CardComment> {
        const response = await api.post(`/projects/${projectId}/cards/${cardId}/comments`, data);
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to add comment');
        }
        return await response.json();
    },

    async listCardAttachments(projectId: string, cardId: string): Promise<CardAttachment[]> {
        const response = await api.get(`/projects/${projectId}/cards/${cardId}/attachments`);
        if (!response.ok) throw new Error('Failed to fetch attachments');
        return await response.json();
    },

    async uploadCardAttachment(
        projectId: string,
        cardId: string,
        file: File,
    ): Promise<CardAttachment> {
        const formData = new FormData();
        formData.append('file', file);
        const response = await api.postForm(`/projects/${projectId}/cards/${cardId}/attachments`, formData);
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to upload attachment');
        }
        return await response.json();
    },

    async downloadCardAttachment(
        projectId: string,
        cardId: string,
        attachmentId: string,
    ): Promise<{ blob: Blob; fileName: string }> {
        const response = await api.get(
            `/projects/${projectId}/cards/${cardId}/attachments/${attachmentId}`,
        );
        if (!response.ok) throw new Error('Failed to download attachment');

        const disposition = response.headers.get('content-disposition') || '';
        const fileNameMatch = disposition.match(/filename=\"?([^\";]+)\"?/i);
        const fileName = fileNameMatch?.[1] || 'attachment.bin';

        return {
            blob: await response.blob(),
            fileName,
        };
    },

    async listCardHistory(projectId: string, cardId: string): Promise<CardActivity[]> {
        const response = await api.get(`/projects/${projectId}/cards/${cardId}/history`);
        if (!response.ok) throw new Error('Failed to fetch card history');
        return await response.json();
    },

    async listSprints(projectId: string): Promise<Sprint[]> {
        const response = await api.get(`/projects/${projectId}/sprints`);
        if (!response.ok) throw new Error('Failed to fetch sprints');
        return await response.json();
    },

    async createSprint(projectId: string, data: CreateSprintDto): Promise<Sprint> {
        const response = await api.post(`/projects/${projectId}/sprints`, data);
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to create sprint');
        }
        return await response.json();
    },

    async updateSprint(projectId: string, sprintId: string, data: UpdateSprintDto): Promise<Sprint> {
        const response = await api.put(`/projects/${projectId}/sprints/${sprintId}`, data);
        if (!response.ok) {
            const err = await response.json().catch(() => ({}));
            throw new Error(err.message || 'Failed to update sprint');
        }
        return await response.json();
    },

    async deleteSprint(projectId: string, sprintId: string): Promise<void> {
        const response = await api.delete(`/projects/${projectId}/sprints/${sprintId}`);
        if (!response.ok) throw new Error('Failed to delete sprint');
    },
};
