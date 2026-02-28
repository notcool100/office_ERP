<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, CircleUser, CalendarHeart, Plus } from 'lucide-svelte';
    import { onMount, onDestroy } from 'svelte';
    import {
        registerShortcut,
        unregisterShortcut,
    } from '$lib/stores/shortcutManager';
    import { calendarService } from '$lib/services/calendar';
    import type { CalendarEvent } from '$lib/types/calendar';

    pageTitle.set({
        title: 'Schedule',
        desc: 'Manage events and notes',
    });

    import Calendar from '../../../../components/schedule/Calendar.svelte';
    import Events from '../../../../components/schedule/Events.svelte';

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Profile', icon: CircleUser },
        { label: 'Schedule', icon: CalendarHeart },
    ]);

    let events: CalendarEvent[] = [];
    let selectedDate = new Date().toISOString().slice(0, 10);
    let note = '';
    let loading = true;
    let errorMessage = '';

    const formatDateLocal = (date: Date) => {
        const year = date.getFullYear();
        const month = String(date.getMonth() + 1).padStart(2, '0');
        const day = String(date.getDate()).padStart(2, '0');
        return `${year}-${month}-${day}`;
    };

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
            console.error('Failed to load personal events:', error);
            errorMessage = 'Failed to load events';
        } finally {
            loading = false;
        }
    }

    async function addNote() {
        if (note.trim()) {
            const start_at = `${selectedDate}T00:00:00`;
            const end_at = `${selectedDate}T23:59:59`;
            try {
                await calendarService.create({
                    title: note.trim(),
                    start_at,
                    end_at,
                    all_day: true,
                    scope: 'personal',
                });
                note = '';
                await loadEvents(selectedDate);
            } catch (error) {
                console.error('Failed to add note:', error);
                errorMessage = 'Failed to add note';
            }
        }
    }

    function handleMonthChange(
        event: CustomEvent<{ year: number; month: number }>,
    ) {
        const { year, month } = event.detail;
        const date = new Date(year, month, 1);
        const dateStr = formatDateLocal(date);
        selectedDate = dateStr;
        loadEvents(dateStr);
    }

    onMount(() => {
        registerShortcut({
            key: 'N',
            action: 'Add Note',
            icon: Plus,
            handler: addNote,
        });
        loadEvents(selectedDate);
    });

    onDestroy(() => unregisterShortcut('N'));

    $: eventDateKeys = (() => {
        const keys = new Set<string>();
        for (const event of events) {
            const start = new Date(event.start_at);
            const end = new Date(event.end_at);
            const cursor = new Date(
                start.getFullYear(),
                start.getMonth(),
                start.getDate(),
            );
            const endDate = new Date(
                end.getFullYear(),
                end.getMonth(),
                end.getDate(),
            );
            while (cursor <= endDate) {
                keys.add(formatDateLocal(cursor));
                cursor.setDate(cursor.getDate() + 1);
            }
        }
        return Array.from(keys);
    })();
</script>

{#if errorMessage}
    <div class="text-sm text-error mb-2">{errorMessage}</div>
{/if}

{#if loading}
    <div class="flex justify-center p-8">
        <span class="loading loading-spinner loading-lg"></span>
    </div>
{:else}
    <div class="grid md:grid-cols-2 gap-6">
        <Events events={events} />
        <div>
            <Calendar
                bind:selectedDate={selectedDate}
                eventDates={eventDateKeys}
                on:monthChange={handleMonthChange} />
            <div class="space-y-2">
                <input
                    type="date"
                    bind:value={selectedDate}
                    class="input input-bordered w-full" />
                <textarea
                    bind:value={note}
                    rows="3"
                    placeholder="Add note"
                    class="textarea textarea-bordered w-full"></textarea>
                <button class="btn btn-primary btn-sm" on:click={addNote}
                    >Add</button>
            </div>
        </div>
    </div>
{/if}
