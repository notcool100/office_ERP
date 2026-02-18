<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import {
        Home,
        Settings as SettingsIcon,
        Calendar as CalendarIcon,
        Plus,
        Filter,
    } from 'lucide-svelte';
    import { onMount, onDestroy } from 'svelte';
    import { registerShortcut, unregisterShortcut } from '$lib/stores/shortcutManager';
    import Calendar from '../../../../components/schedule/Calendar.svelte';
    import Events from '../../../../components/schedule/Events.svelte';
    import { calendarService } from '$lib/services/calendar';
    import type {
        CalendarEvent,
        CalendarScope,
    } from '$lib/types/calendar';
    import { userStore } from '$lib/stores/user';
    import {
        navigationStore,
        canRead,
        canCreate,
        canUpdate,
        canDelete,
    } from '$lib/stores/navigation';

    pageTitle.set({
        title: 'Calendar',
        desc: 'Manage events and notes',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Settings', icon: SettingsIcon },
        { label: 'Calendar', icon: CalendarIcon },
    ]);

    const navPath = '/admin/settings/calendar';
    let events: CalendarEvent[] = [];
    let selectedDate = new Date().toISOString().slice(0, 10);
    let loading = true;
    let errorMessage = '';
    let showModal = false;
    let editingEvent: CalendarEvent | null = null;

    let formData = {
        title: '',
        description: '',
        location: '',
        date: selectedDate,
        startTime: '09:00',
        endTime: '10:00',
        all_day: false,
        scope: 'personal' as CalendarScope,
    };

    let scopeFilter = {
        personal: true,
        department: true,
        company: true,
    };

    const formatDateLocal = (date: Date) => {
        const year = date.getFullYear();
        const month = String(date.getMonth() + 1).padStart(2, '0');
        const day = String(date.getDate()).padStart(2, '0');
        return `${year}-${month}-${day}`;
    };

    const formatTimeLocal = (date: Date) => {
        const hours = String(date.getHours()).padStart(2, '0');
        const minutes = String(date.getMinutes()).padStart(2, '0');
        return `${hours}:${minutes}`;
    };

    function buildDateTime(date: string, time: string, fallback: string) {
        if (!time) return `${date}T${fallback}`;
        return `${date}T${time}:00`;
    }

    function getMonthRange(dateStr: string) {
        const date = new Date(dateStr);
        const start = new Date(date.getFullYear(), date.getMonth(), 1);
        const end = new Date(date.getFullYear(), date.getMonth() + 1, 0);
        return {
            start: formatDateLocal(start),
            end: formatDateLocal(end),
        };
    }

    async function loadEvents(dateStr: string) {
        loading = true;
        errorMessage = '';
        try {
            const range = getMonthRange(dateStr);
            events = await calendarService.list(range.start, range.end);
        } catch (error) {
            console.error('Failed to load events:', error);
            errorMessage = 'Failed to load events';
        } finally {
            loading = false;
        }
    }

    function openCreateModal() {
        editingEvent = null;
        formData = {
            title: '',
            description: '',
            location: '',
            date: selectedDate,
            startTime: '09:00',
            endTime: '10:00',
            all_day: false,
            scope: 'personal',
        };
        showModal = true;
    }

    function openEditModal(event: CalendarEvent) {
        const startDate = new Date(event.start_at);
        const endDate = new Date(event.end_at);

        editingEvent = event;
        formData = {
            title: event.title,
            description: event.description || '',
            location: event.location || '',
            date: formatDateLocal(startDate),
            startTime: formatTimeLocal(startDate),
            endTime: formatTimeLocal(endDate),
            all_day: event.all_day,
            scope: (event.scope as CalendarScope) || 'personal',
        };
        showModal = true;
    }

    async function handleSubmit() {
        errorMessage = '';

        if (!formData.title.trim()) {
            errorMessage = 'Title is required';
            return;
        }

        if (!formData.all_day && formData.endTime < formData.startTime) {
            errorMessage = 'End time must be after start time';
            return;
        }

        const start_at = formData.all_day
            ? buildDateTime(formData.date, '', '00:00:00')
            : buildDateTime(formData.date, formData.startTime, '00:00:00');
        const end_at = formData.all_day
            ? buildDateTime(formData.date, '', '23:59:59')
            : buildDateTime(formData.date, formData.endTime, '23:59:00');

        try {
            if (editingEvent) {
                await calendarService.update(editingEvent.id, {
                    title: formData.title,
                    description: formData.description || undefined,
                    location: formData.location || undefined,
                    start_at,
                    end_at,
                    all_day: formData.all_day,
                    scope: formData.scope,
                });
            } else {
                await calendarService.create({
                    title: formData.title,
                    description: formData.description || undefined,
                    location: formData.location || undefined,
                    start_at,
                    end_at,
                    all_day: formData.all_day,
                    scope: formData.scope,
                });
            }
            showModal = false;
            await loadEvents(formData.date);
        } catch (error: any) {
            console.error('Failed to save event:', error);
            errorMessage = error?.message || 'Failed to save event';
        }
    }

    async function handleDelete(event: CalendarEvent) {
        if (!confirm('Delete this event?')) return;
        try {
            await calendarService.delete(event.id);
            await loadEvents(selectedDate);
        } catch (error) {
            console.error('Failed to delete event:', error);
            errorMessage = 'Failed to delete event';
        }
    }

    function handleMonthChange(event: CustomEvent<{ year: number; month: number }>) {
        const { year, month } = event.detail;
        const date = new Date(year, month, 1);
        const dateStr = formatDateLocal(date);
        selectedDate = dateStr;
        loadEvents(dateStr);
    }

    onMount(() => {
        registerShortcut({ key: 'N', action: 'Add Event', icon: Plus, handler: openCreateModal });
        loadEvents(selectedDate);
    });

    onDestroy(() => unregisterShortcut('N'));

    $: canReadShared = canRead(navPath, $navigationStore) || $userStore.user?.isAdmin;
    $: canCreateShared = canCreate(navPath, $navigationStore) || $userStore.user?.isAdmin;
    $: canUpdateShared = canUpdate(navPath, $navigationStore) || $userStore.user?.isAdmin;
    $: canDeleteShared = canDelete(navPath, $navigationStore) || $userStore.user?.isAdmin;

    $: visibleScopes = {
        personal: true,
        department: scopeFilter.department && !!canReadShared,
        company: scopeFilter.company && !!canReadShared,
    };

    $: eventDateKeys = (() => {
        const keys = new Set<string>();
        for (const event of events) {
            const start = new Date(event.start_at);
            const end = new Date(event.end_at);
            const cursor = new Date(start.getFullYear(), start.getMonth(), start.getDate());
            const endDate = new Date(end.getFullYear(), end.getMonth(), end.getDate());
            while (cursor <= endDate) {
                keys.add(formatDateLocal(cursor));
                cursor.setDate(cursor.getDate() + 1);
            }
        }
        return Array.from(keys);
    })();

    $: selectedEvents = events.filter((event) => {
        const startKey = formatDateLocal(new Date(event.start_at));
        const endKey = formatDateLocal(new Date(event.end_at));
        const withinRange = selectedDate >= startKey && selectedDate <= endKey;
        const scopeAllowed =
            (event.scope === 'personal' && scopeFilter.personal) ||
            (event.scope === 'department' && visibleScopes.department) ||
            (event.scope === 'company' && visibleScopes.company);
        return withinRange && scopeAllowed;
    });

    function canEditEvent(event: CalendarEvent) {
        if (event.scope === 'personal') {
            return event.created_by === $userStore.user?.id;
        }
        return !!canUpdateShared;
    }

    function canDeleteEvent(event: CalendarEvent) {
        if (event.scope === 'personal') {
            return event.created_by === $userStore.user?.id;
        }
        return !!canDeleteShared;
    }

    $: eventsWithActions = selectedEvents.map((event) => ({
        ...event,
        can_edit: canEditEvent(event),
        can_delete: canDeleteEvent(event),
    }));

    $: scopeOptions = (() => {
        const options: { value: CalendarScope; label: string }[] = [
            { value: 'personal', label: 'Personal' },
        ];
        if (canCreateShared) {
            options.push(
                { value: 'department', label: 'Department' },
                { value: 'company', label: 'Company' },
            );
        }
        if (editingEvent && !options.find((opt) => opt.value === editingEvent?.scope)) {
            options.push({
                value: editingEvent.scope as CalendarScope,
                label: editingEvent.scope,
            });
        }
        return options;
    })();
