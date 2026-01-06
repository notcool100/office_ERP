<script lang="ts">
    import '../../app.css';
    import Breadcrumb from '../../components/Breadcrumb.svelte';
    import ShortcutPanel from '../../components/ShortcutPanel.svelte';
    import Sidebar from '../../components/layout/Sidebar.svelte';
    import StatusBar from '../../components/layout/StatusBar.svelte';
    import { pageTitle } from '$lib/stores/page-title';
    import { userStore } from '$lib/stores/user';
    import { onMount, onDestroy } from 'svelte';
    import {
        initShortcutListener,
        registerShortcut,
        unregisterShortcut,
    } from '$lib/stores/shortcutManager';
    import { adminShortcuts } from '$lib/layout/shortcuts';
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';
    const { children } = $props();

    interface Tab {
        name: string;
        path: string;
        active: boolean;
        modified: boolean;
    }

    let openTabs: Tab[] = $state([]);

    // Sync tabs with navigation
    $effect(() => {
        const path = $page.url.pathname;
        const existingTab = openTabs.find((t) => t.path === path);

        // Deactivate all
        openTabs.forEach((t) => (t.active = false));

        if (existingTab) {
            existingTab.active = true;
        } else {
            openTabs.push({
                name: $pageTitle.title || 'Loading...',
                path: path,
                active: true,
                modified: false,
            });
        }
    });

    // Sync tab name with page title
    $effect(() => {
        const activeTab = openTabs.find((t) => t.active);
        if (activeTab && $pageTitle.title) {
            activeTab.name = $pageTitle.title;
        }
    });

    // Auth Protection
    $effect(() => {
        if (!$userStore.loading && !$userStore.isAuthenticated) {
            goto('/login');
        }
    });

    function selectTab(tab: Tab) {
        goto(tab.path);
    }

    function closeTab(tabPath: string) {
        const index = openTabs.findIndex((t) => t.path === tabPath);
        if (index === -1) return;

        const wasActive = openTabs[index].active;
        openTabs.splice(index, 1);

        if (wasActive && openTabs.length > 0) {
            const newIndex = Math.min(index, openTabs.length - 1);
            goto(openTabs[newIndex].path);
        } else if (wasActive && openTabs.length === 0) {
            // Optional: Navigate to a default page if all tabs closed?
            // For now, let's leave it, but typically you'd want at least one tab or redirect to dashboard.
            goto('/admin/dashboard');
        }
    }

    onMount(async () => {
        initShortcutListener();
        adminShortcuts.forEach((sc) => registerShortcut(sc));
        await userStore.init();
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
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div
                    class="flex items-center cursor-pointer bg-base-100 {tab.active
                        ? 'bg-base-content text-base-100'
                        : ''} px-3 py-1 border-r border-base-300"
                    onclick={() => selectTab(tab)}>
                    <span class="mr-2">{tab.name}</span>
                    {#if tab.modified}
                        <span class="text-warning">●</span>
                    {/if}
                    <button
                        class="ml-2 hover:bg-error hover:text-error-content rounded px-1"
                        onclick={(e) => {
                            e.stopPropagation();
                            closeTab(tab.path);
                        }}
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
