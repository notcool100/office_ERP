import { api } from './api';
import { PLATFORMS, STATUSES, formatCurrency, formatROI } from './campaigns';

export interface ReportSummary {
  total_campaigns: number;
  active_campaigns: number;
  total_budget: number;
  total_spent: number;
  total_revenue: number;
  overall_roi: number | null;
}

export interface PlatformStat {
  platform: string;
  campaign_count: number;
  total_budget: number;
  total_spent: number;
  total_revenue: number;
  roi: number | null;
}

export interface StatusStat {
  status: string;
  count: number;
}

export interface CampaignPerformance {
  id: string;
  name: string;
  platform: string;
  status: string;
  budget: number;
  spent: number;
  revenue: number;
  roi: number | null;
  start_date: string | null;
  end_date: string | null;
}

export interface MarketingReport {
  summary: ReportSummary;
  by_platform: PlatformStat[];
  by_status: StatusStat[];
  campaigns: CampaignPerformance[];
}

export interface ReportQuery {
  start_date?: string;
  end_date?: string;
  platform?: string;
  status?: string;
}

export { PLATFORMS, STATUSES, formatCurrency, formatROI };

export const marketingReportService = {
  async fetch(query: ReportQuery = {}): Promise<MarketingReport> {
    const p = new URLSearchParams();
    if (query.start_date) p.set('start_date', query.start_date);
    if (query.end_date)   p.set('end_date',   query.end_date);
    if (query.platform)   p.set('platform',   query.platform);
    if (query.status)     p.set('status',     query.status);
    const qs = p.toString();
    const res = await api.get(`/marketing-reports${qs ? `?${qs}` : ''}`);
    if (!res.ok) throw new Error('Failed to load report');
    return res.json();
  },
};
