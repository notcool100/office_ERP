<script lang="ts">
    import { onMount } from 'svelte';
    import { dailyLogService } from '$lib/services/daily-log';
    import { projectService } from '$lib/services/project';
    import { userStore } from '$lib/stores/user';
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import type {
        DailyLog,
        DailyLogLink,
        ListDailyLogQuery,
    } from '$lib/types/daily-log';
    import type { Project, Card } from '$lib/types/project';
    import PageSection from '../../../components/PageSection.svelte';
    import RichTextEditor from '$lib/components/RichTextEditor.svelte';
    import {
        ChevronDown,
        ChevronRight,
        Plus,
        Calendar,
        User as UserIcon,
        ClipboardList,
        Trash2,
        Edit3,
        X,
        Save,
        Search,
        Link as LinkIcon,
        Home,
        LayoutDashboard,
    } from 'lucide-svelte';
    import { fade, slide } from 'svelte/transition';

    pageTitle.set({
        title: 'Daily Log',
        desc: 'Track and manage your daily progress',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Admin', icon: LayoutDashboard },
        { label: 'Daily Log', icon: ClipboardList },
    ]);

    let logs: DailyLog[] = [];
    let projects: Project[] = [];
    let availableCards: Card[] = [];
    let loading = true;
    let saving = false;
    let error = '';

    // Form state
    let showForm = false;
    let editingLogId: string | null = null;
    let formDate = new Date().toISOString().split('T')[0];
    let formContent = '';
    let selectedCards: Card[] = [];
    let cardSearchTerm = '';
    let filteredCards: Card[] = [];

    // Filter state
    let filterUser = '';
    let filterStartDate = '';
    let filterEndDate = '';
    let showAllUsers = false;

    $: canSeeAll = $userStore.user?.isAdmin || false; // Simple check for demo, would check dept in real app

    onMount(async () => {
        await loadData();
    });

    async function loadData() {
        loading = true;
        error = '';
        try {
            const query: ListDailyLogQuery = {};
            if (!showAllUsers && $userStore.user) {
                query.user_id = $userStore.user.id;
            }
            if (filterStartDate) query.start_date = filterStartDate;
            if (filterEndDate) query.end_date = filterEndDate;

            const [logsData, projectsData] = await Promise.all([
                dailyLogService.list(query),
                projectService.list(),
            ]);

            logs = logsData;
            projects = projectsData;

            // Load all cards from all projects for the selector
            // In a large app, we'd fetch these on demand/search
            const cardsPromises = projects.map((p) =>
                projectService.listCards(p.id),
            );
            const allCardsResult = await Promise.all(cardsPromises);
            availableCards = allCardsResult.flat();
            filteredCards = availableCards;
        } catch (e: any) {
            console.error('Failed to load daily logs:', e);
            error = e.message || 'Failed to load logs';
        } finally {
            loading = false;
        }
    }

    function openCreateForm() {
        editingLogId = null;
        formDate = new Date().toISOString().split('T')[0];
        formContent = '';
        selectedCards = [];
        showForm = true;
    }

    function openEditForm(log: DailyLog) {
        editingLogId = log.id;
        formDate = log.log_date;
        formContent = log.content;
        selectedCards = availableCards.filter((c) =>
            log.links.some((l) => l.card_id === c.id),
        );
        showForm = true;
    }

    async function handleSave() {
        if (!formContent.trim()) {
            error = 'Log content cannot be empty';
            return;
        }

        saving = true;
        error = '';
        try {
            const dto = {
                log_date: formDate,
                content: formContent,
                card_ids: selectedCards.map((c) => c.id),
            };

            if (editingLogId) {
                await dailyLogService.update(editingLogId, dto);
            } else {
                await dailyLogService.create(dto);
            }

            showForm = false;
            await loadData();
        } catch (e: any) {
            error = e.message || 'Failed to save log';
        } finally {
            saving = false;
        }
    }

    async function handleDelete(id: string) {
        if (!confirm('Are you sure you want to delete this log?')) return;

        try {
            await dailyLogService.delete(id);
            await loadData();
        } catch (e: any) {
            error = e.message || 'Failed to delete log';
        }
    }

    function toggleCard(card: Card) {
        if (selectedCards.find((c) => c.id === card.id)) {
            selectedCards = selectedCards.filter((c) => c.id !== card.id);
        } else {
            selectedCards = [...selectedCards, card];
        }
    }

    $: {
        if (cardSearchTerm) {
            const term = cardSearchTerm.toLowerCase();
            filteredCards = availableCards.filter(
                (c) =>
                    c.card_key.toLowerCase().includes(term) ||
                    c.title.toLowerCase().includes(term),
            );
        } else {
            filteredCards = availableCards;
        }
    }

    function formatDate(dateStr: string) {
        return new Date(dateStr).toLocaleDateString('en-US', {
            weekday: 'short',
            year: 'numeric',
            month: 'short',
            day: 'numeric',
        });
    }

    function handleContentChange(event: any) {
        formContent = event.detail;
    }
