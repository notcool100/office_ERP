<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, Megaphone, BarChart2, Plus, Pencil, Trash2, Search, X, TrendingUp, TrendingDown, DollarSign, Target } from 'lucide-svelte';
    import { onMount } from 'svelte';
    import { navigationStore, canCreate, canUpdate, canDelete } from '$lib/stores/navigation';
    import {
        campaignService, PLATFORMS, STATUSES,
        getPlatformLabel, getPlatformColor, getStatusBadge, getStatusLabel,
        formatCurrency, formatROI,
    } from '$lib/services/campaigns';
    import type { Campaign, CreateCampaignDto } from '$lib/services/campaigns';

    pageTitle.set({ title: 'Campaign Manager', desc: 'Track campaigns, budgets, and ROI' });
    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Digital Marketing', icon: Megaphone },
        { label: 'Campaigns', icon: BarChart2 },
    ]);

    const navPath = '/admin/digital-marketing/campaigns';

    // ── State ────────────────────────────────────────────────────────────
    let campaigns: Campaign[] = [];
    let loading = true;
    let error = '';
    let filterPlatform = '';
    let filterStatus = '';
    let searchInput = '';
    let searchTerm = '';
    let searchTimeout: ReturnType<typeof setTimeout>;

    // Modal
    let showModal = false;
    let editingId: string | null = null;
    let saving = false;
    let saveError = '';

    // Form fields
    let fName = '';
    let fDescription = '';
    let fPlatform = 'google_ads';
    let fStatus = 'draft';
    let fBudget = '';
    let fSpent = '';
    let fRevenue = '';
    let fStartDate = '';
    let fEndDate = '';
    let fTargetAudience = '';
    let fGoals = '';

    // ── Computed summary ─────────────────────────────────────────────────
    $: totalBudget  = campaigns.reduce((s, c) => s + Number(c.budget),  0);
    $: totalSpent   = campaigns.reduce((s, c) => s + Number(c.spent),   0);
    $: totalRevenue = campaigns.reduce((s, c) => s + Number(c.revenue), 0);
    $: overallROI   = totalSpent > 0 ? ((totalRevenue - totalSpent) / totalSpent * 100) : null;
    $: activeCampaigns = campaigns.filter(c => c.status === 'active').length;

    // ── Data ─────────────────────────────────────────────────────────────
    async function load() {
        loading = true; error = '';
        try {
            campaigns = await campaignService.list({
                platform: filterPlatform || undefined,
                status: filterStatus || undefined,
                search: searchTerm || undefined,
            });
        } catch (e: any) {
            error = e.message ?? 'Failed to load';
        } finally {
            loading = false;
        }
    }

    function onSearchInput() {
        clearTimeout(searchTimeout);
        searchTimeout = setTimeout(() => { searchTerm = searchInput; load(); }, 400);
    }

    // ── Modal ─────────────────────────────────────────────────────────────
    function openCreate() {
        editingId = null;
        fName = ''; fDescription = ''; fPlatform = 'google_ads'; fStatus = 'draft';
        fBudget = ''; fSpent = ''; fRevenue = '';
        fStartDate = ''; fEndDate = ''; fTargetAudience = ''; fGoals = '';
        saveError = '';
        showModal = true;
    }

    function openEdit(c: Campaign) {
        editingId = c.id;
        fName = c.name;
        fDescription = c.description ?? '';
        fPlatform = c.platform;
        fStatus = c.status;
        fBudget = String(c.budget);
        fSpent = String(c.spent);
        fRevenue = String(c.revenue);
        fStartDate = c.start_date ?? '';
        fEndDate = c.end_date ?? '';
        fTargetAudience = c.target_audience ?? '';
        fGoals = c.goals ?? '';
        saveError = '';
        showModal = true;
    }

    async function submitModal() {
        if (!fName.trim()) { saveError = 'Name is required'; return; }
        saving = true; saveError = '';
        try {
            const dto: CreateCampaignDto = {
                name: fName.trim(),
                description: fDescription || undefined,
                platform: fPlatform,
                status: fStatus,
                budget: fBudget ? Number(fBudget) : 0,
                spent: fSpent ? Number(fSpent) : 0,
                revenue: fRevenue ? Number(fRevenue) : 0,
                start_date: fStartDate || undefined,
                end_date: fEndDate || undefined,
                target_audience: fTargetAudience || undefined,
                goals: fGoals || undefined,
            };
            if (editingId) {
                const updated = await campaignService.update(editingId, dto);
                campaigns = campaigns.map(c => c.id === updated.id ? updated : c);
            } else {
                const created = await campaignService.create(dto);
                campaigns = [created, ...campaigns];
            }
            showModal = false;
        } catch (e: any) {
            saveError = e.message ?? 'Failed to save';
        } finally {
            saving = false;
        }
    }

    async function deleteCampaign(c: Campaign) {
        if (!confirm(`Delete campaign "${c.name}"? This cannot be undone.`)) return;
        try {
            await campaignService.delete(c.id);
            campaigns = campaigns.filter(x => x.id !== c.id);
        } catch (e: any) {
            alert(e.message ?? 'Failed to delete');
        }
    }

    onMount(load);
