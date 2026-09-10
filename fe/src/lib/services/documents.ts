import { api, buildApiUrl } from './api';

export type DocCategory = 'company' | 'client' | 'employee';

export interface FolderMeta {
    id: string;
    name: string;
    parentId: string | null;
    createdBy: string | null;
    createdByName: string | null;
    createdAt: string;
    updatedAt: string;
}

export interface DocumentMeta {
    id: string;
    folderId: string | null;
    docType: string | null;
    title: string;
    description: string | null;
    fileName: string;
    contentType: string;
    fileSize: number;
    isImage: boolean;
    expiryDate: string | null;
    uploadedBy: string | null;
    uploadedByName: string | null;
    createdAt: string;
    updatedAt: string;
}

export interface FolderListing {
    folders: FolderMeta[];
    documents: DocumentMeta[];
    breadcrumb: FolderMeta[];
}

export interface UpdateDocumentDto {
    title?: string;
    description?: string;
    docType?: string;
    expiryDate?: string;
}

function base(category: DocCategory) {
    return `/documents/${category}`;
}

function ownerParams(ownerId?: string): string {
    const params = new URLSearchParams();
    if (ownerId) params.set('ownerId', ownerId);
    const qs = params.toString();
    return qs ? `?${qs}` : '';
}

export function formatFileSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
}

export const documentsService = {
    async list(
        category: DocCategory,
        ownerId?: string,
        folderId?: string,
    ): Promise<FolderListing> {
        const params = new URLSearchParams();
        if (ownerId) params.set('ownerId', ownerId);
        if (folderId) params.set('folderId', folderId);
        const qs = params.toString();
        const res = await api.get(`${base(category)}${qs ? `?${qs}` : ''}`);
        if (!res.ok) throw new Error('Failed to load documents');
        return res.json();
    },

    async createFolder(
        category: DocCategory,
        ownerId: string | undefined,
        parentId: string | undefined,
        name: string,
    ): Promise<FolderMeta> {
        const res = await api.post(`${base(category)}/folders`, {
            ownerId,
            parentId,
            name,
        });
        if (!res.ok) {
            const text = await res.text();
            throw new Error(text || 'Failed to create folder');
        }
        return res.json();
    },

    async deleteFolder(
        category: DocCategory,
        ownerId: string | undefined,
        id: string,
    ): Promise<void> {
        const res = await api.delete(
            `${base(category)}/folders/${id}${ownerParams(ownerId)}`,
        );
        if (!res.ok) throw new Error('Failed to delete folder');
    },

    async upload(category: DocCategory, formData: FormData): Promise<DocumentMeta> {
        const res = await api.postForm(base(category), formData);
        if (!res.ok) {
            const text = await res.text();
            throw new Error(text || 'Upload failed');
        }
        return res.json();
    },

    async update(
        category: DocCategory,
        ownerId: string | undefined,
        id: string,
        dto: UpdateDocumentDto,
    ): Promise<DocumentMeta> {
        const res = await api.put(
            `${base(category)}/${id}${ownerParams(ownerId)}`,
            dto,
        );
        if (!res.ok) throw new Error('Failed to update document');
        return res.json();
    },

    async delete(
        category: DocCategory,
        ownerId: string | undefined,
        id: string,
    ): Promise<void> {
        const res = await api.delete(`${base(category)}/${id}${ownerParams(ownerId)}`);
        if (!res.ok) throw new Error('Failed to delete document');
    },

    fileUrl(category: DocCategory, ownerId: string | undefined, id: string): string {
        return buildApiUrl(`${base(category)}/${id}/file${ownerParams(ownerId)}`);
    },

    async fetchBlob(
        category: DocCategory,
        ownerId: string | undefined,
        id: string,
    ): Promise<string> {
        const token =
            typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;
        const res = await fetch(this.fileUrl(category, ownerId, id), {
            headers: token ? { Authorization: `Bearer ${token}` } : {},
        });
        if (!res.ok) throw new Error('Failed to load file');
        const blob = await res.blob();
        return URL.createObjectURL(blob);
    },
};
