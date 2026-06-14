<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, Megaphone, CalendarDays, Plus, ChevronLeft, ChevronRight, X, Pencil, Trash2 } from 'lucide-svelte';
    import { onMount } from 'svelte';
    import { navigationStore, canCreate, canUpdate, canDelete } from '$lib/stores/navigation';
    import {
        contentCalendarService,
        CHANNELS, CONTENT_TYPES, STATUSES,
        getChannelColor, getChannelLabel, getStatusBadge, getStatusLabel,
    } from '$lib/services/content-calendar';
    import type { ContentCalendarItem, CreateContentItemDto } from '$lib/services/content-calendar';

    pageTitle.set({ title: 'Content Calendar', desc: 'Plan and schedule digital marketing content' });
    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Digital Marketing', icon: Megaphone },
        { label: 'Content Calendar', icon: CalendarDays },
    ]);

    const navPath = '/admin/digital-marketing/content-calendar';

    // ── State ────────────────────────────────────────────────────────────
    let items: ContentCalendarItem[] = [];
    let loading = true;
    let error = '';

    let today = new Date();
    let viewYear  = today.getFullYear();
    let viewMonth = today.getMonth(); // 0-based

    let filterStatus  = '';
    let filterChannel = '';

    let showModal = false;
    let editingItem: ContentCalendarItem | null = null;
    let saving = false;
    let modalError = '';

    let form: CreateContentItemDto = emptyForm();

    function emptyForm(): CreateContentItemDto {
        const d = formatDate(today);
        return {
            title: '',
            description: '',
            content_type: 'post',
            channel: 'instagram',
            scheduled_date: d,
            scheduled_time: '',
            status: 'draft',
            campaign_name: '',
            assigned_to: undefined,
            notes: '',
        };
    }

    // ── Date helpers ─────────────────────────────────────────────────────
    function formatDate(d: Date): string {
        return `${d.getFullYear()}-${String(d.getMonth()+1).padStart(2,'0')}-${String(d.getDate()).padStart(2,'0')}`;
    }

    function monthStart(y: number, m: number) { return new Date(y, m, 1); }
    function monthEnd(y: number, m: number)   { return new Date(y, m+1, 0); }

    function monthLabel(y: number, m: number) {
        return new Date(y, m, 1).toLocaleString('default', { month: 'long', year: 'numeric' });
    }

    function prevMonth() {
        if (viewMonth === 0) { viewMonth = 11; viewYear--; } else viewMonth--;
        loadItems();
    }
    function nextMonth() {
        if (viewMonth === 11) { viewMonth = 0; viewYear++; } else viewMonth++;
        loadItems();
    }

    // Build the 6-week grid for the calendar
    $: calendarDays = (() => {
        const start = monthStart(viewYear, viewMonth);
        const end   = monthEnd(viewYear, viewMonth);
        const days: Date[] = [];
        // pad from Monday
        const startDow = (start.getDay() + 6) % 7; // Mon=0
        for (let i = startDow; i > 0; i--) {
            const d = new Date(start); d.setDate(d.getDate() - i); days.push(d);
        }
        for (let d = new Date(start); d <= end; d.setDate(d.getDate() + 1)) days.push(new Date(d));
        // pad to complete last row
        while (days.length % 7 !== 0) {
            const d = new Date(days[days.length-1]); d.setDate(d.getDate()+1); days.push(d);
        }
        return days;
    })();

    function isCurrentMonth(d: Date) { return d.getMonth() === viewMonth && d.getFullYear() === viewYear; }
    function isToday(d: Date) { return formatDate(d) === formatDate(today); }

    // ── Load ─────────────────────────────────────────────────────────────
    async function loadItems() {
        loading = true; error = '';
        try {
            const start = formatDate(monthStart(viewYear, viewMonth));
            const end   = formatDate(monthEnd(viewYear, viewMonth));
            items = await contentCalendarService.list({
                start_date: start, end_date: end,
                status:  filterStatus  || undefined,
                channel: filterChannel || undefined,
            });
        } catch (e: any) {
            error = e.message ?? 'Failed to load';
        } finally {
            loading = false;
        }
    }

    function itemsForDate(d: Date): ContentCalendarItem[] {
        const key = formatDate(d);
        return items.filter(i => i.scheduled_date === key);
    }

    // ── Modal ─────────────────────────────────────────────────────────────
    function openCreate(date?: string) {
        editingItem = null;
        form = emptyForm();
        if (date) form.scheduled_date = date;
        showModal = true; modalError = '';
    }

    function openEdit(item: ContentCalendarItem) {
        editingItem = item;
        form = {
            title:          item.title,
            description:    item.description ?? '',
            content_type:   item.content_type,
            channel:        item.channel,
            scheduled_date: item.scheduled_date,
            scheduled_time: item.scheduled_time ?? '',
            status:         item.status,
            campaign_name:  item.campaign_name ?? '',
            notes:          item.notes ?? '',
        };
        showModal = true; modalError = '';
    }

    function closeModal() { showModal = false; editingItem = null; }

    async function saveItem() {
        if (!form.title.trim()) { modalError = 'Title is required'; return; }
        saving = true; modalError = '';
        try {
            const payload = {
                ...form,
                scheduled_time: form.scheduled_time || undefined,
                description:    form.description    || undefined,
                campaign_name:  form.campaign_name  || undefined,
                notes:          form.notes          || undefined,
            };
            if (editingItem) {
                await contentCalendarService.update(editingItem.id, payload);
            } else {
                await contentCalendarService.create(payload);
            }
            closeModal();
            await loadItems();
        } catch (e: any) {
            modalError = e.message ?? 'Failed to save';
        } finally {
            saving = false;
        }
    }

    async function deleteItem(item: ContentCalendarItem) {
        if (!confirm(`Delete "${item.title}"?`)) return;
        try {
            await contentCalendarService.delete(item.id);
            await loadItems();
        } catch (e: any) {
            error = e.message ?? 'Failed to delete';
        }
    }

    async function quickStatus(item: ContentCalendarItem, status: string) {
        try {
            await contentCalendarService.update(item.id, { status });
            await loadItems();
        } catch {}
    }

    onMount(loadItems);
