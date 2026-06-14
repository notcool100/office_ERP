import { api } from './api';

export type CampaignPlatform =
  | 'google_ads' | 'facebook' | 'instagram' | 'linkedin'
  | 'tiktok' | 'youtube' | 'email' | 'twitter' | 'other';

export type CampaignStatus =
  | 'draft' | 'active' | 'paused' | 'completed' | 'cancelled';

export interface Campaign {
  id: string;
  name: string;
  description: string | null;
  platform: CampaignPlatform;
  status: CampaignStatus;
  budget: number;
  spent: number;
  revenue: number;
  roi: number | null;
  start_date: string | null;
  end_date: string | null;
  target_audience: string | null;
  goals: string | null;
  created_by: string | null;
  created_by_name: string | null;
  assigned_to: string | null;
  assigned_to_name: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateCampaignDto {
  name: string;
  description?: string;
  platform: string;
  status?: string;
  budget?: number;
  spent?: number;
  revenue?: number;
  start_date?: string;
  end_date?: string;
  target_audience?: string;
  goals?: string;
  assigned_to?: string;
}

export interface UpdateCampaignDto {
  name?: string;
  description?: string;
  platform?: string;
  status?: string;
  budget?: number;
  spent?: number;
  revenue?: number;
  start_date?: string;
  end_date?: string;
  target_audience?: string;
  goals?: string;
  assigned_to?: string;
}

export interface ListCampaignsQuery {
  platform?: string;
  status?: string;
  search?: string;
}

export const PLATFORMS: { value: CampaignPlatform | ''; label: string; color: string }[] = [
  { value: '',            label: 'All Platforms',  color: '' },
  { value: 'google_ads',  label: 'Google Ads',     color: '#4285F4' },
  { value: 'facebook',    label: 'Facebook',        color: '#1877F2' },
  { value: 'instagram',   label: 'Instagram',       color: '#E1306C' },
  { value: 'linkedin',    label: 'LinkedIn',        color: '#0A66C2' },
  { value: 'tiktok',      label: 'TikTok',          color: '#010101' },
  { value: 'youtube',     label: 'YouTube',         color: '#FF0000' },
  { value: 'email',       label: 'Email',           color: '#6366F1' },
  { value: 'twitter',     label: 'Twitter / X',     color: '#1DA1F2' },
  { value: 'other',       label: 'Other',           color: '#6B7280' },
];

export const STATUSES: { value: CampaignStatus | ''; label: string; badge: string }[] = [
  { value: '',          label: 'All Statuses', badge: 'badge-neutral' },
  { value: 'draft',     label: 'Draft',        badge: 'badge-ghost' },
  { value: 'active',    label: 'Active',       badge: 'badge-success' },
  { value: 'paused',    label: 'Paused',       badge: 'badge-warning' },
  { value: 'completed', label: 'Completed',    badge: 'badge-info' },
  { value: 'cancelled', label: 'Cancelled',    badge: 'badge-error' },
];

export function getPlatformLabel(value: string): string {
  return PLATFORMS.find(p => p.value === value)?.label ?? value;
}

export function getPlatformColor(value: string): string {
  return PLATFORMS.find(p => p.value === value)?.color ?? '#6B7280';
}

export function getStatusBadge(value: string): string {
  return STATUSES.find(s => s.value === value)?.badge ?? 'badge-neutral';
}

export function getStatusLabel(value: string): string {
  return STATUSES.find(s => s.value === value)?.label ?? value;
}

export function formatCurrency(n: number): string {
  return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', maximumFractionDigits: 0 }).format(n);
}

export function formatROI(roi: number | null): string {
  if (roi === null) return '—';
  return `${roi > 0 ? '+' : ''}${Number(roi).toFixed(1)}%`;
}

export const campaignService = {
  async list(query: ListCampaignsQuery = {}): Promise<Campaign[]> {
    const params = new URLSearchParams();
    if (query.platform) params.set('platform', query.platform);
    if (query.status) params.set('status', query.status);
    if (query.search) params.set('search', query.search);
    const qs = params.toString();
    const res = await api.get(`/campaigns${qs ? `?${qs}` : ''}`);
    if (!res.ok) throw new Error('Failed to load campaigns');
    return res.json();
  },

  async get(id: string): Promise<Campaign> {
    const res = await api.get(`/campaigns/${id}`);
    if (!res.ok) throw new Error('Campaign not found');
    return res.json();
  },

  async create(dto: CreateCampaignDto): Promise<Campaign> {
    const res = await api.post('/campaigns', dto);
    if (!res.ok) { const t = await res.text(); throw new Error(t || 'Failed to create'); }
    return res.json();
  },

  async update(id: string, dto: UpdateCampaignDto): Promise<Campaign> {
    const res = await api.put(`/campaigns/${id}`, dto);
    if (!res.ok) { const t = await res.text(); throw new Error(t || 'Failed to update'); }
    return res.json();
  },

  async delete(id: string): Promise<void> {
    const res = await api.delete(`/campaigns/${id}`);
    if (!res.ok) throw new Error('Failed to delete');
  },
};
