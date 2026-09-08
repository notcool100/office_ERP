import { api } from './api';

export interface CompanySettings {
    name: string;
    logo_url: string | null;
    favicon_url: string | null;
    latitude: number | null;
    longitude: number | null;
    location_name: string | null;
    updated_at: string;
}

export interface UpdateCompanyDetailsDto {
    name: string;
    latitude: number | null;
    longitude: number | null;
    location_name: string | null;
}

export const companySettingsService = {
    async getSettings(): Promise<CompanySettings> {
        const res = await api.get('/company-settings');
        if (!res.ok) {
            const err = await res.json().catch(() => ({}));
            throw new Error((err as any).message || 'Failed to fetch company settings');
        }
        return res.json();
    },

    async updateDetails(dto: UpdateCompanyDetailsDto): Promise<CompanySettings> {
        const res = await api.put('/company-settings/manage', dto);
        if (!res.ok) {
            const err = await res.json().catch(() => ({}));
            throw new Error((err as any).message || 'Failed to update company settings');
        }
        return res.json();
    },

    async uploadLogo(file: File): Promise<CompanySettings> {
        const fd = new FormData();
        fd.append('file', file);
        const res = await api.postForm('/company-settings/manage/logo', fd);
        if (!res.ok) {
            const err = await res.json().catch(() => ({}));
            throw new Error((err as any).message || 'Failed to upload logo');
        }
        return res.json();
    },

    async uploadFavicon(file: File): Promise<CompanySettings> {
        const fd = new FormData();
        fd.append('file', file);
        const res = await api.postForm('/company-settings/manage/favicon', fd);
        if (!res.ok) {
            const err = await res.json().catch(() => ({}));
            throw new Error((err as any).message || 'Failed to upload favicon');
        }
        return res.json();
    },
};
