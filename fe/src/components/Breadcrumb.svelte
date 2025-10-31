<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { ChevronRight, Settings } from 'lucide-svelte';
</script>

<div
    class="bg-base-200 px-4 py-2 border-b border-base-300 text-xs flex items-center space-x-2">
    {#each $breadcrumb as crumb, i}
        {#if i > 0}
            <ChevronRight class="w-3 h-3 text-base-content/40" />
        {/if}
        <div class="flex items-center space-x-2">
            <crumb.icon class="w-3 h-3" />
            <span>{crumb.label}</span>
        </div>
    {/each}

    <div
        class="ml-auto flex items-center space-x-4 text-xs text-base-content/70">
        <span>👤 Admin</span>
        <span>🕒 Last updated: 2 hours ago</span>
        <span>📦 Workspace: Xenith</span>
    </div>
</div>
