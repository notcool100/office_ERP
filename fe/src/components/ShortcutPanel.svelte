<script lang="ts">
    import {
        registeredShortcuts,
        shortcutStack,
    } from '$lib/stores/shortcutManager';
    import { CircleArrowOutUpLeft, ArrowLeft } from 'lucide-svelte';
    let outlineCollapsed = $state(false);
</script>

<div class="w-64 bg-base-200 border-l border-base-300 flex flex-col">
    <div class="p-2 border-b border-base-300 flex items-center justify-between">
        <span class="font-semibold">⌨️ Shortcuts</span>
        <button
            class="btn btn-ghost btn-xs"
            onclick={() => (outlineCollapsed = !outlineCollapsed)}
            aria-label="Toggle shortcuts">
            {outlineCollapsed ? '▶' : '◀'}
        </button>
    </div>

    {#if !outlineCollapsed}
        <div class="flex-1 overflow-y-auto p-2 text-xs">
            <ul class="space-y-2">
                {#each $registeredShortcuts as item}
                    <li
                        class="flex items-center bg-base-100 hover:bg-base-300 px-2 py-1 rounded">
                        <!-- Left-aligned text and optional icon -->
                        <div class="flex-1 flex items-center space-x-2">
                            {#if item.icon}
                                <item.icon
                                    class="w-4 h-4 text-base-content/70" />
                            {/if}
                            <span class="text-left">{item.action}</span>
                        </div>

                        <!-- Right-aligned key -->
                        <kbd
                            class="ml-auto px-1 py-0.5 bg-base-300 rounded text-xs font-mono font-semibold">
                            {item.key}
                        </kbd>
                    </li>
                {/each}
                {#if $shortcutStack.length > 0}
                    <li
                        class="flex items-center bg-base-100 hover:bg-base-300 px-2 py-1 rounded">
                        <div class="flex-1 flex items-center space-x-2">
                            <ArrowLeft class="w-4 h-4 text-base-content/70" />
                            <span class="text-left">Back</span>
                        </div>
                        <kbd
                            class="ml-auto px-1 py-0.5 bg-base-300 rounded text-xs font-mono font-semibold">
                            Backspace
                        </kbd>
                    </li>
                    <li
                        class="flex items-center bg-base-100 hover:bg-base-300 px-2 py-1 rounded">
                        <div class="flex-1 flex items-center space-x-2">
                            <CircleArrowOutUpLeft
                                class="w-4 h-4 text-base-content/70" />
                            <span class="text-left">Cancel</span>
                        </div>
                        <kbd
                            class="ml-auto px-1 py-0.5 bg-base-300 rounded text-xs font-mono font-semibold">
                            Esc
                        </kbd>
                    </li>
                {/if}
            </ul>
        </div>
    {/if}
</div>