</script>

<div class="flex items-center justify-between mb-4">
    <div class="text-sm text-error">{errorMessage}</div>
    <div class="flex items-center gap-2">
        <button class="btn btn-sm btn-primary" on:click={openCreateModal}>
            <Plus class="w-4 h-4 mr-1" /> Add Event
        </button>
    </div>
</div>

{#if loading}
    <div class="flex justify-center p-8">
        <span class="loading loading-spinner loading-lg"></span>
    </div>
{:else}
    <div class="grid xl:grid-cols-[1.4fr_1fr] gap-6">
        <div class="space-y-4">
            <div class="flex items-center gap-2 text-xs opacity-80">
                <Filter class="w-4 h-4" />
                <label class="flex items-center gap-2">
                    <input
                        type="checkbox"
                        class="checkbox checkbox-xs"
                        bind:checked={scopeFilter.personal} />
                    Personal
                </label>
                {#if canReadShared}
                    <label class="flex items-center gap-2">
                        <input
                            type="checkbox"
                            class="checkbox checkbox-xs"
                            bind:checked={scopeFilter.department} />
                        Department
                    </label>
                    <label class="flex items-center gap-2">
                        <input
                            type="checkbox"
                            class="checkbox checkbox-xs"
                            bind:checked={scopeFilter.company} />
                        Company
                    </label>
                {/if}
            </div>

            <Events
                events={eventsWithActions}
                onEdit={openEditModal}
                onDelete={handleDelete} />
        </div>

        <div class="space-y-4">
            <Calendar
                bind:selectedDate
                eventDates={eventDateKeys}
                on:monthChange={handleMonthChange} />
            <div class="text-xs opacity-70">
                Selected date: {selectedDate}
            </div>
        </div>
    </div>
{/if}

{#if showModal}
    <dialog class="modal modal-open">
        <div class="modal-box">
            <h3 class="font-bold text-lg mb-4">
                {editingEvent ? 'Edit Event' : 'Create Event'}
            </h3>
            <form on:submit|preventDefault={handleSubmit}>
                <div class="form-control">
                    <label class="label" for="title">
                        <span class="label-text">Title *</span>
                    </label>
                    <input
                        id="title"
                        type="text"
                        class="input input-bordered"
                        bind:value={formData.title}
                        required />
                </div>
                <div class="form-control mt-4">
                    <label class="label" for="description">
                        <span class="label-text">Description</span>
                    </label>
                    <textarea
                        id="description"
                        class="textarea textarea-bordered"
                        rows="3"
                        bind:value={formData.description}></textarea>
                </div>
                <div class="form-control mt-4">
                    <label class="label" for="location">
                        <span class="label-text">Location</span>
                    </label>
                    <input
                        id="location"
                        type="text"
                        class="input input-bordered"
                        bind:value={formData.location} />
                </div>
                <div class="grid grid-cols-2 gap-4 mt-4">
                    <div class="form-control">
                        <label class="label" for="date">
                            <span class="label-text">Date</span>
                        </label>
                        <input
                            id="date"
                            type="date"
                            class="input input-bordered"
                            bind:value={formData.date} />
                    </div>
                    <div class="form-control">
                        <label class="label" for="scope">
                            <span class="label-text">Scope</span>
                        </label>
                        <select
                            id="scope"
                            class="select select-bordered"
                            bind:value={formData.scope}>
                            {#each scopeOptions as option}
                                <option value={option.value}>
                                    {option.label}
                                </option>
                            {/each}
                        </select>
                    </div>
                </div>

                <div class="form-control mt-4">
                    <label class="label cursor-pointer gap-3 justify-start">
                        <input
                            type="checkbox"
                            class="checkbox checkbox-sm"
                            bind:checked={formData.all_day} />
                        <span class="label-text">All day event</span>
                    </label>
                </div>

                <div class="grid grid-cols-2 gap-4 mt-2">
                    <div class="form-control">
                        <label class="label" for="start-time">
                            <span class="label-text">Start time</span>
                        </label>
                        <input
                            id="start-time"
                            type="time"
                            class="input input-bordered"
                            bind:value={formData.startTime}
                            disabled={formData.all_day} />
                    </div>
                    <div class="form-control">
                        <label class="label" for="end-time">
                            <span class="label-text">End time</span>
                        </label>
                        <input
                            id="end-time"
                            type="time"
                            class="input input-bordered"
                            bind:value={formData.endTime}
                            disabled={formData.all_day} />
                    </div>
                </div>

                <div class="modal-action">
                    <button
                        type="button"
                        class="btn"
                        on:click={() => (showModal = false)}>Cancel</button>
                    <button type="submit" class="btn btn-primary">
                        {editingEvent ? 'Update' : 'Create'}
                    </button>
                </div>
            </form>
        </div>
        <form method="dialog" class="modal-backdrop">
            <button type="button" on:click={() => (showModal = false)}>
                close
            </button>
        </form>
    </dialog>
{/if}