</script>

<!-- ── Header ──────────────────────────────────────────────────────────── -->
<div class="flex flex-wrap items-center justify-between gap-3 mb-4">
    <div class="flex items-center gap-2">
        <button class="btn btn-ghost btn-sm" on:click={prevMonth}><ChevronLeft size={16}/></button>
        <h2 class="text-lg font-semibold min-w-40 text-center">{monthLabel(viewYear, viewMonth)}</h2>
        <button class="btn btn-ghost btn-sm" on:click={nextMonth}><ChevronRight size={16}/></button>
    </div>

    <div class="flex flex-wrap gap-2 items-center">
        <!-- Channel filter -->
        <select class="select select-bordered select-sm" bind:value={filterChannel} on:change={loadItems}>
            <option value="">All Channels</option>
            {#each CHANNELS as ch}
                <option value={ch.value}>{ch.label}</option>
            {/each}
        </select>

        <!-- Status filter -->
        <select class="select select-bordered select-sm" bind:value={filterStatus} on:change={loadItems}>
            <option value="">All Statuses</option>
            {#each STATUSES as s}
                <option value={s.value}>{s.label}</option>
            {/each}
        </select>

        {#if canCreate(navPath, $navigationStore)}
            <button class="btn btn-primary btn-sm gap-1" on:click={() => openCreate()}>
                <Plus size={15}/> New Content
            </button>
        {/if}
    </div>
</div>

{#if error}
    <div class="alert alert-error mb-3 text-sm">{error}</div>
{/if}

<!-- ── Calendar Grid ───────────────────────────────────────────────────── -->
{#if loading}
    <div class="flex justify-center py-16"><span class="loading loading-spinner loading-lg"></span></div>
{:else}
    <div class="rounded-xl border border-base-300 overflow-hidden bg-base-100">
        <!-- Day headers -->
        <div class="grid grid-cols-7 border-b border-base-300">
            {#each ['Mon','Tue','Wed','Thu','Fri','Sat','Sun'] as day}
                <div class="text-center text-xs font-semibold py-2 text-base-content/60">{day}</div>
            {/each}
        </div>

        <!-- Weeks -->
        <div class="grid grid-cols-7">
            {#each calendarDays as day}
                {@const dayItems = itemsForDate(day)}
                {@const inMonth = isCurrentMonth(day)}
                <div
                    class="min-h-28 p-1 border-b border-r border-base-300 last:border-r-0 {inMonth ? '' : 'bg-base-200/40'}"
                    class:ring-2={isToday(day)}
                    class:ring-primary={isToday(day)}
                >
                    <!-- Date number -->
                    <div class="flex justify-between items-start mb-1">
                        <span class="text-xs font-medium {inMonth ? 'text-base-content' : 'text-base-content/30'}
                            {isToday(day) ? 'bg-primary text-primary-content rounded-full w-5 h-5 flex items-center justify-center' : ''}">
                            {day.getDate()}
                        </span>
                        {#if inMonth && canCreate(navPath, $navigationStore)}
                            <button
                                class="opacity-0 hover:opacity-100 group-hover:opacity-100 btn btn-ghost btn-xs p-0 h-5 w-5"
                                on:click={() => openCreate(formatDate(day))}
                                title="Add content">
                                <Plus size={11}/>
                            </button>
                        {/if}
                    </div>

                    <!-- Items for this day -->
                    <div class="flex flex-col gap-0.5">
                        {#each dayItems as item}
                            <button
                                class="text-left w-full rounded px-1.5 py-0.5 text-xs truncate border-l-2 bg-base-200 hover:bg-base-300 transition-colors"
                                style="border-color: {getChannelColor(item.channel)}"
                                on:click={() => openEdit(item)}
                                title="{item.title} — {getChannelLabel(item.channel)}"
                            >
                                <span class="font-medium truncate block">{item.title}</span>
                                <span class="opacity-60">{getChannelLabel(item.channel)}</span>
                            </button>
                        {/each}
                    </div>
                </div>
            {/each}
        </div>
    </div>

    <!-- ── Upcoming List ─────────────────────────────────────────────── -->
    {#if items.length > 0}
        <div class="mt-6">
            <h3 class="font-semibold mb-3">This Month — All Items</h3>
            <div class="overflow-x-auto rounded-xl border border-base-300">
                <table class="table table-sm">
                    <thead>
                        <tr>
                            <th>Date</th>
                            <th>Title</th>
                            <th>Channel</th>
                            <th>Type</th>
                            <th>Status</th>
                            <th>Assigned To</th>
                            <th>Campaign</th>
                            {#if canUpdate(navPath, $navigationStore) || canDelete(navPath, $navigationStore)}
                                <th></th>
                            {/if}
                        </tr>
                    </thead>
                    <tbody>
                        {#each items as item}
                            <tr class="hover">
                                <td class="whitespace-nowrap text-xs">
                                    {item.scheduled_date}
                                    {#if item.scheduled_time}
                                        <span class="opacity-50 ml-1">{item.scheduled_time.slice(0,5)}</span>
                                    {/if}
                                </td>
                                <td class="font-medium max-w-xs truncate">{item.title}</td>
                                <td>
                                    <span class="flex items-center gap-1.5">
                                        <span class="w-2 h-2 rounded-full flex-shrink-0"
                                            style="background:{getChannelColor(item.channel)}"></span>
                                        {getChannelLabel(item.channel)}
                                    </span>
                                </td>
                                <td class="capitalize">{item.content_type}</td>
                                <td>
                                    <span class="badge badge-sm {getStatusBadge(item.status)}">
                                        {getStatusLabel(item.status)}
                                    </span>
                                </td>
                                <td class="text-xs">{item.assigned_to_name ?? '—'}</td>
                                <td class="text-xs opacity-70">{item.campaign_name ?? '—'}</td>
                                {#if canUpdate(navPath, $navigationStore) || canDelete(navPath, $navigationStore)}
                                    <td>
                                        <div class="flex gap-1">
                                            {#if canUpdate(navPath, $navigationStore)}
                                                <button class="btn btn-ghost btn-xs" on:click={() => openEdit(item)} title="Edit">
                                                    <Pencil size={13}/>
                                                </button>
                                            {/if}
                                            {#if canDelete(navPath, $navigationStore)}
                                                <button class="btn btn-ghost btn-xs text-error" on:click={() => deleteItem(item)} title="Delete">
                                                    <Trash2 size={13}/>
                                                </button>
                                            {/if}
                                        </div>
                                    </td>
                                {/if}
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        </div>
    {/if}
{/if}

<!-- ── Modal ──────────────────────────────────────────────────────────── -->
{#if showModal}
    <div class="modal modal-open">
        <div class="modal-box max-w-lg">
            <div class="flex justify-between items-center mb-4">
                <h3 class="font-bold text-lg">{editingItem ? 'Edit Content' : 'New Content'}</h3>
                <button class="btn btn-ghost btn-sm btn-circle" on:click={closeModal}><X size={16}/></button>
            </div>

            {#if modalError}
                <div class="alert alert-error text-sm mb-3">{modalError}</div>
            {/if}

            <div class="flex flex-col gap-3">
                <!-- Title -->
                <div class="form-control">
                    <label class="label py-0" for="cc-title"><span class="label-text font-medium">Title *</span></label>
                    <input id="cc-title" type="text" class="input input-bordered input-sm" bind:value={form.title} placeholder="e.g. Summer Sale Post"/>
                </div>

                <!-- Channel + Content Type -->
                <div class="grid grid-cols-2 gap-3">
                    <div class="form-control">
                        <label class="label py-0" for="cc-channel"><span class="label-text font-medium">Channel *</span></label>
                        <select id="cc-channel" class="select select-bordered select-sm" bind:value={form.channel}>
                            {#each CHANNELS as ch}
                                <option value={ch.value}>{ch.label}</option>
                            {/each}
                        </select>
                    </div>
                    <div class="form-control">
                        <label class="label py-0" for="cc-type"><span class="label-text font-medium">Content Type *</span></label>
                        <select id="cc-type" class="select select-bordered select-sm" bind:value={form.content_type}>
                            {#each CONTENT_TYPES as ct}
                                <option value={ct.value}>{ct.label}</option>
                            {/each}
                        </select>
                    </div>
                </div>

                <!-- Date + Time -->
                <div class="grid grid-cols-2 gap-3">
                    <div class="form-control">
                        <label class="label py-0" for="cc-date"><span class="label-text font-medium">Date *</span></label>
                        <input id="cc-date" type="date" class="input input-bordered input-sm" bind:value={form.scheduled_date}/>
                    </div>
                    <div class="form-control">
                        <label class="label py-0" for="cc-time"><span class="label-text font-medium">Time (optional)</span></label>
                        <input id="cc-time" type="time" class="input input-bordered input-sm" bind:value={form.scheduled_time}/>
                    </div>
                </div>

                <!-- Status -->
                <div class="form-control">
                    <label class="label py-0" for="cc-status"><span class="label-text font-medium">Status</span></label>
                    <select id="cc-status" class="select select-bordered select-sm" bind:value={form.status}>
                        {#each STATUSES as s}
                            <option value={s.value}>{s.label}</option>
                        {/each}
                    </select>
                </div>

                <!-- Campaign Name -->
                <div class="form-control">
                    <label class="label py-0" for="cc-campaign"><span class="label-text font-medium">Campaign Name (optional)</span></label>
                    <input id="cc-campaign" type="text" class="input input-bordered input-sm" bind:value={form.campaign_name} placeholder="e.g. Q3 Product Launch"/>
                </div>

                <!-- Description -->
                <div class="form-control">
                    <label class="label py-0" for="cc-desc"><span class="label-text font-medium">Description / Caption</span></label>
                    <textarea id="cc-desc" class="textarea textarea-bordered textarea-sm" rows="3" bind:value={form.description} placeholder="Content brief or caption..."></textarea>
                </div>

                <!-- Notes -->
                <div class="form-control">
                    <label class="label py-0" for="cc-notes"><span class="label-text font-medium">Notes</span></label>
                    <textarea id="cc-notes" class="textarea textarea-bordered textarea-sm" rows="2" bind:value={form.notes} placeholder="Internal notes..."></textarea>
                </div>
            </div>

            <div class="modal-action">
                <button class="btn btn-ghost btn-sm" on:click={closeModal} disabled={saving}>Cancel</button>
                <button class="btn btn-primary btn-sm" on:click={saveItem} disabled={saving}>
                    {#if saving}<span class="loading loading-spinner loading-xs"></span>{/if}
                    {editingItem ? 'Update' : 'Create'}
                </button>
            </div>
        </div>
        <button class="modal-backdrop" on:click={closeModal} aria-label="Close modal"></button>
    </div>
{/if}