</script>

<!-- ── Modal ─────────────────────────────────────────────────────────────── -->
{#if showModal}
<div class="modal modal-open">
    <div class="modal-box max-w-2xl">
        <h3 class="font-bold text-lg mb-4">{editingId ? 'Edit Campaign' : 'New Campaign'}</h3>

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
            <!-- Name -->
            <div class="sm:col-span-2">
                <label for="cm-name" class="label"><span class="label-text">Campaign Name <span class="text-error">*</span></span></label>
                <input id="cm-name" type="text" class="input input-bordered w-full"
                    placeholder="e.g. Summer Sale 2024" bind:value={fName} />
            </div>

            <!-- Platform -->
            <div>
                <label for="cm-platform" class="label"><span class="label-text">Platform</span></label>
                <select id="cm-platform" class="select select-bordered w-full" bind:value={fPlatform}>
                    {#each PLATFORMS.slice(1) as p}
                        <option value={p.value}>{p.label}</option>
                    {/each}
                </select>
            </div>

            <!-- Status -->
            <div>
                <label for="cm-status" class="label"><span class="label-text">Status</span></label>
                <select id="cm-status" class="select select-bordered w-full" bind:value={fStatus}>
                    {#each STATUSES.slice(1) as s}
                        <option value={s.value}>{s.label}</option>
                    {/each}
                </select>
            </div>

            <!-- Budget / Spent / Revenue -->
            <div>
                <label for="cm-budget" class="label"><span class="label-text">Budget ($)</span></label>
                <input id="cm-budget" type="number" min="0" step="0.01" class="input input-bordered w-full"
                    placeholder="0.00" bind:value={fBudget} />
            </div>
            <div>
                <label for="cm-spent" class="label"><span class="label-text">Spent ($)</span></label>
                <input id="cm-spent" type="number" min="0" step="0.01" class="input input-bordered w-full"
                    placeholder="0.00" bind:value={fSpent} />
            </div>
            <div>
                <label for="cm-revenue" class="label"><span class="label-text">Revenue ($)</span></label>
                <input id="cm-revenue" type="number" min="0" step="0.01" class="input input-bordered w-full"
                    placeholder="0.00" bind:value={fRevenue} />
            </div>

            <!-- ROI preview -->
            <div class="flex items-end pb-2">
                {#if fSpent && Number(fSpent) > 0}
                    {@const roi = ((Number(fRevenue || 0) - Number(fSpent)) / Number(fSpent) * 100)}
                    <div class="stat p-0">
                        <div class="stat-title text-xs">Projected ROI</div>
                        <div class="stat-value text-lg {roi >= 0 ? 'text-success' : 'text-error'}">
                            {roi > 0 ? '+' : ''}{roi.toFixed(1)}%
                        </div>
                    </div>
                {:else}
                    <p class="text-xs text-base-content/40">Enter spent amount to see ROI</p>
                {/if}
            </div>

            <!-- Dates -->
            <div>
                <label for="cm-start" class="label"><span class="label-text">Start Date</span></label>
                <input id="cm-start" type="date" class="input input-bordered w-full" bind:value={fStartDate} />
            </div>
            <div>
                <label for="cm-end" class="label"><span class="label-text">End Date</span></label>
                <input id="cm-end" type="date" class="input input-bordered w-full" bind:value={fEndDate} />
            </div>

            <!-- Target audience -->
            <div class="sm:col-span-2">
                <label for="cm-audience" class="label"><span class="label-text">Target Audience</span></label>
                <input id="cm-audience" type="text" class="input input-bordered w-full"
                    placeholder="e.g. 18-35 year olds in Kathmandu" bind:value={fTargetAudience} />
            </div>

            <!-- Goals -->
            <div class="sm:col-span-2">
                <label for="cm-goals" class="label"><span class="label-text">Goals</span></label>
                <textarea id="cm-goals" class="textarea textarea-bordered w-full" rows="2"
                    placeholder="Campaign objectives…" bind:value={fGoals}></textarea>
            </div>

            <!-- Description -->
            <div class="sm:col-span-2">
                <label for="cm-desc" class="label"><span class="label-text">Description</span></label>
                <textarea id="cm-desc" class="textarea textarea-bordered w-full" rows="2"
                    placeholder="Additional notes…" bind:value={fDescription}></textarea>
            </div>
        </div>

        {#if saveError}
            <div class="alert alert-error mt-3 text-sm">{saveError}</div>
        {/if}

        <div class="modal-action">
            <button class="btn btn-ghost" on:click={() => showModal = false} disabled={saving}>Cancel</button>
            <button class="btn btn-primary" on:click={submitModal} disabled={saving}>
                {#if saving}<span class="loading loading-spinner loading-xs"></span>{/if}
                {editingId ? 'Save Changes' : 'Create Campaign'}
            </button>
        </div>
    </div>
    <button class="modal-backdrop" aria-label="Close modal" on:click={() => showModal = false}></button>
</div>
{/if}

<!-- ── Page ──────────────────────────────────────────────────────────────── -->
<div class="p-6 space-y-6">

    <!-- Header -->
    <div class="flex flex-wrap items-center justify-between gap-3">
        <div>
            <h1 class="text-2xl font-bold">Campaign Manager</h1>
            <p class="text-base-content/60 text-sm">Track budgets, spend, and ROI across all platforms</p>
        </div>
        {#if canCreate(navPath, $navigationStore)}
            <button class="btn btn-primary" on:click={openCreate}>
                <Plus size={16} /> New Campaign
            </button>
        {/if}
    </div>

    <!-- Summary cards -->
    {#if !loading && campaigns.length > 0}
    <div class="grid grid-cols-2 lg:grid-cols-4 gap-4">
        <div class="stat bg-base-100 rounded-box border border-base-300 p-4">
            <div class="stat-figure text-primary"><DollarSign size={28} /></div>
            <div class="stat-title text-xs">Total Budget</div>
            <div class="stat-value text-lg">{formatCurrency(totalBudget)}</div>
            <div class="stat-desc">{activeCampaigns} active</div>
        </div>
        <div class="stat bg-base-100 rounded-box border border-base-300 p-4">
            <div class="stat-figure text-warning"><Target size={28} /></div>
            <div class="stat-title text-xs">Total Spent</div>
            <div class="stat-value text-lg">{formatCurrency(totalSpent)}</div>
            <div class="stat-desc">{totalBudget > 0 ? ((totalSpent/totalBudget)*100).toFixed(0) : 0}% of budget</div>
        </div>
        <div class="stat bg-base-100 rounded-box border border-base-300 p-4">
            <div class="stat-figure text-success"><TrendingUp size={28} /></div>
            <div class="stat-title text-xs">Total Revenue</div>
            <div class="stat-value text-lg">{formatCurrency(totalRevenue)}</div>
            <div class="stat-desc">across all campaigns</div>
        </div>
        <div class="stat bg-base-100 rounded-box border border-base-300 p-4">
            <div class="stat-figure {overallROI !== null && overallROI >= 0 ? 'text-success' : 'text-error'}">
                {#if overallROI !== null && overallROI >= 0}
                    <TrendingUp size={28} />
                {:else}
                    <TrendingDown size={28} />
                {/if}
            </div>
            <div class="stat-title text-xs">Overall ROI</div>
            <div class="stat-value text-lg {overallROI !== null && overallROI >= 0 ? 'text-success' : 'text-error'}">
                {formatROI(overallROI)}
            </div>
            <div class="stat-desc">return on spend</div>
        </div>
    </div>
    {/if}

    <!-- Filters -->
    <div class="flex flex-wrap gap-3">
        <label class="input input-bordered flex items-center gap-2 flex-1 min-w-52">
            <Search size={16} class="text-base-content/50" />
            <input type="text" placeholder="Search campaigns…" class="grow"
                bind:value={searchInput} on:input={onSearchInput} />
            {#if searchInput}
                <button on:click={() => { searchInput=''; searchTerm=''; load(); }}><X size={14}/></button>
            {/if}
        </label>
        <select class="select select-bordered" bind:value={filterPlatform} on:change={load}>
            {#each PLATFORMS as p}
                <option value={p.value}>{p.label}</option>
            {/each}
        </select>
        <select class="select select-bordered" bind:value={filterStatus} on:change={load}>
            {#each STATUSES as s}
                <option value={s.value}>{s.label}</option>
            {/each}
        </select>
    </div>

    <!-- Table -->
    {#if loading}
        <div class="flex justify-center py-20"><span class="loading loading-spinner loading-lg"></span></div>
    {:else if error}
        <div class="alert alert-error">{error}</div>
    {:else if campaigns.length === 0}
        <div class="flex flex-col items-center justify-center py-20 gap-4 text-base-content/40">
            <BarChart2 size={64} />
            <p class="text-lg">No campaigns found</p>
            {#if canCreate(navPath, $navigationStore)}
                <button class="btn btn-primary btn-sm" on:click={openCreate}>Create your first campaign</button>
            {/if}
        </div>
    {:else}
        <div class="overflow-x-auto rounded-box border border-base-300">
            <table class="table table-sm">
                <thead>
                    <tr>
                        <th>Campaign</th>
                        <th>Platform</th>
                        <th>Status</th>
                        <th class="text-right">Budget</th>
                        <th class="text-right">Spent</th>
                        <th class="text-right">Revenue</th>
                        <th class="text-right">ROI</th>
                        <th>Dates</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    {#each campaigns as c (c.id)}
                        <tr class="hover">
                            <!-- Campaign name + assigned -->
                            <td>
                                <div class="font-medium">{c.name}</div>
                                {#if c.assigned_to_name}
                                    <div class="text-xs text-base-content/50">{c.assigned_to_name}</div>
                                {/if}
                            </td>

                            <!-- Platform chip -->
                            <td>
                                <span class="badge badge-sm font-medium"
                                    style="background-color:{getPlatformColor(c.platform)}20; color:{getPlatformColor(c.platform)}; border-color:{getPlatformColor(c.platform)}40">
                                    {getPlatformLabel(c.platform)}
                                </span>
                            </td>

                            <!-- Status -->
                            <td>
                                <span class="badge badge-sm {getStatusBadge(c.status)}">
                                    {getStatusLabel(c.status)}
                                </span>
                            </td>

                            <!-- Financials -->
                            <td class="text-right font-mono text-sm">{formatCurrency(Number(c.budget))}</td>
                            <td class="text-right font-mono text-sm">
                                <span class="{Number(c.spent) > Number(c.budget) ? 'text-error' : ''}">
                                    {formatCurrency(Number(c.spent))}
                                </span>
                                {#if Number(c.budget) > 0}
                                    <div class="w-16 ml-auto mt-0.5">
                                        <progress class="progress progress-warning h-1"
                                            value={Math.min(Number(c.spent), Number(c.budget))}
                                            max={Number(c.budget)}></progress>
                                    </div>
                                {/if}
                            </td>
                            <td class="text-right font-mono text-sm">{formatCurrency(Number(c.revenue))}</td>

                            <!-- ROI -->
                            <td class="text-right font-mono text-sm font-semibold
                                {c.roi === null ? 'text-base-content/40' :
                                 Number(c.roi) >= 0 ? 'text-success' : 'text-error'}">
                                {formatROI(c.roi !== null ? Number(c.roi) : null)}
                            </td>

                            <!-- Dates -->
                            <td class="text-xs text-base-content/60">
                                {#if c.start_date || c.end_date}
                                    <div>{c.start_date ?? '—'}</div>
                                    <div>{c.end_date ?? '—'}</div>
                                {:else}
                                    —
                                {/if}
                            </td>

                            <!-- Actions -->
                            <td>
                                <div class="flex gap-1 justify-end">
                                    {#if canUpdate(navPath, $navigationStore)}
                                        <button class="btn btn-ghost btn-xs" title="Edit" on:click={() => openEdit(c)}>
                                            <Pencil size={13} />
                                        </button>
                                    {/if}
                                    {#if canDelete(navPath, $navigationStore)}
                                        <button class="btn btn-ghost btn-xs text-error" title="Delete"
                                            on:click={() => deleteCampaign(c)}>
                                            <Trash2 size={13} />
                                        </button>
                                    {/if}
                                </div>
                            </td>
                        </tr>

                        <!-- Expandable goals/audience row -->
                        {#if c.target_audience || c.goals}
                            <tr class="bg-base-200/30 border-t-0">
                                <td colspan="9" class="py-1 px-4">
                                    <div class="text-xs text-base-content/60 flex flex-wrap gap-4">
                                        {#if c.target_audience}
                                            <span><strong>Audience:</strong> {c.target_audience}</span>
                                        {/if}
                                        {#if c.goals}
                                            <span><strong>Goals:</strong> {c.goals}</span>
                                        {/if}
                                    </div>
                                </td>
                            </tr>
                        {/if}
                    {/each}
                </tbody>
                <!-- Totals footer -->
                <tfoot class="bg-base-200/50 font-semibold text-sm">
                    <tr>
                        <td colspan="3">Totals ({campaigns.length} campaigns)</td>
                        <td class="text-right font-mono">{formatCurrency(totalBudget)}</td>
                        <td class="text-right font-mono">{formatCurrency(totalSpent)}</td>
                        <td class="text-right font-mono">{formatCurrency(totalRevenue)}</td>
                        <td class="text-right font-mono {overallROI !== null && overallROI >= 0 ? 'text-success' : 'text-error'}">
                            {formatROI(overallROI)}
                        </td>
                        <td colspan="2"></td>
                    </tr>
                </tfoot>
            </table>
        </div>
    {/if}
</div>
