import { api } from './api';

export type ClientStatus = 'active' | 'inactive' | 'prospect';
export type DeliverableStatus = 'pending' | 'in_progress' | 'review' | 'done' | 'cancelled';
export type DeliverableType =
  | 'social_post' | 'blog_post' | 'ad_creative' | 'report'
  | 'video' | 'design' | 'email_campaign' | 'other';

export interface Client {
  id: string;
  name: string;
  contact_person: string | null;
  email: string | null;
  phone: string | null;
  industry: string | null;
  website: string | null;
  address: string | null;
  notes: string | null;
  status: ClientStatus;
  deliverable_count: number;
  created_by: string | null;
  created_by_name: string | null;
  created_at: string;
  updated_at: string;
}

export interface Deliverable {
  id: string;
  client_id: string;
  client_name: string;
  campaign_id: string | null;
  campaign_name: string | null;
  title: string;
  description: string | null;
  deliverable_type: DeliverableType;
  due_date: string | null;
  status: DeliverableStatus;
  assigned_to: string | null;
  assigned_to_name: string | null;
  created_by: string | null;
  created_by_name: string | null;
  created_at: string;
  updated_at: string;
}

export const CLIENT_STATUSES: { value: ClientStatus | ''; label: string; badge: string }[] = [
  { value: '',         label: 'All',      badge: 'badge-neutral' },
  { value: 'active',   label: 'Active',   badge: 'badge-success' },
  { value: 'prospect', label: 'Prospect', badge: 'badge-info' },
  { value: 'inactive', label: 'Inactive', badge: 'badge-ghost' },
];

export const DELIVERABLE_STATUSES: { value: DeliverableStatus | ''; label: string; badge: string }[] = [
  { value: '',            label: 'All',         badge: 'badge-neutral' },
  { value: 'pending',     label: 'Pending',     badge: 'badge-ghost' },
  { value: 'in_progress', label: 'In Progress', badge: 'badge-warning' },
  { value: 'review',      label: 'In Review',   badge: 'badge-info' },
  { value: 'done',        label: 'Done',        badge: 'badge-success' },
  { value: 'cancelled',   label: 'Cancelled',   badge: 'badge-error' },
];

export const DELIVERABLE_TYPES: { value: DeliverableType; label: string }[] = [
  { value: 'social_post',    label: 'Social Post' },
  { value: 'blog_post',      label: 'Blog Post' },
  { value: 'ad_creative',    label: 'Ad Creative' },
  { value: 'report',         label: 'Report' },
  { value: 'video',          label: 'Video' },
  { value: 'design',         label: 'Design' },
  { value: 'email_campaign', label: 'Email Campaign' },
  { value: 'other',          label: 'Other' },
];

export function getClientStatusBadge(v: string) {
  return CLIENT_STATUSES.find(s => s.value === v)?.badge ?? 'badge-neutral';
}
export function getClientStatusLabel(v: string) {
  return CLIENT_STATUSES.find(s => s.value === v)?.label ?? v;
}
export function getDelivStatusBadge(v: string) {
  return DELIVERABLE_STATUSES.find(s => s.value === v)?.badge ?? 'badge-neutral';
}
export function getDelivStatusLabel(v: string) {
  return DELIVERABLE_STATUSES.find(s => s.value === v)?.label ?? v;
}
export function getDelivTypeLabel(v: string) {
  return DELIVERABLE_TYPES.find(t => t.value === v)?.label ?? v;
}

function isDueSoon(due: string | null): boolean {
  if (!due) return false;
  const d = new Date(due);
  const now = new Date();
  const diff = (d.getTime() - now.getTime()) / 86400000;
  return diff >= 0 && diff <= 3;
}
function isOverdue(due: string | null, status: string): boolean {
  if (!due || status === 'done' || status === 'cancelled') return false;
  return new Date(due) < new Date();
}
export { isDueSoon, isOverdue };

const BASE = '/client-management';

export const clientService = {
  async listClients(query: { status?: string; search?: string } = {}): Promise<Client[]> {
    const p = new URLSearchParams();
    if (query.status) p.set('status', query.status);
    if (query.search) p.set('search', query.search);
    const qs = p.toString();
    const res = await api.get(`${BASE}/clients${qs ? `?${qs}` : ''}`);
    if (!res.ok) throw new Error('Failed to load clients');
    return res.json();
  },

  async createClient(dto: object): Promise<Client> {
    const res = await api.post(`${BASE}/clients`, dto);
    if (!res.ok) { const t = await res.text(); throw new Error(t || 'Failed to create'); }
    return res.json();
  },

  async updateClient(id: string, dto: object): Promise<Client> {
    const res = await api.put(`${BASE}/clients/${id}`, dto);
    if (!res.ok) { const t = await res.text(); throw new Error(t || 'Failed to update'); }
    return res.json();
  },

  async deleteClient(id: string): Promise<void> {
    const res = await api.delete(`${BASE}/clients/${id}`);
    if (!res.ok) throw new Error('Failed to delete');
  },

  async listDeliverables(query: { client_id?: string; campaign_id?: string; status?: string; search?: string } = {}): Promise<Deliverable[]> {
    const p = new URLSearchParams();
    if (query.client_id)  p.set('client_id',  query.client_id);
    if (query.campaign_id) p.set('campaign_id', query.campaign_id);
    if (query.status)     p.set('status',     query.status);
    if (query.search)     p.set('search',     query.search);
    const qs = p.toString();
    const res = await api.get(`${BASE}/deliverables${qs ? `?${qs}` : ''}`);
    if (!res.ok) throw new Error('Failed to load deliverables');
    return res.json();
  },

  async createDeliverable(dto: object): Promise<Deliverable> {
    const res = await api.post(`${BASE}/deliverables`, dto);
    if (!res.ok) { const t = await res.text(); throw new Error(t || 'Failed to create'); }
    return res.json();
  },

  async updateDeliverable(id: string, dto: object): Promise<Deliverable> {
    const res = await api.put(`${BASE}/deliverables/${id}`, dto);
    if (!res.ok) { const t = await res.text(); throw new Error(t || 'Failed to update'); }
    return res.json();
  },

  async deleteDeliverable(id: string): Promise<void> {
    const res = await api.delete(`${BASE}/deliverables/${id}`);
    if (!res.ok) throw new Error('Failed to delete');
  },
};
