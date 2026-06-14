<script lang="ts">
    import { page } from '$app/stores';
    import { onMount } from 'svelte';
    import { loadUserNavigation, fallbackMenu } from '$lib/layout/navigationLoader';
    import type { MenuItem } from '$lib/layout/types';
    import { goto } from '$app/navigation';
    import { userStore } from '$lib/stores/user';
    import { LogOut, ChevronDown, ChevronRight, Building2, PanelLeftClose, PanelLeftOpen } from 'lucide-svelte';
    import { authService } from '$lib/services/auth.service';

    let { open = $bindable(true) } = $props();

    let menuItems: MenuItem[] = $state(fallbackMenu);
    let loading = $state(true);
    let expandedFolders = $state(new Set<string>());

    onMount(async () => {
        try {
            const dynamicMenu = await loadUserNavigation();
            if (dynamicMenu && dynamicMenu.length > 0) {
                menuItems = dynamicMenu;
                // Auto-expand the folder containing the active page
                const current = $page.url.pathname;
                const next = new Set<string>();
                menuItems.forEach(item => {
                    if (item.type === 'folder' && item.children?.some(c => current.startsWith(c.url ?? ''))) {
                        next.add(item.name);
                    }
                });
                expandedFolders = next;
            }
        } catch {
            // use fallback
        } finally {
            loading = false;
        }
    });

    let currentPath = $derived($page.url.pathname);

    function toggleFolder(name: string) {
        const next = new Set(expandedFolders);
        if (next.has(name)) next.delete(name);
        else next.add(name);
        expandedFolders = next;
    }

    function isActive(url?: string): boolean {
        if (!url) return false;
        return currentPath === url || currentPath.startsWith(url + '/');
    }

    function isFolderActive(item: MenuItem): boolean {
        return item.children?.some(c => isActive(c.url)) ?? false;
    }

    async function logout() {
        try { await authService.logout(); } catch {}
        goto('/login');
    }

    function getInitials(name: string): string {
        return name.split(' ').map(n => n[0]).join('').slice(0, 2).toUpperCase();
    }
</script>

<!-- Sidebar -->
<aside class="flex flex-col bg-base-100 border-r border-base-300 transition-all duration-200 {open ? 'w-56' : 'w-12'} shrink-0">

    <!-- Brand header -->
    <div class="h-14 flex items-center gap-3 px-3 border-b border-base-300 shrink-0">
        <div class="w-7 h-7 rounded-lg bg-primary flex items-center justify-center shrink-0">
            <Building2 size={15} class="text-primary-content" />
        </div>
        {#if open}
            <span class="font-bold text-base tracking-tight truncate">Office ERP</span>
        {/if}
        <button
            class="ml-auto btn btn-ghost btn-xs btn-circle shrink-0"
            onclick={() => open = !open}
            aria-label="Toggle sidebar">
            {#if open}
                <PanelLeftClose size={15} />
            {:else}
                <PanelLeftOpen size={15} />
            {/if}
        </button>
    </div>

    <!-- Nav items -->
    <nav class="flex-1 overflow-y-auto py-2 px-1.5 space-y-0.5">
        {#if loading}
            <div class="flex justify-center p-4">
                <span class="loading loading-spinner loading-xs"></span>
            </div>
        {:else}
            {#each menuItems as item}
                {#if item.type === 'folder'}
                    {@const folderActive = isFolderActive(item)}
                    {@const expanded = expandedFolders.has(item.name)}
                    {@const FolderIcon = item.icon}
                    <!-- Folder button -->
                    <button
                        class="flex items-center w-full gap-2 px-2 py-1.5 rounded-lg text-sm transition-colors
                            {folderActive ? 'text-primary bg-primary/8 font-medium' : 'text-base-content/70 hover:bg-base-200 hover:text-base-content'}"
                        onclick={() => toggleFolder(item.name)}>
                        <FolderIcon size={16} class="shrink-0 {item.color ?? ''}" />
                        {#if open}
                            <span class="flex-1 text-left truncate">{item.name}</span>
                            <span class="shrink-0 text-base-content/40">
                                {#if expanded}<ChevronDown size={13}/>{:else}<ChevronRight size={13}/>{/if}
                            </span>
                        {/if}
                    </button>

                    <!-- Children -->
                    {#if expanded || !open}
                        <div class="{open ? 'pl-4' : ''} space-y-0.5">
                            {#each item.children ?? [] as child}
                                {@const active = isActive(child.url)}
                                {@const ChildIcon = child.icon}
                                <button
                                    class="flex items-center w-full gap-2 px-2 py-1.5 rounded-lg text-sm transition-colors
                                        {active ? 'bg-primary text-primary-content font-medium' : 'text-base-content/60 hover:bg-base-200 hover:text-base-content'}"
                                    onclick={() => child.url && goto(child.url)}>
                                    <ChildIcon size={15} class="shrink-0" />
                                    {#if open}
                                        <span class="truncate">{child.name}</span>
                                    {/if}
                                </button>
                            {/each}
                        </div>
                    {/if}

                {:else}
                    {@const active = isActive(item.url)}
                    {@const ItemIcon = item.icon}
                    <button
                        class="flex items-center w-full gap-2 px-2 py-1.5 rounded-lg text-sm transition-colors
                            {active ? 'bg-primary text-primary-content font-medium' : 'text-base-content/70 hover:bg-base-200 hover:text-base-content'}"
                        onclick={() => item.url && goto(item.url)}>
                        <ItemIcon size={16} class="shrink-0 {item.color ?? ''}" />
                        {#if open}
                            <span class="truncate">{item.name}</span>
                        {/if}
                    </button>
                {/if}
            {/each}
        {/if}
    </nav>

    <!-- User + Logout -->
    <div class="border-t border-base-300 p-2 shrink-0">
        <div class="flex items-center gap-2 px-1 py-1">
            <div class="w-7 h-7 rounded-full bg-primary/20 text-primary flex items-center justify-center text-xs font-bold shrink-0">
                {getInitials($userStore.user?.userName ?? 'U')}
            </div>
            {#if open}
                <div class="flex-1 min-w-0">
                    <p class="text-xs font-medium truncate">{$userStore.user?.userName ?? 'User'}</p>
                    <p class="text-xs text-base-content/50 truncate">{$userStore.user?.email ?? ''}</p>
                </div>
            {/if}
            <button
                class="btn btn-ghost btn-xs btn-circle text-base-content/50 hover:text-error shrink-0"
                onclick={logout}
                title="Logout">
                <LogOut size={14} />
            </button>
        </div>
    </div>
</aside>
