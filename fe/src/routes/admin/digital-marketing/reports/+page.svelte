<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import {
        Home, Megaphone, FileBarChart2,
        TrendingUp, TrendingDown, DollarSign, Target, BarChart2, RefreshCw
    } from 'lucide-svelte';
    import { onMount } from 'svelte';
    import {
        marketingReportService, PLATFORMS, STATUSES,
        formatCurrency, formatROI,
    } from '$lib/services/marketing-reports';
    import type { MarketingReport } from '$lib/services/marketing-reports';

    pageTitle.set({ title: 'Marketing Reports', desc: 'Campaign performance analytics' });
    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Digital Marketing', icon: Megaphone },
        { label: 'Marketing Reports', icon: FileBarChart2 },
    ]);

    // ── State ─────────────────────────────────────────────────────────────────
    let report: MarketingReport | null = null;
    let loading = true;
    let error = '';

    let filterStartDate = '';
    let filterEndDate = '';
    let filterPlatform = '';
    let filterStatus = '';

    let sortCol: 'roi' | 'revenue' | 'spent' | 'budget' = 'roi';
    let sortAsc = false;

    // ── Load ──────────────────────────────────────────────────────────────────
    async function load() {
        loading = true; error = '';
        try {
            report = await marketingReportService.fetch({
                start_date: filterStartDate || undefined,
                end_date:   filterEndDate   || undefined,
                platform:   filterPlatform  || undefined,
                status:     filterStatus    || undefined,
            });
        } catch (e: any) {
            error = e.message ?? 'Failed to load report';
        } finally {
            loading = false;
        }
    }

    // ── Derived ───────────────────────────────────────────────────────────────
    $: sortedCampaigns = report
        ? [...report.campaigns].sort((a, b) => {
            const av = a[sortCol] ?? (sortAsc ? Infinity : -Infinity);
            const bv = b[sortCol] ?? (sortAsc ? Infinity : -Infinity);
            return sortAsc ? Number(av) - Number(bv) : Number(bv) - Number(av);
          })
        : [];

    $: maxRevenue = report
        ? Math.max(...report.by_platform.map(p => Number(p.total_revenue)), 1)
        : 1;

    $: totalCampaignCount = report
        ? report.by_status.reduce((s, x) => s + x.count, 0)
        : 0;

    function getPlatformLabel(v: string) {
        return PLATFORMS.find(p => p.value === v)?.label ?? v;
    }
    function getPlatformColor(v: string) {
        return PLATFORMS.find(p => p.value === v)?.color ?? '#6B7280';
    }
    function getStatusBadge(v: string) {
        return STATUSES.find(s => s.value === v)?.badge ?? 'badge-neutral';
    }
    function getStatusLabel(v: string) {
        return STATUSES.find(s => s.value === v)?.label ?? v;
    }

    function toggleSort(col: typeof sortCol) {
        if (sortCol === col) sortAsc = !sortAsc;
        else { sortCol = col; sortAsc = false; }
    }

    function roiClass(roi: number | null): string {
        if (roi === null) return 'text-base-content/40';
        return Number(roi) >= 0 ? 'text-success font-semibold' : 'text-error font-semibold';
    }

    onMount(load);
</script>

