<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, CircleUser, CalendarHeart, Plus } from 'lucide-svelte';
    import { onMount, onDestroy } from 'svelte';
    import {
        registerShortcut,
        unregisterShortcut,
    } from '$lib/stores/shortcutManager';

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

    type Event = { date: string; note: string };
    let events: Event[] = [];
    let selectedDate = new Date().toISOString().slice(0, 10);
    let note = '';

    function addNote() {
        if (note.trim()) {
            events = [...events, { date: selectedDate, note }];
            note = '';
        }
    }

    onMount(() => {
        registerShortcut({
            key: 'N',
            action: 'Add Note',
            icon: Plus,
            handler: addNote,
        });
    });

    onDestroy(() => unregisterShortcut('N'));
</script>

<div class="grid md:grid-cols-2 gap-6">
    <Events events={events} />
    <div>
        <Calendar bind:selectedDate={selectedDate} />
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
