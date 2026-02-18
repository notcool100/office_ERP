<script lang="ts">
    import type { CalendarEvent } from '$lib/types/calendar';
    import { Pencil, Trash2 } from 'lucide-svelte';

    type CalendarEventWithActions = CalendarEvent & {
        can_edit?: boolean;
        can_delete?: boolean;
    };

    export let events: CalendarEventWithActions[] = [];
    export let onEdit: ((event: CalendarEvent) => void) | null = null;
    export let onDelete: ((event: CalendarEvent) => void) | null = null;

    function formatTime(value: string, allDay: boolean) {
        if (allDay) return 'All day';
        const date = new Date(value);
        return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    }
</script>

<div>
    <h2 class="font-semibold mb-2">Events</h2>
    {#if events.length === 0}
        <p class="text-sm text-base-content/60">No events yet.</p>
    {/if}
    <ul class="space-y-2">
        {#each events as event}
            <li class="border border-base-300 p-2 rounded flex flex-col gap-1">
                <div class="flex items-start justify-between gap-2">
                    <div>
                        <div class="font-semibold">{event.title}</div>
                        <div class="text-xs opacity-70 capitalize">
                            {event.scope}
                        </div>
                    </div>
                    {#if onEdit || onDelete}
                        <div class="flex items-center gap-1">
                            {#if onEdit && event.can_edit}
                                <button
                                    class="btn btn-xs btn-ghost"
                                    on:click={() => onEdit?.(event)}
                                    title="Edit event">
                                    <Pencil class="w-3 h-3" />
                                </button>
                            {/if}
                            {#if onDelete && event.can_delete}
                                <button
                                    class="btn btn-xs btn-ghost text-error"
                                    on:click={() => onDelete?.(event)}
                                    title="Delete event">
                                    <Trash2 class="w-3 h-3" />
                                </button>
                            {/if}
                        </div>
                    {/if}
                </div>
                <div class="text-xs opacity-80">
                    {formatTime(event.start_at, event.all_day)} - {formatTime(event.end_at, event.all_day)}
                </div>
                {#if event.location}
                    <div class="text-xs opacity-70">Location: {event.location}</div>
                {/if}
                {#if event.description}
                    <div class="text-sm">{event.description}</div>
                {/if}
            </li>
        {/each}
    </ul>
</div>
