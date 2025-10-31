<script lang="ts">
    type Event = { date: string; note: string };
    export let events: Event[] = [];
</script>

<div>
    <h2 class="font-semibold mb-2">Upcoming Events / Todo / Notes</h2>
    {#if events.length === 0}
        <p class="text-sm text-base-content/60">No events yet.</p>
    {/if}
    <ul class="space-y-2">
        {#each events as e}
            <li class="border border-base-300 p-2 rounded flex flex-col">
                <span class="font-semibold">{e.date}</span>
                <span class="mt-1">{e.note}</span>
            </li>
        {/each}
    </ul>
</div>
