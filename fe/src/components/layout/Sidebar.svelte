<script lang="ts">
    import { page } from '$app/stores';
    import { onMount } from 'svelte';
    import {
        loadUserNavigation,
        fallbackMenu,
    } from '$lib/layout/navigationLoader';
    import type { MenuItem } from '$lib/layout/types';
    import { ListTree, Power } from 'lucide-svelte';
    import { goto } from '$app/navigation';

    let menuItems: MenuItem[] = fallbackMenu;
    let loading = true;
    let sidebarCollapsed = false;
    let activeFile = '';

    onMount(async () => {
        try {
            const dynamicMenu = await loadUserNavigation();
            if (dynamicMenu && dynamicMenu.length > 0) {
                menuItems = dynamicMenu;
            }
        } catch (error) {
            console.error('Failed to load navigation, using fallback:', error);
        } finally {
            loading = false;
        }
    });

    $: currentPath = $page.url.pathname;

    function toggleFolder(item: MenuItem) {
        item.expanded = !item.expanded;
        menuItems = menuItems; // Trigger reactivity
    }

    function selectFile(fileName: string, url?: string) {
        activeFile = fileName;
        if (url) goto(url);
    }
</script>

<div class="w-64 bg-base-200 border-r border-base-300 flex flex-col">
    <div class="p-2 border-b border-base-300 flex items-center justify-between">
        <div class="flex items-center space-x-2 font-semibold">
            <ListTree class="w-4 h-4 text-base-content" />
            <span>Explorer</span>
        </div>
        <button
            class="btn btn-ghost btn-xs"
            on:click={() => (sidebarCollapsed = !sidebarCollapsed)}
            aria-label="Toggle sidebar">
            {sidebarCollapsed ? '▶' : '◀'}
        </button>
    </div>

    {#if !sidebarCollapsed}
        <div class="flex-1 overflow-y-auto p-2">
            {#if loading}
                <div class="flex justify-center p-4">
                    <span class="loading loading-spinner loading-sm"></span>
                </div>
            {:else}
                {#each menuItems as item}
                    <div class="mb-1">
                        {#if item.type === 'folder'}
                            <button
                                class="flex items-center w-full text-left hover:bg-base-300 px-1 py-0.5 rounded"
                                on:click={() => toggleFolder(item)}
                                aria-label="Toggle folder">
                                <svelte:component
                                    this={item.icon}
                                    class="w-4 h-4 mr-2 {item.color ?? ''}" />
                                <span>{item.name}</span>
                            </button>

                            {#if item.expanded && item.children}
                                <div class="ml-4">
                                    {#each item.children as child}
                                        {#if child.type === 'folder'}
                                            <button
                                                class="flex items-center w-full text-left hover:bg-base-300 px-1 py-0.5 rounded"
                                                on:click={() =>
                                                    toggleFolder(child)}
                                                aria-label="Toggle folder">
                                                <svelte:component
                                                    this={child.icon}
                                                    class="w-4 h-4 mr-2 {item.color ??
                                                        ''}" />
                                                <span>{child.name}</span>
                                            </button>
                                            {#if child.expanded && child.children}
                                                <div class="ml-4">
                                                    {#each child.children as file}
                                                        <button
                                                            class="flex items-center w-full text-left hover:bg-base-300 px-1 py-0.5 rounded {activeFile ===
                                                            file.name
                                                                ? 'bg-primary text-primary-content'
                                                                : ''}"
                                                            on:click={() =>
                                                                selectFile(
                                                                    file.name,
                                                                    file.url,
                                                                )}
                                                            aria-label="Select file">
                                                            <svelte:component
                                                                this={file.icon}
                                                                class="w-4 h-4 mr-2 {file.color ??
                                                                    child.color ??
                                                                    item.color ??
                                                                    ''}" />
                                                            <span
                                                                >{file.name}</span>
                                                        </button>
                                                    {/each}
                                                </div>
                                            {/if}
                                        {:else}
                                            <button
                                                class="flex items-center w-full text-left hover:bg-base-300 px-1 py-0.5 rounded {activeFile ===
                                                child.name
                                                    ? 'bg-primary text-primary-content'
                                                    : ''}"
                                                on:click={() =>
                                                    selectFile(
                                                        child.name,
                                                        child.url,
                                                    )}
                                                aria-label="Select file">
                                                <svelte:component
                                                    this={child.icon}
                                                    class="w-4 h-4 mr-2 {child.color ??
                                                        item.color ??
                                                        ''}" />
                                                <span>{child.name}</span>
                                            </button>
                                        {/if}
                                    {/each}
                                </div>
                            {/if}
                        {:else}
                            <button
                                class="flex items-center w-full text-left hover:bg-base-300 px-1 py-0.5 rounded {activeFile ===
                                item.name
                                    ? 'bg-primary text-primary-content'
                                    : ''}"
                                on:click={() => selectFile(item.name, item.url)}
                                aria-label="Select file">
                                <svelte:component
                                    this={item.icon}
                                    class="w-4 h-4 mr-2 {item.color ?? ''}" />
                                <span>{item.name}</span>
                            </button>
                        {/if}
                    </div>
                {/each}
            {/if}
        </div>
    {/if}

    <!-- Logout Button -->
    <div class="p-2 border-t border-base-300 mt-auto">
        <button
            class="flex items-center text-sm text-base-content hover:text-error px-2 py-1 w-full rounded hover:bg-error/10 transition"
            on:click={() => goto('/login')}>
            <Power class="w-4 h-4 mr-2" />
            <span>Logout</span>
        </button>
    </div>
</div>