<div class="p-6 space-y-6">

    <!-- Header -->
    <div class="flex flex-wrap items-center justify-between gap-3">
        <div>
            <h1 class="text-2xl font-bold">Marketing Reports</h1>
            <p class="text-base-content/60 text-sm">Campaign performance — budget, spend, revenue &amp; ROI</p>
        </div>
        <button class="btn btn-outline btn-sm" on:click={load} disabled={loading}>
            <RefreshCw size={14} class={loading ? 'animate-spin' : ''} /> Refresh
        </button>
    </div>

    <!-- Filters -->
    <div class="flex flex-wrap gap-3 bg-base-200/40 p-3 rounded-box">
        <div>
            <label for="rpt-start" class="label py-0"><span class="label-text text-xs">From</span></label>
            <input id="rpt-start" type="date" class="input input-bordered input-sm" bind:value={filterStartDate} />
        </div>
        <div>
            <label for="rpt-end" class="label py-0"><span class="label-text text-xs">To</span></label>
            <input id="rpt-end" type="date" class="input input-bordered input-sm" bind:value={filterEndDate} />
        </div>
        <div>
            <label for="rpt-platform" class="label py-0"><span class="label-text text-xs">Platform</span></label>
            <select id="rpt-platform" class="select select-bordered select-sm" bind:value={filterPlatform}>
                {#each PLATFORMS as p}<option value={p.value}>{p.label}</option>{/each}
            </select>
        </div>
        <div>
            <label for="rpt-status" class="label py-0"><span class="label-text text-xs">Status</span></label>
            <select id="rpt-status" class="select select-bordered select-sm" bind:value={filterStatus}>
                {#each STATUSES as s}<option value={s.value}>{s.label}</option>{/each}
            </select>
        </div>
        <div class="flex items-end">
            <button class="btn btn-primary btn-sm" on:click={load} disabled={loading}>
                {#if loading}<span class="loading loading-spinner loading-xs"></span>{/if}
                Apply
            </button>
        </div>
    </div>

    {#if loading}
        <div class="flex justify-center py-24"><span class="loading loading-spinner loading-lg"></span></div>
    {:else if error}
        <div class="alert alert-error">{error}</div>
    {:else if report}

        <!-- ── KPI Cards ─────────────────────────────────────────────────── -->
        <div class="grid grid-cols-2 lg:grid-cols-3 xl:grid-cols-6 gap-4">
            <div class="stat bg-base-100 rounded-box border border-base-300 p-4">
                <div class="stat-figure text-primary"><BarChart2 size={26}/></div>
                <div class="stat-title text-xs">Total Campaigns</div>
                <div class="stat-value text-xl">{report.summary.total_campaigns}</div>
                <div class="stat-desc">{report.summary.active_campaigns} active</div>
            </div>
            <div class="stat bg-base-100 rounded-box border border-base-300 p-4">
                <div class="stat-figure text-info"><DollarSign size={26}/></div>
                <div class="stat-title text-xs">Total Budget</div>
                <div class="stat-value text-xl">{formatCurrency(Number(report.summary.total_budget))}</div>
            </div>
            <div class="stat bg-base-100 rounded-box border border-base-300 p-4">
                <div class="stat-figure text-warning"><Target size={26}/></div>
                <div class="stat-title text-xs">Total Spent</div>
                <div class="stat-value text-xl">{formatCurrency(Number(report.summary.total_spent))}</div>
                <div class="stat-desc">
                    {#if Number(report.summary.total_budget) > 0}
                        {((Number(report.summary.total_spent) / Number(report.summary.total_budget)) * 100).toFixed(0)}% of budget
                    {/if}
                </div>
            </div>
            <div class="stat bg-base-100 rounded-box border border-base-300 p-4">
                <div class="stat-figure text-success"><TrendingUp size={26}/></div>
                <div class="stat-title text-xs">Total Revenue</div>
                <div class="stat-value text-xl">{formatCurrency(Number(report.summary.total_revenue))}</div>
            </div>
            <div class="stat bg-base-100 rounded-box border border-base-300 p-4">
                {#if (Number(report.summary.total_revenue) - Number(report.summary.total_spent)) >= 0}
                    <div class="stat-figure text-success"><DollarSign size={26}/></div>
                {:else}
                    <div class="stat-figure text-error"><DollarSign size={26}/></div>
                {/if}
                <div class="stat-title text-xs">Net Profit</div>
                <div class="stat-value text-xl {(Number(report.summary.total_revenue) - Number(report.summary.total_spent)) >= 0 ? 'text-success' : 'text-error'}">
                    {formatCurrency(Number(report.summary.total_revenue) - Number(report.summary.total_spent))}
                </div>
            </div>
            <div class="stat bg-base-100 rounded-box border border-base-300 p-4">
                <div class="stat-figure {report.summary.overall_roi !== null && Number(report.summary.overall_roi) >= 0 ? 'text-success' : 'text-error'}">
                    {#if report.summary.overall_roi !== null && Number(report.summary.overall_roi) >= 0}
                        <TrendingUp size={26}/>
                    {:else}
                        <TrendingDown size={26}/>
                    {/if}
                </div>
                <div class="stat-title text-xs">Overall ROI</div>
                <div class="stat-value text-xl {roiClass(report.summary.overall_roi !== null ? Number(report.summary.overall_roi) : null)}">
                    {formatROI(report.summary.overall_roi !== null ? Number(report.summary.overall_roi) : null)}
                </div>
                <div class="stat-desc">return on spend</div>
            </div>
        </div>

        <!-- ── Platform + Status breakdown ──────────────────────────────── -->
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">

            <!-- Platform performance -->
            <div class="lg:col-span-2 bg-base-100 rounded-box border border-base-300 p-4">
                <h2 class="font-semibold mb-3">Performance by Platform</h2>
                {#if report.by_platform.length === 0}
                    <p class="text-sm text-base-content/40 py-4 text-center">No data</p>
                {:else}
                    <div class="space-y-3">
                        {#each report.by_platform as p}
                            {@const barPct = Math.min((Number(p.total_revenue) / maxRevenue) * 100, 100)}
                            {@const color = getPlatformColor(p.platform)}
                            <div>
                                <div class="flex items-center justify-between text-sm mb-1">
                                    <div class="flex items-center gap-2">
                                        <span class="w-2 h-2 rounded-full flex-shrink-0" style="background:{color}"></span>
                                        <span class="font-medium">{getPlatformLabel(p.platform)}</span>
                                        <span class="text-xs text-base-content/50">{p.campaign_count} campaigns</span>
                                    </div>
                                    <div class="flex items-center gap-4 text-xs">
                                        <span class="text-base-content/60">Budget: {formatCurrency(Number(p.total_budget))}</span>
                                        <span class="text-base-content/60">Spent: {formatCurrency(Number(p.total_spent))}</span>
                                        <span class="font-semibold">Rev: {formatCurrency(Number(p.total_revenue))}</span>
                                        <span class="{roiClass(p.roi !== null ? Number(p.roi) : null)}">{formatROI(p.roi !== null ? Number(p.roi) : null)}</span>
                                    </div>
                                </div>
                                <div class="w-full bg-base-200 rounded-full h-2">
                                    <div class="h-2 rounded-full transition-all"
                                        style="width:{barPct}%; background:{color}80"></div>
                                </div>
                            </div>
                        {/each}
                    </div>
                {/if}
            </div>

            <!-- Status distribution -->
            <div class="bg-base-100 rounded-box border border-base-300 p-4">
                <h2 class="font-semibold mb-3">Campaign Status</h2>
                {#if report.by_status.length === 0}
                    <p class="text-sm text-base-content/40 py-4 text-center">No data</p>
                {:else}
                    <div class="space-y-2">
                        {#each report.by_status as s}
                            {@const pct = totalCampaignCount > 0 ? (s.count / totalCampaignCount) * 100 : 0}
                            <div>
                                <div class="flex items-center justify-between text-sm mb-1">
                                    <span class="badge badge-sm {getStatusBadge(s.status)}">{getStatusLabel(s.status)}</span>
                                    <span class="font-semibold">{s.count} <span class="text-xs text-base-content/50">({pct.toFixed(0)}%)</span></span>
                                </div>
                                <progress class="progress w-full h-2 {getStatusBadge(s.status).replace('badge-','progress-')}"
                                    value={s.count} max={totalCampaignCount}></progress>
                            </div>
                        {/each}
                    </div>
                {/if}
            </div>
        </div>

        <!-- ── Campaign performance table ───────────────────────────────── -->
        <div class="bg-base-100 rounded-box border border-base-300">
            <div class="p-4 border-b border-base-300 flex items-center justify-between">
                <h2 class="font-semibold">Campaign Breakdown</h2>
                <p class="text-xs text-base-content/50">Click column headers to sort</p>
            </div>
            {#if sortedCampaigns.length === 0}
                <p class="text-sm text-base-content/40 py-8 text-center">No campaigns match the selected filters</p>
            {:else}
                <div class="overflow-x-auto">
                    <table class="table table-sm">
                        <thead>
                            <tr>
                                <th>#</th>
                                <th>Campaign</th>
                                <th>Platform</th>
                                <th>Status</th>
                                <th class="text-right cursor-pointer select-none hover:bg-base-200"
                                    on:click={() => toggleSort('budget')}>
                                    Budget {sortCol==='budget' ? (sortAsc?'↑':'↓') : ''}
                                </th>
                                <th class="text-right cursor-pointer select-none hover:bg-base-200"
                                    on:click={() => toggleSort('spent')}>
                                    Spent {sortCol==='spent' ? (sortAsc?'↑':'↓') : ''}
                                </th>
                                <th class="text-right cursor-pointer select-none hover:bg-base-200"
                                    on:click={() => toggleSort('revenue')}>
                                    Revenue {sortCol==='revenue' ? (sortAsc?'↑':'↓') : ''}
                                </th>
                                <th class="text-right">Profit</th>
                                <th class="text-right cursor-pointer select-none hover:bg-base-200"
                                    on:click={() => toggleSort('roi')}>
                                    ROI {sortCol==='roi' ? (sortAsc?'↑':'↓') : ''}
                                </th>
                                <th>Budget Use</th>
                                <th>Period</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each sortedCampaigns as c, i (c.id)}
                                {@const profit = Number(c.revenue) - Number(c.spent)}
                                {@const budgetUsePct = Number(c.budget) > 0
                                    ? Math.min((Number(c.spent) / Number(c.budget)) * 100, 100) : 0}
                                <tr class="hover">
                                    <td class="text-base-content/40 text-xs">{i + 1}</td>
                                    <td class="font-medium max-w-xs">
                                        <div class="truncate">{c.name}</div>
                                    </td>
                                    <td>
                                        <span class="badge badge-sm font-medium"
                                            style="background:{getPlatformColor(c.platform)}20;color:{getPlatformColor(c.platform)};border-color:{getPlatformColor(c.platform)}40">
                                            {getPlatformLabel(c.platform)}
                                        </span>
                                    </td>
                                    <td><span class="badge badge-sm {getStatusBadge(c.status)}">{getStatusLabel(c.status)}</span></td>
                                    <td class="text-right font-mono text-xs">{formatCurrency(Number(c.budget))}</td>
                                    <td class="text-right font-mono text-xs {Number(c.spent) > Number(c.budget) ? 'text-error' : ''}">
                                        {formatCurrency(Number(c.spent))}
                                    </td>
                                    <td class="text-right font-mono text-xs">{formatCurrency(Number(c.revenue))}</td>
                                    <td class="text-right font-mono text-xs {profit >= 0 ? 'text-success' : 'text-error'}">
                                        {formatCurrency(profit)}
                                    </td>
                                    <td class="text-right font-mono text-sm {roiClass(c.roi !== null ? Number(c.roi) : null)}">
                                        {formatROI(c.roi !== null ? Number(c.roi) : null)}
                                    </td>
                                    <td class="min-w-24">
                                        <div class="flex items-center gap-1">
                                            <progress class="progress progress-warning h-1.5 flex-1"
                                                value={budgetUsePct} max={100}></progress>
                                            <span class="text-xs text-base-content/50">{budgetUsePct.toFixed(0)}%</span>
                                        </div>
                                    </td>
                                    <td class="text-xs text-base-content/50">
                                        {#if c.start_date || c.end_date}
                                            <div>{c.start_date ?? '—'}</div>
                                            <div>{c.end_date ?? '—'}</div>
                                        {:else}—{/if}
                                    </td>
                                </tr>
                            {/each}
                        </tbody>
                        <tfoot class="bg-base-200/50 font-semibold">
                            <tr>
                                <td colspan="4">Totals</td>
                                <td class="text-right font-mono text-xs">{formatCurrency(Number(report.summary.total_budget))}</td>
                                <td class="text-right font-mono text-xs">{formatCurrency(Number(report.summary.total_spent))}</td>
                                <td class="text-right font-mono text-xs">{formatCurrency(Number(report.summary.total_revenue))}</td>
                                <td class="text-right font-mono text-xs {Number(report.summary.total_revenue) - Number(report.summary.total_spent) >= 0 ? 'text-success' : 'text-error'}">
                                    {formatCurrency(Number(report.summary.total_revenue) - Number(report.summary.total_spent))}
                                </td>
                                <td class="text-right font-mono text-sm {roiClass(report.summary.overall_roi !== null ? Number(report.summary.overall_roi) : null)}">
                                    {formatROI(report.summary.overall_roi !== null ? Number(report.summary.overall_roi) : null)}
                                </td>
                                <td colspan="2"></td>
                            </tr>
                        </tfoot>
                    </table>
                </div>
            {/if}
        </div>

    {/if}
</div>