</script>

<PageSection title="Daily Log">
    <div class="flex justify-end mb-4">
        <button class="btn btn-primary btn-sm" on:click={openCreateForm}>
            <Plus class="w-4 h-4 mr-1" /> New Entry
        </button>
    </div>

    <div class="space-y-6">
        <!-- Filters Area -->
        <div class="bg-base-200 p-4 rounded-lg flex flex-wrap gap-4 items-end">
            <div class="form-control">
                <label class="label py-1" for="filter-start"
                    ><span class="label-text text-xs">Start Date</span></label>
                <input
                    id="filter-start"
                    type="date"
                    class="input input-sm input-bordered"
                    bind:value={filterStartDate} />
            </div>
            <div class="form-control">
                <label class="label py-1" for="filter-end"
                    ><span class="label-text text-xs">End Date</span></label>
                <input
                    id="filter-end"
                    type="date"
                    class="input input-sm input-bordered"
                    bind:value={filterEndDate} />
            </div>

            {#if canSeeAll}
                <div class="form-control">
                    <label class="label cursor-pointer gap-2 py-1">
                        <span class="label-text text-xs">Show all users</span>
                        <input
                            type="checkbox"
                            class="checkbox checkbox-sm"
                            bind:checked={showAllUsers} />
                    </label>
                </div>
            {/if}

            <button class="btn btn-sm btn-outline" on:click={loadData}>
                Apply Filters
            </button>
        </div>

        {#if error}
            <div class="alert alert-error shadow-sm py-2">
                <span>{error}</span>
            </div>
        {/if}

        {#if loading}
            <div class="flex justify-center p-12">
                <span class="loading loading-spinner loading-lg"></span>
            </div>
        {:else if logs.length === 0}
            <div
                class="text-center p-12 bg-base-200/50 rounded-xl border-2 border-dashed border-base-300">
                <ClipboardList class="w-12 h-12 mx-auto opacity-20 mb-4" />
                <h3 class="font-bold text-lg">No logs found</h3>
                <p class="opacity-60">
                    Start tracking your daily progress by creating your first
                    entry.
                </p>
                <button class="btn btn-primary mt-4" on:click={openCreateForm}
                    >Create First Log</button>
            </div>
        {:else}
            <div
                class="grid grid-cols-1 md:grid-cols-[1fr_300px] gap-6 items-start">
                <!-- Main Timeline -->
                <div class="space-y-4">
                    {#each logs as log (log.id)}
                        <div
                            class="card bg-base-100 border border-base-300 shadow-sm hover:shadow-md transition-shadow transition-all overflow-hidden">
                            <div class="card-body p-5">
                                <div
                                    class="flex justify-between items-start mb-4">
                                    <div class="flex items-center gap-3">
                                        <div
                                            class="bg-primary/10 text-primary p-2 rounded-lg">
                                            <Calendar class="w-5 h-5" />
                                        </div>
                                        <div>
                                            <div class="font-bold text-lg">
                                                {formatDate(log.log_date)}
                                            </div>
                                            <div
                                                class="text-xs opacity-60 flex items-center gap-1">
                                                <UserIcon class="w-3 h-3" />
                                                {log.user_name}
                                                {#if log.updated_at !== log.created_at}
                                                    • Updated {new Date(
                                                        log.updated_at,
                                                    ).toLocaleTimeString()}
                                                {/if}
                                            </div>
                                        </div>
                                    </div>

                                    {#if log.user_id === $userStore.user?.id}
                                        <div class="join">
                                            <button
                                                class="btn btn-ghost btn-xs join-item"
                                                on:click={() =>
                                                    openEditForm(log)}>
                                                <Edit3 class="w-4 h-4" />
                                            </button>
                                            <button
                                                class="btn btn-ghost btn-xs join-item text-error"
                                                on:click={() =>
                                                    handleDelete(log.id)}>
                                                <Trash2 class="w-4 h-4" />
                                            </button>
                                        </div>
                                    {/if}
                                </div>

                                <div
                                    class="prose prose-sm max-w-none text-base-content/90 border-l-4 border-primary/20 pl-4 py-1">
                                    {@html log.content}
                                </div>

                                {#if log.links.length > 0}
                                    <div class="mt-4 flex flex-wrap gap-2">
                                        {#each log.links as link}
                                            <div
                                                class="badge badge-outline badge-sm gap-1 hover:badge-primary cursor-default py-3">
                                                <LinkIcon class="w-3 h-3" />
                                                <span
                                                    class="font-mono text-[10px] opacity-70"
                                                    >{link.card_key}</span>
                                                <span
                                                    class="max-w-[150px] truncate ml-1"
                                                    >{link.card_title}</span>
                                            </div>
                                        {/each}
                                    </div>
                                {/if}
                            </div>
                        </div>
                    {/each}
                </div>

                <!-- Right sidebar: stats or summary -->
                <div class="bg-base-200 rounded-xl p-4 sticky top-4 space-y-6">
                    <div>
                        <h4 class="font-bold text-sm uppercase opacity-50 mb-3">
                            Your Progress
                        </h4>
                        <div class="grid grid-cols-2 gap-2">
                            <div
                                class="bg-base-100 p-3 rounded-lg border border-base-300">
                                <div class="text-2xl font-bold">
                                    {logs.filter(
                                        (l) =>
                                            l.user_id === $userStore.user?.id,
                                    ).length}
                                </div>
                                <div class="text-[10px] uppercase opacity-60">
                                    Total Logs
                                </div>
                            </div>
                            <div
                                class="bg-base-100 p-3 rounded-lg border border-base-300">
                                <div class="text-2xl font-bold">
                                    {logs.filter((l) => l.links.length > 0)
                                        .length}
                                </div>
                                <div class="text-[10px] uppercase opacity-60">
                                    Linked to Tasks
                                </div>
                            </div>
                        </div>
                    </div>

                    <div>
                        <h4 class="font-bold text-sm uppercase opacity-50 mb-3">
                            Active Projects
                        </h4>
                        <div class="space-y-2">
                            {#each projects as project}
                                <div
                                    class="flex items-center justify-between text-xs p-2 bg-base-100 rounded border border-base-300">
                                    <span
                                        class="font-medium truncate max-w-[150px]"
                                        >{project.name}</span>
                                    <span class="badge badge-ghost badge-xs"
                                        >{project.project_key}</span>
                                </div>
                            {/each}
                        </div>
                    </div>
                </div>
            </div>
        {/if}
    </div>

    <!-- Create/Edit Modal -->
    {#if showForm}
        <div class="modal modal-open bg-black/50 backdrop-blur-sm z-50">
            <div
                class="modal-box max-w-4xl p-0 overflow-hidden bg-base-100 rounded-2xl shadow-2xl">
                <div
                    class="p-4 bg-primary text-primary-content flex justify-between items-center">
                    <h3 class="font-bold text-lg flex items-center gap-2">
                        <ClipboardList class="w-5 h-5" />
                        {editingLogId ? 'Update' : 'New'} Daily Log
                    </h3>
                    <button
                        class="btn btn-circle btn-ghost btn-sm"
                        on:click={() => (showForm = false)}>
                        <X class="w-5 h-5" />
                    </button>
                </div>

                <div class="p-6 space-y-6 max-h-[80vh] overflow-y-auto">
                    <div
                        class="grid grid-cols-1 md:grid-cols-[1fr_250px] gap-8">
                        <div class="space-y-6">
                            <div class="form-control">
                                <label class="label" for="log-date"
                                    ><span class="label-text font-semibold"
                                        >Date</span
                                    ></label>
                                <div class="relative">
                                    <Calendar
                                        class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 opacity-40" />
                                    <input
                                        id="log-date"
                                        type="date"
                                        class="input input-bordered w-full pl-10"
                                        bind:value={formDate} />
                                </div>
                            </div>

                            <div class="form-control">
                                <label class="label"
                                    ><span class="label-text font-semibold"
                                        >What did you do today?</span
                                    ></label>
                                <RichTextEditor
                                    content={formContent}
                                    placeholder="Describe your work, achievements, and blockers..."
                                    on:change={handleContentChange} />
                            </div>
                        </div>

                        <div class="space-y-4">
                            <div class="form-control">
                                <label class="label"
                                    ><span class="label-text font-semibold"
                                        >Link Project Cards</span
                                    ></label>
                                <div class="relative mb-2">
                                    <Search
                                        class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 opacity-40" />
                                    <input
                                        type="text"
                                        placeholder="Search tasks..."
                                        class="input input-bordered input-sm w-full pl-9"
                                        bind:value={cardSearchTerm} />
                                </div>
                                <div
                                    class="bg-base-200 rounded-lg p-2 max-h-[400px] overflow-y-auto space-y-1 border border-base-300">
                                    {#if filteredCards.length === 0}
                                        <div
                                            class="text-[10px] opacity-50 text-center py-4">
                                            No cards found
                                        </div>
                                    {:else}
                                        {#each filteredCards as card}
                                            <button
                                                class="w-full text-left p-2 rounded text-[11px] transition-colors flex items-start gap-2 {selectedCards.some(
                                                    (c) => c.id === card.id,
                                                )
                                                    ? 'bg-primary/20 border-primary border'
                                                    : 'hover:bg-base-300 border border-transparent'}"
                                                on:click={() =>
                                                    toggleCard(card)}>
                                                <div class="mt-0.5">
                                                    {#if selectedCards.some((c) => c.id === card.id)}
                                                        <div
                                                            class="w-3 h-3 bg-primary rounded-full">
                                                        </div>
                                                    {:else}
                                                        <div
                                                            class="w-3 h-3 bg-base-300 rounded-full">
                                                        </div>
                                                    {/if}
                                                </div>
                                                <div class="flex-1 min-w-0">
                                                    <div
                                                        class="font-mono opacity-60">
                                                        {card.card_key}
                                                    </div>
                                                    <div
                                                        class="font-medium truncate">
                                                        {card.title}
                                                    </div>
                                                </div>
                                            </button>
                                        {/each}
                                    {/if}
                                </div>
                            </div>

                            {#if selectedCards.length > 0}
                                <div class="pt-2">
                                    <div
                                        class="text-[10px] uppercase opacity-50 font-bold mb-2">
                                        Selected ({selectedCards.length})
                                    </div>
                                    <div class="flex flex-wrap gap-1">
                                        {#each selectedCards as card}
                                            <div
                                                class="badge badge-primary badge-sm gap-1 pr-1">
                                                {card.card_key}
                                                <button
                                                    on:click|stopPropagation={() =>
                                                        toggleCard(card)}
                                                    ><X
                                                        class="w-3 h-3" /></button>
                                            </div>
                                        {/each}
                                    </div>
                                </div>
                            {/if}
                        </div>
                    </div>
                </div>

                <div
                    class="p-4 bg-base-200 flex justify-end gap-3 border-t border-base-300">
                    <button
                        class="btn"
                        on:click={() => (showForm = false)}
                        disabled={saving}>Cancel</button>
                    <button
                        class="btn btn-primary"
                        on:click={handleSave}
                        disabled={saving}>
                        {#if saving}
                            <span class="loading loading-spinner loading-sm"
                            ></span>
                        {:else}
                            <Save class="w-4 h-4 mr-1" />
                        {/if}
                        {editingLogId ? 'Update Log' : 'Save Entry'}
                    </button>
                </div>
            </div>
        </div>
    {/if}
</PageSection>

<style>
    /* Styling for the proactive L-shaped lines already in page.svelte could be here, but we'll use base utilities */
</style>
