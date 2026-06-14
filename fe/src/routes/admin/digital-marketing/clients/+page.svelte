<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import {
        Home, Megaphone, Briefcase, Plus, Pencil, Trash2,
        Search, X, ChevronDown, ChevronRight, AlertCircle, Clock,
        Building2, Mail, Phone, Globe, PackageCheck
    } from 'lucide-svelte';
    import { onMount } from 'svelte';
    import { navigationStore, canCreate, canUpdate, canDelete } from '$lib/stores/navigation';
    import {
        clientService,
        CLIENT_STATUSES, DELIVERABLE_STATUSES, DELIVERABLE_TYPES,
        getClientStatusBadge, getClientStatusLabel,
        getDelivStatusBadge, getDelivStatusLabel, getDelivTypeLabel,
        isDueSoon, isOverdue,
    } from '$lib/services/client-management';
    import type { Client, Deliverable } from '$lib/services/client-management';

    pageTitle.set({ title: 'Client Management', desc: 'Track clients, campaigns, and deliverables' });
    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Digital Marketing', icon: Megaphone },
        { label: 'Clients', icon: Briefcase },
    ]);

    const navPath = '/admin/digital-marketing/clients';

    // ── State ─────────────────────────────────────────────────────────────────
    let activeTab: 'clients' | 'deliverables' = 'clients';

    let clients: Client[] = [];
    let deliverables: Deliverable[] = [];
    let loadingClients = true;
    let loadingDelivs = false;
    let error = '';

    let expandedClients = new Set<string>();
    let clientDeliverables: Record<string, Deliverable[]> = {};

    // Filters
    let clientStatusFilter = '';
    let clientSearch = '';
    let clientSearchInput = '';
    let delivStatusFilter = '';
    let delivClientFilter = '';
    let delivSearch = '';
    let delivSearchInput = '';
    let searchTimeout: ReturnType<typeof setTimeout>;

    // Client modal
    let showClientModal = false;
    let editingClientId: string | null = null;
    let cName = '', cContact = '', cEmail = '', cPhone = '';
    let cIndustry = '', cWebsite = '', cAddress = '', cNotes = '';
    let cStatus = 'active';
    let cSaving = false, cError = '';

    // Deliverable modal
    let showDelivModal = false;
    let editingDelivId: string | null = null;
    let dClientId = '', dCampaignId = '', dTitle = '', dDescription = '';
    let dType = 'other', dStatus = 'pending', dDueDate = '', dAssignedTo = '';
    let dSaving = false, dError = '';
    // prefill client when adding from client row
    let prefillClientId = '';

    // ── Load data ─────────────────────────────────────────────────────────────
    async function loadClients() {
        loadingClients = true; error = '';
        try {
            clients = await clientService.listClients({
                status: clientStatusFilter || undefined,
                search: clientSearch || undefined,
            });
        } catch (e: any) { error = e.message; }
        finally { loadingClients = false; }
    }

    async function loadDeliverables() {
        loadingDelivs = true; error = '';
        try {
            deliverables = await clientService.listDeliverables({
                client_id: delivClientFilter || undefined,
                status: delivStatusFilter || undefined,
                search: delivSearch || undefined,
            });
        } catch (e: any) { error = e.message; }
        finally { loadingDelivs = false; }
    }

    async function toggleExpand(client: Client) {
        if (expandedClients.has(client.id)) {
            expandedClients.delete(client.id);
            expandedClients = new Set(expandedClients);
        } else {
            expandedClients.add(client.id);
            expandedClients = new Set(expandedClients);
            if (!clientDeliverables[client.id]) {
                const rows = await clientService.listDeliverables({ client_id: client.id });
                clientDeliverables[client.id] = rows;
                clientDeliverables = { ...clientDeliverables };
            }
        }
    }

    function onClientSearch() {
        clearTimeout(searchTimeout);
        searchTimeout = setTimeout(() => { clientSearch = clientSearchInput; loadClients(); }, 400);
    }
    function onDelivSearch() {
        clearTimeout(searchTimeout);
        searchTimeout = setTimeout(() => { delivSearch = delivSearchInput; loadDeliverables(); }, 400);
    }

    // ── Client modal ──────────────────────────────────────────────────────────
    function openClientCreate() {
        editingClientId = null;
        cName = ''; cContact = ''; cEmail = ''; cPhone = '';
        cIndustry = ''; cWebsite = ''; cAddress = ''; cNotes = '';
        cStatus = 'active'; cError = '';
        showClientModal = true;
    }

    function openClientEdit(c: Client) {
        editingClientId = c.id;
        cName = c.name; cContact = c.contact_person ?? '';
        cEmail = c.email ?? ''; cPhone = c.phone ?? '';
        cIndustry = c.industry ?? ''; cWebsite = c.website ?? '';
        cAddress = c.address ?? ''; cNotes = c.notes ?? '';
        cStatus = c.status; cError = '';
        showClientModal = true;
    }

    async function submitClient() {
        if (!cName.trim()) { cError = 'Name is required'; return; }
        cSaving = true; cError = '';
        const dto = {
            name: cName.trim(), contact_person: cContact || undefined,
            email: cEmail || undefined, phone: cPhone || undefined,
            industry: cIndustry || undefined, website: cWebsite || undefined,
            address: cAddress || undefined, notes: cNotes || undefined,
            status: cStatus,
        };
        try {
            if (editingClientId) {
                const updated = await clientService.updateClient(editingClientId, dto);
                clients = clients.map(c => c.id === updated.id ? updated : c);
            } else {
                const created = await clientService.createClient(dto);
                clients = [created, ...clients];
            }
            showClientModal = false;
        } catch (e: any) { cError = e.message; }
        finally { cSaving = false; }
    }

    async function deleteClient(c: Client) {
        if (!confirm(`Delete client "${c.name}" and all their deliverables? This cannot be undone.`)) return;
        try {
            await clientService.deleteClient(c.id);
            clients = clients.filter(x => x.id !== c.id);
            delete clientDeliverables[c.id];
        } catch (e: any) { alert(e.message); }
    }

    // ── Deliverable modal ─────────────────────────────────────────────────────
    function openDelivCreate(preClientId = '') {
        editingDelivId = null;
        dClientId = preClientId;
        dCampaignId = ''; dTitle = ''; dDescription = '';
        dType = 'other'; dStatus = 'pending'; dDueDate = ''; dAssignedTo = '';
        dError = '';
        showDelivModal = true;
    }

    function openDelivEdit(d: Deliverable) {
        editingDelivId = d.id;
        dClientId = d.client_id; dCampaignId = d.campaign_id ?? '';
        dTitle = d.title; dDescription = d.description ?? '';
        dType = d.deliverable_type; dStatus = d.status;
        dDueDate = d.due_date ?? ''; dAssignedTo = d.assigned_to ?? '';
        dError = '';
        showDelivModal = true;
    }

    async function submitDeliv() {
        if (!dTitle.trim()) { dError = 'Title is required'; return; }
        if (!dClientId) { dError = 'Select a client'; return; }
        dSaving = true; dError = '';
        const dto = {
            client_id: dClientId,
            campaign_id: dCampaignId || undefined,
            title: dTitle.trim(),
            description: dDescription || undefined,
            deliverable_type: dType,
            due_date: dDueDate || undefined,
            status: dStatus,
            assigned_to: dAssignedTo || undefined,
        };
        try {
            if (editingDelivId) {
                const updated = await clientService.updateDeliverable(editingDelivId, dto);
                deliverables = deliverables.map(d => d.id === updated.id ? updated : d);
                // refresh expanded client panel
                if (clientDeliverables[updated.client_id]) {
                    clientDeliverables[updated.client_id] = clientDeliverables[updated.client_id]
                        .map(d => d.id === updated.id ? updated : d);
                    clientDeliverables = { ...clientDeliverables };
                }
            } else {
                const created = await clientService.createDeliverable(dto);
                deliverables = [created, ...deliverables];
                // update count + panel
                clients = clients.map(c => c.id === created.client_id
                    ? { ...c, deliverable_count: c.deliverable_count + 1 } : c);
                if (clientDeliverables[created.client_id]) {
                    clientDeliverables[created.client_id] = [...clientDeliverables[created.client_id], created];
                    clientDeliverables = { ...clientDeliverables };
                }
            }
            showDelivModal = false;
        } catch (e: any) { dError = e.message; }
        finally { dSaving = false; }
    }

    async function deleteDeliv(d: Deliverable) {
        if (!confirm(`Delete deliverable "${d.title}"?`)) return;
        try {
            await clientService.deleteDeliverable(d.id);
            deliverables = deliverables.filter(x => x.id !== d.id);
            clients = clients.map(c => c.id === d.client_id
                ? { ...c, deliverable_count: Math.max(0, c.deliverable_count - 1) } : c);
            if (clientDeliverables[d.client_id]) {
                clientDeliverables[d.client_id] = clientDeliverables[d.client_id].filter(x => x.id !== d.id);
                clientDeliverables = { ...clientDeliverables };
            }
        } catch (e: any) { alert(e.message); }
    }

    onMount(loadClients);

    $: if (activeTab === 'deliverables' && deliverables.length === 0 && !loadingDelivs) loadDeliverables();
