import { api, buildApiUrl } from './api';

export type AssetCategory =
  | 'logos'
  | 'creatives'
  | 'brand-assets'
  | 'templates'
  | 'social-media'
  | 'videos'
  | 'documents'
  | 'other';

export interface MediaAsset {
  id: string;
  name: string;
  description: string | null;
  category: AssetCategory;
  tags: string[];
  file_name: string;
  content_type: string;
  file_size: number;
  is_image: boolean;
  uploaded_by: string | null;
  uploaded_by_name: string | null;
  created_at: string;
  updated_at: string;
}

export interface UpdateAssetDto {
  name?: string;
  description?: string;
  category?: AssetCategory;
  tags?: string[];
}

export interface ListAssetsQuery {
  category?: string;
  search?: string;
}

export const CATEGORIES: { value: AssetCategory | ''; label: string }[] = [
  { value: '', label: 'All Categories' },
  { value: 'logos', label: 'Logos' },
  { value: 'creatives', label: 'Creatives' },
  { value: 'brand-assets', label: 'Brand Assets' },
  { value: 'templates', label: 'Templates' },
  { value: 'social-media', label: 'Social Media' },
  { value: 'videos', label: 'Videos' },
  { value: 'documents', label: 'Documents' },
  { value: 'other', label: 'Other' },
];

export const CATEGORY_COLORS: Record<AssetCategory, string> = {
  logos: 'badge-primary',
  creatives: 'badge-secondary',
  'brand-assets': 'badge-accent',
  templates: 'badge-info',
  'social-media': 'badge-success',
  videos: 'badge-warning',
  documents: 'badge-neutral',
  other: 'badge-ghost',
};

export function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
}

export const mediaLibraryService = {
  async list(query: ListAssetsQuery = {}): Promise<MediaAsset[]> {
    const params = new URLSearchParams();
    if (query.category) params.set('category', query.category);
    if (query.search) params.set('search', query.search);
    const qs = params.toString();
    const res = await api.get(`/media-library${qs ? `?${qs}` : ''}`);
    if (!res.ok) throw new Error('Failed to fetch assets');
    return res.json();
  },

  async get(id: string): Promise<MediaAsset> {
    const res = await api.get(`/media-library/${id}`);
    if (!res.ok) throw new Error('Failed to fetch asset');
    return res.json();
  },

  async upload(formData: FormData): Promise<MediaAsset> {
    const res = await api.postForm('/media-library', formData);
    if (!res.ok) {
      const text = await res.text();
      throw new Error(text || 'Upload failed');
    }
    return res.json();
  },

  async update(id: string, dto: UpdateAssetDto): Promise<MediaAsset> {
    const res = await api.put(`/media-library/${id}`, dto);
    if (!res.ok) throw new Error('Failed to update asset');
    return res.json();
  },

  async delete(id: string): Promise<void> {
    const res = await api.delete(`/media-library/${id}`);
    if (!res.ok) throw new Error('Failed to delete asset');
  },

  fileUrl(id: string): string {
    return buildApiUrl(`/media-library/${id}/file`);
  },

  async fetchBlob(id: string): Promise<string> {
    const token = typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;
    const res = await fetch(buildApiUrl(`/media-library/${id}/file`), {
      headers: token ? { Authorization: `Bearer ${token}` } : {},
    });
    if (!res.ok) throw new Error('Failed to load file');
    const blob = await res.blob();
    return URL.createObjectURL(blob);
  },
};
