<script lang="ts">
    import '../../app.css';
    import Breadcrumb from '../../components/Breadcrumb.svelte';
    import ShortcutPanel from '../../components/ShortcutPanel.svelte';
    import Sidebar from '../../components/layout/Sidebar.svelte';
    import StatusBar from '../../components/layout/StatusBar.svelte';
    import { pageTitle } from '$lib/stores/page-title';
    import { onMount, onDestroy } from 'svelte';
    import {
        initShortcutListener,
        registerShortcut,
        unregisterShortcut,
    } from '$lib/stores/shortcutManager';
    import { adminShortcuts } from '$lib/layout/shortcuts';
    const { children } = $props();

    const openTabs = [
        { name: 'dashboard', active: true, modified: false },
        { name: 'settings', active: false, modified: true },
        { name: 'report', active: false, modified: false },
    ];

    function closeTab(tabName: string) {
        // Implementation for closing tabs
    }

    onMount(() => {
        initShortcutListener();
        adminShortcuts.forEach((sc) => registerShortcut(sc));
    });

    onDestroy(() => {
        adminShortcuts.forEach((sc) => unregisterShortcut(sc.key));
    });
</script>

<svelte:head>
    <title>{$pageTitle.title} - XenLedger</title>
</svelte:head>

<div
    class="h-screen bg-base-300 text-base-content flex flex-col font-mono text-sm">
    <!-- Top Tab Bar -->
    <div class="bg-base-200 border-b border-base-300 flex items-center px-2">
        <div class="flex items-center space-x-1">
            {#each openTabs as tab}
                <div
                    class="flex items-center bg-base-100 {tab.active
                        ? 'bg-base-content text-base-100'
                        : ''} px-3 py-1 border-r border-base-300">
                    <span class="mr-2">{tab.name}</span>
                    {#if tab.modified}
                        <span class="text-warning">●</span>
                    {/if}
                    <button
                        class="ml-2 hover:bg-error hover:text-error-content rounded px-1"
                        onclick={() => closeTab(tab.name)}
                        aria-label="Close tab">
                        ×
                    </button>
                </div>
            {/each}
        </div>
    </div>

    <div class="flex flex-1 overflow-hidden">
        <Sidebar />
        <!-- Main Content Area -->
        <div class="flex-1 bg-base-100 flex flex-col">
            <Breadcrumb />

            <main class="space-y-6 p-4 overflow-y-auto">
                <h1 class="text-2xl font-bold">{$pageTitle.title}</h1>

                {@render children()}
            </main>
        </div>
        <ShortcutPanel />
    </div>
    <StatusBar />
</div>