</script>

<!-- ── Client Modal ───────────────────────────────────────────────────────── -->
{#if showClientModal}
<div class="modal modal-open">
    <div class="modal-box max-w-2xl">
        <h3 class="font-bold text-lg mb-4">{editingClientId ? 'Edit Client' : 'Add Client'}</h3>
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
            <div class="sm:col-span-2">
                <label for="cl-name" class="label"><span class="label-text">Company / Client Name <span class="text-error">*</span></span></label>
                <input id="cl-name" type="text" class="input input-bordered w-full" bind:value={cName} placeholder="e.g. Acme Corp" />
            </div>
            <div>
                <label for="cl-contact" class="label"><span class="label-text">Contact Person</span></label>
                <input id="cl-contact" type="text" class="input input-bordered w-full" bind:value={cContact} placeholder="Full name" />
            </div>
            <div>
                <label for="cl-status" class="label"><span class="label-text">Status</span></label>
                <select id="cl-status" class="select select-bordered w-full" bind:value={cStatus}>
                    {#each CLIENT_STATUSES.slice(1) as s}
                        <option value={s.value}>{s.label}</option>
                    {/each}
                </select>
            </div>
            <div>
                <label for="cl-email" class="label"><span class="label-text">Email</span></label>
                <input id="cl-email" type="email" class="input input-bordered w-full" bind:value={cEmail} placeholder="contact@company.com" />
            </div>
            <div>
                <label for="cl-phone" class="label"><span class="label-text">Phone</span></label>
                <input id="cl-phone" type="text" class="input input-bordered w-full" bind:value={cPhone} placeholder="+977 ..." />
            </div>
            <div>
                <label for="cl-industry" class="label"><span class="label-text">Industry</span></label>
                <input id="cl-industry" type="text" class="input input-bordered w-full" bind:value={cIndustry} placeholder="e.g. E-commerce" />
            </div>
            <div>
                <label for="cl-website" class="label"><span class="label-text">Website</span></label>
                <input id="cl-website" type="text" class="input input-bordered w-full" bind:value={cWebsite} placeholder="https://..." />
            </div>
            <div class="sm:col-span-2">
                <label for="cl-address" class="label"><span class="label-text">Address</span></label>
                <input id="cl-address" type="text" class="input input-bordered w-full" bind:value={cAddress} placeholder="City, Country" />
            </div>
            <div class="sm:col-span-2">
                <label for="cl-notes" class="label"><span class="label-text">Notes</span></label>
                <textarea id="cl-notes" class="textarea textarea-bordered w-full" rows="2" bind:value={cNotes}></textarea>
            </div>
        </div>
        {#if cError}<div class="alert alert-error mt-3 text-sm">{cError}</div>{/if}
        <div class="modal-action">
            <button class="btn btn-ghost" on:click={() => showClientModal = false} disabled={cSaving}>Cancel</button>
            <button class="btn btn-primary" on:click={submitClient} disabled={cSaving}>
                {#if cSaving}<span class="loading loading-spinner loading-xs"></span>{/if}
                {editingClientId ? 'Save Changes' : 'Add Client'}
            </button>
        </div>
    </div>
    <button class="modal-backdrop" aria-label="Close" on:click={() => showClientModal = false}></button>
</div>
{/if}

<!-- ── Deliverable Modal ──────────────────────────────────────────────────── -->
{#if showDelivModal}
<div class="modal modal-open">
    <div class="modal-box max-w-2xl">
        <h3 class="font-bold text-lg mb-4">{editingDelivId ? 'Edit Deliverable' : 'New Deliverable'}</h3>
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
            <div class="sm:col-span-2">
                <label for="dl-title" class="label"><span class="label-text">Title <span class="text-error">*</span></span></label>
                <input id="dl-title" type="text" class="input input-bordered w-full" bind:value={dTitle} placeholder="e.g. Instagram posts for March" />
            </div>
            <div>
                <label for="dl-client" class="label"><span class="label-text">Client <span class="text-error">*</span></span></label>
                <select id="dl-client" class="select select-bordered w-full" bind:value={dClientId}>
                    <option value="">— Select client —</option>
                    {#each clients as c}
                        <option value={c.id}>{c.name}</option>
                    {/each}
                </select>
            </div>
            <div>
                <label for="dl-type" class="label"><span class="label-text">Type</span></label>
                <select id="dl-type" class="select select-bordered w-full" bind:value={dType}>
                    {#each DELIVERABLE_TYPES as t}
                        <option value={t.value}>{t.label}</option>
                    {/each}
                </select>
            </div>
            <div>
                <label for="dl-status" class="label"><span class="label-text">Status</span></label>
                <select id="dl-status" class="select select-bordered w-full" bind:value={dStatus}>
                    {#each DELIVERABLE_STATUSES.slice(1) as s}
                        <option value={s.value}>{s.label}</option>
                    {/each}
                </select>
            </div>
            <div>
                <label for="dl-due" class="label"><span class="label-text">Due Date</span></label>
                <input id="dl-due" type="date" class="input input-bordered w-full" bind:value={dDueDate} />
            </div>
            <div class="sm:col-span-2">
                <label for="dl-desc" class="label"><span class="label-text">Description</span></label>
                <textarea id="dl-desc" class="textarea textarea-bordered w-full" rows="2" bind:value={dDescription}></textarea>
            </div>
        </div>
        {#if dError}<div class="alert alert-error mt-3 text-sm">{dError}</div>{/if}
        <div class="modal-action">
            <button class="btn btn-ghost" on:click={() => showDelivModal = false} disabled={dSaving}>Cancel</button>
            <button class="btn btn-primary" on:click={submitDeliv} disabled={dSaving}>
                {#if dSaving}<span class="loading loading-spinner loading-xs"></span>{/if}
                {editingDelivId ? 'Save Changes' : 'Add Deliverable'}
            </button>
        </div>
    </div>
    <button class="modal-backdrop" aria-label="Close" on:click={() => showDelivModal = false}></button>
</div>
{/if}

<!-- ── Page ───────────────────────────────────────────────────────────────── -->
<div class="p-6 space-y-5">

    <!-- Header -->
    <div class="flex flex-wrap items-center justify-between gap-3">
        <div>
            <h1 class="text-2xl font-bold">Client Management</h1>
            <p class="text-base-content/60 text-sm">Track clients, linked campaigns, and deliverables</p>
        </div>
        <div class="flex gap-2">
            {#if canCreate(navPath, $navigationStore)}
                <button class="btn btn-outline btn-sm" on:click={() => openDelivCreate()}>
                    <PackageCheck size={15} /> New Deliverable
                </button>
                <button class="btn btn-primary btn-sm" on:click={openClientCreate}>
                    <Plus size={15} /> Add Client
                </button>
            {/if}
        </div>
    </div>

    <!-- Tabs -->
    <div class="tabs tabs-bordered">
        <button class="tab {activeTab === 'clients' ? 'tab-active' : ''}"
            on:click={() => { activeTab = 'clients'; loadClients(); }}>
            <Building2 size={15} class="mr-1" /> Clients ({clients.length})
        </button>
        <button class="tab {activeTab === 'deliverables' ? 'tab-active' : ''}"
            on:click={() => { activeTab = 'deliverables'; loadDeliverables(); }}>
            <PackageCheck size={15} class="mr-1" /> All Deliverables ({deliverables.length})
        </button>
    </div>

    <!-- ── CLIENTS TAB ──────────────────────────────────────────────────── -->
    {#if activeTab === 'clients'}
        <!-- Filters -->
        <div class="flex flex-wrap gap-3">
            <label class="input input-bordered flex items-center gap-2 flex-1 min-w-48">
                <Search size={15} class="text-base-content/50" />
                <input type="text" placeholder="Search clients…" class="grow"
                    bind:value={clientSearchInput} on:input={onClientSearch} />
                {#if clientSearchInput}
                    <button on:click={() => { clientSearchInput=''; clientSearch=''; loadClients(); }}><X size={13}/></button>
                {/if}
            </label>
            <select class="select select-bordered" bind:value={clientStatusFilter} on:change={loadClients}>
                {#each CLIENT_STATUSES as s}
                    <option value={s.value}>{s.label}</option>
                {/each}
            </select>
        </div>

        {#if loadingClients}
            <div class="flex justify-center py-16"><span class="loading loading-spinner loading-lg"></span></div>
        {:else if error}
            <div class="alert alert-error">{error}</div>
        {:else if clients.length === 0}
            <div class="flex flex-col items-center justify-center py-16 gap-3 text-base-content/40">
                <Briefcase size={56} />
                <p>No clients yet</p>
                {#if canCreate(navPath, $navigationStore)}
                    <button class="btn btn-primary btn-sm" on:click={openClientCreate}>Add your first client</button>
                {/if}
            </div>
        {:else}
            <div class="space-y-2">
                {#each clients as c (c.id)}
                    {@const expanded = expandedClients.has(c.id)}
                    <div class="rounded-box border border-base-300 bg-base-100 overflow-hidden">
                        <!-- Client row -->
                        <div class="flex items-center gap-3 p-3 hover:bg-base-200/40 transition-colors">
                            <!-- Expand toggle -->
                            <button class="btn btn-ghost btn-xs btn-circle" on:click={() => toggleExpand(c)}>
                                {#if expanded}<ChevronDown size={15}/>{:else}<ChevronRight size={15}/>{/if}
                            </button>

                            <!-- Main info -->
                            <div class="flex-1 min-w-0">
                                <div class="flex flex-wrap items-center gap-2">
                                    <span class="font-semibold">{c.name}</span>
                                    <span class="badge badge-sm {getClientStatusBadge(c.status)}">{getClientStatusLabel(c.status)}</span>
                                    {#if c.industry}
                                        <span class="text-xs text-base-content/50">{c.industry}</span>
                                    {/if}
                                </div>
                                <div class="flex flex-wrap gap-3 mt-1 text-xs text-base-content/60">
                                    {#if c.contact_person}<span class="flex items-center gap-1"><Building2 size={11}/>{c.contact_person}</span>{/if}
                                    {#if c.email}<span class="flex items-center gap-1"><Mail size={11}/>{c.email}</span>{/if}
                                    {#if c.phone}<span class="flex items-center gap-1"><Phone size={11}/>{c.phone}</span>{/if}
                                    {#if c.website}<a href={c.website} target="_blank" rel="noreferrer" class="flex items-center gap-1 link link-hover"><Globe size={11}/>{c.website}</a>{/if}
                                </div>
                            </div>

                            <!-- Deliverable count -->
                            <div class="text-center hidden sm:block">
                                <div class="text-lg font-bold">{c.deliverable_count}</div>
                                <div class="text-xs text-base-content/50">deliverables</div>
                            </div>

                            <!-- Actions -->
                            <div class="flex gap-1">
                                {#if canCreate(navPath, $navigationStore)}
                                    <button class="btn btn-ghost btn-xs" title="Add deliverable"
                                        on:click={() => openDelivCreate(c.id)}>
                                        <Plus size={13}/>
                                    </button>
                                {/if}
                                {#if canUpdate(navPath, $navigationStore)}
                                    <button class="btn btn-ghost btn-xs" on:click={() => openClientEdit(c)}>
                                        <Pencil size={13}/>
                                    </button>
                                {/if}
                                {#if canDelete(navPath, $navigationStore)}
                                    <button class="btn btn-ghost btn-xs text-error" on:click={() => deleteClient(c)}>
                                        <Trash2 size={13}/>
                                    </button>
                                {/if}
                            </div>
                        </div>

                        <!-- Expanded deliverables panel -->
                        {#if expanded}
                            <div class="border-t border-base-300 bg-base-200/30 p-3">
                                {#if !clientDeliverables[c.id]}
                                    <div class="flex justify-center py-4"><span class="loading loading-spinner loading-sm"></span></div>
                                {:else if clientDeliverables[c.id].length === 0}
                                    <p class="text-sm text-base-content/50 text-center py-2">No deliverables yet.
                                        {#if canCreate(navPath, $navigationStore)}
                                            <button class="link" on:click={() => openDelivCreate(c.id)}>Add one</button>
                                        {/if}
                                    </p>
                                {:else}
                                    <div class="overflow-x-auto">
                                        <table class="table table-xs">
                                            <thead>
                                                <tr>
                                                    <th>Deliverable</th>
                                                    <th>Type</th>
                                                    <th>Status</th>
                                                    <th>Due Date</th>
                                                    <th>Assigned</th>
                                                    <th></th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {#each clientDeliverables[c.id] as d (d.id)}
                                                    {@const overdue = isOverdue(d.due_date, d.status)}
                                                    {@const soon = isDueSoon(d.due_date)}
                                                    <tr class="hover {overdue ? 'bg-error/5' : ''}">
                                                        <td>
                                                            <div class="flex items-center gap-1">
                                                                {#if overdue}<AlertCircle size={12} class="text-error flex-shrink-0"/>{/if}
                                                                {#if soon && !overdue}<Clock size={12} class="text-warning flex-shrink-0"/>{/if}
                                                                <span class="font-medium">{d.title}</span>
                                                            </div>
                                                            {#if d.campaign_name}
                                                                <div class="text-xs text-base-content/50">↳ {d.campaign_name}</div>
                                                            {/if}
                                                        </td>
                                                        <td><span class="badge badge-xs badge-outline">{getDelivTypeLabel(d.deliverable_type)}</span></td>
                                                        <td><span class="badge badge-xs {getDelivStatusBadge(d.status)}">{getDelivStatusLabel(d.status)}</span></td>
                                                        <td class="text-xs {overdue ? 'text-error font-semibold' : soon ? 'text-warning font-semibold' : ''}">
                                                            {d.due_date ?? '—'}
                                                        </td>
                                                        <td class="text-xs text-base-content/60">{d.assigned_to_name ?? '—'}</td>
                                                        <td>
                                                            <div class="flex gap-1">
                                                                {#if canUpdate(navPath, $navigationStore)}
                                                                    <button class="btn btn-ghost btn-xs" on:click={() => openDelivEdit(d)}><Pencil size={12}/></button>
                                                                {/if}
                                                                {#if canDelete(navPath, $navigationStore)}
                                                                    <button class="btn btn-ghost btn-xs text-error" on:click={() => deleteDeliv(d)}><Trash2 size={12}/></button>
                                                                {/if}
                                                            </div>
                                                        </td>
                                                    </tr>
                                                {/each}
                                            </tbody>
                                        </table>
                                    </div>
                                {/if}
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>
        {/if}

    <!-- ── DELIVERABLES TAB ─────────────────────────────────────────────── -->
    {:else}
        <!-- Filters -->
        <div class="flex flex-wrap gap-3">
            <label class="input input-bordered flex items-center gap-2 flex-1 min-w-48">
                <Search size={15} class="text-base-content/50"/>
                <input type="text" placeholder="Search deliverables…" class="grow"
                    bind:value={delivSearchInput} on:input={onDelivSearch}/>
                {#if delivSearchInput}
                    <button on:click={() => { delivSearchInput=''; delivSearch=''; loadDeliverables(); }}><X size={13}/></button>
                {/if}
            </label>
            <select class="select select-bordered" bind:value={delivClientFilter} on:change={loadDeliverables}>
                <option value="">All Clients</option>
                {#each clients as c}
                    <option value={c.id}>{c.name}</option>
                {/each}
            </select>
            <select class="select select-bordered" bind:value={delivStatusFilter} on:change={loadDeliverables}>
                {#each DELIVERABLE_STATUSES as s}
                    <option value={s.value}>{s.label}</option>
                {/each}
            </select>
        </div>

        {#if loadingDelivs}
            <div class="flex justify-center py-16"><span class="loading loading-spinner loading-lg"></span></div>
        {:else if deliverables.length === 0}
            <div class="flex flex-col items-center justify-center py-16 gap-3 text-base-content/40">
                <PackageCheck size={56}/>
                <p>No deliverables found</p>
            </div>
        {:else}
            <div class="overflow-x-auto rounded-box border border-base-300">
                <table class="table table-sm">
                    <thead>
                        <tr>
                            <th>Deliverable</th>
                            <th>Client</th>
                            <th>Campaign</th>
                            <th>Type</th>
                            <th>Status</th>
                            <th>Due Date</th>
                            <th>Assigned To</th>
                            <th></th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each deliverables as d (d.id)}
                            {@const overdue = isOverdue(d.due_date, d.status)}
                            {@const soon = isDueSoon(d.due_date)}
                            <tr class="hover {overdue ? 'bg-error/5' : ''}">
                                <td>
                                    <div class="flex items-center gap-1 font-medium">
                                        {#if overdue}<AlertCircle size={13} class="text-error flex-shrink-0"/>{/if}
                                        {#if soon && !overdue}<Clock size={13} class="text-warning flex-shrink-0"/>{/if}
                                        {d.title}
                                    </div>
                                    {#if d.description}
                                        <div class="text-xs text-base-content/50 truncate max-w-xs">{d.description}</div>
                                    {/if}
                                </td>
                                <td class="font-medium text-sm">{d.client_name}</td>
                                <td class="text-xs text-base-content/60">{d.campaign_name ?? '—'}</td>
                                <td><span class="badge badge-xs badge-outline">{getDelivTypeLabel(d.deliverable_type)}</span></td>
                                <td><span class="badge badge-sm {getDelivStatusBadge(d.status)}">{getDelivStatusLabel(d.status)}</span></td>
                                <td class="text-xs {overdue ? 'text-error font-semibold' : soon ? 'text-warning font-semibold' : ''}">
                                    {d.due_date ?? '—'}
                                </td>
                                <td class="text-xs text-base-content/60">{d.assigned_to_name ?? '—'}</td>
                                <td>
                                    <div class="flex gap-1">
                                        {#if canUpdate(navPath, $navigationStore)}
                                            <button class="btn btn-ghost btn-xs" on:click={() => openDelivEdit(d)}><Pencil size={13}/></button>
                                        {/if}
                                        {#if canDelete(navPath, $navigationStore)}
                                            <button class="btn btn-ghost btn-xs text-error" on:click={() => deleteDeliv(d)}><Trash2 size={13}/></button>
                                        {/if}
                                    </div>
                                </td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        {/if}
    {/if}
</div>
