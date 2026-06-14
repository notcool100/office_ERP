<script lang="ts">
    import '../../app.css';
    import Sidebar from '../../components/layout/Sidebar.svelte';
    import { pageTitle } from '$lib/stores/page-title';
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { userStore } from '$lib/stores/user';
    import { onMount, onDestroy } from 'svelte';
    import { initShortcutListener, registerShortcut, unregisterShortcut } from '$lib/stores/shortcutManager';
    import { adminShortcuts } from '$lib/layout/shortcuts';
    import { goto } from '$app/navigation';
    import ToastContainer from '$lib/components/ToastContainer.svelte';
    import { notificationService } from '$lib/services/notification-service';
    import { ChevronRight, Bell } from 'lucide-svelte';

    const { children } = $props();

    let sidebarOpen = true;

    $effect(() => {
        if (!$userStore.loading && !$userStore.isAuthenticated) {
            goto('/login');
        }
    });

    onMount(async () => {
        initShortcutListener();
        adminShortcuts.forEach((sc) => registerShortcut(sc));
        await userStore.init();
        if ($userStore.isAuthenticated) {
            notificationService.connect();
        }
    });

    onDestroy(() => {
        adminShortcuts.forEach((sc) => unregisterShortcut(sc.key));
        notificationService.disconnect();
    });

    function getInitials(name: string): string {
        return name.split(' ').map(n => n[0]).join('').slice(0, 2).toUpperCase();
    }
</script>

<svelte:head>
    <title>{$pageTitle.title} — Office ERP</title>
</svelte:head>

<ToastContainer />

<div class="h-screen flex overflow-hidden bg-base-200">
    <!-- Sidebar -->
    <Sidebar bind:open={sidebarOpen} />

    <!-- Main column -->
    <div class="flex-1 flex flex-col overflow-hidden">

        <!-- Top bar -->
        <header class="h-14 bg-base-100 border-b border-base-300 flex items-center gap-3 px-4 shrink-0">
            <!-- Breadcrumb -->
            <nav class="flex items-center gap-1.5 text-sm flex-1 min-w-0 text-base-content/60">
                {#each $breadcrumb as crumb, i}
                    {#if i > 0}
                        <ChevronRight size={13} class="shrink-0 text-base-content/30" />
                    {/if}
                    <div class="flex items-center gap-1 {i === $breadcrumb.length - 1 ? 'text-base-content font-medium' : ''}">
                        <svelte:component this={crumb.icon} size={13} class="shrink-0" />
                        <span class="truncate">{crumb.label}</span>
                    </div>
                {/each}
            </nav>

            <!-- Right side -->
            <div class="flex items-center gap-2 shrink-0">
                <button class="btn btn-ghost btn-sm btn-circle" aria-label="Notifications">
                    <Bell size={16} />
                </button>
                <div class="flex items-center gap-2 pl-2 border-l border-base-300">
                    <div class="w-7 h-7 rounded-full bg-primary text-primary-content flex items-center justify-center text-xs font-semibold">
                        {getInitials($userStore.user?.userName ?? 'U')}
                    </div>
                    <span class="text-sm font-medium hidden sm:block">{$userStore.user?.userName ?? ''}</span>
                </div>
            </div>
        </header>

        <!-- Page content -->
        <main class="flex-1 overflow-y-auto bg-base-200">
            {@render children()}
        </main>
    </div>
</div>
